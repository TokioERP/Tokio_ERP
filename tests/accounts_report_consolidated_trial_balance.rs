use tokio_erp::erpnext::accounts::report::consolidated_trial_balance::consolidated_trial_balance::{
    calculate_foreign_currency_translation_reserve, consolidate_trial_balance_data, execute,
    get_columns, get_fctr_root_row_index, get_reporting_currency, prepare_companywise_tb_data,
    update_to_presentation_currency, validate_companies, CompanyNode, ConsolidatedTrialBalanceFilters,
    ConsolidatedTrialBalanceRow, CurrencyRate, ReportColumn, VALUE_FIELDS,
};

fn filters() -> ConsolidatedTrialBalanceFilters {
    ConsolidatedTrialBalanceFilters {
        company: vec!["Parent Co".to_string(), "Child Co".to_string()],
        from_date: "2026-01-01".to_string(),
        to_date: "2026-12-31".to_string(),
        presentation_currency: None,
        show_net_values: false,
        show_group_accounts: true,
        show_zero_values: false,
        report_template: false,
    }
}

fn company_tree() -> Vec<CompanyNode> {
    vec![
        CompanyNode::new("Parent Co", None, 1, 6, "USD", Some("USD")),
        CompanyNode::new("Child Co", Some("Parent Co"), 2, 3, "UZS", Some("USD")),
        CompanyNode::new("Other Root", None, 7, 8, "EUR", Some("EUR")),
    ]
}

fn row(
    account: &str,
    account_name: &str,
    root_type: &str,
    parent: Option<&str>,
) -> ConsolidatedTrialBalanceRow {
    ConsolidatedTrialBalanceRow {
        company: None,
        account: account.to_string(),
        account_name: account_name.to_string(),
        acc_name: account_name.to_string(),
        acc_number: None,
        parent_account: parent.map(str::to_string),
        indent: if parent.is_some() { 1 } else { 0 },
        from_date: "2026-01-01".to_string(),
        to_date: "2026-12-31".to_string(),
        currency: "USD".to_string(),
        is_group_account: parent.is_none(),
        root_type: root_type.to_string(),
        account_type: String::new(),
        opening_debit: 0.0,
        opening_credit: 0.0,
        debit: 0.0,
        credit: 0.0,
        closing_debit: 0.0,
        closing_credit: 0.0,
        has_value: true,
        warn_if_negative: false,
    }
}

fn company_row(
    company: &str,
    account: &str,
    account_name: &str,
    root_type: &str,
    parent: Option<&str>,
) -> ConsolidatedTrialBalanceRow {
    let mut row = row(account, account_name, root_type, parent);
    row.company = Some(company.to_string());
    row
}

#[test]
fn consolidated_trial_balance_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::data("Account", "account_name", 300, false, None),
            ReportColumn::data("Account Name", "acc_name", 250, true, None),
            ReportColumn::data("Account Number", "acc_number", 120, true, None),
            ReportColumn::link("Currency", "currency", "Currency", 0, true),
            ReportColumn::currency("Opening (Dr)", "opening_debit", 120),
            ReportColumn::currency("Opening (Cr)", "opening_credit", 120),
            ReportColumn::currency("Debit", "debit", 120),
            ReportColumn::currency("Credit", "credit", 120),
            ReportColumn::currency("Closing (Dr)", "closing_debit", 120),
            ReportColumn::currency("Closing (Cr)", "closing_credit", 120),
        ]
    );
}

#[test]
fn consolidated_trial_balance_validate_companies_requires_same_root_and_sorts_by_lft() {
    let sorted = validate_companies(
        &["Child Co".to_string(), "Parent Co".to_string()],
        &company_tree(),
    )
    .expect("same root");
    assert_eq!(
        sorted,
        vec!["Parent Co".to_string(), "Child Co".to_string()]
    );

    assert_eq!(
        validate_companies(
            &["Parent Co".to_string(), "Other Root".to_string()],
            &company_tree()
        )
        .unwrap_err(),
        "Consolidated Trial Balance can be generated for Companies having same root Company."
    );
}

#[test]
fn consolidated_trial_balance_reporting_currency_matches_default_currency_rules() {
    assert_eq!(
        get_reporting_currency(&["Parent Co".to_string()], &company_tree()),
        Some(("USD".to_string(), true))
    );
    assert_eq!(
        get_reporting_currency(
            &["Parent Co".to_string(), "Child Co".to_string()],
            &company_tree()
        ),
        Some(("USD".to_string(), false))
    );
}

#[test]
fn consolidated_trial_balance_prepare_companywise_rows_formats_and_flags_values() {
    let mut asset = row("Cash - P", "Cash", "Asset", None);
    asset.acc_number = Some("1000".to_string());
    asset.opening_debit = 10.1239;
    asset.debit = 5.5555;
    asset.closing_debit = 15.6794;
    let mut zero = row("Zero - P", "Zero", "Asset", None);
    zero.has_value = false;

    let rows =
        prepare_companywise_tb_data(&[asset, zero], "2026-01-01", "2026-12-31", "USD", 0.005);

    assert_eq!(rows[0].account_name, "1000 - Cash");
    assert_eq!(rows[0].opening_debit, 10.124);
    assert_eq!(rows[0].debit, 5.556);
    assert!(rows[0].has_value);
    assert!(!rows[1].has_value);
}

#[test]
fn consolidated_trial_balance_consolidates_existing_accounts_and_inserts_children_below_parent() {
    let mut data = vec![
        row("Assets - P", "Assets", "Asset", None),
        row("Cash - P", "Cash", "Asset", Some("Assets - P")),
    ];
    data[1].debit = 100.0;
    let mut tb_data = vec![
        row("Assets - C", "Assets", "Asset", None),
        row("Cash - C", "Cash", "Asset", Some("Assets - C")),
        row("Bank - C", "Bank", "Asset", Some("Assets - C")),
    ];
    tb_data[1].debit = 40.0;
    tb_data[2].debit = 25.0;

    consolidate_trial_balance_data(&mut data, &tb_data);

    assert_eq!(data[1].account_name, "Bank");
    assert_eq!(data[1].parent_account.as_deref(), Some("Assets - P"));
    assert_eq!(data[1].indent, 1);
    assert_eq!(data[2].account_name, "Cash");
    assert_eq!(data[2].debit, 140.0);
}

#[test]
fn consolidated_trial_balance_fctr_row_uses_equity_or_liability_parent_and_updates_totals() {
    let mut total = row("Total", "Total", "", None);
    total.opening_debit = 100.0;
    total.opening_credit = 130.0;
    total.debit = 80.0;
    total.credit = 20.0;
    let mut data = vec![
        row("Liabilities - P", "Liabilities", "Liability", None),
        row("Equity - P", "Equity", "Equity", None),
    ];

    calculate_foreign_currency_translation_reserve(&mut total, &mut data, &filters());

    assert_eq!(get_fctr_root_row_index(&data[..2]), 1);
    assert_eq!(data[2].account_name, "Foreign Currency Translation Reserve");
    assert_eq!(data[2].parent_account.as_deref(), Some("Equity - P"));
    assert_eq!(data[2].opening_debit, 30.0);
    assert_eq!(data[2].credit, 60.0);
    assert_eq!(total.opening_debit, 130.0);
    assert_eq!(total.credit, 80.0);
}

#[test]
fn consolidated_trial_balance_presentation_currency_updates_value_fields_only_when_needed() {
    let mut data = vec![row("Cash", "Cash", "Asset", None)];
    data[0].opening_debit = 10.0;
    data[0].debit = 5.0;
    let rate = CurrencyRate::new("USD", "UZS", "2026-12-31", 12_500.0);

    update_to_presentation_currency(
        &mut data,
        "USD",
        "UZS",
        "2026-12-31",
        false,
        &[rate.clone()],
    );
    assert_eq!(data[0].opening_debit, 125_000.0);
    assert_eq!(data[0].debit, 62_500.0);
    assert_eq!(data[0].currency, "UZS");

    update_to_presentation_currency(&mut data, "UZS", "EUR", "2026-12-31", true, &[rate]);
    assert_eq!(data[0].opening_debit, 125_000.0);
    assert_eq!(data[0].currency, "EUR");
}

#[test]
fn consolidated_trial_balance_execute_returns_columns_and_data_with_total_row() {
    let report = execute(
        &filters(),
        company_tree(),
        vec![
            {
                let mut row = company_row("Parent Co", "Cash - P", "Cash", "Asset", None);
                row.debit = 100.0;
                row.closing_debit = 100.0;
                row
            },
            {
                let mut row = company_row("Child Co", "Cash - C", "Cash", "Asset", None);
                row.debit = 50.0;
                row.closing_debit = 50.0;
                row
            },
            company_row("Parent Co", "Equity - P", "Equity", "Equity", None),
        ],
        Vec::new(),
    )
    .expect("report");

    assert_eq!(report.columns, get_columns());
    assert_eq!(report.data[0].account_name, "Cash");
    assert_eq!(report.data[0].debit, 150.0);
    assert_eq!(report.data.last().unwrap().account_name, "Total");
    assert!(VALUE_FIELDS.contains(&"closing_credit"));
}
