use tokio_erp::erpnext::stock::doctype::warehouse_type::warehouse_type::WarehouseType;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn warehouse_type_matches_erpnext_metadata_and_fields() {
    assert_eq!(WarehouseType::DOCTYPE, "Warehouse Type");
    assert_eq!(WarehouseType::MODULE, "Stock");
    assert_eq!(WarehouseType::AUTONAME, "Prompt");
    assert_eq!(WarehouseType::FIELD_ORDER, ["description"]);
    assert!(WarehouseType::QUICK_ENTRY);
    assert_eq!(WarehouseType::SORT_FIELD, "creation");
    assert_eq!(WarehouseType::SORT_ORDER, "ASC");
    assert!(WarehouseType::TRACK_CHANGES);

    assert_eq!(
        WarehouseType::fields(),
        vec![FieldSpec::small_text("description", "Description")]
    );
}

#[test]
fn warehouse_type_preserves_pass_controller_behavior() {
    let warehouse_type = WarehouseType::new(Some("Cold storage"));

    assert_eq!(warehouse_type.description.as_deref(), Some("Cold storage"));
    assert_eq!(warehouse_type.doctype(), "Warehouse Type");
    assert_eq!(warehouse_type.module(), "Stock");
    assert!(warehouse_type.custom_hooks().is_empty());
}

#[test]
fn warehouse_type_test_class_is_noop() {
    let warehouse_type = WarehouseType::default();

    assert_eq!(warehouse_type.description, None);
    assert!(warehouse_type.custom_hooks().is_empty());
}
