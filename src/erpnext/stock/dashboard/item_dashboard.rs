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
pub struct BinRow {
    pub item_code: String,
    pub warehouse: String,
    pub projected_qty: f64,
    pub reserved_qty: f64,
    pub reserved_qty_for_production: f64,
    pub reserved_qty_for_sub_contract: f64,
    pub actual_qty: f64,
    pub valuation_rate: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemInfo {
    pub item_code: String,
    pub item_name: Option<String>,
    pub stock_uom: Option<String>,
    pub has_batch_no: bool,
    pub has_serial_no: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemReservedStock {
    pub item_code: String,
    pub warehouse: String,
    pub reserved_stock: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemDashboardQueryPlan {
    pub doctype: &'static str,
    pub fields: [&'static str; 8],
    pub or_filters: Vec<DashboardFilter>,
    pub filters: Vec<DashboardFilter>,
    pub order_by: String,
    pub limit_start: usize,
    pub limit_page_length: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemDashboardRow {
    pub item_code: String,
    pub item_name: String,
    pub stock_uom: String,
    pub warehouse: String,
    pub disable_quick_entry: bool,
    pub projected_qty: f64,
    pub reserved_qty: f64,
    pub reserved_qty_for_production: f64,
    pub reserved_qty_for_sub_contract: f64,
    pub actual_qty: f64,
    pub valuation_rate: f64,
    pub reserved_stock: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemDashboardData {
    pub plan: ItemDashboardQueryPlan,
    pub reserved_stock_item_codes: Vec<String>,
    pub reserved_stock_warehouses: Vec<String>,
    pub rows: Vec<ItemDashboardRow>,
}

impl DashboardFilter {
    pub fn eq(fieldname: &'static str, value: DashboardFilterValue) -> Self {
        Self {
            fieldname,
            operator: "=",
            value,
        }
    }

    pub fn not_eq(fieldname: &'static str, value: DashboardFilterValue) -> Self {
        Self {
            fieldname,
            operator: "!=",
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

impl BinRow {
    pub fn new(item_code: impl Into<String>, warehouse: impl Into<String>) -> Self {
        Self {
            item_code: item_code.into(),
            warehouse: warehouse.into(),
            projected_qty: 0.0,
            reserved_qty: 0.0,
            reserved_qty_for_production: 0.0,
            reserved_qty_for_sub_contract: 0.0,
            actual_qty: 0.0,
            valuation_rate: 0.0,
        }
    }

    pub fn projected_qty(mut self, value: f64) -> Self {
        self.projected_qty = value;
        self
    }

    pub fn reserved_qty(mut self, value: f64) -> Self {
        self.reserved_qty = value;
        self
    }

    pub fn reserved_qty_for_production(mut self, value: f64) -> Self {
        self.reserved_qty_for_production = value;
        self
    }

    pub fn reserved_qty_for_sub_contract(mut self, value: f64) -> Self {
        self.reserved_qty_for_sub_contract = value;
        self
    }

    pub fn actual_qty(mut self, value: f64) -> Self {
        self.actual_qty = value;
        self
    }

    pub fn valuation_rate(mut self, value: f64) -> Self {
        self.valuation_rate = value;
        self
    }
}

impl ItemReservedStock {
    pub fn new(
        item_code: impl Into<String>,
        warehouse: impl Into<String>,
        reserved_stock: Option<f64>,
    ) -> Self {
        Self {
            item_code: item_code.into(),
            warehouse: warehouse.into(),
            reserved_stock,
        }
    }
}

pub fn get_data(
    item_code: Option<&str>,
    warehouse: Option<&str>,
    item_group_items: Option<Vec<String>>,
    start: usize,
    sort_by: &str,
    sort_order: &str,
    warehouse_permission: WarehousePermission,
    rows: Vec<BinRow>,
    item_info: &[ItemInfo],
    reserved_stock: &[ItemReservedStock],
    precision: i32,
) -> ItemDashboardData {
    let filters = get_filters(item_code, warehouse, item_group_items);
    let (no_permission, filters) =
        get_warehouse_filter_based_on_permissions(filters, warehouse_permission);
    if no_permission {
        return empty_data(filters, start, sort_by, sort_order);
    }

    get_bin_data(
        filters,
        start,
        sort_by,
        sort_order,
        rows,
        item_info,
        reserved_stock,
        precision,
    )
    .with_reserved_request_overrides(item_code, warehouse)
}

pub fn get_filters(
    item_code: Option<&str>,
    warehouse: Option<&str>,
    item_group_items: Option<Vec<String>>,
) -> Vec<DashboardFilter> {
    let mut filters = Vec::new();
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
    if let Some(items) = item_group_items {
        filters.push(DashboardFilter::in_list("item_code", items));
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

pub fn get_bin_data(
    filters: Vec<DashboardFilter>,
    start: usize,
    sort_by: &str,
    sort_order: &str,
    rows: Vec<BinRow>,
    item_info: &[ItemInfo],
    reserved_stock: &[ItemReservedStock],
    precision: i32,
) -> ItemDashboardData {
    let reserved_stock_item_codes = rows.iter().map(|row| row.item_code.clone()).collect();
    let reserved_stock_warehouses = rows.iter().map(|row| row.warehouse.clone()).collect();
    let rows = rows
        .into_iter()
        .map(|row| update_item_dashboard_row(row, item_info, reserved_stock, precision))
        .collect();

    ItemDashboardData {
        plan: bin_query_plan(filters, start, sort_by, sort_order),
        reserved_stock_item_codes,
        reserved_stock_warehouses,
        rows,
    }
}

fn empty_data(
    filters: Vec<DashboardFilter>,
    start: usize,
    sort_by: &str,
    sort_order: &str,
) -> ItemDashboardData {
    ItemDashboardData {
        plan: bin_query_plan(filters, start, sort_by, sort_order),
        reserved_stock_item_codes: Vec::new(),
        reserved_stock_warehouses: Vec::new(),
        rows: Vec::new(),
    }
}

fn bin_query_plan(
    filters: Vec<DashboardFilter>,
    start: usize,
    sort_by: &str,
    sort_order: &str,
) -> ItemDashboardQueryPlan {
    ItemDashboardQueryPlan {
        doctype: "Bin",
        fields: [
            "item_code",
            "warehouse",
            "projected_qty",
            "reserved_qty",
            "reserved_qty_for_production",
            "reserved_qty_for_sub_contract",
            "actual_qty",
            "valuation_rate",
        ],
        or_filters: vec![
            DashboardFilter::not_eq("projected_qty", DashboardFilterValue::Int(0)),
            DashboardFilter::not_eq("reserved_qty", DashboardFilterValue::Int(0)),
            DashboardFilter::not_eq("reserved_qty_for_production", DashboardFilterValue::Int(0)),
            DashboardFilter::not_eq(
                "reserved_qty_for_sub_contract",
                DashboardFilterValue::Int(0),
            ),
            DashboardFilter::not_eq("actual_qty", DashboardFilterValue::Int(0)),
        ],
        filters,
        order_by: format!("{sort_by} {sort_order}"),
        limit_start: start,
        limit_page_length: 21,
    }
}

fn update_item_dashboard_row(
    row: BinRow,
    item_info: &[ItemInfo],
    reserved_stock: &[ItemReservedStock],
    precision: i32,
) -> ItemDashboardRow {
    let info = item_info
        .iter()
        .find(|info| info.item_code == row.item_code);
    let reserved_stock_qty = reserved_stock
        .iter()
        .find(|reserved| reserved.item_code == row.item_code && reserved.warehouse == row.warehouse)
        .and_then(|reserved| reserved.reserved_stock)
        .unwrap_or(0.0);

    ItemDashboardRow {
        item_code: escape_html(&row.item_code),
        item_name: escape_html(
            info.and_then(|info| info.item_name.as_deref())
                .unwrap_or_default(),
        ),
        stock_uom: escape_html(
            info.and_then(|info| info.stock_uom.as_deref())
                .unwrap_or_default(),
        ),
        warehouse: escape_html(&row.warehouse),
        disable_quick_entry: info
            .map(|info| info.has_batch_no || info.has_serial_no)
            .unwrap_or(false),
        projected_qty: flt(row.projected_qty, Some(precision)),
        reserved_qty: flt(row.reserved_qty, Some(precision)),
        reserved_qty_for_production: flt(row.reserved_qty_for_production, Some(precision)),
        reserved_qty_for_sub_contract: flt(row.reserved_qty_for_sub_contract, Some(precision)),
        actual_qty: flt(row.actual_qty, Some(precision)),
        valuation_rate: row.valuation_rate,
        reserved_stock: flt(reserved_stock_qty, None),
    }
}

impl ItemDashboardData {
    fn with_reserved_request_overrides(
        mut self,
        item_code: Option<&str>,
        warehouse: Option<&str>,
    ) -> Self {
        if let Some(item_code) = item_code {
            self.reserved_stock_item_codes = vec![item_code.to_string()];
        }
        if let Some(warehouse) = warehouse {
            self.reserved_stock_warehouses = vec![warehouse.to_string()];
        }
        self
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
