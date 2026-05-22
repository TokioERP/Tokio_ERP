use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LedgerHealthMonitorCompany {
    pub company: Option<String>,
}

impl LedgerHealthMonitorCompany {
    pub const DOCTYPE: &'static str = "Ledger Health Monitor Company";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["company"];
    pub const IS_TABLE: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(company: impl Into<String>) -> Self {
        Self {
            company: Some(company.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("company", "Company")
            .options("Company")
            .in_list_view()]
    }
}

impl DocumentController for LedgerHealthMonitorCompany {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
