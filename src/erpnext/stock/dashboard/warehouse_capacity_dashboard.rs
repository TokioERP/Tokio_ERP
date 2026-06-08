#[derive(Clone, Debug, PartialEq)]
pub struct DashboardFilter {
    pub fieldname: &'static str,
    pub operator: &'static str,
    pub value: DashboardFilterValue,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DashboardFilterValue {
    Int(i64),
    Text(String),
    TextList(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarehousePermission {
    Unrestricted,
    Restricted(Vec<String>),
    Denied,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PutawayRuleRow {
    pub item_code: String,
    pub warehouse: String,
    pub stock_capacity: f64,
    pub company: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StockBalance {
    pub item_code: String,
    pub warehouse: String,
    pub balance_qty: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseCapacityQueryPlan {
    pub doctype: &'static str,
    pub fields: [&'static str; 4],
    pub filters: Vec<DashboardFilter>,
    pub limit_start: usize,
    pub limit_page_length: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseCapacityRow {
    pub item_code: String,
    pub warehouse: String,
    pub stock_capacity: f64,
    pub company: String,
    pub actual_qty: f64,
    pub percent_occupied: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarehouseCapacityData {
    pub plan: WarehouseCapacityQueryPlan,
    pub rows: Vec<WarehouseCapacityRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarehouseCapacityError {
    DivisionByZero {
        item_code: String,
        warehouse: String,
    },
    UnknownSortKey(String),
}

impl DashboardFilter {
    pub fn eq(fieldname: &'static str, value: DashboardFilterValue) -> Self {
        Self {
            fieldname,
            operator: "=",
            value,
        }
    }

    pub fn in_list(fieldname: &'static str, values: Vec<String>) -> Self {
        Self {
            fieldname,
            operator: "in",
            value: DashboardFilterValue::TextList(values),
        }
    }
}

impl PutawayRuleRow {
    pub fn new(
        item_code: impl Into<String>,
        warehouse: impl Into<String>,
        stock_capacity: f64,
        company: impl Into<String>,
    ) -> Self {
        Self {
            item_code: item_code.into(),
            warehouse: warehouse.into(),
            stock_capacity,
            company: company.into(),
        }
    }
}

impl StockBalance {
    pub fn new(
        item_code: impl Into<String>,
        warehouse: impl Into<String>,
        balance_qty: Option<f64>,
    ) -> Self {
        Self {
            item_code: item_code.into(),
            warehouse: warehouse.into(),
            balance_qty,
        }
    }
}

pub fn get_data(
    item_code: Option<&str>,
    warehouse: Option<&str>,
    parent_warehouse_descendants: Option<Vec<String>>,
    company: Option<&str>,
    start: usize,
    sort_by: &str,
    sort_order: &str,
    warehouse_permission: WarehousePermission,
    rows: Vec<PutawayRuleRow>,
    balances: &[StockBalance],
) -> Result<Vec<WarehouseCapacityRow>, WarehouseCapacityError> {
    let filters = get_filters(item_code, warehouse, parent_warehouse_descendants, company);
    let (no_permission, filters) =
        get_warehouse_filter_based_on_permissions(filters, warehouse_permission);
    if no_permission {
        return Ok(Vec::new());
    }

    let mut capacity_data = get_warehouse_capacity_data(filters, start, rows, balances)?.rows;
    sort_capacity_data(&mut capacity_data, sort_by, sort_order)?;
    Ok(capacity_data)
}

pub fn get_filters(
    item_code: Option<&str>,
    warehouse: Option<&str>,
    parent_warehouse_descendants: Option<Vec<String>>,
    company: Option<&str>,
) -> Vec<DashboardFilter> {
    let mut filters = vec![DashboardFilter::eq("disable", DashboardFilterValue::Int(0))];

    if let Some(item_code) = item_code {
        filters.push(DashboardFilter::eq(
            "item_code",
            DashboardFilterValue::Text(item_code.to_string()),
        ));
    }
    if let Some(warehouse) = warehouse {
        filters.push(DashboardFilter::eq(
            "warehouse",
            DashboardFilterValue::Text(warehouse.to_string()),
        ));
    }
    if let Some(company) = company {
        filters.push(DashboardFilter::eq(
            "company",
            DashboardFilterValue::Text(company.to_string()),
        ));
    }
    if let Some(warehouses) = parent_warehouse_descendants {
        filters.push(DashboardFilter::in_list("warehouse", warehouses));
    }

    filters
}

pub fn get_warehouse_filter_based_on_permissions(
    mut filters: Vec<DashboardFilter>,
    warehouse_permission: WarehousePermission,
) -> (bool, Vec<DashboardFilter>) {
    match warehouse_permission {
        WarehousePermission::Unrestricted => (false, filters),
        WarehousePermission::Restricted(warehouses) => {
            filters.push(DashboardFilter::in_list("warehouse", warehouses));
            (false, filters)
        }
        WarehousePermission::Denied => (true, Vec::new()),
    }
}

pub fn get_warehouse_capacity_data(
    filters: Vec<DashboardFilter>,
    start: usize,
    rows: Vec<PutawayRuleRow>,
    balances: &[StockBalance],
) -> Result<WarehouseCapacityData, WarehouseCapacityError> {
    let plan = WarehouseCapacityQueryPlan {
        doctype: "Putaway Rule",
        fields: ["item_code", "warehouse", "stock_capacity", "company"],
        filters,
        limit_start: start,
        limit_page_length: "11",
    };
    let rows = rows
        .into_iter()
        .map(|row| update_capacity_row(row, balances))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(WarehouseCapacityData { plan, rows })
}

fn update_capacity_row(
    row: PutawayRuleRow,
    balances: &[StockBalance],
) -> Result<WarehouseCapacityRow, WarehouseCapacityError> {
    let balance_qty = balances
        .iter()
        .find(|balance| balance.item_code == row.item_code && balance.warehouse == row.warehouse)
        .and_then(|balance| balance.balance_qty)
        .unwrap_or(0.0);
    let stock_capacity = flt(row.stock_capacity, None);
    if stock_capacity == 0.0 {
        return Err(WarehouseCapacityError::DivisionByZero {
            item_code: row.item_code,
            warehouse: row.warehouse,
        });
    }

    let percent_occupied = flt((flt(balance_qty, None) / stock_capacity) * 100.0, Some(0));

    Ok(WarehouseCapacityRow {
        item_code: escape_html(&row.item_code),
        warehouse: escape_html(&row.warehouse),
        stock_capacity: row.stock_capacity,
        company: escape_html(&row.company),
        actual_qty: balance_qty,
        percent_occupied,
    })
}

fn sort_capacity_data(
    rows: &mut [WarehouseCapacityRow],
    sort_by: &str,
    sort_order: &str,
) -> Result<(), WarehouseCapacityError> {
    numeric_value(rows.first(), sort_by)?;
    let desc = sort_order == "desc";
    rows.sort_by(|left, right| {
        let left = numeric_value(Some(left), sort_by).unwrap();
        let right = numeric_value(Some(right), sort_by).unwrap();
        if desc {
            right.total_cmp(&left)
        } else {
            left.total_cmp(&right)
        }
    });
    Ok(())
}

fn numeric_value(
    row: Option<&WarehouseCapacityRow>,
    sort_by: &str,
) -> Result<f64, WarehouseCapacityError> {
    let Some(row) = row else {
        return Ok(0.0);
    };
    match sort_by {
        "stock_capacity" => Ok(row.stock_capacity),
        "actual_qty" => Ok(row.actual_qty),
        "percent_occupied" => Ok(row.percent_occupied),
        other => Err(WarehouseCapacityError::UnknownSortKey(other.to_string())),
    }
}

fn flt(value: f64, precision: Option<i32>) -> f64 {
    let Some(precision) = precision else {
        return value;
    };
    bankers_rounding_legacy(value, precision)
}

fn bankers_rounding_legacy(value: f64, precision: i32) -> f64 {
    let multiplier = 10_f64.powi(precision);
    let scaled = if precision == 0 {
        round_to_places(value, 8)
    } else {
        round_to_places(value * multiplier, 8)
    };
    let floor = scaled.floor();
    let decimal = scaled - floor;
    let rounded = if decimal == 0.5 {
        if precision == 0 {
            if floor as i64 % 2 == 0 {
                floor
            } else {
                floor + 1.0
            }
        } else {
            floor + 1.0
        }
    } else {
        scaled.round()
    };

    if precision == 0 {
        rounded
    } else {
        rounded / multiplier
    }
}

fn round_to_places(value: f64, places: i32) -> f64 {
    let multiplier = 10_f64.powi(places);
    (value * multiplier).round() / multiplier
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
