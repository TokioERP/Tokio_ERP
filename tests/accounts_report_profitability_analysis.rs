use tokio_erp::erpnext::accounts::report::profitability_analysis::profitability_analysis::{
    execute, get_accounts_data_plan, get_columns, get_gl_entries_query_plan, AccountDataPlan,
    ProfitabilityAccount, ProfitabilityColumn, ProfitabilityDimension, ProfitabilityFilters,
    ProfitabilityGlEntry, ProfitabilityInput, ProfitabilityRow,
};

#[test]
fn profitability_analysis_columns_match_erpnext_based_on_filter() {
    let filters = ProfitabilityFilters {
        based_on: "Cost Center".to_string(),
        ..Default::default()
    };

    assert_eq!(
        get_columns(&filters),
        vec![
            ProfitabilityColumn::new(
                "account",
                "Cost Center",
                "Link",
                Some("Cost Center"),
                300,
                false
            ),
            ProfitabilityColumn::new("currency", "Currency", "Link", Some("Currency"), 0, true),
            ProfitabilityColumn::new("income", "Income", "Currency", Some("currency"), 305, false),
            ProfitabilityColumn::new(
                "expense",
                "Expense",
                "Currency",
                Some("currency"),
                305,
                false
            ),
            ProfitabilityColumn::new(
                "gross_profit_loss",
                "Gross Profit / Loss",
                "Currency",
                Some("currency"),
                307,
                false,
            ),
        ]
    );
}

#[test]
fn profitability_analysis_requires_accounting_dimension_selection_like_erpnext() {
    let filters = ProfitabilityFilters {
        based_on: "Accounting Dimension".to_string(),
        accounting_dimension: None,
        ..Default::default()
    };

    assert_eq!(
        execute(ProfitabilityInput {
            filters,
            ..Default::default()
        })
        .unwrap_err(),
        "Select Accounting Dimension."
    );
}

#[test]
fn profitability_analysis_account_source_plans_match_erpnext_branches() {
    assert_eq!(
        get_accounts_data_plan("Cost Center", "_Test Company", false),
        AccountDataPlan {
            doctype: "Cost Center".to_string(),
            fields: vec![
                "name".to_string(),
                "parent_cost_center as parent_account".to_string(),
                "cost_center_name as account_name".to_string(),
                "lft".to_string(),
                "rgt".to_string(),
            ],
            filters: vec!["company = _Test Company".to_string()],
            order_by: "name".to_string(),
        }
    );

    assert_eq!(
        get_accounts_data_plan("Project", "_Test Company", false),
        AccountDataPlan {
            doctype: "Project".to_string(),
            fields: vec!["name".to_string()],
            filters: vec!["company = _Test Company".to_string()],
            order_by: "name".to_string(),
        }
    );

    assert_eq!(
        get_accounts_data_plan("Sales Person", "_Test Company", true),
        AccountDataPlan {
            doctype: "Sales Person".to_string(),
            fields: vec!["name".to_string()],
            filters: vec!["company = _Test Company".to_string()],
            order_by: "name".to_string(),
        }
    );

    assert_eq!(
        get_accounts_data_plan("Region", "_Test Company", false).filters,
        Vec::<String>::new()
    );
}

#[test]
fn profitability_analysis_gl_query_plan_matches_erpnext_date_and_closing_conditions() {
    let filters = ProfitabilityFilters {
        company: "_Test Company".to_string(),
        from_date: Some("2026-05-01".to_string()),
        to_date: Some("2026-05-31".to_string()),
        with_period_closing_entry: false,
        ..Default::default()
    };

    assert_eq!(
        get_gl_entries_query_plan(&filters, "cost_center"),
        vec![
            "company = _Test Company".to_string(),
            "cost_center is not null".to_string(),
            "is_cancelled = 0".to_string(),
            "posting_date between 2026-05-01 and 2026-05-31".to_string(),
            "voucher_type != Period Closing Voucher".to_string(),
        ]
    );

    let only_from = ProfitabilityFilters {
        to_date: None,
        with_period_closing_entry: true,
        ..filters.clone()
    };
    assert_eq!(
        get_gl_entries_query_plan(&only_from, "project"),
        vec![
            "company = _Test Company".to_string(),
            "project is not null".to_string(),
            "is_cancelled = 0".to_string(),
            "posting_date >= 2026-05-01".to_string(),
        ]
    );
}

#[test]
fn profitability_analysis_calculates_values_accumulates_parents_and_filters_zero_rows() {
    let input = ProfitabilityInput {
        filters: ProfitabilityFilters {
            company: "_Test Company".to_string(),
            based_on: "Cost Center".to_string(),
            fiscal_year: Some("2026".to_string()),
            from_date: Some("2026-05-01".to_string()),
            to_date: Some("2026-05-31".to_string()),
            company_currency: "INR".to_string(),
            ..Default::default()
        },
        accounts: vec![
            ProfitabilityAccount::new("Main - TC", None, Some("Main"), 1, 6),
            ProfitabilityAccount::new("Sales - TC", Some("Main - TC"), Some("Sales"), 2, 3),
            ProfitabilityAccount::new("Ops - TC", Some("Main - TC"), Some("Ops"), 4, 5),
            ProfitabilityAccount::new("Zero - TC", None, Some("Zero"), 7, 8),
        ],
        dimensions: vec![ProfitabilityDimension::new("Cost Center", "cost_center")],
        gl_entries: vec![
            ProfitabilityGlEntry::new("Sales - TC", "2026-05-03", "Income", 10.0, 110.0),
            ProfitabilityGlEntry::new("Sales - TC", "2026-05-04", "Expense", 30.0, 5.0),
            ProfitabilityGlEntry::new("Ops - TC", "2026-05-05", "Expense", 80.0, 20.0),
            ProfitabilityGlEntry {
                is_opening: "Yes".to_string(),
                ..ProfitabilityGlEntry::new("Sales - TC", "2026-05-06", "Income", 0.0, 999.0)
            },
            ProfitabilityGlEntry {
                voucher_type: "Period Closing Voucher".to_string(),
                ..ProfitabilityGlEntry::new("Ops - TC", "2026-05-07", "Income", 0.0, 50.0)
            },
            ProfitabilityGlEntry {
                is_cancelled: true,
                ..ProfitabilityGlEntry::new("Ops - TC", "2026-05-08", "Income", 0.0, 200.0)
            },
        ],
    };

    let report = execute(input).unwrap();

    assert_eq!(report.rows.len(), 4);
    assert_eq!(
        report.rows[0],
        ProfitabilityRow::account(
            "Main",
            "Main - TC",
            None,
            0,
            "2026",
            "INR",
            "Cost Center",
            100.0,
            85.0,
            15.0,
            true
        )
    );
    assert_eq!(
        report.rows[1],
        ProfitabilityRow::account(
            "Ops",
            "Ops - TC",
            Some("Main - TC"),
            1,
            "2026",
            "INR",
            "Cost Center",
            0.0,
            60.0,
            -60.0,
            true,
        )
    );
    assert_eq!(
        report.rows[2],
        ProfitabilityRow::account(
            "Sales",
            "Sales - TC",
            Some("Main - TC"),
            1,
            "2026",
            "INR",
            "Cost Center",
            100.0,
            25.0,
            75.0,
            true,
        )
    );
    assert_eq!(report.rows[3], ProfitabilityRow::total(100.0, 85.0, 15.0));
}

#[test]
fn profitability_analysis_show_zero_values_keeps_blank_row_like_erpnext() {
    let input = ProfitabilityInput {
        filters: ProfitabilityFilters {
            company: "_Test Company".to_string(),
            based_on: "Project".to_string(),
            show_zero_values: true,
            company_currency: "INR".to_string(),
            ..Default::default()
        },
        accounts: vec![ProfitabilityAccount::new("PRJ-0001", None, None, 1, 2)],
        dimensions: vec![ProfitabilityDimension::new("Project", "project")],
        gl_entries: vec![],
    };

    let report = execute(input).unwrap();

    assert_eq!(report.rows.len(), 3);
    assert_eq!(report.rows[0].account.as_deref(), Some("PRJ-0001"));
    assert_eq!(report.rows[0].fiscal_year, None);
    assert!(report.rows[1].account.is_none());
    assert_eq!(report.rows[2], ProfitabilityRow::total(0.0, 0.0, 0.0));
}
