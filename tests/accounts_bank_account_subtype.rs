use tokio_erp::erpnext::accounts::doctype::bank_account_subtype::bank_account_subtype::BankAccountSubtype;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_account_subtype_matches_erpnext_metadata() {
    assert_eq!(BankAccountSubtype::DOCTYPE, "Bank Account Subtype");
    assert_eq!(BankAccountSubtype::MODULE, "Accounts");
    assert_eq!(BankAccountSubtype::AUTONAME, "field:account_subtype");
    assert_eq!(BankAccountSubtype::FIELD_ORDER, ["account_subtype"]);
    assert!(BankAccountSubtype::ALLOW_IMPORT);
    assert!(BankAccountSubtype::ALLOW_RENAME);
    assert!(BankAccountSubtype::QUICK_ENTRY);

    assert_eq!(
        BankAccountSubtype::fields(),
        vec![FieldSpec::data("account_subtype", "Account Subtype").unique()]
    );
}

#[test]
fn bank_account_subtype_preserves_pass_controller_behavior() {
    let blank = BankAccountSubtype::default();
    assert_eq!(blank.account_subtype, None);
    assert!(blank.custom_hooks().is_empty());

    let savings = BankAccountSubtype::new("Savings");
    assert_eq!(savings.account_subtype.as_deref(), Some("Savings"));
    assert_eq!(savings.doctype(), "Bank Account Subtype");
    assert_eq!(savings.module(), "Accounts");
    assert!(savings.custom_hooks().is_empty());
}
