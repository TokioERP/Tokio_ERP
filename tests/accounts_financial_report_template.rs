use tokio_erp::erpnext::accounts::financial_report_template::{
    FinancialReportTemplate, FINANCIAL_REPORT_TEMPLATES,
};

#[test]
fn financial_report_template_registry_matches_erpnext_static_templates() {
    assert_eq!(
        FINANCIAL_REPORT_TEMPLATES,
        [
            FinancialReportTemplate {
                template_name: "Financial Ratios Analysis",
                module: "Accounts",
                report_type: "Custom Financial Statement",
                rows: 63,
            },
            FinancialReportTemplate {
                template_name: "Horizontal Balance Sheet (Columnar)",
                module: "Accounts",
                report_type: "Balance Sheet",
                rows: 64,
            },
            FinancialReportTemplate {
                template_name: "Horizontal Profit and Loss (Columnar)",
                module: "Accounts",
                report_type: "Profit and Loss Statement",
                rows: 66,
            },
            FinancialReportTemplate {
                template_name: "Standard Balance Sheet (IFRS)",
                module: "Accounts",
                report_type: "Balance Sheet",
                rows: 53,
            },
            FinancialReportTemplate {
                template_name: "Standard Cash Flow Statement (IFRS)",
                module: "Accounts",
                report_type: "Cash Flow",
                rows: 48,
            },
            FinancialReportTemplate {
                template_name: "Standard Profit and Loss (IFRS)",
                module: "Accounts",
                report_type: "Profit and Loss Statement",
                rows: 26,
            },
        ]
    );
}

#[test]
fn financial_report_template_init_modules_are_noop_markers() {
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::MODULE,
        "Accounts"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::financial_ratios_analysis::TEMPLATE_NAME,
        "Financial Ratios Analysis"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::horizontal_balance_sheet_columnar::TEMPLATE_NAME,
        "Horizontal Balance Sheet (Columnar)"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::horizontal_profit_and_loss_columnar::TEMPLATE_NAME,
        "Horizontal Profit and Loss (Columnar)"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::standard_balance_sheet_ifrs::TEMPLATE_NAME,
        "Standard Balance Sheet (IFRS)"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::standard_cash_flow_statement_ifrs::TEMPLATE_NAME,
        "Standard Cash Flow Statement (IFRS)"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::financial_report_template::standard_profit_and_loss_ifrs::TEMPLATE_NAME,
        "Standard Profit and Loss (IFRS)"
    );
}
