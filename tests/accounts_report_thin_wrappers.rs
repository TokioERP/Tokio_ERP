use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::accounts_payable::accounts_payable;
use tokio_erp::erpnext::accounts::report::accounts_payable_summary::accounts_payable_summary;
use tokio_erp::erpnext::accounts::report::custom_financial_statement::custom_financial_statement;
use tokio_erp::erpnext::accounts::report::purchase_invoice_trends::purchase_invoice_trends;
use tokio_erp::erpnext::accounts::report::sales_invoice_trends::sales_invoice_trends;
use tokio_erp::erpnext::accounts::report::supplier_ledger_summary::supplier_ledger_summary;
use tokio_erp::erpnext::accounts::report::{DelegatedReportExecution, ReportArg};

#[test]
fn payable_report_wrappers_match_erpnext_delegate_args() {
    let filters = BTreeMap::from([("company".to_string(), "_Test Company".to_string())]);

    assert_eq!(
        accounts_payable::execute(Some(filters.clone())),
        DelegatedReportExecution {
            delegate: "ReceivablePayableReport",
            filters: Some(filters.clone()),
            args: vec![
                ("account_type", ReportArg::Text("Payable")),
                (
                    "naming_by",
                    ReportArg::List(&["Buying Settings", "supp_master_name"])
                ),
            ],
        }
    );

    assert_eq!(
        accounts_payable_summary::execute(Some(filters)),
        DelegatedReportExecution {
            delegate: "AccountsReceivableSummary",
            filters: Some(BTreeMap::from([(
                "company".to_string(),
                "_Test Company".to_string()
            )])),
            args: vec![
                ("account_type", ReportArg::Text("Payable")),
                (
                    "naming_by",
                    ReportArg::List(&["Buying Settings", "supp_master_name"])
                ),
            ],
        }
    );
}

#[test]
fn supplier_ledger_summary_matches_erpnext_delegate_args() {
    assert_eq!(
        supplier_ledger_summary::execute(None),
        DelegatedReportExecution {
            delegate: "PartyLedgerSummaryReport",
            filters: None,
            args: vec![
                ("party_type", ReportArg::Text("Supplier")),
                (
                    "naming_by",
                    ReportArg::List(&["Buying Settings", "supp_master_name"])
                ),
            ],
        }
    );
}

#[test]
fn invoice_trend_wrappers_preserve_document_type_and_empty_filter_default() {
    assert_eq!(
        purchase_invoice_trends::execute(None).document_type,
        "Purchase Invoice"
    );
    assert!(purchase_invoice_trends::execute(None).filters.is_empty());
    assert_eq!(
        sales_invoice_trends::execute(None).document_type,
        "Sales Invoice"
    );
    assert!(sales_invoice_trends::execute(None).filters.is_empty());

    let filters = BTreeMap::from([("period".to_string(), "Monthly".to_string())]);
    let plan = sales_invoice_trends::execute(Some(filters.clone()));
    assert_eq!(plan.filters, filters);
    assert_eq!(
        plan.columns_source,
        "erpnext.controllers.trends.get_columns"
    );
    assert_eq!(plan.data_source, "erpnext.controllers.trends.get_data");
}

#[test]
fn custom_financial_statement_only_runs_when_report_template_is_set() {
    assert_eq!(custom_financial_statement::execute(None), None);
    assert_eq!(
        custom_financial_statement::execute(Some(
            custom_financial_statement::CustomFinancialStatementFilters::default()
        )),
        None
    );

    let filters = custom_financial_statement::CustomFinancialStatementFilters {
        report_template: Some("Standard Balance Sheet (IFRS)".to_string()),
    };
    let plan = custom_financial_statement::execute(Some(filters.clone())).unwrap();
    assert_eq!(plan.delegate, "FinancialReportEngine");
    assert_eq!(plan.method, "execute");
    assert_eq!(plan.filters, filters);
    assert_eq!(
        custom_financial_statement::XLSX_STYLES_HOOK,
        "get_xlsx_styles"
    );
}
