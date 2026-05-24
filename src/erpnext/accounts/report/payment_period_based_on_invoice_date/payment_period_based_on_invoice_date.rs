use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentPeriodError {
    InvalidPartyTypeForPaymentType {
        payment_type: String,
        party_type: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentPeriodFilters {
    pub payment_type: String,
    pub company: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub report_date: Option<String>,
    pub range: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentPeriodColumn {
    pub fieldname: &'static str,
    pub label: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentPeriodTransaction {
    pub voucher_type: String,
    pub voucher_no: String,
    pub party_type: String,
    pub party: String,
    pub posting_date: String,
    pub amount: f64,
    pub remarks: String,
    pub against_voucher_no: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentInvoice {
    pub doctype: String,
    pub name: String,
    pub company: String,
    pub posting_date: String,
    pub due_date: String,
    pub submitted: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentPeriodRow {
    pub payment_document: String,
    pub payment_entry: String,
    pub party_type: String,
    pub party: String,
    pub posting_date: String,
    pub invoice: String,
    pub invoice_posting_date: Option<String>,
    pub due_date: Option<String>,
    pub amount: f64,
    pub remarks: String,
    pub age: Option<i64>,
    pub range1: f64,
    pub range2: f64,
    pub range3: f64,
    pub range4: f64,
    pub delay_in_payment: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentPeriodReport {
    pub filters: PaymentPeriodFilters,
    pub columns: Vec<PaymentPeriodColumn>,
    pub rows: Vec<PaymentPeriodRow>,
}

impl PaymentPeriodFilters {
    pub fn new(payment_type: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            payment_type: payment_type.into(),
            company: company.into(),
            party_type: None,
            party: None,
            from_date: None,
            to_date: None,
            report_date: None,
            range: None,
        }
    }

    pub fn with_report_date(mut self, report_date: impl Into<String>) -> Self {
        self.report_date = Some(report_date.into());
        self
    }

    pub fn with_range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }
}

impl PaymentPeriodColumn {
    pub const fn data(fieldname: &'static str, label: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label,
            fieldtype: "Data",
            options: "",
            width,
        }
    }

    pub const fn date(fieldname: &'static str, label: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label,
            fieldtype: "Date",
            options: "",
            width,
        }
    }

    pub const fn int(fieldname: &'static str, label: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label,
            fieldtype: "Int",
            options: "",
            width,
        }
    }

    pub const fn currency(fieldname: &'static str, label: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label,
            fieldtype: "Currency",
            options: "",
            width,
        }
    }

    pub const fn dynamic_link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            fieldname,
            label,
            fieldtype: "Dynamic Link",
            options,
            width,
        }
    }

    pub const fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            fieldname,
            label,
            fieldtype: "Link",
            options,
            width,
        }
    }
}

impl PaymentPeriodTransaction {
    pub fn new(
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
        party_type: impl Into<String>,
        party: impl Into<String>,
        posting_date: impl Into<String>,
        amount: f64,
        remarks: impl Into<String>,
        against_voucher_no: impl Into<String>,
    ) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
            party_type: party_type.into(),
            party: party.into(),
            posting_date: posting_date.into(),
            amount,
            remarks: remarks.into(),
            against_voucher_no: against_voucher_no.into(),
        }
    }
}

impl PaymentInvoice {
    pub fn new(
        doctype: impl Into<String>,
        name: impl Into<String>,
        company: impl Into<String>,
        posting_date: impl Into<String>,
        due_date: impl Into<String>,
        submitted: bool,
    ) -> Self {
        Self {
            doctype: doctype.into(),
            name: name.into(),
            company: company.into(),
            posting_date: posting_date.into(),
            due_date: due_date.into(),
            submitted,
        }
    }
}

pub fn execute(
    filters: PaymentPeriodFilters,
    entries: Vec<PaymentPeriodTransaction>,
    invoices: Vec<PaymentInvoice>,
) -> Result<PaymentPeriodReport, PaymentPeriodError> {
    validate_filters(&filters)?;
    let invoice_details = get_invoice_posting_date_map(&filters, &invoices);
    let rows = entries
        .into_iter()
        .map(|entry| build_row(&filters, entry, &invoice_details))
        .collect();

    Ok(PaymentPeriodReport {
        columns: get_columns(&filters),
        filters,
        rows,
    })
}

pub fn validate_filters(filters: &PaymentPeriodFilters) -> Result<(), PaymentPeriodError> {
    match (filters.payment_type.as_str(), filters.party_type.as_deref()) {
        ("Incoming", Some("Supplier")) | ("Outgoing", Some("Customer")) => {
            Err(PaymentPeriodError::InvalidPartyTypeForPaymentType {
                payment_type: filters.payment_type.clone(),
                party_type: filters.party_type.clone().unwrap_or_default(),
            })
        }
        _ => Ok(()),
    }
}

pub fn get_columns(filters: &PaymentPeriodFilters) -> Vec<PaymentPeriodColumn> {
    vec![
        PaymentPeriodColumn::data("payment_document", "Payment Document Type", 100),
        PaymentPeriodColumn::dynamic_link(
            "Payment Document",
            "payment_entry",
            "payment_document",
            160,
        ),
        PaymentPeriodColumn::data("party_type", "Party Type", 100),
        PaymentPeriodColumn::dynamic_link("Party", "party", "party_type", 160),
        PaymentPeriodColumn::date("posting_date", "Posting Date", 100),
        PaymentPeriodColumn::link(
            "Invoice",
            "invoice",
            if filters.payment_type == "Outgoing" {
                "Purchase Invoice"
            } else {
                "Sales Invoice"
            },
            160,
        ),
        PaymentPeriodColumn::date("invoice_posting_date", "Invoice Posting Date", 100),
        PaymentPeriodColumn::date("due_date", "Payment Due Date", 100),
        PaymentPeriodColumn::currency("amount", "Amount", 140),
        PaymentPeriodColumn::data("remarks", "Remarks", 200),
        PaymentPeriodColumn::int("age", "Age", 50),
        PaymentPeriodColumn::currency("range1", "0-30", 140),
        PaymentPeriodColumn::currency("range2", "30-60", 140),
        PaymentPeriodColumn::currency("range3", "60-90", 140),
        PaymentPeriodColumn::currency("range4", "90 Above", 140),
        PaymentPeriodColumn::int("delay_in_payment", "Delay in payment (Days)", 100),
    ]
}

pub fn get_conditions(filters: &PaymentPeriodFilters) -> Vec<&'static str> {
    let mut conditions = vec!["delinked = 0"];
    if filters.payment_type == "Outgoing" {
        conditions.push("party_type = Supplier");
        conditions.push("against_voucher_type = Purchase Invoice");
    } else {
        conditions.push("party_type = Customer");
        conditions.push("against_voucher_type = Sales Invoice");
    }
    if filters.party.is_some() {
        conditions.push("party = filters.party");
    }
    if filters.from_date.is_some() {
        conditions.push("posting_date >= filters.from_date");
    }
    if filters.to_date.is_some() {
        conditions.push("posting_date <= filters.to_date");
    }
    if !filters.company.is_empty() {
        conditions.push("company = filters.company");
    }
    conditions
}

pub fn get_invoice_posting_date_map(
    filters: &PaymentPeriodFilters,
    invoices: &[PaymentInvoice],
) -> BTreeMap<String, PaymentInvoice> {
    let invoice_doctype = if filters.payment_type == "Incoming" {
        "Sales Invoice"
    } else {
        "Purchase Invoice"
    };
    invoices
        .iter()
        .filter(|invoice| {
            invoice.submitted
                && invoice.doctype == invoice_doctype
                && invoice.company == filters.company
        })
        .map(|invoice| (invoice.name.clone(), invoice.clone()))
        .collect()
}

fn build_row(
    filters: &PaymentPeriodFilters,
    entry: PaymentPeriodTransaction,
    invoice_details: &BTreeMap<String, PaymentInvoice>,
) -> PaymentPeriodRow {
    let invoice = invoice_details.get(&entry.against_voucher_no);
    let amount = entry.amount.abs();
    let mut row = PaymentPeriodRow {
        payment_document: entry.voucher_type,
        payment_entry: entry.voucher_no,
        party_type: entry.party_type,
        party: entry.party,
        posting_date: entry.posting_date,
        invoice: entry.against_voucher_no,
        invoice_posting_date: invoice.map(|invoice| invoice.posting_date.clone()),
        due_date: invoice.map(|invoice| invoice.due_date.clone()),
        amount,
        remarks: entry.remarks,
        age: None,
        range1: 0.0,
        range2: 0.0,
        range3: 0.0,
        range4: 0.0,
        delay_in_payment: None,
    };

    if let Some(invoice) = invoice {
        apply_ageing(filters, &mut row, &invoice.posting_date, amount);
        row.delay_in_payment =
            Some(days_between(&invoice.due_date, &row.posting_date).unwrap_or(0));
    }

    row
}

fn apply_ageing(
    filters: &PaymentPeriodFilters,
    row: &mut PaymentPeriodRow,
    entry_date: &str,
    outstanding: f64,
) {
    let Some(age_as_on) = filters.report_date.as_deref() else {
        return;
    };
    let Some(age) = days_between(entry_date, age_as_on) else {
        return;
    };
    row.age = Some(age);

    let ranges = parse_ranges(filters.range.as_deref().unwrap_or("30, 60, 90, 120"));
    let bucket = ranges
        .iter()
        .position(|days| age <= *days)
        .map(|index| index + 1)
        .unwrap_or(ranges.len() + 1);

    match bucket {
        1 => row.range1 = outstanding,
        2 => row.range2 = outstanding,
        3 => row.range3 = outstanding,
        4 => row.range4 = outstanding,
        _ => {}
    }
}

fn parse_ranges(range: &str) -> Vec<i64> {
    range
        .split(',')
        .filter_map(|part| part.trim().parse::<i64>().ok())
        .collect()
}

fn days_between(start: &str, end: &str) -> Option<i64> {
    Some(days_from_civil(end)? - days_from_civil(start)?)
}

fn days_from_civil(date: &str) -> Option<i64> {
    let mut parts = date.split('-');
    let mut year = parts.next()?.parse::<i64>().ok()?;
    let month = parts.next()?.parse::<i64>().ok()?;
    let day = parts.next()?.parse::<i64>().ok()?;
    year -= (month <= 2) as i64;
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_prime = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month_prime + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146097 + day_of_era - 719468)
}
