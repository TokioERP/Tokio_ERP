use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShareType {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareTypeDashboard {
    pub fieldname: &'static str,
    pub transactions: Vec<(&'static str, Vec<&'static str>)>,
}

impl ShareType {
    pub const DOCTYPE: &'static str = "Share Type";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:title";
    pub const FIELD_ORDER: [&'static str; 2] = ["title", "description"];
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("title", "Title")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::long_text("description", "Description"),
        ]
    }
}

impl DocumentController for ShareType {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn share_type_dashboard() -> ShareTypeDashboard {
    ShareTypeDashboard {
        fieldname: "share_type",
        transactions: vec![("References", vec!["Share Transfer", "Shareholder"])],
    }
}

pub fn share_type_js_hooks() -> [&'static str; 1] {
    ["refresh"]
}
