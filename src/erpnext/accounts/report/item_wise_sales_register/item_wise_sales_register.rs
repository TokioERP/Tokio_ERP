use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemWiseSalesFilters {
    pub company: Option<String>,
    pub customer: Option<String>,
    pub customer_group: Option<String>,
    pub mode_of_payment: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub warehouse: Option<String>,
    pub brand: Option<String>,
    pub item_code: Option<String>,
    pub item_group: Option<String>,
    pub income_account: Option<String>,
    pub group_by: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: String,
    pub options: Option<String>,
    pub width: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdditionalColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: String,
    pub width: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomerDetail {
    pub customer_name: String,
    pub customer_group: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesInvoiceItemRow {
    pub name: String,
    pub parent: String,
    pub posting_date: String,
    pub debit_to: String,
    pub unrealized_profit_loss_account: Option<String>,
    pub is_internal_customer: bool,
    pub customer: String,
    pub territory: String,
    pub company: String,
    pub item_code: String,
    pub description: String,
    pub si_item_name: Option<String>,
    pub si_item_group: Option<String>,
    pub i_item_name: Option<String>,
    pub i_item_group: Option<String>,
    pub project: Option<String>,
    pub sales_order: Option<String>,
    pub delivery_note: Option<String>,
    pub so_detail: Option<String>,
    pub income_account: Option<String>,
    pub cost_center: Option<String>,
    pub enable_deferred_revenue: bool,
    pub deferred_revenue_account: Option<String>,
    pub stock_qty: f64,
    pub stock_uom: String,
    pub base_net_rate: f64,
    pub base_net_amount: f64,
    pub customer_name: String,
    pub customer_group: String,
    pub update_stock: bool,
    pub uom: String,
    pub qty: f64,
    pub warehouse: Option<String>,
    pub brand: Option<String>,
    pub docstatus: i32,
    pub parenttype: String,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemTaxDetail {
    pub item_row: String,
    pub account_head: String,
    pub tax_amount: f64,
    pub tax_rate: f64,
    pub is_other_charges: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemWiseSalesInput {
    pub company_currency: String,
    pub items: Vec<SalesInvoiceItemRow>,
    pub item_taxes: Vec<ItemTaxDetail>,
    pub customer_details: BTreeMap<String, CustomerDetail>,
    pub mode_of_payments: BTreeMap<String, Vec<String>>,
    pub delivery_notes_by_so_detail: BTreeMap<String, Vec<String>>,
    pub invoice_base_grand_totals: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemWiseSalesRow {
    pub values: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemWiseSalesReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<ItemWiseSalesRow>,
    pub skip_total_row: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct SubtotalRow {
    values: BTreeMap<String, Value>,
    stock_qty: f64,
    amount: f64,
    total_tax: f64,
    total: f64,
    percent_gt: f64,
    tax_amounts: BTreeMap<String, f64>,
}

impl ReportColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data".to_string(),
            options: None,
            width,
        }
    }

    pub fn typed(label: &str, fieldname: &str, fieldtype: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: fieldtype.to_string(),
            options: None,
            width,
        }
    }

    pub fn link(label: &str, fieldname: &str, options: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Link".to_string(),
            options: Some(options.to_string()),
            width,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Currency".to_string(),
            options: Some("currency".to_string()),
            width,
        }
    }
}

impl AdditionalColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data".to_string(),
            width,
        }
    }
}

impl From<&AdditionalColumn> for ReportColumn {
    fn from(column: &AdditionalColumn) -> Self {
        Self {
            label: column.label.clone(),
            fieldname: column.fieldname.clone(),
            fieldtype: column.fieldtype.clone(),
            options: None,
            width: column.width,
        }
    }
}

impl ItemTaxDetail {
    pub fn new(
        item_row: &str,
        account_head: &str,
        tax_amount: f64,
        tax_rate: f64,
        is_other_charges: bool,
    ) -> Self {
        Self {
            item_row: item_row.to_string(),
            account_head: account_head.to_string(),
            tax_amount,
            tax_rate,
            is_other_charges,
        }
    }
}

pub fn execute(
    filters: ItemWiseSalesFilters,
    input: ItemWiseSalesInput,
    additional_table_columns: Vec<AdditionalColumn>,
) -> Result<ItemWiseSalesReport, String> {
    let mut item_list = get_items(&filters, &input);
    let mut tax_columns = tax_columns(&item_list, &input.item_taxes);
    tax_columns.sort();
    let columns = get_columns(&additional_table_columns, &filters, &tax_columns);
    let itemised_tax = get_tax_accounts(&item_list, &input.item_taxes);
    let _so_dn_map =
        get_delivery_notes_against_sales_order(&item_list, &input.delivery_notes_by_so_detail);

    let grand_total = if filters.group_by.is_some() {
        grand_total(&item_list, &input)
    } else {
        0.0
    };

    let mut rows = Vec::new();
    let mut total_row_map = BTreeMap::<String, SubtotalRow>::new();
    let mut prev_group_by_value = String::new();

    for d in &mut item_list {
        let row = build_row(
            d,
            &additional_table_columns,
            &tax_columns,
            &itemised_tax,
            &input,
        );

        if let Some(group_by) = filters.group_by.as_deref() {
            let group_by_field = group_by_field(group_by);
            let group_value = string_for_group(d, group_by_field);
            if prev_group_by_value != group_value {
                if !prev_group_by_value.is_empty() {
                    if let Some(total_row) = total_row_map.get(&prev_group_by_value).cloned() {
                        rows.push(ItemWiseSalesRow {
                            values: total_row.to_values(&tax_columns),
                        });
                        rows.push(ItemWiseSalesRow {
                            values: BTreeMap::new(),
                        });
                        add_sub_total_row(
                            &total_row,
                            &mut total_row_map,
                            "total_row",
                            &tax_columns,
                        );
                    }
                }
                prev_group_by_value = group_value.clone();
                total_row_map
                    .entry(group_value.clone())
                    .or_insert_with(|| subtotal_row(group_by, group_by_field, d));
                total_row_map
                    .entry("total_row".to_string())
                    .or_insert_with(|| grand_total_row(group_by));
            }
            let total = row.values["total"].as_f64().unwrap_or(0.0);
            let percent_gt = if grand_total != 0.0 {
                total / grand_total * 100.0
            } else {
                0.0
            };
            let mut row = row;
            row.values
                .insert("percent_gt".to_string(), json!(percent_gt));
            add_row_to_subtotal(&row, &mut total_row_map, &group_value, &tax_columns);
            rows.push(row);
        } else {
            rows.push(row);
        }
    }

    let mut skip_total_row = false;
    if filters.group_by.is_some() && !item_list.is_empty() {
        if let Some(total_row) = total_row_map.get(&prev_group_by_value).cloned() {
            rows.push(ItemWiseSalesRow {
                values: total_row.to_values(&tax_columns),
            });
            rows.push(ItemWiseSalesRow {
                values: BTreeMap::new(),
            });
            add_sub_total_row(&total_row, &mut total_row_map, "total_row", &tax_columns);
        }
        if let Some(grand_total) = total_row_map.get("total_row") {
            rows.push(ItemWiseSalesRow {
                values: grand_total.to_values(&tax_columns),
            });
        }
        skip_total_row = true;
    }

    Ok(ItemWiseSalesReport {
        columns,
        rows,
        skip_total_row,
    })
}

pub fn get_columns(
    additional_table_columns: &[AdditionalColumn],
    filters: &ItemWiseSalesFilters,
    tax_columns: &[String],
) -> Vec<ReportColumn> {
    let mut columns = Vec::new();

    if filters.group_by.as_deref() != Some("Item") {
        columns.push(ReportColumn::link("Item Code", "item_code", "Item", 120));
        columns.push(ReportColumn::data("Item Name", "item_name", 120));
    }

    if !matches!(
        filters.group_by.as_deref(),
        Some("Item") | Some("Item Group")
    ) {
        columns.push(ReportColumn::link(
            "Item Group",
            "item_group",
            "Item Group",
            120,
        ));
    }

    columns.extend([
        ReportColumn::data("Description", "description", 150),
        ReportColumn::link("Invoice", "invoice", "Sales Invoice", 150),
        ReportColumn::typed("Posting Date", "posting_date", "Date", 120),
    ]);

    if filters.group_by.as_deref() != Some("Customer") {
        columns.push(ReportColumn::link(
            "Customer Group",
            "customer_group",
            "Customer Group",
            120,
        ));
    }

    if !matches!(
        filters.group_by.as_deref(),
        Some("Customer") | Some("Customer Group")
    ) {
        columns.push(ReportColumn::link("Customer", "customer", "Customer", 120));
        columns.push(ReportColumn::data("Customer Name", "customer_name", 120));
    }

    columns.extend(additional_table_columns.iter().map(ReportColumn::from));

    columns.push(ReportColumn::link(
        "Receivable Account",
        "debit_to",
        "Account",
        80,
    ));
    columns.push(ReportColumn::data(
        "Mode Of Payment",
        "mode_of_payment",
        120,
    ));

    if filters.group_by.as_deref() != Some("Territory") {
        columns.push(ReportColumn::link(
            "Territory",
            "territory",
            "Territory",
            80,
        ));
    }

    columns.extend([
        ReportColumn::link("Project", "project", "Project", 80),
        ReportColumn::link("Company", "company", "Company", 80),
        ReportColumn::link("Sales Order", "sales_order", "Sales Order", 100),
        ReportColumn::link("Delivery Note", "delivery_note", "Delivery Note", 100),
        ReportColumn::link("Income Account", "income_account", "Account", 100),
        ReportColumn::link("Cost Center", "cost_center", "Cost Center", 100),
        ReportColumn::typed("Stock Qty", "stock_qty", "Float", 100),
        ReportColumn::link("Stock UOM", "stock_uom", "UOM", 100),
        ReportColumn::typed("Rate", "rate", "Float", 100),
        ReportColumn::currency("Amount", "amount", 100),
    ]);

    if filters.group_by.is_some() {
        columns.push(ReportColumn::typed(
            "% Of Grand Total",
            "percent_gt",
            "Float",
            80,
        ));
    }

    for tax in tax_columns {
        let title = title_from_scrub(tax);
        columns.push(ReportColumn::typed(
            &format!("{title} Rate"),
            &format!("{tax}_rate"),
            "Float",
            100,
        ));
        columns.push(ReportColumn::currency(
            &format!("{title} Amount"),
            &format!("{tax}_amount"),
            100,
        ));
    }

    columns.extend([
        ReportColumn::currency("Total Tax", "total_tax", 100),
        ReportColumn::currency("Total Other Charges", "total_other_charges", 100),
        ReportColumn::currency("Total", "total", 100),
        ReportColumn::currency("Currency", "currency", 80),
    ]);

    columns
}

pub fn get_items(
    filters: &ItemWiseSalesFilters,
    input: &ItemWiseSalesInput,
) -> Vec<SalesInvoiceItemRow> {
    let mut rows: Vec<SalesInvoiceItemRow> = input
        .items
        .iter()
        .filter(|row| row.docstatus == 1 && row.parenttype == "Sales Invoice")
        .filter(|row| {
            filters
                .company
                .as_ref()
                .map(|company| &row.company == company)
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .customer
                .as_ref()
                .map(|customer| &row.customer == customer)
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .customer_group
                .as_ref()
                .map(|customer_group| &row.customer_group == customer_group)
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .from_date
                .as_ref()
                .map(|from_date| &row.posting_date >= from_date)
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .to_date
                .as_ref()
                .map(|to_date| &row.posting_date <= to_date)
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .mode_of_payment
                .as_ref()
                .map(|mode| {
                    input
                        .mode_of_payments
                        .get(&row.parent)
                        .map(|payments| payments.iter().any(|payment| payment == mode))
                        .unwrap_or(false)
                })
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .warehouse
                .as_ref()
                .map(|warehouse| row.warehouse.as_ref() == Some(warehouse))
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .brand
                .as_ref()
                .map(|brand| row.brand.as_ref() == Some(brand))
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .item_code
                .as_ref()
                .map(|item_code| &row.item_code == item_code)
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .item_group
                .as_ref()
                .map(|item_group| {
                    row.si_item_group.as_ref() == Some(item_group)
                        || row.i_item_group.as_ref() == Some(item_group)
                })
                .unwrap_or(true)
        })
        .filter(|row| {
            filters
                .income_account
                .as_ref()
                .map(|income_account| {
                    row.income_account.as_ref() == Some(income_account)
                        || row.deferred_revenue_account.as_ref() == Some(income_account)
                        || row.unrealized_profit_loss_account.as_ref() == Some(income_account)
                })
                .unwrap_or(true)
        })
        .cloned()
        .collect();

    apply_order_by_conditions(&mut rows, filters);
    rows
}

pub fn get_delivery_notes_against_sales_order(
    item_list: &[SalesInvoiceItemRow],
    delivery_notes_by_so_detail: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>> {
    let so_item_rows: BTreeSet<String> = item_list
        .iter()
        .filter_map(|row| row.so_detail.clone())
        .collect();

    so_item_rows
        .into_iter()
        .filter_map(|so_detail| {
            delivery_notes_by_so_detail
                .get(&so_detail)
                .map(|delivery_notes| (so_detail, delivery_notes.clone()))
        })
        .collect()
}

fn build_row(
    d: &SalesInvoiceItemRow,
    additional_table_columns: &[AdditionalColumn],
    tax_columns: &[String],
    itemised_tax: &BTreeMap<String, BTreeMap<String, ItemTaxDetail>>,
    input: &ItemWiseSalesInput,
) -> ItemWiseSalesRow {
    let customer_detail = input.customer_details.get(&d.customer);
    let mut values = BTreeMap::from([
        ("item_code".to_string(), json!(d.item_code)),
        (
            "item_name".to_string(),
            json!(d.si_item_name.as_ref().or(d.i_item_name.as_ref())),
        ),
        (
            "item_group".to_string(),
            json!(d.si_item_group.as_ref().or(d.i_item_group.as_ref())),
        ),
        ("description".to_string(), json!(d.description)),
        ("invoice".to_string(), json!(d.parent)),
        ("posting_date".to_string(), json!(d.posting_date)),
        ("customer".to_string(), json!(d.customer)),
        (
            "customer_name".to_string(),
            json!(customer_detail
                .map(|detail| detail.customer_name.as_str())
                .unwrap_or(&d.customer_name)),
        ),
        (
            "customer_group".to_string(),
            json!(customer_detail
                .map(|detail| detail.customer_group.as_str())
                .unwrap_or(&d.customer_group)),
        ),
        ("debit_to".to_string(), json!(d.debit_to)),
        (
            "mode_of_payment".to_string(),
            json!(input
                .mode_of_payments
                .get(&d.parent)
                .map(|payments| payments.join(", "))
                .unwrap_or_default()),
        ),
        ("territory".to_string(), json!(d.territory)),
        ("project".to_string(), json!(d.project)),
        ("company".to_string(), json!(d.company)),
        ("sales_order".to_string(), json!(d.sales_order)),
        ("delivery_note".to_string(), json!(d.delivery_note)),
        ("income_account".to_string(), json!(get_income_account(d))),
        ("cost_center".to_string(), json!(d.cost_center)),
        ("stock_qty".to_string(), json!(d.stock_qty)),
        ("stock_uom".to_string(), json!(d.stock_uom)),
    ]);

    for column in additional_table_columns {
        values.insert(
            column.fieldname.clone(),
            d.extra
                .get(&column.fieldname)
                .cloned()
                .unwrap_or(Value::Null),
        );
    }

    let rate = if d.stock_uom != d.uom && d.stock_qty != 0.0 {
        (d.base_net_rate * d.qty) / d.stock_qty
    } else {
        d.base_net_rate
    };
    values.insert("rate".to_string(), json!(rate));
    values.insert("amount".to_string(), json!(d.base_net_amount));

    for tax in tax_columns {
        values.insert(format!("{tax}_rate"), json!(0.0));
        values.insert(format!("{tax}_amount"), json!(0.0));
    }

    let mut total_tax = 0.0;
    let mut total_other_charges = 0.0;
    if let Some(taxes) = itemised_tax.get(&d.name) {
        for (tax, details) in taxes {
            values.insert(format!("{tax}_rate"), json!(details.tax_rate));
            values.insert(format!("{tax}_amount"), json!(details.tax_amount));
            if details.is_other_charges {
                total_other_charges += details.tax_amount;
            } else {
                total_tax += details.tax_amount;
            }
        }
    }

    values.insert("total_tax".to_string(), json!(total_tax));
    values.insert(
        "total_other_charges".to_string(),
        json!(total_other_charges),
    );
    values.insert(
        "total".to_string(),
        json!(d.base_net_amount + total_tax + total_other_charges),
    );
    values.insert("currency".to_string(), json!(input.company_currency));

    ItemWiseSalesRow { values }
}

fn get_income_account(row: &SalesInvoiceItemRow) -> Option<String> {
    if row.enable_deferred_revenue {
        row.deferred_revenue_account.clone()
    } else if row.is_internal_customer {
        row.unrealized_profit_loss_account.clone()
    } else {
        row.income_account.clone()
    }
}

fn get_tax_accounts(
    item_list: &[SalesInvoiceItemRow],
    item_taxes: &[ItemTaxDetail],
) -> BTreeMap<String, BTreeMap<String, ItemTaxDetail>> {
    let item_rows: BTreeSet<&str> = item_list.iter().map(|row| row.name.as_str()).collect();
    let mut taxes = BTreeMap::<String, BTreeMap<String, ItemTaxDetail>>::new();

    for tax in item_taxes
        .iter()
        .filter(|tax| item_rows.contains(tax.item_row.as_str()))
    {
        let tax_name = scrub(&tax.account_head);
        let entry = taxes
            .entry(tax.item_row.clone())
            .or_default()
            .entry(tax_name)
            .or_insert_with(|| ItemTaxDetail {
                item_row: tax.item_row.clone(),
                account_head: tax.account_head.clone(),
                tax_amount: 0.0,
                tax_rate: if tax.tax_rate == 0.0 {
                    0.0
                } else {
                    tax.tax_rate
                },
                is_other_charges: tax.is_other_charges,
            });
        entry.tax_amount += tax.tax_amount;
    }

    taxes
}

fn tax_columns(item_list: &[SalesInvoiceItemRow], item_taxes: &[ItemTaxDetail]) -> Vec<String> {
    let item_rows: BTreeSet<&str> = item_list.iter().map(|row| row.name.as_str()).collect();
    item_taxes
        .iter()
        .filter(|tax| item_rows.contains(tax.item_row.as_str()))
        .filter(|tax| tax.tax_amount != 0.0)
        .map(|tax| scrub(&tax.account_head))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn apply_order_by_conditions(rows: &mut [SalesInvoiceItemRow], filters: &ItemWiseSalesFilters) {
    match filters.group_by.as_deref() {
        None => rows.sort_by(|left, right| {
            right
                .posting_date
                .cmp(&left.posting_date)
                .then_with(|| right.si_item_group.cmp(&left.si_item_group))
        }),
        Some("Invoice") => rows.sort_by(|left, right| right.parent.cmp(&left.parent)),
        Some("Item") => rows.sort_by(|left, right| left.item_code.cmp(&right.item_code)),
        Some("Item Group") => {
            rows.sort_by(|left, right| left.si_item_group.cmp(&right.si_item_group))
        }
        Some("Customer") => rows.sort_by(|left, right| right.customer.cmp(&left.customer)),
        Some("Customer Group") => {
            rows.sort_by(|left, right| right.customer_group.cmp(&left.customer_group))
        }
        Some("Territory") => rows.sort_by(|left, right| right.territory.cmp(&left.territory)),
        Some("Supplier") => rows.sort_by(|left, right| right.customer.cmp(&left.customer)),
        Some(_) => {}
    }
}

fn grand_total(item_list: &[SalesInvoiceItemRow], input: &ItemWiseSalesInput) -> f64 {
    if input.invoice_base_grand_totals.is_empty() {
        item_list
            .iter()
            .map(|row| {
                let taxes = input
                    .item_taxes
                    .iter()
                    .filter(|tax| tax.item_row == row.name)
                    .map(|tax| tax.tax_amount)
                    .sum::<f64>();
                row.base_net_amount + taxes
            })
            .sum()
    } else {
        input.invoice_base_grand_totals.values().sum()
    }
}

fn group_by_field(group_by: &str) -> &str {
    match group_by {
        "Item" => "item_code",
        "Invoice" => "parent",
        _ => scrub_static(group_by),
    }
}

fn string_for_group(row: &SalesInvoiceItemRow, field: &str) -> String {
    match field {
        "item_code" => row.item_code.clone(),
        "item_group" => row.si_item_group.clone().unwrap_or_default(),
        "parent" => row.parent.clone(),
        "customer" => row.customer.clone(),
        "customer_group" => row.customer_group.clone(),
        "territory" => row.territory.clone(),
        _ => String::new(),
    }
}

fn subtotal_row(group_by: &str, group_by_field: &str, item: &SalesInvoiceItemRow) -> SubtotalRow {
    let display_field = subtotal_display_field(group_by);
    let mut values = BTreeMap::new();
    values.insert(
        display_field.to_string(),
        json!(get_display_value(group_by, group_by_field, item)),
    );
    values.insert("bold".to_string(), json!(1));

    SubtotalRow {
        values,
        stock_qty: 0.0,
        amount: 0.0,
        total_tax: 0.0,
        total: 0.0,
        percent_gt: 0.0,
        tax_amounts: BTreeMap::new(),
    }
}

fn grand_total_row(group_by: &str) -> SubtotalRow {
    let mut values = BTreeMap::new();
    values.insert(subtotal_display_field(group_by).to_string(), json!("Total"));
    values.insert("bold".to_string(), json!(1));

    SubtotalRow {
        values,
        stock_qty: 0.0,
        amount: 0.0,
        total_tax: 0.0,
        total: 0.0,
        percent_gt: 0.0,
        tax_amounts: BTreeMap::new(),
    }
}

fn get_display_value(group_by: &str, group_by_field: &str, item: &SalesInvoiceItemRow) -> String {
    if group_by == "Item" {
        let item_name = item
            .si_item_name
            .as_ref()
            .or(item.i_item_name.as_ref())
            .cloned()
            .unwrap_or_default();
        if item.item_code != item_name {
            format!("{}: {}", item.item_code, item_name)
        } else {
            item.item_code.clone()
        }
    } else if group_by == "Customer" {
        if item.customer != item.customer_name {
            format!("{}: {}", item.customer, item.customer_name)
        } else {
            item.customer.clone()
        }
    } else {
        string_for_group(item, group_by_field)
    }
}

fn subtotal_display_field(group_by: &str) -> &str {
    if group_by == "Item" {
        "invoice"
    } else if group_by == "Invoice" {
        "item_code"
    } else {
        "item_code"
    }
}

fn add_row_to_subtotal(
    row: &ItemWiseSalesRow,
    total_row_map: &mut BTreeMap<String, SubtotalRow>,
    group_by_value: &str,
    tax_columns: &[String],
) {
    add_sub_total_row_values(row, total_row_map, group_by_value, tax_columns);
}

fn add_sub_total_row(
    subtotal: &SubtotalRow,
    total_row_map: &mut BTreeMap<String, SubtotalRow>,
    group_by_value: &str,
    tax_columns: &[String],
) {
    let row = ItemWiseSalesRow {
        values: subtotal.to_values(tax_columns),
    };
    add_sub_total_row_values(&row, total_row_map, group_by_value, tax_columns);
}

fn add_sub_total_row_values(
    row: &ItemWiseSalesRow,
    total_row_map: &mut BTreeMap<String, SubtotalRow>,
    group_by_value: &str,
    tax_columns: &[String],
) {
    let total_row = total_row_map
        .get_mut(group_by_value)
        .expect("subtotal row should exist before accumulation");
    total_row.stock_qty += number_value(&row.values, "stock_qty");
    total_row.amount += number_value(&row.values, "amount");
    total_row.total_tax += number_value(&row.values, "total_tax");
    total_row.total += number_value(&row.values, "total");
    total_row.percent_gt += number_value(&row.values, "percent_gt");

    for tax in tax_columns {
        *total_row.tax_amounts.entry(tax.clone()).or_default() +=
            number_value(&row.values, &format!("{tax}_amount"));
    }
}

impl SubtotalRow {
    fn to_values(&self, tax_columns: &[String]) -> BTreeMap<String, Value> {
        let mut values = self.values.clone();
        values.insert("stock_qty".to_string(), json!(self.stock_qty));
        values.insert("amount".to_string(), json!(self.amount));
        values.insert("total_tax".to_string(), json!(self.total_tax));
        values.insert("total".to_string(), json!(self.total));
        values.insert("percent_gt".to_string(), json!(self.percent_gt));

        for tax in tax_columns {
            values.insert(
                format!("{tax}_amount"),
                json!(self.tax_amounts.get(tax).copied().unwrap_or_default()),
            );
        }

        values
    }
}

fn number_value(values: &BTreeMap<String, Value>, key: &str) -> f64 {
    values.get(key).and_then(Value::as_f64).unwrap_or_default()
}

fn scrub(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn scrub_static(value: &str) -> &str {
    match value {
        "Item Group" => "item_group",
        "Customer Group" => "customer_group",
        _ => {
            if value == "Customer" {
                "customer"
            } else if value == "Territory" {
                "territory"
            } else {
                ""
            }
        }
    }
}

fn title_from_scrub(value: &str) -> String {
    value
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
