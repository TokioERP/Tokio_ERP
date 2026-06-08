use crate::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("delivery_note")
        .non_standard_fieldnames(vec![
            ("Stock Entry", "delivery_note_no"),
            ("Quality Inspection", "reference_name"),
            ("Auto Repeat", "reference_document"),
            ("Purchase Receipt", "inter_company_reference"),
        ])
        .internal_links(vec![
            ("Sales Order", vec!["items", "against_sales_order"]),
            ("Material Request", vec!["items", "material_request"]),
            ("Purchase Order", vec!["items", "purchase_order"]),
        ])
        .internal_and_external_links(vec![(
            "Sales Invoice",
            vec!["items", "against_sales_invoice"],
        )])
        .transactions(vec![
            DashboardSection::labeled(
                "Related",
                vec!["Sales Invoice", "Packing Slip", "Delivery Trip"],
            ),
            DashboardSection::labeled(
                "Reference",
                vec!["Sales Order", "Shipment", "Quality Inspection"],
            ),
            DashboardSection::labeled("Returns", vec!["Stock Entry"]),
            DashboardSection::labeled("Subscription", vec!["Auto Repeat"]),
            DashboardSection::labeled(
                "Internal Transfer",
                vec!["Material Request", "Purchase Order", "Purchase Receipt"],
            ),
        ])
}
