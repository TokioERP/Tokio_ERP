use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetDistribution {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub amount: f64,
    pub percent: f64,
}

impl BudgetDistribution {
    pub const DOCTYPE: &'static str = "Budget Distribution";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] = ["start_date", "end_date", "amount", "percent"];
    pub const IS_TABLE: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("start_date", "Start Date")
                .read_only()
                .in_list_view(),
            FieldSpec::date("end_date", "End Date")
                .read_only()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount").in_list_view(),
            FieldSpec::percent("percent", "Percent").in_list_view(),
        ]
    }
}

impl DocumentController for BudgetDistribution {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
