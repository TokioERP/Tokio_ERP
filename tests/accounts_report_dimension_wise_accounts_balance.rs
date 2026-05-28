use tokio_erp::erpnext::accounts::report::dimension_wise_accounts_balance_report::dimension_wise_accounts_balance_report::{
    accumulate_values_into_parents, execute, format_gl_entries, get_columns, get_condition,
    get_dimensions, prepare_data, DimensionMeta, DimensionRecord, DimensionWiseAccount,
    DimensionWiseFilters, DimensionWiseGlEntry, DimensionWiseQueryPlan, ReportColumn,
};

fn filters() -> DimensionWiseFilters {
    DimensionWiseFilters {
        company: "_Test Company".to_string(),
        from_date: "2026-01-01".to_string(),
        to_date: "2026-12-31".to_string(),
        dimension: "Cost Center".to_string(),
        finance_book: None,
        include_default_book_entries: false,
        default_finance_book: None,
    }
}

fn accounts() -> Vec<DimensionWiseAccount> {
    vec![
        DimensionWiseAccount::new(
            "Expense - TC",
            None,
            None,
            1,
            4,
            "Expense",
            "Profit and Loss",
            "Expenses",
            false,
        ),
        DimensionWiseAccount::new(
            "Meals - TC",
            Some("5100"),
            Some("Expense - TC"),
            2,
            3,
            "Expense",
            "Profit and Loss",
            "Meals",
            false,
        ),
        DimensionWiseAccount::new(
            "Asset - TC",
            None,
            None,
            5,
            6,
            "Asset",
            "Balance Sheet",
            "Asset",
            true,
        ),
    ]
}

#[test]
fn dimension_wise_columns_match_erpnext_dynamic_dimension_shape() {
    assert_eq!(
        get_columns(&["Main - TC".to_string(), "Admin - TC".to_string()]),
        vec![
            ReportColumn::link("Account", "account", "Account", 300),
            ReportColumn::hidden_link("Currency", "currency", "Currency"),
            ReportColumn::currency("Main - TC", "main_tc", 150),
            ReportColumn::currency("Admin - TC", "admin_tc", 150),
            ReportColumn::currency("Total", "total", 150),
        ]
    );
}

#[test]
fn dimension_wise_query_plan_matches_erpnext_sql_shape() {
    assert_eq!(
        DimensionWiseQueryPlan::for_filters(&filters(), &["Main - TC".to_string()]),
        DimensionWiseQueryPlan {
            account_doctype: "Account",
            account_fields: vec![
                "name",
                "account_number",
                "parent_account",
                "lft",
                "rgt",
                "root_type",
                "report_type",
                "account_name",
                "include_in_gross",
                "account_type",
                "is_group",
            ],
            account_order_by: "lft",
            account_filters: vec!["company = filters.company"],
            gl_doctype: "GL Entry",
            gl_fields: vec![
                "posting_date".to_string(),
                "account".to_string(),
                "cost_center".to_string(),
                "debit".to_string(),
                "credit".to_string(),
                "is_opening".to_string(),
                "fiscal_year".to_string(),
                "debit_in_account_currency".to_string(),
                "credit_in_account_currency".to_string(),
                "account_currency".to_string(),
            ],
            gl_conditions: vec![
                "company = filters.company".to_string(),
                "cost_center in %(dimensions)s".to_string(),
                "account in selected account tree".to_string(),
                "posting_date >= filters.from_date".to_string(),
                "posting_date <= filters.to_date".to_string(),
                "is_cancelled = 0".to_string(),
            ],
            gl_order_by: "account, posting_date",
            finance_book: String::new(),
            company_default_finance_book: None,
            dimensions: vec!["Main - TC".to_string()],
        }
    );

    assert_eq!(
        get_condition("Cost Center"),
        " and cost_center in %(dimensions)s"
    );
}

#[test]
fn dimension_wise_dimensions_follow_meta_company_filter() {
    let records = vec![
        DimensionRecord::new("Main - TC", Some("_Test Company")),
        DimensionRecord::new("Admin - TC", Some("_Test Company")),
        DimensionRecord::new("Other - OC", Some("Other Company")),
    ];

    assert_eq!(
        get_dimensions(&filters(), &DimensionMeta { has_company: true }, &records),
        vec!["Main - TC".to_string(), "Admin - TC".to_string()]
    );

    assert_eq!(
        get_dimensions(&filters(), &DimensionMeta { has_company: false }, &records),
        vec![
            "Main - TC".to_string(),
            "Admin - TC".to_string(),
            "Other - OC".to_string()
        ]
    );
}

#[test]
fn dimension_wise_format_accumulate_and_prepare_data_match_erpnext_steps() {
    let dimension_list = vec!["Main - TC".to_string(), "Admin - TC".to_string()];
    let mut account_rows = accounts();
    let entries = vec![
        DimensionWiseGlEntry::new(
            "Meals - TC",
            "_Test Company",
            "Main - TC",
            "2026-01-15",
            125.5554,
            25.1111,
            false,
        ),
        DimensionWiseGlEntry::new(
            "Meals - TC",
            "_Test Company",
            "Admin - TC",
            "2026-01-16",
            0.0,
            10.0,
            false,
        ),
        DimensionWiseGlEntry::new(
            "Meals - TC",
            "_Test Company",
            "Ignored - TC",
            "2026-01-17",
            999.0,
            0.0,
            false,
        ),
        DimensionWiseGlEntry::new(
            "Meals - TC",
            "_Test Company",
            "Main - TC",
            "2026-01-18",
            999.0,
            0.0,
            true,
        ),
    ];

    format_gl_entries(&mut account_rows, &entries, &dimension_list, "Cost Center");
    accumulate_values_into_parents(&mut account_rows, &dimension_list);
    let rows = prepare_data(&account_rows, &filters(), "USD", &dimension_list);

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].account, "Expense - TC");
    assert_eq!(rows[0].values["main_tc"], 100.444);
    assert_eq!(rows[0].values["admin_tc"], -10.0);
    assert_eq!(rows[0].total, 90.444);
    assert!(rows[0].has_value);
    assert_eq!(rows[1].account, "Meals - TC");
    assert_eq!(rows[1].account_name, "5100 - Meals");
    assert_eq!(rows[1].values["main_tc"], 100.444);
    assert_eq!(rows[2].account, "Asset - TC");
    assert!(!rows[2].has_value);
}

#[test]
fn dimension_wise_execute_filters_gl_entries_like_erpnext_sql() {
    let report = execute(
        &filters(),
        &DimensionMeta { has_company: true },
        &[DimensionRecord::new("Main - TC", Some("_Test Company"))],
        &accounts(),
        &[
            DimensionWiseGlEntry::new(
                "Meals - TC",
                "_Test Company",
                "Main - TC",
                "2026-01-15",
                50.0,
                0.0,
                false,
            ),
            DimensionWiseGlEntry::new(
                "Meals - TC",
                "Other Company",
                "Main - TC",
                "2026-01-15",
                999.0,
                0.0,
                false,
            ),
            DimensionWiseGlEntry::new(
                "Meals - TC",
                "_Test Company",
                "Main - TC",
                "2027-01-15",
                999.0,
                0.0,
                false,
            ),
            DimensionWiseGlEntry::new(
                "Missing - TC",
                "_Test Company",
                "Main - TC",
                "2026-01-15",
                999.0,
                0.0,
                false,
            ),
        ],
        "USD",
    );

    let rows = report.rows.expect("data rows");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].values["main_tc"], 50.0);
    assert_eq!(rows[1].values["main_tc"], 50.0);
}

#[test]
fn dimension_wise_execute_returns_empty_report_when_dimension_list_is_empty() {
    let report = execute(
        &filters(),
        &DimensionMeta { has_company: true },
        &[],
        &accounts(),
        &[],
        "USD",
    );

    assert!(report.columns.is_empty());
    assert_eq!(report.rows, Some(Vec::new()));
}
