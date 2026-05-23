use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BankReconciliationTool {
    pub account_currency: Option<String>,
    pub account_opening_balance: f64,
    pub bank_account: Option<String>,
    pub bank_statement_closing_balance: f64,
    pub bank_statement_from_date: Option<String>,
    pub bank_statement_to_date: Option<String>,
    pub company: Option<String>,
    pub filter_by_reference_date: bool,
    pub from_reference_date: Option<String>,
    pub to_reference_date: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionFilterOp {
    Eq,
    Gt,
    Le,
    Ge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionFilter {
    pub field: &'static str,
    pub op: TransactionFilterOp,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankTransactionsPlan {
    pub doctype: &'static str,
    pub fields: [&'static str; 12],
    pub filters: Vec<TransactionFilter>,
    pub order_by: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateBankTransactionPlan {
    pub doctype: &'static str,
    pub name: String,
    pub updates: BTreeMap<&'static str, Option<String>>,
    pub return_fields: [&'static str; 12],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutoReconcilePlan {
    Enqueue {
        queue: &'static str,
        batches: Vec<Vec<String>>,
        message: &'static str,
    },
    StartNow {
        bank_transactions: Vec<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        filter_by_reference_date: bool,
        from_reference_date: Option<String>,
        to_reference_date: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconcileIndicator {
    Blue,
    Green,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentVoucher {
    pub payment_doctype: String,
    pub payment_name: String,
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllocatedAmount {
    pub doctype: String,
    pub name: String,
    pub gl_account: String,
    pub total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankTransactionInput {
    pub name: String,
    pub bank_account: String,
    pub currency: Option<String>,
    pub deposit: f64,
    pub withdrawal: f64,
    pub unallocated_amount: f64,
    pub reference_number: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoucherMatch {
    PaymentEntry,
    JournalEntry,
    SalesInvoice,
    PurchaseInvoice,
    BankTransaction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchingQueryPlan {
    pub exact_match: bool,
    pub account_from_to: &'static str,
    pub payment_type: &'static str,
    pub common_amount: f64,
    pub bank_account: String,
    pub company: String,
    pub sources: Vec<VoucherMatch>,
    pub date_field: &'static str,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub from_reference_date: Option<String>,
    pub to_reference_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalEntryBtsArgs<'a> {
    pub reference_number: Option<&'a str>,
    pub reference_date: Option<&'a str>,
    pub posting_date: Option<&'a str>,
    pub entry_type: Option<&'a str>,
    pub second_account: &'a str,
    pub mode_of_payment: Option<&'a str>,
    pub party_type: Option<&'a str>,
    pub party: Option<&'a str>,
    pub allow_edit: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryBtsContext<'a> {
    pub company_account: &'a str,
    pub second_account_type: &'a str,
    pub company: &'a str,
    pub company_default_currency: &'a str,
    pub company_account_currency: &'a str,
    pub second_account_currency: &'a str,
    pub default_cost_center: &'a str,
    pub transaction_to_company_exchange_rate: f64,
    pub second_account_exchange_rate: f64,
    pub company_account_exchange_rate: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryPlan {
    pub voucher_type: Option<String>,
    pub company: String,
    pub posting_date: Option<String>,
    pub cheque_date: Option<String>,
    pub cheque_no: Option<String>,
    pub mode_of_payment: Option<String>,
    pub multi_currency: bool,
    pub allow_edit: bool,
    pub accounts_len: usize,
    pub reconcile_voucher: Option<PaymentVoucher>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankReconciliationError {
    PartyRequiredForReceivableOrPayable { account: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentEntryBtsArgs<'a> {
    pub reference_number: Option<&'a str>,
    pub reference_date: Option<&'a str>,
    pub party_type: &'a str,
    pub party: &'a str,
    pub posting_date: Option<&'a str>,
    pub mode_of_payment: Option<&'a str>,
    pub project: Option<&'a str>,
    pub cost_center: Option<&'a str>,
    pub allow_edit: bool,
    pub company_bank_account: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntryBtsContext<'a> {
    pub bank_account: &'a str,
    pub company: &'a str,
    pub party_account: &'a str,
    pub bank_currency: &'a str,
    pub party_currency: &'a str,
    pub exchange_rate: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntryPlan {
    pub payment_type: &'static str,
    pub company: String,
    pub paid_from: String,
    pub paid_to: String,
    pub paid_from_account_currency: String,
    pub paid_to_account_currency: String,
    pub paid_amount: f64,
    pub received_amount: f64,
    pub bank_account: Option<String>,
    pub allow_edit: bool,
    pub reconcile_voucher: Option<PaymentVoucher>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReconcileVouchersPlan {
    pub bank_transaction_name: String,
    pub vouchers: Vec<PaymentVoucher>,
    pub transaction_methods: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankReconciliationClientConfig {
    pub bank_account_query_company_filter: &'static str,
    pub bank_account_query_is_company_account: bool,
    pub default_company_source: &'static str,
    pub from_date_default: &'static str,
    pub to_date_default: &'static str,
    pub upload_button: &'static str,
    pub auto_reconcile_button: &'static str,
    pub get_unreconciled_button: &'static str,
}

impl BankReconciliationTool {
    pub const DOCTYPE: &'static str = "Bank Reconciliation Tool";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 16] = [
        "company",
        "bank_account",
        "column_break_1",
        "bank_statement_from_date",
        "bank_statement_to_date",
        "from_reference_date",
        "to_reference_date",
        "filter_by_reference_date",
        "column_break_2",
        "account_currency",
        "account_opening_balance",
        "bank_statement_closing_balance",
        "section_break_1",
        "reconciliation_tool_cards",
        "reconciliation_tool_dt",
        "no_bank_transactions",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const HIDE_TOOLBAR: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SINGLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .ignore_user_permissions(),
            FieldSpec::link("bank_account", "Bank Account").options("Bank Account"),
            FieldSpec::column_break("column_break_1"),
            FieldSpec::date("bank_statement_from_date", "From Date")
                .depends_on("eval: doc.bank_account && !doc.filter_by_reference_date"),
            FieldSpec::date("bank_statement_to_date", "To Date")
                .depends_on("eval: doc.bank_account && !doc.filter_by_reference_date"),
            FieldSpec::date("from_reference_date", "From Reference Date")
                .depends_on("eval:doc.filter_by_reference_date"),
            FieldSpec::date("to_reference_date", "To Reference Date")
                .depends_on("eval:doc.filter_by_reference_date"),
            FieldSpec::check("filter_by_reference_date", "Filter by Reference Date").default("0"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::currency("account_opening_balance", "Account Opening Balance")
                .options("account_currency")
                .depends_on("eval: doc.bank_statement_from_date")
                .read_only(),
            FieldSpec::currency("bank_statement_closing_balance", "Closing Balance")
                .options("account_currency")
                .depends_on("eval: doc.bank_statement_to_date"),
            FieldSpec::section_break("section_break_1").label("Reconcile"),
            FieldSpec::html("reconciliation_tool_cards", "Reconciliation Tool Cards"),
            FieldSpec::html("reconciliation_tool_dt", "Reconciliation Tool DataTable"),
            FieldSpec::html("no_bank_transactions", "No Bank Transactions").options(
                "<div class=\"text-muted text-center\">No Matching Bank Transactions Found</div>",
            ),
        ]
    }
}

impl DocumentController for BankReconciliationTool {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

impl TransactionFilter {
    pub fn new(field: &'static str, op: TransactionFilterOp, value: impl Into<String>) -> Self {
        Self {
            field,
            op,
            value: value.into(),
        }
    }
}

impl PaymentVoucher {
    pub fn new(
        payment_doctype: impl Into<String>,
        payment_name: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            payment_doctype: payment_doctype.into(),
            payment_name: payment_name.into(),
            amount,
        }
    }
}

impl AllocatedAmount {
    pub fn new(
        doctype: impl Into<String>,
        name: impl Into<String>,
        gl_account: impl Into<String>,
        total: f64,
    ) -> Self {
        Self {
            doctype: doctype.into(),
            name: name.into(),
            gl_account: gl_account.into(),
            total,
        }
    }
}

impl BankReconciliationClientConfig {
    pub fn from_erpnext_js() -> Self {
        Self {
            bank_account_query_company_filter: "company",
            bank_account_query_is_company_account: true,
            default_company_source: "frappe.defaults.get_default(\"company\")",
            from_date_default: "today - 1 month",
            to_date_default: "today",
            upload_button: "Upload Bank Statement",
            auto_reconcile_button: "Auto Reconcile",
            get_unreconciled_button: "Get Unreconciled Entries",
        }
    }
}

pub fn get_bank_transactions_plan(
    bank_account: &str,
    from_date: Option<&str>,
    to_date: Option<&str>,
) -> BankTransactionsPlan {
    let mut filters = vec![
        TransactionFilter::new("bank_account", TransactionFilterOp::Eq, bank_account),
        TransactionFilter::new("docstatus", TransactionFilterOp::Eq, "1"),
        TransactionFilter::new("unallocated_amount", TransactionFilterOp::Gt, "0"),
    ];

    if let Some(to_date) = to_date {
        filters.push(TransactionFilter::new(
            "date",
            TransactionFilterOp::Le,
            to_date,
        ));
    }
    if let Some(from_date) = from_date {
        filters.push(TransactionFilter::new(
            "date",
            TransactionFilterOp::Ge,
            from_date,
        ));
    }

    BankTransactionsPlan {
        doctype: "Bank Transaction",
        fields: BANK_TRANSACTION_FIELDS,
        filters,
        order_by: "date",
    }
}

pub fn account_balance(
    balance_as_per_system: f64,
    entries: &[(f64, f64)],
    amounts_not_reflected_in_system: f64,
) -> f64 {
    let (total_debit, total_credit) = entries
        .iter()
        .fold((0.0, 0.0), |(debit, credit), (row_debit, row_credit)| {
            (debit + row_debit, credit + row_credit)
        });

    balance_as_per_system - total_debit + total_credit + amounts_not_reflected_in_system
}

pub fn update_bank_transaction_plan(
    bank_transaction_name: &str,
    reference_number: &str,
    party_type: Option<&str>,
    party: Option<&str>,
) -> UpdateBankTransactionPlan {
    UpdateBankTransactionPlan {
        doctype: "Bank Transaction",
        name: bank_transaction_name.to_string(),
        updates: BTreeMap::from([
            ("reference_number", Some(reference_number.to_string())),
            ("party_type", party_type.map(str::to_string)),
            ("party", party.map(str::to_string)),
        ]),
        return_fields: BANK_TRANSACTION_FIELDS,
    }
}

pub fn auto_reconcile_plan(
    bank_transactions: Vec<String>,
    from_date: Option<&str>,
    to_date: Option<&str>,
    filter_by_reference_date: bool,
    from_reference_date: Option<&str>,
    to_reference_date: Option<&str>,
) -> AutoReconcilePlan {
    if bank_transactions.len() > 10 {
        return AutoReconcilePlan::Enqueue {
            queue: "long",
            batches: bank_transactions
                .chunks(1000)
                .map(|chunk| chunk.to_vec())
                .collect(),
            message: "Auto Reconciliation has started in the background",
        };
    }

    AutoReconcilePlan::StartNow {
        bank_transactions,
        from_date: from_date.map(str::to_string),
        to_date: to_date.map(str::to_string),
        filter_by_reference_date,
        from_reference_date: from_reference_date.map(str::to_string),
        to_reference_date: to_reference_date.map(str::to_string),
    }
}

pub fn get_auto_reconcile_message(
    partially_reconciled: &[String],
    reconciled: &[String],
) -> (String, ReconcileIndicator) {
    if partially_reconciled.is_empty() && reconciled.is_empty() {
        return (
            "No matches occurred via auto reconciliation".to_string(),
            ReconcileIndicator::Blue,
        );
    }

    let mut alert_message = String::new();
    if !reconciled.is_empty() {
        alert_message.push_str(&format!("{} Transaction(s) Reconciled", reconciled.len()));
        alert_message.push_str("<br>");
    }

    if !partially_reconciled.is_empty() {
        let label = if partially_reconciled.len() > 1 {
            "Transactions"
        } else {
            "Transaction"
        };
        alert_message.push_str(&format!(
            "{} {} Partially Reconciled",
            partially_reconciled.len(),
            label
        ));
    }

    (alert_message, ReconcileIndicator::Green)
}

pub fn subtract_allocations(
    gl_account: &str,
    vouchers: &[PaymentVoucher],
    allocated_amounts: &[AllocatedAmount],
) -> Vec<PaymentVoucher> {
    vouchers
        .iter()
        .map(|voucher| {
            let allocated = allocated_amounts
                .iter()
                .find(|allocated| {
                    allocated.doctype == voucher.payment_doctype
                        && allocated.name == voucher.payment_name
                        && allocated.gl_account == gl_account
                })
                .map(|allocated| allocated.total)
                .unwrap_or(0.0);

            PaymentVoucher {
                amount: voucher.amount - allocated,
                ..voucher.clone()
            }
        })
        .collect()
}

pub fn get_matching_query_plan(
    bank_account: &str,
    company: &str,
    transaction: &BankTransactionInput,
    document_types: &[&str],
    from_date: Option<&str>,
    to_date: Option<&str>,
    filter_by_reference_date: bool,
    from_reference_date: Option<&str>,
    to_reference_date: Option<&str>,
) -> MatchingQueryPlan {
    let exact_match = document_types.contains(&"exact_match");
    let account_from_to = if transaction.deposit > 0.0 {
        "paid_to"
    } else {
        "paid_from"
    };
    let payment_type = if transaction.deposit > 0.0 {
        "Receive"
    } else {
        "Pay"
    };
    let date_field = if filter_by_reference_date {
        "reference_date"
    } else {
        "posting_date"
    };

    let mut sources = Vec::new();
    if document_types.contains(&"payment_entry") {
        sources.push(VoucherMatch::PaymentEntry);
    }
    if document_types.contains(&"journal_entry") {
        sources.push(VoucherMatch::JournalEntry);
    }
    if transaction.deposit > 0.0 && document_types.contains(&"sales_invoice") {
        sources.push(VoucherMatch::SalesInvoice);
    }
    if transaction.withdrawal > 0.0 && document_types.contains(&"purchase_invoice") {
        sources.push(VoucherMatch::PurchaseInvoice);
    }
    if document_types.contains(&"bank_transaction") {
        sources.push(VoucherMatch::BankTransaction);
    }

    MatchingQueryPlan {
        exact_match,
        account_from_to,
        payment_type,
        common_amount: transaction.unallocated_amount,
        bank_account: bank_account.to_string(),
        company: company.to_string(),
        sources,
        date_field,
        from_date: from_date.map(str::to_string),
        to_date: to_date.map(str::to_string),
        from_reference_date: from_reference_date.map(str::to_string),
        to_reference_date: to_reference_date.map(str::to_string),
    }
}

pub fn create_journal_entry_bts_plan(
    transaction: &BankTransactionInput,
    args: JournalEntryBtsArgs<'_>,
    context: JournalEntryBtsContext<'_>,
) -> Result<JournalEntryPlan, BankReconciliationError> {
    if matches!(context.second_account_type, "Receivable" | "Payable")
        && !(args.party_type.is_some() && args.party.is_some())
    {
        return Err(
            BankReconciliationError::PartyRequiredForReceivableOrPayable {
                account: args.second_account.to_string(),
            },
        );
    }

    let transaction_currency = transaction
        .currency
        .as_deref()
        .unwrap_or(context.company_default_currency);
    let multi_currency = context.company_default_currency != context.company_account_currency
        || context.company_default_currency != context.second_account_currency
        || context.company_default_currency != transaction_currency;
    let paid_amount = if transaction.deposit > 0.0 {
        transaction.deposit
    } else {
        transaction.withdrawal
    };

    Ok(JournalEntryPlan {
        voucher_type: args.entry_type.map(str::to_string),
        company: context.company.to_string(),
        posting_date: args.posting_date.map(str::to_string),
        cheque_date: args.reference_date.map(str::to_string),
        cheque_no: args.reference_number.map(str::to_string),
        mode_of_payment: args.mode_of_payment.map(str::to_string),
        multi_currency,
        allow_edit: args.allow_edit,
        accounts_len: 2,
        reconcile_voucher: (!args.allow_edit)
            .then(|| PaymentVoucher::new("Journal Entry", "<new>", paid_amount)),
    })
}

pub fn create_payment_entry_bts_plan(
    transaction: &BankTransactionInput,
    args: PaymentEntryBtsArgs<'_>,
    context: PaymentEntryBtsContext<'_>,
) -> PaymentEntryPlan {
    let payment_type = if transaction.deposit > 0.0 {
        "Receive"
    } else {
        "Pay"
    };
    let amount_in_party_currency = transaction.unallocated_amount * context.exchange_rate;
    let amount_in_bank_currency = transaction.unallocated_amount;

    PaymentEntryPlan {
        payment_type,
        company: context.company.to_string(),
        paid_from: if payment_type == "Receive" {
            context.party_account.to_string()
        } else {
            context.bank_account.to_string()
        },
        paid_to: if payment_type == "Pay" {
            context.party_account.to_string()
        } else {
            context.bank_account.to_string()
        },
        paid_from_account_currency: if payment_type == "Receive" {
            context.party_currency.to_string()
        } else {
            context.bank_currency.to_string()
        },
        paid_to_account_currency: if payment_type == "Pay" {
            context.party_currency.to_string()
        } else {
            context.bank_currency.to_string()
        },
        paid_amount: if payment_type == "Receive" {
            amount_in_party_currency
        } else {
            amount_in_bank_currency
        },
        received_amount: if payment_type == "Pay" {
            amount_in_party_currency
        } else {
            amount_in_bank_currency
        },
        bank_account: args.company_bank_account.map(str::to_string),
        allow_edit: args.allow_edit,
        reconcile_voucher: (!args.allow_edit)
            .then(|| PaymentVoucher::new("Payment Entry", "<new>", amount_in_bank_currency)),
    }
}

pub fn reconcile_vouchers_plan(
    bank_transaction_name: &str,
    vouchers: Vec<PaymentVoucher>,
) -> ReconcileVouchersPlan {
    ReconcileVouchersPlan {
        bank_transaction_name: bank_transaction_name.to_string(),
        vouchers,
        transaction_methods: vec![
            "add_payment_entries",
            "validate_duplicate_references",
            "allocate_payment_entries",
            "update_allocated_amount",
            "set_status",
            "save",
        ],
    }
}

const BANK_TRANSACTION_FIELDS: [&str; 12] = [
    "date",
    "deposit",
    "withdrawal",
    "currency",
    "description",
    "name",
    "bank_account",
    "company",
    "unallocated_amount",
    "reference_number",
    "party_type",
    "party",
];
