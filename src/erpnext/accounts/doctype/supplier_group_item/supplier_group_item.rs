use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupplierGroupItem {
    pub supplier_group: Option<String>,
}

impl SupplierGroupItem {
    pub const DOCTYPE: &'static str = "Supplier Group Item";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["supplier_group"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(supplier_group: impl Into<String>) -> Self {
        Self {
            supplier_group: Some(supplier_group.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("supplier_group", "Supplier Group")
            .options("Supplier Group")
            .in_list_view()]
    }
}

impl DocumentController for SupplierGroupItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
