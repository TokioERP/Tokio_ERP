use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("reference_name")
        .internal_links(vec![("Sales Invoice", vec!["invoices", "sales_invoice"])])
        .transactions(vec![
            DashboardSection::labeled("Reference", vec!["Sales Invoice"]),
            DashboardSection::labeled("Payment", vec!["Payment Entry", "Journal Entry"]),
        ])
}
