use tokio_erp::erpnext::accounts::report::asset_depreciations_and_balances::asset_depreciations_and_balances::{
    assemble_group_by_asset_category_data, assemble_group_by_asset_data,
    combine_asset_depreciation_rows, combine_category_depreciation_rows, execute, get_columns,
    get_data, AssetDepreciationByAssetRow, AssetDepreciationByCategoryRow,
    AssetDepreciationsAndBalancesData, AssetOpeningDepreciationByAssetRow,
    AssetOpeningDepreciationByCategoryRow,
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
    assert_eq!(report.rows.len(), 1);
    assert_eq!(report.rows[0].asset_category.as_deref(), Some("Computers"));
    assert_eq!(report.rows[0].net_asset_value_as_on_to_date, 890.0);
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

    let rows = get_data(&filters("Asset"), &data);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].asset.as_deref(), Some("AST-0001"));
    assert_eq!(rows[0].net_asset_value_as_on_to_date, 675.0);
    assert!(get_data(&filters("Cost Center"), &data).is_empty());
}
