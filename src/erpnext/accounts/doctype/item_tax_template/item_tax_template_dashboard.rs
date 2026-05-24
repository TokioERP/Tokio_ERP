use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("item_tax_template").transactions(vec![
        DashboardSection::labeled("Pre Sales", vec!["Quotation", "Supplier Quotation"]),
        DashboardSection::labeled(
            "Sales",
            vec!["Sales Invoice", "Sales Order", "Delivery Note"],
        ),
        DashboardSection::labeled(
            "Purchase",
            vec!["Purchase Invoice", "Purchase Order", "Purchase Receipt"],
        ),
        DashboardSection::labeled("Stock", vec!["Item Groups", "Item"]),
    ])
}
