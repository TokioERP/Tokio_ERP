use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::accounts::report::item_wise_sales_register::item_wise_sales_register::{
    execute, get_columns, get_delivery_notes_against_sales_order, get_items, AdditionalColumn,
    CustomerDetail, ItemTaxDetail, ItemWiseSalesFilters, ItemWiseSalesInput, ReportColumn,
    SalesInvoiceItemRow,
};

fn filters() -> ItemWiseSalesFilters {
    ItemWiseSalesFilters {
        company: Some("Acme".to_string()),
        customer: None,
        customer_group: None,
        mode_of_payment: None,
        from_date: Some("2026-01-01".to_string()),
        to_date: Some("2026-12-31".to_string()),
        warehouse: None,
        brand: None,
        item_code: None,
        item_group: None,
        income_account: None,
        group_by: None,
    }
}

fn row(
    name: &str,
    parent: &str,
    item_code: &str,
    customer: &str,
    amount: f64,
) -> SalesInvoiceItemRow {
    SalesInvoiceItemRow {
        name: name.to_string(),
        parent: parent.to_string(),
        posting_date: "2026-02-01".to_string(),
        debit_to: "Debtors".to_string(),
        unrealized_profit_loss_account: None,
        is_internal_customer: false,
        customer: customer.to_string(),
        territory: "Uzbekistan".to_string(),
        company: "Acme".to_string(),
        item_code: item_code.to_string(),
        description: format!("{item_code} description"),
        si_item_name: Some(format!("{item_code} SI")),
        si_item_group: Some("Products".to_string()),
        i_item_name: Some(format!("{item_code} Item")),
        i_item_group: Some("Fallback Group".to_string()),
        project: Some("PRJ".to_string()),
        sales_order: Some("SO-1".to_string()),
        delivery_note: None,
        so_detail: Some("SO-DETAIL-1".to_string()),
        income_account: Some("Sales".to_string()),
        cost_center: Some("Main".to_string()),
        enable_deferred_revenue: false,
        deferred_revenue_account: None,
        stock_qty: 2.0,
        stock_uom: "Nos".to_string(),
        base_net_rate: 50.0,
        base_net_amount: amount,
        customer_name: format!("{customer} Name"),
        customer_group: "Retail".to_string(),
        update_stock: false,
        uom: "Box".to_string(),
        qty: 1.0,
        warehouse: Some("Stores".to_string()),
        brand: Some("Brand A".to_string()),
        docstatus: 1,
        parenttype: "Sales Invoice".to_string(),
        extra: BTreeMap::from([("branch".to_string(), json!("Tashkent"))]),
    }
}

fn input() -> ItemWiseSalesInput {
    ItemWiseSalesInput {
        company_currency: "USD".to_string(),
        items: vec![row("SII-1", "SINV-1", "ITEM-1", "CUST-1", 100.0), {
            let mut r = row("SII-2", "SINV-2", "ITEM-2", "CUST-2", 80.0);
            r.posting_date = "2026-01-15".to_string();
            r.delivery_note = Some("DN-2".to_string());
            r.so_detail = None;
            r.is_internal_customer = true;
            r.unrealized_profit_loss_account = Some("Unrealized".to_string());
            r.income_account = Some("Sales Domestic".to_string());
            r.stock_qty = 0.0;
            r.stock_uom = "Nos".to_string();
            r.uom = "Nos".to_string();
            r
        }],
        item_taxes: vec![
            ItemTaxDetail::new("SII-1", "VAT", 10.0, 5.0, false),
            ItemTaxDetail::new("SII-1", "Freight", 3.0, 0.0, true),
            ItemTaxDetail::new("SII-2", "VAT", 8.0, 10.0, false),
        ],
        customer_details: BTreeMap::from([
            (
                "CUST-1".to_string(),
                CustomerDetail {
                    customer_name: "CUST-1 Name".to_string(),
                    customer_group: "Retail".to_string(),
                },
            ),
            (
                "CUST-2".to_string(),
                CustomerDetail {
                    customer_name: "CUST-2 Name".to_string(),
                    customer_group: "Retail".to_string(),
                },
            ),
        ]),
        mode_of_payments: BTreeMap::from([
            (
                "SINV-1".to_string(),
                vec!["Cash".to_string(), "Card".to_string()],
            ),
            ("SINV-2".to_string(), vec!["Bank".to_string()]),
        ]),
        delivery_notes_by_so_detail: BTreeMap::from([(
            "SO-DETAIL-1".to_string(),
            vec!["DN-FROM-SO".to_string()],
        )]),
        invoice_base_grand_totals: BTreeMap::from([
            ("SINV-1".to_string(), 113.0),
            ("SINV-2".to_string(), 88.0),
        ]),
    }
}

#[test]
fn item_wise_sales_columns_follow_group_by_rules_and_dynamic_tax_columns() {
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
    assert!(columns.iter().any(|column| column.fieldname == "customer"));
    assert!(columns.iter().any(|column| column.fieldname == "branch"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "freight_rate"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "vat_amount"));
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "total_other_charges"));

    let mut by_customer = filters();
    by_customer.group_by = Some("Customer".to_string());
    let grouped = get_columns(&additional, &by_customer, &[]);
    assert!(!grouped.iter().any(|column| column.fieldname == "customer"));
    assert!(grouped
        .iter()
        .any(|column| column.fieldname == "percent_gt"));

    assert_eq!(
        get_delivery_notes_against_sales_order(&input.items, &input.delivery_notes_by_so_detail)
            ["SO-DETAIL-1"],
        vec!["DN-FROM-SO"]
    );
}

#[test]
fn item_wise_sales_get_items_filters_and_orders_like_erpnext() {
    let mut f = filters();
    f.customer = Some("CUST-1".to_string());
    let rows = get_items(&f, &input());

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "SII-1");

    let all = get_items(&filters(), &input());
    assert_eq!(all[0].parent, "SINV-1");
    assert_eq!(all[1].parent, "SINV-2");

    let mut by_mop = filters();
    by_mop.mode_of_payment = Some("Bank".to_string());
    assert_eq!(get_items(&by_mop, &input())[0].parent, "SINV-2");

    let mut by_income = filters();
    by_income.income_account = Some("Unrealized".to_string());
    assert_eq!(get_items(&by_income, &input())[0].parent, "SINV-2");
}

#[test]
fn item_wise_sales_execute_maps_item_rows_taxes_and_direct_delivery_note() {
    let report = execute(
        filters(),
        input(),
        vec![AdditionalColumn::data("Branch", "branch", 80)],
    )
    .unwrap();

    let first = &report.rows[0].values;
    assert_eq!(first["invoice"], json!("SINV-1"));
    assert_eq!(first["item_name"], json!("ITEM-1 SI"));
    assert_eq!(first["delivery_note"], json!(null));
    assert_eq!(first["income_account"], json!("Sales"));
    assert_eq!(first["mode_of_payment"], json!("Cash, Card"));
    assert_eq!(first["branch"], json!("Tashkent"));
    assert_eq!(first["rate"], json!(25.0));
    assert_eq!(first["amount"], json!(100.0));
    assert_eq!(first["vat_amount"], json!(10.0));
    assert_eq!(first["freight_amount"], json!(3.0));
    assert_eq!(first["total_tax"], json!(10.0));
    assert_eq!(first["total_other_charges"], json!(3.0));
    assert_eq!(first["total"], json!(113.0));

    let second = &report.rows[1].values;
    assert_eq!(second["income_account"], json!("Unrealized"));
    assert_eq!(second["delivery_note"], json!("DN-2"));
    assert_eq!(second["rate"], json!(50.0));
}

#[test]
fn item_wise_sales_execute_adds_group_subtotals_and_grand_total() {
    let mut grouped = filters();
    grouped.group_by = Some("Customer".to_string());
    let report = execute(grouped, input(), Vec::new()).unwrap();

    assert!(report.skip_total_row);
    assert!(report.rows.iter().any(|row| {
        row.values.get("item_code") == Some(&json!("CUST-1: CUST-1 Name"))
            && row.values.get("total") == Some(&json!(113.0))
    }));
    assert_eq!(
        report.rows.last().unwrap().values["item_code"],
        json!("Total")
    );
    assert_eq!(report.rows.last().unwrap().values["total"], json!(201.0));
}
