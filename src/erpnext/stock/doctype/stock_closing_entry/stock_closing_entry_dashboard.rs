use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("stock_closing_entry").transactions(vec![DashboardSection::labeled(
        "Stock Closing Log",
        vec!["Stock Closing Balance"],
    )])
}
