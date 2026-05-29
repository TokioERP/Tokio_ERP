#[derive(Clone, Debug, PartialEq)]
pub struct BankReconciliationFilters {
    pub company: String,
    pub account: Option<String>,
    pub report_date: String,
    pub account_currency: String,
    pub balance_as_per_system: f64,
    pub include_pos_transactions: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankReconciliationSource {
    JournalEntry,
    PaymentEntry,
    PurchaseInvoice,
    SalesInvoice,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankReconciliationEntry {
    pub source: BankReconciliationSource,
    pub payment_entry: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub against_account: String,
    pub reference_no: Option<String>,
    pub ref_date: Option<String>,
    pub clearance_date: Option<String>,
    pub account_currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncorrectlyClearedJournalEntry {
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncorrectlyClearedPaymentEntry {
    pub paid_from_matches_account: bool,
    pub paid_amount: f64,
    pub received_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncorrectlyClearedPurchaseInvoice {
    pub paid_amount: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: &'static str,
    pub label: &'static str,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankReconciliationRow {
    pub posting_date: Option<String>,
    pub payment_document: Option<String>,
    pub payment_entry: Option<String>,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
    pub against_account: Option<String>,
    pub reference_no: Option<String>,
    pub ref_date: Option<String>,
    pub clearance_date: Option<String>,
    pub account_currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankReconciliationReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<BankReconciliationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankReconciliationQueryPlan {
    pub source: BankReconciliationSource,
    pub base_filters: Vec<&'static str>,
    pub ordering: Vec<&'static str>,
}

impl ReportColumn {
    pub fn new(
        label: &'static str,
        fieldname: &'static str,
        fieldtype: &'static str,
        options: Option<&'static str>,
        width: u16,
    ) -> Self {
        Self {
            fieldname,
            label,
            fieldtype,
            options,
            width,
        }
    }

    pub fn date(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self::new(label, fieldname, "Date", None, width)
    }

    pub fn data(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self::new(label, fieldname, "Data", None, width)
    }

    pub fn dynamic_link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self::new(label, fieldname, "Dynamic Link", Some(options), width)
    }

    pub fn currency(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self::new(label, fieldname, "Currency", Some(options), width)
    }

    pub fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self::new(label, fieldname, "Link", Some(options), width)
    }
}

impl BankReconciliationEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: BankReconciliationSource,
        payment_entry: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
        against_account: &str,
        reference_no: Option<&str>,
        ref_date: Option<&str>,
        clearance_date: Option<&str>,
        account_currency: &str,
    ) -> Self {
        Self {
            source,
            payment_entry: payment_entry.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
            against_account: against_account.to_string(),
            reference_no: reference_no.map(str::to_string),
            ref_date: ref_date.map(str::to_string),
            clearance_date: clearance_date.map(str::to_string),
            account_currency: account_currency.to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn journal_entry(
        payment_entry: &str,
        posting_date: &str,
        debit_in_account_currency: f64,
        credit_in_account_currency: f64,
        against_account: &str,
        cheque_no: Option<&str>,
        cheque_date: Option<&str>,
        clearance_date: Option<&str>,
        account_currency: &str,
    ) -> Self {
        Self::new(
            BankReconciliationSource::JournalEntry,
            payment_entry,
            posting_date,
            debit_in_account_currency,
            credit_in_account_currency,
            against_account,
            cheque_no,
            cheque_date,
            clearance_date,
            account_currency,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn payment_entry(
        payment_entry: &str,
        posting_date: &str,
        paid_from: &str,
        paid_to: &str,
        party: Option<&str>,
        received_amount_after_tax: f64,
        paid_amount_after_tax: f64,
        paid_to_account_currency: &str,
        paid_from_account_currency: &str,
        reference_no: Option<&str>,
        ref_date: Option<&str>,
        clearance_date: Option<&str>,
        account: &str,
    ) -> Self {
        let paid_to_matches_account = paid_to == account;
        let paid_from_matches_account = paid_from == account;
        let debit = if paid_to_matches_account {
            received_amount_after_tax
        } else {
            0.0
        };
        let credit = if paid_from_matches_account {
            paid_amount_after_tax
        } else {
            0.0
        };
        let against_account = party.unwrap_or(if paid_from_matches_account {
            paid_to
        } else {
            paid_from
        });
        let account_currency = if paid_to_matches_account {
            paid_to_account_currency
        } else {
            paid_from_account_currency
        };

        Self::new(
            BankReconciliationSource::PaymentEntry,
            payment_entry,
            posting_date,
            debit,
            credit,
            against_account,
            reference_no,
            ref_date,
            clearance_date,
            account_currency,
        )
    }

    pub fn purchase_invoice(
        payment_entry: &str,
        bill_no: Option<&str>,
        posting_date: &str,
        paid_amount: f64,
        supplier: &str,
        clearance_date: Option<&str>,
        account_currency: &str,
    ) -> Self {
        let debit = if paid_amount < 0.0 {
            paid_amount.abs()
        } else {
            0.0
        };
        let credit = if paid_amount > 0.0 { paid_amount } else { 0.0 };

        Self::new(
            BankReconciliationSource::PurchaseInvoice,
            payment_entry,
            posting_date,
            debit,
            credit,
            supplier,
            bill_no,
            Some(posting_date),
            clearance_date,
            account_currency,
        )
    }

    pub fn pos_sales_invoice(
        payment_entry: &str,
        posting_date: &str,
        amount: f64,
        debit_to: &str,
        clearance_date: Option<&str>,
        account_currency: &str,
    ) -> Self {
        Self::new(
            BankReconciliationSource::SalesInvoice,
            payment_entry,
            posting_date,
            amount,
            0.0,
            debit_to,
            None,
            None,
            clearance_date,
            account_currency,
        )
    }
}

impl IncorrectlyClearedJournalEntry {
    pub fn new(debit_in_account_currency: f64, credit_in_account_currency: f64) -> Self {
        Self {
            debit_in_account_currency,
            credit_in_account_currency,
        }
    }
}

impl IncorrectlyClearedPaymentEntry {
    pub fn new(paid_from_matches_account: bool, paid_amount: f64, received_amount: f64) -> Self {
        Self {
            paid_from_matches_account,
            paid_amount,
            received_amount,
        }
    }
}

impl IncorrectlyClearedPurchaseInvoice {
    pub fn new(paid_amount: f64) -> Self {
        Self { paid_amount }
    }
}

impl BankReconciliationRow {
    pub fn blank() -> Self {
        Self {
            posting_date: None,
            payment_document: None,
            payment_entry: None,
            debit: None,
            credit: None,
            against_account: None,
            reference_no: None,
            ref_date: None,
            clearance_date: None,
            account_currency: None,
        }
    }

    pub fn is_blank(&self) -> bool {
        *self == Self::blank()
    }

    pub fn balance(label: &str, amount: f64, account_currency: &str) -> Self {
        get_balance_row(label, amount, account_currency)
    }

    pub fn outstanding(debit: f64, credit: f64, account_currency: &str) -> Self {
        Self {
            posting_date: None,
            payment_document: None,
            payment_entry: Some("Outstanding Cheques and Deposits to clear".to_string()),
            debit: Some(debit),
            credit: Some(credit),
            against_account: None,
            reference_no: None,
            ref_date: None,
            clearance_date: None,
            account_currency: Some(account_currency.to_string()),
        }
    }
}

impl BankReconciliationQueryPlan {
    pub fn for_filters(filters: &BankReconciliationFilters) -> Vec<Self> {
        let mut plans = vec![
            Self {
                source: BankReconciliationSource::JournalEntry,
                base_filters: vec![
                    "docstatus = 1",
                    "account = filters.account",
                    "posting_date <= filters.report_date",
                    "clearance_date is null or clearance_date > filters.report_date",
                    "company = filters.company",
                    "is_opening is null or is_opening = 'No'",
                ],
                ordering: vec!["posting_date", "name desc"],
            },
            Self {
                source: BankReconciliationSource::PaymentEntry,
                base_filters: vec![
                    "docstatus = 1",
                    "paid_from = filters.account or paid_to = filters.account",
                    "posting_date <= filters.report_date",
                    "clearance_date is null or clearance_date > filters.report_date",
                    "company = filters.company",
                ],
                ordering: vec!["posting_date", "name desc"],
            },
            Self {
                source: BankReconciliationSource::PurchaseInvoice,
                base_filters: vec![
                    "docstatus = 1",
                    "is_paid = 1",
                    "cash_bank_account = filters.account",
                    "posting_date <= filters.report_date",
                    "clearance_date is null or clearance_date > filters.report_date",
                    "company = filters.company",
                ],
                ordering: vec!["posting_date", "name desc"],
            },
        ];

        if filters.include_pos_transactions {
            plans.push(Self {
                source: BankReconciliationSource::SalesInvoice,
                base_filters: vec![
                    "Sales Invoice Payment.account = filters.account",
                    "Sales Invoice.docstatus = 1",
                    "Sales Invoice.posting_date <= filters.report_date",
                    "Sales Invoice Payment.clearance_date is null or clearance_date > filters.report_date",
                    "Sales Invoice.company = filters.company",
                ],
                ordering: vec!["posting_date", "Sales Invoice Payment.name desc"],
            });
        }

        plans
    }
}

pub fn execute(
    filters: &BankReconciliationFilters,
    entries: Vec<BankReconciliationEntry>,
    amounts_not_reflected_in_system: f64,
) -> BankReconciliationReport {
    let columns = get_columns();

    if filters.account.is_none() {
        return BankReconciliationReport {
            columns,
            rows: Vec::new(),
        };
    }

    let account_currency = filters.account_currency.clone();
    let mut rows = get_entries(entries);
    let total_debit: f64 = rows.iter().map(|row| row.debit.unwrap_or(0.0)).sum();
    let total_credit: f64 = rows.iter().map(|row| row.credit.unwrap_or(0.0)).sum();
    let bank_balance = filters.balance_as_per_system - total_debit
        + total_credit
        + amounts_not_reflected_in_system;

    rows.extend([
        get_balance_row(
            "Bank Statement balance as per General Ledger",
            filters.balance_as_per_system,
            &account_currency,
        ),
        BankReconciliationRow::blank(),
        BankReconciliationRow::outstanding(total_debit, total_credit, &account_currency),
        get_balance_row(
            "Cheques and Deposits incorrectly cleared",
            amounts_not_reflected_in_system,
            &account_currency,
        ),
        BankReconciliationRow::blank(),
        get_balance_row(
            "Calculated Bank Statement balance",
            bank_balance,
            &account_currency,
        ),
    ]);

    BankReconciliationReport { columns, rows }
}

pub fn execute_with_uncleared_entries(
    filters: &BankReconciliationFilters,
    entries: Vec<BankReconciliationEntry>,
    journal_entries: &[IncorrectlyClearedJournalEntry],
    payment_entries: &[IncorrectlyClearedPaymentEntry],
    purchase_invoices: &[IncorrectlyClearedPurchaseInvoice],
) -> BankReconciliationReport {
    let amounts_not_reflected_in_system = get_amounts_not_reflected_in_system_from_entries(
        journal_entries,
        payment_entries,
        purchase_invoices,
    );

    execute(filters, entries, amounts_not_reflected_in_system)
}

pub fn get_amounts_not_reflected_in_system_from_entries(
    journal_entries: &[IncorrectlyClearedJournalEntry],
    payment_entries: &[IncorrectlyClearedPaymentEntry],
    purchase_invoices: &[IncorrectlyClearedPurchaseInvoice],
) -> f64 {
    let journal_entry_amount: f64 = journal_entries
        .iter()
        .map(|entry| entry.debit_in_account_currency - entry.credit_in_account_currency)
        .sum();
    let payment_entry_amount: f64 = payment_entries
        .iter()
        .map(|entry| {
            if entry.paid_from_matches_account {
                entry.paid_amount
            } else {
                entry.received_amount
            }
        })
        .sum();
    let purchase_invoice_amount: f64 = purchase_invoices
        .iter()
        .map(|entry| entry.paid_amount)
        .sum();

    journal_entry_amount + payment_entry_amount + purchase_invoice_amount
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::date("Posting Date", "posting_date", 90),
        ReportColumn::data("Payment Document Type", "payment_document", 220),
        ReportColumn::dynamic_link("Payment Document", "payment_entry", "payment_document", 220),
        ReportColumn::currency("Debit", "debit", "account_currency", 120),
        ReportColumn::currency("Credit", "credit", "account_currency", 120),
        ReportColumn::link("Against Account", "against_account", "Account", 200),
        ReportColumn::data("Reference", "reference_no", 100),
        ReportColumn::date("Ref Date", "ref_date", 110),
        ReportColumn::date("Clearance Date", "clearance_date", 110),
        ReportColumn::link("Currency", "account_currency", "Currency", 100),
    ]
}

pub fn get_entries(mut entries: Vec<BankReconciliationEntry>) -> Vec<BankReconciliationRow> {
    entries.sort_by(|left, right| left.posting_date.cmp(&right.posting_date));

    entries
        .into_iter()
        .map(BankReconciliationRow::from)
        .collect()
}

pub fn get_balance_row(label: &str, amount: f64, account_currency: &str) -> BankReconciliationRow {
    if amount > 0.0 {
        BankReconciliationRow {
            posting_date: None,
            payment_document: None,
            payment_entry: Some(label.to_string()),
            debit: Some(amount),
            credit: Some(0.0),
            against_account: None,
            reference_no: None,
            ref_date: None,
            clearance_date: None,
            account_currency: Some(account_currency.to_string()),
        }
    } else {
        BankReconciliationRow {
            posting_date: None,
            payment_document: None,
            payment_entry: Some(label.to_string()),
            debit: Some(0.0),
            credit: Some(amount.abs()),
            against_account: None,
            reference_no: None,
            ref_date: None,
            clearance_date: None,
            account_currency: Some(account_currency.to_string()),
        }
    }
}

impl From<BankReconciliationEntry> for BankReconciliationRow {
    fn from(entry: BankReconciliationEntry) -> Self {
        Self {
            posting_date: Some(entry.posting_date),
            payment_document: Some(entry.source.label().to_string()),
            payment_entry: Some(entry.payment_entry),
            debit: Some(entry.debit),
            credit: Some(entry.credit),
            against_account: Some(entry.against_account),
            reference_no: entry.reference_no,
            ref_date: entry.ref_date,
            clearance_date: entry.clearance_date,
            account_currency: Some(entry.account_currency),
        }
    }
}

impl BankReconciliationSource {
    fn label(self) -> &'static str {
        match self {
            Self::JournalEntry => "Journal Entry",
            Self::PaymentEntry => "Payment Entry",
            Self::PurchaseInvoice => "Purchase Invoice",
            Self::SalesInvoice => "Sales Invoice",
        }
    }
}
