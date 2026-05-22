use tokio_erp::erpnext::accounts::doctype::allowed_dimension::allowed_dimension::AllowedDimension;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn allowed_dimension_matches_erpnext_metadata() {
    assert_eq!(AllowedDimension::DOCTYPE, "Allowed Dimension");
    assert_eq!(AllowedDimension::MODULE, "Accounts");
    assert_eq!(
        AllowedDimension::FIELD_ORDER,
        ["accounting_dimension", "dimension_value"]
    );
    assert!(AllowedDimension::IS_TABLE);
    assert!(AllowedDimension::QUICK_ENTRY);
    assert!(AllowedDimension::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(AllowedDimension::TRACK_CHANGES);

    assert_eq!(
        AllowedDimension::fields(),
        vec![
            FieldSpec::link("accounting_dimension", "Accounting Dimension")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("dimension_value")
                .options("accounting_dimension")
                .in_list_view(),
        ]
    );
}

#[test]
fn allowed_dimension_preserves_pass_controller_behavior() {
    let blank = AllowedDimension::default();
    assert_eq!(blank.accounting_dimension, None);
    assert_eq!(blank.dimension_value, None);
    assert!(blank.custom_hooks().is_empty());

    let dimension = AllowedDimension::new("Cost Center", "Main - TC");
    assert_eq!(
        dimension.accounting_dimension.as_deref(),
        Some("Cost Center")
    );
    assert_eq!(dimension.dimension_value.as_deref(), Some("Main - TC"));
    assert_eq!(dimension.doctype(), "Allowed Dimension");
    assert_eq!(dimension.module(), "Accounts");
    assert!(dimension.custom_hooks().is_empty());
}
