use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn cost_center_dashboard() -> DashboardData {
    DashboardData::new("cost_center").reports(vec![DashboardSection::labeled(
        "Reports",
        vec!["Budget Variance Report", "General Ledger"],
    )])
}
