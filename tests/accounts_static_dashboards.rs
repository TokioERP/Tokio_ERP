use tokio_erp::erpnext::accounts::doctype::bank::bank_dashboard::bank_dashboard;
use tokio_erp::erpnext::accounts::doctype::cost_center::cost_center_dashboard::cost_center_dashboard;
use tokio_erp::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};
use tokio_erp::erpnext::accounts::doctype::exchange_rate_revaluation::exchange_rate_revaluation_dashboard::exchange_rate_revaluation_dashboard;
use tokio_erp::erpnext::accounts::doctype::finance_book::finance_book_dashboard::get_data as finance_book_dashboard;
use tokio_erp::erpnext::accounts::doctype::loyalty_program::loyalty_program_dashboard::loyalty_program_dashboard;
use tokio_erp::erpnext::accounts::doctype::payment_gateway_account::payment_gateway_account_dashboard::payment_gateway_account_dashboard;
use tokio_erp::erpnext::accounts::doctype::payment_order::payment_order_dashboard::payment_order_dashboard;
use tokio_erp::erpnext::accounts::doctype::promotional_scheme::promotional_scheme_dashboard::get_data as promotional_scheme_dashboard;

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
}
