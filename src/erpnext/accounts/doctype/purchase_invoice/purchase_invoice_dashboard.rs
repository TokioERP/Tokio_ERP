use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("purchase_invoice")
        .non_standard_fieldnames(vec![
            ("Journal Entry", "reference_name"),
            ("Payment Entry", "reference_name"),
            ("Payment Request", "reference_name"),
            ("Landed Cost Voucher", "receipt_document"),
            ("Purchase Invoice", "return_against"),
            ("Auto Repeat", "reference_document"),
        ])
        .internal_links(vec![
            ("Purchase Order", vec!["items", "purchase_order"]),
            ("Purchase Receipt", vec!["items", "purchase_receipt"]),
        ])
        .transactions(vec![
            DashboardSection::labeled(
                "Payment",
                vec!["Payment Entry", "Payment Request", "Journal Entry"],
            ),
            DashboardSection::labeled(
                "Reference",
                vec![
                    "Purchase Order",
                    "Purchase Receipt",
                    "Asset",
                    "Landed Cost Voucher",
                ],
            ),
            DashboardSection::labeled("Returns", vec!["Purchase Invoice"]),
            DashboardSection::labeled("Subscription", vec!["Auto Repeat"]),
        ])
}
