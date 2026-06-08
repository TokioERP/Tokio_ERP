use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("batch_no").transactions(vec![
        DashboardSection::labeled("Buy", vec!["Purchase Invoice", "Purchase Receipt"]),
        DashboardSection::labeled("Sell", vec!["Sales Invoice", "Delivery Note"]),
        DashboardSection::labeled("Move", vec!["Stock Entry", "Serial and Batch Bundle"]),
        DashboardSection::labeled("Quality", vec!["Quality Inspection"]),
    ])
}
