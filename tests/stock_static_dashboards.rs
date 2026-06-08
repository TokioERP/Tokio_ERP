use tokio_erp::erpnext::stock::doctype::dashboard::{DashboardData, DashboardSection};
use tokio_erp::erpnext::stock::doctype::delivery_note::delivery_note_dashboard::get_data as delivery_note_dashboard;
use tokio_erp::erpnext::stock::doctype::material_request::material_request_dashboard::get_data as material_request_dashboard;
use tokio_erp::erpnext::stock::doctype::pick_list::pick_list_dashboard::get_data as pick_list_dashboard;
use tokio_erp::erpnext::stock::doctype::purchase_receipt::purchase_receipt_dashboard::get_data as purchase_receipt_dashboard;
use tokio_erp::erpnext::stock::doctype::stock_closing_entry::stock_closing_entry_dashboard::get_data as stock_closing_entry_dashboard;
use tokio_erp::erpnext::stock::doctype::stock_entry::stock_entry_dashboard::get_data as stock_entry_dashboard;

#[test]
fn stock_closing_entry_dashboard_matches_erpnext_payload() {
    assert_eq!(
        stock_closing_entry_dashboard(),
        DashboardData::new("stock_closing_entry").transactions(vec![DashboardSection::labeled(
            "Stock Closing Log",
            vec!["Stock Closing Balance"],
        )])
    );
}

#[test]
fn stock_entry_dashboard_matches_erpnext_payload() {
    assert_eq!(
        stock_entry_dashboard(),
        DashboardData::new("stock_entry")
            .non_standard_fieldnames(vec![("Stock Reservation Entry", "from_voucher_no")])
            .transactions(vec![DashboardSection::labeled(
                "Stock Reservation",
                vec!["Stock Reservation Entry"],
            )])
    );
}

#[test]
fn material_request_dashboard_matches_erpnext_payload() {
    assert_eq!(
        material_request_dashboard(),
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
    );
}

#[test]
fn pick_list_dashboard_matches_erpnext_payload() {
    assert_eq!(
        pick_list_dashboard(),
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
    );
}

#[test]
fn delivery_note_dashboard_matches_erpnext_payload() {
    assert_eq!(
        delivery_note_dashboard(),
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
    );
}

#[test]
fn purchase_receipt_dashboard_matches_erpnext_payload() {
    assert_eq!(
        purchase_receipt_dashboard(),
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
    );
}
