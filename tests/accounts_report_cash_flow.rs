use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::cash_flow::cash_flow::{
    add_total_row_account, execute, get_account_type_based_data, get_cash_flow_accounts,
    get_chart_data, get_report_summary, get_start_date, show_opening_and_closing_balance,
    CashFlowAccountAmount, CashFlowChart, CashFlowChartDataset, CashFlowFilters, CashFlowPeriod,
    CashFlowReportSummary, CashFlowRow,
};

fn periods() -> Vec<CashFlowPeriod> {
    vec![
        CashFlowPeriod::new(
            "jan_2026",
            "Jan 2026",
            "2026-01-01",
            "2026-01-31",
            "2026-01-01",
        ),
        CashFlowPeriod::new(
            "feb_2026",
            "Feb 2026",
            "2026-02-01",
            "2026-02-28",
            "2026-01-01",
        ),
    ]
}

fn filters() -> CashFlowFilters {
    CashFlowFilters {
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        accumulated_values: false,
        accumulated_in_group_company: false,
        show_opening_and_closing_balance: false,
        report_template: false,
    }
}

fn account_amounts() -> Vec<CashFlowAccountAmount> {
    vec![
        CashFlowAccountAmount::new("Depreciation", "jan_2026", 12.0),
        CashFlowAccountAmount::new("Depreciation", "feb_2026", 8.0),
        CashFlowAccountAmount::new("Receivable", "jan_2026", -30.0),
        CashFlowAccountAmount::new("Receivable", "feb_2026", 20.0),
        CashFlowAccountAmount::new("Payable", "jan_2026", 10.0),
        CashFlowAccountAmount::new("Payable", "feb_2026", 25.0),
        CashFlowAccountAmount::new("Stock", "jan_2026", -40.0),
        CashFlowAccountAmount::new("Stock", "feb_2026", -10.0),
        CashFlowAccountAmount::new("Fixed Asset", "jan_2026", -200.0),
        CashFlowAccountAmount::new("Fixed Asset", "feb_2026", 50.0),
        CashFlowAccountAmount::new("Equity", "jan_2026", 100.0),
        CashFlowAccountAmount::new("Equity", "feb_2026", 0.0),
    ]
}

fn profit_loss_row() -> CashFlowRow {
    CashFlowRow::amounts(
        "'Profit for the year'",
        &[("jan_2026", 100.0), ("feb_2026", 80.0)],
    )
}

#[test]
fn cash_flow_account_sections_match_erpnext_static_registry() {
    let sections = get_cash_flow_accounts();

    assert_eq!(sections.len(), 3);
    assert_eq!(sections[0].section_name, "Operations");
    assert_eq!(sections[0].section_header, "Cash Flow from Operations");
    assert_eq!(sections[0].section_footer, "Net Cash from Operations");
    assert_eq!(
        sections[0]
            .account_types
            .iter()
            .map(|row| (row.account_type, row.label))
            .collect::<Vec<_>>(),
        vec![
            ("Depreciation", "Depreciation"),
            ("Receivable", "Net Change in Accounts Receivable"),
            ("Payable", "Net Change in Accounts Payable"),
            ("Stock", "Net Change in Inventory"),
        ]
    );
    assert_eq!(sections[1].section_footer, "Net Cash from Investing");
    assert_eq!(sections[2].section_footer, "Net Cash from Financing");
}

#[test]
fn cash_flow_start_date_matches_erpnext_accumulated_and_period_branches() {
    let period = &periods()[1];

    assert_eq!(get_start_date(period, false, "2025-04-01"), "2026-02-01");
    assert_eq!(get_start_date(period, true, "2025-04-01"), "2025-04-01");
}

#[test]
fn cash_flow_account_type_data_negates_depreciation_and_totals_periods() {
    let data = get_account_type_based_data(
        "Depreciation",
        &periods(),
        false,
        "2026-01-01",
        &account_amounts(),
    );

    assert_eq!(data.value("jan_2026"), -12.0);
    assert_eq!(data.value("feb_2026"), -8.0);
    assert_eq!(data.total, -20.0);

    let receivable = get_account_type_based_data(
        "Receivable",
        &periods(),
        false,
        "2026-01-01",
        &account_amounts(),
    );
    assert_eq!(receivable.value("jan_2026"), -30.0);
    assert_eq!(receivable.value("feb_2026"), 20.0);
    assert_eq!(receivable.total, -10.0);
}

#[test]
fn cash_flow_total_rows_sum_parent_section_rows_and_summary_like_erpnext() {
    let rows = vec![
        CashFlowRow::section("Cash Flow from Operations"),
        CashFlowRow::child(
            "Cash Flow from Operations",
            "Profit",
            &[("jan_2026", 100.0), ("feb_2026", 80.0)],
        ),
        CashFlowRow::child(
            "Cash Flow from Operations",
            "Depreciation",
            &[("jan_2026", -12.0), ("feb_2026", -8.0)],
        ),
    ];
    let mut out = rows.clone();
    let mut summary = BTreeMap::new();

    let total = add_total_row_account(
        &mut out,
        &rows,
        "Net Cash from Operations",
        &periods(),
        "USD",
        &mut summary,
        &filters(),
        false,
    );

    assert_eq!(total.section_name, "'Net Cash from Operations'");
    assert_eq!(total.value("jan_2026"), 88.0);
    assert_eq!(total.value("feb_2026"), 72.0);
    assert_eq!(total.total, 160.0);
    assert_eq!(summary["Net Cash from Operations"], 160.0);
    assert!(out.last().unwrap().is_empty);
}

#[test]
fn cash_flow_opening_and_closing_rows_match_erpnext_running_total_rules() {
    let net_change = CashFlowRow::amounts(
        "'Net Change in Cash'",
        &[("jan_2026", -32.0), ("feb_2026", 157.0)],
    );
    let mut rows = Vec::new();

    show_opening_and_closing_balance(&mut rows, &periods(), "USD", &net_change, 500.0);

    assert_eq!(rows[0].section_name, "Opening");
    assert_eq!(rows[0].value("jan_2026"), 500.0);
    assert_eq!(rows[0].value("feb_2026"), 468.0);
    assert_eq!(rows[0].total, 500.0);
    assert_eq!(rows[2].section_name, "Closing (Opening + Total)");
    assert_eq!(rows[2].value("jan_2026"), 468.0);
    assert_eq!(rows[2].value("feb_2026"), 625.0);
    assert_eq!(rows[2].total, 625.0);
    assert!(rows[3].is_empty);
}

#[test]
fn cash_flow_summary_and_chart_match_erpnext_shapes() {
    let mut summary = BTreeMap::new();
    summary.insert("Net Cash from Operations".to_string(), 160.0);
    summary.insert("Net Cash from Investing".to_string(), -150.0);

    assert_eq!(
        get_report_summary(&summary, "USD"),
        vec![
            CashFlowReportSummary::currency("Net Cash from Operations", 160.0, "USD"),
            CashFlowReportSummary::currency("Net Cash from Investing", -150.0, "USD"),
        ]
    );

    let data = vec![
        CashFlowRow::section_total(
            "'Net Cash from Operations'",
            &[("jan_2026", 88.0), ("feb_2026", 72.0)],
            "USD",
        ),
        CashFlowRow::section_total(
            "'Net Cash from Investing'",
            &[("jan_2026", -200.0), ("feb_2026", 50.0)],
            "USD",
        ),
        CashFlowRow::section_total(
            "'Net Cash from Financing'",
            &[("jan_2026", 100.0), ("feb_2026", 0.0)],
            "USD",
        ),
        CashFlowRow::section_total(
            "'Net Change in Cash'",
            &[("jan_2026", -12.0), ("feb_2026", 122.0)],
            "USD",
        ),
        CashFlowRow::section_total(
            "Opening",
            &[("jan_2026", 500.0), ("feb_2026", 488.0)],
            "USD",
        ),
    ];

    assert_eq!(
        get_chart_data(&periods(), &data, "USD"),
        CashFlowChart {
            labels: vec!["Jan 2026".to_string(), "Feb 2026".to_string()],
            datasets: vec![
                CashFlowChartDataset::new("Net Cash from Operations", vec![88.0, 72.0]),
                CashFlowChartDataset::new("Net Cash from Investing", vec![-200.0, 50.0]),
                CashFlowChartDataset::new("Net Cash from Financing", vec![100.0, 0.0]),
            ],
            chart_type: "bar".to_string(),
            fieldtype: "Currency".to_string(),
            options: "currency".to_string(),
            currency: "USD".to_string(),
        }
    );
}

#[test]
fn cash_flow_execute_builds_sections_totals_summary_chart_and_opening_rows() {
    let mut filters = filters();
    filters.show_opening_and_closing_balance = true;
    let report = execute(
        &filters,
        vec!["Account".to_string()],
        periods(),
        Some(profit_loss_row()),
        &account_amounts(),
        500.0,
        "2026-01-01",
    );

    assert_eq!(report.columns, vec!["Account"]);
    assert!(!report.delegated_to_financial_report_engine);
    assert_eq!(report.rows[0].section_name, "'Cash Flow from Operations'");
    assert_eq!(report.rows[1].section, "'Profit for the year'");
    assert_eq!(
        report.rows[1].parent_section.as_deref(),
        Some("Cash Flow from Operations")
    );
    assert_eq!(report.rows[1].indent, 1.0);
    assert_eq!(report.rows[2].section_name, "Depreciation");
    assert_eq!(report.rows[2].value("jan_2026"), -12.0);
    assert_eq!(report.rows[6].section_name, "'Net Cash from Operations'");
    assert_eq!(report.rows[6].total, 135.0);
    assert_eq!(report.rows[10].section_name, "'Net Cash from Investing'");
    assert_eq!(report.rows[10].total, -150.0);
    assert_eq!(report.rows[14].section_name, "'Net Cash from Financing'");
    assert_eq!(report.rows[14].total, 100.0);
    assert_eq!(report.rows[16].section_name, "'Net Change in Cash'");
    assert_eq!(report.rows[16].value("jan_2026"), -72.0);
    assert_eq!(report.rows[16].value("feb_2026"), 157.0);
    assert_eq!(report.rows[18].section_name, "Opening");
    assert_eq!(report.rows[20].section_name, "Closing (Opening + Total)");
    assert_eq!(report.chart.datasets.len(), 5);
    assert_eq!(report.chart.datasets[4].name, "Opening");
    assert_eq!(
        report.report_summary,
        vec![
            CashFlowReportSummary::currency("Net Cash from Operations", 135.0, "USD"),
            CashFlowReportSummary::currency("Net Cash from Investing", -150.0, "USD"),
            CashFlowReportSummary::currency("Net Cash from Financing", 100.0, "USD"),
            CashFlowReportSummary::currency("Net Change in Cash", 85.0, "USD"),
        ]
    );
}
