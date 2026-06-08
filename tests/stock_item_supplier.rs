use tokio_erp::erpnext::stock::doctype::item_supplier::item_supplier::ItemSupplier;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn item_supplier_matches_erpnext_metadata_and_fields() {
    assert_eq!(ItemSupplier::DOCTYPE, "Item Supplier");
    assert_eq!(ItemSupplier::MODULE, "Stock");
    assert_eq!(
        ItemSupplier::FIELD_ORDER,
        ["supplier", "column_break_vcuv", "supplier_part_no"]
    );
    assert!(ItemSupplier::EDITABLE_GRID);
    assert!(ItemSupplier::IS_TABLE);
    assert_eq!(ItemSupplier::SORT_FIELD, "creation");
    assert_eq!(ItemSupplier::SORT_ORDER, "DESC");
    assert!(ItemSupplier::TRACK_CHANGES);

    assert_eq!(
        ItemSupplier::fields(),
        vec![
            FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_vcuv"),
            FieldSpec::data("supplier_part_no", "Supplier Part Number")
                .in_global_search()
                .in_list_view()
                .width("200px"),
        ]
    );
}

#[test]
fn item_supplier_preserves_pass_controller_behavior() {
    let supplier = ItemSupplier::new(
        Some("Acme Supplies"),
        Some("ACME-001"),
        Some("ERP Item"),
        Some("supplier_items"),
        Some("Item"),
    );

    assert_eq!(supplier.supplier.as_deref(), Some("Acme Supplies"));
    assert_eq!(supplier.supplier_part_no.as_deref(), Some("ACME-001"));
    assert_eq!(supplier.parent.as_deref(), Some("ERP Item"));
    assert_eq!(supplier.parentfield.as_deref(), Some("supplier_items"));
    assert_eq!(supplier.parenttype.as_deref(), Some("Item"));
    assert_eq!(supplier.doctype(), "Item Supplier");
    assert_eq!(supplier.module(), "Stock");
    assert!(supplier.custom_hooks().is_empty());
}

#[test]
fn item_supplier_defaults_match_empty_child_row_state() {
    let supplier = ItemSupplier::default();

    assert_eq!(supplier.supplier, None);
    assert_eq!(supplier.supplier_part_no, None);
    assert_eq!(supplier.parent, None);
    assert_eq!(supplier.parentfield, None);
    assert_eq!(supplier.parenttype, None);
    assert!(supplier.custom_hooks().is_empty());
}
