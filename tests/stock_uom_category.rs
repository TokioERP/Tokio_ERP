use tokio_erp::erpnext::stock::doctype::uom_category::uom_category::UomCategory;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn uom_category_matches_erpnext_metadata_and_fields() {
    assert_eq!(UomCategory::DOCTYPE, "UOM Category");
    assert_eq!(UomCategory::MODULE, "Stock");
    assert_eq!(UomCategory::AUTONAME, "field:category_name");
    assert_eq!(UomCategory::FIELD_ORDER, ["category_name"]);
    assert!(UomCategory::ALLOW_RENAME);
    assert!(UomCategory::EDITABLE_GRID);
    assert!(UomCategory::QUICK_ENTRY);
    assert_eq!(UomCategory::SORT_FIELD, "creation");
    assert_eq!(UomCategory::SORT_ORDER, "DESC");

    assert_eq!(
        UomCategory::fields(),
        vec![FieldSpec::data("category_name", "Category Name")
            .in_list_view()
            .required()
            .unique(),]
    );
}

#[test]
fn uom_category_preserves_pass_controller_behavior() {
    let category = UomCategory::new("Volume");

    assert_eq!(category.category_name.as_deref(), Some("Volume"));
    assert_eq!(category.doctype(), "UOM Category");
    assert_eq!(category.module(), "Stock");
    assert!(category.custom_hooks().is_empty());
}

#[test]
fn uom_category_test_class_is_noop() {
    let category = UomCategory::default();

    assert_eq!(category.category_name, None);
    assert!(category.custom_hooks().is_empty());
}
