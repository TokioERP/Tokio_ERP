use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemVariant {
    pub item_attribute: Option<String>,
    pub item_attribute_value: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemVariant {
    pub const DOCTYPE: &'static str = "Item Variant";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 2] = ["item_attribute", "item_attribute_value"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        item_attribute: impl Into<String>,
        item_attribute_value: impl Into<String>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            item_attribute: Some(item_attribute.into()),
            item_attribute_value: Some(item_attribute_value.into()),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("item_attribute", "Item Attribute")
                .options("Item Attribute")
                .in_list_view()
                .required(),
            FieldSpec::data("item_attribute_value", "Item Attribute Value")
                .in_list_view()
                .required(),
        ]
    }
}

impl DocumentController for ItemVariant {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
