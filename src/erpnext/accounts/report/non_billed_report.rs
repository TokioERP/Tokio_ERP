use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonBilledArgs {
    pub doctype: String,
    pub party: String,
    pub date: String,
    pub reference_field: String,
    pub order: String,
    pub order_by: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonBilledFilters {
    pub company: String,
    pub posting_date: String,
    pub references: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonBilledDocument {
    pub name: String,
    pub date_fields: BTreeMap<String, String>,
    pub party: String,
    pub party_name: String,
    pub status: String,
    pub docstatus: i32,
    pub company: String,
    pub posting_date: String,
    pub conversion_rate: Option<f64>,
    pub project: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonBilledItem {
    pub parent: String,
    pub item_code: String,
    pub base_amount: f64,
    pub billed_amt: f64,
    pub base_rate: f64,
    pub returned_qty: Option<f64>,
    pub amount: f64,
    pub item_name: String,
    pub description: String,
    pub project: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonBilledItemMaster {
    pub name: String,
    pub is_stock_item: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonBilledRow {
    pub name: String,
    pub date: String,
    pub party: String,
    pub party_name: String,
    pub item_code: String,
    pub amount: f64,
    pub billed_amount: f64,
    pub returned_amount: f64,
    pub pending_amount: f64,
    pub item_name: String,
    pub description: String,
    pub project: Option<String>,
    pub company: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonBilledQueryPlan {
    pub doctype: String,
    pub child_tab: String,
    pub party_field: String,
    pub party_name_field: String,
    pub date_field: String,
    pub reference_field: String,
    pub docname_filter: Option<String>,
    pub project_source: &'static str,
    pub order_field: String,
    pub order_by: String,
    pub precision: u32,
    pub joins: Vec<String>,
    pub filters: Vec<String>,
}

impl NonBilledQueryPlan {
    pub fn from_args(
        args: &NonBilledArgs,
        filters: &NonBilledFilters,
        precision: Option<u32>,
    ) -> Self {
        let child_tab = format!("{} Item", args.doctype);
        let docname_filter = filters.references.get(&args.reference_field).cloned();
        let mut query_filters = vec![
            "docstatus = 1".to_string(),
            "status not in Closed,Completed".to_string(),
            "company = filters.company".to_string(),
            "posting_date <= filters.posting_date".to_string(),
            "child.amount > 0".to_string(),
            "item.is_stock_item = 1".to_string(),
            "base_amount - Round(billed_amt * IfNull(conversion_rate, 1), precision) - base_rate * IfNull(returned_qty, 0) > 0".to_string(),
        ];

        if docname_filter.is_some() {
            query_filters.push("name = filters[reference_field]".to_string());
        }

        Self {
            doctype: args.doctype.clone(),
            child_tab: child_tab.clone(),
            party_field: args.party.clone(),
            party_name_field: format!("{}_name", args.party),
            date_field: args.date.clone(),
            reference_field: args.reference_field.clone(),
            docname_filter,
            project_source: get_project_field(&args.party),
            order_field: args.order.clone(),
            order_by: args.order_by.clone(),
            precision: precision.unwrap_or(2),
            joins: vec![
                format!("{}.name = {}.parent", args.doctype, child_tab),
                format!("Item.name = {}.item_code", child_tab),
            ],
            filters: query_filters,
        }
    }
}

pub fn get_project_field(party: &str) -> &'static str {
    if party == "supplier" {
        "child_doctype.project"
    } else {
        "doctype.project"
    }
}

pub fn get_ordered_to_be_billed_data(
    args: &NonBilledArgs,
    filters: &NonBilledFilters,
    documents: &[NonBilledDocument],
    children: &[NonBilledItem],
    item_masters: &[NonBilledItemMaster],
    precision: Option<u32>,
) -> Vec<NonBilledRow> {
    let plan = NonBilledQueryPlan::from_args(args, filters, precision);
    let item_stock_map: HashMap<&str, bool> = item_masters
        .iter()
        .map(|item| (item.name.as_str(), item.is_stock_item))
        .collect();

    let mut rows = Vec::new();

    for doc in documents {
        if !document_matches_filters(doc, filters, plan.docname_filter.as_deref()) {
            continue;
        }

        for child in children.iter().filter(|child| child.parent == doc.name) {
            if !child_matches_filters(child, doc, &item_stock_map, plan.precision) {
                continue;
            }

            let billed_amount = child.billed_amt * doc.conversion_rate.unwrap_or(1.0);
            let returned_amount = child.base_rate * child.returned_qty.unwrap_or(0.0);
            let pending_amount = child.base_amount - billed_amount - returned_amount;

            rows.push(NonBilledRow {
                name: doc.name.clone(),
                date: doc.date_fields.get(&args.date).cloned().unwrap_or_default(),
                party: doc.party.clone(),
                party_name: doc.party_name.clone(),
                item_code: child.item_code.clone(),
                amount: child.base_amount,
                billed_amount,
                returned_amount,
                pending_amount,
                item_name: child.item_name.clone(),
                description: child.description.clone(),
                project: if args.party == "supplier" {
                    child.project.clone()
                } else {
                    doc.project.clone()
                },
                company: doc.company.clone(),
            });
        }
    }

    sort_rows_like_orderby(&mut rows, args);
    rows
}

fn document_matches_filters(
    doc: &NonBilledDocument,
    filters: &NonBilledFilters,
    docname_filter: Option<&str>,
) -> bool {
    doc.docstatus == 1
        && doc.status != "Closed"
        && doc.status != "Completed"
        && doc.company == filters.company
        && doc.posting_date <= filters.posting_date
        && docname_filter.map_or(true, |docname| doc.name == docname)
}

fn child_matches_filters(
    child: &NonBilledItem,
    doc: &NonBilledDocument,
    item_stock_map: &HashMap<&str, bool>,
    precision: u32,
) -> bool {
    if child.amount <= 0.0
        || !item_stock_map
            .get(child.item_code.as_str())
            .copied()
            .unwrap_or(false)
    {
        return false;
    }

    let rounded_billed_amount = round_to(
        child.billed_amt * doc.conversion_rate.unwrap_or(1.0),
        precision,
    );
    let returned_amount = child.base_rate * child.returned_qty.unwrap_or(0.0);

    child.base_amount - rounded_billed_amount - returned_amount > 0.0
}

fn sort_rows_like_orderby(rows: &mut [NonBilledRow], args: &NonBilledArgs) {
    rows.sort_by(|left, right| {
        let left_key = row_order_value(left, &args.order);
        let right_key = row_order_value(right, &args.order);
        left_key
            .cmp(&right_key)
            .then_with(|| left.name.cmp(&right.name))
    });

    if args.order_by.eq_ignore_ascii_case("desc") {
        rows.reverse();
    }
}

fn row_order_value(row: &NonBilledRow, field: &str) -> String {
    match field {
        "name" => row.name.clone(),
        "company" => row.company.clone(),
        _ => row.date.clone(),
    }
}

fn round_to(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
