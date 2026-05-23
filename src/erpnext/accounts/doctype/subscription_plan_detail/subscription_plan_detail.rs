use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SubscriptionPlanDetail {
    pub plan: Option<String>,
    pub qty: i32,
}

impl SubscriptionPlanDetail {
    pub const DOCTYPE: &'static str = "Subscription Plan Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["plan", "qty"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(plan: impl Into<String>, qty: i32) -> Self {
        Self {
            plan: Some(plan.into()),
            qty,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("plan", "Plan")
                .options("Subscription Plan")
                .required()
                .in_list_view(),
            FieldSpec::int("qty", "Quantity").required().in_list_view(),
        ]
    }
}

impl DocumentController for SubscriptionPlanDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
