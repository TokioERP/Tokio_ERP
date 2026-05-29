use std::collections::BTreeMap;

pub type ReportRow = BTreeMap<String, ReportCell>;

#[derive(Clone, Debug, PartialEq)]
pub struct GrossProfitReport {
    pub columns: Vec<ReportColumn>,
    pub data: GrossProfitReportData,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GrossProfitReportData {
    Invoice(Vec<ReportRow>),
    Grouped(Vec<Vec<ReportCell>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrossProfitFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub group_by: String,
    pub currency: String,
    pub currency_precision: u32,
    pub float_precision: u32,
    pub include_returned_invoices: bool,
    pub item_group: Option<String>,
    pub sales_person: Option<String>,
    pub sales_invoice: Option<String>,
    pub item_code: Option<String>,
    pub cost_center: Vec<String>,
    pub project: Vec<String>,
    pub warehouse: Option<String>,
    pub accounting_dimensions: Vec<AccountingDimensionFilter>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MasterNameSettings {
    pub supplier_master_name: String,
    pub customer_master_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingDimensionFilter {
    pub fieldname: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryPlan {
    pub source: &'static str,
    pub joins: Vec<&'static str>,
    pub left_joins: Vec<&'static str>,
    pub selects: Vec<&'static str>,
    pub conditions: Vec<String>,
    pub group_by: Vec<&'static str>,
    pub order_by: Vec<&'static str>,
    pub limit: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceItemLoadQueryPlans {
    pub normal: QueryPlan,
    pub returns: QueryPlan,
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

#[derive(Clone, Debug, PartialEq)]
pub struct GrossProfitInvoiceRow {
    pub parent_invoice: String,
    pub parenttype: String,
    pub indent: f64,
    pub parent: Option<String>,
    pub invoice_or_item: String,
    pub posting_date: String,
    pub posting_time: String,
    pub project: String,
    pub update_stock: bool,
    pub customer: String,
    pub customer_group: String,
    pub customer_name: String,
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub description: Option<String>,
    pub warehouse: Option<String>,
    pub item_group: Option<String>,
    pub brand: Option<String>,
    pub dn_detail: Option<String>,
    pub delivery_note: Option<String>,
    pub qty: Option<f64>,
    pub item_row: Option<String>,
    pub is_return: bool,
    pub cost_center: String,
    pub base_net_amount: f64,
    pub invoice_base_net_total: f64,
    pub invoice: Option<String>,
    pub serial_and_batch_bundle: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductBundleItem {
    pub item_code: String,
    pub item_name: String,
    pub description: String,
    pub warehouse: Option<String>,
    pub total_qty: f64,
    pub parent_detail_docname: String,
    pub serial_and_batch_bundle: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductBundleLoadRow {
    pub parenttype: String,
    pub parent: String,
    pub parent_item: String,
    pub item: ProductBundleItem,
}

pub type ProductBundles =
    BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<ProductBundleItem>>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnAdjustedRow {
    pub parent: String,
    pub item_code: String,
    pub qty: f64,
    pub base_amount: f64,
    pub buying_rate: f64,
    pub buying_amount: f64,
    pub delivered_by_supplier: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnedInvoiceItem {
    pub return_against: String,
    pub item_code: String,
    pub qty: f64,
    pub base_amount: f64,
}

pub type ReturnedInvoices = BTreeMap<String, BTreeMap<String, Vec<ReturnedInvoiceItem>>>;

#[derive(Clone, Debug, PartialEq)]
pub struct StockLedgerEntry {
    pub voucher_type: String,
    pub voucher_no: String,
    pub voucher_detail_no: String,
    pub stock_value: f64,
    pub qty: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StockLedgerCache {
    pub entries: BTreeMap<(String, String), Vec<StockLedgerEntry>>,
    pub requests: Vec<QueryPlan>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeliveryNoteSummary {
    pub total_qty: f64,
    pub total_incoming_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeliveryNoteLoadRow {
    pub si_detail: String,
    pub total_qty: f64,
    pub total_incoming_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrossProfitBuyingAmountRow {
    pub item_code: String,
    pub qty: f64,
    pub delivered_by_supplier: bool,
    pub so_detail: Option<String>,
    pub sales_order: Option<String>,
    pub project: String,
    pub cost_center: String,
    pub update_stock: bool,
    pub dn_detail: Option<String>,
    pub parenttype: String,
    pub parent: String,
    pub invoice: Option<String>,
    pub delivery_note: Option<String>,
    pub item_row: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrossProfitBuyingAmountContext {
    pub po_details: Vec<String>,
    pub delivered_purchase_amount: Option<f64>,
    pub non_stock_items: Vec<String>,
    pub last_purchase_rate: Option<f64>,
    pub stock_ledger_entries: Vec<StockLedgerEntry>,
    pub delivery_note: Option<DeliveryNoteSummary>,
    pub so_dn_incoming_rate: Option<f64>,
    pub average_buying_rate: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncomingRateRequest {
    pub item_code: String,
    pub warehouse: String,
    pub parenttype: String,
    pub parent: String,
    pub company: String,
    pub serial_and_batch_bundle: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IncomingRateCache {
    pub rates: BTreeMap<(String, String), f64>,
    pub requests: Vec<IncomingRateRequest>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackedItemOverride {
    pub parent_invoice: String,
    pub item_code: String,
    pub parent_detail_docname: String,
    pub warehouse: String,
    pub base_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrossProfitProcessRow {
    pub parent_invoice: String,
    pub parenttype: String,
    pub indent: f64,
    pub parent: Option<String>,
    pub invoice_or_item: String,
    pub project: String,
    pub customer: String,
    pub customer_group: String,
    pub customer_name: String,
    pub posting_date: String,
    pub monthly: String,
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub item_group: Option<String>,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub warehouse: Option<String>,
    pub qty: Option<f64>,
    pub item_row: Option<String>,
    pub update_stock: bool,
    pub dn_detail: Option<String>,
    pub delivery_note: Option<String>,
    pub delivered_by_supplier: bool,
    pub base_net_amount: f64,
    pub base_amount: f64,
    pub buying_amount: f64,
    pub buying_rate: Option<f64>,
    pub base_rate: Option<f64>,
    pub gross_profit: f64,
    pub gross_profit_percent: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GrossProfitCalculatedRow {
    pub invoice_or_item: String,
    pub customer: String,
    pub customer_group: String,
    pub customer_name: String,
    pub posting_date: String,
    pub monthly: String,
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

pub fn prepare_invoice_query_plan(filters: &GrossProfitFilters) -> QueryPlan {
    let mut plan = base_invoice_query_plan(filters);

    if filters.include_returned_invoices {
        plan.conditions.push(
            "(sales_invoice.is_return = 0 or (sales_invoice.is_return = 1 and sales_invoice.return_against is null))"
                .to_string(),
        );
    } else {
        plan.conditions
            .push("sales_invoice.is_return = 0".to_string());
    }

    plan
}

pub fn prepare_return_invoice_query_plan(
    filters: &GrossProfitFilters,
    vouchers_to_ignore: &[String],
) -> QueryPlan {
    let mut plan = base_invoice_query_plan(filters);

    plan.conditions.push(
        "(sales_invoice.is_return = 1 and sales_invoice.return_against is not null)".to_string(),
    );
    if !vouchers_to_ignore.is_empty() {
        plan.conditions.push(format!(
            "sales_invoice.return_against not in [{}]",
            vouchers_to_ignore.join(", ")
        ));
    }

    plan
}

pub fn load_invoice_items_query_plans(
    filters: &GrossProfitFilters,
    normal_rows: &[GrossProfitInvoiceRow],
) -> InvoiceItemLoadQueryPlans {
    let normal = prepare_invoice_query_plan(filters);
    let vouchers_to_ignore = prepare_vouchers_to_ignore(normal_rows);
    let returns = prepare_return_invoice_query_plan(filters, &vouchers_to_ignore);

    InvoiceItemLoadQueryPlans { normal, returns }
}

pub fn get_returned_invoice_items_query_plan(filters: &GrossProfitFilters) -> QueryPlan {
    QueryPlan {
        source: "Sales Invoice",
        joins: vec!["Sales Invoice Item"],
        selects: vec![
            "sales_invoice.name",
            "sales_invoice_item.item_code",
            "sales_invoice_item.stock_qty as qty",
            "sales_invoice_item.base_net_amount as base_amount",
            "sales_invoice.return_against",
        ],
        conditions: vec![
            "sales_invoice.name = sales_invoice_item.parent".to_string(),
            "sales_invoice.docstatus = 1".to_string(),
            "sales_invoice.is_return = 1".to_string(),
            format!(
                "sales_invoice.posting_date between {} and {}",
                filters.from_date, filters.to_date
            ),
        ],
        ..QueryPlan::default()
    }
}

pub fn group_returned_invoice_items(rows: &[ReturnedInvoiceItem]) -> ReturnedInvoices {
    let mut returned_invoices = ReturnedInvoices::new();
    for row in rows {
        returned_invoices
            .entry(row.return_against.clone())
            .or_default()
            .entry(row.item_code.clone())
            .or_default()
            .push(row.clone());
    }
    returned_invoices
}

pub fn prepare_vouchers_to_ignore(rows: &[GrossProfitInvoiceRow]) -> Vec<String> {
    rows.iter().filter_map(|row| row.parent.clone()).collect()
}

pub fn get_delivery_notes_query_plan(invoices: &[String]) -> QueryPlan {
    QueryPlan {
        source: "Delivery Note Item",
        selects: vec![
            "delivery_note_item.si_detail",
            "sum(delivery_note_item.stock_qty * delivery_note_item.incoming_rate) as total_incoming_value",
            "sum(delivery_note_item.stock_qty) as total_qty",
        ],
        conditions: vec![
            "delivery_note_item.docstatus = 1".to_string(),
            format!(
                "delivery_note_item.against_sales_invoice in [{}]",
                invoices.join(", ")
            ),
            "delivery_note_item.si_detail is not null".to_string(),
            "delivery_note_item.si_detail != ".to_string(),
        ],
        group_by: vec!["delivery_note_item.si_detail"],
        ..QueryPlan::default()
    }
}

pub fn group_delivery_notes(rows: &[DeliveryNoteLoadRow]) -> BTreeMap<String, DeliveryNoteSummary> {
    rows.iter()
        .map(|row| {
            (
                row.si_detail.clone(),
                DeliveryNoteSummary {
                    total_qty: row.total_qty,
                    total_incoming_value: row.total_incoming_value,
                },
            )
        })
        .collect()
}

pub fn get_product_bundle_query_plan() -> QueryPlan {
    QueryPlan {
        source: "Packed Item",
        selects: vec![
            "packed_item.parenttype",
            "packed_item.parent",
            "packed_item.parent_item",
            "packed_item.item_code",
            "packed_item.warehouse",
            "-1 * packed_item.qty as total_qty",
            "packed_item.rate",
            "packed_item.rate * packed_item.qty as base_amount",
            "packed_item.parent_detail_docname",
            "packed_item.serial_and_batch_bundle",
        ],
        conditions: vec!["packed_item.docstatus = 1".to_string()],
        ..QueryPlan::default()
    }
}

pub fn load_non_stock_items_query_plan() -> QueryPlan {
    QueryPlan {
        source: "Item",
        selects: vec!["item.name"],
        conditions: vec!["item.is_stock_item = 0".to_string()],
        ..QueryPlan::default()
    }
}

pub fn group_product_bundles(rows: &[ProductBundleLoadRow]) -> ProductBundles {
    let mut product_bundles: ProductBundles = BTreeMap::new();

    for row in rows {
        product_bundles
            .entry(row.parenttype.clone())
            .or_default()
            .entry(row.parent.clone())
            .or_default()
            .entry(row.parent_item.clone())
            .or_default()
            .push(row.item.clone());
    }

    product_bundles
}

pub fn get_stock_ledger_query_plan(item_code: &str, warehouse: &str, company: &str) -> QueryPlan {
    QueryPlan {
        source: "Stock Ledger Entry",
        selects: vec![
            "stock_ledger_entry.item_code",
            "stock_ledger_entry.voucher_type",
            "stock_ledger_entry.voucher_no",
            "stock_ledger_entry.voucher_detail_no",
            "stock_ledger_entry.stock_value",
            "stock_ledger_entry.warehouse",
            "stock_ledger_entry.actual_qty as qty",
        ],
        conditions: vec![
            format!("stock_ledger_entry.company = {company}"),
            format!("stock_ledger_entry.item_code = {item_code}"),
            format!("stock_ledger_entry.warehouse = {warehouse}"),
            "stock_ledger_entry.is_cancelled = 0".to_string(),
        ],
        order_by: vec![
            "stock_ledger_entry.item_code",
            "stock_ledger_entry.warehouse desc",
            "stock_ledger_entry.posting_datetime desc",
            "stock_ledger_entry.creation desc",
        ],
        ..QueryPlan::default()
    }
}

impl StockLedgerCache {
    pub fn get_entries(
        &mut self,
        item_code: &str,
        warehouse: &str,
        company: &str,
        loaded_entries: &[StockLedgerEntry],
    ) -> Vec<StockLedgerEntry> {
        if item_code.is_empty() || warehouse.is_empty() {
            return Vec::new();
        }

        let key = (item_code.to_string(), warehouse.to_string());
        if let Some(entries) = self.entries.get(&key) {
            return entries.clone();
        }

        self.requests
            .push(get_stock_ledger_query_plan(item_code, warehouse, company));
        self.entries.insert(key.clone(), loaded_entries.to_vec());
        self.entries.get(&key).cloned().unwrap_or_default()
    }
}

pub fn get_last_purchase_rate_query_plan(
    item_code: &str,
    project: Option<&str>,
    cost_center: Option<&str>,
    to_date: &str,
) -> QueryPlan {
    let mut plan = QueryPlan {
        source: "Purchase Invoice Item",
        joins: vec!["Purchase Invoice"],
        selects: vec!["purchase_invoice_item.base_rate / purchase_invoice_item.conversion_factor"],
        conditions: vec![
            "purchase_invoice.docstatus = 1".to_string(),
            format!("purchase_invoice.posting_date <= {to_date}"),
            format!("purchase_invoice_item.item_code = {item_code}"),
            "purchase_invoice.is_return = 0".to_string(),
            "purchase_invoice_item.parenttype = Purchase Invoice".to_string(),
        ],
        order_by: vec!["purchase_invoice.posting_date desc"],
        limit: Some(1),
        ..QueryPlan::default()
    };

    if let Some(project) = project {
        plan.conditions
            .push(format!("purchase_invoice_item.project = {project}"));
    }
    if let Some(cost_center) = cost_center {
        plan.conditions
            .push(format!("purchase_invoice_item.cost_center = {cost_center}"));
    }

    plan
}

pub fn prepare_delivered_by_supplier_purchase_query_plan(po_details: &[String]) -> QueryPlan {
    QueryPlan {
        source: "Purchase Invoice Item",
        selects: vec!["sum(purchase_invoice_item.qty * purchase_invoice_item.base_net_rate)"],
        conditions: vec![
            format!(
                "purchase_invoice_item.po_detail in [{}]",
                po_details.join(", ")
            ),
            "purchase_invoice_item.docstatus = 1".to_string(),
        ],
        ..QueryPlan::default()
    }
}

pub fn get_buying_amount_from_so_dn_query_plan(
    sales_order: &str,
    so_detail: &str,
    item_code: &str,
) -> QueryPlan {
    QueryPlan {
        source: "Delivery Note Item",
        selects: vec!["avg(delivery_note_item.incoming_rate)"],
        conditions: vec![
            "delivery_note_item.docstatus = 1".to_string(),
            format!("delivery_note_item.item_code = {item_code}"),
            format!("delivery_note_item.against_sales_order = {sales_order}"),
            format!("delivery_note_item.so_detail = {so_detail}"),
        ],
        group_by: vec!["delivery_note_item.item_code"],
        ..QueryPlan::default()
    }
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

fn base_invoice_query_plan(filters: &GrossProfitFilters) -> QueryPlan {
    let mut plan = QueryPlan {
        source: "Sales Invoice",
        joins: vec!["Sales Invoice Item", "Item"],
        selects: invoice_query_selects(),
        conditions: vec![
            "sales_invoice.docstatus = 1".to_string(),
            "sales_invoice.is_opening != Yes".to_string(),
        ],
        order_by: vec![
            "sales_invoice.posting_date desc",
            "sales_invoice.posting_time desc",
        ],
        ..QueryPlan::default()
    };

    apply_common_filter_plan(&mut plan, filters);

    match filters.group_by.as_str() {
        "Sales Person" => {
            plan.selects.extend([
                "sales_team.sales_person",
                "sales_team.allocated_percentage * sales_invoice_item.base_net_amount / 100 as allocated_amount",
                "sales_team.incentives",
            ]);
            plan.left_joins.push("Sales Team");
        }
        "Payment Term" => {
            plan.selects.extend([
                "case when sales_invoice.is_return = 1 then Sales Return else coalesce(payment_schedule.payment_term, No Terms) end as payment_term",
                "payment_schedule.invoice_portion",
                "payment_schedule.payment_amount",
            ]);
            plan.left_joins.push("Payment Schedule");
        }
        _ => {}
    }

    plan
}

fn invoice_query_selects() -> Vec<&'static str> {
    vec![
        "sales_invoice_item.parenttype",
        "sales_invoice_item.parent",
        "sales_invoice.posting_date",
        "sales_invoice.posting_time",
        "sales_invoice.project",
        "sales_invoice.update_stock",
        "sales_invoice.customer",
        "sales_invoice.customer_group",
        "sales_invoice.customer_name",
        "sales_invoice.territory",
        "sales_invoice_item.item_code",
        "sales_invoice.base_net_total as invoice_base_net_total",
        "sales_invoice_item.item_name",
        "sales_invoice_item.description",
        "sales_invoice_item.warehouse",
        "sales_invoice_item.item_group",
        "sales_invoice_item.brand",
        "sales_invoice_item.so_detail",
        "sales_invoice_item.sales_order",
        "sales_invoice_item.dn_detail",
        "sales_invoice_item.delivery_note",
        "sales_invoice_item.stock_qty as qty",
        "sales_invoice_item.base_net_rate",
        "sales_invoice_item.base_net_amount",
        "sales_invoice_item.name as item_row",
        "sales_invoice.is_return",
        "sales_invoice_item.cost_center",
        "sales_invoice_item.serial_and_batch_bundle",
        "sales_invoice_item.delivered_by_supplier",
    ]
}

fn apply_common_filter_plan(plan: &mut QueryPlan, filters: &GrossProfitFilters) {
    if !filters.company.is_empty() {
        plan.conditions
            .push(format!("sales_invoice.company = {}", filters.company));
    }
    if !filters.from_date.is_empty() {
        plan.conditions.push(format!(
            "sales_invoice.posting_date >= {}",
            filters.from_date
        ));
    }
    if !filters.to_date.is_empty() {
        plan.conditions
            .push(format!("sales_invoice.posting_date <= {}", filters.to_date));
    }
    if let Some(item_group) = filters.item_group.as_deref() {
        plan.conditions
            .push(format!("item group condition for {item_group}"));
    }
    if let Some(sales_person) = filters.sales_person.as_deref() {
        plan.conditions.push(format!(
            "exists sales_team where parent = sales_invoice.name and sales_person = {sales_person}"
        ));
    }
    if let Some(sales_invoice) = filters.sales_invoice.as_deref() {
        plan.conditions
            .push(format!("sales_invoice.name = {sales_invoice}"));
    }
    if let Some(item_code) = filters.item_code.as_deref() {
        plan.conditions
            .push(format!("sales_invoice_item.item_code = {item_code}"));
    }
    if !filters.cost_center.is_empty() {
        plan.conditions.push(format!(
            "sales_invoice_item.cost_center in [{}]",
            filters.cost_center.join(", ")
        ));
    }
    if !filters.project.is_empty() {
        plan.conditions.push(format!(
            "sales_invoice_item.project in [{}]",
            filters.project.join(", ")
        ));
    }
    for dimension in &filters.accounting_dimensions {
        plan.conditions.push(format!(
            "sales_invoice_item.{} in [{}]",
            dimension.fieldname,
            dimension.values.join(", ")
        ));
    }
    if let Some(warehouse) = filters.warehouse.as_deref() {
        plan.conditions.push(format!(
            "sales_invoice_item.warehouse in warehouse descendants of {warehouse}"
        ));
    }
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

pub fn get_report_columns(
    filters: &GrossProfitFilters,
    master_settings: &MasterNameSettings,
) -> Vec<ReportColumn> {
    let columns = get_columns(filters, master_settings);
    if filters.group_by == "Invoice" {
        get_columns_for_grouped_by_invoice(&columns, master_settings)
    } else {
        columns
    }
}

pub fn gross_profit_report(
    filters: &GrossProfitFilters,
    master_settings: &MasterNameSettings,
    invoice_rows: &[GrossProfitProcessRow],
    grouped_rows: &[GrossProfitSourceRow],
) -> GrossProfitReport {
    let columns = get_report_columns(filters, master_settings);
    let data = if filters.group_by == "Invoice" {
        GrossProfitReportData::Invoice(get_data_when_grouped_by_invoice(invoice_rows, filters))
    } else {
        GrossProfitReportData::Grouped(get_data_when_not_grouped_by_invoice(
            grouped_rows,
            filters,
            master_settings,
        ))
    };

    GrossProfitReport { columns, data }
}

pub fn get_columns_for_grouped_by_invoice(
    columns: &[ReportColumn],
    master_settings: &MasterNameSettings,
) -> Vec<ReportColumn> {
    let mut columns = columns.to_vec();
    if let Some(first) = columns.first_mut() {
        first.fieldname = "sales_invoice";
        first.options = "Item";
        first.width = 300;
    }

    if hides_customer_name(master_settings) {
        if columns.len() >= 6 {
            columns.drain(4..6);
        }
    } else if columns.len() >= 7 {
        columns.drain(5..7);
    }

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
        monthly: if filters.group_by == "Monthly" {
            format_month_year(&row.posting_date)
        } else {
            String::new()
        },
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

pub fn get_invoice_row(row: &GrossProfitInvoiceRow) -> GrossProfitInvoiceRow {
    GrossProfitInvoiceRow {
        parent_invoice: String::new(),
        parenttype: row.parenttype.clone(),
        indent: 0.0,
        parent: None,
        invoice_or_item: row.parent.clone().unwrap_or_default(),
        posting_date: row.posting_date.clone(),
        posting_time: row.posting_time.clone(),
        project: row.project.clone(),
        update_stock: row.update_stock,
        customer: row.customer.clone(),
        customer_group: row.customer_group.clone(),
        customer_name: row.customer_name.clone(),
        item_code: None,
        item_name: None,
        description: None,
        warehouse: None,
        item_group: None,
        brand: None,
        dn_detail: None,
        delivery_note: None,
        qty: None,
        item_row: None,
        is_return: row.is_return,
        cost_center: row.cost_center.clone(),
        base_net_amount: row.invoice_base_net_total,
        invoice_base_net_total: row.invoice_base_net_total,
        invoice: None,
        serial_and_batch_bundle: None,
    }
}

pub fn get_bundle_item_row(
    row: &GrossProfitInvoiceRow,
    item: &ProductBundleItem,
) -> GrossProfitInvoiceRow {
    GrossProfitInvoiceRow {
        parent_invoice: row.item_code.clone().unwrap_or_default(),
        parenttype: row.parenttype.clone(),
        indent: row.indent + 1.0,
        parent: None,
        invoice_or_item: item.item_code.clone(),
        posting_date: row.posting_date.clone(),
        posting_time: row.posting_time.clone(),
        project: row.project.clone(),
        update_stock: row.update_stock,
        customer: row.customer.clone(),
        customer_group: row.customer_group.clone(),
        customer_name: row.customer_name.clone(),
        item_code: Some(item.item_code.clone()),
        item_name: Some(item.item_name.clone()),
        description: Some(item.description.clone()),
        warehouse: item.warehouse.clone().or_else(|| row.warehouse.clone()),
        item_group: Some(String::new()),
        brand: Some(String::new()),
        dn_detail: row.dn_detail.clone(),
        delivery_note: row.delivery_note.clone(),
        qty: Some(item.total_qty * -1.0),
        item_row: row.item_row.clone(),
        is_return: row.is_return,
        cost_center: row.cost_center.clone(),
        base_net_amount: 0.0,
        invoice_base_net_total: row.invoice_base_net_total,
        invoice: row.parent.clone(),
        serial_and_batch_bundle: row.serial_and_batch_bundle.clone(),
    }
}

pub fn group_items_by_invoice(
    rows: &[GrossProfitInvoiceRow],
    product_bundles: &[(&str, &str, Vec<ProductBundleItem>)],
) -> Vec<GrossProfitInvoiceRow> {
    let mut grouped: BTreeMap<String, Vec<GrossProfitInvoiceRow>> = BTreeMap::new();
    let mut invoice_order = Vec::new();

    for row in rows {
        let invoice = row.parent.clone().unwrap_or_default();
        if !grouped.contains_key(&invoice) {
            invoice_order.push(invoice.clone());
            grouped.insert(invoice.clone(), vec![get_invoice_row(row)]);
        }

        let mut item_row = row.clone();
        item_row.indent = 1.0;
        item_row.parent_invoice = invoice.clone();
        item_row.invoice_or_item = item_row.item_code.clone().unwrap_or_default();
        grouped
            .get_mut(&invoice)
            .expect("invoice group")
            .push(item_row.clone());

        for (_, _, bundle_items) in product_bundles.iter().filter(|(parent, item, _)| {
            *parent == invoice && Some(*item) == item_row.item_code.as_deref()
        }) {
            for bundle_item in bundle_items {
                grouped
                    .get_mut(&invoice)
                    .expect("invoice group")
                    .push(get_bundle_item_row(&item_row, bundle_item));
            }
        }
    }

    invoice_order
        .into_iter()
        .flat_map(|invoice| grouped.remove(&invoice).unwrap_or_default())
        .collect()
}

pub fn should_skip_row(row: &GrossProfitCalculatedRow, group_by: &str) -> bool {
    if group_by == "Invoice" {
        return false;
    }

    row_text_value(row, scrub(group_by).as_str()).is_some_and(|value| value.is_empty())
}

pub fn update_return_invoices(
    row: &mut ReturnAdjustedRow,
    returned_items: &mut [ReturnedInvoiceItem],
    currency_precision: u32,
) {
    for returned_item in returned_items.iter_mut().filter(|returned_item| {
        returned_item.return_against == row.parent && returned_item.item_code == row.item_code
    }) {
        if returned_item.qty != 0.0 {
            if row.qty >= returned_item.qty.abs() {
                row.qty += returned_item.qty;
                row.base_amount += round_to(returned_item.base_amount, currency_precision);
                returned_item.qty = 0.0;
                returned_item.base_amount = 0.0;
            } else {
                row.qty = 0.0;
                row.base_amount = 0.0;
                returned_item.qty += row.qty;
                returned_item.base_amount += row.base_amount;
            }
        }
    }

    if !row.delivered_by_supplier {
        row.buying_amount = round_to(row.qty * row.buying_rate, currency_precision);
    }
}

pub fn calculate_buying_amount_from_sle(
    row_qty: f64,
    stock_ledger_entries: &[StockLedgerEntry],
    parenttype: &str,
    parent: &str,
    item_row: &str,
    average_buying_rate: f64,
) -> f64 {
    for (index, sle) in stock_ledger_entries.iter().enumerate() {
        if sle.voucher_type == parenttype
            && sle.voucher_no == parent
            && sle.voucher_detail_no == item_row
        {
            let previous_stock_value = stock_ledger_entries
                .get(index + 1)
                .map(|next| next.stock_value)
                .unwrap_or(0.0);
            if previous_stock_value != 0.0 {
                return (previous_stock_value - sle.stock_value).abs() * row_qty / sle.qty.abs();
            }
            return row_qty * average_buying_rate;
        }
    }

    0.0
}

pub fn calculate_buying_amount_from_delivery_note(
    row_qty: f64,
    delivery_note: &DeliveryNoteSummary,
    average_buying_rate: f64,
) -> f64 {
    if delivery_note.total_qty != 0.0 {
        row_qty * delivery_note.total_incoming_value / delivery_note.total_qty
    } else {
        row_qty * average_buying_rate
    }
}

pub fn get_buying_amount_from_product_bundle(
    item_row: &str,
    product_bundle: &[ProductBundleItem],
    incoming_rates: &[(&str, f64)],
    currency_precision: u32,
) -> f64 {
    let total = product_bundle
        .iter()
        .filter(|packed_item| packed_item.parent_detail_docname == item_row)
        .map(|packed_item| {
            let incoming_rate = incoming_rates
                .iter()
                .find_map(|(item_code, rate)| {
                    (*item_code == packed_item.item_code).then_some(*rate)
                })
                .unwrap_or(0.0);
            packed_item.total_qty * -1.0 * incoming_rate
        })
        .sum();

    round_to(total, currency_precision)
}

pub fn get_buying_amount(
    row: &GrossProfitBuyingAmountRow,
    context: &GrossProfitBuyingAmountContext,
) -> f64 {
    if row.delivered_by_supplier && row.so_detail.is_some() && !context.po_details.is_empty() {
        return context.delivered_purchase_amount.unwrap_or(0.0);
    }

    if context.non_stock_items.contains(&row.item_code)
        && (!row.project.is_empty() || !row.cost_center.is_empty())
    {
        return row.qty * context.last_purchase_rate.unwrap_or(0.0);
    }

    if (row.update_stock || row.dn_detail.is_some()) && !context.stock_ledger_entries.is_empty() {
        let mut parenttype = row.parenttype.as_str();
        let mut parent = row.invoice.as_deref().unwrap_or(&row.parent);

        if row.dn_detail.is_some() {
            parenttype = "Delivery Note";
            parent = row.delivery_note.as_deref().unwrap_or_default();
        }

        return calculate_buying_amount_from_sle(
            row.qty,
            &context.stock_ledger_entries,
            parenttype,
            parent,
            row.item_row.as_deref().unwrap_or_default(),
            context.average_buying_rate,
        );
    }

    if row.item_row.is_some() {
        if let Some(delivery_note) = &context.delivery_note {
            return calculate_buying_amount_from_delivery_note(
                row.qty,
                delivery_note,
                context.average_buying_rate,
            );
        }
    }

    if row.sales_order.is_some() && row.so_detail.is_some() {
        if let Some(incoming_rate) = context.so_dn_incoming_rate {
            if incoming_rate != 0.0 {
                return row.qty * incoming_rate;
            }
        }
    }

    row.qty * context.average_buying_rate
}

pub fn get_average_buying_rate(
    cache: &mut IncomingRateCache,
    request: &IncomingRateRequest,
    incoming_rate: f64,
    precision: u32,
) -> f64 {
    let key = (request.item_code.clone(), request.warehouse.clone());
    if let Some(rate) = cache.rates.get(&key) {
        return *rate;
    }

    let rate = round_to(incoming_rate, precision);
    cache.requests.push(request.clone());
    cache.rates.insert(key, rate);
    rate
}

pub fn process_gross_profit_rows(
    rows: &[GrossProfitProcessRow],
    filters: &GrossProfitFilters,
    buying_amounts: &[(&str, f64)],
    packed_overrides: &[PackedItemOverride],
    returned_items: &mut [ReturnedInvoiceItem],
) -> Vec<GrossProfitProcessRow> {
    let mut processed = rows.to_vec();
    let grouped_by_invoice = filters.group_by == "Invoice";
    let mut invoice_buying_amount = 0.0;
    let mut invoice_base_amount = 0.0;

    for row in processed.iter_mut().rev() {
        if filters.group_by == "Monthly" {
            row.monthly = format_month_year(&row.posting_date);
        }

        row.base_amount = round_to(row.base_net_amount, filters.currency_precision);

        if let Some(dn_detail) = row.dn_detail.clone() {
            row.item_row = Some(dn_detail);
            apply_packed_item_override(row, packed_overrides, filters.currency_precision);
        }

        row.buying_amount = round_to(
            row.item_row
                .as_deref()
                .and_then(|item_row| buying_amount_for_row(item_row, buying_amounts))
                .unwrap_or(0.0),
            filters.currency_precision,
        );

        if grouped_by_invoice && row.indent == 0.0 {
            row.buying_amount = invoice_buying_amount;
            row.base_amount = invoice_base_amount;
            row.buying_rate = None;
            row.base_rate = None;
            invoice_buying_amount = 0.0;
            invoice_base_amount = 0.0;
        }

        set_process_rates(row, filters);
        update_return_for_process_row(row, returned_items, filters);

        if grouped_by_invoice && row.indent == 1.0 {
            invoice_buying_amount += row.buying_amount;
            invoice_base_amount += row.base_amount;
        }

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
    }

    processed
}

pub fn get_grouped_by_invoice_total_row(
    rows: &[GrossProfitProcessRow],
    filters: &GrossProfitFilters,
) -> Vec<ReportCell> {
    let total_base_amount: f64 = rows
        .iter()
        .filter(|row| row.indent == 1.0)
        .map(|row| row.base_amount)
        .sum();
    let total_buying_amount: f64 = rows
        .iter()
        .filter(|row| row.indent == 1.0)
        .map(|row| row.buying_amount)
        .sum();
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

    vec![
        ReportCell::Text("Total".to_string()),
        ReportCell::Empty,
        ReportCell::Empty,
        ReportCell::Number(round_to(total_base_amount, filters.currency_precision)),
        ReportCell::Number(round_to(total_buying_amount, filters.currency_precision)),
        ReportCell::Number(total_gross_profit),
        ReportCell::Number(total_percent),
    ]
}

pub fn get_data_when_grouped_by_invoice(
    rows: &[GrossProfitProcessRow],
    filters: &GrossProfitFilters,
) -> Vec<ReportRow> {
    let mut data: Vec<ReportRow> = rows
        .iter()
        .map(|src| {
            let mut row = ReportRow::new();
            row.insert("indent".to_string(), ReportCell::Number(src.indent));
            row.insert(
                "parent_invoice".to_string(),
                ReportCell::Text(src.parent_invoice.clone()),
            );
            row.insert(
                "currency".to_string(),
                ReportCell::Text(filters.currency.clone()),
            );

            for col in get_group_wise_columns()
                .get("invoice")
                .into_iter()
                .flatten()
            {
                let fieldname = get_column_names()
                    .get(col)
                    .copied()
                    .expect("invoice column has fieldname");
                row.insert(fieldname.to_string(), process_row_cell_value(src, col));
            }

            row
        })
        .collect();

    data.push(grouped_by_invoice_total_report_row(rows, filters));
    data
}

pub fn get_data_when_not_grouped_by_invoice(
    grouped_data: &[GrossProfitSourceRow],
    filters: &GrossProfitFilters,
    master_settings: &MasterNameSettings,
) -> Vec<Vec<ReportCell>> {
    group_rows(grouped_data, filters, master_settings)
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

        if filters.group_by == "Payment Term" {
            let portion = invoice_portion(source);
            if let Some(existing) = grouped.get_mut(&key) {
                existing.qty = round_to(existing.qty + row.qty, filters.float_precision);
                apply_payment_term_portion(existing, &row, portion, filters);
            } else {
                let mut new_row = row;
                apply_payment_term_portion_first(&mut new_row, portion);
                set_average_rate(&mut new_row, filters);
                order.push(key.clone());
                grouped.insert(key, new_row);
            }
        } else if let Some(existing) = grouped.get_mut(&key) {
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

fn invoice_portion(row: &GrossProfitSourceRow) -> f64 {
    if row.is_return {
        100.0
    } else if row.invoice_portion != 0.0 {
        row.invoice_portion
    } else if row.payment_amount != 0.0 && row.base_net_amount != 0.0 {
        row.payment_amount * 100.0 / row.base_net_amount
    } else {
        0.0
    }
}

fn buying_amount_for_row(item_row: &str, buying_amounts: &[(&str, f64)]) -> Option<f64> {
    buying_amounts
        .iter()
        .find_map(|(row_name, amount)| (*row_name == item_row).then_some(*amount))
}

fn apply_packed_item_override(
    row: &mut GrossProfitProcessRow,
    packed_overrides: &[PackedItemOverride],
    currency_precision: u32,
) {
    if row.parent.is_some() {
        return;
    }

    let Some(item_code) = row.item_code.as_deref() else {
        return;
    };
    let Some(item_row) = row.item_row.as_deref() else {
        return;
    };

    if let Some(packed_item) = packed_overrides.iter().find(|packed_item| {
        packed_item.parent_invoice == row.parent_invoice
            && packed_item.item_code == item_code
            && packed_item.parent_detail_docname == item_row
    }) {
        row.warehouse = Some(packed_item.warehouse.clone());
        row.base_amount = round_to(packed_item.base_amount, currency_precision);
    }
}

fn set_process_rates(row: &mut GrossProfitProcessRow, filters: &GrossProfitFilters) {
    match row.qty {
        Some(qty) if qty != 0.0 => {
            row.buying_rate = if row.delivered_by_supplier {
                None
            } else {
                Some(round_to(row.buying_amount / qty, filters.float_precision))
            };
            row.base_rate = Some(round_to(row.base_amount / qty, filters.float_precision));
        }
        _ if is_not_invoice_process_row(row, filters) => {
            row.buying_rate = Some(0.0);
            row.base_rate = Some(0.0);
        }
        _ => {}
    }
}

fn update_return_for_process_row(
    row: &mut GrossProfitProcessRow,
    returned_items: &mut [ReturnedInvoiceItem],
    filters: &GrossProfitFilters,
) {
    if !is_not_invoice_process_row(row, filters) {
        return;
    }
    let (Some(parent), Some(item_code), Some(qty)) =
        (row.parent.clone(), row.item_code.clone(), row.qty)
    else {
        return;
    };

    let mut adjusted = ReturnAdjustedRow {
        parent,
        item_code,
        qty,
        base_amount: row.base_amount,
        buying_rate: row.buying_rate.unwrap_or(0.0),
        buying_amount: row.buying_amount,
        delivered_by_supplier: row.delivered_by_supplier,
    };
    update_return_invoices(&mut adjusted, returned_items, filters.currency_precision);
    row.qty = Some(adjusted.qty);
    row.base_amount = adjusted.base_amount;
    row.buying_amount = adjusted.buying_amount;
}

fn is_not_invoice_process_row(row: &GrossProfitProcessRow, filters: &GrossProfitFilters) -> bool {
    is_not_invoice_process_row_for_group(row, &filters.group_by)
}

fn is_not_invoice_process_row_for_group(row: &GrossProfitProcessRow, group_by: &str) -> bool {
    (group_by == "Invoice" && row.indent != 0.0) || group_by != "Invoice"
}

fn format_month_year(date: &str) -> String {
    let mut parts = date.split('-');
    let year = parts.next().unwrap_or_default();
    let month = parts
        .next()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let month_name = match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    };

    if month_name.is_empty() || year.is_empty() {
        String::new()
    } else {
        format!("{month_name} {year}")
    }
}

fn process_row_cell_value(row: &GrossProfitProcessRow, col: &str) -> ReportCell {
    match col {
        "invoice_or_item" => ReportCell::Text(row.invoice_or_item.clone()),
        "customer" => ReportCell::Text(row.customer.clone()),
        "customer_group" => ReportCell::Text(row.customer_group.clone()),
        "customer_name" => ReportCell::Text(row.customer_name.clone()),
        "posting_date" => ReportCell::Text(row.posting_date.clone()),
        "item_code" => option_text_cell(row.item_code.as_deref()),
        "item_name" => option_text_cell(row.item_name.as_deref()),
        "item_group" => option_text_cell(row.item_group.as_deref()),
        "brand" => option_text_cell(row.brand.as_deref()),
        "description" => option_text_cell(row.description.as_deref()),
        "warehouse" => option_text_cell(row.warehouse.as_deref()),
        "qty" => option_number_cell(row.qty),
        "base_rate" => option_number_cell(row.base_rate),
        "buying_rate" => option_number_cell(row.buying_rate),
        "base_amount" => ReportCell::Number(row.base_amount),
        "buying_amount" => ReportCell::Number(row.buying_amount),
        "gross_profit" => ReportCell::Number(row.gross_profit),
        "gross_profit_percent" => ReportCell::Number(row.gross_profit_percent),
        "project" => ReportCell::Text(row.project.clone()),
        _ => ReportCell::Empty,
    }
}

fn grouped_by_invoice_total_report_row(
    rows: &[GrossProfitProcessRow],
    filters: &GrossProfitFilters,
) -> ReportRow {
    let cells = get_grouped_by_invoice_total_row(rows, filters);
    let mut row = ReportRow::new();
    row.insert("sales_invoice".to_string(), cells[0].clone());
    row.insert("qty".to_string(), cells[1].clone());
    row.insert("avg._selling_rate".to_string(), cells[2].clone());
    row.insert("valuation_rate".to_string(), cells[2].clone());
    row.insert("selling_amount".to_string(), cells[3].clone());
    row.insert("buying_amount".to_string(), cells[4].clone());
    row.insert("gross_profit".to_string(), cells[5].clone());
    row.insert("gross_profit_%".to_string(), cells[6].clone());
    row
}

fn option_text_cell(value: Option<&str>) -> ReportCell {
    value
        .map(|value| ReportCell::Text(value.to_string()))
        .unwrap_or(ReportCell::Empty)
}

fn option_number_cell(value: Option<f64>) -> ReportCell {
    value.map(ReportCell::Number).unwrap_or(ReportCell::Empty)
}

fn apply_payment_term_portion_first(row: &mut GrossProfitCalculatedRow, portion: f64) {
    row.base_amount = row.base_amount * portion / 100.0;
    row.buying_amount = row.buying_amount * portion / 100.0;
    row.gross_profit = row.gross_profit * portion / 100.0;
}

fn apply_payment_term_portion(
    existing: &mut GrossProfitCalculatedRow,
    row: &GrossProfitCalculatedRow,
    portion: f64,
    filters: &GrossProfitFilters,
) {
    existing.base_amount = round_to(
        existing.base_amount + row.base_amount * portion / 100.0,
        filters.currency_precision,
    );
    existing.buying_amount = round_to(
        existing.buying_amount + row.buying_amount * portion / 100.0,
        filters.currency_precision,
    );
    existing.gross_profit = round_to(
        existing.gross_profit + row.gross_profit * portion / 100.0,
        filters.currency_precision,
    );
    set_average_rate(existing, filters);
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
    cells.push(ReportCell::Empty);
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
        "monthly" => Some(row.monthly.clone()),
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
