use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemAttributeValue {
    pub attribute_value: Option<String>,
    pub abbr: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemAttributeValue {
    pub const DOCTYPE: &'static str = "Item Attribute Value";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 2] = ["attribute_value", "abbr"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        attribute_value: Option<&str>,
        abbr: Option<&str>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            attribute_value: attribute_value.map(ToOwned::to_owned),
            abbr: abbr.map(ToOwned::to_owned),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("attribute_value", "Attribute Value")
                .in_list_view()
                .required(),
            FieldSpec::data("abbr", "Abbreviation")
                .description("This will be appended to the Item Code of the variant. For example, if your abbreviation is \"SM\", and the item code is \"T-SHIRT\", the item code of the variant will be \"T-SHIRT-SM\"")
                .in_list_view()
                .required()
                .search_index(),
        ]
    }
}

impl DocumentController for ItemAttributeValue {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
