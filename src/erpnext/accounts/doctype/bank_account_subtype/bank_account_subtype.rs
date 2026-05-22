use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankAccountSubtype {
    pub account_subtype: Option<String>,
}

impl BankAccountSubtype {
    pub const DOCTYPE: &'static str = "Bank Account Subtype";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:account_subtype";
    pub const FIELD_ORDER: [&'static str; 1] = ["account_subtype"];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const QUICK_ENTRY: bool = true;

    pub fn new(account_subtype: impl Into<String>) -> Self {
        Self {
            account_subtype: Some(account_subtype.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::data("account_subtype", "Account Subtype").unique()]
    }
}

impl DocumentController for BankAccountSubtype {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
