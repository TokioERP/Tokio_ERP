use tokio_erp::erpnext::accounts::doctype::party_account::party_account::PartyAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn party_account_matches_erpnext_metadata() {
    assert_eq!(PartyAccount::DOCTYPE, "Party Account");
    assert_eq!(PartyAccount::MODULE, "Accounts");
    assert_eq!(
        PartyAccount::FIELD_ORDER,
        ["company", "account", "advance_account"]
    );
    assert!(PartyAccount::IS_TABLE);
    assert!(PartyAccount::EDITABLE_GRID);
    assert!(PartyAccount::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(PartyAccount::QUICK_ENTRY);

    assert_eq!(
        PartyAccount::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .ignore_user_permissions()
                .in_list_view(),
            FieldSpec::link("account", "Default Account")
                .options("Account")
                .in_list_view(),
            FieldSpec::link("advance_account", "Advance Account").options("Account"),
        ]
    );
}

#[test]
fn party_account_preserves_pass_controller_behavior() {
    let blank = PartyAccount::default();
    assert_eq!(blank.company, None);
    assert_eq!(blank.account, None);
    assert_eq!(blank.advance_account, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PartyAccount::new("Test Company");
    assert_eq!(row.company.as_deref(), Some("Test Company"));
    assert_eq!(row.doctype(), "Party Account");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
