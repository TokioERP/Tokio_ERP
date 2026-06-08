use crate::erpnext::stock::dashboard_chart_source::{
    ChartDataset, StockValueChart, WarehouseFilter,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseStockValueRow {
    pub warehouse: String,
    pub stock_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseWiseStockValueReport {
    pub warehouse_filters: Vec<WarehouseFilter>,
    pub warehouse_order_by: Option<&'static str>,
    pub query: WarehouseWiseStockValueQuery,
    pub chart: StockValueChart,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseWiseStockValueQuery {
    pub fields: [&'static str; 2],
    pub bin_filters: BinStockValueFilters,
    pub group_by: &'static str,
    pub order_by: &'static str,
    pub limit_page_length: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinStockValueFilters {
    pub warehouses: Vec<String>,
    pub stock_value_operator: &'static str,
    pub stock_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WarehouseWiseStockValueResponse {
    Empty,
    Chart(WarehouseWiseStockValueReport),
}

impl WarehouseStockValueRow {
    pub fn new(warehouse: impl Into<String>, stock_value: f64) -> Self {
        Self {
            warehouse: warehouse.into(),
            stock_value,
        }
    }
}

pub fn get(
    filters_company: Option<&str>,
    warehouses: Vec<String>,
    rows: Vec<WarehouseStockValueRow>,
) -> WarehouseWiseStockValueResponse {
    let warehouse_filters = warehouse_filters(filters_company);
    let query = WarehouseWiseStockValueQuery {
        fields: ["warehouse", "SUM(stock_value) AS stock_value"],
        bin_filters: BinStockValueFilters {
            warehouses,
            stock_value_operator: ">",
            stock_value: 0.0,
        },
        group_by: "warehouse",
        order_by: "stock_value DESC",
        limit_page_length: 10,
    };

    if rows.is_empty() {
        return WarehouseWiseStockValueResponse::Empty;
    }

    let (labels, datapoints): (Vec<_>, Vec<_>) = rows
        .into_iter()
        .map(|row| (row.warehouse, row.stock_value))
        .unzip();

    WarehouseWiseStockValueResponse::Chart(WarehouseWiseStockValueReport {
        warehouse_filters,
        warehouse_order_by: Some("name"),
        query,
        chart: StockValueChart {
            labels,
            datasets: vec![ChartDataset {
                name: "Stock Value".to_string(),
                values: datapoints,
            }],
            chart_type: Some("bar".to_string()),
        },
    })
}

fn warehouse_filters(company: Option<&str>) -> Vec<WarehouseFilter> {
    let mut filters = vec![WarehouseFilter::eq_int("is_group", 0)];
    if let Some(company) = company {
        filters.push(WarehouseFilter::eq_text("company", company));
    }
    filters
}
