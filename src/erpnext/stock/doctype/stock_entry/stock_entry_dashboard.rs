use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("stock_entry")
        .non_standard_fieldnames(vec![("Stock Reservation Entry", "from_voucher_no")])
        .transactions(vec![DashboardSection::labeled(
            "Stock Reservation",
            vec!["Stock Reservation Entry"],
        )])
}
