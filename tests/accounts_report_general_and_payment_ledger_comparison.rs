use tokio_erp::erpnext::accounts::report::general_and_payment_ledger_comparison::general_and_payment_ledger_comparison::{
    execute, get_columns, AccountRecord, ComparisonFilters, ComparisonQueryPlan,
    GeneralLedgerEntry, LedgerComparisonRow, PaymentLedgerComparisonEntry, ReportColumn,
};

fn filters() -> ComparisonFilters {
    ComparisonFilters {
        company: "_Test Company".to_string(),
        account: Vec::new(),
        voucher_no: None,
        period_start_date: None,
        period_end_date: None,
        party_type: None,
        party: None,
    }
}

fn accounts() -> Vec<AccountRecord> {
    vec![
        AccountRecord::new("Debtors - TC", "_Test Company", "Receivable"),
        AccountRecord::new("Creditors - TC", "_Test Company", "Payable"),
        AccountRecord::new("Bank - TC", "_Test Company", "Bank"),
        AccountRecord::new("Debtors - OC", "Other Company", "Receivable"),
    ]
}

#[test]
fn general_payment_ledger_comparison_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::new("Company", "company", "Link", Some("Company"), "100"),
            ReportColumn::new("Account", "account", "Link", Some("Account"), "100"),
            ReportColumn::new("Voucher Type", "voucher_type", "Data", None, "100"),
            ReportColumn::new(
                "Voucher No",
                "voucher_no",
                "Dynamic Link",
                Some("voucher_type"),
                "100",
            ),
            ReportColumn::new("Party Type", "party_type", "Data", None, "100"),
            ReportColumn::new("Party", "party", "Dynamic Link", Some("party_type"), "100",),
            ReportColumn::new(
                "GL Balance",
                "gl_balance",
                "Currency",
                Some("Company:company:default_currency"),
                "100",
            ),
            ReportColumn::new(
                "Payment Ledger Balance",
                "pl_balance",
                "Currency",
                Some("Company:company:default_currency"),
                "100",
            ),
        ]
    );
}

#[test]
fn general_payment_ledger_comparison_query_plan_matches_erpnext_grouping_and_filters() {
    let mut report_filters = filters();
    report_filters.account = vec!["Debtors - TC".to_string()];
    report_filters.voucher_no = Some("SINV-0001".to_string());
    report_filters.period_start_date = Some("2026-01-01".to_string());
    report_filters.period_end_date = Some("2026-01-31".to_string());
    report_filters.party_type = Some("Customer".to_string());
    report_filters.party = Some("_Test Customer".to_string());

    assert_eq!(
        ComparisonQueryPlan::for_filters(&report_filters),
        ComparisonQueryPlan {
            account_doctype: "Account",
            account_filters: vec![
                "company = filters.company".to_string(),
                "account_type in ['Receivable', 'Payable']".to_string(),
                "name in filters.account".to_string(),
            ],
            gl_doctype: "GL Entry",
            gl_base_filters: vec![
                "company = filters.company".to_string(),
                "is_cancelled = 0".to_string(),
                "account in account_type.accounts".to_string(),
            ],
            ple_doctype: "Payment Ledger Entry",
            ple_base_filters: vec![
                "company = filters.company".to_string(),
                "delinked = 0".to_string(),
                "account in account_type.accounts".to_string(),
            ],
            optional_filters: vec![
                "voucher_no = filters.voucher_no".to_string(),
                "posting_date >= filters.period_start_date".to_string(),
                "posting_date <= filters.period_end_date".to_string(),
                "party_type = filters.party_type".to_string(),
                "party = filters.party".to_string(),
            ],
            group_by: vec![
                "company",
                "account",
                "voucher_type",
                "voucher_no",
                "party_type",
                "party",
            ],
        }
    );
}

#[test]
fn general_payment_ledger_comparison_applies_account_and_runtime_filters() {
    let mut report_filters = filters();
    report_filters.account = vec!["Debtors - TC".to_string()];
    report_filters.voucher_no = Some("SINV-0001".to_string());
    report_filters.period_start_date = Some("2026-01-01".to_string());
    report_filters.period_end_date = Some("2026-01-31".to_string());
    report_filters.party_type = Some("Customer".to_string());
    report_filters.party = Some("_Test Customer".to_string());

    let report = execute(
        &report_filters,
        &accounts(),
        vec![
            GeneralLedgerEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Sales Invoice",
                "SINV-0001",
                "Customer",
                "_Test Customer",
                "2026-01-15",
                100.0,
                0.0,
                false,
            ),
            GeneralLedgerEntry::new(
                "_Test Company",
                "Creditors - TC",
                "Purchase Invoice",
                "PINV-0001",
                "Supplier",
                "_Test Supplier",
                "2026-01-15",
                0.0,
                80.0,
                false,
            ),
            GeneralLedgerEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Sales Invoice",
                "SINV-0002",
                "Customer",
                "_Test Customer",
                "2026-01-15",
                999.0,
                0.0,
                false,
            ),
        ],
        vec![PaymentLedgerComparisonEntry::new(
            "_Test Company",
            "Debtors - TC",
            "Sales Invoice",
            "SINV-0001",
            "Customer",
            "_Test Customer",
            "2026-01-15",
            99.0,
            false,
        )],
    );

    assert_eq!(report.rows.len(), 1);
    assert_eq!(
        report.rows[0],
        LedgerComparisonRow {
            company: "_Test Company".to_string(),
            account: "Debtors - TC".to_string(),
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: "SINV-0001".to_string(),
            party_type: "Customer".to_string(),
            party: "_Test Customer".to_string(),
            gl_balance: Some(100.0),
            pl_balance: Some(99.0),
        }
    );
}

#[test]
fn general_payment_ledger_comparison_uses_receivable_and_payable_outstanding_formulas() {
    let report = execute(
        &filters(),
        &accounts(),
        vec![
            GeneralLedgerEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Sales Invoice",
                "SINV-0001",
                "Customer",
                "_Test Customer",
                "2026-01-15",
                120.0,
                20.0,
                false,
            ),
            GeneralLedgerEntry::new(
                "_Test Company",
                "Creditors - TC",
                "Purchase Invoice",
                "PINV-0001",
                "Supplier",
                "_Test Supplier",
                "2026-01-16",
                30.0,
                90.0,
                false,
            ),
        ],
        vec![
            PaymentLedgerComparisonEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Sales Invoice",
                "SINV-0001",
                "Customer",
                "_Test Customer",
                "2026-01-15",
                99.0,
                false,
            ),
            PaymentLedgerComparisonEntry::new(
                "_Test Company",
                "Creditors - TC",
                "Purchase Invoice",
                "PINV-0001",
                "Supplier",
                "_Test Supplier",
                "2026-01-16",
                55.0,
                false,
            ),
        ],
    );

    assert_eq!(report.rows.len(), 2);
    let sinv = report
        .rows
        .iter()
        .find(|row| row.voucher_no == "SINV-0001")
        .expect("sales invoice row");
    let pinv = report
        .rows
        .iter()
        .find(|row| row.voucher_no == "PINV-0001")
        .expect("purchase invoice row");
    assert_eq!(sinv.gl_balance, Some(100.0));
    assert_eq!(sinv.pl_balance, Some(99.0));
    assert_eq!(pinv.gl_balance, Some(60.0));
    assert_eq!(pinv.pl_balance, Some(55.0));
}

#[test]
fn general_payment_ledger_comparison_reports_gl_only_and_ple_only_variations() {
    let report = execute(
        &filters(),
        &accounts(),
        vec![GeneralLedgerEntry::new(
            "_Test Company",
            "Debtors - TC",
            "Sales Invoice",
            "SINV-0001",
            "Customer",
            "_Test Customer",
            "2026-01-15",
            100.0,
            0.0,
            false,
        )],
        vec![PaymentLedgerComparisonEntry::new(
            "_Test Company",
            "Debtors - TC",
            "Sales Invoice",
            "SINV-0002",
            "Customer",
            "_Test Customer",
            "2026-01-16",
            80.0,
            false,
        )],
    );

    assert_eq!(report.rows.len(), 2);
    assert_eq!(report.rows[0].voucher_no, "SINV-0001");
    assert_eq!(report.rows[0].gl_balance, Some(100.0));
    assert_eq!(report.rows[0].pl_balance, None);
    assert_eq!(report.rows[1].voucher_no, "SINV-0002");
    assert_eq!(report.rows[1].gl_balance, Some(0.0));
    assert_eq!(report.rows[1].pl_balance, Some(80.0));
}
