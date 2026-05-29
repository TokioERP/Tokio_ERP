use tokio_erp::erpnext::accounts::report::profit_and_loss_statement::profit_and_loss_statement::{
    execute, get_chart_data, get_net_profit_loss, get_report_summary, ProfitLossChart,
    ProfitLossChartDataset, ProfitLossFilters, ProfitLossPeriod, ProfitLossReportSummary,
    ProfitLossRow,
};

fn periods() -> Vec<ProfitLossPeriod> {
    vec![
        ProfitLossPeriod::new("jan_2026", "Jan 2026"),
        ProfitLossPeriod::new("feb_2026", "Feb 2026"),
    ]
}

fn income_rows() -> Vec<ProfitLossRow> {
    vec![
        ProfitLossRow::account("Sales", &[("jan_2026", 300.0), ("feb_2026", 200.0)]),
        ProfitLossRow::account("Total Income", &[("jan_2026", 300.0), ("feb_2026", 200.0)]),
        ProfitLossRow::account("Spacer", &[]),
    ]
}

fn expense_rows() -> Vec<ProfitLossRow> {
    vec![
        ProfitLossRow::account("COGS", &[("jan_2026", 120.0), ("feb_2026", 80.0)]),
        ProfitLossRow::account("Total Expense", &[("jan_2026", 120.0), ("feb_2026", 80.0)]),
        ProfitLossRow::account("Spacer", &[]),
    ]
}

fn filters() -> ProfitLossFilters {
    ProfitLossFilters {
        company: "_Test Company".to_string(),
        presentation_currency: Some("USD".to_string()),
        accumulated_values: false,
        periodicity: "Monthly".to_string(),
        selected_view: None,
        accumulated_in_group_company: false,
        report_template: false,
    }
}

#[test]
fn profit_and_loss_get_net_profit_loss_matches_erpnext_second_last_total_rows() {
    let net_profit = get_net_profit_loss(
        &income_rows(),
        &expense_rows(),
        &periods(),
        "_Test Company",
        Some("USD"),
        false,
    )
    .expect("net profit");

    assert_eq!(net_profit.account_name, "'Profit for the year'");
    assert_eq!(net_profit.account, "'Profit for the year'");
    assert!(net_profit.warn_if_negative);
    assert_eq!(net_profit.currency, "USD");
    assert_eq!(net_profit.value("jan_2026"), 180.0);
    assert_eq!(net_profit.value("feb_2026"), 120.0);
    assert_eq!(net_profit.total, 300.0);
}

#[test]
fn profit_and_loss_report_summary_matches_accumulated_and_yearly_label_rules() {
    let monthly_net = get_net_profit_loss(
        &income_rows(),
        &expense_rows(),
        &periods(),
        "_Test Company",
        Some("USD"),
        false,
    )
    .expect("net profit");
    let (summary, primitive) = get_report_summary(
        &periods(),
        "Monthly",
        &income_rows(),
        &expense_rows(),
        Some(&monthly_net),
        "USD",
        &filters(),
        false,
    );

    assert_eq!(
        summary,
        vec![
            ProfitLossReportSummary::currency("Total Income", 500.0, "USD"),
            ProfitLossReportSummary::currency("Total Expense", 200.0, "USD"),
            ProfitLossReportSummary::profit("Net Profit", 300.0, "Green", "USD"),
        ]
    );
    assert_eq!(primitive, 300.0);

    let mut yearly_filters = filters();
    yearly_filters.accumulated_values = true;
    let one_period = vec![ProfitLossPeriod::new("fy_2026", "2026")];
    let income = vec![
        ProfitLossRow::account("Income", &[("fy_2026", 500.0)]),
        ProfitLossRow::account("Total Income", &[("fy_2026", 500.0)]),
        ProfitLossRow::account("Spacer", &[]),
    ];
    let expense = vec![
        ProfitLossRow::account("Expense", &[("fy_2026", 700.0)]),
        ProfitLossRow::account("Total Expense", &[("fy_2026", 700.0)]),
        ProfitLossRow::account("Spacer", &[]),
    ];
    let net = get_net_profit_loss(
        &income,
        &expense,
        &one_period,
        "_Test Company",
        Some("USD"),
        false,
    )
    .expect("net loss");

    let (summary, primitive) = get_report_summary(
        &one_period,
        "Yearly",
        &income,
        &expense,
        Some(&net),
        "USD",
        &yearly_filters,
        false,
    );

    assert_eq!(
        summary,
        vec![
            ProfitLossReportSummary::currency("Total Income This Year", 500.0, "USD"),
            ProfitLossReportSummary::currency("Total Expense This Year", 700.0, "USD"),
            ProfitLossReportSummary::profit("Profit This Year", -200.0, "Red", "USD"),
        ]
    );
    assert_eq!(primitive, -200.0);
}

#[test]
fn profit_and_loss_chart_data_matches_erpnext_datasets_and_chart_type() {
    let net_profit = get_net_profit_loss(
        &income_rows(),
        &expense_rows(),
        &periods(),
        "_Test Company",
        Some("USD"),
        false,
    )
    .expect("net profit");

    assert_eq!(
        get_chart_data(
            &filters(),
            &periods(),
            &income_rows(),
            &expense_rows(),
            Some(&net_profit),
            "USD",
        ),
        ProfitLossChart {
            labels: vec!["Jan 2026".to_string(), "Feb 2026".to_string()],
            datasets: vec![
                ProfitLossChartDataset::new("Income", vec![300.0, 200.0]),
                ProfitLossChartDataset::new("Expense", vec![120.0, 80.0]),
                ProfitLossChartDataset::new("Net Profit/Loss", vec![180.0, 120.0]),
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
            &income_rows(),
            &expense_rows(),
            Some(&net_profit),
            "USD",
        )
        .chart_type,
        "line"
    );
}

#[test]
fn profit_and_loss_execute_appends_net_profit_and_delegates_template_reports() {
    let report = execute(
        &filters(),
        vec!["Account".to_string(), "Jan 2026".to_string()],
        periods(),
        income_rows(),
        expense_rows(),
    );

    assert_eq!(report.columns, vec!["Account", "Jan 2026"]);
    assert_eq!(report.rows.len(), 7);
    assert_eq!(report.rows[0].account, "Sales");
    assert_eq!(report.rows[3].account, "COGS");
    assert_eq!(report.rows[6].account, "'Profit for the year'");
    assert_eq!(report.rows[6].total, 300.0);
    assert_eq!(report.chart.as_ref().unwrap().chart_type, "bar");
    assert_eq!(report.primitive_summary, 300.0);
    assert_eq!(report.report_summary[2].indicator.as_deref(), Some("Green"));

    let mut template_filters = filters();
    template_filters.report_template = true;
    assert!(
        execute(
            &template_filters,
            vec!["Account".to_string()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .delegated_to_financial_report_engine
    );
}

#[test]
fn profit_and_loss_execute_applies_growth_view_after_summary_and_chart_like_erpnext() {
    let mut growth_filters = filters();
    growth_filters.selected_view = Some("Growth".to_string());

    let report = execute(
        &growth_filters,
        vec!["Account".to_string()],
        periods(),
        income_rows(),
        expense_rows(),
    );

    assert_eq!(report.rows[0].value("jan_2026"), 300.0);
    assert_eq!(report.rows[0].value("feb_2026"), -33.33);
    assert_eq!(report.rows[6].value("feb_2026"), -33.33);
    assert_eq!(
        report
            .chart
            .as_ref()
            .unwrap()
            .datasets
            .last()
            .unwrap()
            .values,
        vec![180.0, 120.0]
    );
    assert_eq!(report.primitive_summary, 300.0);
}

#[test]
fn profit_and_loss_execute_applies_margin_view_against_income_base_row() {
    let mut margin_filters = filters();
    margin_filters.selected_view = Some("Margin".to_string());

    let income = vec![
        ProfitLossRow::account("Income", &[("jan_2026", 500.0), ("feb_2026", 400.0)]),
        ProfitLossRow::account("Total Income", &[("jan_2026", 500.0), ("feb_2026", 400.0)]),
        ProfitLossRow::account("Spacer", &[]),
    ];
    let expense = vec![
        ProfitLossRow::account("Expense", &[("jan_2026", 125.0), ("feb_2026", 80.0)]),
        ProfitLossRow::account("Total Expense", &[("jan_2026", 125.0), ("feb_2026", 80.0)]),
        ProfitLossRow::account("Spacer", &[]),
    ];

    let report = execute(
        &margin_filters,
        vec!["Account".to_string()],
        periods(),
        income,
        expense,
    );

    assert_eq!(report.rows[0].value("jan_2026"), 100.0);
    assert_eq!(report.rows[3].value("jan_2026"), 25.0);
    assert_eq!(report.rows[6].value("feb_2026"), 80.0);
    assert_eq!(report.rows[6].total, 77.22);
    assert_eq!(report.primitive_summary, 695.0);
}
