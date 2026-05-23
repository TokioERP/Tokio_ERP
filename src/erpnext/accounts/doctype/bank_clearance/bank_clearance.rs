use crate::erpnext::accounts::doctype::bank_clearance_detail::bank_clearance_detail::BankClearanceDetail;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BankClearance {
    pub account: Option<String>,
    pub account_currency: Option<String>,
    pub bank_account: Option<String>,
    pub from_date: Option<String>,
    pub include_pos_transactions: bool,
    pub include_reconciled_entries: bool,
    pub payment_entries: Vec<BankClearanceDetail>,
    pub to_date: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BankClearanceRawEntry {
    pub payment_document: String,
    pub payment_entry: String,
    pub against_account: Option<String>,
    pub cheque_number: Option<String>,
    pub cheque_date: Option<String>,
    pub clearance_date: Option<String>,
    pub debit: f64,
    pub credit: f64,
    pub posting_date: String,
    pub account_currency: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankClearanceError {
    FromDateAndToDateMandatory,
    AccountMandatory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankClearanceClientConfig {
    pub account_fetch: (&'static str, &'static str, &'static str),
    pub account_type_filter: [&'static str; 2],
    pub account_is_group: bool,
    pub bank_account_is_company_account: bool,
    pub onload_from_date: &'static str,
    pub onload_to_date: &'static str,
    pub primary_button_without_entries: &'static str,
    pub primary_button_with_entries: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaymentEntrySource {
    JournalEntry,
    PaymentEntry,
    PaidPurchaseInvoice,
    PosSalesInvoice,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankClearanceSourcePlan {
    pub source: PaymentEntrySource,
    pub account: String,
    pub from_date: String,
    pub to_date: String,
    pub unreconciled_only: bool,
    pub bank_account_filter: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankClearanceQueryPlan {
    pub sources: Vec<BankClearanceSourcePlan>,
    pub sort_order: [&'static str; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClearanceDateUpdateRow {
    pub idx: usize,
    pub payment_document: Option<String>,
    pub payment_entry: String,
    pub cheque_date: Option<String>,
    pub clearance_date: Option<String>,
    pub old_clearance_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClearanceDateUpdateAction {
    SalesInvoicePayment {
        parent: String,
        account: String,
        amount_greater_than_zero: bool,
        clearance_date: Option<String>,
        comment: String,
    },
    DocumentDbSet {
        doctype: String,
        name: String,
        fieldname: &'static str,
        clearance_date: Option<String>,
        comment: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClearanceDateUpdatePlan {
    InvalidRows {
        invalid_document_rows: Vec<usize>,
        invalid_cheque_date_rows: Vec<usize>,
        message: String,
    },
    NoClearanceDateMentioned {
        message: &'static str,
    },
    Updated {
        actions: Vec<ClearanceDateUpdateAction>,
        refresh_payment_entries: bool,
        message: &'static str,
    },
}

impl BankClearance {
    pub const DOCTYPE: &'static str = "Bank Clearance";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 10] = [
        "account",
        "account_currency",
        "from_date",
        "to_date",
        "column_break_5",
        "bank_account",
        "include_reconciled_entries",
        "include_pos_transactions",
        "section_break_10",
        "payment_entries",
    ];
    pub const ALLOW_COPY: bool = true;
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const HIDE_TOOLBAR: bool = true;
    pub const ICON: &'static str = "fa fa-check";
    pub const IS_SINGLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const READ_ONLY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "ASC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .fetch_from("bank_account.account")
                .fetch_if_empty()
                .in_list_view()
                .required(),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .hidden()
                .print_hide(),
            FieldSpec::date("from_date", "From Date")
                .in_list_view()
                .required(),
            FieldSpec::date("to_date", "To Date")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::link("bank_account", "Bank Account")
                .options("Bank Account")
                .description("Select the Bank Account to reconcile."),
            FieldSpec::check("include_reconciled_entries", "Include Reconciled Entries")
                .default("0")
                .in_list_view(),
            FieldSpec::check("include_pos_transactions", "Include POS Transactions").default("0"),
            FieldSpec::section_break("section_break_10"),
            FieldSpec::table("payment_entries", "Payment Entries")
                .options("Bank Clearance Detail")
                .allow_bulk_edit(),
        ]
    }

    pub fn get_payment_entries_from_hooks(
        &mut self,
        hook_entries: Vec<Vec<BankClearanceRawEntry>>,
        precision: usize,
        default_currency: &str,
    ) -> Result<(), BankClearanceError> {
        if !(self.from_date.is_some() && self.to_date.is_some()) {
            return Err(BankClearanceError::FromDateAndToDateMandatory);
        }

        if self.account.is_none() {
            return Err(BankClearanceError::AccountMandatory);
        }

        let mut entries: Vec<BankClearanceRawEntry> = hook_entries.into_iter().flatten().collect();
        entries.sort_by(|left, right| left.posting_date.cmp(&right.posting_date));

        self.payment_entries.clear();
        for entry in entries {
            let amount = entry.debit - entry.credit;
            let currency = entry
                .account_currency
                .as_deref()
                .unwrap_or(default_currency);
            let debit_or_credit = if amount > 0.0 { "Dr" } else { "Cr" };

            self.payment_entries.push(BankClearanceDetail {
                payment_document: Some(entry.payment_document),
                payment_entry: Some(entry.payment_entry),
                against_account: entry.against_account,
                amount: Some(format!(
                    "{:.*} {} {}",
                    precision,
                    amount.abs(),
                    currency,
                    debit_or_credit
                )),
                posting_date: Some(entry.posting_date),
                cheque_number: entry.cheque_number,
                cheque_date: entry.cheque_date,
                clearance_date: entry.clearance_date,
            });
        }

        Ok(())
    }

    pub fn update_clearance_date_plan(
        &self,
        rows: &[ClearanceDateUpdateRow],
    ) -> ClearanceDateUpdatePlan {
        let mut invalid_document_rows = Vec::new();
        let mut invalid_cheque_date_rows = Vec::new();
        let mut entries_to_update = Vec::new();

        for row in rows {
            let mut is_valid = true;
            if row.payment_document.is_none() {
                invalid_document_rows.push(row.idx);
                is_valid = false;
            }

            if let (Some(clearance_date), Some(cheque_date)) =
                (&row.clearance_date, &row.cheque_date)
            {
                if clearance_date < cheque_date {
                    invalid_cheque_date_rows.push(row.idx);
                    is_valid = false;
                }
            }

            if is_valid && (row.clearance_date.is_some() || self.include_reconciled_entries) {
                entries_to_update.push(row);
            }
        }

        if !(invalid_document_rows.is_empty() && invalid_cheque_date_rows.is_empty()) {
            return ClearanceDateUpdatePlan::InvalidRows {
                message: build_invalid_rows_message(
                    &invalid_document_rows,
                    &invalid_cheque_date_rows,
                ),
                invalid_document_rows,
                invalid_cheque_date_rows,
            };
        }

        if entries_to_update.is_empty() {
            return ClearanceDateUpdatePlan::NoClearanceDateMentioned {
                message: "Clearance Date not mentioned",
            };
        }

        let mut actions = Vec::new();
        for row in entries_to_update {
            if row.clearance_date.is_none() && row.old_clearance_date.is_none() {
                continue;
            }

            let payment_document = row.payment_document.as_deref().unwrap_or_default();
            let comment = format_clearance_comment(
                row.old_clearance_date.as_deref(),
                row.clearance_date.as_deref(),
            );

            if payment_document == "Sales Invoice" {
                actions.push(ClearanceDateUpdateAction::SalesInvoicePayment {
                    parent: row.payment_entry.clone(),
                    account: self.account.clone().unwrap_or_default(),
                    amount_greater_than_zero: true,
                    clearance_date: row.clearance_date.clone(),
                    comment,
                });
            } else {
                actions.push(ClearanceDateUpdateAction::DocumentDbSet {
                    doctype: payment_document.to_string(),
                    name: row.payment_entry.clone(),
                    fieldname: "clearance_date",
                    clearance_date: row.clearance_date.clone(),
                    comment,
                });
            }
        }

        ClearanceDateUpdatePlan::Updated {
            actions,
            refresh_payment_entries: true,
            message: "Clearance Date updated",
        }
    }
}

impl DocumentController for BankClearance {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["get_payment_entries", "update_clearance_date"]
    }
}

impl BankClearanceClientConfig {
    pub fn from_erpnext_js() -> Self {
        Self {
            account_fetch: ("account", "account_currency", "account_currency"),
            account_type_filter: ["Bank", "Cash"],
            account_is_group: false,
            bank_account_is_company_account: true,
            onload_from_date: "month_start",
            onload_to_date: "month_end",
            primary_button_without_entries: "Get Payment Entries",
            primary_button_with_entries: "Update Clearance Date",
        }
    }
}

pub fn get_payment_entries_for_bank_clearance_plan(
    from_date: &str,
    to_date: &str,
    account: &str,
    _bank_account: Option<&str>,
    include_reconciled_entries: bool,
    include_pos_transactions: bool,
) -> BankClearanceQueryPlan {
    let mut sources = vec![
        source_plan(
            PaymentEntrySource::JournalEntry,
            account,
            from_date,
            to_date,
            include_reconciled_entries,
        ),
        source_plan(
            PaymentEntrySource::PaymentEntry,
            account,
            from_date,
            to_date,
            include_reconciled_entries,
        ),
        source_plan(
            PaymentEntrySource::PaidPurchaseInvoice,
            account,
            from_date,
            to_date,
            include_reconciled_entries,
        ),
    ];

    if include_pos_transactions {
        sources.push(source_plan(
            PaymentEntrySource::PosSalesInvoice,
            account,
            from_date,
            to_date,
            include_reconciled_entries,
        ));
    }

    BankClearanceQueryPlan {
        sources,
        sort_order: ["posting_date asc", "name desc"],
    }
}

fn source_plan(
    source: PaymentEntrySource,
    account: &str,
    from_date: &str,
    to_date: &str,
    include_reconciled_entries: bool,
) -> BankClearanceSourcePlan {
    BankClearanceSourcePlan {
        source,
        account: account.to_string(),
        from_date: from_date.to_string(),
        to_date: to_date.to_string(),
        unreconciled_only: !include_reconciled_entries,
        bank_account_filter: None,
    }
}

fn build_invalid_rows_message(
    invalid_document_rows: &[usize],
    invalid_cheque_date_rows: &[usize],
) -> String {
    let mut message = "<p>Please correct the following row(s):</p><ul>".to_string();
    if !invalid_document_rows.is_empty() {
        message.push_str(&format!(
            "<li>Payment document required for row(s): {}</li>",
            join_rows(invalid_document_rows)
        ));
    }

    if !invalid_cheque_date_rows.is_empty() {
        message.push_str(&format!(
            "<li>Clearance date must be after cheque date for row(s): {}</li>",
            join_rows(invalid_cheque_date_rows)
        ));
    }

    message.push_str("</ul>");
    message
}

fn join_rows(rows: &[usize]) -> String {
    rows.iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_clearance_comment(
    old_clearance_date: Option<&str>,
    clearance_date: Option<&str>,
) -> String {
    format!(
        "Clearance date changed from {} to {} via Bank Clearance Tool",
        old_clearance_date.unwrap_or("None"),
        clearance_date.unwrap_or("None")
    )
}
