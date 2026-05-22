use tokio_erp::erpnext::accounts::doctype::allowed_to_transact_with::allowed_to_transact_with::AllowedToTransactWith;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn allowed_to_transact_with_matches_erpnext_metadata() {
    assert_eq!(AllowedToTransactWith::DOCTYPE, "Allowed To Transact With");
    assert_eq!(AllowedToTransactWith::MODULE, "Accounts");
    assert_eq!(AllowedToTransactWith::FIELD_ORDER, ["company"]);
    assert!(AllowedToTransactWith::IS_TABLE);
    assert!(AllowedToTransactWith::QUICK_ENTRY);
    assert!(AllowedToTransactWith::TRACK_CHANGES);

    assert_eq!(
        AllowedToTransactWith::fields(),
        vec![FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .in_list_view()
            .ignore_user_permissions(),]
    );
}

#[test]
fn allowed_to_transact_with_preserves_pass_controller_behavior() {
    let blank = AllowedToTransactWith::default();
    assert_eq!(blank.company, None);
    assert!(blank.custom_hooks().is_empty());

    let row = AllowedToTransactWith::new("UzKing LLC");
    assert_eq!(row.company.as_deref(), Some("UzKing LLC"));
    assert_eq!(row.doctype(), "Allowed To Transact With");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
