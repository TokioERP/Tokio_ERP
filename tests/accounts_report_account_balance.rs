use tokio_erp::erpnext::accounts::report::account_balance::account_balance::{
    execute, get_columns, get_conditions, AccountBalanceAccount, AccountBalanceFilters,
    AccountBalanceRow, ReportColumn,
};

#[test]
fn account_balance_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::link("Account", "account", "Account", 200),
            ReportColumn::link("Currency", "currency", "Currency", 100).hidden(),
            ReportColumn::currency("Balance", "balance", "currency", 100),
        ]
    );
}

#[test]
fn account_balance_conditions_only_include_present_filters() {
    let filters = AccountBalanceFilters {
        company: Some("_Test Company 2".to_string()),
        account_type: None,
        root_type: Some("Income".to_string()),
        report_date: Some("2026-05-24".to_string()),
    };

    assert_eq!(
        get_conditions(&filters),
        vec![("company", "_Test Company 2"), ("root_type", "Income")]
    );
}

#[test]
fn account_balance_execute_sorts_accounts_and_uses_report_date_for_balance() {
    let filters = AccountBalanceFilters {
        company: Some("_Test Company 2".to_string()),
        account_type: None,
        root_type: Some("Income".to_string()),
        report_date: Some("2026-05-24".to_string()),
    };
    let accounts = vec![
        AccountBalanceAccount::new("Sales - _TC2", "EUR"),
        AccountBalanceAccount::new("Direct Income - _TC2", "EUR"),
        AccountBalanceAccount::new("Service - _TC2", "EUR"),
    ];

    let report = execute(filters, accounts, |account, date| match (account, date) {
        ("Direct Income - _TC2", Some("2026-05-24")) => -100.0,
        ("Sales - _TC2", Some("2026-05-24")) => -100.0,
        ("Service - _TC2", Some("2026-05-24")) => 0.0,
        _ => f64::NAN,
    });

    assert_eq!(report.columns, get_columns());
    assert_eq!(
        report.rows,
        vec![
            AccountBalanceRow::new("Direct Income - _TC2", "EUR", -100.0),
            AccountBalanceRow::new("Sales - _TC2", "EUR", -100.0),
            AccountBalanceRow::new("Service - _TC2", "EUR", 0.0),
        ]
    );
}
