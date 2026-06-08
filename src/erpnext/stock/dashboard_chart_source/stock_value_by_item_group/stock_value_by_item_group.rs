use crate::erpnext::stock::dashboard_chart_source::{
    ChartDataset, StockValueChart, WarehouseFilter,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ItemGroupStockValueRow {
    pub item_group: String,
    pub stock_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemGroupStockValueQuery {
    pub doctype: &'static str,
    pub join_doctype: &'static str,
    pub join_on: &'static str,
    pub select: [&'static str; 2],
    pub group_by: &'static str,
    pub order_by: &'static str,
    pub limit: usize,
    pub warehouse_filter: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StockValueByItemGroupResponse {
    pub company: Option<String>,
    pub warehouse_filters: Vec<WarehouseFilter>,
    pub query: ItemGroupStockValueQuery,
    pub chart: StockValueChart,
}

impl ItemGroupStockValueRow {
    pub fn new(item_group: impl Into<String>, stock_value: f64) -> Self {
        Self {
            item_group: item_group.into(),
            stock_value,
        }
    }
}

pub fn get(
    filters_company: Option<&str>,
    default_company: Option<&str>,
    warehouses: Vec<String>,
    rows: Vec<ItemGroupStockValueRow>,
) -> StockValueByItemGroupResponse {
    let company = filters_company.or(default_company);
    get_stock_value_by_item_group(company, warehouses, rows)
}

pub fn get_stock_value_by_item_group(
    company: Option<&str>,
    warehouses: Vec<String>,
    rows: Vec<ItemGroupStockValueRow>,
) -> StockValueByItemGroupResponse {
    let warehouse_filters = warehouse_filters(company);
    let query = ItemGroupStockValueQuery {
        doctype: "Bin",
        join_doctype: "Item",
        join_on: "Bin.item_code = Item.name",
        select: ["Item.item_group", "SUM(Bin.stock_value) AS stock_value"],
        group_by: "Item.item_group",
        order_by: "SUM(Bin.stock_value) desc",
        limit: 10,
        warehouse_filter: (!warehouses.is_empty()).then_some(warehouses),
    };
    let (labels, datapoints): (Vec<_>, Vec<_>) = rows
        .into_iter()
        .filter(|row| row.stock_value != 0.0)
        .map(|row| (row.item_group, row.stock_value))
        .unzip();

    StockValueByItemGroupResponse {
        company: company.map(ToOwned::to_owned),
        warehouse_filters,
        query,
        chart: StockValueChart {
            labels,
            datasets: vec![ChartDataset {
                name: "Stock Value".to_string(),
                values: datapoints,
            }],
            chart_type: None,
        },
    }
}

fn warehouse_filters(company: Option<&str>) -> Vec<WarehouseFilter> {
    let mut filters = vec![WarehouseFilter::eq_int("is_group", 0)];
    if let Some(company) = company {
        filters.push(WarehouseFilter::eq_text("company", company));
    }
    filters
}
