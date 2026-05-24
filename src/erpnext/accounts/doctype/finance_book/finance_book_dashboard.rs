use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("finance_book")
        .non_standard_fieldnames(vec![
            ("Asset", "default_finance_book"),
            ("Company", "default_finance_book"),
        ])
        .transactions(vec![
            DashboardSection::labeled("Assets", vec!["Asset", "Asset Value Adjustment"]),
            DashboardSection::items(vec!["Company"]),
            DashboardSection::items(vec!["Journal Entry"]),
        ])
}
