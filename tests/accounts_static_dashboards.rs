use tokio_erp::erpnext::accounts::doctype::bank::bank_dashboard::bank_dashboard;
use tokio_erp::erpnext::accounts::doctype::cost_center::cost_center_dashboard::cost_center_dashboard;
use tokio_erp::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};
use tokio_erp::erpnext::accounts::doctype::exchange_rate_revaluation::exchange_rate_revaluation_dashboard::exchange_rate_revaluation_dashboard;
use tokio_erp::erpnext::accounts::doctype::finance_book::finance_book_dashboard::get_data as finance_book_dashboard;
use tokio_erp::erpnext::accounts::doctype::invoice_discounting::invoice_discounting_dashboard::get_data as invoice_discounting_dashboard;
use tokio_erp::erpnext::accounts::doctype::item_tax_template::item_tax_template_dashboard::get_data as item_tax_template_dashboard;
use tokio_erp::erpnext::accounts::doctype::loyalty_program::loyalty_program_dashboard::loyalty_program_dashboard;
use tokio_erp::erpnext::accounts::doctype::monthly_distribution::monthly_distribution_dashboard::get_data as monthly_distribution_dashboard;
use tokio_erp::erpnext::accounts::doctype::payment_gateway_account::payment_gateway_account_dashboard::payment_gateway_account_dashboard;
use tokio_erp::erpnext::accounts::doctype::payment_order::payment_order_dashboard::payment_order_dashboard;
use tokio_erp::erpnext::accounts::doctype::payment_request::payment_request_dashboard::get_data as payment_request_dashboard;
use tokio_erp::erpnext::accounts::doctype::promotional_scheme::promotional_scheme_dashboard::get_data as promotional_scheme_dashboard;
use tokio_erp::erpnext::accounts::doctype::purchase_invoice::purchase_invoice_dashboard::get_data as purchase_invoice_dashboard;

#[test]
fn selected_accounts_dashboards_match_erpnext_static_get_data() {
    assert_eq!(
        exchange_rate_revaluation_dashboard(),
        DashboardData::new("reference_name")
            .transactions(vec![DashboardSection::items(vec!["Journal Entry",])])
    );
    assert_eq!(
        loyalty_program_dashboard(),
        DashboardData::new("loyalty_program").transactions(vec![DashboardSection::items(vec![
            "Sales Invoice",
            "Customer",
        ])])
    );
    assert_eq!(
        payment_order_dashboard(),
        DashboardData::new("payment_order").transactions(vec![DashboardSection::items(vec![
            "Payment Entry",
            "Journal Entry",
        ])])
    );
    assert_eq!(
        payment_gateway_account_dashboard(),
        DashboardData::new("payment_gateway_account")
            .non_standard_fieldnames(vec![("Subscription Plan", "payment_gateway")])
            .transactions(vec![
                DashboardSection::items(vec!["Payment Request"]),
                DashboardSection::items(vec!["Subscription Plan"]),
            ])
    );
    assert_eq!(
        bank_dashboard(),
        DashboardData::new("bank").transactions(vec![DashboardSection::labeled(
            "Bank Details",
            vec!["Bank Account", "Bank Guarantee"],
        )])
    );
    assert_eq!(
        cost_center_dashboard(),
        DashboardData::new("cost_center").reports(vec![DashboardSection::labeled(
            "Reports",
            vec!["Budget Variance Report", "General Ledger"],
        )])
    );
    assert_eq!(
        promotional_scheme_dashboard(),
        DashboardData::new("promotional_scheme").transactions(vec![DashboardSection::labeled(
            "Reference",
            vec!["Pricing Rule"],
        )])
    );
    assert_eq!(
        finance_book_dashboard(),
        DashboardData::new("finance_book")
            .non_standard_fieldnames(vec![
                ("Asset", "default_finance_book"),
                ("Company", "default_finance_book"),
            ])
            .transactions(vec![
                DashboardSection::labeled("Assets", vec!["Asset", "Asset Value Adjustment"]),
                DashboardSection::items(vec!["Company"]),
                DashboardSection::items(vec!["Journal Entry"]),
            ])
    );
    assert_eq!(
        invoice_discounting_dashboard(),
        DashboardData::new("reference_name")
            .internal_links(vec![("Sales Invoice", vec!["invoices", "sales_invoice"])])
            .transactions(vec![
                DashboardSection::labeled("Reference", vec!["Sales Invoice"]),
                DashboardSection::labeled("Payment", vec!["Payment Entry", "Journal Entry"]),
            ])
    );
    assert_eq!(
        item_tax_template_dashboard(),
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
    );
    assert_eq!(
        monthly_distribution_dashboard(),
        DashboardData::new("monthly_distribution")
            .non_standard_fieldnames(vec![
                ("Sales Person", "distribution_id"),
                ("Territory", "distribution_id"),
                ("Sales Partner", "distribution_id"),
            ])
            .transactions(vec![DashboardSection::labeled(
                "Target Details",
                vec!["Sales Person", "Territory", "Sales Partner"],
            )])
    );
    assert_eq!(
        payment_request_dashboard(),
        DashboardData::new("payment_request")
            .internal_links(vec![
                ("Payment Entry", vec!["references", "payment_request"]),
                ("Payment Order", vec!["references", "payment_order"]),
            ])
            .transactions(vec![DashboardSection::labeled(
                "Payment",
                vec!["Payment Entry", "Payment Order"],
            )])
    );
    assert_eq!(
        purchase_invoice_dashboard(),
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
    );
}
