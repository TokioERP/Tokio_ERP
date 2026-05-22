use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomerGroupItem {
    pub customer_group: Option<String>,
}

impl CustomerGroupItem {
    pub const DOCTYPE: &'static str = "Customer Group Item";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["customer_group"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(customer_group: impl Into<String>) -> Self {
        Self {
            customer_group: Some(customer_group.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .in_list_view()]
    }
}

impl DocumentController for CustomerGroupItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
