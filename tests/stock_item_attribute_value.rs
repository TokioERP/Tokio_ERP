use tokio_erp::erpnext::stock::doctype::item_attribute_value::item_attribute_value::ItemAttributeValue;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn item_attribute_value_matches_erpnext_metadata_and_fields() {
    assert_eq!(ItemAttributeValue::DOCTYPE, "Item Attribute Value");
    assert_eq!(ItemAttributeValue::MODULE, "Stock");
    assert_eq!(ItemAttributeValue::FIELD_ORDER, ["attribute_value", "abbr"]);
    assert!(ItemAttributeValue::EDITABLE_GRID);
    assert!(ItemAttributeValue::IS_TABLE);
    assert_eq!(ItemAttributeValue::SORT_FIELD, "creation");
    assert_eq!(ItemAttributeValue::SORT_ORDER, "DESC");

    assert_eq!(
        ItemAttributeValue::fields(),
        vec![
            FieldSpec::data("attribute_value", "Attribute Value")
                .in_list_view()
                .required(),
            FieldSpec::data("abbr", "Abbreviation")
                .description("This will be appended to the Item Code of the variant. For example, if your abbreviation is \"SM\", and the item code is \"T-SHIRT\", the item code of the variant will be \"T-SHIRT-SM\"")
                .in_list_view()
                .required()
                .search_index(),
        ]
    );
}

#[test]
fn item_attribute_value_preserves_pass_controller_behavior() {
    let value = ItemAttributeValue::new(
        Some("Small"),
        Some("SM"),
        Some("Size"),
        Some("item_attribute_values"),
        Some("Item Attribute"),
    );

    assert_eq!(value.attribute_value.as_deref(), Some("Small"));
    assert_eq!(value.abbr.as_deref(), Some("SM"));
    assert_eq!(value.parent.as_deref(), Some("Size"));
    assert_eq!(value.parentfield.as_deref(), Some("item_attribute_values"));
    assert_eq!(value.parenttype.as_deref(), Some("Item Attribute"));
    assert_eq!(value.doctype(), "Item Attribute Value");
    assert_eq!(value.module(), "Stock");
    assert!(value.custom_hooks().is_empty());
}

#[test]
fn item_attribute_value_defaults_match_empty_child_row_state() {
    let value = ItemAttributeValue::default();

    assert_eq!(value.attribute_value, None);
    assert_eq!(value.abbr, None);
    assert_eq!(value.parent, None);
    assert_eq!(value.parentfield, None);
    assert_eq!(value.parenttype, None);
    assert!(value.custom_hooks().is_empty());
}
