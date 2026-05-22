use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankAccountType {
    pub account_type: Option<String>,
}

impl BankAccountType {
    pub const DOCTYPE: &'static str = "Bank Account Type";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:account_type";
    pub const FIELD_ORDER: [&'static str; 1] = ["account_type"];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const QUICK_ENTRY: bool = true;

    pub fn new(account_type: impl Into<String>) -> Self {
        Self {
            account_type: Some(account_type.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::data("account_type", "Account Type").unique()]
    }
}

impl DocumentController for BankAccountType {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
