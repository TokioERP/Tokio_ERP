#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankClearanceFilters {
    pub account: String,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankClearanceEntry {
    pub payment_document_type: String,
    pub payment_entry: String,
    pub posting_date: String,
    pub cheque_no: String,
    pub clearance_date: Option<String>,
    pub against: String,
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankClearanceSummaryReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<BankClearanceEntry>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankClearanceSource {
    JournalEntry,
    PaymentEntry,
    PurchaseInvoice,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankClearanceQueryPlan {
    pub source: BankClearanceSource,
    pub account_filter: &'static str,
    pub date_filter: &'static str,
    pub docstatus_filter: &'static str,
    pub extra_filter: &'static str,
    pub ordering: Vec<&'static str>,
    pub amount_formula: &'static str,
}

impl BankClearanceFilters {
    pub fn new(
        account: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            from_date: from_date.into(),
            to_date: to_date.into(),
        }
    }
}

impl ReportColumn {
    pub const fn data(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Data",
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
            label,
            fieldname,
            fieldtype: "Dynamic Link",
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

    pub const fn plain(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "",
            options: "",
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
            label,
            fieldname,
            fieldtype: "Link",
            options,
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

impl BankClearanceEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        payment_document_type: impl Into<String>,
        payment_entry: impl Into<String>,
        posting_date: impl Into<String>,
        cheque_no: impl Into<String>,
        clearance_date: Option<&str>,
        against: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            payment_document_type: payment_document_type.into(),
            payment_entry: payment_entry.into(),
            posting_date: posting_date.into(),
            cheque_no: cheque_no.into(),
            clearance_date: clearance_date.map(str::to_string),
            against: against.into(),
            amount,
        }
    }
}

impl BankClearanceSource {
    pub fn amount_from(
        self,
        debit_in_account_currency: f64,
        credit_in_account_currency: f64,
        paid_or_received_amount: f64,
        total_taxes_and_charges: f64,
        paid_from_selected_account: bool,
    ) -> f64 {
        match self {
            Self::JournalEntry => debit_in_account_currency - credit_in_account_currency,
            Self::PaymentEntry if paid_from_selected_account => {
                (paid_or_received_amount * -1.0) - total_taxes_and_charges
            }
            Self::PaymentEntry => paid_or_received_amount,
            Self::PurchaseInvoice => paid_or_received_amount * -1.0,
        }
    }
}

impl BankClearanceQueryPlan {
    pub fn from_filters(_filters: &BankClearanceFilters) -> Vec<Self> {
        vec![
            Self {
                source: BankClearanceSource::JournalEntry,
                account_filter: "Journal Entry Account.account = filters.account",
                date_filter: "Journal Entry.posting_date between filters.from_date and filters.to_date",
                docstatus_filter: "Journal Entry.docstatus = 1",
                extra_filter: "(Journal Entry.is_opening = 'No' or Journal Entry.is_opening is null)",
                ordering: vec!["Journal Entry.posting_date desc", "Journal Entry.name desc"],
                amount_formula: "debit_in_account_currency - credit_in_account_currency",
            },
            Self {
                source: BankClearanceSource::PaymentEntry,
                account_filter: "Payment Entry.paid_from = filters.account or Payment Entry.paid_to = filters.account",
                date_filter: "Payment Entry.posting_date between filters.from_date and filters.to_date",
                docstatus_filter: "Payment Entry.docstatus = 1",
                extra_filter: "",
                ordering: vec!["Payment Entry.posting_date desc", "Payment Entry.name desc"],
                amount_formula: "if paid_from == filters.account then (paid_amount * -1) - total_taxes_and_charges else received_amount",
            },
            Self {
                source: BankClearanceSource::PurchaseInvoice,
                account_filter: "Purchase Invoice.cash_bank_account = filters.account",
                date_filter: "Purchase Invoice.posting_date between filters.from_date and filters.to_date",
                docstatus_filter: "Purchase Invoice.docstatus = 1",
                extra_filter: "Purchase Invoice.is_paid = 1",
                ordering: vec![
                    "Purchase Invoice.posting_date desc",
                    "Purchase Invoice.name desc",
                ],
                amount_formula: "paid_amount * -1",
            },
        ]
    }
}

pub fn execute(
    _filters: BankClearanceFilters,
    hook_entries: Vec<Vec<BankClearanceEntry>>,
) -> BankClearanceSummaryReport {
    let mut rows = hook_entries.into_iter().flatten().collect::<Vec<_>>();
    rows.sort_by(|left, right| left.posting_date.cmp(&right.posting_date));

    BankClearanceSummaryReport {
        columns: get_columns(),
        rows,
    }
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::data("Payment Document Type", "payment_document_type", 130),
        ReportColumn::dynamic_link(
            "Payment Entry",
            "payment_entry",
            "payment_document_type",
            140,
        ),
        ReportColumn::date("Posting Date", "posting_date", 120),
        ReportColumn::plain("Cheque/Reference No", "cheque_no", 120),
        ReportColumn::date("Clearance Date", "clearance_date", 120),
        ReportColumn::link("Against Account", "against", "Account", 200),
        ReportColumn::currency("Amount", "amount", 120),
    ]
}
