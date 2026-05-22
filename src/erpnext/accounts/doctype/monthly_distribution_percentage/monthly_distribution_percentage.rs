use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MonthlyDistributionPercentage {
    pub month: Option<String>,
    pub percentage_allocation: Option<f64>,
}

impl MonthlyDistributionPercentage {
    pub const DOCTYPE: &'static str = "Monthly Distribution Percentage";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["month", "percentage_allocation"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const AUTONAME: Option<&'static str> = Some("hash");
    pub const IDX: Option<u16> = Some(1);

    pub fn new(month: impl Into<String>, percentage_allocation: f64) -> Self {
        Self {
            month: Some(month.into()),
            percentage_allocation: Some(percentage_allocation),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("month", "Month")
                .required()
                .read_only()
                .in_list_view()
                .oldfield("month", "Data"),
            FieldSpec::float("percentage_allocation", "Percentage Allocation")
                .in_list_view()
                .oldfield("percentage_allocation", "Currency"),
        ]
    }
}

impl DocumentController for MonthlyDistributionPercentage {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
