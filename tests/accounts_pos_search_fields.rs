use tokio_erp::erpnext::accounts::doctype::pos_search_fields::pos_search_fields::PosSearchFields;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_search_fields_matches_erpnext_metadata() {
    assert_eq!(PosSearchFields::DOCTYPE, "POS Search Fields");
    assert_eq!(PosSearchFields::MODULE, "Accounts");
    assert_eq!(PosSearchFields::FIELD_ORDER, ["field", "fieldname"]);
    assert!(PosSearchFields::IS_TABLE);
    assert!(PosSearchFields::EDITABLE_GRID);
    assert!(PosSearchFields::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        PosSearchFields::fields(),
        vec![
            FieldSpec::select("field", "Field")
                .required()
                .in_list_view(),
            FieldSpec::data("fieldname", "Fieldname"),
        ]
    );
}

#[test]
fn pos_search_fields_preserves_pass_controller_behavior() {
    let blank = PosSearchFields::default();
    assert_eq!(blank.field, None);
    assert_eq!(blank.fieldname, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosSearchFields::new("item_code");
    assert_eq!(row.field.as_deref(), Some("item_code"));
    assert_eq!(row.doctype(), "POS Search Fields");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
