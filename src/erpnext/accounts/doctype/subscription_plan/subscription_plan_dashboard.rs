use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("subscription_plan")
        .non_standard_fieldnames(vec![("Payment Request", "plan"), ("Subscription", "plan")])
        .transactions(vec![DashboardSection::labeled(
            "References",
            vec!["Payment Request", "Subscription"],
        )])
}
