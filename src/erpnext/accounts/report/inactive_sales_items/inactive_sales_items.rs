use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InactiveSalesItemsFilters {
    pub based_on: String,
    pub days: i32,
    pub territory: Option<String>,
    pub item_group: Option<String>,
    pub item: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SalesDetailsQueryPlan {
    pub doctype: &'static str,
    pub child_doctype: &'static str,
    pub date_field: &'static str,
    pub join_condition: &'static str,
    pub docstatus_filter: &'static str,
    pub order_by: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Territory {
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InactiveSalesItem {
    pub item_code: String,
    pub item_group: String,
    pub item_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesDetail {
    pub territory: String,
    pub customer: String,
    pub item_code: String,
    pub qty: f64,
    pub last_order_date: String,
    pub days_since_last_order: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InactiveSalesItemsRow {
    pub territory: String,
    pub item_group: String,
    pub item: String,
    pub item_name: String,
    pub customer: Option<String>,
    pub last_order_date: Option<String>,
    pub qty: Option<f64>,
    pub days_since_last_order: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InactiveSalesItemsReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<InactiveSalesItemsRow>,
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
        }
    }

    pub const fn data(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Data",
            options: "",
            width,
        }
    }

    pub const fn date(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Date",
            options: "",
            width,
        }
    }

    pub const fn float(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Float",
            options: "",
            width,
        }
    }

    pub const fn int(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Int",
            options: "",
            width,
        }
    }
}

impl Territory {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl InactiveSalesItem {
    pub fn new(
        item_code: impl Into<String>,
        item_group: impl Into<String>,
        item_name: impl Into<String>,
    ) -> Self {
        Self {
            item_code: item_code.into(),
            item_group: item_group.into(),
            item_name: item_name.into(),
        }
    }
}

impl SalesDetail {
    pub fn new(
        territory: impl Into<String>,
        customer: impl Into<String>,
        item_code: impl Into<String>,
        qty: f64,
        last_order_date: impl Into<String>,
        days_since_last_order: i32,
    ) -> Self {
        Self {
            territory: territory.into(),
            customer: customer.into(),
            item_code: item_code.into(),
            qty,
            last_order_date: last_order_date.into(),
            days_since_last_order,
        }
    }
}

pub fn execute(
    filters: &InactiveSalesItemsFilters,
    territories: &[Territory],
    items: &[InactiveSalesItem],
    sales_details: &[SalesDetail],
) -> InactiveSalesItemsReport {
    InactiveSalesItemsReport {
        columns: get_columns(),
        rows: get_data(filters, territories, items, sales_details),
    }
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Territory", "territory", "Territory", 100),
        ReportColumn::link("Item Group", "item_group", "Item Group", 150),
        ReportColumn::link("Item", "item", "Item", 150),
        ReportColumn::data("Item Name", "item_name", 150),
        ReportColumn::link("Customer", "customer", "Customer", 100),
        ReportColumn::date("Last Order Date", "last_order_date", 100),
        ReportColumn::float("Quantity", "qty", 100),
        ReportColumn::int("Days Since Last Order", "days_since_last_order", 100),
    ]
}

pub fn get_data(
    filters: &InactiveSalesItemsFilters,
    territories: &[Territory],
    items: &[InactiveSalesItem],
    sales_details: &[SalesDetail],
) -> Vec<InactiveSalesItemsRow> {
    let sales_detail_map = get_sales_details_map(sales_details);
    let mut rows = Vec::new();

    for territory in territories {
        for item in items {
            let mut row = InactiveSalesItemsRow {
                territory: territory.name.clone(),
                item_group: item.item_group.clone(),
                item: item.item_code.clone(),
                item_name: item.item_name.clone(),
                customer: None,
                last_order_date: None,
                qty: None,
                days_since_last_order: None,
            };

            if let Some(item_obj) =
                sales_detail_map.get(&(territory.name.as_str(), item.item_code.as_str()))
            {
                if item_obj.days_since_last_order > filters.days {
                    row.territory = item_obj.territory.clone();
                    row.customer = Some(item_obj.customer.clone());
                    row.last_order_date = Some(item_obj.last_order_date.clone());
                    row.qty = Some(item_obj.qty);
                    row.days_since_last_order = Some(item_obj.days_since_last_order);
                } else {
                    continue;
                }
            }

            rows.push(row);
        }
    }

    rows
}

pub fn get_sales_details_map(sales_details: &[SalesDetail]) -> HashMap<(&str, &str), &SalesDetail> {
    let mut item_details_map = HashMap::new();
    for detail in sales_details {
        item_details_map
            .entry((detail.territory.as_str(), detail.item_code.as_str()))
            .or_insert(detail);
    }
    item_details_map
}

pub fn get_sales_details_query_plan(filters: &InactiveSalesItemsFilters) -> SalesDetailsQueryPlan {
    let date_field = if filters.based_on == "Sales Order" {
        "s.transaction_date"
    } else {
        "s.posting_date"
    };
    let child_doctype = if filters.based_on == "Sales Order" {
        "Sales Order Item"
    } else {
        "Sales Invoice Item"
    };

    SalesDetailsQueryPlan {
        doctype: if filters.based_on == "Sales Order" {
            "Sales Order"
        } else {
            "Sales Invoice"
        },
        child_doctype,
        date_field,
        join_condition: "s.name = si.parent",
        docstatus_filter: "s.docstatus = 1",
        order_by: "days_since_last_order",
    }
}

pub fn get_territories_query_plan(
    filters: &InactiveSalesItemsFilters,
) -> Vec<(&'static str, &'static str, String)> {
    filters
        .territory
        .as_ref()
        .map(|territory| vec![("name", "=", territory.clone())])
        .unwrap_or_default()
}

pub fn get_items_query_plan(
    filters: &InactiveSalesItemsFilters,
) -> Vec<(&'static str, &'static str, String)> {
    let mut query_filters = vec![
        ("disabled", "=", "0".to_string()),
        ("is_stock_item", "=", "1".to_string()),
    ];

    if let Some(item_group) = filters.item_group.as_ref() {
        query_filters.push(("item_group", "=", item_group.clone()));
    }

    if let Some(item) = filters.item.as_ref() {
        query_filters.push(("name", "=", item.clone()));
    }

    query_filters.push(("order_by", "=", "name".to_string()));
    query_filters
}
