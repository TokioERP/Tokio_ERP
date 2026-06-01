use tokio_erp::erpnext::accounts::doctype::sales_invoice_item::sales_invoice_item::{
    SalesInvoiceItem, SalesInvoiceItemError,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_invoice_item_matches_erpnext_metadata_shape() {
    assert_eq!(SalesInvoiceItem::DOCTYPE, "Sales Invoice Item");
    assert_eq!(SalesInvoiceItem::MODULE, "Accounts");
    assert_eq!(SalesInvoiceItem::AUTONAME, "hash");
    assert_eq!(SalesInvoiceItem::DOCUMENT_TYPE, "Document");
    assert!(SalesInvoiceItem::EDITABLE_GRID);
    assert_eq!(SalesInvoiceItem::FIELD_ORDER.len(), 115);
    assert_eq!(
        &SalesInvoiceItem::FIELD_ORDER[..20],
        [
            "barcode",
            "has_item_scanned",
            "item_code",
            "col_break1",
            "item_name",
            "customer_item_code",
            "description_section",
            "description",
            "item_group",
            "brand",
            "image_section",
            "image",
            "image_view",
            "quantity_and_rate",
            "qty",
            "stock_uom",
            "col_break2",
            "uom",
            "conversion_factor",
            "stock_qty",
        ]
    );
    assert_eq!(
        &SalesInvoiceItem::FIELD_ORDER[SalesInvoiceItem::FIELD_ORDER.len() - 8..],
        [
            "column_break_92",
            "purchase_order_item",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
            "project",
            "section_break_54",
            "page_break",
        ]
    );

    let fields = SalesInvoiceItem::fields();
    assert_eq!(
        fields[0],
        FieldSpec::data("barcode", "Barcode").print_hide()
    );
    assert_eq!(
        fields[1],
        FieldSpec::link("item_code", "Item")
            .options("Item")
            .oldfield("item_code", "Link")
            .columns(4)
            .bold()
            .in_list_view()
            .search_index()
    );
    assert_eq!(
        fields[3],
        FieldSpec::data("item_name", "Item Name")
            .oldfield("item_name", "Data")
            .in_global_search()
            .print_hide()
            .required()
    );
    assert_eq!(
        fields[11],
        FieldSpec::currency("amount", "Amount")
            .options("currency")
            .oldfield("export_amount", "Currency")
            .columns(2)
            .read_only()
            .required()
            .in_list_view()
    );
}

#[test]
fn sales_invoice_item_cost_center_and_stock_helpers_match_erpnext() {
    let mut item = SalesInvoiceItem {
        idx: 3,
        item_code: Some("ITEM-001".to_string()),
        warehouse: Some("Stores - AC".to_string()),
        cost_center: Some("Main - AC".to_string()),
        ..Default::default()
    };
    assert_eq!(item.doctype(), "Sales Invoice Item");
    assert_eq!(item.module(), "Accounts");
    assert_eq!(item.validate_cost_center("Acme", Some("Acme")), Ok(()));
    assert_eq!(
        item.validate_cost_center("Other", Some("Acme")),
        Err(SalesInvoiceItemError::CostCenterWrongCompany {
            row: 3,
            cost_center: "Main - AC".to_string(),
            company: "Other".to_string(),
        })
    );

    item.set_actual_qty(Some(12.5));
    assert_eq!(item.actual_qty, 12.5);
    item.item_code = None;
    item.set_actual_qty(Some(99.0));
    assert_eq!(item.actual_qty, 12.5);
}

#[test]
fn sales_invoice_item_fixed_asset_income_account_matches_erpnext() {
    let mut non_asset = SalesInvoiceItem {
        is_fixed_asset: false,
        income_account: Some("Sales - AC".to_string()),
        ..Default::default()
    };
    non_asset.set_income_account_for_fixed_asset("Disposal - AC", Some("Dep CC - AC"));
    assert_eq!(non_asset.income_account.as_deref(), Some("Sales - AC"));

    let mut asset = SalesInvoiceItem {
        is_fixed_asset: true,
        income_account: Some("Sales - AC".to_string()),
        cost_center: None,
        ..Default::default()
    };
    asset.set_income_account_for_fixed_asset("Disposal - AC", Some("Dep CC - AC"));
    assert_eq!(asset.income_account.as_deref(), Some("Disposal - AC"));
    assert_eq!(asset.cost_center.as_deref(), Some("Dep CC - AC"));

    let mut with_cost_center = SalesInvoiceItem {
        is_fixed_asset: true,
        cost_center: Some("Existing CC - AC".to_string()),
        ..Default::default()
    };
    with_cost_center.set_income_account_for_fixed_asset("Disposal - AC", Some("Dep CC - AC"));
    assert_eq!(
        with_cost_center.cost_center.as_deref(),
        Some("Existing CC - AC")
    );
}
