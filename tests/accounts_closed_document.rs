use tokio_erp::erpnext::accounts::doctype::closed_document::closed_document::ClosedDocument;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn closed_document_matches_erpnext_metadata() {
    assert_eq!(ClosedDocument::DOCTYPE, "Closed Document");
    assert_eq!(ClosedDocument::MODULE, "Accounts");
    assert_eq!(ClosedDocument::FIELD_ORDER, ["document_type", "closed"]);
    assert!(ClosedDocument::IS_TABLE);
    assert!(ClosedDocument::QUICK_ENTRY);
    assert!(ClosedDocument::TRACK_CHANGES);

    assert_eq!(
        ClosedDocument::fields(),
        vec![
            FieldSpec::link("document_type", "Document Type")
                .options("DocType")
                .required()
                .in_list_view(),
            FieldSpec::check("closed", "Closed")
                .default("0")
                .in_list_view(),
        ]
    );
}

#[test]
fn closed_document_preserves_pass_controller_behavior() {
    let blank = ClosedDocument::default();
    assert_eq!(blank.document_type, None);
    assert!(!blank.closed);
    assert!(blank.custom_hooks().is_empty());

    let row = ClosedDocument::new("Sales Invoice", true);
    assert_eq!(row.document_type.as_deref(), Some("Sales Invoice"));
    assert!(row.closed);
    assert_eq!(row.doctype(), "Closed Document");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
