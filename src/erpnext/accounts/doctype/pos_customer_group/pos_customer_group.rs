use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosCustomerGroup {
    pub customer_group: Option<String>,
}

impl PosCustomerGroup {
    pub const DOCTYPE: &'static str = "POS Customer Group";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["customer_group"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(customer_group: impl Into<String>) -> Self {
        Self {
            customer_group: Some(customer_group.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .required()
            .in_list_view()]
    }
}

impl DocumentController for PosCustomerGroup {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
