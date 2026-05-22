use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AllowedDimension {
    pub accounting_dimension: Option<String>,
    pub dimension_value: Option<String>,
}

impl AllowedDimension {
    pub const DOCTYPE: &'static str = "Allowed Dimension";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["accounting_dimension", "dimension_value"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        accounting_dimension: impl Into<String>,
        dimension_value: impl Into<String>,
    ) -> Self {
        Self {
            accounting_dimension: Some(accounting_dimension.into()),
            dimension_value: Some(dimension_value.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("accounting_dimension", "Accounting Dimension")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("dimension_value")
                .options("accounting_dimension")
                .in_list_view(),
        ]
    }
}

impl DocumentController for AllowedDimension {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
