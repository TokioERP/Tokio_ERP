use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn exchange_rate_revaluation_dashboard() -> DashboardData {
    DashboardData::new("reference_name")
        .transactions(vec![DashboardSection::items(vec!["Journal Entry"])])
}
