use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::financial_statements::{
    accumulate_values_into_parents, add_total_row, calculate_values, compute_growth_view_data,
    compute_margin_view_data, filter_accounts, filter_out_zero_value_rows, get_columns, get_label,
    get_months, get_period_list, prepare_data, AccountRow, FinancialStatementGlEntry,
    FiscalYearData, Period, ReportColumn,
};

fn account(
    name: &str,
    parent_account: Option<&str>,
    account_name: &str,
    root_type: &str,
    report_type: &str,
    lft: i32,
    rgt: i32,
    is_group: bool,
) -> AccountRow {
    AccountRow {
        name: name.to_string(),
        account_number: None,
        parent_account: parent_account.map(str::to_string),
        account_name: account_name.to_string(),
        root_type: root_type.to_string(),
        report_type: report_type.to_string(),
        include_in_gross: false,
        account_type: None,
        is_group,
        lft,
        rgt,
        indent: 0,
        opening_balance: 0.0,
        values: BTreeMap::new(),
    }
}

#[test]
fn financial_statements_period_list_matches_date_range_labels_and_keys() {
    let fiscal_years = BTreeMap::from([(
        "FY2026".to_string(),
        FiscalYearData {
            year_start_date: "2026-01-01".to_string(),
            year_end_date: "2026-12-31".to_string(),
        },
    )]);

    let monthly = get_period_list(
        "FY2026",
        "FY2026",
        Some("2026-01-15"),
        Some("2026-04-20"),
        "Date Range",
        "Monthly",
        false,
        true,
        true,
        false,
        &fiscal_years,
    )
    .unwrap();

    assert_eq!(monthly.len(), 4);
    assert_eq!(monthly[0].from_date, "2026-01-15");
    assert_eq!(monthly[0].to_date, "2026-01-31");
    assert_eq!(monthly[0].key, "jan_2026");
    assert_eq!(monthly[0].label, "Jan 2026");
    assert_eq!(monthly[3].to_date, "2026-04-20");

    let yearly = get_period_list(
        "FY2026",
        "FY2026",
        None,
        None,
        "Fiscal Year",
        "Yearly",
        true,
        true,
        false,
        false,
        &fiscal_years,
    )
    .unwrap();
    assert_eq!(yearly[0].label, "2026");
    assert_eq!(get_months("2026-01-01", "2026-12-31").unwrap(), 12);
    assert_eq!(
        get_label("Quarterly", "2026-01-01", "2026-03-31").unwrap(),
        "Jan 26-Mar 26"
    );
}

#[test]
fn financial_statements_calculates_accumulated_values_and_opening_balance() {
    let periods = vec![
        Period::new("jan_2026", "Jan 2026", "2026-01-01", "2026-01-31"),
        Period::new("feb_2026", "Feb 2026", "2026-02-01", "2026-02-28"),
    ];
    let mut accounts = BTreeMap::from([(
        "Cash - A".to_string(),
        account(
            "Cash - A",
            None,
            "Cash",
            "Asset",
            "Balance Sheet",
            1,
            2,
            false,
        ),
    )]);
    let entries = BTreeMap::from([(
        "Cash - A".to_string(),
        vec![
            FinancialStatementGlEntry::new("Cash - A", "2025-12-31", 100.0, 20.0, "FY2025"),
            FinancialStatementGlEntry::new("Cash - A", "2026-01-20", 50.0, 10.0, "FY2026"),
            FinancialStatementGlEntry::new("Cash - A", "2026-02-05", 0.0, 25.0, "FY2026"),
        ],
    )]);

    calculate_values(&mut accounts, &entries, &periods, true, false).unwrap();

    let cash = accounts.get("Cash - A").unwrap();
    assert_eq!(cash.opening_balance, 80.0);
    assert_eq!(cash.values["jan_2026"], 120.0);
    assert_eq!(cash.values["feb_2026"], 95.0);

    calculate_values(&mut accounts, &entries, &periods, false, false).unwrap();
    let cash = accounts.get("Cash - A").unwrap();
    assert_eq!(cash.values["jan_2026"], 40.0);
    assert_eq!(cash.values["feb_2026"], -25.0);
}

#[test]
fn financial_statements_prepare_data_filters_zero_rows_and_adds_total() {
    let periods = vec![Period::new(
        "jan_2026",
        "Jan 2026",
        "2026-01-01",
        "2026-01-31",
    )];
    let accounts = vec![
        account(
            "Assets - A",
            None,
            "Assets",
            "Asset",
            "Balance Sheet",
            1,
            6,
            true,
        ),
        account(
            "Cash - A",
            Some("Assets - A"),
            "Cash",
            "Asset",
            "Balance Sheet",
            2,
            3,
            false,
        )
        .with_value("jan_2026", 25.0),
        account(
            "Zero - A",
            Some("Assets - A"),
            "Zero",
            "Asset",
            "Balance Sheet",
            4,
            5,
            false,
        ),
    ];
    let (filtered, mut by_name, parent_children_map) = filter_accounts(accounts);
    accumulate_values_into_parents(&filtered, &mut by_name, &periods);
    let filtered = filtered
        .into_iter()
        .map(|account| by_name[&account.name].clone())
        .collect::<Vec<_>>();
    let mut data = prepare_data(&filtered, "Debit", &periods, "USD", true);
    data = filter_out_zero_value_rows(data, &parent_children_map, false);
    add_total_row(&mut data, "Asset", "Debit", &periods, "USD");

    assert!(data
        .iter()
        .any(|row| row.account.as_deref() == Some("Assets - A")));
    assert!(data
        .iter()
        .any(|row| row.account.as_deref() == Some("Cash - A")));
    assert!(!data
        .iter()
        .any(|row| row.account.as_deref() == Some("Zero - A")));
    assert_eq!(
        data.iter()
            .find(|row| row.account.as_deref() == Some("'Total Asset (Debit)'"))
            .unwrap()
            .values["jan_2026"],
        25.0
    );
    assert!(data.last().unwrap().is_blank);
}

#[test]
fn financial_statements_columns_follow_cash_flow_and_total_rules() {
    let periods = vec![Period::new(
        "jan_2026",
        "Jan 2026",
        "2026-01-01",
        "2026-01-31",
    )];

    let columns = get_columns("Monthly", &periods, false, Some("Acme"), false);
    assert_eq!(
        columns[0],
        ReportColumn::link("Account", "account", "Account", 300, false)
    );
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "currency" && column.hidden));
    assert_eq!(columns.last().unwrap().fieldname, "total");

    let cash_flow = get_columns("Monthly", &periods, true, None, true);
    assert_eq!(cash_flow[0].fieldname, "section");
    assert!(!cash_flow
        .iter()
        .any(|column| column.fieldname == "acc_name"));
}

#[test]
fn financial_statements_growth_and_margin_views_match_erpnext_helpers() {
    let periods = vec![
        Period::new("jan_2026", "Jan 2026", "2026-01-01", "2026-01-31"),
        Period::new("feb_2026", "Feb 2026", "2026-02-01", "2026-02-28"),
    ];
    let mut rows = vec![
        account(
            "Income",
            None,
            "Income",
            "Income",
            "Profit and Loss",
            1,
            2,
            false,
        )
        .to_statement_row("USD", &periods, &[("jan_2026", 100.0), ("feb_2026", 150.0)]),
        account(
            "Expense",
            None,
            "Expense",
            "Expense",
            "Profit and Loss",
            3,
            4,
            false,
        )
        .to_statement_row("USD", &periods, &[("jan_2026", 25.0), ("feb_2026", 60.0)]),
    ];

    compute_growth_view_data(&mut rows, &periods);
    assert_eq!(rows[0].values["feb_2026"], 50.0);
    assert_eq!(rows[1].values["feb_2026"], 140.0);

    let mut margin_rows = vec![
        account(
            "Income",
            None,
            "Income",
            "Income",
            "Profit and Loss",
            1,
            2,
            false,
        )
        .to_statement_row("USD", &periods, &[("jan_2026", 100.0), ("feb_2026", 200.0)]),
        account(
            "Expense",
            None,
            "Expense",
            "Expense",
            "Profit and Loss",
            3,
            4,
            false,
        )
        .to_statement_row("USD", &periods, &[("jan_2026", 25.0), ("feb_2026", 60.0)]),
    ];

    compute_margin_view_data(&mut margin_rows, &periods, true);
    assert_eq!(margin_rows[0].values["jan_2026"], 100.0);
    assert_eq!(margin_rows[1].values["feb_2026"], 30.0);
}
