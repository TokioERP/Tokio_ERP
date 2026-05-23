use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessStatementOfAccountsCc {
    pub cc: Option<String>,
}

impl ProcessStatementOfAccountsCc {
    pub const DOCTYPE: &'static str = "Process Statement Of Accounts CC";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["cc"];
    pub const IS_TABLE: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(cc: impl Into<String>) -> Self {
        Self {
            cc: Some(cc.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("cc", "CC").options("User").in_list_view()]
    }
}

impl DocumentController for ProcessStatementOfAccountsCc {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
