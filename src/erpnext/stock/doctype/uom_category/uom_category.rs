use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UomCategory {
    pub category_name: Option<String>,
}

impl UomCategory {
    pub const DOCTYPE: &'static str = "UOM Category";
    pub const MODULE: &'static str = "Stock";
    pub const AUTONAME: &'static str = "field:category_name";
    pub const FIELD_ORDER: [&'static str; 1] = ["category_name"];
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(category_name: impl Into<String>) -> Self {
        Self {
            category_name: Some(category_name.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::data("category_name", "Category Name")
            .in_list_view()
            .required()
            .unique()]
    }
}

impl DocumentController for UomCategory {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
