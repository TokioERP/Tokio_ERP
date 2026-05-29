use tokio_erp::erpnext::accounts::report::balance_sheet::balance_sheet::{
    check_opening_balance, execute, get_chart_data, get_provisional_profit_loss,
    get_report_summary, BalanceSheetChart, BalanceSheetChartDataset, BalanceSheetFilters,
    BalanceSheetPeriod, BalanceSheetReportSummary, BalanceSheetRow,
};

fn periods() -> Vec<BalanceSheetPeriod> {
    vec![
        BalanceSheetPeriod::new("jan_2026", "Jan 2026"),
        BalanceSheetPeriod::new("feb_2026", "Feb 2026"),
    ]
}

fn asset_rows() -> Vec<BalanceSheetRow> {
    vec![
        BalanceSheetRow::account("Bank", &[("jan_2026", 300.0), ("feb_2026", 500.0)]),
        BalanceSheetRow::account("Total Asset", &[("jan_2026", 300.0), ("feb_2026", 500.0)]),
        BalanceSheetRow::empty(),
    ]
}

fn liability_rows() -> Vec<BalanceSheetRow> {
    vec![
        BalanceSheetRow::account("Payable", &[("jan_2026", 120.0), ("feb_2026", 200.0)]),
        BalanceSheetRow::account(
            "Total Liability",
            &[("jan_2026", 120.0), ("feb_2026", 200.0)],
        ),
        BalanceSheetRow::empty(),
    ]
}

fn equity_rows() -> Vec<BalanceSheetRow> {
    vec![
        BalanceSheetRow::account("Capital", &[("jan_2026", 80.0), ("feb_2026", 150.0)]),
        BalanceSheetRow::account("Total Equity", &[("jan_2026", 80.0), ("feb_2026", 150.0)]),
        BalanceSheetRow::empty(),
    ]
}

fn filters() -> BalanceSheetFilters {
    BalanceSheetFilters {
        company: "_Test Company".to_string(),
        presentation_currency: Some("USD".to_string()),
        accumulated_values: false,
        selected_view: None,
        accumulated_in_group_company: false,
        report_template: false,
    }
}

#[test]
fn balance_sheet_get_provisional_profit_loss_matches_erpnext_second_last_total_rows() {
    let (provisional, total_credit) = get_provisional_profit_loss(
        &asset_rows(),
        &liability_rows(),
        &equity_rows(),
        &periods(),
        "_Test Company",
        Some("USD"),
        false,
    );
    let provisional = provisional.expect("provisional profit/loss");

    assert_eq!(
        provisional.account_name,
        "'Provisional Profit / Loss (Credit)'"
    );
    assert_eq!(provisional.account, "'Provisional Profit / Loss (Credit)'");
    assert!(provisional.warn_if_negative);
    assert_eq!(provisional.currency, "USD");
    assert_eq!(provisional.value("jan_2026"), 100.0);
    assert_eq!(provisional.value("feb_2026"), 150.0);
    assert_eq!(provisional.total, 250.0);

    assert_eq!(total_credit.account_name, "'Total (Credit)'");
    assert_eq!(total_credit.value("jan_2026"), 300.0);
    assert_eq!(total_credit.value("feb_2026"), 500.0);
    assert_eq!(total_credit.total, 800.0);
}

#[test]
fn balance_sheet_check_opening_balance_uses_last_rows_like_erpnext() {
    let mut asset = asset_rows();
    let mut liability = liability_rows();
    let mut equity = equity_rows();
    *asset.last_mut().unwrap() = BalanceSheetRow::opening_balance(500.0);
    *liability.last_mut().unwrap() = BalanceSheetRow::opening_balance(125.0);
    *equity.last_mut().unwrap() = BalanceSheetRow::opening_balance(25.0);

    assert_eq!(
        check_opening_balance(&asset, &liability, &equity, 2),
        Some(("Previous Financial Year is not closed".to_string(), 350.0))
    );

    assert_eq!(
        check_opening_balance(&asset_rows(), &liability_rows(), &equity_rows(), 2),
        None
    );
}

#[test]
fn balance_sheet_report_summary_matches_erpnext_accumulated_rules() {
    let provisional = get_provisional_profit_loss(
        &asset_rows(),
        &liability_rows(),
        &equity_rows(),
        &periods(),
        "_Test Company",
        Some("USD"),
        false,
    )
    .0;

    let (summary, primitive) = get_report_summary(
        &periods(),
        &asset_rows(),
        &liability_rows(),
        &equity_rows(),
        provisional.as_ref(),
        "USD",
        &filters(),
        false,
    );

    assert_eq!(
        summary,
        vec![
            BalanceSheetReportSummary::currency("Total Asset", 800.0, "USD"),
            BalanceSheetReportSummary::currency("Total Liability", 320.0, "USD"),
            BalanceSheetReportSummary::currency("Total Equity", 230.0, "USD"),
            BalanceSheetReportSummary::provisional(
                "Provisional Profit / Loss (Credit)",
                250.0,
                "Green",
                "USD",
            ),
        ]
    );
    assert_eq!(primitive, 710.0);

    let mut accumulated = filters();
    accumulated.accumulated_values = true;
    let (summary, primitive) = get_report_summary(
        &periods(),
        &asset_rows(),
        &liability_rows(),
        &equity_rows(),
        provisional.as_ref(),
        "USD",
        &accumulated,
        false,
    );

    assert_eq!(summary[0].value, 500.0);
    assert_eq!(summary[1].value, 200.0);
    assert_eq!(summary[2].value, 150.0);
    assert_eq!(summary[3].value, 150.0);
    assert_eq!(primitive, 450.0);
}

#[test]
fn balance_sheet_report_summary_filters_group_company_period_like_consolidated_erpnext() {
    let periods = vec![
        BalanceSheetPeriod::new("Parent Co", "Parent Co"),
        BalanceSheetPeriod::new("Child Co", "Child Co"),
    ];
    let asset = vec![
        BalanceSheetRow::account("Bank", &[("Parent Co", 1000.0), ("Child Co", 250.0)]),
        BalanceSheetRow::account("Total Asset", &[("Parent Co", 1000.0), ("Child Co", 250.0)]),
        BalanceSheetRow::empty(),
    ];
    let liability = vec![
        BalanceSheetRow::account("Payable", &[("Parent Co", 400.0), ("Child Co", 75.0)]),
        BalanceSheetRow::account(
            "Total Liability",
            &[("Parent Co", 400.0), ("Child Co", 75.0)],
        ),
        BalanceSheetRow::empty(),
    ];
    let equity = vec![
        BalanceSheetRow::account("Equity", &[("Parent Co", 100.0), ("Child Co", 25.0)]),
        BalanceSheetRow::account("Total Equity", &[("Parent Co", 100.0), ("Child Co", 25.0)]),
        BalanceSheetRow::empty(),
    ];
    let provisional = get_provisional_profit_loss(
        &asset,
        &liability,
        &equity,
        &periods,
        "Parent Co",
        Some("USD"),
        true,
    )
    .0;
    let mut filters = filters();
    filters.company = "Parent Co".to_string();
    filters.accumulated_in_group_company = true;

    let (summary, primitive) = get_report_summary(
        &periods,
        &asset,
        &liability,
        &equity,
        provisional.as_ref(),
        "USD",
        &filters,
        true,
    );

    assert_eq!(summary[0].value, 1000.0);
    assert_eq!(summary[1].value, 400.0);
    assert_eq!(summary[2].value, 100.0);
    assert_eq!(summary[3].value, 500.0);
    assert_eq!(primitive, 700.0);
}

#[test]
fn balance_sheet_chart_data_matches_erpnext_dataset_and_chart_type_rules() {
    assert_eq!(
        get_chart_data(
            &filters(),
            &periods(),
            &asset_rows(),
            &liability_rows(),
            &equity_rows(),
            "USD",
        ),
        BalanceSheetChart {
            labels: vec!["Jan 2026".to_string(), "Feb 2026".to_string()],
            datasets: vec![
                BalanceSheetChartDataset::new("Assets", vec![300.0, 500.0]),
                BalanceSheetChartDataset::new("Liabilities", vec![120.0, 200.0]),
                BalanceSheetChartDataset::new("Equity", vec![80.0, 150.0]),
            ],
            chart_type: "bar".to_string(),
            fieldtype: "Currency".to_string(),
            options: "currency".to_string(),
            currency: "USD".to_string(),
        }
    );

    let mut accumulated = filters();
    accumulated.accumulated_values = true;
    assert_eq!(
        get_chart_data(
            &accumulated,
            &periods(),
            &asset_rows(),
            &liability_rows(),
            &equity_rows(),
            "USD",
        )
        .chart_type,
        "line"
    );
}

#[test]
fn balance_sheet_execute_appends_unclosed_and_provisional_rows_like_erpnext() {
    let mut asset = asset_rows();
    *asset.last_mut().unwrap() = BalanceSheetRow::opening_balance(25.0);

    let report = execute(
        &filters(),
        vec!["Account".to_string(), "Jan 2026".to_string()],
        periods(),
        asset,
        liability_rows(),
        equity_rows(),
        2,
    );

    assert_eq!(report.columns, vec!["Account", "Jan 2026"]);
    assert_eq!(
        report.message,
        Some("Previous Financial Year is not closed".to_string())
    );
    assert_eq!(
        report.rows[9].account,
        "'Unclosed Fiscal Years Profit / Loss (Credit)'"
    );
    assert_eq!(report.rows[9].value("jan_2026"), 25.0);
    assert_eq!(
        report.rows[10].account,
        "'Provisional Profit / Loss (Credit)'"
    );
    assert_eq!(report.rows[10].value("jan_2026"), 75.0);
    assert_eq!(report.rows[11].account, "'Total (Credit)'");
    assert_eq!(report.chart.as_ref().unwrap().chart_type, "bar");
    assert_eq!(report.report_summary[3].indicator.as_deref(), Some("Green"));

    let mut template_filters = filters();
    template_filters.report_template = true;
    assert!(
        execute(
            &template_filters,
            vec!["Account".to_string()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            2,
        )
        .delegated_to_financial_report_engine
    );
}

#[test]
fn balance_sheet_erpnext_test_fixture_keeps_income_out_and_reports_balance_sheet_totals() {
    let asset = vec![
        BalanceSheetRow::account("My Bank", &[("fy_2026", 1100.0)]),
        BalanceSheetRow::account("Application of Funds (Assets)", &[("fy_2026", 1100.0)]),
        BalanceSheetRow::empty(),
    ];
    let liability = vec![
        BalanceSheetRow::account("VAT Liabilities", &[("fy_2026", 10.0)]),
        BalanceSheetRow::account("Advance VAT Paid", &[("fy_2026", -10.0)]),
        BalanceSheetRow::account("Duties and Taxes", &[("fy_2026", 0.0)]),
        BalanceSheetRow::empty(),
    ];
    let equity = vec![
        BalanceSheetRow::account("Capital Stock", &[("fy_2026", 1000.0)]),
        BalanceSheetRow::account("Equity", &[("fy_2026", 1000.0)]),
        BalanceSheetRow::empty(),
    ];

    let report = execute(
        &filters(),
        vec!["Account".to_string()],
        vec![BalanceSheetPeriod::new("fy_2026", "2026")],
        asset,
        liability,
        equity,
        2,
    );
    let name_and_total = report
        .rows
        .iter()
        .filter(|row| !row.account_name.is_empty())
        .map(|row| (row.account_name.as_str(), row.total))
        .collect::<std::collections::BTreeMap<_, _>>();

    assert!(!name_and_total.contains_key("Sales"));
    assert_eq!(name_and_total["My Bank"], 1100.0);
    assert_eq!(name_and_total["VAT Liabilities"], 10.0);
    assert_eq!(name_and_total["Advance VAT Paid"], -10.0);
    assert_eq!(name_and_total["Duties and Taxes"], 0.0);
    assert_eq!(name_and_total["Application of Funds (Assets)"], 1100.0);
    assert_eq!(name_and_total["Equity"], 1000.0);
    assert_eq!(
        name_and_total["'Provisional Profit / Loss (Credit)'"],
        100.0
    );
}
