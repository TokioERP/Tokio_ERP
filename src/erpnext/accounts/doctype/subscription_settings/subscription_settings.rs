use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SubscriptionSettings {
    pub grace_period: i32,
    pub cancel_after_grace: bool,
    pub prorate: bool,
}

impl SubscriptionSettings {
    pub const DOCTYPE: &'static str = "Subscription Settings";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["grace_period", "cancel_after_grace", "prorate"];
    pub const EDITABLE_GRID: bool = true;
    pub const GRID_PAGE_LENGTH: u16 = 50;
    pub const HIDE_TOOLBAR: bool = false;
    pub const IS_SINGLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(grace_period: i32, cancel_after_grace: bool, prorate: bool) -> Self {
        Self {
            grace_period,
            cancel_after_grace,
            prorate,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::int("grace_period", "Grace Period")
                .default("1")
                .description("Number of days after invoice date has elapsed before canceling subscription or marking subscription as unpaid"),
            FieldSpec::check(
                "cancel_after_grace",
                "Cancel Subscription After Grace Period",
            )
            .default("0"),
            FieldSpec::check("prorate", "Prorate").default("1"),
        ]
    }
}

impl DocumentController for SubscriptionSettings {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
