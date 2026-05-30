use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::customer_ledger_summary::customer_ledger_summary::{
    execute, PartyDetail, PartyLedgerArgs, PartyLedgerColumn, PartyLedgerFilters,
    PartyLedgerGlEntry, PartyLedgerSummaryContext, PartyType,
};
use tokio_erp::erpnext::accounts::report::supplier_ledger_summary::supplier_ledger_summary;
use tokio_erp::erpnext::accounts::report::{DelegatedReportExecution, ReportArg};

fn filters() -> PartyLedgerFilters {
    PartyLedgerFilters {
        company: "_Test Company".to_string(),
        from_date: "2026-05-30".to_string(),
        to_date: "2026-05-30".to_string(),
    }
}

fn supplier_args() -> PartyLedgerArgs {
    PartyLedgerArgs {
        party_type: PartyType::Supplier,
        party_naming_by: "Naming Series".to_string(),
    }
}

fn supplier_context() -> PartyLedgerSummaryContext {
    PartyLedgerSummaryContext {
        party_details: BTreeMap::from([(
            "_Test Supplier".to_string(),
            PartyDetail {
                party: "_Test Supplier".to_string(),
                party_name: "_Test Supplier".to_string(),
                party_group: "All Supplier Groups".to_string(),
                territory: None,
            },
        )]),
        gl_entries: vec![PartyLedgerGlEntry {
            posting_date: "2026-05-30".to_string(),
            party: "_Test Supplier".to_string(),
            voucher_type: "Purchase Invoice".to_string(),
            voucher_no: "PINV-0001".to_string(),
            against_voucher: None,
            debit: 0.0,
            credit: 300.0,
            is_opening: "No".to_string(),
        }],
        company_currency: "INR".to_string(),
        ..PartyLedgerSummaryContext::default()
    }
}

#[test]
fn supplier_ledger_summary_execute_delegates_to_party_ledger_summary_like_erpnext() {
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
fn supplier_ledger_summary_basic_purchase_invoice_row_matches_erpnext_test() {
    let report = execute(&filters(), &supplier_args(), &supplier_context()).unwrap();

    assert_eq!(report.rows.len(), 1);
    let row = &report.rows[0];
    assert_eq!(row.party, "_Test Supplier");
    assert_eq!(row.party_name.as_deref(), Some("_Test Supplier"));
    assert_eq!(row.opening_balance, 0.0);
    assert_eq!(row.invoiced_amount, 300.0);
    assert_eq!(row.paid_amount, 0.0);
    assert_eq!(row.return_amount, 0.0);
    assert_eq!(row.closing_balance, 300.0);
    assert_eq!(row.currency, "INR");
    assert_eq!(row.supplier_group.as_deref(), Some("All Supplier Groups"));
    assert_eq!(row.dr_or_cr.as_deref(), Some("Cr"));
}

#[test]
fn supplier_ledger_summary_columns_and_supplier_group_filter_shape_match_erpnext() {
    let report = execute(&filters(), &supplier_args(), &supplier_context()).unwrap();

    assert_eq!(
        &report.columns[..4],
        [
            PartyLedgerColumn::link("Supplier", "party", "Supplier", 200),
            PartyLedgerColumn::data("Supplier Name", "party_name", 150),
            PartyLedgerColumn::link("Supplier Group", "supplier_group", "Supplier Group", 120),
            PartyLedgerColumn::currency("Opening Balance", "opening_balance", 120),
        ]
    );
    assert!(report
        .columns
        .contains(&PartyLedgerColumn::currency("Debit Note", "return_amount", 120)));
    assert_eq!(
        report.rows[0].supplier_group.as_deref(),
        Some("All Supplier Groups")
    );
}
