use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetAccount {
    pub account: Option<String>,
    pub budget_amount: f64,
}

impl BudgetAccount {
    pub const DOCTYPE: &'static str = "Budget Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["account", "budget_amount"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;

    pub fn new(account: impl Into<String>, budget_amount: f64) -> Self {
        Self {
            account: Some(account.into()),
            budget_amount,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::currency("budget_amount", "Budget Amount")
                .options("Company:company:default_currency")
                .required()
                .in_list_view(),
        ]
    }
}

impl DocumentController for BudgetAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
