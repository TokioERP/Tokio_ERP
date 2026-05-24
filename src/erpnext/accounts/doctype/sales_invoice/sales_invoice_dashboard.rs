use crate::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};

pub fn get_data() -> DashboardData {
    DashboardData::new("sales_invoice")
        .non_standard_fieldnames(vec![
            ("Delivery Note", "against_sales_invoice"),
            ("Journal Entry", "reference_name"),
            ("Payment Entry", "reference_name"),
            ("Payment Request", "reference_name"),
            ("Sales Invoice", "return_against"),
            ("Auto Repeat", "reference_document"),
            ("Purchase Invoice", "inter_company_invoice_reference"),
        ])
        .internal_links(vec![
            ("Sales Order", vec!["items", "sales_order"]),
            ("Timesheet", vec!["timesheets", "time_sheet"]),
        ])
        .internal_and_external_links(vec![("Delivery Note", vec!["items", "delivery_note"])])
        .transactions(vec![
            DashboardSection::labeled(
                "Payment",
                vec![
                    "Payment Entry",
                    "Payment Request",
                    "Journal Entry",
                    "Invoice Discounting",
                    "Dunning",
                ],
            ),
            DashboardSection::labeled(
                "Reference",
                vec!["Timesheet", "Delivery Note", "Sales Order"],
            ),
            DashboardSection::labeled("Returns", vec!["Sales Invoice"]),
            DashboardSection::labeled("Subscription", vec!["Auto Repeat"]),
            DashboardSection::labeled("Internal Transfers", vec!["Purchase Invoice"]),
        ])
}
