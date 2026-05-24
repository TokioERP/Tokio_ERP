#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BilledItemsToBeReceivedFilters {
    pub company: String,
    pub posting_date: String,
    pub purchase_invoice: Option<String>,
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
pub struct ReportField {
    pub doctype: &'static str,
    pub fieldname: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportFilter {
    pub doctype: &'static str,
    pub fieldname: &'static str,
    pub operator: &'static str,
    pub value: ReportFilterValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReportFilterValue {
    Scalar(String),
    List(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseInvoice {
    pub name: String,
    pub supplier: String,
    pub company: String,
    pub posting_date: String,
    pub currency: String,
    pub docstatus: i32,
    pub per_received: f64,
    pub update_stock: bool,
    pub is_opening: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseInvoiceItem {
    pub parent: String,
    pub item_code: String,
    pub item_name: String,
    pub uom: String,
    pub qty: f64,
    pub received_qty: f64,
    pub rate: f64,
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BilledItemsToBeReceivedRow {
    pub name: String,
    pub supplier: String,
    pub company: String,
    pub posting_date: String,
    pub currency: String,
    pub item_code: String,
    pub item_name: String,
    pub uom: String,
    pub qty: f64,
    pub received_qty: f64,
    pub rate: f64,
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BilledItemsToBeReceivedReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<BilledItemsToBeReceivedRow>,
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

    pub const fn date(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Date",
            options: "",
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

    pub const fn float(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Float",
            options: "",
            width,
        }
    }

    pub const fn currency(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Currency",
            options: "",
            width,
        }
    }
}

impl ReportField {
    pub const fn new(doctype: &'static str, fieldname: &'static str) -> Self {
        Self { doctype, fieldname }
    }

    pub fn frappe_field(&self) -> String {
        format!("`tab{}`.`{}`", self.doctype, self.fieldname)
    }
}

impl ReportFilter {
    pub fn new(
        doctype: &'static str,
        fieldname: &'static str,
        operator: &'static str,
        value: impl Into<String>,
    ) -> Self {
        Self {
            doctype,
            fieldname,
            operator,
            value: ReportFilterValue::Scalar(value.into()),
        }
    }

    pub fn list(
        doctype: &'static str,
        fieldname: &'static str,
        operator: &'static str,
        values: Vec<&str>,
    ) -> Self {
        Self {
            doctype,
            fieldname,
            operator,
            value: ReportFilterValue::List(values.into_iter().map(str::to_string).collect()),
        }
    }
}

pub fn execute(
    filters: &BilledItemsToBeReceivedFilters,
    invoices: &[PurchaseInvoice],
    items: &[PurchaseInvoiceItem],
) -> BilledItemsToBeReceivedReport {
    BilledItemsToBeReceivedReport {
        columns: get_columns(),
        rows: get_data(filters, invoices, items),
    }
}

pub fn get_data(
    filters: &BilledItemsToBeReceivedFilters,
    invoices: &[PurchaseInvoice],
    items: &[PurchaseInvoiceItem],
) -> Vec<BilledItemsToBeReceivedRow> {
    let mut rows = Vec::new();

    for invoice in invoices {
        if !invoice_matches_filters(invoice, filters) {
            continue;
        }

        for item in items.iter().filter(|item| item.parent == invoice.name) {
            rows.push(BilledItemsToBeReceivedRow {
                name: invoice.name.clone(),
                supplier: invoice.supplier.clone(),
                company: invoice.company.clone(),
                posting_date: invoice.posting_date.clone(),
                currency: invoice.currency.clone(),
                item_code: item.item_code.clone(),
                item_name: item.item_name.clone(),
                uom: item.uom.clone(),
                qty: item.qty,
                received_qty: item.received_qty,
                rate: item.rate,
                amount: item.amount,
            });
        }
    }

    rows
}

pub fn get_report_filters(filters: &BilledItemsToBeReceivedFilters) -> Vec<ReportFilter> {
    let mut report_filters = vec![
        ReportFilter::new("Purchase Invoice", "company", "=", filters.company.clone()),
        ReportFilter::new(
            "Purchase Invoice",
            "posting_date",
            "<=",
            filters.posting_date.clone(),
        ),
        ReportFilter::new("Purchase Invoice", "docstatus", "=", "1"),
        ReportFilter::new("Purchase Invoice", "per_received", "<", "100"),
        ReportFilter::new("Purchase Invoice", "update_stock", "=", "0"),
        ReportFilter::new("Purchase Invoice", "is_opening", "!=", "Yes"),
    ];

    if let Some(purchase_invoice) = filters.purchase_invoice.as_deref() {
        report_filters.push(ReportFilter::list(
            "Purchase Invoice",
            "per_received",
            "in",
            vec![purchase_invoice],
        ));
    }

    report_filters
}

pub fn get_report_fields() -> Vec<ReportField> {
    vec![
        ReportField::new("Purchase Invoice", "name"),
        ReportField::new("Purchase Invoice", "supplier"),
        ReportField::new("Purchase Invoice", "company"),
        ReportField::new("Purchase Invoice", "posting_date"),
        ReportField::new("Purchase Invoice", "currency"),
        ReportField::new("Purchase Invoice Item", "item_code"),
        ReportField::new("Purchase Invoice Item", "item_name"),
        ReportField::new("Purchase Invoice Item", "uom"),
        ReportField::new("Purchase Invoice Item", "qty"),
        ReportField::new("Purchase Invoice Item", "received_qty"),
        ReportField::new("Purchase Invoice Item", "rate"),
        ReportField::new("Purchase Invoice Item", "amount"),
    ]
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Purchase Invoice", "name", "Purchase Invoice", 170),
        ReportColumn::link("Supplier", "supplier", "Supplier", 120),
        ReportColumn::date("Posting Date", "posting_date", 100),
        ReportColumn::link("Item Code", "item_code", "Item", 100),
        ReportColumn::data("Item Name", "item_name", 100),
        ReportColumn::link("UOM", "uom", "UOM", 100),
        ReportColumn::float("Invoiced Qty", "qty", 100),
        ReportColumn::float("Received Qty", "received_qty", 100),
        ReportColumn::currency("Rate", "rate", 100),
        ReportColumn::currency("Amount", "amount", 100),
    ]
}

fn invoice_matches_filters(
    invoice: &PurchaseInvoice,
    filters: &BilledItemsToBeReceivedFilters,
) -> bool {
    invoice.company == filters.company
        && invoice.posting_date <= filters.posting_date
        && invoice.docstatus == 1
        && invoice.per_received < 100.0
        && !invoice.update_stock
        && invoice.is_opening != "Yes"
        && filters
            .purchase_invoice
            .as_deref()
            .map_or(true, |purchase_invoice| {
                invoice.per_received.to_string() == purchase_invoice
            })
}
