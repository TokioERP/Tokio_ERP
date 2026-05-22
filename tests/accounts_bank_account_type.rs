use tokio_erp::erpnext::accounts::doctype::bank_account_type::bank_account_type::BankAccountType;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_account_type_matches_erpnext_metadata() {
    assert_eq!(BankAccountType::DOCTYPE, "Bank Account Type");
    assert_eq!(BankAccountType::MODULE, "Accounts");
    assert_eq!(BankAccountType::AUTONAME, "field:account_type");
    assert_eq!(BankAccountType::FIELD_ORDER, ["account_type"]);
    assert!(BankAccountType::ALLOW_IMPORT);
    assert!(BankAccountType::ALLOW_RENAME);
    assert!(BankAccountType::QUICK_ENTRY);

    assert_eq!(
        BankAccountType::fields(),
        vec![FieldSpec::data("account_type", "Account Type").unique()]
    );
}

#[test]
fn bank_account_type_preserves_pass_controller_behavior() {
    let blank = BankAccountType::default();
    assert_eq!(blank.account_type, None);
    assert!(blank.custom_hooks().is_empty());

    let checking = BankAccountType::new("Checking");
    assert_eq!(checking.account_type.as_deref(), Some("Checking"));
    assert_eq!(checking.doctype(), "Bank Account Type");
    assert_eq!(checking.module(), "Accounts");
    assert!(checking.custom_hooks().is_empty());
}
