use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn loyalty_program_dashboard() -> DashboardData {
    DashboardData::new("loyalty_program").transactions(vec![DashboardSection::items(vec![
        "Sales Invoice",
        "Customer",
    ])])
}
