use tokio_erp::erpnext::stock::dashboard_chart_source::stock_value_by_item_group::stock_value_by_item_group::{
    get as get_stock_value_by_item_group, get_stock_value_by_item_group as build_item_group_data,
    ItemGroupStockValueRow,
};
use tokio_erp::erpnext::stock::dashboard_chart_source::warehouse_wise_stock_value::warehouse_wise_stock_value::{
    get as get_warehouse_wise_stock_value, WarehouseStockValueRow, WarehouseWiseStockValueResponse,
};
use tokio_erp::erpnext::stock::dashboard_chart_source::{
    ChartDataset, StockValueChart, WarehouseFilter,
};

#[test]
fn stock_value_by_item_group_uses_filter_company_before_default_company() {
    let response = get_stock_value_by_item_group(
        Some("_Filter Company"),
        Some("_Default Company"),
        vec!["WH-A".to_string(), "WH-B".to_string()],
        vec![
            ItemGroupStockValueRow::new("Raw Material", 125.0),
            ItemGroupStockValueRow::new("Empty Group", 0.0),
            ItemGroupStockValueRow::new("Finished Goods", 90.0),
        ],
    );

    assert_eq!(
        response.warehouse_filters,
        vec![
            WarehouseFilter::eq_int("is_group", 0),
            WarehouseFilter::eq_text("company", "_Filter Company"),
        ]
    );
    assert_eq!(
        response.query.warehouse_filter,
        Some(vec!["WH-A".to_string(), "WH-B".to_string()])
    );
    assert_eq!(response.query.group_by, "Item.item_group");
    assert_eq!(response.query.order_by, "SUM(Bin.stock_value) desc");
    assert_eq!(response.query.limit, 10);
    assert_eq!(
        response.chart,
        StockValueChart {
            labels: vec!["Raw Material".to_string(), "Finished Goods".to_string()],
            datasets: vec![ChartDataset {
                name: "Stock Value".to_string(),
                values: vec![125.0, 90.0],
            }],
            chart_type: None,
        }
    );
}

#[test]
fn stock_value_by_item_group_falls_back_to_default_company_and_no_warehouse_where_when_empty() {
    let response =
        get_stock_value_by_item_group(None, Some("_Default Company"), Vec::new(), Vec::new());

    assert_eq!(
        response.warehouse_filters,
        vec![
            WarehouseFilter::eq_int("is_group", 0),
            WarehouseFilter::eq_text("company", "_Default Company"),
        ]
    );
    assert_eq!(response.query.warehouse_filter, None);
}

#[test]
fn stock_value_by_item_group_builder_skips_falsy_stock_values_like_erpnext() {
    assert_eq!(
        build_item_group_data(
            Some("_Test Company"),
            vec!["Stores - TC".to_string()],
            vec![
                ItemGroupStockValueRow::new("Zero", 0.0),
                ItemGroupStockValueRow::new("Negative", -5.0),
                ItemGroupStockValueRow::new("Positive", 5.0),
            ],
        )
        .chart
        .labels,
        vec!["Negative", "Positive"]
    );
}

#[test]
fn warehouse_wise_stock_value_query_and_chart_match_erpnext() {
    let response = get_warehouse_wise_stock_value(
        Some("_Test Company"),
        vec!["WH-A".to_string(), "WH-B".to_string()],
        vec![
            WarehouseStockValueRow::new("WH-B", 200.0),
            WarehouseStockValueRow::new("WH-A", 100.0),
        ],
    );

    let WarehouseWiseStockValueResponse::Chart(report) = response else {
        panic!("expected chart response");
    };

    assert_eq!(
        report.warehouse_filters,
        vec![
            WarehouseFilter::eq_int("is_group", 0),
            WarehouseFilter::eq_text("company", "_Test Company"),
        ]
    );
    assert_eq!(report.warehouse_order_by, Some("name"));
    assert_eq!(report.query.bin_filters.warehouses, vec!["WH-A", "WH-B"]);
    assert_eq!(report.query.bin_filters.stock_value_operator, ">");
    assert_eq!(report.query.bin_filters.stock_value, 0.0);
    assert_eq!(report.query.group_by, "warehouse");
    assert_eq!(report.query.order_by, "stock_value DESC");
    assert_eq!(report.query.limit_page_length, 10);
    assert_eq!(
        report.chart,
        StockValueChart {
            labels: vec!["WH-B".to_string(), "WH-A".to_string()],
            datasets: vec![ChartDataset {
                name: "Stock Value".to_string(),
                values: vec![200.0, 100.0],
            }],
            chart_type: Some("bar".to_string()),
        }
    );
}

#[test]
fn warehouse_wise_stock_value_returns_empty_list_when_bin_query_has_no_rows() {
    assert_eq!(
        get_warehouse_wise_stock_value(None, vec!["WH-A".to_string()], Vec::new()),
        WarehouseWiseStockValueResponse::Empty
    );
}
