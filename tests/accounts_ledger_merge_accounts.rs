use tokio_erp::erpnext::accounts::doctype::ledger_merge_accounts::ledger_merge_accounts::LedgerMergeAccounts;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn ledger_merge_accounts_matches_erpnext_metadata() {
    assert_eq!(LedgerMergeAccounts::DOCTYPE, "Ledger Merge Accounts");
    assert_eq!(LedgerMergeAccounts::MODULE, "Accounts");
    assert_eq!(
        LedgerMergeAccounts::FIELD_ORDER,
        ["account", "account_name", "merged"]
    );
    assert!(LedgerMergeAccounts::IS_TABLE);
    assert!(LedgerMergeAccounts::ALLOW_RENAME);
    assert!(LedgerMergeAccounts::EDITABLE_GRID);
    assert!(LedgerMergeAccounts::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        LedgerMergeAccounts::fields(),
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .columns(4)
                .required()
                .in_list_view(),
            FieldSpec::data("account_name", "Account Name")
                .columns(4)
                .required()
                .read_only(),
            FieldSpec::check("merged", "Merged")
                .columns(2)
                .default("0")
                .read_only()
                .in_list_view(),
        ]
    );
}

#[test]
fn ledger_merge_accounts_preserves_pass_controller_behavior() {
    let blank = LedgerMergeAccounts::default();
    assert_eq!(blank.account, None);
    assert_eq!(blank.account_name, None);
    assert!(!blank.merged);
    assert!(blank.custom_hooks().is_empty());

    let row = LedgerMergeAccounts::new("Cash - TC", "Cash");
    assert_eq!(row.account.as_deref(), Some("Cash - TC"));
    assert_eq!(row.account_name.as_deref(), Some("Cash"));
    assert!(!row.merged);
    assert_eq!(row.doctype(), "Ledger Merge Accounts");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
