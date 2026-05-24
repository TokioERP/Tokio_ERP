use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::financial_ratios::financial_ratios::{
    calculate_ratio, get_columns, get_ratios_data_from_gl, setup_filters, update_balances,
    AccountEntry, FinancialRatioFilters, FinancialRatioInput, FiscalYearSpan, Period, RatioColumn,
};

#[test]
fn financial_ratios_setup_filters_matches_erpnext_defaults() {
    let fiscal_years = [
        FiscalYearSpan::new("FY2025", "2025-01-01", "2025-12-31"),
        FiscalYearSpan::new("FY2026", "2026-01-01", "2026-12-31"),
    ];
    let mut filters = FinancialRatioFilters::new("FY2025", "FY2026", "Test Company");

    setup_filters(&mut filters, &fiscal_years).unwrap();

    assert_eq!(filters.period_start_date.as_deref(), Some("2025-01-01"));
    assert_eq!(filters.period_end_date.as_deref(), Some("2026-12-31"));
    assert_eq!(filters.filter_based_on.as_deref(), Some("Fiscal Year"));
}

#[test]
fn financial_ratios_columns_return_year_keys_like_erpnext() {
    let periods = vec![
        Period::new("2025", "FY 2025", "2025-01-01", "2025-12-31"),
        Period::new("2026", "FY 2026", "2026-01-01", "2026-12-31"),
    ];

    let (columns, years) = get_columns(&periods);

    assert_eq!(
        columns,
        vec![
            RatioColumn::data("Ratios", "ratio", 200),
            RatioColumn::float("FY 2025", "2025", 150),
            RatioColumn::float("FY 2026", "2026", 150),
        ]
    );
    assert_eq!(years, vec!["2025".to_string(), "2026".to_string()]);
}

#[test]
fn financial_ratios_calculate_ratio_matches_frappe_flt_precision() {
    assert_eq!(calculate_ratio(10.0, 4.0, 2), 2.5);
    assert_eq!(calculate_ratio(1.0, 3.0, 2), 0.33);
    assert_eq!(calculate_ratio(10.0, 0.0, 2), 0.0);
}

#[test]
fn financial_ratios_update_balances_matches_erpnext_asset_and_expense_branches() {
    let year = "2026";
    let assets = vec![
        AccountEntry::root("Total Asset").with_amount(year, 1000.0),
        AccountEntry::group("Current Asset", "Current Asset").with_amount(year, 400.0),
        AccountEntry::leaf("Bank Account", "Bank").with_amount(year, 100.0),
        AccountEntry::leaf("Cash Account", "Cash").with_amount(year, 50.0),
        AccountEntry::leaf("Receivable Account", "Receivable").with_amount(year, 80.0),
    ];

    let mut current_asset: BTreeMap<String, f64> = Default::default();
    let mut total_asset: BTreeMap<String, f64> = Default::default();
    let mut quick_asset: BTreeMap<String, f64> = Default::default();
    update_balances(
        &mut current_asset,
        &mut total_asset,
        "Current Asset",
        year,
        &assets,
        "Asset",
        Some(&mut quick_asset),
    );

    assert_eq!(current_asset.get(year), Some(&400.0));
    assert_eq!(total_asset.get(year), Some(&1000.0));
    assert_eq!(quick_asset.get(year), Some(&230.0));

    let expenses = vec![
        AccountEntry::root("Total Expense").with_amount(year, 300.0),
        AccountEntry::group("Direct Expense", "Direct Expense").with_amount(year, 80.0),
    ];
    let mut direct_expense: BTreeMap<String, f64> = Default::default();
    let mut direct_expense_total: BTreeMap<String, f64> = Default::default();
    update_balances(
        &mut direct_expense,
        &mut direct_expense_total,
        "Direct Expense",
        year,
        &expenses,
        "Expense",
        None,
    );
    assert_eq!(direct_expense.get(year), Some(&80.0));
}

#[test]
fn financial_ratios_builds_liquidity_solvency_and_turnover_rows_like_erpnext() {
    let periods = vec![Period::new("2026", "FY 2026", "2026-01-01", "2026-12-31")];
    let years = vec!["2026".to_string()];

    let input = FinancialRatioInput {
        precision: 2,
        assets: vec![
            AccountEntry::root("Total Asset").with_amount("2026", 1000.0),
            AccountEntry::group("Current Asset", "Current Asset").with_amount("2026", 400.0),
            AccountEntry::leaf("Bank Account", "Bank").with_amount("2026", 100.0),
            AccountEntry::leaf("Cash Account", "Cash").with_amount("2026", 50.0),
            AccountEntry::leaf("Receivable Account", "Receivable").with_amount("2026", 80.0),
        ],
        liabilities: vec![
            AccountEntry::root("Total Liability").with_amount("2026", 600.0),
            AccountEntry::group("Current Liability", "Current Liability")
                .with_amount("2026", 200.0),
        ],
        income: vec![
            AccountEntry::root("Total Income").with_amount("2026", 700.0),
            AccountEntry::group("Direct Income", "Direct Income").with_amount("2026", 500.0),
        ],
        expense: vec![
            AccountEntry::root("Total Expense").with_amount("2026", 300.0),
            AccountEntry::group("Cost of Goods Sold", "Cost of Goods Sold")
                .with_amount("2026", 120.0),
            AccountEntry::group("Direct Expense", "Direct Expense").with_amount("2026", 80.0),
        ],
        avg_debtors: [("2026".to_string(), 250.0)].into(),
        avg_creditors: [("2026".to_string(), 40.0)].into(),
        avg_stock: [("2026".to_string(), 30.0)].into(),
    };

    let rows = get_ratios_data_from_gl(&periods, &years, input);

    assert_eq!(rows[0].ratio, "Liquidity Ratios");
    assert_eq!(rows[1].ratio, "Current Ratio");
    assert_eq!(rows[1].value("2026"), Some(2.0));
    assert_eq!(rows[2].ratio, "Quick Ratio");
    assert_eq!(rows[2].value("2026"), Some(1.15));
    assert_eq!(rows[4].ratio, "Debt Equity Ratio");
    assert_eq!(rows[4].value("2026"), Some(1.5));
    assert_eq!(rows[5].ratio, "Gross Profit Ratio");
    assert_eq!(rows[5].value("2026"), Some(0.76));
    assert_eq!(rows[6].ratio, "Net Profit Ratio");
    assert_eq!(rows[6].value("2026"), Some(0.8));
    assert_eq!(rows[13].ratio, "Inventory Turnover Ratio");
    assert_eq!(rows[13].value("2026"), Some(4.0));
}
