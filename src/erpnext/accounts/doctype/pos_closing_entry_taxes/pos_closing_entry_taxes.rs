use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosClosingEntryTaxes {
    pub account_head: Option<String>,
    pub amount: Option<String>,
}

impl PosClosingEntryTaxes {
    pub const DOCTYPE: &'static str = "POS Closing Entry Taxes";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["account_head", "amount"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(account_head: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            account_head: Some(account_head.into()),
            amount: Some(amount.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account_head", "Account Head")
                .options("Account")
                .in_list_view()
                .read_only(),
            FieldSpec::currency("amount", "Amount")
                .in_list_view()
                .read_only(),
        ]
    }
}

impl DocumentController for PosClosingEntryTaxes {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
