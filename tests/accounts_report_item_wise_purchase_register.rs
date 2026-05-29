use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::accounts::report::item_wise_purchase_register::item_wise_purchase_register::{
    execute, get_columns, get_items, get_purchase_receipts_against_purchase_order,
    AdditionalColumn, ItemTaxDetail, ItemWisePurchaseFilters, ItemWisePurchaseInput,
    PurchaseInvoiceItemRow, ReportColumn,
};

fn filters() -> ItemWisePurchaseFilters {
    ItemWisePurchaseFilters {
        company: Some("Acme".to_string()),
        supplier: None,
        mode_of_payment: None,
        from_date: Some("2026-01-01".to_string()),
        to_date: Some("2026-12-31".to_string()),
        item_code: None,
        item_group: None,
        group_by: None,
    }
}

fn row(
    name: &str,
    parent: &str,
    item_code: &str,
    supplier: &str,
    amount: f64,
) -> PurchaseInvoiceItemRow {
    PurchaseInvoiceItemRow {
        name: name.to_string(),
        parent: parent.to_string(),
        posting_date: "2026-02-01".to_string(),
        credit_to: "Creditors".to_string(),
        company: "Acme".to_string(),
        supplier: supplier.to_string(),
        supplier_name: format!("{supplier} Name"),
        unrealized_profit_loss_account: None,
        item_code: item_code.to_string(),
        description: format!("{item_code} description"),
        pi_item_name: Some(format!("{item_code} PI")),
        pi_item_group: Some("Products".to_string()),
        i_item_name: Some(format!("{item_code} Item")),
        i_item_group: Some("Fallback Group".to_string()),
        project: Some("PRJ".to_string()),
        purchase_order: Some("PO-1".to_string()),
        purchase_receipt: None,
        po_detail: Some("PO-DETAIL-1".to_string()),
        expense_account: Some("Expenses".to_string()),
        stock_qty: 2.0,
        stock_uom: "Nos".to_string(),
        base_net_amount: amount,
        mode_of_payment: Some("Cash".to_string()),
        docstatus: 1,
        parenttype: "Purchase Invoice".to_string(),
        extra: BTreeMap::from([("branch".to_string(), json!("Tashkent"))]),
    }
}

fn input() -> ItemWisePurchaseInput {
    ItemWisePurchaseInput {
        company_currency: "USD".to_string(),
        items: vec![row("PII-1", "PINV-1", "ITEM-1", "SUP-1", 100.0), {
            let mut r = row("PII-2", "PINV-2", "ITEM-2", "SUP-2", 80.0);
            r.posting_date = "2026-01-15".to_string();
            r.purchase_receipt = Some("PR-2".to_string());
            r.po_detail = None;
            r.unrealized_profit_loss_account = Some("Unrealized".to_string());
            r.expense_account = None;
            r.stock_qty = 0.0;
            r
        }],
        item_taxes: vec![
            ItemTaxDetail::new("PII-1", "VAT", 10.0, 5.0, false),
            ItemTaxDetail::new("PII-1", "Freight", 3.0, 0.0, true),
            ItemTaxDetail::new("PII-2", "VAT", 8.0, 10.0, false),
        ],
        company_stock_received_but_not_billed: BTreeMap::from([(
            "Acme".to_string(),
            "Stock Received But Not Billed".to_string(),
        )]),
        purchase_receipts_by_po_detail: BTreeMap::from([(
            "PO-DETAIL-1".to_string(),
            vec!["PR-FROM-PO".to_string()],
        )]),
    }
}

#[test]
fn item_wise_purchase_columns_follow_group_by_rules_and_dynamic_tax_columns() {
    let input = input();
    let additional = vec![AdditionalColumn::data("Branch", "branch", 80)];
    let columns = get_columns(
        &additional,
        &filters(),
        &["freight".to_string(), "vat".to_string()],
    );

    assert_eq!(
        columns[0],
        ReportColumn::link("Item Code", "item_code", "Item", 120)
    );
    assert!(columns.iter().any(|column| column.fieldname == "supplier"));
    assert!(columns.iter().any(|column| column.fieldname == "branch"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "freight_rate"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "vat_amount"));
    assert!(columns.iter().any(|column| column.fieldname == "total_tax"));

    let mut by_item = filters();
    by_item.group_by = Some("Item".to_string());
    let grouped = get_columns(&additional, &by_item, &[]);
    assert!(!grouped.iter().any(|column| column.fieldname == "item_code"));
    assert!(grouped
        .iter()
        .any(|column| column.fieldname == "percent_gt"));

    assert_eq!(
        get_purchase_receipts_against_purchase_order(
            &input.items,
            &input.purchase_receipts_by_po_detail
        )["PO-DETAIL-1"],
        vec!["PR-FROM-PO"]
    );
}

#[test]
fn item_wise_purchase_get_items_filters_and_orders_like_erpnext() {
    let mut f = filters();
    f.supplier = Some("SUP-1".to_string());
    let rows = get_items(&f, &input());

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "PII-1");

    let all = get_items(&filters(), &input());
    assert_eq!(all[0].parent, "PINV-1");
    assert_eq!(all[1].parent, "PINV-2");
}

#[test]
fn item_wise_purchase_execute_maps_item_rows_taxes_and_receipt_fallbacks() {
    let report = execute(
        filters(),
        input(),
        vec![AdditionalColumn::data("Branch", "branch", 80)],
    )
    .expect("report");

    assert!(!report.skip_total_row);
    assert_eq!(report.rows[0].values["item_name"], json!("ITEM-1 PI"));
    assert_eq!(
        report.rows[0].values["purchase_receipt"],
        json!("PR-FROM-PO")
    );
    assert_eq!(report.rows[0].values["expense_account"], json!("Expenses"));
    assert_eq!(report.rows[0].values["rate"], json!(50.0));
    assert_eq!(report.rows[0].values["freight_amount"], json!(3.0));
    assert_eq!(report.rows[0].values["total_tax"], json!(10.0));
    assert_eq!(report.rows[0].values["total"], json!(110.0));
    assert_eq!(report.rows[1].values["purchase_receipt"], json!("PR-2"));
    assert_eq!(
        report.rows[1].values["expense_account"],
        json!("Unrealized")
    );
    assert_eq!(report.rows[1].values["rate"], json!(80.0));
}

#[test]
fn item_wise_purchase_execute_adds_group_subtotals_and_grand_total() {
    let mut f = filters();
    f.group_by = Some("Supplier".to_string());
    let report = execute(f, input(), Vec::new()).expect("report");

    assert!(report.skip_total_row);
    assert!(report
        .rows
        .iter()
        .any(|row| row.values.get("item_code") == Some(&json!("SUP-1: SUP-1 Name"))));
    assert_eq!(
        report.rows.last().unwrap().values["item_code"],
        json!("Total")
    );
    assert_eq!(report.rows.last().unwrap().values["amount"], json!(180.0));
}
