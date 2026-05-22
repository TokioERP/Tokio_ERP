use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LedgerMergeAccounts {
    pub account: Option<String>,
    pub account_name: Option<String>,
    pub merged: bool,
}

impl LedgerMergeAccounts {
    pub const DOCTYPE: &'static str = "Ledger Merge Accounts";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["account", "account_name", "merged"];
    pub const IS_TABLE: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(account: impl Into<String>, account_name: impl Into<String>) -> Self {
        Self {
            account: Some(account.into()),
            account_name: Some(account_name.into()),
            merged: false,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
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
    }
}

impl DocumentController for LedgerMergeAccounts {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
