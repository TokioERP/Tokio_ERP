#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChequeClearanceFilters {
    pub account: String,
    pub report_date: String,
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
pub struct ChequeVoucher {
    pub doctype: String,
    pub name: String,
    pub amount: Option<f64>,
    pub payment_type: Option<String>,
    pub party_type: Option<String>,
    pub debit_in_account_currency: Option<f64>,
    pub credit_in_account_currency: Option<f64>,
    pub posting_date: Option<String>,
    pub clearance_date: Option<String>,
    pub payment_document: Option<String>,
    pub payment_entry: Option<String>,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChequeClearanceQueryPlan {
    pub source_doctype: &'static str,
    pub child_doctype: Option<&'static str>,
    pub account_filter: &'static str,
    pub posting_date_filter: &'static str,
    pub clearance_date_filter: &'static str,
    pub docstatus_filter: &'static str,
    pub extra_filter: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChequesIncorrectlyClearedReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<ChequeVoucher>,
}

impl ChequeClearanceFilters {
    pub fn new(account: impl Into<String>, report_date: impl Into<String>) -> Self {
        Self {
            account: account.into(),
            report_date: report_date.into(),
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
}

impl ChequeVoucher {
    pub fn payment_entry(
        name: impl Into<String>,
        amount: f64,
        payment_type: impl Into<String>,
        party_type: Option<&str>,
        posting_date: impl Into<String>,
        clearance_date: impl Into<String>,
    ) -> Self {
        Self {
            doctype: "Payment Entry".to_string(),
            name: name.into(),
            amount: Some(amount),
            payment_type: Some(payment_type.into()),
            party_type: party_type.map(str::to_string),
            debit_in_account_currency: None,
            credit_in_account_currency: None,
            posting_date: Some(posting_date.into()),
            clearance_date: Some(clearance_date.into()),
            payment_document: None,
            payment_entry: None,
            debit: None,
            credit: None,
        }
    }

    pub fn journal_entry(
        name: impl Into<String>,
        debit_in_account_currency: f64,
        credit_in_account_currency: f64,
        posting_date: impl Into<String>,
        clearance_date: impl Into<String>,
    ) -> Self {
        Self {
            doctype: "Journal Entry".to_string(),
            name: name.into(),
            amount: None,
            payment_type: None,
            party_type: None,
            debit_in_account_currency: Some(debit_in_account_currency),
            credit_in_account_currency: Some(credit_in_account_currency),
            posting_date: Some(posting_date.into()),
            clearance_date: Some(clearance_date.into()),
            payment_document: None,
            payment_entry: None,
            debit: None,
            credit: None,
        }
    }

    pub fn unknown(doctype: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            doctype: doctype.into(),
            name: name.into(),
            amount: None,
            payment_type: None,
            party_type: None,
            debit_in_account_currency: None,
            credit_in_account_currency: None,
            posting_date: None,
            clearance_date: None,
            payment_document: None,
            payment_entry: None,
            debit: None,
            credit: None,
        }
    }

    pub fn report_row(
        payment_document: impl Into<String>,
        payment_entry: impl Into<String>,
        posting_date: impl Into<String>,
        clearance_date: impl Into<String>,
        debit: f64,
        credit: f64,
    ) -> Self {
        Self {
            doctype: String::new(),
            name: String::new(),
            amount: None,
            payment_type: None,
            party_type: None,
            debit_in_account_currency: None,
            credit_in_account_currency: None,
            posting_date: Some(posting_date.into()),
            clearance_date: Some(clearance_date.into()),
            payment_document: Some(payment_document.into()),
            payment_entry: Some(payment_entry.into()),
            debit: Some(debit),
            credit: Some(credit),
        }
    }
}

impl ChequeClearanceQueryPlan {
    pub fn from_filters(_filters: &ChequeClearanceFilters) -> Vec<Self> {
        vec![
            Self {
                source_doctype: "Journal Entry",
                child_doctype: Some("Journal Entry Account"),
                account_filter: "Journal Entry Account.account = filters.account",
                posting_date_filter: "Journal Entry.posting_date > filters.report_date",
                clearance_date_filter: "Journal Entry.clearance_date <= filters.report_date",
                docstatus_filter: "Journal Entry.docstatus = 1",
                extra_filter: "(Journal Entry.is_opening is null or Journal Entry.is_opening = 'No')",
            },
            Self {
                source_doctype: "Payment Entry",
                child_doctype: None,
                account_filter: "Payment Entry.paid_from = filters.account or Payment Entry.paid_to = filters.account",
                posting_date_filter: "Payment Entry.posting_date > filters.report_date",
                clearance_date_filter: "Payment Entry.clearance_date <= filters.report_date",
                docstatus_filter: "Payment Entry.docstatus = 1",
                extra_filter: "",
            },
        ]
    }
}

pub fn execute(vouchers: Vec<ChequeVoucher>) -> ChequesIncorrectlyClearedReport {
    ChequesIncorrectlyClearedReport {
        columns: get_columns(),
        rows: build_data(vouchers),
    }
}

pub fn build_data(vouchers: Vec<ChequeVoucher>) -> Vec<ChequeVoucher> {
    vouchers
        .iter()
        .filter_map(|voucher| match voucher.doctype.as_str() {
            "Payment Entry" => Some(build_payment_entry_dict(voucher)),
            "Journal Entry" => Some(build_journal_entry_dict(voucher)),
            _ => None,
        })
        .collect()
}

pub fn build_payment_entry_dict(row: &ChequeVoucher) -> ChequeVoucher {
    let payment_type = row.payment_type.as_deref();
    let party_type = row.party_type.as_deref();
    let is_receive_party = payment_type == Some("Receive")
        && matches!(party_type, Some("Customer") | Some("Supplier"));
    let amount = row.amount.unwrap_or_default();
    let (debit, credit) = if is_receive_party {
        (amount, 0.0)
    } else {
        (0.0, amount)
    };

    ChequeVoucher::report_row(
        "Payment Entry",
        row.name.clone(),
        row.posting_date.clone().unwrap_or_default(),
        row.clearance_date.clone().unwrap_or_default(),
        debit,
        credit,
    )
}

pub fn build_journal_entry_dict(row: &ChequeVoucher) -> ChequeVoucher {
    ChequeVoucher::report_row(
        "Journal Entry",
        row.name.clone(),
        row.posting_date.clone().unwrap_or_default(),
        row.clearance_date.clone().unwrap_or_default(),
        row.debit_in_account_currency.unwrap_or_default(),
        row.credit_in_account_currency.unwrap_or_default(),
    )
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::data("Payment Document Type", "payment_document", 220),
        ReportColumn::dynamic_link("Payment Document", "payment_entry", "payment_document", 220),
        ReportColumn::currency("Debit", "debit", "account_currency", 120),
        ReportColumn::currency("Credit", "credit", "account_currency", 120),
        ReportColumn::date("Posting Date", "posting_date", 110),
        ReportColumn::date("Clearance Date", "clearance_date", 110),
    ]
}
