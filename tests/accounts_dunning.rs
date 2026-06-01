use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::dunning::dunning::{
    get_dunning_letter_text, update_linked_dunnings_plan, Dunning, DunningError, DunningLetterText,
    DunningStatus, DunningStatusUpdatePlan, InvoiceOutstanding, OverduePayment, PartyDetails,
    PaymentScheduleOutstanding, SalesInvoiceSnapshot,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000_001,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn dunning_matches_erpnext_metadata_shape() {
    assert_eq!(Dunning::DOCTYPE, "Dunning");
    assert_eq!(Dunning::MODULE, "Accounts");
    assert_eq!(Dunning::AUTONAME, "naming_series:");
    assert!(Dunning::IS_SUBMITTABLE);
    assert_eq!(Dunning::FIELD_ORDER.len(), 54);
    assert_eq!(
        &Dunning::FIELD_ORDER[..12],
        [
            "naming_series",
            "customer",
            "customer_name",
            "column_break_3",
            "company",
            "posting_date",
            "posting_time",
            "status",
            "section_break_9",
            "currency",
            "column_break_11",
            "conversion_rate",
        ]
    );
    assert_eq!(
        &Dunning::FIELD_ORDER[Dunning::FIELD_ORDER.len() - 6..],
        [
            "contact_email",
            "section_break_xban",
            "column_break_16",
            "company_address",
            "company_address_display",
            "column_break_lqmf",
        ]
    );

    let fields = Dunning::fields();
    assert!(fields.contains(
        &FieldSpec::link("customer", "Customer")
            .options("Customer")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::select("status", "Status")
            .options("Draft\nResolved\nUnresolved\nCancelled")
            .default("Unresolved")
            .read_only()
            .allow_on_submit()
            .in_standard_filter()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("dunning_fee", "Dunning Fee")
            .options("currency")
            .default("0")
            .precision("2")
            .fetch_from("dunning_type.dunning_fee")
            .fetch_if_empty()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("base_dunning_amount", "Dunning Amount (Company Currency)")
            .options("Company:company:default_currency")
            .default("0")
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::table("overdue_payments", "Overdue Payments").options("Overdue Payment")
    ));

    let controller = Dunning::default();
    assert_eq!(controller.doctype(), "Dunning");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(controller.custom_hooks(), &["validate", "on_cancel"]);
}

#[test]
fn dunning_validate_matches_currency_interest_totals_party_and_levels() {
    let mut dunning = Dunning {
        name: Some("DUNN-0001".to_string()),
        customer: "CUST-001".to_string(),
        company: "Wind Power LLC".to_string(),
        posting_date: "2026-05-10".to_string(),
        currency: Some("USD".to_string()),
        conversion_rate: 2.0,
        rate_of_interest: 10.0,
        dunning_fee: 25.0,
        overdue_payments: vec![
            OverduePayment {
                sales_invoice: "SI-0001".to_string(),
                payment_schedule: Some("PS-0001".to_string()),
                parent: Some("DUNN-0001".to_string()),
                due_date: "2026-05-01".to_string(),
                outstanding: 36_500.0,
                ..Default::default()
            },
            OverduePayment {
                sales_invoice: "SI-0002".to_string(),
                payment_schedule: Some("PS-0002".to_string()),
                parent: Some("DUNN-0001".to_string()),
                due_date: "2026-05-08".to_string(),
                outstanding: 7_300.0,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let invoice_currencies = BTreeMap::from([
        ("SI-0001".to_string(), "USD".to_string()),
        ("SI-0002".to_string(), "USD".to_string()),
    ]);
    let past_dunning_counts = BTreeMap::from([
        ("PS-0001".to_string(), 2_usize),
        ("PS-0002".to_string(), 0_usize),
    ]);

    dunning
        .validate(
            &invoice_currencies,
            PartyDetails {
                customer_address: Some("ADDR-CUST".to_string()),
                address_display: Some("Customer address".to_string()),
                company_address: Some("ADDR-COMP".to_string()),
                contact_person: Some("CONT-001".to_string()),
                contact_display: Some("Contact Display".to_string()),
                contact_mobile: Some("+998900000000".to_string()),
                company_address_display: Some("Company address".to_string()),
            },
            &past_dunning_counts,
        )
        .expect("matching currencies validate");

    assert_eq!(dunning.overdue_payments[0].overdue_days, 9);
    assert_close(dunning.overdue_payments[0].interest, 90.0);
    assert_eq!(dunning.overdue_payments[0].dunning_level, 3);
    assert_eq!(dunning.overdue_payments[1].overdue_days, 2);
    assert_close(dunning.overdue_payments[1].interest, 4.0);
    assert_eq!(dunning.overdue_payments[1].dunning_level, 1);
    assert_close(dunning.total_outstanding, 43_800.0);
    assert_close(dunning.total_interest, 94.0);
    assert_close(dunning.dunning_amount, 119.0);
    assert_close(dunning.base_dunning_amount, 238.0);
    assert_close(dunning.grand_total, 43_919.0);
    assert_eq!(dunning.customer_address.as_deref(), Some("ADDR-CUST"));
    assert_eq!(
        dunning.company_address_display.as_deref(),
        Some("Company address")
    );

    let mismatched = BTreeMap::from([("SI-0001".to_string(), "EUR".to_string())]);
    assert_eq!(
        dunning.validate_same_currency(&mismatched),
        Err(DunningError::CurrencyMismatch {
            sales_invoice: "SI-0001".to_string(),
            invoice_currency: Some("EUR".to_string()),
            dunning_currency: Some("USD".to_string()),
        })
    );
}

#[test]
fn dunning_on_cancel_records_same_ignored_doctypes_as_erpnext() {
    let mut dunning = Dunning::default();

    dunning.on_cancel();

    assert_eq!(
        dunning.ignore_linked_doctypes,
        vec![
            "GL Entry",
            "Stock Ledger Entry",
            "Repost Item Valuation",
            "Repost Payment Ledger",
            "Repost Payment Ledger Items",
            "Repost Accounting Ledger",
            "Repost Accounting Ledger Items",
            "Unreconcile Payment",
            "Unreconcile Payment Entries",
            "Payment Ledger Entry",
            "Serial and Batch Bundle",
        ]
    );
}

#[test]
fn linked_dunning_status_plan_matches_sales_invoice_outstanding_changes() {
    let sales_invoice = SalesInvoiceSnapshot {
        doctype: "Sales Invoice".to_string(),
        name: "SI-0001".to_string(),
        is_return: false,
        outstanding_amount: 0.0,
    };
    let linked = vec![Dunning {
        name: Some("DUNN-0001".to_string()),
        status: DunningStatus::Unresolved,
        overdue_payments: vec![OverduePayment {
            sales_invoice: "SI-0001".to_string(),
            payment_schedule: Some("PS-0001".to_string()),
            outstanding: 100.0,
            ..Default::default()
        }],
        ..Default::default()
    }];

    assert_eq!(
        update_linked_dunnings_plan(
            &sales_invoice,
            100.0,
            &linked,
            &[InvoiceOutstanding {
                sales_invoice: "SI-0001".to_string(),
                outstanding_amount: 0.0,
            }],
            &[PaymentScheduleOutstanding {
                payment_schedule: "PS-0001".to_string(),
                outstanding: 0.0,
            }],
        ),
        vec![DunningStatusUpdatePlan {
            dunning_name: "DUNN-0001".to_string(),
            new_status: DunningStatus::Resolved,
        }]
    );

    let increased_invoice = SalesInvoiceSnapshot {
        outstanding_amount: 125.0,
        ..sales_invoice
    };
    let linked = vec![Dunning {
        name: Some("DUNN-0002".to_string()),
        status: DunningStatus::Resolved,
        overdue_payments: vec![OverduePayment {
            sales_invoice: "SI-0001".to_string(),
            payment_schedule: Some("PS-0001".to_string()),
            outstanding: 100.0,
            ..Default::default()
        }],
        ..Default::default()
    }];
    assert_eq!(
        update_linked_dunnings_plan(
            &increased_invoice,
            100.0,
            &linked,
            &[InvoiceOutstanding {
                sales_invoice: "SI-0001".to_string(),
                outstanding_amount: 125.0,
            }],
            &[PaymentScheduleOutstanding {
                payment_schedule: "PS-0001".to_string(),
                outstanding: 125.0,
            }],
        ),
        vec![DunningStatusUpdatePlan {
            dunning_name: "DUNN-0002".to_string(),
            new_status: DunningStatus::Unresolved,
        }]
    );

    assert!(update_linked_dunnings_plan(&increased_invoice, 125.0, &linked, &[], &[]).is_empty());
}

#[test]
fn dunning_letter_text_selects_language_fallback_and_renders_doc_context() {
    let texts = vec![
        DunningLetterText {
            parent: "Standard".to_string(),
            body_text: "Dear {{ customer_name }}".to_string(),
            closing_text: "Pay {{ grand_total }}".to_string(),
            language: "en".to_string(),
            is_default_language: true,
        },
        DunningLetterText {
            parent: "Standard".to_string(),
            body_text: "Bonjour {{ customer_name }}".to_string(),
            closing_text: "Total {{ grand_total }}".to_string(),
            language: "fr".to_string(),
            is_default_language: false,
        },
    ];
    let doc = BTreeMap::from([
        ("language".to_string(), "fr".to_string()),
        ("customer_name".to_string(), "Ada".to_string()),
        ("grand_total".to_string(), "120.00".to_string()),
    ]);

    let rendered = get_dunning_letter_text("Standard", &doc, None, &texts).expect("language row");
    assert_eq!(rendered.body_text, "Bonjour Ada");
    assert_eq!(rendered.closing_text, "Total 120.00");
    assert_eq!(rendered.language, "fr");

    let fallback =
        get_dunning_letter_text("Standard", &BTreeMap::new(), None, &texts).expect("default row");
    assert_eq!(fallback.body_text, "Dear {{ customer_name }}");
    assert_eq!(fallback.language, "en");

    assert!(get_dunning_letter_text("Missing", &doc, Some("fr"), &texts).is_none());
}
