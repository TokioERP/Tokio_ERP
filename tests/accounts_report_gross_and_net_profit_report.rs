use tokio_erp::erpnext::accounts::report::gross_and_net_profit_report::gross_and_net_profit_report::{
    execute, get_net_profit, get_profit, get_revenue, FinancialStatementDataCall, GrossNetExecutionPlan,
    GrossNetFilters, GrossNetReport, GrossNetRow, Period,
};

fn periods() -> Vec<Period> {
    vec![Period::new("jan_2026"), Period::new("feb_2026")]
}

fn income_rows() -> Vec<GrossNetRow> {
    vec![
        GrossNetRow::account(
            "Income",
            "Income",
            "",
            true,
            1,
            0.0,
            &[("jan_2026", 300.0), ("feb_2026", 200.0)],
            500.0,
        ),
        GrossNetRow::account(
            "Sales",
            "Sales",
            "Income",
            false,
            1,
            1.0,
            &[("jan_2026", 300.0), ("feb_2026", 200.0)],
            500.0,
        ),
        GrossNetRow::account(
            "Other Income",
            "Other Income",
            "",
            true,
            0,
            0.0,
            &[("jan_2026", 50.0), ("feb_2026", 0.0)],
            50.0,
        ),
        GrossNetRow::account(
            "Interest",
            "Interest",
            "Other Income",
            false,
            0,
            1.0,
            &[("jan_2026", 50.0), ("feb_2026", 0.0)],
            50.0,
        ),
        GrossNetRow::account(
            "Empty Group",
            "Empty Group",
            "",
            true,
            1,
            0.0,
            &[("jan_2026", 0.0), ("feb_2026", 0.0)],
            0.0,
        ),
    ]
}

fn expense_rows() -> Vec<GrossNetRow> {
    vec![
        GrossNetRow::account(
            "Expense",
            "Expense",
            "",
            true,
            1,
            0.0,
            &[("jan_2026", 100.0), ("feb_2026", 80.0)],
            180.0,
        ),
        GrossNetRow::account(
            "COGS",
            "COGS",
            "Expense",
            false,
            1,
            1.0,
            &[("jan_2026", 100.0), ("feb_2026", 80.0)],
            180.0,
        ),
        GrossNetRow::account(
            "Operating Expense",
            "Operating Expense",
            "",
            true,
            0,
            0.0,
            &[("jan_2026", 40.0), ("feb_2026", 20.0)],
            60.0,
        ),
        GrossNetRow::account(
            "Rent",
            "Rent",
            "Operating Expense",
            false,
            0,
            1.0,
            &[("jan_2026", 40.0), ("feb_2026", 20.0)],
            60.0,
        ),
    ]
}

#[test]
fn gross_net_execution_plan_matches_erpnext_financial_statement_delegation() {
    assert_eq!(
        GrossNetExecutionPlan::default(),
        GrossNetExecutionPlan {
            period_source: "erpnext.accounts.report.financial_statements.get_period_list",
            columns_source: "erpnext.accounts.report.financial_statements.get_columns",
            income_data_call: FinancialStatementDataCall {
                root_type: "Income",
                balance_must_be: "Credit",
                ignore_closing_entries: true,
                ignore_accumulated_values_for_fy: true,
                total: false,
            },
            expense_data_call: FinancialStatementDataCall {
                root_type: "Expense",
                balance_must_be: "Debit",
                ignore_closing_entries: true,
                ignore_accumulated_values_for_fy: true,
                total: false,
            },
        }
    );
}

#[test]
fn gross_net_get_revenue_filters_by_include_in_gross_keeps_groups_and_prunes_empty_groups() {
    let revenue = get_revenue(&income_rows(), &periods(), 1);

    assert_eq!(
        revenue
            .iter()
            .map(|row| row.account_value())
            .collect::<Vec<_>>(),
        vec!["Income", "Sales"]
    );
    assert_eq!(revenue[0].value("jan_2026"), 300.0);
    assert_eq!(revenue[0].value("feb_2026"), 200.0);
    assert_eq!(revenue[0].total, Some(500.0));
}

#[test]
fn gross_net_get_profit_matches_erpnext_period_difference_and_total() {
    let gross_income = get_revenue(&income_rows(), &periods(), 1);
    let gross_expense = get_revenue(&expense_rows(), &periods(), 1);

    let profit = get_profit(
        &gross_income,
        &gross_expense,
        &periods(),
        "Test Company",
        "Gross Profit",
        Some("USD"),
    )
    .expect("gross profit row");

    assert_eq!(profit.account_value(), "'Gross Profit'");
    assert_eq!(profit.currency.as_deref(), Some("USD"));
    assert!(profit.warn_if_negative);
    assert_eq!(profit.value("jan_2026"), 200.0);
    assert_eq!(profit.value("feb_2026"), 120.0);
    assert_eq!(profit.total, Some(320.0));
}

#[test]
fn gross_net_get_net_profit_uses_only_root_rows_like_python_indent_filter() {
    let gross_income = get_revenue(&income_rows(), &periods(), 1);
    let non_gross_income = get_revenue(&income_rows(), &periods(), 0);
    let gross_expense = get_revenue(&expense_rows(), &periods(), 1);
    let non_gross_expense = get_revenue(&expense_rows(), &periods(), 0);

    let profit = get_net_profit(
        &non_gross_income,
        &gross_income,
        &gross_expense,
        &non_gross_expense,
        &periods(),
        "Test Company",
        Some("USD"),
    )
    .expect("net profit row");

    assert_eq!(profit.account_value(), "'Net Profit'");
    assert_eq!(profit.value("jan_2026"), 210.0);
    assert_eq!(profit.value("feb_2026"), 100.0);
    assert_eq!(profit.total, Some(310.0));
}

#[test]
fn gross_net_execute_matches_erpnext_section_order() {
    let report = execute(
        GrossNetFilters {
            company: "Test Company".to_string(),
            presentation_currency: Some("USD".to_string()),
        },
        vec!["Account".to_string(), "Jan 2026".to_string()],
        periods(),
        income_rows(),
        expense_rows(),
    );

    assert_eq!(report.columns, vec!["Account", "Jan 2026"]);
    assert_eq!(report.rows[0].account_value(), "'Included in Gross Profit'");
    assert!(report.rows[1].is_blank());
    assert_eq!(report.rows[2].account_value(), "Income");
    assert_eq!(report.rows[3].account_value(), "Sales");
    assert!(report.rows[4].is_blank());
    assert_eq!(report.rows[5].account_value(), "Expense");
    assert_eq!(report.rows[8].account_value(), "'Gross Profit'");
    assert_eq!(report.rows.last().unwrap().account_value(), "'Net Profit'");
}

#[test]
fn gross_net_execute_returns_nothing_included_when_both_gross_sections_are_empty() {
    let report = execute(
        GrossNetFilters {
            company: "Test Company".to_string(),
            presentation_currency: Some("USD".to_string()),
        },
        vec!["Account".to_string()],
        periods(),
        vec![GrossNetRow::account(
            "Other Income",
            "Other Income",
            "",
            false,
            0,
            0.0,
            &[("jan_2026", 10.0)],
            10.0,
        )],
        Vec::new(),
    );

    assert_eq!(
        report,
        GrossNetReport {
            columns: vec!["Account".to_string()],
            rows: vec![GrossNetRow::message("'Nothing is included in gross'")],
        }
    );
}

#[test]
fn gross_net_execute_appends_none_profit_rows_like_python_when_profit_has_no_value() {
    let report = execute(
        GrossNetFilters {
            company: "Test Company".to_string(),
            presentation_currency: Some("USD".to_string()),
        },
        vec!["Account".to_string()],
        periods(),
        vec![
            GrossNetRow::account(
                "Income",
                "Income",
                "",
                true,
                1,
                0.0,
                &[("jan_2026", 100.0), ("feb_2026", 0.0)],
                100.0,
            ),
            GrossNetRow::account(
                "Sales",
                "Sales",
                "Income",
                false,
                1,
                1.0,
                &[("jan_2026", 100.0), ("feb_2026", 0.0)],
                100.0,
            ),
        ],
        vec![
            GrossNetRow::account(
                "Expense",
                "Expense",
                "",
                true,
                1,
                0.0,
                &[("jan_2026", 100.0), ("feb_2026", 0.0)],
                100.0,
            ),
            GrossNetRow::account(
                "COGS",
                "COGS",
                "Expense",
                false,
                1,
                1.0,
                &[("jan_2026", 100.0), ("feb_2026", 0.0)],
                100.0,
            ),
        ],
    );

    assert!(report.rows[8].is_none);
    assert!(report.rows.last().unwrap().is_none);
}
