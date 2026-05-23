use tokio_erp::erpnext::accounts::doctype::repost_allowed_types::repost_allowed_types::RepostAllowedTypes;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn repost_allowed_types_matches_erpnext_metadata() {
    assert_eq!(RepostAllowedTypes::DOCTYPE, "Repost Allowed Types");
    assert_eq!(RepostAllowedTypes::MODULE, "Accounts");
    assert_eq!(RepostAllowedTypes::FIELD_ORDER, ["document_type"]);
    assert!(RepostAllowedTypes::ALLOW_RENAME);
    assert!(RepostAllowedTypes::IS_TABLE);
    assert!(RepostAllowedTypes::EDITABLE_GRID);
    assert!(RepostAllowedTypes::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        RepostAllowedTypes::fields(),
        vec![FieldSpec::link("document_type", "Doctype")
            .options("DocType")
            .in_list_view()]
    );
}

#[test]
fn repost_allowed_types_controller_is_pass_through() {
    let row = RepostAllowedTypes::new("Sales Invoice");
    assert_eq!(row.document_type.as_deref(), Some("Sales Invoice"));
    assert_eq!(row.doctype(), "Repost Allowed Types");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
