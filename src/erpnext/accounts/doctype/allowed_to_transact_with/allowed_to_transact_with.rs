use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AllowedToTransactWith {
    pub company: Option<String>,
}

impl AllowedToTransactWith {
    pub const DOCTYPE: &'static str = "Allowed To Transact With";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["company"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(company: impl Into<String>) -> Self {
        Self {
            company: Some(company.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .in_list_view()
            .ignore_user_permissions()]
    }
}

impl DocumentController for AllowedToTransactWith {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
