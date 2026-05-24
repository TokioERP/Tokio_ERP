use std::collections::BTreeMap;

pub mod account_balance;
pub mod accounts_payable;
pub mod accounts_payable_summary;
pub mod accounts_receivable;
pub mod accounts_receivable_summary;
pub mod asset_depreciation_ledger;
pub mod asset_depreciations_and_balances;
pub mod balance_sheet;
pub mod bank_clearance_summary;
pub mod bank_reconciliation_statement;
pub mod billed_items_to_be_received;
pub mod budget_variance_report;
pub mod calculated_discount_mismatch;
pub mod cash_flow;
pub mod cheques_and_deposits_incorrectly_cleared;
pub mod consolidated_financial_statement;
pub mod consolidated_trial_balance;
pub mod custom_financial_statement;
pub mod customer_ledger_summary;
pub mod deferred_revenue_and_expense;
pub mod delivered_items_to_be_billed;
pub mod dimension_wise_accounts_balance_report;
pub mod financial_ratios;
pub mod general_and_payment_ledger_comparison;
pub mod general_ledger;
pub mod gross_and_net_profit_report;
pub mod gross_profit;
pub mod inactive_sales_items;
pub mod invalid_ledger_entries;
pub mod item_wise_purchase_register;
pub mod item_wise_sales_register;
pub mod payment_ledger;
pub mod payment_period_based_on_invoice_date;
pub mod pos_register;
pub mod profit_and_loss_statement;
pub mod profitability_analysis;
pub mod purchase_invoice_trends;
pub mod purchase_register;
pub mod received_items_to_be_billed;
pub mod sales_invoice_trends;
pub mod sales_partners_commission;
pub mod sales_payment_summary;
pub mod sales_register;
pub mod share_balance;
pub mod share_ledger;
pub mod supplier_ledger_summary;
pub mod tax_withholding_details;
pub mod tds_computation_summary;
pub mod trial_balance;
pub mod trial_balance_for_party;
pub mod trial_balance_simple;
pub mod voucher_wise_balance;

pub type ReportFilters = BTreeMap<String, String>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReportArg {
    Text(&'static str),
    List(&'static [&'static str]),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelegatedReportExecution {
    pub delegate: &'static str,
    pub filters: Option<ReportFilters>,
    pub args: Vec<(&'static str, ReportArg)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrendReportExecution {
    pub document_type: &'static str,
    pub filters: ReportFilters,
    pub columns_source: &'static str,
    pub data_source: &'static str,
}

impl TrendReportExecution {
    pub fn new(document_type: &'static str, filters: Option<ReportFilters>) -> Self {
        Self {
            document_type,
            filters: filters.unwrap_or_default(),
            columns_source: "erpnext.controllers.trends.get_columns",
            data_source: "erpnext.controllers.trends.get_data",
        }
    }
}
