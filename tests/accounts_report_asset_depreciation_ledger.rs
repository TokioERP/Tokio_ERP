use tokio_erp::erpnext::accounts::report::asset_depreciation_ledger::asset_depreciation_ledger::{
    execute, get_columns, get_filter_plan, AssetDepreciationFilters, AssetDepreciationInput,
    AssetDepreciationRow, AssetDetail, DepreciationColumn, DepreciationScheduleAmount,
    FinanceBookSelection, GlEntry,
};

#[test]
fn asset_depreciation_ledger_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            DepreciationColumn::new("Asset", "asset", "Link", Some("Asset"), 120),
            DepreciationColumn::new("Asset Name", "asset_name", "Data", None, 140),
            DepreciationColumn::new("Depreciation Date", "depreciation_date", "Date", None, 120),
            DepreciationColumn::new(
                "Purchase Amount",
                "net_purchase_amount",
                "Currency",
                None,
                120
            ),
            DepreciationColumn::new(
                "Opening Accumulated Depreciation",
                "opening_accumulated_depreciation",
                "Currency",
                None,
                140,
            ),
            DepreciationColumn::new(
                "Depreciation Amount",
                "depreciation_amount",
                "Currency",
                None,
                140
            ),
            DepreciationColumn::new(
                "Accumulated Depreciation Amount",
                "accumulated_depreciation_amount",
                "Currency",
                None,
                210,
            ),
            DepreciationColumn::new(
                "Value After Depreciation",
                "value_after_depreciation",
                "Currency",
                None,
                180
            ),
            DepreciationColumn::new(
                "Depreciation Entry",
                "depreciation_entry",
                "Link",
                Some("Journal Entry"),
                140,
            ),
            DepreciationColumn::new(
                "Asset Category",
                "asset_category",
                "Link",
                Some("Asset Category"),
                120,
            ),
            DepreciationColumn::new(
                "Cost Center",
                "cost_center",
                "Link",
                Some("Cost Center"),
                100
            ),
            DepreciationColumn::new("Current Status", "status", "Data", None, 120),
            DepreciationColumn::new("Purchase Date", "purchase_date", "Date", None, 120),
        ]
    );
}

#[test]
fn asset_depreciation_ledger_filter_plan_matches_erpnext_base_and_optional_filters() {
    let filters = AssetDepreciationFilters {
        company: "_Test Company".to_string(),
        from_date: "2026-01-01".to_string(),
        to_date: "2026-12-31".to_string(),
        asset: Some("AST-0001".to_string()),
        asset_category: Some("Vehicles".to_string()),
        ..Default::default()
    };

    let plan = get_filter_plan(
        &filters,
        vec![
            "Depreciation - TC".to_string(),
            "Accum Dep - TC".to_string(),
        ],
        vec!["AST-0002".to_string(), "AST-0003".to_string()],
        None,
    )
    .unwrap();

    assert_eq!(
        plan.filters,
        vec![
            "company = _Test Company".to_string(),
            "posting_date >= 2026-01-01".to_string(),
            "posting_date <= 2026-12-31".to_string(),
            "against_voucher_type = Asset".to_string(),
            "account in ['Depreciation - TC', 'Accum Dep - TC']".to_string(),
            "is_cancelled = 0".to_string(),
            "against_voucher = AST-0001".to_string(),
            "against_voucher in ['AST-0002', 'AST-0003']".to_string(),
        ]
    );
    assert_eq!(
        plan.or_filters,
        vec![
            "finance_book in ['']".to_string(),
            "finance_book is not set".to_string(),
        ]
    );
    assert_eq!(
        plan.asset_category_query.as_deref(),
        Some("asset_category = Vehicles and docstatus = 1")
    );
}

#[test]
fn asset_depreciation_ledger_finance_book_selection_matches_default_book_rules() {
    let filters = AssetDepreciationFilters {
        company: "_Test Company".to_string(),
        include_default_book_assets: true,
        finance_book: Some("IFRS".to_string()),
        ..Default::default()
    };

    assert_eq!(
        get_filter_plan(&filters, vec![], vec![], Some("Company FB".to_string())).unwrap_err(),
        "To use a different finance book, please uncheck 'Include Default FB Assets'"
    );

    let default_book_filters = AssetDepreciationFilters {
        include_default_book_assets: true,
        ..filters.clone()
    };
    let plan = get_filter_plan(
        &AssetDepreciationFilters {
            finance_book: None,
            ..default_book_filters
        },
        vec![],
        vec![],
        Some("Company FB".to_string()),
    )
    .unwrap();
    assert_eq!(
        plan.finance_book,
        FinanceBookSelection::Selected("Company FB".to_string())
    );
    assert_eq!(
        plan.or_filters,
        vec![
            "finance_book in ['', 'Company FB']".to_string(),
            "finance_book is not set".to_string(),
        ]
    );
}

#[test]
fn asset_depreciation_ledger_builds_rows_with_schedule_fallback_and_running_accumulation() {
    let input = AssetDepreciationInput {
        filters: AssetDepreciationFilters {
            company: "_Test Company".to_string(),
            from_date: "2026-01-01".to_string(),
            to_date: "2026-12-31".to_string(),
            ..Default::default()
        },
        depreciation_accounts: vec!["Depreciation - TC".to_string()],
        assets_for_category: vec![],
        company_default_finance_book: None,
        gl_entries: vec![
            GlEntry::new("AST-0001", 100.0, "JV-0001", "2026-01-31", ""),
            GlEntry::new("AST-0001", 125.0, "JV-0002", "2026-02-28", ""),
            GlEntry::new("AST-0002", 50.0, "JV-0003", "2026-01-31", ""),
            GlEntry {
                account: "Repairs - TC".to_string(),
                ..GlEntry::new("AST-0001", 999.0, "JV-SKIP", "2026-03-31", "")
            },
        ],
        assets: vec![
            AssetDetail::new("AST-0001", "Truck", 1000.0, 0.0, "Vehicles"),
            AssetDetail::new("AST-0002", "Laptop", 600.0, 200.0, "Hardware"),
        ],
        schedule_amounts: vec![DepreciationScheduleAmount::new(
            "AST-0001",
            "2026-01-31",
            300.0,
        )],
    };

    let report = execute(input).unwrap();

    assert_eq!(
        report.rows,
        vec![
            AssetDepreciationRow::new(
                "AST-0001",
                "Truck",
                "2026-01-31",
                1000.0,
                200.0,
                100.0,
                300.0,
                700.0,
                "JV-0001",
                "Vehicles",
            ),
            AssetDepreciationRow::new(
                "AST-0001",
                "Truck",
                "2026-02-28",
                1000.0,
                300.0,
                125.0,
                425.0,
                575.0,
                "JV-0002",
                "Vehicles",
            ),
            AssetDepreciationRow::new(
                "AST-0002",
                "Laptop",
                "2026-01-31",
                600.0,
                200.0,
                50.0,
                250.0,
                350.0,
                "JV-0003",
                "Hardware",
            ),
        ]
    );
}

#[test]
fn asset_depreciation_ledger_skips_gl_entries_without_asset_details() {
    let input = AssetDepreciationInput {
        filters: AssetDepreciationFilters::default(),
        depreciation_accounts: vec!["Depreciation - TC".to_string()],
        gl_entries: vec![GlEntry::new("MISSING", 100.0, "JV-0001", "2026-01-31", "")],
        ..Default::default()
    };

    assert!(execute(input).unwrap().rows.is_empty());
}
