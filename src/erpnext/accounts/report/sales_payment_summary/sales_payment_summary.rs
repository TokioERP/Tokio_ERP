use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesPaymentFilters {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub company: Option<String>,
    pub customer: Option<String>,
    pub owner: Option<String>,
    pub is_pos: bool,
    pub payment_detail: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SalesPaymentInput {
    pub sales_invoices: Vec<SalesInvoice>,
    pub sales_invoice_payments: Vec<SalesInvoicePayment>,
    pub payment_entries: Vec<PaymentEntryAllocation>,
    pub journal_entries: Vec<JournalEntryAllocation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesPaymentReport {
    pub columns: Vec<String>,
    pub rows: Vec<SalesPaymentRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesPaymentRow {
    pub cells: Vec<ReportCell>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReportCell {
    Text(String),
    Number(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesInvoice {
    pub name: String,
    pub posting_date: String,
    pub owner: String,
    pub company: String,
    pub customer: String,
    pub docstatus: i32,
    pub is_pos: bool,
    pub net_total: f64,
    pub total_taxes_and_charges: f64,
    pub base_paid_amount: f64,
    pub outstanding_amount: f64,
    pub base_change_amount: f64,
    pub warehouse: Option<String>,
    pub cost_center: Option<String>,
    pub base_total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesInvoicePayment {
    pub parent: String,
    pub mode_of_payment: String,
    pub payment_type: String,
    pub base_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntryAllocation {
    pub reference_name: String,
    pub mode_of_payment: String,
    pub allocated_amount: f64,
    pub docstatus: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryAllocation {
    pub reference_name: String,
    pub voucher_type: String,
    pub reference_type: String,
    pub credit: f64,
    pub docstatus: i32,
}

#[derive(Clone, Debug, Default)]
struct InvoiceGroup {
    posting_date: String,
    owner: String,
    net_total: f64,
    total_taxes: f64,
}

impl ReportCell {
    pub fn text(value: &str) -> Self {
        Self::Text(value.to_string())
    }

    pub const fn number(value: f64) -> Self {
        Self::Number(value)
    }
}

impl SalesPaymentRow {
    pub fn new(cells: Vec<ReportCell>) -> Self {
        Self { cells }
    }
}

impl SalesInvoicePayment {
    pub fn new(parent: &str, mode_of_payment: &str, payment_type: &str, base_amount: f64) -> Self {
        Self {
            parent: parent.to_string(),
            mode_of_payment: mode_of_payment.to_string(),
            payment_type: payment_type.to_string(),
            base_amount,
        }
    }
}

impl PaymentEntryAllocation {
    pub fn new(
        reference_name: &str,
        mode_of_payment: &str,
        allocated_amount: f64,
        docstatus: i32,
    ) -> Self {
        Self {
            reference_name: reference_name.to_string(),
            mode_of_payment: mode_of_payment.to_string(),
            allocated_amount,
            docstatus,
        }
    }
}

impl JournalEntryAllocation {
    pub fn new(
        reference_name: &str,
        voucher_type: &str,
        reference_type: &str,
        credit: f64,
        docstatus: i32,
    ) -> Self {
        Self {
            reference_name: reference_name.to_string(),
            voucher_type: voucher_type.to_string(),
            reference_type: reference_type.to_string(),
            credit,
            docstatus,
        }
    }
}

pub fn execute(filters: &SalesPaymentFilters, input: &SalesPaymentInput) -> SalesPaymentReport {
    let columns = get_columns(filters);
    let rows = if filters.is_pos {
        get_pos_sales_payment_data(filters, input)
    } else {
        get_sales_payment_data(filters, input)
    };

    SalesPaymentReport { columns, rows }
}

pub fn get_pos_columns() -> Vec<String> {
    vec![
        "Date:Date:80".to_string(),
        "Owner:Data:200".to_string(),
        "Payment Mode:Data:240".to_string(),
        "Sales and Returns:Currency/currency:120".to_string(),
        "Taxes:Currency/currency:120".to_string(),
        "Payments:Currency/currency:120".to_string(),
        "Warehouse:Data:200".to_string(),
        "Cost Center:Data:200".to_string(),
    ]
}

pub fn get_columns(filters: &SalesPaymentFilters) -> Vec<String> {
    if filters.is_pos {
        get_pos_columns()
    } else {
        vec![
            "Date:Date:80".to_string(),
            "Owner:Data:200".to_string(),
            "Payment Mode:Data:240".to_string(),
            "Sales and Returns:Currency/currency:120".to_string(),
            "Taxes:Currency/currency:120".to_string(),
            "Payments:Currency/currency:120".to_string(),
        ]
    }
}

pub fn get_conditions(filters: &SalesPaymentFilters) -> String {
    let mut conditions = "1=1".to_string();
    if filters.from_date.is_some() {
        conditions.push_str(" and a.posting_date >= %(from_date)s");
    }
    if filters.to_date.is_some() {
        conditions.push_str(" and a.posting_date <= %(to_date)s");
    }
    if filters.company.is_some() {
        conditions.push_str(" and a.company=%(company)s");
    }
    if filters.customer.is_some() {
        conditions.push_str(" and a.customer = %(customer)s");
    }
    if filters.owner.is_some() {
        conditions.push_str(" and a.owner = %(owner)s");
    }
    if filters.is_pos {
        conditions.push_str(" and a.is_pos = %(is_pos)s");
    }
    conditions
}

pub fn get_pos_sales_payment_data(
    filters: &SalesPaymentFilters,
    input: &SalesPaymentInput,
) -> Vec<SalesPaymentRow> {
    filtered_invoices(filters, input)
        .into_iter()
        .map(|invoice| {
            let mode_of_payment = input
                .sales_invoice_payments
                .iter()
                .find(|payment| payment.parent == invoice.name)
                .map(|payment| payment.mode_of_payment.as_str())
                .unwrap_or("");

            SalesPaymentRow::new(vec![
                ReportCell::text(&invoice.posting_date),
                ReportCell::text(&invoice.owner),
                ReportCell::text(mode_of_payment),
                ReportCell::number(invoice.net_total),
                ReportCell::number(invoice.total_taxes_and_charges),
                ReportCell::number(invoice.base_paid_amount),
                ReportCell::text(invoice.warehouse.as_deref().unwrap_or("")),
                ReportCell::text(invoice.cost_center.as_deref().unwrap_or("")),
            ])
        })
        .collect()
}

pub fn get_sales_payment_data(
    filters: &SalesPaymentFilters,
    input: &SalesPaymentInput,
) -> Vec<SalesPaymentRow> {
    let sales_invoice_data = get_sales_invoice_data(filters, input);
    let mode_of_payments = get_mode_of_payments(filters, input);
    let mode_of_payment_details = get_mode_of_payment_details(filters, input);
    let mut rows = Vec::new();

    for invoice_group in sales_invoice_data {
        let owner_posting_date =
            owner_posting_date_key(&invoice_group.owner, &invoice_group.posting_date);
        if filters.payment_detail {
            rows.push(SalesPaymentRow::new(vec![
                ReportCell::text(&invoice_group.posting_date),
                ReportCell::text(&invoice_group.owner),
                ReportCell::text(" "),
                ReportCell::number(invoice_group.net_total),
                ReportCell::number(invoice_group.total_taxes),
                ReportCell::number(0.0),
            ]));

            for (mode_of_payment, paid_amount) in mode_of_payment_details
                .get(&owner_posting_date)
                .into_iter()
                .flatten()
            {
                rows.push(SalesPaymentRow::new(vec![
                    ReportCell::text(&invoice_group.posting_date),
                    ReportCell::text(&invoice_group.owner),
                    ReportCell::text(mode_of_payment),
                    ReportCell::number(0.0),
                    ReportCell::number(0.0),
                    ReportCell::number(*paid_amount),
                    ReportCell::number(0.0),
                ]));
            }
        } else {
            let total_payment = mode_of_payment_details
                .get(&owner_posting_date)
                .into_iter()
                .flatten()
                .map(|(_, paid_amount)| paid_amount)
                .sum();
            let payment_modes = mode_of_payments
                .get(&owner_posting_date)
                .map(|modes| modes.join(", "))
                .unwrap_or_default();

            rows.push(SalesPaymentRow::new(vec![
                ReportCell::text(&invoice_group.posting_date),
                ReportCell::text(&invoice_group.owner),
                ReportCell::text(&payment_modes),
                ReportCell::number(invoice_group.net_total),
                ReportCell::number(invoice_group.total_taxes),
                ReportCell::number(total_payment),
            ]));
        }
    }

    rows
}

fn get_sales_invoice_data(
    filters: &SalesPaymentFilters,
    input: &SalesPaymentInput,
) -> Vec<InvoiceGroup> {
    let mut order = Vec::new();
    let mut groups: BTreeMap<String, InvoiceGroup> = BTreeMap::new();

    for invoice in filtered_invoices(filters, input) {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        if !groups.contains_key(&key) {
            order.push(key.clone());
            groups.insert(
                key.clone(),
                InvoiceGroup {
                    posting_date: invoice.posting_date.clone(),
                    owner: invoice.owner.clone(),
                    net_total: 0.0,
                    total_taxes: 0.0,
                },
            );
        }

        if let Some(group) = groups.get_mut(&key) {
            group.net_total += invoice.net_total;
            group.total_taxes += invoice.total_taxes_and_charges;
        }
    }

    order
        .into_iter()
        .filter_map(|key| groups.remove(&key))
        .collect()
}

fn get_mode_of_payments(
    filters: &SalesPaymentFilters,
    input: &SalesPaymentInput,
) -> BTreeMap<String, Vec<String>> {
    let invoices = filtered_invoices(filters, input);
    let mut mode_of_payments: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for invoice in &invoices {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for payment in input
            .sales_invoice_payments
            .iter()
            .filter(|payment| payment.parent == invoice.name)
        {
            mode_of_payments
                .entry(key.clone())
                .or_default()
                .push(payment.mode_of_payment.clone());
        }
    }

    for invoice in &invoices {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for payment_entry in input.payment_entries.iter().filter(|payment_entry| {
            payment_entry.reference_name == invoice.name && payment_entry.docstatus == 1
        }) {
            mode_of_payments
                .entry(key.clone())
                .or_default()
                .push(payment_entry.mode_of_payment.clone());
        }
    }

    for invoice in &invoices {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for journal_entry in input.journal_entries.iter().filter(|journal_entry| {
            journal_entry.reference_name == invoice.name
                && journal_entry.reference_type == "Sales Invoice"
                && journal_entry.docstatus == 1
        }) {
            mode_of_payments
                .entry(key.clone())
                .or_default()
                .push(journal_entry.voucher_type.clone());
        }
    }

    mode_of_payments
}

fn get_mode_of_payment_details(
    filters: &SalesPaymentFilters,
    input: &SalesPaymentInput,
) -> BTreeMap<String, Vec<(String, f64)>> {
    let invoices = filtered_invoices(filters, input);
    let mut grouped: BTreeMap<(String, String), f64> = BTreeMap::new();
    let mut order = Vec::new();

    for invoice in &invoices {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for payment in input
            .sales_invoice_payments
            .iter()
            .filter(|payment| payment.parent == invoice.name)
        {
            add_payment_detail(
                &mut grouped,
                &mut order,
                key.clone(),
                payment.mode_of_payment.clone(),
                payment.base_amount,
            );
        }
    }

    for invoice in &invoices {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for payment_entry in input.payment_entries.iter().filter(|payment_entry| {
            payment_entry.reference_name == invoice.name && payment_entry.docstatus == 1
        }) {
            add_payment_detail(
                &mut grouped,
                &mut order,
                key.clone(),
                payment_entry.mode_of_payment.clone(),
                payment_entry.allocated_amount,
            );
        }
    }

    for invoice in &invoices {
        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for journal_entry in input.journal_entries.iter().filter(|journal_entry| {
            journal_entry.reference_name == invoice.name
                && journal_entry.reference_type == "Sales Invoice"
                && journal_entry.docstatus == 1
        }) {
            add_payment_detail(
                &mut grouped,
                &mut order,
                key.clone(),
                journal_entry.voucher_type.clone(),
                journal_entry.credit,
            );
        }
    }

    for invoice in &invoices {
        if invoice.base_change_amount <= 0.0 {
            continue;
        }

        let key = owner_posting_date_key(&invoice.owner, &invoice.posting_date);
        for payment in input
            .sales_invoice_payments
            .iter()
            .filter(|payment| payment.parent == invoice.name && payment.payment_type == "Cash")
        {
            let detail_key = (key.clone(), payment.mode_of_payment.clone());
            if let Some(paid_amount) = grouped.get_mut(&detail_key) {
                *paid_amount -= invoice.base_change_amount;
            }
        }
    }

    let mut details: BTreeMap<String, Vec<(String, f64)>> = BTreeMap::new();
    for (key, mode_of_payment) in order {
        if let Some(paid_amount) = grouped.get(&(key.clone(), mode_of_payment.clone())) {
            details
                .entry(key)
                .or_default()
                .push((mode_of_payment, *paid_amount));
        }
    }

    details
}

fn add_payment_detail(
    grouped: &mut BTreeMap<(String, String), f64>,
    order: &mut Vec<(String, String)>,
    key: String,
    mode_of_payment: String,
    paid_amount: f64,
) {
    let detail_key = (key, mode_of_payment);
    if !grouped.contains_key(&detail_key) {
        order.push(detail_key.clone());
    }
    *grouped.entry(detail_key).or_insert(0.0) += paid_amount;
}

fn filtered_invoices<'a>(
    filters: &SalesPaymentFilters,
    input: &'a SalesPaymentInput,
) -> Vec<&'a SalesInvoice> {
    input
        .sales_invoices
        .iter()
        .filter(|invoice| invoice.docstatus == 1)
        .filter(|invoice| {
            filters
                .from_date
                .as_ref()
                .map_or(true, |from_date| invoice.posting_date >= *from_date)
        })
        .filter(|invoice| {
            filters
                .to_date
                .as_ref()
                .map_or(true, |to_date| invoice.posting_date <= *to_date)
        })
        .filter(|invoice| {
            filters
                .company
                .as_ref()
                .map_or(true, |company| invoice.company == *company)
        })
        .filter(|invoice| {
            filters
                .customer
                .as_ref()
                .map_or(true, |customer| invoice.customer == *customer)
        })
        .filter(|invoice| {
            filters
                .owner
                .as_ref()
                .map_or(true, |owner| invoice.owner == *owner)
        })
        .filter(|invoice| !filters.is_pos || invoice.is_pos)
        .collect()
}

fn owner_posting_date_key(owner: &str, posting_date: &str) -> String {
    format!("{owner}{posting_date}")
}
