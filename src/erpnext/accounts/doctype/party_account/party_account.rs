use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyAccount {
    pub company: Option<String>,
    pub account: Option<String>,
    pub advance_account: Option<String>,
}

impl PartyAccount {
    pub const DOCTYPE: &'static str = "Party Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["company", "account", "advance_account"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const QUICK_ENTRY: bool = true;

    pub fn new(company: impl Into<String>) -> Self {
        Self {
            company: Some(company.into()),
            account: None,
            advance_account: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .ignore_user_permissions()
                .in_list_view(),
            FieldSpec::link("account", "Default Account")
                .options("Account")
                .in_list_view(),
            FieldSpec::link("advance_account", "Advance Account").options("Account"),
        ]
    }
}

impl DocumentController for PartyAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
