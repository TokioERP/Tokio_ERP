use tokio_erp::erpnext::stock::doctype::variant_field::variant_field::VariantField;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn variant_field_matches_erpnext_metadata_and_fields() {
    assert_eq!(VariantField::DOCTYPE, "Variant Field");
    assert_eq!(VariantField::MODULE, "Stock");
    assert_eq!(VariantField::FIELD_ORDER, ["field_name"]);
    assert!(VariantField::EDITABLE_GRID);
    assert!(VariantField::QUICK_ENTRY);
    assert_eq!(VariantField::SORT_FIELD, "creation");
    assert_eq!(VariantField::SORT_ORDER, "DESC");
    assert!(VariantField::TRACK_CHANGES);

    assert_eq!(
        VariantField::fields(),
        vec![FieldSpec::autocomplete("field_name", "Field Name")
            .in_list_view()
            .required(),]
    );
}

#[test]
fn variant_field_preserves_pass_controller_behavior() {
    let field = VariantField::new(
        Some("item_group"),
        Some("Item Variant Settings"),
        Some("fields"),
        Some("Item Variant Settings"),
    );

    assert_eq!(field.field_name.as_deref(), Some("item_group"));
    assert_eq!(field.parent.as_deref(), Some("Item Variant Settings"));
    assert_eq!(field.parentfield.as_deref(), Some("fields"));
    assert_eq!(field.parenttype.as_deref(), Some("Item Variant Settings"));
    assert_eq!(field.doctype(), "Variant Field");
    assert_eq!(field.module(), "Stock");
    assert!(field.custom_hooks().is_empty());
}

#[test]
fn variant_field_defaults_match_empty_child_row_state() {
    let field = VariantField::default();

    assert_eq!(field.field_name, None);
    assert_eq!(field.parent, None);
    assert_eq!(field.parentfield, None);
    assert_eq!(field.parenttype, None);
    assert!(field.custom_hooks().is_empty());
}
