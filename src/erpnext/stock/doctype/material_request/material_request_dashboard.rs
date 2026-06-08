use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("material_request")
        .internal_links(vec![
            ("Sales Order", vec!["items", "sales_order"]),
            ("Project", vec!["items", "project"]),
            ("Cost Center", vec!["items", "cost_center"]),
        ])
        .transactions(vec![
            DashboardSection::labeled(
                "Reference",
                vec![
                    "Sales Order",
                    "Request for Quotation",
                    "Supplier Quotation",
                    "Purchase Order",
                ],
            ),
            DashboardSection::labeled(
                "Stock",
                vec!["Stock Entry", "Purchase Receipt", "Pick List"],
            ),
            DashboardSection::labeled("Manufacturing", vec!["Work Order"]),
            DashboardSection::labeled("Internal Transfer", vec!["Sales Order"]),
            DashboardSection::labeled("Accounting Dimensions", vec!["Project", "Cost Center"]),
        ])
}
