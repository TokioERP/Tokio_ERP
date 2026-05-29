use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemWisePurchaseFilters {
    pub company: Option<String>,
    pub supplier: Option<String>,
    pub mode_of_payment: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub item_code: Option<String>,
    pub item_group: Option<String>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseInvoiceItemRow {
    pub name: String,
    pub parent: String,
    pub posting_date: String,
    pub credit_to: String,
    pub company: String,
    pub supplier: String,
    pub supplier_name: String,
    pub unrealized_profit_loss_account: Option<String>,
    pub item_code: String,
    pub description: String,
    pub pi_item_name: Option<String>,
    pub pi_item_group: Option<String>,
    pub i_item_name: Option<String>,
    pub i_item_group: Option<String>,
    pub project: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_receipt: Option<String>,
    pub po_detail: Option<String>,
    pub expense_account: Option<String>,
    pub stock_qty: f64,
    pub stock_uom: String,
    pub base_net_amount: f64,
    pub mode_of_payment: Option<String>,
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
pub struct ItemWisePurchaseInput {
    pub company_currency: String,
    pub items: Vec<PurchaseInvoiceItemRow>,
    pub item_taxes: Vec<ItemTaxDetail>,
    pub company_stock_received_but_not_billed: BTreeMap<String, String>,
    pub purchase_receipts_by_po_detail: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemWisePurchaseRow {
    pub values: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ItemWisePurchaseReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<ItemWisePurchaseRow>,
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
    filters: ItemWisePurchaseFilters,
    input: ItemWisePurchaseInput,
    additional_table_columns: Vec<AdditionalColumn>,
) -> Result<ItemWisePurchaseReport, String> {
    let mut item_list = get_items(&filters, &input);
    let mut tax_columns = tax_columns(&item_list, &input.item_taxes);
    tax_columns.sort();
    let columns = get_columns(&additional_table_columns, &filters, &tax_columns);
    let itemised_tax = get_tax_accounts(&item_list, &input.item_taxes);
    let po_pr_map = get_purchase_receipts_against_purchase_order(
        &item_list,
        &input.purchase_receipts_by_po_detail,
    );

    let grand_total = if filters.group_by.is_some() {
        item_list.iter().map(|row| row.base_net_amount).sum::<f64>()
            + input
                .item_taxes
                .iter()
                .filter(|tax| item_list.iter().any(|item| item.name == tax.item_row))
                .filter(|tax| !tax.is_other_charges)
                .map(|tax| tax.tax_amount)
                .sum::<f64>()
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
            &po_pr_map,
            &input,
        );

        if let Some(group_by) = filters.group_by.as_deref() {
            let group_by_field = group_by_field(group_by);
            let group_value = string_for_group(d, group_by_field);
            if prev_group_by_value != group_value {
                if !prev_group_by_value.is_empty() {
                    if let Some(total_row) = total_row_map.get(&prev_group_by_value).cloned() {
                        rows.push(ItemWisePurchaseRow {
                            values: total_row.to_values(&tax_columns),
                        });
                        rows.push(ItemWisePurchaseRow {
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
            rows.push(ItemWisePurchaseRow {
                values: total_row.to_values(&tax_columns),
            });
            rows.push(ItemWisePurchaseRow {
                values: BTreeMap::new(),
            });
            add_sub_total_row(&total_row, &mut total_row_map, "total_row", &tax_columns);
        }
        if let Some(grand_total) = total_row_map.get("total_row") {
            rows.push(ItemWisePurchaseRow {
                values: grand_total.to_values(&tax_columns),
            });
        }
        skip_total_row = true;
    }

    Ok(ItemWisePurchaseReport {
        columns,
        rows,
        skip_total_row,
    })
}

pub fn get_columns(
    additional_table_columns: &[AdditionalColumn],
    filters: &ItemWisePurchaseFilters,
    tax_columns: &[String],
) -> Vec<ReportColumn> {
    let mut columns = Vec::new();
    if filters.group_by.as_deref() != Some("Item") {
        columns.push(ReportColumn::link("Item Code", "item_code", "Item", 120));
        columns.push(ReportColumn::data("Item Name", "item_name", 120));
    }
    if !matches!(filters.group_by.as_deref(), Some("Item" | "Item Group")) {
        columns.push(ReportColumn::link(
            "Item Group",
            "item_group",
            "Item Group",
            120,
        ));
    }

    columns.extend(vec![
        ReportColumn::data("Description", "description", 150),
        ReportColumn::link("Invoice", "invoice", "Purchase Invoice", 150),
        ReportColumn::typed("Posting Date", "posting_date", "Date", 120),
    ]);

    if filters.group_by.as_deref() != Some("Supplier") {
        columns.push(ReportColumn::link("Supplier", "supplier", "Supplier", 120));
        columns.push(ReportColumn::data("Supplier Name", "supplier_name", 120));
    }

    columns.extend(additional_table_columns.iter().map(ReportColumn::from));
    columns.extend(vec![
        ReportColumn::link("Payable Account", "credit_to", "Account", 80),
        ReportColumn::link("Mode Of Payment", "mode_of_payment", "Mode of Payment", 120),
        ReportColumn::link("Project", "project", "Project", 80),
        ReportColumn::link("Company", "company", "Company", 80),
        ReportColumn::link("Purchase Order", "purchase_order", "Purchase Order", 100),
        ReportColumn::link(
            "Purchase Receipt",
            "purchase_receipt",
            "Purchase Receipt",
            100,
        ),
        ReportColumn::link("Expense Account", "expense_account", "Account", 100),
        ReportColumn::typed("Stock Qty", "stock_qty", "Float", 100),
        ReportColumn::link("Stock UOM", "stock_uom", "UOM", 100),
        ReportColumn::typed("Rate", "rate", "Float", 100),
        ReportColumn::currency("Amount", "amount", 100),
    ]);

    for tax in tax_columns {
        columns.push(ReportColumn::typed(
            &format!("{} Rate", title_from_scrub(tax)),
            &format!("{tax}_rate"),
            "Float",
            100,
        ));
        columns.push(ReportColumn::currency(
            &format!("{} Amount", title_from_scrub(tax)),
            &format!("{tax}_amount"),
            100,
        ));
    }

    columns.extend(vec![
        ReportColumn::currency("Total Tax", "total_tax", 100),
        ReportColumn::currency("Total", "total", 100),
        ReportColumn::typed("Currency", "currency", "Currency", 80),
    ]);

    if filters.group_by.is_some() {
        columns.push(ReportColumn::typed(
            "% Of Grand Total",
            "percent_gt",
            "Float",
            80,
        ));
    }

    columns
}

pub fn get_items(
    filters: &ItemWisePurchaseFilters,
    input: &ItemWisePurchaseInput,
) -> Vec<PurchaseInvoiceItemRow> {
    let mut rows = input
        .items
        .iter()
        .filter(|row| row.docstatus == 1 && row.parenttype == "Purchase Invoice")
        .filter(|row| {
            filters
                .company
                .as_ref()
                .is_none_or(|company| row.company == *company)
        })
        .filter(|row| {
            filters
                .supplier
                .as_ref()
                .is_none_or(|supplier| row.supplier == *supplier)
        })
        .filter(|row| {
            filters
                .mode_of_payment
                .as_ref()
                .is_none_or(|mode| row.mode_of_payment.as_ref() == Some(mode))
        })
        .filter(|row| {
            filters
                .from_date
                .as_ref()
                .is_none_or(|from_date| row.posting_date >= *from_date)
        })
        .filter(|row| {
            filters
                .to_date
                .as_ref()
                .is_none_or(|to_date| row.posting_date <= *to_date)
        })
        .filter(|row| {
            filters
                .item_code
                .as_ref()
                .is_none_or(|item| row.item_code == *item)
        })
        .filter(|row| {
            filters.item_group.as_ref().is_none_or(|group| {
                row.pi_item_group.as_deref() == Some(group.as_str())
                    || row.i_item_group.as_deref() == Some(group.as_str())
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    apply_order_by_conditions(&mut rows, filters.group_by.as_deref());
    rows
}

pub fn get_purchase_receipts_against_purchase_order(
    item_list: &[PurchaseInvoiceItemRow],
    purchase_receipts_by_po_detail: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>> {
    item_list
        .iter()
        .filter_map(|row| row.po_detail.as_deref())
        .filter_map(|po_detail| {
            purchase_receipts_by_po_detail
                .get(po_detail)
                .map(|receipts| (po_detail.to_string(), receipts.clone()))
        })
        .collect()
}

fn build_row(
    d: &PurchaseInvoiceItemRow,
    additional_table_columns: &[AdditionalColumn],
    tax_columns: &[String],
    itemised_tax: &BTreeMap<String, BTreeMap<String, ItemTaxDetail>>,
    po_pr_map: &BTreeMap<String, Vec<String>>,
    input: &ItemWisePurchaseInput,
) -> ItemWisePurchaseRow {
    let purchase_receipt = d
        .purchase_receipt
        .clone()
        .or_else(|| {
            d.po_detail
                .as_ref()
                .and_then(|detail| po_pr_map.get(detail).map(|receipts| receipts.join(", ")))
        })
        .unwrap_or_default();
    let expense_account = d
        .unrealized_profit_loss_account
        .clone()
        .or_else(|| d.expense_account.clone())
        .or_else(|| {
            input
                .company_stock_received_but_not_billed
                .get(&d.company)
                .cloned()
        })
        .unwrap_or_default();

    let mut values = BTreeMap::new();
    values.insert("item_code".to_string(), json!(d.item_code));
    values.insert(
        "item_name".to_string(),
        json!(d
            .pi_item_name
            .clone()
            .or_else(|| d.i_item_name.clone())
            .unwrap_or_default()),
    );
    values.insert(
        "item_group".to_string(),
        json!(d
            .pi_item_group
            .clone()
            .or_else(|| d.i_item_group.clone())
            .unwrap_or_default()),
    );
    values.insert("description".to_string(), json!(d.description));
    values.insert("invoice".to_string(), json!(d.parent));
    values.insert("posting_date".to_string(), json!(d.posting_date));
    values.insert("supplier".to_string(), json!(d.supplier));
    values.insert("supplier_name".to_string(), json!(d.supplier_name));
    for column in additional_table_columns {
        values.insert(
            column.fieldname.clone(),
            d.extra
                .get(&column.fieldname)
                .cloned()
                .unwrap_or(Value::Null),
        );
    }
    values.insert("credit_to".to_string(), json!(d.credit_to));
    values.insert(
        "mode_of_payment".to_string(),
        json!(d.mode_of_payment.clone().unwrap_or_default()),
    );
    values.insert(
        "project".to_string(),
        json!(d.project.clone().unwrap_or_default()),
    );
    values.insert("company".to_string(), json!(d.company));
    values.insert(
        "purchase_order".to_string(),
        json!(d.purchase_order.clone().unwrap_or_default()),
    );
    values.insert("purchase_receipt".to_string(), json!(purchase_receipt));
    values.insert("expense_account".to_string(), json!(expense_account));
    values.insert("stock_qty".to_string(), json!(d.stock_qty));
    values.insert("stock_uom".to_string(), json!(d.stock_uom));
    values.insert(
        "rate".to_string(),
        json!(if d.stock_qty != 0.0 {
            d.base_net_amount / d.stock_qty
        } else {
            d.base_net_amount
        }),
    );
    values.insert("amount".to_string(), json!(d.base_net_amount));

    for tax in tax_columns {
        values.insert(format!("{tax}_rate"), json!(0.0));
        values.insert(format!("{tax}_amount"), json!(0.0));
    }

    let mut total_tax = 0.0;
    if let Some(taxes) = itemised_tax.get(&d.name) {
        for (tax, detail) in taxes {
            values.insert(format!("{tax}_rate"), json!(detail.tax_rate));
            values.insert(format!("{tax}_amount"), json!(detail.tax_amount));
            if !detail.is_other_charges {
                total_tax += detail.tax_amount;
            }
        }
    }

    values.insert("total_tax".to_string(), json!(total_tax));
    values.insert("total".to_string(), json!(d.base_net_amount + total_tax));
    values.insert("currency".to_string(), json!(input.company_currency));
    ItemWisePurchaseRow { values }
}

fn get_tax_accounts(
    item_list: &[PurchaseInvoiceItemRow],
    item_taxes: &[ItemTaxDetail],
) -> BTreeMap<String, BTreeMap<String, ItemTaxDetail>> {
    let item_names = item_list
        .iter()
        .map(|item| item.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut map = BTreeMap::<String, BTreeMap<String, ItemTaxDetail>>::new();
    for tax in item_taxes
        .iter()
        .filter(|tax| item_names.contains(tax.item_row.as_str()))
    {
        map.entry(tax.item_row.clone())
            .or_default()
            .insert(scrub(&tax.account_head), tax.clone());
    }
    map
}

fn tax_columns(item_list: &[PurchaseInvoiceItemRow], item_taxes: &[ItemTaxDetail]) -> Vec<String> {
    let item_names = item_list
        .iter()
        .map(|item| item.name.as_str())
        .collect::<BTreeSet<_>>();
    item_taxes
        .iter()
        .filter(|tax| item_names.contains(tax.item_row.as_str()))
        .filter(|tax| tax.tax_amount != 0.0)
        .map(|tax| scrub(&tax.account_head))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn apply_order_by_conditions(rows: &mut [PurchaseInvoiceItemRow], group_by: Option<&str>) {
    match group_by {
        None => rows.sort_by(|left, right| {
            right
                .posting_date
                .cmp(&left.posting_date)
                .then_with(|| right.pi_item_group.cmp(&left.pi_item_group))
        }),
        Some("Invoice") => rows.sort_by(|left, right| right.parent.cmp(&left.parent)),
        Some("Item") => rows.sort_by(|left, right| left.item_code.cmp(&right.item_code)),
        Some("Item Group") => {
            rows.sort_by(|left, right| left.pi_item_group.cmp(&right.pi_item_group))
        }
        Some("Supplier") => rows.sort_by(|left, right| right.supplier.cmp(&left.supplier)),
        _ => {}
    }
}

fn group_by_field(group_by: &str) -> &str {
    match group_by {
        "Item" => "item_code",
        "Invoice" => "parent",
        "Supplier" => "supplier",
        "Item Group" => "item_group",
        _ => "item_code",
    }
}

fn string_for_group(row: &PurchaseInvoiceItemRow, field: &str) -> String {
    match field {
        "item_code" => row.item_code.clone(),
        "parent" => row.parent.clone(),
        "supplier" => row.supplier.clone(),
        "item_group" => row
            .pi_item_group
            .clone()
            .or_else(|| row.i_item_group.clone())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn subtotal_row(group_by: &str, group_by_field: &str, row: &PurchaseInvoiceItemRow) -> SubtotalRow {
    let display = get_display_value(group_by, group_by_field, row);
    let display_field = subtotal_display_field(group_by);
    let mut values = BTreeMap::new();
    values.insert(display_field.to_string(), json!(display));
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

fn get_display_value(group_by: &str, group_by_field: &str, row: &PurchaseInvoiceItemRow) -> String {
    if group_by == "Item" {
        let item_name = row
            .pi_item_name
            .clone()
            .or_else(|| row.i_item_name.clone())
            .unwrap_or_default();
        if row.item_code != item_name {
            format!("{}: {}", row.item_code, item_name)
        } else {
            row.item_code.clone()
        }
    } else if group_by == "Supplier" {
        if row.supplier != row.supplier_name {
            format!("{}: {}", row.supplier, row.supplier_name)
        } else {
            row.supplier.clone()
        }
    } else {
        string_for_group(row, group_by_field)
    }
}

fn subtotal_display_field(group_by: &str) -> &str {
    match group_by {
        "Item" | "Invoice" | "Supplier" | "Item Group" => "item_code",
        _ => "item_code",
    }
}

fn add_row_to_subtotal(
    row: &ItemWisePurchaseRow,
    total_row_map: &mut BTreeMap<String, SubtotalRow>,
    group_by_value: &str,
    tax_columns: &[String],
) {
    if let Some(total_row) = total_row_map.get_mut(group_by_value) {
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
}

fn add_sub_total_row(
    subtotal: &SubtotalRow,
    total_row_map: &mut BTreeMap<String, SubtotalRow>,
    group_by_value: &str,
    tax_columns: &[String],
) {
    if let Some(total_row) = total_row_map.get_mut(group_by_value) {
        total_row.stock_qty += subtotal.stock_qty;
        total_row.amount += subtotal.amount;
        total_row.total_tax += subtotal.total_tax;
        total_row.total += subtotal.total;
        total_row.percent_gt += subtotal.percent_gt;
        for tax in tax_columns {
            *total_row.tax_amounts.entry(tax.clone()).or_default() +=
                subtotal.tax_amounts.get(tax).copied().unwrap_or(0.0);
        }
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
                json!(self.tax_amounts.get(tax).copied().unwrap_or(0.0)),
            );
        }
        values
    }
}

fn number_value(values: &BTreeMap<String, Value>, fieldname: &str) -> f64 {
    values.get(fieldname).and_then(Value::as_f64).unwrap_or(0.0)
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
