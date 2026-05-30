use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModeOfPaymentAccount {
    pub company: Option<String>,
    pub default_account: Option<String>,
}

impl ModeOfPaymentAccount {
    pub const DOCTYPE: &'static str = "Mode of Payment Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["company", "default_account"];
    pub const IS_TABLE: bool = true;

    pub fn new(company: impl Into<String>, default_account: impl Into<String>) -> Self {
        Self {
            company: Some(company.into()),
            default_account: Some(default_account.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view(),
            FieldSpec::link("default_account", "Default Account")
                .options("Account")
                .in_list_view(),
        ]
    }
}

impl DocumentController for ModeOfPaymentAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
