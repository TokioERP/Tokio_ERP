use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn payment_order_dashboard() -> DashboardData {
    DashboardData::new("payment_order").transactions(vec![DashboardSection::items(vec![
        "Payment Entry",
        "Journal Entry",
    ])])
}
