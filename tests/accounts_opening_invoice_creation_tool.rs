use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::doctype::opening_invoice_creation_tool::opening_invoice_creation_tool::{
    get_opening_invoice_summary, get_temporary_opening_account, publish_payload, start_import_plan,
    CompanyCurrency, CompanyDetails, ImportAttempt, InvoiceType, OpeningInvoiceAggregate,
    OpeningInvoiceCreationError, OpeningInvoiceCreationTool, OpeningInvoiceRow, PartyDefaults,
};

fn sales_tool() -> OpeningInvoiceCreationTool {
    OpeningInvoiceCreationTool {
        name: "OICT-1".to_string(),
        company: Some("Wind Power LLC".to_string()),
        cost_center: Some("Main - WP".to_string()),
        create_missing_party: false,
        invoice_type: InvoiceType::Sales,
        ..Default::default()
    }
}

#[test]
fn opening_invoice_set_missing_and_validate_party_rules_match_erpnext() {
    let mut tool = sales_tool();
    tool.create_missing_party = true;
    let mut row = OpeningInvoiceRow {
        idx: 3,
        party_name: Some("Ada".to_string()),
        outstanding_amount: 120.0,
        ..Default::default()
    };

    tool.set_missing_values(&mut row, Some("Temporary Opening - WP"), "2026-06-01");
    assert_eq!(row.qty, 1.0);
    assert_eq!(row.party_type.as_deref(), Some("Customer"));
    assert_eq!(row.item_name.as_deref(), Some("Opening Invoice Item"));
    assert_eq!(row.posting_date.as_deref(), Some("2026-06-01"));
    assert_eq!(row.due_date.as_deref(), Some("2026-06-01"));

    let party_plan = tool
        .validate_mandatory_invoice_fields(&mut row, &BTreeSet::new(), Some("All Supplier Groups"))
        .unwrap()
        .unwrap();
    assert_eq!(party_plan.party_type, "Customer");
    assert_eq!(party_plan.party_name, "Ada");
    assert_eq!(row.party.as_deref(), Some("Ada"));

    let mut missing = OpeningInvoiceRow {
        idx: 4,
        party_type: Some("Customer".to_string()),
        outstanding_amount: 10.0,
        temporary_opening_account: Some("Temporary Opening - WP".to_string()),
        ..Default::default()
    };
    assert_eq!(
        sales_tool().validate_mandatory_invoice_fields(&mut missing, &BTreeSet::new(), None),
        Err(OpeningInvoiceCreationError::MissingPartyId { idx: 4 })
    );
}

#[test]
fn opening_invoice_dict_uses_customer_supplier_account_fields_and_dimensions() {
    let mut row = OpeningInvoiceRow {
        idx: 1,
        party: Some("CUST-1".to_string()),
        party_type: Some("Customer".to_string()),
        item_name: Some("Opening Invoice Item".to_string()),
        qty: 2.0,
        outstanding_amount: 50.0,
        temporary_opening_account: Some("Temporary Opening - WP".to_string()),
        posting_date: Some("2026-06-01".to_string()),
        due_date: Some("2026-06-30".to_string()),
        invoice_number: Some("INV-OLD-1".to_string()),
        dimensions: BTreeMap::from([("project".to_string(), "PROJ-1".to_string())]),
        ..Default::default()
    };

    let invoice = sales_tool()
        .get_invoice_dict(
            &mut row,
            &CompanyDetails {
                default_currency: Some("USD".to_string()),
                default_letter_head: Some("LH".to_string()),
                cost_center: Some("Company CC - WP".to_string()),
            },
            "Nos",
            &["project".to_string()],
        )
        .unwrap();

    assert_eq!(invoice.doctype, "Sales Invoice");
    assert_eq!(invoice.party_field, "customer");
    assert_eq!(invoice.party, "CUST-1");
    assert_eq!(
        invoice.items[0].income_account.as_deref(),
        Some("Temporary Opening - WP")
    );
    assert_eq!(invoice.items[0].rate, 25.0);
    assert_eq!(invoice.items[0].cost_center, "Company CC - WP");
    assert_eq!(invoice.dimensions["project"], "PROJ-1");
    assert_eq!(invoice.invoice_number.as_deref(), Some("INV-OLD-1"));

    let mut purchase_tool = sales_tool();
    purchase_tool.invoice_type = InvoiceType::Purchase;
    let mut purchase_row = row.clone();
    purchase_row.party = Some("SUP-1".to_string());
    purchase_row.party_type = Some("Supplier".to_string());
    purchase_row.supplier_invoice_date = Some("2026-05-20".to_string());
    let purchase = purchase_tool
        .get_invoice_dict(
            &mut purchase_row,
            &CompanyDetails {
                cost_center: Some("Company CC - WP".to_string()),
                ..Default::default()
            },
            "Nos",
            &Vec::<String>::new(),
        )
        .unwrap();
    assert_eq!(purchase.doctype, "Purchase Invoice");
    assert_eq!(purchase.party_field, "supplier");
    assert_eq!(
        purchase.items[0].expense_account.as_deref(),
        Some("Temporary Opening - WP")
    );
    assert_eq!(purchase.bill_date.as_deref(), Some("2026-05-20"));

    let mut no_cost_center = row;
    assert_eq!(
        sales_tool().get_invoice_dict(
            &mut no_cost_center,
            &CompanyDetails::default(),
            "Nos",
            &Vec::<String>::new(),
        ),
        Err(OpeningInvoiceCreationError::MissingCompanyCostCenter {
            company: "Wind Power LLC".to_string(),
        })
    );
}

#[test]
fn opening_invoice_get_invoices_and_make_invoices_follow_sync_async_thresholds() {
    let mut tool = sales_tool();
    tool.invoices = vec![OpeningInvoiceRow {
        idx: 1,
        party: Some("CUST-1".to_string()),
        outstanding_amount: 10.0,
        ..Default::default()
    }];

    let invoices = tool
        .get_invoices(
            Some("Temporary Opening - WP"),
            "2026-06-01",
            &BTreeSet::from([("Customer".to_string(), "CUST-1".to_string())]),
            &BTreeMap::from([(
                ("Customer".to_string(), "CUST-1".to_string()),
                PartyDefaults {
                    default_currency: Some("EUR".to_string()),
                },
            )]),
            &CompanyDetails {
                default_currency: Some("USD".to_string()),
                default_letter_head: Some("LH".to_string()),
                cost_center: Some("Main - WP".to_string()),
            },
            "Nos",
            &[],
            None,
        )
        .unwrap();
    assert_eq!(invoices[0].currency.as_deref(), Some("EUR"));
    assert_eq!(invoices[0].letter_head.as_deref(), Some("LH"));

    assert_eq!(
        tool.make_invoices_plan(49, false, false, false)
            .unwrap()
            .mode,
        "sync"
    );
    assert_eq!(
        tool.make_invoices_plan(50, false, true, false)
            .unwrap()
            .mode,
        "enqueue"
    );
    assert_eq!(
        tool.make_invoices_plan(50, false, false, true)
            .unwrap()
            .mode,
        "noop"
    );
    assert_eq!(
        tool.make_invoices_plan(50, true, false, false),
        Err(OpeningInvoiceCreationError::SchedulerInactive)
    );
}

#[test]
fn opening_invoice_import_publish_and_temporary_account_helpers_match_source() {
    let import = start_import_plan(&[
        ImportAttempt {
            doctype: "Sales Invoice".to_string(),
            invoice_number: Some("OLD-1".to_string()),
            generated_name: "OLD-1".to_string(),
            succeeds: true,
        },
        ImportAttempt {
            doctype: "Purchase Invoice".to_string(),
            generated_name: "PINV-1".to_string(),
            succeeds: false,
            ..Default::default()
        },
    ]);
    assert_eq!(import.names, vec!["OLD-1"]);
    assert_eq!(import.errors, 1);

    let payload = publish_payload(0, 2, "Sales Invoice", "user@example.com");
    assert_eq!(payload.count, 1);
    assert_eq!(payload.total, 2);
    assert_eq!(payload.message, "Creating 1 out of 2 Sales Invoice");
    assert_eq!(payload.user, "user@example.com");

    assert_eq!(
        get_temporary_opening_account(
            Some("Wind Power LLC"),
            &["Temporary Opening - WP".to_string()]
        ),
        Ok(Some("Temporary Opening - WP".to_string()))
    );
    assert_eq!(get_temporary_opening_account(None, &[]), Ok(None));
    assert_eq!(
        get_temporary_opening_account(Some("Wind Power LLC"), &[]),
        Err(OpeningInvoiceCreationError::MissingTemporaryOpeningAccount)
    );
}

#[test]
fn opening_invoice_summary_groups_company_currency_and_max_counts() {
    let (summary, max_count) = get_opening_invoice_summary(
        &[CompanyCurrency {
            company: "A&B LLC".to_string(),
            currency: "USD".to_string(),
        }],
        &[OpeningInvoiceAggregate {
            company: "A&B LLC".to_string(),
            total_invoices: 2,
            outstanding_amount: 90.0,
            paid_amount: 10.0,
        }],
        &[OpeningInvoiceAggregate {
            company: "A&B LLC".to_string(),
            total_invoices: 1,
            outstanding_amount: 50.0,
            paid_amount: 0.0,
        }],
    );

    let summary = summary.unwrap();
    let company = summary.get("A&amp;B LLC").unwrap();
    assert_eq!(company.currency.as_deref(), Some("USD"));
    assert_eq!(company.sales_invoice.as_ref().unwrap().total_invoices, 2);
    assert_eq!(
        company
            .purchase_invoice
            .as_ref()
            .unwrap()
            .outstanding_amount,
        50.0
    );

    let max_count = max_count.unwrap();
    assert_eq!(max_count["Sales Invoice"].max_paid, 10.0);
    assert_eq!(max_count["Sales Invoice"].max_due, 90.0);
    assert_eq!(max_count["Purchase Invoice"].max_due, 50.0);

    assert_eq!(get_opening_invoice_summary(&[], &[], &[]), (None, None));
}
