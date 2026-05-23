use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TransactionDeletionRecordDetails {
    pub doctype_name: Option<String>,
    pub docfield_name: Option<String>,
    pub no_of_docs: i32,
    pub done: bool,
}

impl TransactionDeletionRecordDetails {
    pub const DOCTYPE: &'static str = "Transaction Deletion Record Details";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] =
        ["doctype_name", "docfield_name", "no_of_docs", "done"];
    pub const ALLOW_RENAME: bool = true;
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(doctype_name: impl Into<String>, no_of_docs: i32) -> Self {
        Self {
            doctype_name: Some(doctype_name.into()),
            no_of_docs,
            done: false,
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("doctype_name", "DocType")
                .options("DocType")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("docfield_name", "DocField").read_only(),
            FieldSpec::int("no_of_docs", "No of Docs")
                .read_only()
                .in_list_view(),
            FieldSpec::check("done", "Done")
                .default("0")
                .read_only()
                .in_list_view(),
        ]
    }
}

impl DocumentController for TransactionDeletionRecordDetails {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
