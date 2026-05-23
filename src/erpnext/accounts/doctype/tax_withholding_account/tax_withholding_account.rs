use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxWithholdingAccount {
    pub company: Option<String>,
    pub account: Option<String>,
}

impl TaxWithholdingAccount {
    pub const DOCTYPE: &'static str = "Tax Withholding Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["company", "account"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(company: impl Into<String>, account: impl Into<String>) -> Self {
        Self {
            company: Some(company.into()),
            account: Some(account.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view()
                .ignore_user_permissions(),
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view()
                .ignore_user_permissions(),
        ]
    }
}

impl DocumentController for TaxWithholdingAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
