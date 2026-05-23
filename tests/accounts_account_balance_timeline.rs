use tokio_erp::erpnext::accounts::dashboard_chart_source::account_balance_timeline::account_balance_timeline::{
    build_account_balance_timeline_chart, build_result, get_dates_from_timegrain, get_gl_entries_plan,
    AccountBalanceTimelineChart, AccountBalanceTimelineDataset, AccountBalanceTimelineRequest,
    AccountBalanceTimelineResult, GlEntry,
};

#[test]
fn account_balance_timeline_dates_match_erpnext_timegrain_endings() {
    assert_eq!(
        get_dates_from_timegrain("2026-01-10", "2026-04-02", "Monthly"),
        vec!["2026-01-31", "2026-02-28", "2026-03-31", "2026-04-30"]
    );
    assert_eq!(
        get_dates_from_timegrain("2026-02-05", "2026-07-01", "Quarterly"),
        vec!["2026-03-31", "2026-06-30", "2026-09-30"]
    );
    assert_eq!(
        get_dates_from_timegrain("2026-05-20", "2026-06-02", "Weekly"),
        vec!["2026-05-23", "2026-05-30", "2026-06-06"]
    );
    assert_eq!(
        get_dates_from_timegrain("2026-05-20", "2026-05-22", "Daily"),
        vec!["2026-05-20", "2026-05-21", "2026-05-22"]
    );
}

#[test]
fn account_balance_timeline_build_result_matches_erpnext_balance_rules() {
    let dates = vec![
        "2026-01-31".to_string(),
        "2026-02-28".to_string(),
        "2026-03-31".to_string(),
    ];
    let entries = vec![
        GlEntry::new("2026-01-05", 100.0, 10.0),
        GlEntry::new("2026-02-01", 20.0, 50.0),
        GlEntry::new("2026-03-20", 40.0, 5.0),
    ];

    assert_eq!(
        build_result("Cash - TC", "Asset", &dates, &entries),
        vec![
            AccountBalanceTimelineResult::new("2026-01-31", 90.0),
            AccountBalanceTimelineResult::new("2026-02-28", 60.0),
            AccountBalanceTimelineResult::new("2026-03-31", 95.0),
        ]
    );
    assert_eq!(
        build_result("Income - TC", "Income", &dates, &entries),
        vec![
            AccountBalanceTimelineResult::new("2026-01-31", -90.0),
            AccountBalanceTimelineResult::new("2026-02-28", 30.0),
            AccountBalanceTimelineResult::new("2026-03-31", -35.0),
        ]
    );
}

#[test]
fn account_balance_timeline_gl_entry_plan_matches_erpnext_filters() {
    let plan = get_gl_entries_plan(
        "Cash - TC",
        "2026-04-30",
        &["Bank - TC".to_string(), "Petty Cash - TC".to_string()],
    );

    assert_eq!(plan.doctype, "GL Entry");
    assert_eq!(
        plan.fields,
        [
            "posting_date".to_string(),
            "debit".to_string(),
            "credit".to_string()
        ]
    );
    assert_eq!(
        plan.to_date_filter,
        (
            "posting_date".to_string(),
            "<".to_string(),
            "2026-04-30".to_string()
        )
    );
    assert_eq!(
        plan.account_filter,
        (
            "account".to_string(),
            "in".to_string(),
            vec![
                "Bank - TC".to_string(),
                "Petty Cash - TC".to_string(),
                "Cash - TC".to_string(),
            ],
        )
    );
    assert_eq!(
        plan.voucher_type_filter,
        (
            "voucher_type".to_string(),
            "!=".to_string(),
            "Period Closing Voucher".to_string(),
        )
    );
    assert_eq!(plan.order_by, "posting_date asc");
}

#[test]
fn account_balance_timeline_request_validation_matches_erpnext_chart_errors() {
    let missing_account = AccountBalanceTimelineRequest {
        chart_name: Some("Cash Balance".to_string()),
        account: None,
        from_date: Some("2026-01-01".to_string()),
        to_date: Some("2026-03-31".to_string()),
        timespan: "Last Quarter".to_string(),
        time_interval: "Monthly".to_string(),
    };

    assert_eq!(
        missing_account.validate(true),
        Err("Account is not set for the dashboard chart Dashboard Chart/Cash Balance".to_string())
    );

    let missing_account_doc = AccountBalanceTimelineRequest {
        account: Some("Missing - TC".to_string()),
        ..missing_account
    };
    assert_eq!(
        missing_account_doc.validate(false),
        Err(
            "Account Missing - TC does not exists in the dashboard chart Dashboard Chart/Cash Balance"
                .to_string()
        )
    );
}

#[test]
fn account_balance_timeline_chart_shape_matches_erpnext_get_response() {
    let dates = vec!["2026-01-31".to_string(), "2026-02-28".to_string()];
    let entries = vec![
        GlEntry::new("2026-01-01", 10.0, 0.0),
        GlEntry::new("2026-02-01", 5.0, 0.0),
    ];

    assert_eq!(
        build_account_balance_timeline_chart("Cash - TC", "Asset", &dates, &entries),
        AccountBalanceTimelineChart {
            labels: vec!["2026-01-31".to_string(), "2026-02-28".to_string()],
            datasets: vec![AccountBalanceTimelineDataset {
                name: "Cash - TC".to_string(),
                values: vec![10.0, 15.0],
            }],
        }
    );
}
