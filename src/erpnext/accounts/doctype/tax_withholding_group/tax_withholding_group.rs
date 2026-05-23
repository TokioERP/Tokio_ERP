use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxWithholdingGroup {
    pub group_name: Option<String>,
}

impl TaxWithholdingGroup {
    pub const DOCTYPE: &'static str = "Tax Withholding Group";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:group_name";
    pub const FIELD_ORDER: [&'static str; 1] = ["group_name"];
    pub const ALLOW_RENAME: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(group_name: impl Into<String>) -> Self {
        Self {
            group_name: Some(group_name.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::data("group_name", "Group Name")
            .required()
            .unique()
            .in_list_view()]
    }
}

impl DocumentController for TaxWithholdingGroup {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
