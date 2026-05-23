use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PsoaCostCenter {
    pub cost_center_name: Option<String>,
}

impl PsoaCostCenter {
    pub const DOCTYPE: &'static str = "PSOA Cost Center";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["cost_center_name"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(cost_center_name: impl Into<String>) -> Self {
        Self {
            cost_center_name: Some(cost_center_name.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("cost_center_name", "Cost Center")
            .options("Cost Center")
            .required()
            .in_list_view()]
    }
}

impl DocumentController for PsoaCostCenter {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
