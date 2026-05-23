use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn bank_dashboard() -> DashboardData {
    DashboardData::new("bank").transactions(vec![DashboardSection::labeled(
        "Bank Details",
        vec!["Bank Account", "Bank Guarantee"],
    )])
}
