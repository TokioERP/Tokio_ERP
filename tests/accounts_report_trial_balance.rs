use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::trial_balance::trial_balance::{
    execute, get_columns, prepare_opening_closing, validate_filters, AccountRow, FiscalYear,
    GlEntry, OpeningBalance, ReportColumn, TrialBalanceFilters, TrialBalanceInput,
};

fn filters() -> TrialBalanceFilters {
    TrialBalanceFilters {
        company: "Acme".to_string(),
        fiscal_year: "FY2026".to_string(),
        from_date: None,
        to_date: None,
        year_start_date: None,
        year_end_date: None,
        presentation_currency: None,
        show_net_values: true,
        show_group_accounts: true,
        show_zero_values: false,
    }
}

fn account(
    name: &str,
    account_number: Option<&str>,
    parent_account: Option<&str>,
    account_name: &str,
    root_type: &str,
    report_type: &str,
    is_group: bool,
    lft: i32,
    rgt: i32,
) -> AccountRow {
    AccountRow {
        company: "Acme".to_string(),
        name: name.to_string(),
        account_number: account_number.map(str::to_string),
        parent_account: parent_account.map(str::to_string),
        account_name: account_name.to_string(),
        root_type: root_type.to_string(),
        report_type: report_type.to_string(),
        account_type: None,
        is_group,
        lft,
        rgt,
        opening_debit: 0.0,
        opening_credit: 0.0,
        debit: 0.0,
        credit: 0.0,
        closing_debit: 0.0,
        closing_credit: 0.0,
        indent: 0,
    }
}

fn input() -> TrialBalanceInput {
    TrialBalanceInput {
        company_currency: "USD".to_string(),
        fiscal_years: BTreeMap::from([(
            "FY2026".to_string(),
            FiscalYear {
                year_start_date: "2026-01-01".to_string(),
                year_end_date: "2026-12-31".to_string(),
            },
        )]),
        accounts: vec![
            account(
                "Assets - A",
                None,
                None,
                "Assets",
                "Asset",
                "Balance Sheet",
                true,
                1,
                6,
            ),
            account(
                "Cash - A",
                Some("1010"),
                Some("Assets - A"),
                "Cash",
                "Asset",
                "Balance Sheet",
                false,
                2,
                3,
            ),
            account(
                "Zero Child - A",
                None,
                Some("Assets - A"),
                "Zero Child",
                "Asset",
                "Balance Sheet",
                false,
                4,
                5,
            ),
            account(
                "Income - A",
                None,
                None,
                "Income",
                "Income",
                "Profit and Loss",
                true,
                7,
                10,
            ),
            account(
                "Sales - A",
                Some("4010"),
                Some("Income - A"),
                "Sales",
                "Income",
                "Profit and Loss",
                false,
                8,
                9,
            ),
        ],
        opening_balances: vec![
            OpeningBalance::new("Cash - A", 100.0, 0.0),
            OpeningBalance::new("Sales - A", 0.0, 30.0),
        ],
        gl_entries: vec![
            GlEntry::new("Acme", "Cash - A", "2026-02-01", 50.0, 20.0, "No", false),
            GlEntry::new("Acme", "Sales - A", "2026-02-05", 0.0, 80.0, "No", false),
            GlEntry::new("Acme", "Cash - A", "2026-03-01", 999.0, 0.0, "Yes", false),
            GlEntry::new("Acme", "Cash - A", "2026-03-01", 999.0, 0.0, "No", true),
        ],
    }
}

#[test]
fn trial_balance_validate_filters_defaults_and_clamps_to_fiscal_year() {
    let mut f = filters();
    f.from_date = Some("2025-12-01".to_string());
    f.to_date = Some("2027-01-01".to_string());

    validate_filters(&mut f, &input()).unwrap();

    assert_eq!(f.year_start_date.as_deref(), Some("2026-01-01"));
    assert_eq!(f.year_end_date.as_deref(), Some("2026-12-31"));
    assert_eq!(f.from_date.as_deref(), Some("2026-01-01"));
    assert_eq!(f.to_date.as_deref(), Some("2026-12-31"));

    let mut invalid = filters();
    invalid.from_date = Some("2026-05-01".to_string());
    invalid.to_date = Some("2026-04-01".to_string());
    assert_eq!(
        validate_filters(&mut invalid, &input()).unwrap_err(),
        "From Date cannot be greater than To Date"
    );
}

#[test]
fn trial_balance_columns_match_erpnext_labels_and_hidden_fields() {
    let columns = get_columns();

    assert_eq!(
        columns[0],
        ReportColumn::link("Account", "account", "Account", 300, false)
    );
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "acc_name" && column.hidden));
    assert_eq!(columns.last().unwrap().fieldname, "closing_credit");
}

#[test]
fn trial_balance_execute_accumulates_net_values_and_total_row() {
    let report = execute(filters(), input()).unwrap();

    let cash = report
        .rows
        .iter()
        .find(|row| row.account.as_deref() == Some("Cash - A"))
        .unwrap();
    assert_eq!(cash.account_name.as_deref(), Some("1010 - Cash"));
    assert_eq!(cash.opening_debit, 100.0);
    assert_eq!(cash.opening_credit, 0.0);
    assert_eq!(cash.debit, 50.0);
    assert_eq!(cash.credit, 20.0);
    assert_eq!(cash.closing_debit, 130.0);
    assert_eq!(cash.closing_credit, 0.0);

    let assets = report
        .rows
        .iter()
        .find(|row| row.account.as_deref() == Some("Assets - A"))
        .unwrap();
    assert!(assets.is_group_account);
    assert_eq!(assets.indent, 0);
    assert_eq!(assets.closing_debit, 130.0);

    let sales = report
        .rows
        .iter()
        .find(|row| row.account.as_deref() == Some("Sales - A"))
        .unwrap();
    assert_eq!(sales.opening_credit, 30.0);
    assert_eq!(sales.credit, 80.0);
    assert_eq!(sales.closing_credit, 110.0);

    assert!(!report.rows.iter().any(|row| row.is_blank));
    let total = report.rows.last().unwrap();
    assert_eq!(total.account.as_deref(), Some("'Total'"));
    assert_eq!(total.opening_debit, 100.0);
    assert_eq!(total.opening_credit, 30.0);
    assert_eq!(total.debit, 50.0);
    assert_eq!(total.credit, 100.0);
    assert_eq!(total.closing_debit, 130.0);
    assert_eq!(total.closing_credit, 110.0);
}

#[test]
fn trial_balance_hide_group_accounts_and_zero_rows_like_financial_statement_filter() {
    let mut f = filters();
    f.show_group_accounts = false;

    let report = execute(f, input()).unwrap();

    assert!(!report
        .rows
        .iter()
        .any(|row| row.account.as_deref() == Some("Assets - A")));
    assert!(report
        .rows
        .iter()
        .any(|row| row.account.as_deref() == Some("Cash - A") && row.indent == 0));
    assert!(!report
        .rows
        .iter()
        .any(|row| row.account.as_deref() == Some("Zero Child - A")));
}

#[test]
fn trial_balance_prepare_opening_closing_uses_root_type_side() {
    let mut row = AccountRow {
        company: "Acme".to_string(),
        name: "Expense - A".to_string(),
        account_number: None,
        parent_account: None,
        account_name: "Expense".to_string(),
        root_type: "Expense".to_string(),
        report_type: "Profit and Loss".to_string(),
        account_type: None,
        is_group: false,
        lft: 1,
        rgt: 2,
        opening_debit: 0.0,
        opening_credit: 0.0,
        debit: 0.0,
        credit: 0.0,
        closing_debit: 0.0,
        closing_credit: 0.0,
        indent: 0,
    }
    .with_amounts(10.0, 30.0, 0.0, 0.0);

    prepare_opening_closing(&mut row);

    assert_eq!(row.opening_debit, 0.0);
    assert_eq!(row.opening_credit, 20.0);
    assert_eq!(row.closing_debit, 0.0);
    assert_eq!(row.closing_credit, 20.0);
}
