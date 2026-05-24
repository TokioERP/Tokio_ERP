use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("payment_request")
        .internal_links(vec![
            ("Payment Entry", vec!["references", "payment_request"]),
            ("Payment Order", vec!["references", "payment_order"]),
        ])
        .transactions(vec![DashboardSection::labeled(
            "Payment",
            vec!["Payment Entry", "Payment Order"],
        )])
}
