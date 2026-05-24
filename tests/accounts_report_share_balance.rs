use tokio_erp::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;
use tokio_erp::erpnext::accounts::report::share_balance::share_balance::{
    execute, get_all_shares_query_plan, get_columns, ShareBalanceError, ShareBalanceFilters,
    ShareBalanceRow,
};

#[test]
fn share_balance_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            "Shareholder:Link/Shareholder:150",
            "Share Type::90",
            "No of Shares::90",
            "Average Rate:Currency:90",
            "Amount:Currency:90",
        ]
    );
}

#[test]
fn share_balance_requires_date_like_erpnext() {
    let err = execute(
        ShareBalanceFilters::new(None, Some("SH-0001")),
        vec![ShareBalance::new("Ordinary", 1, 10, 5)],
    )
    .unwrap_err();

    assert_eq!(err, ShareBalanceError::MissingDate);
}

#[test]
fn share_balance_returns_no_rows_when_shareholder_filter_is_missing() {
    let report = execute(
        ShareBalanceFilters::new(Some("2026-05-24"), None),
        vec![ShareBalance::new("Ordinary", 1, 10, 5)],
    )
    .unwrap();

    assert_eq!(report.columns, get_columns());
    assert!(report.rows.is_empty());
}

#[test]
fn share_balance_get_all_shares_query_plan_matches_erpnext_document_child_table_lookup() {
    assert_eq!(
        get_all_shares_query_plan("SH-0001"),
        "frappe.get_doc(\"Shareholder\", \"SH-0001\").share_balance"
    );
}

#[test]
fn share_balance_groups_rows_by_share_type_and_recomputes_average_rate() {
    let report = execute(
        ShareBalanceFilters::new(Some("2026-05-24"), Some("SH-0001")),
        vec![
            ShareBalance::new("Ordinary", 1, 10, 5),
            ShareBalance::new("Preference", 11, 15, 8),
            ShareBalance::new("Ordinary", 16, 20, 7),
        ],
    )
    .unwrap();

    assert_eq!(
        report.rows,
        vec![
            ShareBalanceRow {
                shareholder: "SH-0001".to_string(),
                share_type: "Ordinary".to_string(),
                no_of_shares: 15,
                rate: 85.0 / 15.0,
                amount: 85,
            },
            ShareBalanceRow {
                shareholder: "SH-0001".to_string(),
                share_type: "Preference".to_string(),
                no_of_shares: 5,
                rate: 8.0,
                amount: 40,
            },
        ]
    );
}

#[test]
fn share_balance_sets_average_rate_to_zero_when_aggregated_shares_cancel_out() {
    let mut reversal = ShareBalance::new("Ordinary", 11, 15, 5);
    reversal.no_of_shares = -10;
    reversal.amount = -50;

    let report = execute(
        ShareBalanceFilters::new(Some("2026-05-24"), Some("SH-0001")),
        vec![ShareBalance::new("Ordinary", 1, 10, 5), reversal],
    )
    .unwrap();

    assert_eq!(
        report.rows,
        vec![ShareBalanceRow {
            shareholder: "SH-0001".to_string(),
            share_type: "Ordinary".to_string(),
            no_of_shares: 0,
            rate: 0.0,
            amount: 0,
        }]
    );
}
