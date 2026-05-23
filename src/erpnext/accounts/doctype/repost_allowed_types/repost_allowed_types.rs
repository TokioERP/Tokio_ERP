use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostAllowedTypes {
    pub document_type: Option<String>,
}

impl RepostAllowedTypes {
    pub const DOCTYPE: &'static str = "Repost Allowed Types";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["document_type"];
    pub const ALLOW_RENAME: bool = true;
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(document_type: impl Into<String>) -> Self {
        Self {
            document_type: Some(document_type.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("document_type", "Doctype")
            .options("DocType")
            .in_list_view()]
    }
}

impl DocumentController for RepostAllowedTypes {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
