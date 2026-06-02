use tokio_erp::erpnext::accounts::doctype::pos_invoice_item::pos_invoice_item::PosInvoiceItem;
use tokio_erp::erpnext::accounts::doctype::sales_invoice_item::sales_invoice_item::{
    SalesInvoiceItem, SalesInvoiceItemError,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_invoice_item_matches_erpnext_metadata_shape() {
    assert_eq!(PosInvoiceItem::DOCTYPE, "POS Invoice Item");
    assert_eq!(PosInvoiceItem::MODULE, "Accounts");
    assert_eq!(PosInvoiceItem::AUTONAME, "hash");
    assert_eq!(PosInvoiceItem::DOCUMENT_TYPE, "Document");
    assert!(PosInvoiceItem::EDITABLE_GRID);
    assert!(PosInvoiceItem::IS_TABLE);
    assert_eq!(PosInvoiceItem::ROW_FORMAT, "Dynamic");
    assert_eq!(PosInvoiceItem::SORT_FIELD, "creation");
    assert_eq!(PosInvoiceItem::SORT_ORDER, "DESC");
    assert_eq!(PosInvoiceItem::FIELD_ORDER.len(), 98);
    assert_eq!(
        &PosInvoiceItem::FIELD_ORDER[..20],
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
        &PosInvoiceItem::FIELD_ORDER[PosInvoiceItem::FIELD_ORDER.len() - 8..],
        [
            "dn_detail",
            "delivered_qty",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
            "project",
            "section_break_54",
            "page_break",
        ]
    );

    let fields = PosInvoiceItem::fields();
    assert_eq!(
        fields[0],
        FieldSpec::data("barcode", "Barcode").print_hide()
    );
    assert_eq!(
        field(&fields, "item_code"),
        &FieldSpec::link("item_code", "Item")
            .options("Item")
            .oldfield("item_code", "Link")
            .columns(4)
            .bold()
            .in_list_view()
            .search_index()
    );
    assert_eq!(
        field(&fields, "item_name"),
        &FieldSpec::data("item_name", "Item Name")
            .oldfield("item_name", "Data")
            .in_global_search()
            .print_hide()
            .required()
    );
    assert_eq!(
        field(&fields, "income_account"),
        &FieldSpec::link("income_account", "Income Account")
            .options("Account")
            .oldfield("income_account", "Link")
            .width("120px")
            .print_hide()
            .required()
    );
    assert_eq!(
        field(&fields, "cost_center"),
        &FieldSpec::link("cost_center", "Cost Center")
            .options("Cost Center")
            .default(":Company")
            .oldfield("cost_center", "Link")
            .width("120px")
            .print_hide()
            .required()
    );
}

fn field<'a>(fields: &'a [FieldSpec], fieldname: &str) -> &'a FieldSpec {
    fields
        .iter()
        .find(|field| field.fieldname == fieldname)
        .expect("field exists")
}

#[test]
fn pos_invoice_item_inherits_sales_invoice_item_core_behaviour() {
    let mut item = PosInvoiceItem::from_sales_invoice_item(SalesInvoiceItem {
        idx: 7,
        item_code: Some("ITEM-001".to_string()),
        warehouse: Some("Main Warehouse - AC".to_string()),
        cost_center: Some("Main - AC".to_string()),
        ..Default::default()
    });

    assert_eq!(item.doctype(), "POS Invoice Item");
    assert_eq!(item.module(), "Accounts");
    assert_eq!(item.validate_cost_center("Acme", Some("Acme")), Ok(()));
    assert_eq!(
        item.validate_cost_center("Other", Some("Acme")),
        Err(SalesInvoiceItemError::CostCenterWrongCompany {
            row: 7,
            cost_center: "Main - AC".to_string(),
            company: "Other".to_string(),
        })
    );

    item.set_actual_qty(Some(4.25));
    assert_eq!(item.sales_invoice_item().actual_qty, 4.25);
    item.sales_invoice_item_mut().item_code = None;
    item.set_actual_qty(Some(9.0));
    assert_eq!(item.sales_invoice_item().actual_qty, 4.25);
}

#[test]
fn pos_invoice_item_delegates_fixed_asset_income_account_logic() {
    let mut item = PosInvoiceItem::from_sales_invoice_item(SalesInvoiceItem {
        is_fixed_asset: true,
        income_account: Some("Sales - AC".to_string()),
        cost_center: None,
        ..Default::default()
    });

    item.set_income_account_for_fixed_asset("Disposal - AC", Some("Dep CC - AC"));

    assert_eq!(
        item.sales_invoice_item().income_account.as_deref(),
        Some("Disposal - AC")
    );
    assert_eq!(
        item.sales_invoice_item().cost_center.as_deref(),
        Some("Dep CC - AC")
    );
}
