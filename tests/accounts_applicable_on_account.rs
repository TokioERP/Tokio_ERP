use tokio_erp::erpnext::accounts::doctype::applicable_on_account::applicable_on_account::ApplicableOnAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn applicable_on_account_matches_erpnext_metadata() {
    assert_eq!(ApplicableOnAccount::DOCTYPE, "Applicable On Account");
    assert_eq!(ApplicableOnAccount::MODULE, "Accounts");
    assert_eq!(
        ApplicableOnAccount::FIELD_ORDER,
        ["applicable_on_account", "is_mandatory"]
    );
    assert!(ApplicableOnAccount::IS_TABLE);
    assert!(ApplicableOnAccount::QUICK_ENTRY);
    assert!(ApplicableOnAccount::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ApplicableOnAccount::TRACK_CHANGES);

    assert_eq!(
        ApplicableOnAccount::fields(),
        vec![
            FieldSpec::link("applicable_on_account", "Accounts")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::check("is_mandatory", "Is Mandatory")
                .default("0")
                .columns(2)
                .in_list_view(),
        ]
    );
}

#[test]
fn applicable_on_account_preserves_pass_controller_behavior() {
    let blank = ApplicableOnAccount::default();
    assert_eq!(blank.applicable_on_account, None);
    assert!(!blank.is_mandatory);
    assert!(blank.custom_hooks().is_empty());

    let row = ApplicableOnAccount::new("Cash - TC", true);
    assert_eq!(row.applicable_on_account.as_deref(), Some("Cash - TC"));
    assert!(row.is_mandatory);
    assert_eq!(row.doctype(), "Applicable On Account");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
