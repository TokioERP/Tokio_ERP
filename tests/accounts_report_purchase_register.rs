use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::accounts::report::purchase_register::purchase_register::{
    execute, get_account_columns, get_invoice_expense_map, get_invoice_po_pr_map,
    get_invoice_tax_map, AdditionalColumn, AdvanceTaxCharge, OpeningRow, PartyDetail, PaymentEntry,
    PurchaseInvoice, PurchaseInvoiceItem, PurchaseRegisterFilters, PurchaseRegisterInput,
    PurchaseTaxCharge, ReportColumn,
};

fn filters(include_payments: bool) -> PurchaseRegisterFilters {
    PurchaseRegisterFilters {
        company: "Acme".to_string(),
        supplier: if include_payments {
            Some("SUP-1".to_string())
        } else {
            None
        },
        supplier_group: None,
        include_payments,
        from_date: Some("2026-01-01".to_string()),
        to_date: Some("2026-12-31".to_string()),
        mode_of_payment: None,
    }
}

fn invoice(name: &str, posting_date: &str, supplier: &str, grand_total: f64) -> PurchaseInvoice {
    PurchaseInvoice {
        doctype: "Purchase Invoice".to_string(),
        name: name.to_string(),
        posting_date: posting_date.to_string(),
        credit_to: "Creditors".to_string(),
        supplier: supplier.to_string(),
        supplier_name: format!("{supplier} Name"),
        tax_id: format!("TIN-{supplier}"),
        bill_no: Some(format!("BILL-{name}")),
        bill_date: Some(posting_date.to_string()),
        remarks: "remark".to_string(),
        base_net_total: grand_total - 10.0,
        base_grand_total: grand_total,
        base_rounded_total: grand_total,
        outstanding_amount: grand_total / 2.0,
        mode_of_payment: Some("Cash".to_string()),
        is_internal_supplier: false,
        unrealized_profit_loss_account: None,
        represents_company: None,
        company: "Acme".to_string(),
        extra: BTreeMap::from([("branch".to_string(), json!("Tashkent"))]),
    }
}

fn item(parent: &str, expense_account: &str, amount: f64) -> PurchaseInvoiceItem {
    PurchaseInvoiceItem {
        parent: parent.to_string(),
        parenttype: "Purchase Invoice".to_string(),
        docstatus: 1,
        expense_account: expense_account.to_string(),
        base_net_amount: amount,
        purchase_order: None,
        purchase_receipt: None,
        po_detail: None,
        project: None,
    }
}

fn input() -> PurchaseRegisterInput {
    let mut inv1 = invoice("PINV-1", "2026-02-01", "SUP-1", 120.0);
    inv1.is_internal_supplier = true;
    inv1.unrealized_profit_loss_account = Some("Unrealized P/L".to_string());
    inv1.represents_company = Some("Acme".to_string());
    let mut inv2 = invoice("PINV-2", "2026-01-15", "SUP-2", 80.0);
    inv2.credit_to = "Creditors 2".to_string();

    PurchaseRegisterInput {
        company_currency: "USD".to_string(),
        invoices: vec![inv1, inv2],
        invoice_items: vec![
            {
                let mut row = item("PINV-1", "Expenses", 90.0);
                row.purchase_order = Some("PO-1".to_string());
                row.po_detail = Some("PO-DETAIL-1".to_string());
                row.project = Some("PRJ-1".to_string());
                row
            },
            {
                let mut row = item("PINV-2", "Services", 70.0);
                row.purchase_receipt = Some("PR-2".to_string());
                row.project = Some("PRJ-2".to_string());
                row
            },
        ],
        purchase_taxes: vec![
            PurchaseTaxCharge::new("PINV-1", "VAT", 10.0),
            PurchaseTaxCharge::new("PINV-2", "Services", 5.0),
        ],
        advance_taxes: vec![AdvanceTaxCharge::new("PAY-1", "WHT", -2.0)],
        payment_entries: vec![PaymentEntry::new(
            "Payment Entry",
            "PAY-1",
            "2026-02-10",
            "Creditors",
            "SUP-1",
            "SUP-1 Name",
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
            PartyDetail::supplier("SUP-1", "Services", "TIN-1"),
            PartyDetail::supplier("SUP-2", "Goods", "TIN-2"),
        ],
        po_purchase_receipts: BTreeMap::from([(
            "PO-DETAIL-1".to_string(),
            vec!["PR-FROM-PO".to_string()],
        )]),
    }
}

#[test]
fn purchase_register_dynamic_account_columns_match_erpnext_shape() {
    let input = input();
    let (columns, accounts) = get_account_columns(&input, false);

    assert_eq!(accounts.expense_accounts, vec!["Expenses", "Services"]);
    assert_eq!(accounts.tax_accounts, vec!["Services", "VAT"]);
    assert_eq!(
        accounts.unrealized_profit_loss_accounts,
        vec!["Unrealized P/L"]
    );
    assert_eq!(columns.expense_columns[0].fieldname, "expenses");
    assert_eq!(columns.expense_columns[1].fieldname, "services");
    assert_eq!(
        columns.unrealized_profit_loss_account_columns[0].fieldname,
        "unrealized_p_l"
    );
    assert_eq!(columns.tax_columns[0].fieldname, "vat");

    let (payment_columns, payment_accounts) = get_account_columns(&input, true);
    assert!(payment_accounts.tax_accounts.contains(&"WHT".to_string()));
    assert!(payment_columns
        .tax_columns
        .iter()
        .any(|column| column.fieldname == "wht"));
}

#[test]
fn purchase_register_maps_expenses_taxes_and_po_pr_projects_like_python_helpers() {
    let input = input();
    let invoices = input.invoices.clone();
    let expense_map = get_invoice_expense_map(&invoices, &input.invoice_items);
    assert_eq!(expense_map["PINV-1"]["Expenses"], 90.0);

    let (expense_map, tax_map) = get_invoice_tax_map(
        &invoices,
        expense_map,
        &["Services".to_string()],
        &input,
        false,
    );
    assert_eq!(expense_map["PINV-2"]["Services"], 75.0);
    assert_eq!(tax_map["PINV-1"]["VAT"], 10.0);

    let po_pr = get_invoice_po_pr_map(&invoices, &input.invoice_items, &input.po_purchase_receipts);
    assert_eq!(po_pr["PINV-1"]["purchase_order"], vec!["PO-1"]);
    assert_eq!(po_pr["PINV-1"]["purchase_receipt"], vec!["PR-FROM-PO"]);
    assert_eq!(po_pr["PINV-1"]["project"], vec!["PRJ-1"]);
    assert_eq!(po_pr["PINV-2"]["purchase_receipt"], vec!["PR-2"]);
}

#[test]
fn purchase_register_columns_include_additional_columns_only_without_payments() {
    let input = input();
    let additional = vec![AdditionalColumn::data("Branch", "branch", 80)];

    let (columns, _, _, _) =
        tokio_erp::erpnext::accounts::report::purchase_register::purchase_register::get_columns(
            &input,
            &additional,
            false,
        );
    assert!(columns.iter().any(|column| column.fieldname == "branch"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "supplier_group"));
    assert!(columns.iter().any(|column| column.fieldname == "total_tax"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "grand_total"));

    let (payment_columns, _, _, _) =
        tokio_erp::erpnext::accounts::report::purchase_register::purchase_register::get_columns(
            &input,
            &additional,
            true,
        );
    assert!(!payment_columns
        .iter()
        .any(|column| column.fieldname == "branch"));
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
fn purchase_register_execute_builds_rows_sorts_by_posting_date_and_maps_amounts() {
    let report = execute(
        filters(false),
        input(),
        vec![AdditionalColumn::data("Branch", "branch", 80)],
    )
    .expect("report");

    assert!(!report.skip_total_row);
    assert_eq!(report.rows[0].voucher_no.as_deref(), Some("PINV-2"));
    assert_eq!(report.rows[0].values["services"], json!(75.0));
    assert_eq!(report.rows[0].values["net_total"], json!(75.0));
    assert_eq!(report.rows[0].values["purchase_receipt"], json!("PR-2"));
    assert_eq!(report.rows[1].voucher_no.as_deref(), Some("PINV-1"));
    assert_eq!(report.rows[1].values["expenses"], json!(0.0));
    assert_eq!(report.rows[1].values["vat"], json!(10.0));
    assert_eq!(report.rows[1].values["unrealized_p_l"], json!(110.0));
    assert_eq!(report.rows[1].values["purchase_order"], json!("PO-1"));
    assert_eq!(
        report.rows[1].values["purchase_receipt"],
        json!("PR-FROM-PO")
    );
    assert_eq!(report.rows[1].values["project"], json!("PRJ-1"));
}

#[test]
fn purchase_register_execute_include_payments_requires_supplier_and_calculates_running_balance() {
    let err = execute(filters(true).without_supplier(), input(), Vec::new()).unwrap_err();
    assert_eq!(err, "Please select a supplier for fetching payments.");

    let report = execute(filters(true), input(), Vec::new()).expect("report");
    assert!(report.skip_total_row);
    assert_eq!(report.rows[0].values["payable_account"], json!("Opening"));
    assert_eq!(report.rows[0].values["balance"], json!(40.0));
    assert_eq!(report.rows[1].voucher_no.as_deref(), Some("PINV-1"));
    assert_eq!(report.rows[1].values["balance"], json!(160.0));
    assert_eq!(report.rows[2].voucher_no.as_deref(), Some("PAY-1"));
    assert_eq!(report.rows[2].values["balance"], json!(130.0));
}
