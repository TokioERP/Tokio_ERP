use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn payment_gateway_account_dashboard() -> DashboardData {
    DashboardData::new("payment_gateway_account")
        .non_standard_fieldnames(vec![("Subscription Plan", "payment_gateway")])
        .transactions(vec![
            DashboardSection::items(vec!["Payment Request"]),
            DashboardSection::items(vec!["Subscription Plan"]),
        ])
}
