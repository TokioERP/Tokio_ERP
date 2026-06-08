use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemWebsiteSpecification {
    pub label: Option<String>,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemWebsiteSpecification {
    pub const DOCTYPE: &'static str = "Item Website Specification";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 2] = ["label", "description"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        label: Option<&str>,
        description: Option<&str>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            label: label.map(ToOwned::to_owned),
            description: description.map(ToOwned::to_owned),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("label", "Label")
                .in_list_view()
                .width("150px"),
            FieldSpec::text_editor("description", "Description")
                .in_list_view()
                .width("300px"),
        ]
    }
}

impl DocumentController for ItemWebsiteSpecification {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
