pub mod financial_ratios_analysis;
#[path = "horizontal_balance_sheet_(columnar)/mod.rs"]
pub mod horizontal_balance_sheet_columnar;
#[path = "horizontal_profit_and_loss_(columnar)/mod.rs"]
pub mod horizontal_profit_and_loss_columnar;
#[path = "standard_balance_sheet_(ifrs)/mod.rs"]
pub mod standard_balance_sheet_ifrs;
#[path = "standard_cash_flow_statement_(ifrs)/mod.rs"]
pub mod standard_cash_flow_statement_ifrs;
#[path = "standard_profit_and_loss_(ifrs)/mod.rs"]
pub mod standard_profit_and_loss_ifrs;

pub const MODULE: &str = "Accounts";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialReportTemplate {
    pub template_name: &'static str,
    pub module: &'static str,
    pub report_type: &'static str,
    pub rows: usize,
}

pub const FINANCIAL_REPORT_TEMPLATES: [FinancialReportTemplate; 6] = [
    FinancialReportTemplate {
        template_name: financial_ratios_analysis::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Custom Financial Statement",
        rows: 63,
    },
    FinancialReportTemplate {
        template_name: horizontal_balance_sheet_columnar::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Balance Sheet",
        rows: 64,
    },
    FinancialReportTemplate {
        template_name: horizontal_profit_and_loss_columnar::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Profit and Loss Statement",
        rows: 66,
    },
    FinancialReportTemplate {
        template_name: standard_balance_sheet_ifrs::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Balance Sheet",
        rows: 53,
    },
    FinancialReportTemplate {
        template_name: standard_cash_flow_statement_ifrs::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Cash Flow",
        rows: 48,
    },
    FinancialReportTemplate {
        template_name: standard_profit_and_loss_ifrs::TEMPLATE_NAME,
        module: MODULE,
        report_type: "Profit and Loss Statement",
        rows: 26,
    },
];
