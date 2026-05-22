use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosSearchFields {
    pub field: Option<String>,
    pub fieldname: Option<String>,
}

impl PosSearchFields {
    pub const DOCTYPE: &'static str = "POS Search Fields";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["field", "fieldname"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: Some(field.into()),
            fieldname: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("field", "Field")
                .required()
                .in_list_view(),
            FieldSpec::data("fieldname", "Fieldname"),
        ]
    }
}

impl DocumentController for PosSearchFields {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
