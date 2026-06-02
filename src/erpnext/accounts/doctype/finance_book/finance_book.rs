use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinanceBook {
    pub finance_book_name: Option<String>,
}

impl FinanceBook {
    pub const DOCTYPE: &'static str = "Finance Book";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["finance_book_name"];
    pub const ALLOW_IMPORT: bool = true;
    pub const AUTONAME: &'static str = "field:finance_book_name";
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const ICON: &'static str = "fa fa-book";
    pub const QUICK_ENTRY: bool = true;
    pub const SEARCH_FIELDS: &'static str = "finance_book_name";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const TRACK_SEEN: bool = true;

    pub fn new(finance_book_name: impl Into<String>) -> Self {
        Self {
            finance_book_name: Some(finance_book_name.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::data("finance_book_name", "Name").unique()]
    }
}

impl DocumentController for FinanceBook {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
