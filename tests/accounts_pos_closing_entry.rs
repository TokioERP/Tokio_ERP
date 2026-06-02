use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::pos_closing_entry::pos_closing_entry::{
    build_invoice_query_plan, get_cashiers, get_invoices, get_payments, get_taxes,
    make_closing_entry_from_opening, BuildInvoiceQueryPlan, InvoiceCandidate, InvoicePayment,
    InvoiceTax, OpeningEntrySnapshot, POSClosingEntry, POSClosingEntryError, POSInvoiceRecord,
    POSInvoiceReferenceRow, SalesInvoiceRecord, SalesInvoiceReferenceRow,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_entry() -> POSClosingEntry {
    POSClosingEntry {
        name: "PCE-1".to_string(),
        owner: "cashier@example.com".to_string(),
        company: "Wind Power LLC".to_string(),
        pos_profile: "POS-1".to_string(),
        user: "cashier@example.com".to_string(),
        pos_opening_entry: "POE-1".to_string(),
        posting_date: Some("2026-05-31".to_string()),
        posting_time: Some("09:00:00".to_string()),
        invoice_type: "POS Invoice".to_string(),
        ..Default::default()
    }
}

#[test]
fn pos_closing_entry_matches_erpnext_metadata_and_hooks() {
    assert_eq!(POSClosingEntry::DOCTYPE, "POS Closing Entry");
    assert_eq!(POSClosingEntry::MODULE, "Accounts");
    assert_eq!(POSClosingEntry::AUTONAME, "POS-CLO-.YYYY.-.#####");
    assert_eq!(POSClosingEntry::FIELD_ORDER.len(), 31);
    assert_eq!(
        &POSClosingEntry::FIELD_ORDER[..8],
        [
            "period_details_section",
            "period_start_date",
            "period_end_date",
            "column_break_3",
            "posting_date",
            "posting_time",
            "pos_opening_entry",
            "status",
        ]
    );

    let fields = POSClosingEntry::fields();
    assert!(fields.contains(
        &FieldSpec::datetime("period_start_date", "Period Start Date")
            .fetch_from("pos_opening_entry.period_start_date")
            .in_list_view()
            .read_only()
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("pos_opening_entry", "POS Opening Entry")
            .options("POS Opening Entry")
            .print_hide()
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::select("status", "Status")
            .options("Draft\nSubmitted\nQueued\nFailed\nCancelled")
            .default("Draft")
            .hidden()
            .read_only()
            .print_hide()
            .allow_on_submit()
    ));
    assert!(fields.contains(
        &FieldSpec::table("pos_invoices", "POS Transactions")
            .options("POS Invoice Reference")
            .read_only()
            .print_hide()
    ));

    let controller = POSClosingEntry::default();
    assert_eq!(controller.doctype(), "POS Closing Entry");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(
        controller.custom_hooks(),
        &[
            "validate",
            "on_submit",
            "before_cancel",
            "on_cancel",
            "retry"
        ]
    );
}

#[test]
fn pos_closing_entry_validate_matches_erpnext_invoice_mode_rules() {
    let mut entry = base_entry();
    entry.pos_invoices = vec![
        POSInvoiceReferenceRow {
            idx: 1,
            pos_invoice: "POS-INV-1".to_string(),
            ..Default::default()
        },
        POSInvoiceReferenceRow {
            idx: 2,
            pos_invoice: "POS-INV-1".to_string(),
            ..Default::default()
        },
    ];

    assert_eq!(
        entry.validate(
            "Open",
            "POS Invoice",
            "2026-06-02",
            "10:11:12",
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Err(POSClosingEntryError::DuplicatePosInvoices(vec![
            "POS-INV-1 is added multiple times on rows: [1, 2]".to_string()
        ]))
    );

    let mut sales_mode = base_entry();
    sales_mode.invoice_type = "Sales Invoice".to_string();
    sales_mode.pos_invoices = vec![POSInvoiceReferenceRow {
        idx: 1,
        pos_invoice: "POS-INV-1".to_string(),
        ..Default::default()
    }];
    assert_eq!(
        sales_mode.validate(
            "Open",
            "Sales Invoice",
            "2026-06-02",
            "10:11:12",
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Err(POSClosingEntryError::PosInvoicesNotAllowedForSalesInvoiceMode)
    );

    let mut invalid = base_entry();
    invalid.pos_invoices = vec![POSInvoiceReferenceRow {
        idx: 4,
        pos_invoice: "POS-INV-2".to_string(),
        ..Default::default()
    }];
    assert_eq!(
        invalid.validate(
            "Open",
            "POS Invoice",
            "2026-06-02",
            "10:11:12",
            &BTreeMap::from([(
                "POS-INV-2".to_string(),
                POSInvoiceRecord {
                    consolidated_invoice: None,
                    pos_profile: "OTHER".to_string(),
                    docstatus: 0,
                    owner: "other@example.com".to_string(),
                },
            )]),
            &BTreeMap::new(),
        ),
        Err(POSClosingEntryError::InvalidPosInvoices(vec![
            "Row #4: POS Profile doesn't match POS-1".to_string(),
            "Row #4: POS Invoice is not submitted".to_string(),
            "Row #4: POS Invoice isn't created by user cashier@example.com".to_string(),
        ]))
    );

    let mut valid = base_entry();
    valid.pos_invoices = vec![POSInvoiceReferenceRow {
        idx: 1,
        pos_invoice: "POS-INV-1".to_string(),
        ..Default::default()
    }];
    assert!(valid
        .validate(
            "Open",
            "POS Invoice",
            "2026-06-02",
            "10:11:12",
            &BTreeMap::from([(
                "POS-INV-1".to_string(),
                POSInvoiceRecord {
                    consolidated_invoice: None,
                    pos_profile: "POS-1".to_string(),
                    docstatus: 1,
                    owner: "cashier@example.com".to_string(),
                },
            )]),
            &BTreeMap::new(),
        )
        .is_ok());
    assert_eq!(valid.posting_date.as_deref(), Some("2026-06-02"));
    assert_eq!(valid.posting_time.as_deref(), Some("10:11:12"));
}

#[test]
fn pos_closing_entry_validate_sales_invoice_rows_match_erpnext() {
    let mut entry = base_entry();
    entry.invoice_type = "Sales Invoice".to_string();
    entry.sales_invoices = vec![SalesInvoiceReferenceRow {
        idx: 3,
        sales_invoice: "SINV-1".to_string(),
        ..Default::default()
    }];

    assert_eq!(
        entry.validate(
            "Closed",
            "Sales Invoice",
            "2026-06-02",
            "10:11:12",
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Err(POSClosingEntryError::InvalidOpeningEntry)
    );

    assert_eq!(
        entry.validate(
            "Open",
            "Sales Invoice",
            "2026-06-02",
            "10:11:12",
            &BTreeMap::new(),
            &BTreeMap::from([(
                "SINV-1".to_string(),
                SalesInvoiceRecord {
                    pos_profile: "POS-1".to_string(),
                    docstatus: 1,
                    is_pos: false,
                    owner: "other@example.com".to_string(),
                    is_created_using_pos: false,
                    is_consolidated: false,
                    pos_closing_entry: None,
                },
            )]),
        ),
        Err(POSClosingEntryError::InvalidSalesInvoices(vec![
            "Row #3: Sales Invoice does not have Payments".to_string(),
            "Row #3: Sales Invoice is not created using POS".to_string(),
            "Row #3: Sales Invoice isn't created by user cashier@example.com".to_string(),
        ]))
    );
}

#[test]
fn pos_closing_entry_lifecycle_and_query_plans_match_erpnext() {
    let entry = base_entry();
    assert_eq!(entry.on_submit_plan().consolidate, true);
    assert_eq!(entry.on_submit_plan().realtime_event, "poe_POE-1");
    assert_eq!(
        entry.update_sales_invoices_closing_entry_plan(false),
        Vec::<(String, Option<String>)>::new()
    );
    assert_eq!(
        entry.before_cancel_plan(true),
        Err(POSClosingEntryError::CannotCancelOpenProfile)
    );
    assert_eq!(entry.on_cancel_plan().unconsolidate, true);
    assert_eq!(entry.retry_plan().consolidate, true);
    assert_eq!(
        entry.update_opening_entry_plan(false),
        ("POE-1".to_string(), Some("PCE-1".to_string()))
    );
    assert_eq!(
        entry.update_opening_entry_plan(true),
        ("POE-1".to_string(), None)
    );

    assert_eq!(
        build_invoice_query_plan(
            "POS Invoice",
            "cashier@example.com",
            "POS-1",
            "2026-06-01 00:00:00",
            "2026-06-02 00:00:00",
        ),
        BuildInvoiceQueryPlan {
            invoice_doctype: "POS Invoice".to_string(),
            user: "cashier@example.com".to_string(),
            pos_profile: "POS-1".to_string(),
            start: "2026-06-01 00:00:00".to_string(),
            end: "2026-06-02 00:00:00".to_string(),
            require_unconsolidated_pos_invoice: true,
            require_created_using_pos: false,
            require_no_pos_closing_entry: false,
        }
    );
}

#[test]
fn pos_closing_entry_payment_tax_and_opening_entry_helpers_match_erpnext() {
    let invoices = vec![
        InvoiceCandidate {
            name: "POS-INV-1".to_string(),
            doctype: "POS Invoice".to_string(),
            customer: "Ada".to_string(),
            posting_date: "2026-06-01".to_string(),
            grand_total: 100.0,
            net_total: 80.0,
            total_qty: 2.0,
            total_taxes_and_charges: 20.0,
            change_amount: 5.0,
            account_for_change_amount: Some("Cash - WP".to_string()),
            is_return: false,
            return_against: None,
            timestamp: "2026-06-01 09:00:00".to_string(),
        },
        InvoiceCandidate {
            name: "SINV-1".to_string(),
            doctype: "Sales Invoice".to_string(),
            customer: "Ben".to_string(),
            posting_date: "2026-06-01".to_string(),
            grand_total: 50.0,
            net_total: 45.0,
            total_qty: 1.0,
            total_taxes_and_charges: 5.0,
            change_amount: 0.0,
            account_for_change_amount: None,
            is_return: false,
            return_against: None,
            timestamp: "2026-06-01 08:00:00".to_string(),
        },
    ];
    let payments = get_payments(
        &invoices,
        &[InvoicePayment {
            parent: "POS-INV-1".to_string(),
            parenttype: "POS Invoice".to_string(),
            mode_of_payment: "Cash".to_string(),
            account: "Cash - WP".to_string(),
            amount: 100.0,
        }],
    );
    assert_eq!(payments[0].amount, 95.0);

    let taxes = get_taxes(
        &invoices,
        &[InvoiceTax {
            parent: "POS-INV-1".to_string(),
            parenttype: "POS Invoice".to_string(),
            account_head: "VAT - WP".to_string(),
            tax_amount_after_discount_amount: 20.0,
        }],
    );
    assert_eq!(taxes[0].tax_amount, 20.0);

    let closing = make_closing_entry_from_opening(
        &OpeningEntrySnapshot {
            name: "POE-1".to_string(),
            period_start_date: "2026-06-01 00:00:00".to_string(),
            period_end_date: "2026-06-02 00:00:00".to_string(),
            pos_profile: "POS-1".to_string(),
            user: "cashier@example.com".to_string(),
            company: "Wind Power LLC".to_string(),
        },
        "2026-06-02 12:00:00",
        invoices,
        payments,
        taxes,
    );
    assert_eq!(closing.pos_opening_entry, "POE-1");
    assert_eq!(
        closing.period_end_date.as_deref(),
        Some("2026-06-02 12:00:00")
    );
    assert_eq!(closing.grand_total, 150.0);
    assert_eq!(closing.net_total, 125.0);
    assert_eq!(closing.total_quantity, 3.0);
    assert_eq!(closing.total_taxes_and_charges, 25.0);
    assert_eq!(closing.pos_invoices.len(), 1);
    assert_eq!(closing.sales_invoices.len(), 1);
    assert_eq!(closing.payment_reconciliation[0].expected_amount, 95.0);
    assert_eq!(closing.taxes[0].amount, 20.0);

    assert_eq!(
        get_cashiers(&["cashier@example.com".to_string()]),
        vec!["cashier@example.com"]
    );
}

#[test]
fn pos_closing_entry_get_invoices_combines_sales_and_pos_modes_like_erpnext() {
    let sales = vec![InvoiceCandidate {
        name: "SINV-1".to_string(),
        doctype: "Sales Invoice".to_string(),
        customer: "Ada".to_string(),
        posting_date: "2026-06-01".to_string(),
        grand_total: 50.0,
        net_total: 45.0,
        total_qty: 1.0,
        total_taxes_and_charges: 5.0,
        timestamp: "2026-06-01 09:00:00".to_string(),
        ..Default::default()
    }];
    let pos = vec![InvoiceCandidate {
        name: "POS-1".to_string(),
        doctype: "POS Invoice".to_string(),
        customer: "Ben".to_string(),
        posting_date: "2026-06-01".to_string(),
        grand_total: 100.0,
        net_total: 90.0,
        total_qty: 2.0,
        total_taxes_and_charges: 10.0,
        timestamp: "2026-06-01 08:00:00".to_string(),
        ..Default::default()
    }];

    let sales_only = get_invoices("Sales Invoice", sales.clone(), pos.clone(), &[], &[]);
    assert_eq!(
        sales_only
            .invoices
            .iter()
            .map(|row| row.name.as_str())
            .collect::<Vec<_>>(),
        vec!["SINV-1"]
    );

    let all = get_invoices("POS Invoice", sales, pos, &[], &[]);
    assert_eq!(
        all.invoices
            .iter()
            .map(|row| row.name.as_str())
            .collect::<Vec<_>>(),
        vec!["POS-1", "SINV-1"]
    );
}
