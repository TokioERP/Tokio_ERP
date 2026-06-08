pub mod stock_value_by_item_group;
pub mod warehouse_wise_stock_value;

#[derive(Clone, Debug, PartialEq)]
pub struct ChartDataset {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StockValueChart {
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
    pub chart_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseFilter {
    pub fieldname: &'static str,
    pub operator: &'static str,
    pub value: WarehouseFilterValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WarehouseFilterValue {
    Int(i64),
    Text(String),
}

impl WarehouseFilter {
    pub fn eq_int(fieldname: &'static str, value: i64) -> Self {
        Self {
            fieldname,
            operator: "=",
            value: WarehouseFilterValue::Int(value),
        }
    }

    pub fn eq_text(fieldname: &'static str, value: impl Into<String>) -> Self {
        Self {
            fieldname,
            operator: "=",
            value: WarehouseFilterValue::Text(value.into()),
        }
    }
}
