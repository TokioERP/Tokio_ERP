use tokio_erp::erpnext::accounts::report::share_ledger::share_ledger::{
    execute, get_all_transfers_query_plan, get_columns, ShareLedgerError, ShareLedgerFilters,
    ShareLedgerRow, ShareTransfer,
};

#[test]
fn share_ledger_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            "Shareholder:Link/Shareholder:150",
            "Date:Date:100",
            "Transfer Type::140",
            "Share Type::90",
            "No of Shares::90",
            "Rate:Currency:90",
            "Amount:Currency:90",
            "Company::150",
            "Share Transfer:Link/Share Transfer:90",
        ]
    );
}

#[test]
fn share_ledger_requires_date_like_erpnext() {
    let err = execute(ShareLedgerFilters::new(None, Some("SH-0001")), vec![]).unwrap_err();
    assert_eq!(err, ShareLedgerError::MissingDate);
}

#[test]
fn share_ledger_returns_no_rows_when_shareholder_filter_is_missing() {
    let report = execute(ShareLedgerFilters::new(Some("2026-05-24"), None), vec![]).unwrap();

    assert_eq!(report.columns, get_columns());
    assert!(report.rows.is_empty());
}

#[test]
fn share_ledger_transfer_rows_match_erpnext_to_and_from_suffixes() {
    let transfers = vec![
        ShareTransfer::new(
            "ST-0001",
            "2026-05-01",
            "Transfer",
            "Ordinary",
            10.0,
            5.0,
            50.0,
            "Test Company",
            "SH-0001",
            "SH-0002",
        ),
        ShareTransfer::new(
            "ST-0002",
            "2026-05-02",
            "Transfer",
            "Ordinary",
            4.0,
            7.5,
            30.0,
            "Test Company",
            "SH-0003",
            "SH-0001",
        ),
        ShareTransfer::new(
            "ST-0003",
            "2026-05-03",
            "Issue",
            "Preference",
            2.0,
            11.0,
            22.0,
            "Test Company",
            "",
            "SH-0001",
        ),
    ];

    let report = execute(
        ShareLedgerFilters::new(Some("2026-05-24"), Some("SH-0001")),
        transfers,
    )
    .unwrap();

    assert_eq!(
        report.rows,
        vec![
            ShareLedgerRow {
                shareholder: "SH-0001".to_string(),
                date: "2026-05-01".to_string(),
                transfer_type: "Transfer to SH-0002".to_string(),
                share_type: "Ordinary".to_string(),
                no_of_shares: 10.0,
                rate: 5.0,
                amount: 50.0,
                company: "Test Company".to_string(),
                share_transfer: "ST-0001".to_string(),
            },
            ShareLedgerRow {
                shareholder: "SH-0001".to_string(),
                date: "2026-05-02".to_string(),
                transfer_type: "Transfer from SH-0003".to_string(),
                share_type: "Ordinary".to_string(),
                no_of_shares: 4.0,
                rate: 7.5,
                amount: 30.0,
                company: "Test Company".to_string(),
                share_transfer: "ST-0002".to_string(),
            },
            ShareLedgerRow {
                shareholder: "SH-0001".to_string(),
                date: "2026-05-03".to_string(),
                transfer_type: "Issue".to_string(),
                share_type: "Preference".to_string(),
                no_of_shares: 2.0,
                rate: 11.0,
                amount: 22.0,
                company: "Test Company".to_string(),
                share_transfer: "ST-0003".to_string(),
            },
        ]
    );
}

#[test]
fn share_ledger_query_plan_matches_erpnext_sql_filters() {
    let plan = get_all_transfers_query_plan();

    assert_eq!(
        plan.date_filter,
        "DATE(date) <= filters.date on both from_shareholder and to_shareholder branches"
    );
    assert_eq!(
        plan.shareholder_filter,
        "from_shareholder = filters.shareholder or to_shareholder = filters.shareholder"
    );
    assert_eq!(plan.docstatus_filter, "docstatus = 1");
    assert_eq!(plan.order_by, "date");
    assert_eq!(plan.company_filter, "unused placeholder condition only");
}
