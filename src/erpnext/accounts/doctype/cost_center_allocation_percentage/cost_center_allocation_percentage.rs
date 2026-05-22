use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CostCenterAllocationPercentage {
    pub cost_center: Option<String>,
    pub percentage: Option<f64>,
}

impl CostCenterAllocationPercentage {
    pub const DOCTYPE: &'static str = "Cost Center Allocation Percentage";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["cost_center", "percentage"];
    pub const IS_TABLE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(cost_center: impl Into<String>, percentage: f64) -> Self {
        Self {
            cost_center: Some(cost_center.into()),
            percentage: Some(percentage),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .required()
                .in_list_view(),
            FieldSpec::percent("percentage", "Percentage (%)")
                .required()
                .in_list_view(),
        ]
    }
}

impl DocumentController for CostCenterAllocationPercentage {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
