use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClosedDocument {
    pub document_type: Option<String>,
    pub closed: bool,
}

impl ClosedDocument {
    pub const DOCTYPE: &'static str = "Closed Document";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["document_type", "closed"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(document_type: impl Into<String>, closed: bool) -> Self {
        Self {
            document_type: Some(document_type.into()),
            closed,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("document_type", "Document Type")
                .options("DocType")
                .required()
                .in_list_view(),
            FieldSpec::check("closed", "Closed")
                .default("0")
                .in_list_view(),
        ]
    }
}

impl DocumentController for ClosedDocument {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
