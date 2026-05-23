use tokio_erp::erpnext::accounts::doctype::process_statement_of_accounts_cc::process_statement_of_accounts_cc::ProcessStatementOfAccountsCc;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_statement_of_accounts_cc_matches_erpnext_metadata() {
    assert_eq!(
        ProcessStatementOfAccountsCc::DOCTYPE,
        "Process Statement Of Accounts CC"
    );
    assert_eq!(ProcessStatementOfAccountsCc::MODULE, "Accounts");
    assert_eq!(ProcessStatementOfAccountsCc::FIELD_ORDER, ["cc"]);
    assert!(ProcessStatementOfAccountsCc::IS_TABLE);
    assert!(ProcessStatementOfAccountsCc::ALLOW_RENAME);
    assert!(ProcessStatementOfAccountsCc::EDITABLE_GRID);
    assert!(ProcessStatementOfAccountsCc::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        ProcessStatementOfAccountsCc::fields(),
        vec![FieldSpec::link("cc", "CC").options("User").in_list_view()]
    );
}

#[test]
fn process_statement_of_accounts_cc_preserves_pass_controller_behavior() {
    let blank = ProcessStatementOfAccountsCc::default();
    assert_eq!(blank.cc, None);
    assert!(blank.custom_hooks().is_empty());

    let row = ProcessStatementOfAccountsCc::new("test@example.com");
    assert_eq!(row.cc.as_deref(), Some("test@example.com"));
    assert_eq!(row.doctype(), "Process Statement Of Accounts CC");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
