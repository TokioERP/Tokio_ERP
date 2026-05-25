use tokio_erp::erpnext::accounts::report::budget_variance_report::budget_variance_report::{
    build_budget_map, build_comparison_chart_data, build_report_data, execute,
    get_budget_dimensions_query, get_budget_records_query, get_columns, BudgetDistribution,
    BudgetRecord, BudgetVarianceFilters, FiscalYear, GlEntry, ReportColumn,
};

fn filters(period: &str) -> BudgetVarianceFilters {
    BudgetVarianceFilters {
        company: "_Test Company".to_string(),
        budget_against: "Cost Center".to_string(),
        from_fiscal_year: "2025".to_string(),
        to_fiscal_year: "2025".to_string(),
        period: period.to_string(),
        show_cumulative: false,
        budget_against_filter: None,
    }
}

fn fiscal_year() -> FiscalYear {
    FiscalYear {
        name: "2025".to_string(),
        year_start_date: "2025-01-01".to_string(),
        year_end_date: "2025-12-31".to_string(),
    }
}

#[test]
fn budget_variance_columns_match_monthly_erpnext_shape() {
    assert_eq!(
        get_columns(&filters("Monthly"), &[fiscal_year()]),
        vec![
            ReportColumn::link("Cost Center", "budget_against", "Cost Center", 150),
            ReportColumn::link("Account", "account", "Account", 150),
            ReportColumn::float("Budget (Jan) 2025", "budget_jan_2025", 150),
            ReportColumn::float("Actual (Jan) 2025", "actual_jan_2025", 150),
            ReportColumn::float("Variance (Jan) 2025", "variance_jan_2025", 150),
            ReportColumn::float("Budget (Feb) 2025", "budget_feb_2025", 150),
            ReportColumn::float("Actual (Feb) 2025", "actual_feb_2025", 150),
            ReportColumn::float("Variance (Feb) 2025", "variance_feb_2025", 150),
            ReportColumn::float("Budget (Mar) 2025", "budget_mar_2025", 150),
            ReportColumn::float("Actual (Mar) 2025", "actual_mar_2025", 150),
            ReportColumn::float("Variance (Mar) 2025", "variance_mar_2025", 150),
            ReportColumn::float("Budget (Apr) 2025", "budget_apr_2025", 150),
            ReportColumn::float("Actual (Apr) 2025", "actual_apr_2025", 150),
            ReportColumn::float("Variance (Apr) 2025", "variance_apr_2025", 150),
            ReportColumn::float("Budget (May) 2025", "budget_may_2025", 150),
            ReportColumn::float("Actual (May) 2025", "actual_may_2025", 150),
            ReportColumn::float("Variance (May) 2025", "variance_may_2025", 150),
            ReportColumn::float("Budget (Jun) 2025", "budget_jun_2025", 150),
            ReportColumn::float("Actual (Jun) 2025", "actual_jun_2025", 150),
            ReportColumn::float("Variance (Jun) 2025", "variance_jun_2025", 150),
            ReportColumn::float("Budget (Jul) 2025", "budget_jul_2025", 150),
            ReportColumn::float("Actual (Jul) 2025", "actual_jul_2025", 150),
            ReportColumn::float("Variance (Jul) 2025", "variance_jul_2025", 150),
            ReportColumn::float("Budget (Aug) 2025", "budget_aug_2025", 150),
            ReportColumn::float("Actual (Aug) 2025", "actual_aug_2025", 150),
            ReportColumn::float("Variance (Aug) 2025", "variance_aug_2025", 150),
            ReportColumn::float("Budget (Sep) 2025", "budget_sep_2025", 150),
            ReportColumn::float("Actual (Sep) 2025", "actual_sep_2025", 150),
            ReportColumn::float("Variance (Sep) 2025", "variance_sep_2025", 150),
            ReportColumn::float("Budget (Oct) 2025", "budget_oct_2025", 150),
            ReportColumn::float("Actual (Oct) 2025", "actual_oct_2025", 150),
            ReportColumn::float("Variance (Oct) 2025", "variance_oct_2025", 150),
            ReportColumn::float("Budget (Nov) 2025", "budget_nov_2025", 150),
            ReportColumn::float("Actual (Nov) 2025", "actual_nov_2025", 150),
            ReportColumn::float("Variance (Nov) 2025", "variance_nov_2025", 150),
            ReportColumn::float("Budget (Dec) 2025", "budget_dec_2025", 150),
            ReportColumn::float("Actual (Dec) 2025", "actual_dec_2025", 150),
            ReportColumn::float("Variance (Dec) 2025", "variance_dec_2025", 150),
            ReportColumn::float("Total Budget", "total_budget", 150),
            ReportColumn::float("Total Actual", "total_actual", 150),
            ReportColumn::float("Total Variance", "total_variance", 150),
        ]
    );
}

#[test]
fn budget_variance_aggregates_budget_actual_variance_and_chart_data() {
    let budget_records = vec![BudgetRecord {
        name: "BUD-0001".to_string(),
        account: "Expense - TC".to_string(),
        dimension: "Main - TC".to_string(),
        budget_amount: 1200.0,
        from_fiscal_year: "2025".to_string(),
        to_fiscal_year: "2025".to_string(),
        budget_start_date: "2025-01-01".to_string(),
        budget_end_date: "2025-12-31".to_string(),
        distributions: vec![BudgetDistribution {
            start_date: "2025-01-01".to_string(),
            end_date: "2025-03-01".to_string(),
            amount: 300.0,
            percent: 25.0,
        }],
    }];
    let actuals = vec![
        GlEntry::new("Expense - TC", "Main - TC", "2025", "2025-01-15", 80.0, 0.0),
        GlEntry::new("Expense - TC", "Main - TC", "2025", "2025-02-15", 0.0, 20.0),
        GlEntry::new(
            "Expense - TC",
            "Other - TC",
            "2025",
            "2025-01-15",
            999.0,
            0.0,
        ),
    ];

    let budget_map = build_budget_map(&budget_records, &actuals, &[fiscal_year()]);
    let columns = get_columns(&filters("Monthly"), &[fiscal_year()]);
    let rows = build_report_data(&budget_map, &filters("Monthly"), &[fiscal_year()]);
    let chart = build_comparison_chart_data(&columns, &rows).expect("chart data");

    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.budget_against, "Main - TC");
    assert_eq!(row.account, "Expense - TC");
    assert_eq!(row.values["budget_jan_2025"], 100.0);
    assert_eq!(row.values["actual_jan_2025"], 80.0);
    assert_eq!(row.values["variance_jan_2025"], 20.0);
    assert_eq!(row.values["budget_feb_2025"], 100.0);
    assert_eq!(row.values["actual_feb_2025"], -20.0);
    assert_eq!(row.values["variance_feb_2025"], 120.0);
    assert_eq!(row.values["budget_mar_2025"], 100.0);
    assert_eq!(row.values["total_budget"], 300.0);
    assert_eq!(row.values["total_actual"], 60.0);
    assert_eq!(row.values["total_variance"], 240.0);

    assert_eq!(chart.labels[0], "Cost Center");
    assert_eq!(chart.budget_values[0], 0.0);
    assert_eq!(chart.labels[1], "(Jan) 2025");
    assert_eq!(chart.budget_values[1], 100.0);
    assert_eq!(chart.actual_values[0], 80.0);
    assert_eq!(chart.budget_values[2], 100.0);
    assert_eq!(chart.actual_values[1], -20.0);
}

#[test]
fn budget_variance_cumulative_is_disabled_for_yearly() {
    let mut quarterly = filters("Quarterly");
    quarterly.show_cumulative = true;

    let budget_records = vec![BudgetRecord {
        name: "BUD-0001".to_string(),
        account: "Expense - TC".to_string(),
        dimension: "Main - TC".to_string(),
        budget_amount: 1200.0,
        from_fiscal_year: "2025".to_string(),
        to_fiscal_year: "2025".to_string(),
        budget_start_date: "2025-01-01".to_string(),
        budget_end_date: "2025-12-31".to_string(),
        distributions: vec![BudgetDistribution {
            start_date: "2025-01-01".to_string(),
            end_date: "2025-06-01".to_string(),
            amount: 600.0,
            percent: 50.0,
        }],
    }];

    let budget_map = build_budget_map(&budget_records, &[], &[fiscal_year()]);
    let rows = build_report_data(&budget_map, &quarterly, &[fiscal_year()]);
    assert_eq!(rows[0].values["budget_jan_mar_2025"], 300.0);
    assert_eq!(rows[0].values["budget_apr_jun_2025"], 600.0);

    let mut yearly = filters("Yearly");
    yearly.show_cumulative = true;
    let rows = build_report_data(&budget_map, &yearly, &[fiscal_year()]);
    assert_eq!(rows[0].values["budget_2025"], 600.0);
    assert!(!rows[0].values.contains_key("total_budget"));
}

#[test]
fn budget_variance_uses_fiscal_year_lookup_not_calendar_year() {
    let mut apr_to_mar = filters("Monthly");
    apr_to_mar.from_fiscal_year = "FY2024-25".to_string();
    apr_to_mar.to_fiscal_year = "FY2024-25".to_string();

    let fiscal_year = FiscalYear {
        name: "FY2024-25".to_string(),
        year_start_date: "2024-04-01".to_string(),
        year_end_date: "2025-03-31".to_string(),
    };
    let budget_records = vec![BudgetRecord {
        name: "BUD-0001".to_string(),
        account: "Expense - TC".to_string(),
        dimension: "Main - TC".to_string(),
        budget_amount: 300.0,
        from_fiscal_year: "FY2024-25".to_string(),
        to_fiscal_year: "FY2024-25".to_string(),
        budget_start_date: "2025-01-01".to_string(),
        budget_end_date: "2025-03-01".to_string(),
        distributions: vec![BudgetDistribution {
            start_date: "2025-01-01".to_string(),
            end_date: "2025-03-01".to_string(),
            amount: 300.0,
            percent: 100.0,
        }],
    }];
    let actuals = vec![GlEntry::new(
        "Expense - TC",
        "Main - TC",
        "FY2024-25",
        "2025-01-15",
        80.0,
        0.0,
    )];

    let budget_map = build_budget_map(&budget_records, &actuals, &[fiscal_year.clone()]);
    let rows = build_report_data(&budget_map, &apr_to_mar, &[fiscal_year]);

    assert_eq!(rows[0].values["budget_jan_fy2024_25"], 100.0);
    assert_eq!(rows[0].values["actual_jan_fy2024_25"], 80.0);
}

#[test]
fn budget_variance_preserves_budget_record_insertion_order_like_python_dicts() {
    let budget_records = vec![
        BudgetRecord {
            name: "BUD-0001".to_string(),
            account: "Expense - TC".to_string(),
            dimension: "Zulu - TC".to_string(),
            budget_amount: 100.0,
            from_fiscal_year: "2025".to_string(),
            to_fiscal_year: "2025".to_string(),
            budget_start_date: "2025-01-01".to_string(),
            budget_end_date: "2025-01-01".to_string(),
            distributions: vec![BudgetDistribution {
                start_date: "2025-01-01".to_string(),
                end_date: "2025-01-01".to_string(),
                amount: 100.0,
                percent: 100.0,
            }],
        },
        BudgetRecord {
            name: "BUD-0002".to_string(),
            account: "Expense - TC".to_string(),
            dimension: "Alpha - TC".to_string(),
            budget_amount: 100.0,
            from_fiscal_year: "2025".to_string(),
            to_fiscal_year: "2025".to_string(),
            budget_start_date: "2025-01-01".to_string(),
            budget_end_date: "2025-01-01".to_string(),
            distributions: vec![BudgetDistribution {
                start_date: "2025-01-01".to_string(),
                end_date: "2025-01-01".to_string(),
                amount: 100.0,
                percent: 100.0,
            }],
        },
    ];

    let budget_map = build_budget_map(&budget_records, &[], &[fiscal_year()]);
    let rows = build_report_data(&budget_map, &filters("Monthly"), &[fiscal_year()]);

    assert_eq!(rows[0].budget_against, "Zulu - TC");
    assert_eq!(rows[1].budget_against, "Alpha - TC");
}

#[test]
fn budget_variance_execute_uses_explicit_dimension_filter_and_empty_dimension_branch() {
    let mut report_filters = filters("Monthly");
    report_filters.budget_against_filter = Some(vec!["Main - TC".to_string()]);
    let budget_records = vec![
        BudgetRecord {
            name: "BUD-0001".to_string(),
            account: "Expense - TC".to_string(),
            dimension: "Main - TC".to_string(),
            budget_amount: 100.0,
            from_fiscal_year: "2025".to_string(),
            to_fiscal_year: "2025".to_string(),
            budget_start_date: "2025-01-01".to_string(),
            budget_end_date: "2025-01-01".to_string(),
            distributions: vec![BudgetDistribution {
                start_date: "2025-01-01".to_string(),
                end_date: "2025-01-01".to_string(),
                amount: 100.0,
                percent: 100.0,
            }],
        },
        BudgetRecord {
            name: "BUD-0002".to_string(),
            account: "Expense - TC".to_string(),
            dimension: "Other - TC".to_string(),
            budget_amount: 900.0,
            from_fiscal_year: "2025".to_string(),
            to_fiscal_year: "2025".to_string(),
            budget_start_date: "2025-01-01".to_string(),
            budget_end_date: "2025-01-01".to_string(),
            distributions: vec![BudgetDistribution {
                start_date: "2025-01-01".to_string(),
                end_date: "2025-01-01".to_string(),
                amount: 900.0,
                percent: 100.0,
            }],
        },
    ];

    let report = execute(
        &report_filters,
        &[fiscal_year()],
        &["Main - TC".to_string(), "Other - TC".to_string()],
        &budget_records,
        &[],
    );
    assert_eq!(report.rows.len(), 1);
    assert_eq!(report.rows[0].budget_against, "Main - TC");
    assert!(report.chart_data.is_some());

    let empty = execute(
        &filters("Monthly"),
        &[fiscal_year()],
        &[],
        &budget_records,
        &[],
    );
    assert!(empty.rows.is_empty());
    assert!(empty.chart_data.is_none());
}

#[test]
fn budget_variance_actuals_remain_exact_dimension_matches_like_erpnext_sql_equality() {
    let budget_records = vec![BudgetRecord {
        name: "BUD-0001".to_string(),
        account: "Expense - TC".to_string(),
        dimension: "Parent - TC".to_string(),
        budget_amount: 100.0,
        from_fiscal_year: "2025".to_string(),
        to_fiscal_year: "2025".to_string(),
        budget_start_date: "2025-01-01".to_string(),
        budget_end_date: "2025-01-01".to_string(),
        distributions: vec![BudgetDistribution {
            start_date: "2025-01-01".to_string(),
            end_date: "2025-01-01".to_string(),
            amount: 100.0,
            percent: 100.0,
        }],
    }];
    let actuals = vec![
        GlEntry::new(
            "Expense - TC",
            "Parent - TC",
            "2025",
            "2025-01-10",
            40.0,
            0.0,
        ),
        GlEntry::new(
            "Expense - TC",
            "Child - TC",
            "2025",
            "2025-01-10",
            80.0,
            0.0,
        ),
    ];

    let budget_map = build_budget_map(&budget_records, &actuals, &[fiscal_year()]);
    let rows = build_report_data(&budget_map, &filters("Monthly"), &[fiscal_year()]);

    assert_eq!(rows[0].values["actual_jan_2025"], 40.0);
}

#[test]
fn budget_variance_query_plans_preserve_erpnext_filter_shape() {
    let record_query = get_budget_records_query(&filters("Monthly"), &["Main - TC", "West - TC"]);
    assert_eq!(record_query.doctype, "Budget");
    assert_eq!(record_query.budget_against_field, "cost_center");
    assert_eq!(
        record_query.filters,
        vec![
            "b.company = _Test Company",
            "b.docstatus = 1",
            "b.budget_against = Cost Center",
            "b.cost_center in [Main - TC, West - TC]",
            "b.from_fiscal_year <= 2025",
            "b.to_fiscal_year >= 2025",
        ]
    );

    let dimension_query = get_budget_dimensions_query(&filters("Monthly"));
    assert_eq!(dimension_query.doctype, "Cost Center");
    assert_eq!(dimension_query.filters, vec!["company = _Test Company"]);
    assert_eq!(dimension_query.order_by.as_deref(), Some("lft"));
}
