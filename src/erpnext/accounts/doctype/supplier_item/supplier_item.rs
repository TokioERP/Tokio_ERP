use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupplierItem {
    pub supplier: Option<String>,
}

impl SupplierItem {
    pub const DOCTYPE: &'static str = "Supplier Item";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["supplier"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(supplier: impl Into<String>) -> Self {
        Self {
            supplier: Some(supplier.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("supplier", "Supplier")
            .options("Supplier")
            .in_list_view()]
    }
}

impl DocumentController for SupplierItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
