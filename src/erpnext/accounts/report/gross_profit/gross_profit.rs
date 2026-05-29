use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrossProfitFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub group_by: String,
    pub currency: String,
    pub currency_precision: u32,
    pub float_precision: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MasterNameSettings {
    pub supplier_master_name: String,
    pub customer_master_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrossProfitSourceRow {
    pub parent: String,
    pub invoice_or_item: String,
    pub customer: String,
    pub customer_group: String,
    pub customer_name: String,
    pub posting_date: String,
    pub item_code: String,
    pub item_name: String,
    pub item_group: String,
    pub brand: String,
    pub description: String,
    pub warehouse: String,
    pub qty: f64,
    pub base_net_amount: f64,
    pub buying_amount: f64,
    pub project: String,
    pub cost_center: String,
    pub territory: String,
    pub sales_person: String,
    pub allocated_amount: f64,
    pub payment_term: String,
    pub is_return: bool,
    pub invoice_portion: f64,
    pub payment_amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GrossProfitCalculatedRow {
    pub invoice_or_item: String,
    pub customer: String,
    pub customer_group: String,
    pub customer_name: String,
    pub posting_date: String,
    pub item_code: String,
    pub item_name: String,
    pub item_group: String,
    pub brand: String,
    pub description: String,
    pub warehouse: String,
    pub qty: f64,
    pub base_rate: f64,
    pub buying_rate: f64,
    pub base_amount: f64,
    pub buying_amount: f64,
    pub gross_profit: f64,
    pub gross_profit_percent: f64,
    pub project: String,
    pub cost_center: String,
    pub territory: String,
    pub sales_person: String,
    pub allocated_amount: f64,
    pub payment_term: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReportCell {
    Text(String),
    Number(f64),
    Empty,
}

impl Default for MasterNameSettings {
    fn default() -> Self {
        Self {
            supplier_master_name: String::new(),
            customer_master_name: String::new(),
        }
    }
}

impl ReportColumn {
    pub const fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Link",
            options,
            width,
            hidden: false,
        }
    }

    pub const fn hidden_link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Link",
            options,
            width: 0,
            hidden: true,
        }
    }

    pub const fn date(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Date",
            options: "",
            width,
            hidden: false,
        }
    }

    pub const fn data(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Data",
            options: "",
            width,
            hidden: false,
        }
    }

    pub const fn float(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Float",
            options: "",
            width,
            hidden: false,
        }
    }

    pub const fn currency(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Currency",
            options,
            width,
            hidden: false,
        }
    }

    pub const fn percent(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Percent",
            options: "",
            width,
            hidden: false,
        }
    }
}

pub fn get_group_wise_columns() -> BTreeMap<&'static str, Vec<&'static str>> {
    BTreeMap::from([
        (
            "invoice",
            vec![
                "invoice_or_item",
                "customer",
                "customer_group",
                "customer_name",
                "posting_date",
                "item_code",
                "item_name",
                "item_group",
                "brand",
                "description",
                "warehouse",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
                "project",
            ],
        ),
        (
            "item_code",
            vec![
                "item_code",
                "item_name",
                "brand",
                "description",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "warehouse",
            vec![
                "warehouse",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "brand",
            vec![
                "brand",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "item_group",
            vec![
                "item_group",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "customer",
            vec![
                "customer",
                "customer_group",
                "customer_name",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "customer_group",
            vec![
                "customer_group",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "sales_person",
            vec![
                "sales_person",
                "allocated_amount",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "project",
            vec![
                "project",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "cost_center",
            vec![
                "cost_center",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "territory",
            vec![
                "territory",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "monthly",
            vec![
                "monthly",
                "qty",
                "base_rate",
                "buying_rate",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
        (
            "payment_term",
            vec![
                "payment_term",
                "base_amount",
                "buying_amount",
                "gross_profit",
                "gross_profit_percent",
            ],
        ),
    ])
}

pub fn get_column_names() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("invoice_or_item", "sales_invoice"),
        ("customer", "customer"),
        ("customer_group", "customer_group"),
        ("customer_name", "customer_name"),
        ("posting_date", "posting_date"),
        ("item_code", "item_code"),
        ("item_name", "item_name"),
        ("item_group", "item_group"),
        ("brand", "brand"),
        ("description", "description"),
        ("warehouse", "warehouse"),
        ("qty", "qty"),
        ("base_rate", "avg._selling_rate"),
        ("buying_rate", "valuation_rate"),
        ("base_amount", "selling_amount"),
        ("buying_amount", "buying_amount"),
        ("gross_profit", "gross_profit"),
        ("gross_profit_percent", "gross_profit_%"),
        ("project", "project"),
    ])
}

pub fn get_columns(
    filters: &GrossProfitFilters,
    master_settings: &MasterNameSettings,
) -> Vec<ReportColumn> {
    let group_key = scrub(&filters.group_by);
    let column_map = column_map();
    let mut columns = Vec::new();

    for col in get_group_wise_columns()
        .get(group_key.as_str())
        .into_iter()
        .flatten()
    {
        if *col == "customer_name" && hides_customer_name(master_settings) {
            continue;
        }
        if let Some(column) = column_map.get(col) {
            columns.push(column.clone());
        }
    }

    columns.push(ReportColumn::hidden_link(
        "Currency", "currency", "Currency",
    ));
    columns
}

pub fn calculate_row(
    row: &GrossProfitSourceRow,
    filters: &GrossProfitFilters,
) -> GrossProfitCalculatedRow {
    let base_amount = round_to(row.base_net_amount, filters.currency_precision);
    let buying_amount = round_to(row.buying_amount, filters.currency_precision);
    let base_rate = if row.qty != 0.0 {
        round_to(base_amount / row.qty, filters.float_precision)
    } else {
        0.0
    };
    let buying_rate = if row.qty != 0.0 {
        round_to(buying_amount / row.qty, filters.float_precision)
    } else {
        0.0
    };
    let gross_profit =
        calculate_gross_profit(base_amount, buying_amount, filters.currency_precision);
    let gross_profit_percent =
        calculate_gross_profit_percent(gross_profit, base_amount, filters.currency_precision);

    GrossProfitCalculatedRow {
        invoice_or_item: row.invoice_or_item.clone(),
        customer: row.customer.clone(),
        customer_group: row.customer_group.clone(),
        customer_name: row.customer_name.clone(),
        posting_date: row.posting_date.clone(),
        item_code: row.item_code.clone(),
        item_name: row.item_name.clone(),
        item_group: row.item_group.clone(),
        brand: row.brand.clone(),
        description: row.description.clone(),
        warehouse: row.warehouse.clone(),
        qty: row.qty,
        base_rate,
        buying_rate,
        base_amount,
        buying_amount,
        gross_profit,
        gross_profit_percent,
        project: row.project.clone(),
        cost_center: row.cost_center.clone(),
        territory: row.territory.clone(),
        sales_person: row.sales_person.clone(),
        allocated_amount: row.allocated_amount,
        payment_term: row.payment_term.clone(),
    }
}

pub fn group_rows(
    source_rows: &[GrossProfitSourceRow],
    filters: &GrossProfitFilters,
    master_settings: &MasterNameSettings,
) -> Vec<Vec<ReportCell>> {
    let group_key = scrub(&filters.group_by);
    let mut grouped: BTreeMap<String, GrossProfitCalculatedRow> = BTreeMap::new();
    let mut order = Vec::new();

    for source in source_rows {
        let row = calculate_row(source, filters);
        let Some(key) = row_text_value(&row, &group_key) else {
            continue;
        };
        if key.is_empty() {
            continue;
        }

        if let Some(existing) = grouped.get_mut(&key) {
            existing.qty = round_to(existing.qty + row.qty, filters.float_precision);
            existing.buying_amount = round_to(
                existing.buying_amount + row.buying_amount,
                filters.currency_precision,
            );
            existing.base_amount = round_to(
                existing.base_amount + row.base_amount,
                filters.currency_precision,
            );
            if group_key == "sales_person" {
                existing.allocated_amount = round_to(
                    existing.allocated_amount + row.allocated_amount,
                    filters.currency_precision,
                );
            }
            set_average_rate(existing, filters);
        } else {
            order.push(key.clone());
            grouped.insert(key, row);
        }
    }

    let group_columns = visible_group_columns(filters, master_settings);
    let mut rows = Vec::new();
    let mut total_base_amount = 0.0;
    let mut total_buying_amount = 0.0;

    for key in order {
        let row = grouped.get(&key).expect("grouped row");
        total_base_amount += row.base_amount;
        total_buying_amount += row.buying_amount;
        rows.push(project_row(row, &group_columns, &filters.currency));
    }

    rows.push(total_row(
        &group_columns,
        total_base_amount,
        total_buying_amount,
        filters,
    ));
    rows
}

fn column_map() -> BTreeMap<&'static str, ReportColumn> {
    BTreeMap::from([
        (
            "parent",
            ReportColumn::link("Sales Invoice", "parent_invoice", "Sales Invoice", 120),
        ),
        (
            "invoice_or_item",
            ReportColumn::link("Sales Invoice", "sales_invoice", "Sales Invoice", 120),
        ),
        (
            "posting_date",
            ReportColumn::date("Posting Date", "posting_date", 120),
        ),
        (
            "posting_time",
            ReportColumn::data("Posting Time", "posting_time", 100),
        ),
        (
            "item_code",
            ReportColumn::link("Item Code", "item_code", "Item", 100),
        ),
        (
            "item_name",
            ReportColumn::data("Item Name", "item_name", 100),
        ),
        (
            "item_group",
            ReportColumn::link("Item Group", "item_group", "Item Group", 100),
        ),
        ("brand", ReportColumn::link("Brand", "brand", "Brand", 100)),
        (
            "description",
            ReportColumn::data("Description", "description", 100),
        ),
        (
            "warehouse",
            ReportColumn::link("Warehouse", "warehouse", "Warehouse", 100),
        ),
        ("qty", ReportColumn::float("Qty", "qty", 80)),
        (
            "base_rate",
            ReportColumn::currency("Avg. Selling Rate", "avg._selling_rate", "currency", 100),
        ),
        (
            "buying_rate",
            ReportColumn::currency("Valuation Rate", "valuation_rate", "currency", 100),
        ),
        (
            "base_amount",
            ReportColumn::currency("Selling Amount", "selling_amount", "currency", 100),
        ),
        (
            "buying_amount",
            ReportColumn::currency("Buying Amount", "buying_amount", "currency", 100),
        ),
        (
            "gross_profit",
            ReportColumn::currency("Gross Profit", "gross_profit", "currency", 100),
        ),
        (
            "gross_profit_percent",
            ReportColumn::percent("Gross Profit Percent", "gross_profit_%", 100),
        ),
        (
            "project",
            ReportColumn::link("Project", "project", "Project", 140),
        ),
        (
            "cost_center",
            ReportColumn::link("Cost Center", "cost_center", "Cost Center", 140),
        ),
        (
            "sales_person",
            ReportColumn::link("Sales Person", "sales_person", "Sales Person", 100),
        ),
        (
            "allocated_amount",
            ReportColumn::currency("Allocated Amount", "allocated_amount", "currency", 100),
        ),
        (
            "customer",
            ReportColumn::link("Customer", "customer", "Customer", 100),
        ),
        (
            "customer_group",
            ReportColumn::link("Customer Group", "customer_group", "Customer Group", 100),
        ),
        (
            "customer_name",
            ReportColumn::data("Customer Name", "customer_name", 150),
        ),
        (
            "territory",
            ReportColumn::link("Territory", "territory", "Territory", 100),
        ),
        ("monthly", ReportColumn::data("Monthly", "monthly", 100)),
        (
            "payment_term",
            ReportColumn::link("Payment Term", "payment_term", "Payment Term", 170),
        ),
    ])
}

fn visible_group_columns(
    filters: &GrossProfitFilters,
    master_settings: &MasterNameSettings,
) -> Vec<&'static str> {
    let group_key = scrub(&filters.group_by);
    get_group_wise_columns()
        .get(group_key.as_str())
        .into_iter()
        .flatten()
        .copied()
        .filter(|col| !(*col == "customer_name" && hides_customer_name(master_settings)))
        .collect()
}

fn project_row(
    row: &GrossProfitCalculatedRow,
    group_columns: &[&str],
    currency: &str,
) -> Vec<ReportCell> {
    let mut cells: Vec<ReportCell> = group_columns
        .iter()
        .map(|col| row_cell_value(row, col))
        .collect();
    cells.push(ReportCell::Text(currency.to_string()));
    cells
}

fn total_row(
    group_columns: &[&str],
    total_base_amount: f64,
    total_buying_amount: f64,
    filters: &GrossProfitFilters,
) -> Vec<ReportCell> {
    let total_gross_profit = calculate_gross_profit(
        total_base_amount,
        total_buying_amount,
        filters.currency_precision,
    );
    let total_percent = calculate_gross_profit_percent(
        total_gross_profit,
        total_base_amount,
        filters.currency_precision,
    );

    let mut cells = Vec::new();
    for (index, col) in group_columns.iter().enumerate() {
        cells.push(match *col {
            _ if index == 0 => ReportCell::Text("Total".to_string()),
            "base_amount" => {
                ReportCell::Number(round_to(total_base_amount, filters.currency_precision))
            }
            "buying_amount" => {
                ReportCell::Number(round_to(total_buying_amount, filters.currency_precision))
            }
            "gross_profit" => ReportCell::Number(total_gross_profit),
            "gross_profit_percent" => ReportCell::Number(total_percent),
            _ => ReportCell::Empty,
        });
    }
    cells.push(ReportCell::Text(filters.currency.clone()));
    cells
}

fn set_average_rate(row: &mut GrossProfitCalculatedRow, filters: &GrossProfitFilters) {
    row.gross_profit = calculate_gross_profit(
        row.base_amount,
        row.buying_amount,
        filters.currency_precision,
    );
    row.gross_profit_percent = calculate_gross_profit_percent(
        row.gross_profit,
        row.base_amount,
        filters.currency_precision,
    );
    row.buying_rate = if row.qty != 0.0 {
        round_to(row.buying_amount / row.qty, filters.float_precision)
    } else {
        0.0
    };
    row.base_rate = if row.qty != 0.0 {
        round_to(row.base_amount / row.qty, filters.float_precision)
    } else {
        0.0
    };
}

fn calculate_gross_profit(base_amount: f64, buying_amount: f64, precision: u32) -> f64 {
    round_to(
        if buying_amount < 0.0 {
            base_amount + buying_amount.abs()
        } else {
            base_amount - buying_amount
        },
        precision,
    )
}

fn calculate_gross_profit_percent(gross_profit: f64, base_amount: f64, precision: u32) -> f64 {
    if base_amount == 0.0 {
        0.0
    } else {
        round_to((gross_profit / base_amount.abs()) * 100.0, precision)
    }
}

fn row_text_value(row: &GrossProfitCalculatedRow, col: &str) -> Option<String> {
    match col {
        "invoice_or_item" => Some(row.invoice_or_item.clone()),
        "customer" => Some(row.customer.clone()),
        "customer_group" => Some(row.customer_group.clone()),
        "customer_name" => Some(row.customer_name.clone()),
        "posting_date" => Some(row.posting_date.clone()),
        "item_code" => Some(row.item_code.clone()),
        "item_name" => Some(row.item_name.clone()),
        "item_group" => Some(row.item_group.clone()),
        "brand" => Some(row.brand.clone()),
        "description" => Some(row.description.clone()),
        "warehouse" => Some(row.warehouse.clone()),
        "project" => Some(row.project.clone()),
        "cost_center" => Some(row.cost_center.clone()),
        "territory" => Some(row.territory.clone()),
        "sales_person" => Some(row.sales_person.clone()),
        "payment_term" => Some(row.payment_term.clone()),
        _ => None,
    }
}

fn row_cell_value(row: &GrossProfitCalculatedRow, col: &str) -> ReportCell {
    match col {
        "qty" => ReportCell::Number(row.qty),
        "base_rate" => ReportCell::Number(row.base_rate),
        "buying_rate" => ReportCell::Number(row.buying_rate),
        "base_amount" => ReportCell::Number(row.base_amount),
        "buying_amount" => ReportCell::Number(row.buying_amount),
        "gross_profit" => ReportCell::Number(row.gross_profit),
        "gross_profit_percent" => ReportCell::Number(row.gross_profit_percent),
        "allocated_amount" => ReportCell::Number(row.allocated_amount),
        _ => row_text_value(row, col)
            .map(ReportCell::Text)
            .unwrap_or(ReportCell::Empty),
    }
}

fn hides_customer_name(master_settings: &MasterNameSettings) -> bool {
    master_settings.supplier_master_name == "Supplier Name"
        && master_settings.customer_master_name == "Customer Name"
}

fn scrub(value: &str) -> String {
    value.trim().to_lowercase().replace(' ', "_")
}

fn round_to(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
