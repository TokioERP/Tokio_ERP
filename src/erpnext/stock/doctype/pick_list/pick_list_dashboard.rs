use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("pick_list")
        .non_standard_fieldnames(vec![
            ("Stock Reservation Entry", "from_voucher_no"),
            ("Delivery Note", "against_pick_list"),
        ])
        .internal_links(vec![("Sales Order", vec!["locations", "sales_order"])])
        .transactions(vec![
            DashboardSection::labeled("Sales", vec!["Sales Order", "Delivery Note"]),
            DashboardSection::labeled("Manufacturing", vec!["Stock Entry"]),
            DashboardSection::labeled("Reference", vec!["Stock Reservation Entry"]),
        ])
}
