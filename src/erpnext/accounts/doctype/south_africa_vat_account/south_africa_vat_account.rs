use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SouthAfricaVATAccount {
    pub account: Option<String>,
}

impl SouthAfricaVATAccount {
    pub const DOCTYPE: &'static str = "South Africa VAT Account";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "account";
    pub const FIELD_ORDER: [&'static str; 1] = ["account"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(account: impl Into<String>) -> Self {
        Self {
            account: Some(account.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("account", "Account")
            .options("Account")
            .in_list_view()]
    }
}

impl DocumentController for SouthAfricaVATAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
