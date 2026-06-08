use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WarehouseType {
    pub description: Option<String>,
}

impl WarehouseType {
    pub const DOCTYPE: &'static str = "Warehouse Type";
    pub const MODULE: &'static str = "Stock";
    pub const AUTONAME: &'static str = "Prompt";
    pub const FIELD_ORDER: [&'static str; 1] = ["description"];
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "ASC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(description: Option<&str>) -> Self {
        Self {
            description: description.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::small_text("description", "Description")]
    }
}

impl DocumentController for WarehouseType {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
