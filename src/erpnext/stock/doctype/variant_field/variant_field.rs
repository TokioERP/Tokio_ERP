use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VariantField {
    pub field_name: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl VariantField {
    pub const DOCTYPE: &'static str = "Variant Field";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 1] = ["field_name"];
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        field_name: Option<&str>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            field_name: field_name.map(ToOwned::to_owned),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::autocomplete("field_name", "Field Name")
            .in_list_view()
            .required()]
    }
}

impl DocumentController for VariantField {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
