use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("promotional_scheme").transactions(vec![DashboardSection::labeled(
        "Reference",
        vec!["Pricing Rule"],
    )])
}
