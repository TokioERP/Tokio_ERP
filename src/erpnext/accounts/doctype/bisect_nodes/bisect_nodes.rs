use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BisectNodes;

impl BisectNodes {
    pub const DOCTYPE: &'static str = "Bisect Nodes";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 9] = [
        "root",
        "left_child",
        "right_child",
        "period_from_date",
        "period_to_date",
        "difference",
        "balance_sheet_summary",
        "profit_loss_summary",
        "generated",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("root", "Root").options("Bisect Nodes"),
            FieldSpec::link("left_child", "Left Child").options("Bisect Nodes"),
            FieldSpec::link("right_child", "Right Child").options("Bisect Nodes"),
            FieldSpec::datetime("period_from_date", "Period_from_date"),
            FieldSpec::datetime("period_to_date", "Period To Date"),
            FieldSpec::float("difference", "Difference"),
            FieldSpec::float("balance_sheet_summary", "Balance Sheet Summary"),
            FieldSpec::float("profit_loss_summary", "Profit and Loss Summary"),
            FieldSpec::check("generated", "Generated").default("0"),
        ]
    }
}

impl DocumentController for BisectNodes {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
