use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::accounts::report::sales_register::sales_register::{
    execute, get_account_columns, get_invoice_cc_wh_map, get_invoice_income_map,
    get_invoice_so_dn_map, get_invoice_tax_map, get_mode_of_payments, AdditionalColumn,
    AdvanceTaxCharge, OpeningRow, PartyDetail, PaymentEntry, ReportColumn, SalesInvoice,
    SalesInvoiceItem, SalesRegisterFilters, SalesRegisterInput, SalesTaxCharge,
};

fn filters(include_payments: bool) -> SalesRegisterFilters {
    SalesRegisterFilters {
        company: "Acme".to_string(),
        customer: if include_payments {
            Some("CUST-1".to_string())
        } else {
            None
        },
        include_payments,
        from_date: Some("2026-01-01".to_string()),
        to_date: Some("2026-12-31".to_string()),
        owner: None,
        mode_of_payment: None,
    }
}

fn invoice(name: &str, posting_date: &str, customer: &str, grand_total: f64) -> SalesInvoice {
    SalesInvoice {
        doctype: "Sales Invoice".to_string(),
        name: name.to_string(),
        posting_date: posting_date.to_string(),
        debit_to: "Debtors".to_string(),
        project: Some("PRJ".to_string()),
        customer: customer.to_string(),
        customer_name: format!("{customer} Name"),
        owner: "owner@example.com".to_string(),
        remarks: "remark".to_string(),
        base_net_total: grand_total - 10.0,
        base_grand_total: grand_total,
        base_rounded_total: grand_total,
        outstanding_amount: grand_total / 2.0,
        is_internal_customer: false,
        unrealized_profit_loss_account: None,
        represents_company: None,
        company: "Acme".to_string(),
        extra: BTreeMap::from([("territory".to_string(), json!("Central"))]),
    }
}

fn item(parent: &str, income_account: &str, amount: f64) -> SalesInvoiceItem {
    SalesInvoiceItem {
        parent: parent.to_string(),
        docstatus: 1,
        income_account: income_account.to_string(),
        base_net_amount: amount,
        sales_order: None,
        delivery_note: None,
        so_detail: None,
        cost_center: None,
        warehouse: None,
    }
}

fn input() -> SalesRegisterInput {
    let mut inv1 = invoice("SINV-1", "2026-02-01", "CUST-1", 120.0);
    inv1.is_internal_customer = true;
    inv1.represents_company = Some("Acme".to_string());
    let mut inv2 = invoice("SINV-2", "2026-01-15", "CUST-2", 80.0);
    inv2.debit_to = "Debtors 2".to_string();

    SalesRegisterInput {
        company_currency: "USD".to_string(),
        invoices: vec![inv1, inv2],
        invoice_items: vec![
            {
                let mut row = item("SINV-1", "Sales", 90.0);
                row.sales_order = Some("SO-1".to_string());
                row.so_detail = Some("SO-DETAIL-1".to_string());
                row.cost_center = Some("Main".to_string());
                row.warehouse = Some("Stores".to_string());
                row
            },
            {
                let mut row = item("SINV-2", "Services", 70.0);
                row.delivery_note = Some("DN-2".to_string());
                row.cost_center = Some("Branch".to_string());
                row
            },
        ],
        sales_taxes: vec![
            SalesTaxCharge::new("SINV-1", "VAT", 10.0),
            SalesTaxCharge::new("SINV-2", "Sales", 5.0),
        ],
        advance_taxes: vec![AdvanceTaxCharge::new("PAY-1", "WHT", -2.0)],
        payment_entries: vec![PaymentEntry::new(
            "Payment Entry",
            "PAY-1",
            "2026-02-10",
            "Debtors",
            "CUST-1",
            "CUST-1 Name",
            0.0,
            30.0,
        )],
        opening_row: Some(OpeningRow {
            account: "Opening".to_string(),
            debit: 50.0,
            credit: 10.0,
            balance: 40.0,
        }),
        party_details: vec![
            PartyDetail::customer("CUST-1", "Retail", "Central", "TIN-1"),
            PartyDetail::customer("CUST-2", "Wholesale", "South", "TIN-2"),
        ],
        invoice_payments: vec![
            ("SINV-1".to_string(), "Cash".to_string()),
            ("SINV-1".to_string(), "Card".to_string()),
        ],
        so_delivery_notes: BTreeMap::from([(
            "SO-DETAIL-1".to_string(),
            vec!["DN-FROM-SO".to_string()],
        )]),
    }
}

#[test]
fn sales_register_dynamic_account_columns_match_erpnext_shape() {
    let input = input();
    let (columns, accounts) = get_account_columns(&input, false);

    assert_eq!(accounts.income_accounts, vec!["Sales", "Services"]);
    assert_eq!(accounts.tax_accounts, vec!["Sales", "VAT"]);
    assert_eq!(
        accounts.unrealized_profit_loss_accounts,
        Vec::<String>::new()
    );
    assert_eq!(columns.income_columns[0].fieldname, "sales");
    assert_eq!(columns.income_columns[1].fieldname, "services");
    assert_eq!(columns.tax_columns[0].fieldname, "vat");

    let (payment_columns, payment_accounts) = get_account_columns(&input, true);
    assert!(payment_accounts.tax_accounts.contains(&"WHT".to_string()));
    assert!(payment_columns
        .tax_columns
        .iter()
        .any(|column| column.fieldname == "wht"));
}

#[test]
fn sales_register_maps_income_taxes_links_and_modes_like_python_helpers() {
    let input = input();
    let invoices = input.invoices.clone();
    let income_map = get_invoice_income_map(&invoices, &input.invoice_items);
    assert_eq!(income_map["SINV-1"]["Sales"], 90.0);

    let (income_map, tax_map) =
        get_invoice_tax_map(&invoices, income_map, &["Sales".to_string()], &input, false);
    assert_eq!(income_map["SINV-2"]["Sales"], 5.0);
    assert_eq!(tax_map["SINV-1"]["VAT"], 10.0);

    let so_dn = get_invoice_so_dn_map(&invoices, &input.invoice_items, &input.so_delivery_notes);
    assert_eq!(so_dn["SINV-1"]["sales_order"], vec!["SO-1"]);
    assert_eq!(so_dn["SINV-1"]["delivery_note"], vec!["DN-FROM-SO"]);
    assert_eq!(so_dn["SINV-2"]["delivery_note"], vec!["DN-2"]);

    let cc_wh = get_invoice_cc_wh_map(&invoices, &input.invoice_items);
    assert_eq!(cc_wh["SINV-1"]["cost_center"], vec!["Main"]);
    assert_eq!(cc_wh["SINV-1"]["warehouse"], vec!["Stores"]);

    let mode_map = get_mode_of_payments(&input.invoice_payments);
    assert_eq!(mode_map["SINV-1"], vec!["Cash", "Card"]);
}

#[test]
fn sales_register_columns_include_additional_columns_only_without_payments() {
    let input = input();
    let additional = vec![AdditionalColumn::data("Territory", "territory", 80)];

    let (columns, _, _, _) =
        tokio_erp::erpnext::accounts::report::sales_register::sales_register::get_columns(
            &input,
            &additional,
            false,
        );
    assert!(columns.iter().any(|column| column.fieldname == "territory"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "customer_group"));
    assert!(columns.iter().any(|column| column.fieldname == "tax_total"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "grand_total"));

    let (payment_columns, _, _, _) =
        tokio_erp::erpnext::accounts::report::sales_register::sales_register::get_columns(
            &input,
            &additional,
            true,
        );
    assert!(!payment_columns
        .iter()
        .any(|column| column.fieldname == "territory"));
    assert!(payment_columns
        .iter()
        .any(|column| column.fieldname == "balance"));
    assert!(!payment_columns
        .iter()
        .any(|column| column.fieldname == "grand_total"));
    assert_eq!(
        payment_columns[0],
        ReportColumn::data("Voucher Type", "voucher_type", 120)
    );
}

#[test]
fn sales_register_execute_builds_rows_sorts_by_posting_date_and_maps_amounts() {
    let report = execute(
        filters(false),
        input(),
        vec![AdditionalColumn::data("Territory", "territory", 80)],
    )
    .expect("report");

    assert!(!report.skip_total_row);
    assert_eq!(report.rows[0].voucher_no.as_deref(), Some("SINV-2"));
    assert_eq!(report.rows[0].values["services"], json!(70.0));
    assert_eq!(report.rows[0].values["sales"], json!(5.0));
    assert_eq!(report.rows[0].values["net_total"], json!(75.0));
    assert_eq!(report.rows[0].values["delivery_note"], json!("DN-2"));
    assert_eq!(report.rows[1].voucher_no.as_deref(), Some("SINV-1"));
    assert_eq!(report.rows[1].values["sales"], json!(0.0));
    assert_eq!(report.rows[1].values["vat"], json!(10.0));
    assert_eq!(
        report.rows[1].values["mode_of_payment"],
        json!("Cash, Card")
    );
    assert_eq!(report.rows[1].values["sales_order"], json!("SO-1"));
    assert_eq!(report.rows[1].values["delivery_note"], json!("DN-FROM-SO"));
}

#[test]
fn sales_register_execute_include_payments_requires_customer_and_calculates_running_balance() {
    let err = execute(filters(true).without_customer(), input(), Vec::new()).unwrap_err();
    assert_eq!(err, "Please select a customer for fetching payments.");

    let report = execute(filters(true), input(), Vec::new()).expect("report");
    assert!(report.skip_total_row);
    assert_eq!(
        report.rows[0].values["receivable_account"],
        json!("Opening")
    );
    assert_eq!(report.rows[0].values["balance"], json!(40.0));
    assert_eq!(report.rows[1].voucher_no.as_deref(), Some("SINV-1"));
    assert_eq!(report.rows[1].values["balance"], json!(160.0));
    assert_eq!(report.rows[2].voucher_no.as_deref(), Some("PAY-1"));
    assert_eq!(report.rows[2].values["balance"], json!(130.0));
}
