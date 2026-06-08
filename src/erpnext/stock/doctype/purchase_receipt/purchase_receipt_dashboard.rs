use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("purchase_receipt_no")
        .non_standard_fieldnames(vec![
            ("Purchase Invoice", "purchase_receipt"),
            ("Asset", "purchase_receipt"),
            ("Landed Cost Voucher", "receipt_document"),
            ("Auto Repeat", "reference_document"),
            ("Purchase Receipt", "return_against"),
            ("Stock Reservation Entry", "from_voucher_no"),
            ("Quality Inspection", "reference_name"),
        ])
        .internal_links(vec![
            ("Material Request", vec!["items", "material_request"]),
            ("Purchase Order", vec!["items", "purchase_order"]),
            ("Project", vec!["items", "project"]),
        ])
        .internal_and_external_links(vec![(
            "Purchase Invoice",
            vec!["items", "purchase_invoice"],
        )])
        .transactions(vec![
            DashboardSection::labeled(
                "Related",
                vec![
                    "Purchase Invoice",
                    "Landed Cost Voucher",
                    "Asset",
                    "Stock Reservation Entry",
                ],
            ),
            DashboardSection::labeled(
                "Reference",
                vec![
                    "Material Request",
                    "Purchase Order",
                    "Quality Inspection",
                    "Project",
                ],
            ),
            DashboardSection::labeled("Returns", vec!["Purchase Receipt"]),
            DashboardSection::labeled("Subscription", vec!["Auto Repeat"]),
        ])
}
