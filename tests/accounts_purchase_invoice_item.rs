use tokio_erp::erpnext::accounts::doctype::purchase_invoice_item::purchase_invoice_item::PurchaseInvoiceItem;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn purchase_invoice_item_matches_erpnext_core_metadata() {
    assert_eq!(PurchaseInvoiceItem::DOCTYPE, "Purchase Invoice Item");
    assert_eq!(PurchaseInvoiceItem::MODULE, "Accounts");
    assert_eq!(PurchaseInvoiceItem::FIELD_ORDER.len(), 115);
    assert_eq!(PurchaseInvoiceItem::FIELD_ORDER[0], "item_code");
    assert_eq!(PurchaseInvoiceItem::FIELD_ORDER[114], "page_break");
    assert!(PurchaseInvoiceItem::IS_TABLE);
    assert!(PurchaseInvoiceItem::EDITABLE_GRID);

    let fields = PurchaseInvoiceItem::fields();
    assert_eq!(fields.len(), 115);
    assert!(fields.contains(
        &FieldSpec::link("item_code", "Item")
            .options("Item")
            .print_hide()
            .in_list_view()
            .columns(3)
    ));
    assert!(fields.contains(
        &FieldSpec::data("item_name", "Item Name")
            .required()
            .fetch_from("item_code.item_name")
    ));
    assert!(fields.contains(
        &FieldSpec::float("qty", "Accepted Qty")
            .required()
            .in_list_view()
            .columns(2)
    ));
    assert!(fields.contains(
        &FieldSpec::currency("amount", "Amount")
            .options("currency")
            .required()
            .read_only()
            .in_list_view()
            .columns(2)
    ));
    assert!(fields.contains(
        &FieldSpec::button("add_serial_batch_bundle", "Add Serial / Batch No")
            .depends_on("eval:doc.use_serial_batch_fields === 0 && doc.docstatus === 0")
    ));
}

#[test]
fn purchase_invoice_item_preserves_pass_controller_behavior() {
    let blank = PurchaseInvoiceItem::default();
    assert_eq!(blank.item_code, None);
    assert_eq!(blank.item_name, None);
    assert_eq!(blank.qty, None);
    assert_eq!(blank.rate, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PurchaseInvoiceItem::new("_Test Item");
    assert_eq!(row.item_code.as_deref(), Some("_Test Item"));
    assert_eq!(row.doctype(), "Purchase Invoice Item");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
