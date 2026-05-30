use tokio_erp::erpnext::accounts::doctype::party_link::party_link::{
    create_party_link, PartyLink, PartyLinkError,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn party_link_matches_erpnext_metadata() {
    assert_eq!(PartyLink::DOCTYPE, "Party Link");
    assert_eq!(PartyLink::MODULE, "Accounts");
    assert_eq!(
        PartyLink::FIELD_ORDER,
        [
            "primary_role",
            "secondary_role",
            "column_break_2",
            "primary_party",
            "secondary_party",
        ]
    );
    assert_eq!(PartyLink::AUTONAME, "ACC-PT-LNK-.###.");
    assert!(PartyLink::EDITABLE_GRID);
    assert!(PartyLink::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(PartyLink::SORT_FIELD, "creation");
    assert_eq!(PartyLink::SORT_ORDER, "DESC");
    assert_eq!(PartyLink::TITLE_FIELD, "primary_party");
    assert!(PartyLink::TRACK_CHANGES);

    assert_eq!(
        PartyLink::fields(),
        vec![
            FieldSpec::link("primary_role", "Primary Role")
                .options("DocType")
                .in_list_view()
                .required(),
            FieldSpec::link("secondary_role", "Secondary Role")
                .options("DocType")
                .depends_on("primary_role")
                .mandatory_depends_on("primary_role"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::dynamic_link("primary_party")
                .label("Primary Party")
                .options("primary_role")
                .depends_on("primary_role")
                .mandatory_depends_on("primary_role"),
            FieldSpec::dynamic_link("secondary_party")
                .label("Secondary Party")
                .options("secondary_role")
                .depends_on("secondary_role")
                .mandatory_depends_on("secondary_role"),
        ]
    );
}

#[test]
fn party_link_validation_matches_erpnext_checks() {
    let doc = PartyLink::new("Customer", "CUST-001", "Supplier", "SUP-001");
    assert_eq!(doc.custom_hooks(), ["validate"]);
    assert_eq!(doc.validate(None, None, None), Ok(()));
    assert_eq!(doc.doctype(), "Party Link");
    assert_eq!(doc.module(), "Accounts");

    let invalid = PartyLink::new("Lead", "LEAD-001", "Supplier", "SUP-001");
    assert_eq!(
        invalid.validate(None, None, None),
        Err(PartyLinkError::InvalidPrimaryRole)
    );

    assert_eq!(
        doc.validate(Some("Customer"), None, None),
        Err(PartyLinkError::AlreadyLinked {
            primary_role: "Customer".to_string(),
            primary_party: "CUST-001".to_string(),
            secondary_role: "Supplier".to_string(),
            secondary_party: "SUP-001".to_string(),
        })
    );
    assert_eq!(
        doc.validate(None, Some("Customer"), None),
        Err(PartyLinkError::SecondaryAlreadyLinked {
            secondary_role: "Supplier".to_string(),
            secondary_party: "SUP-001".to_string(),
            existing_primary_role: "Customer".to_string(),
        })
    );
    assert_eq!(
        doc.validate(None, None, Some("Supplier")),
        Err(PartyLinkError::PrimaryAlreadyLinked {
            primary_role: "Customer".to_string(),
            primary_party: "CUST-001".to_string(),
            existing_primary_role: "Supplier".to_string(),
        })
    );
}

#[test]
fn create_party_link_sets_secondary_role_like_erpnext() {
    assert_eq!(
        create_party_link("Supplier", "SUP-001", "CUST-001"),
        PartyLink::new("Supplier", "SUP-001", "Customer", "CUST-001")
    );
    assert_eq!(
        create_party_link("Customer", "CUST-001", "SUP-001"),
        PartyLink::new("Customer", "CUST-001", "Supplier", "SUP-001")
    );
}
