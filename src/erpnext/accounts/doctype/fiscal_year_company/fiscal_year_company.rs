use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FiscalYearCompany {
    pub company: Option<String>,
}

impl FiscalYearCompany {
    pub const DOCTYPE: &'static str = "Fiscal Year Company";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["company"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const DOCUMENT_TYPE: Option<&'static str> = Some("Setup");
    pub const ROW_FORMAT: Option<&'static str> = Some("Dynamic");

    pub fn new(company: impl Into<String>) -> Self {
        Self {
            company: Some(company.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .ignore_user_permissions()
            .in_list_view()]
    }
}

impl DocumentController for FiscalYearCompany {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
