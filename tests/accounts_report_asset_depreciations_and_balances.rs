use tokio_erp::erpnext::accounts::report::asset_depreciations_and_balances::asset_depreciations_and_balances::{
    assemble_group_by_asset_category_data, assemble_group_by_asset_data,
    combine_asset_depreciation_rows, combine_category_depreciation_rows, execute, get_columns,
    get_asset_depreciation_query_plans, get_asset_details_query_plan,
    get_asset_value_adjustment_query_plan, get_category_depreciation_query_plans,
    get_category_values_query_plan, get_data, AssetDepreciationByAssetRow,
    AssetDepreciationByCategoryRow, AssetDepreciationsAndBalancesData,
    AssetOpeningDepreciationByAssetRow, AssetOpeningDepreciationByCategoryRow,
    AssetDepreciationsAndBalancesFilters, AssetDepreciationsAndBalancesRow, AssetDetailValueRow,
    AssetValueAdjustmentRow, AssetValueByCategoryRow, ReportColumn,
};

fn filters(group_by: &str) -> AssetDepreciationsAndBalancesFilters {
    AssetDepreciationsAndBalancesFilters {
        company: "_Test Company".to_string(),
        from_date: "2026-05-01".to_string(),
        to_date: "2026-05-31".to_string(),
        group_by: group_by.to_string(),
        asset_category: None,
        asset: None,
        finance_book: None,
    }
}

fn filtered(group_by: &str) -> AssetDepreciationsAndBalancesFilters {
    AssetDepreciationsAndBalancesFilters {
        asset_category: Some("Computers".to_string()),
        asset: Some("AST-0001".to_string()),
        finance_book: Some("IFRS".to_string()),
        ..filters(group_by)
    }
}

#[test]
fn asset_depreciations_columns_match_erpnext_asset_category_shape() {
    let columns = get_columns(&filters("Asset Category"));

    assert_eq!(
        columns[0],
        ReportColumn::link("Asset Category", "asset_category", "Asset Category", 120)
    );
    assert_eq!(columns.len(), 14);
    assert_eq!(
        columns[1],
        ReportColumn::currency("Value as on 2026-04-30", "value_as_on_from_date", 140)
    );
    assert_eq!(
        columns[7],
        ReportColumn::currency(
            "Accumulated Depreciation as on 2026-04-30",
            "accumulated_depreciation_as_on_from_date",
            270,
        )
    );
    assert_eq!(
        columns[10],
        ReportColumn::currency(
            "Accumulated Depreciation as on 2026-05-31",
            "accumulated_depreciation_as_on_to_date",
            270,
        )
    );
    assert_eq!(
        columns[13],
        ReportColumn::currency(
            "Net Asset value as on 2026-05-31",
            "net_asset_value_as_on_to_date",
            200
        )
    );
}

#[test]
fn asset_depreciations_columns_match_erpnext_asset_shape() {
    let columns = get_columns(&filters("Asset"));

    assert_eq!(
        columns[0],
        ReportColumn::link("Asset", "asset", "Asset", 120)
    );
    assert_eq!(
        columns[1],
        ReportColumn::data("Asset Name", "asset_name", 140)
    );
    assert_eq!(columns.len(), 15);
}

#[test]
fn asset_depreciations_category_rows_apply_erpnext_value_and_depreciation_formulas() {
    let rows = assemble_group_by_asset_category_data(
        &[AssetValueByCategoryRow {
            asset_category: "Computers".to_string(),
            value_as_on_from_date: 1000.0,
            value_of_new_purchase: 300.0,
            value_of_sold_asset: 100.0,
            value_of_scrapped_asset: 50.0,
            value_of_capitalized_asset: 25.0,
        }],
        &[AssetDepreciationByCategoryRow {
            asset_category: "Computers".to_string(),
            accumulated_depreciation_as_on_from_date: 200.0,
            depreciation_eliminated_via_reversal: 10.0,
            depreciation_eliminated_during_the_period: 20.0,
            depreciation_amount_during_the_period: 80.0,
        }],
        &[AssetValueAdjustmentRow {
            key: "Computers".to_string(),
            adjustment_before_from_date: 20.0,
            adjustment_till_to_date: 45.0,
        }],
    );

    assert_eq!(
        rows,
        vec![AssetDepreciationsAndBalancesRow {
            asset_category: Some("Computers".to_string()),
            asset: None,
            asset_name: None,
            value_as_on_from_date: 1020.0,
            value_of_new_purchase: 300.0,
            value_of_sold_asset: 100.0,
            value_of_scrapped_asset: 50.0,
            value_of_capitalized_asset: 25.0,
            adjustment_before_from_date: 20.0,
            adjustment_till_to_date: 45.0,
            adjustment_during_period: 25.0,
            value_as_on_to_date: 1170.0,
            accumulated_depreciation_as_on_from_date: 200.0,
            depreciation_amount_during_the_period: 80.0,
            depreciation_eliminated_during_the_period: 20.0,
            accumulated_depreciation_as_on_to_date: 250.0,
            depreciation_eliminated_via_reversal: 10.0,
            net_asset_value_as_on_from_date: 820.0,
            net_asset_value_as_on_to_date: 920.0,
        }]
    );
}

#[test]
fn asset_depreciations_asset_rows_default_missing_adjustments_like_erpnext() {
    let rows = assemble_group_by_asset_data(
        &[AssetDetailValueRow {
            name: "AST-0001".to_string(),
            asset_name: "Laptop".to_string(),
            value_as_on_from_date: 500.0,
            value_of_new_purchase: 50.0,
            value_of_sold_asset: 0.0,
            value_of_scrapped_asset: 0.0,
            value_of_capitalized_asset: 0.0,
        }],
        &[AssetDepreciationByAssetRow {
            asset: "AST-0001".to_string(),
            accumulated_depreciation_as_on_from_date: 125.0,
            depreciation_eliminated_via_reversal: 5.0,
            depreciation_eliminated_during_the_period: 10.0,
            depreciation_amount_during_the_period: 25.0,
        }],
        &[],
    );

    assert_eq!(
        rows[0],
        AssetDepreciationsAndBalancesRow {
            asset_category: None,
            asset: Some("AST-0001".to_string()),
            asset_name: Some("Laptop".to_string()),
            value_as_on_from_date: 500.0,
            value_of_new_purchase: 50.0,
            value_of_sold_asset: 0.0,
            value_of_scrapped_asset: 0.0,
            value_of_capitalized_asset: 0.0,
            adjustment_before_from_date: 0.0,
            adjustment_till_to_date: 0.0,
            adjustment_during_period: 0.0,
            value_as_on_to_date: 550.0,
            accumulated_depreciation_as_on_from_date: 125.0,
            depreciation_amount_during_the_period: 25.0,
            depreciation_eliminated_during_the_period: 10.0,
            accumulated_depreciation_as_on_to_date: 135.0,
            depreciation_eliminated_via_reversal: 5.0,
            net_asset_value_as_on_from_date: 375.0,
            net_asset_value_as_on_to_date: 415.0,
        }
    );
}

#[test]
#[should_panic(expected = "missing depreciation row for asset category Computers")]
fn asset_depreciations_category_rows_panic_when_depreciation_row_is_missing_like_erpnext() {
    assemble_group_by_asset_category_data(
        &[AssetValueByCategoryRow {
            asset_category: "Computers".to_string(),
            value_as_on_from_date: 1000.0,
            value_of_new_purchase: 0.0,
            value_of_sold_asset: 0.0,
            value_of_scrapped_asset: 0.0,
            value_of_capitalized_asset: 0.0,
        }],
        &[],
        &[],
    );
}

#[test]
#[should_panic(expected = "missing depreciation row for asset AST-0001")]
fn asset_depreciations_asset_rows_panic_when_depreciation_row_is_missing_like_erpnext() {
    assemble_group_by_asset_data(
        &[AssetDetailValueRow {
            name: "AST-0001".to_string(),
            asset_name: "Laptop".to_string(),
            value_as_on_from_date: 1000.0,
            value_of_new_purchase: 0.0,
            value_of_sold_asset: 0.0,
            value_of_scrapped_asset: 0.0,
            value_of_capitalized_asset: 0.0,
        }],
        &[],
        &[],
    );
}

#[test]
fn asset_depreciations_category_depreciation_combines_gl_and_opening_rows_like_erpnext() {
    let rows = combine_category_depreciation_rows(
        &[AssetDepreciationByCategoryRow {
            asset_category: "Furniture".to_string(),
            accumulated_depreciation_as_on_from_date: 100.0,
            depreciation_eliminated_via_reversal: 7.0,
            depreciation_eliminated_during_the_period: 11.0,
            depreciation_amount_during_the_period: 13.0,
        }],
        &[
            AssetOpeningDepreciationByCategoryRow {
                asset_category: "Computers".to_string(),
                accumulated_depreciation_as_on_from_date: 20.0,
                depreciation_eliminated_during_the_period: 3.0,
            },
            AssetOpeningDepreciationByCategoryRow {
                asset_category: "Furniture".to_string(),
                accumulated_depreciation_as_on_from_date: 30.0,
                depreciation_eliminated_during_the_period: 5.0,
            },
        ],
    );

    assert_eq!(
        rows,
        vec![
            AssetDepreciationByCategoryRow {
                asset_category: "Furniture".to_string(),
                accumulated_depreciation_as_on_from_date: 130.0,
                depreciation_eliminated_via_reversal: 7.0,
                depreciation_eliminated_during_the_period: 16.0,
                depreciation_amount_during_the_period: 13.0,
            },
            AssetDepreciationByCategoryRow {
                asset_category: "Computers".to_string(),
                accumulated_depreciation_as_on_from_date: 20.0,
                depreciation_eliminated_via_reversal: 0.0,
                depreciation_eliminated_during_the_period: 3.0,
                depreciation_amount_during_the_period: 0.0,
            },
        ]
    );
}

#[test]
fn asset_depreciations_asset_depreciation_combines_gl_and_opening_rows_like_erpnext() {
    let rows = combine_asset_depreciation_rows(
        &[AssetDepreciationByAssetRow {
            asset: "AST-0002".to_string(),
            accumulated_depreciation_as_on_from_date: 75.0,
            depreciation_eliminated_via_reversal: 2.0,
            depreciation_eliminated_during_the_period: 4.0,
            depreciation_amount_during_the_period: 9.0,
        }],
        &[
            AssetOpeningDepreciationByAssetRow {
                asset: "AST-0001".to_string(),
                accumulated_depreciation_as_on_from_date: 10.0,
                depreciation_eliminated_during_the_period: 1.0,
            },
            AssetOpeningDepreciationByAssetRow {
                asset: "AST-0002".to_string(),
                accumulated_depreciation_as_on_from_date: 15.0,
                depreciation_eliminated_during_the_period: 3.0,
            },
        ],
    );

    assert_eq!(
        rows,
        vec![
            AssetDepreciationByAssetRow {
                asset: "AST-0002".to_string(),
                accumulated_depreciation_as_on_from_date: 90.0,
                depreciation_eliminated_via_reversal: 2.0,
                depreciation_eliminated_during_the_period: 7.0,
                depreciation_amount_during_the_period: 9.0,
            },
            AssetDepreciationByAssetRow {
                asset: "AST-0001".to_string(),
                accumulated_depreciation_as_on_from_date: 10.0,
                depreciation_eliminated_via_reversal: 0.0,
                depreciation_eliminated_during_the_period: 1.0,
                depreciation_amount_during_the_period: 0.0,
            },
        ]
    );
}

#[test]
fn asset_depreciations_execute_routes_group_by_category_like_erpnext() {
    let data = AssetDepreciationsAndBalancesData {
        category_values: vec![AssetValueByCategoryRow {
            asset_category: "Computers".to_string(),
            value_as_on_from_date: 1000.0,
            value_of_new_purchase: 0.0,
            value_of_sold_asset: 0.0,
            value_of_scrapped_asset: 0.0,
            value_of_capitalized_asset: 0.0,
        }],
        category_depreciations: vec![AssetDepreciationByCategoryRow {
            asset_category: "Computers".to_string(),
            accumulated_depreciation_as_on_from_date: 100.0,
            depreciation_eliminated_via_reversal: 0.0,
            depreciation_eliminated_during_the_period: 0.0,
            depreciation_amount_during_the_period: 10.0,
        }],
        ..AssetDepreciationsAndBalancesData::default()
    };

    let report = execute(&filters("Asset Category"), &data);

    assert_eq!(report.columns, get_columns(&filters("Asset Category")));
    let rows = report.rows.expect("category rows");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].asset_category.as_deref(), Some("Computers"));
    assert_eq!(rows[0].net_asset_value_as_on_to_date, 890.0);
}

#[test]
fn asset_depreciations_get_data_routes_group_by_asset_and_unknown_group_like_erpnext() {
    let data = AssetDepreciationsAndBalancesData {
        asset_values: vec![AssetDetailValueRow {
            name: "AST-0001".to_string(),
            asset_name: "Laptop".to_string(),
            value_as_on_from_date: 800.0,
            value_of_new_purchase: 0.0,
            value_of_sold_asset: 0.0,
            value_of_scrapped_asset: 0.0,
            value_of_capitalized_asset: 0.0,
        }],
        asset_depreciations: vec![AssetDepreciationByAssetRow {
            asset: "AST-0001".to_string(),
            accumulated_depreciation_as_on_from_date: 100.0,
            depreciation_eliminated_via_reversal: 0.0,
            depreciation_eliminated_during_the_period: 0.0,
            depreciation_amount_during_the_period: 25.0,
        }],
        ..AssetDepreciationsAndBalancesData::default()
    };

    let rows = get_data(&filters("Asset"), &data).expect("asset rows");

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].asset.as_deref(), Some("AST-0001"));
    assert_eq!(rows[0].net_asset_value_as_on_to_date, 675.0);
    assert_eq!(get_data(&filters("Cost Center"), &data), None);
}

#[test]
fn asset_depreciations_category_value_query_plan_matches_erpnext_filters() {
    let plan = get_category_values_query_plan(&filtered("Asset Category"));

    assert_eq!(plan.source, "Asset");
    assert_eq!(
        plan.selects,
        vec![
            "asset.asset_category",
            "value_as_on_from_date",
            "value_of_new_purchase",
            "value_of_sold_asset",
            "value_of_scrapped_asset",
            "value_of_capitalized_asset",
        ]
    );
    assert_eq!(plan.group_by, Some("asset.asset_category"));
    assert!(plan.conditions.contains(&"asset.docstatus = 1".to_string()));
    assert!(plan
        .conditions
        .contains(&"asset.company = _Test Company".to_string()));
    assert!(plan
        .conditions
        .contains(&"asset.purchase_date <= 2026-05-31".to_string()));
    assert!(plan
        .conditions
        .contains(&"asset.name NOT IN capitalized assets before 2026-05-01".to_string()));
    assert!(plan
        .conditions
        .contains(&"asset.asset_category = Computers".to_string()));
    assert!(plan
        .conditions
        .contains(&"asset.name IN assets with finance_book IFRS".to_string()));
}

#[test]
fn asset_depreciations_asset_detail_query_plan_matches_erpnext_filters() {
    let plan = get_asset_details_query_plan(&filtered("Asset"));

    assert_eq!(plan.source, "Asset");
    assert!(plan.selects.contains(&"asset.name"));
    assert!(plan.selects.contains(&"asset.asset_name"));
    assert_eq!(plan.group_by, Some("asset.name"));
    assert!(plan
        .conditions
        .contains(&"asset.name = AST-0001".to_string()));
    assert!(!plan
        .conditions
        .contains(&"asset.asset_category = Computers".to_string()));
    assert!(plan
        .conditions
        .contains(&"asset.name IN assets with finance_book IFRS".to_string()));
}

#[test]
fn asset_depreciations_depreciation_query_plans_match_erpnext_group_filters() {
    let category = get_category_depreciation_query_plans(&filtered("Asset Category"));
    assert_eq!(category.gl.group_by, Some("asset.asset_category"));
    assert_eq!(category.opening.group_by, Some("asset.asset_category"));
    assert_eq!(
        category.gl.joins,
        vec!["Asset", "Asset Category Account", "Company"]
    );
    assert!(category
        .gl
        .conditions
        .contains(&"gl_entry.is_cancelled = 0".to_string()));
    assert!(category.gl.conditions.contains(
        &"gl_entry.account = ifnull(asset_category_account.depreciation_expense_account, company.depreciation_expense_account)".to_string()
    ));
    assert!(category
        .gl
        .conditions
        .contains(&"asset.asset_category = Computers".to_string()));
    assert!(category
        .opening
        .conditions
        .contains(&"asset.asset_category = Computers".to_string()));
    assert!(category
        .gl
        .conditions
        .contains(&"ifnull(gl_entry.finance_book, '') = IFRS".to_string()));
    assert!(category
        .opening
        .conditions
        .contains(&"asset.name IN assets with finance_book IFRS".to_string()));

    let asset = get_asset_depreciation_query_plans(&filtered("Asset"));
    assert_eq!(asset.gl.group_by, Some("asset.name"));
    assert_eq!(asset.opening.group_by, Some("asset.name"));
    assert!(asset
        .gl
        .conditions
        .contains(&"asset.name = AST-0001".to_string()));
    assert!(asset
        .opening
        .conditions
        .contains(&"asset.name = AST-0001".to_string()));
    assert!(!asset
        .gl
        .conditions
        .contains(&"asset.asset_category = Computers".to_string()));
}

#[test]
fn asset_depreciations_value_adjustment_query_plans_match_erpnext_shape() {
    let category =
        get_asset_value_adjustment_query_plan("Asset Category", &filtered("Asset Category"));
    assert_eq!(category.source, "GL Entry");
    assert_eq!(category.joins, vec!["Asset", "Asset Category Account"]);
    assert_eq!(category.group_by, Some("asset.asset_category"));
    assert!(category.selects.contains(&"asset.asset_category as key"));
    assert!(category
        .conditions
        .contains(&"gl_entry.account = asset_category_account.fixed_asset_account".to_string()));
    assert!(category
        .conditions
        .contains(&"gl_entry.is_opening = No".to_string()));

    let asset = get_asset_value_adjustment_query_plan("Asset", &filtered("Asset"));
    assert_eq!(asset.group_by, Some("asset.name"));
    assert!(asset.selects.contains(&"asset.name as key"));
}
