use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("monthly_distribution")
        .non_standard_fieldnames(vec![
            ("Sales Person", "distribution_id"),
            ("Territory", "distribution_id"),
            ("Sales Partner", "distribution_id"),
        ])
        .transactions(vec![DashboardSection::labeled(
            "Target Details",
            vec!["Sales Person", "Territory", "Sales Partner"],
        )])
}
