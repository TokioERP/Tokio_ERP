use std::collections::{BTreeMap, HashSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankTransactionStatus {
    Pending,
    Settled,
    Unreconciled,
    Reconciled,
    Cancelled,
}

impl Default for BankTransactionStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BankTransaction {
    pub name: Option<String>,
    pub allocated_amount: f64,
    pub amended_from: Option<String>,
    pub bank_account: Option<String>,
    pub bank_party_account_number: Option<String>,
    pub bank_party_iban: Option<String>,
    pub bank_party_name: Option<String>,
    pub company: Option<String>,
    pub currency: Option<String>,
    pub date: Option<String>,
    pub deposit: f64,
    pub description: Option<String>,
    pub docstatus: i32,
    pub excluded_fee: f64,
    pub included_fee: f64,
    pub naming_series: Option<String>,
    pub party: Option<String>,
    pub party_type: Option<String>,
    pub payment_entries: Vec<BankTransactionPayment>,
    pub reference_number: Option<String>,
    pub status: BankTransactionStatus,
    pub transaction_id: Option<String>,
    pub transaction_type: Option<String>,
    pub unallocated_amount: f64,
    pub withdrawal: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankTransactionPayment {
    pub payment_document: String,
    pub payment_entry: String,
    pub allocated_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BankTransactionError {
    DuplicateReference {
        payment_document: String,
        payment_entry: String,
    },
    AlreadyFullyReconciled {
        name: String,
    },
    IncludedFeeBiggerThanWithdrawal,
    ExcludedFeeBiggerThanDeposit,
    DepositAndWithdrawalWithExcludedFee,
    LinkedBankAccountMismatch {
        linked_bank_account: String,
        payment_entry: String,
        gl_bank_account: String,
    },
    VoucherNotAffectingBankAccount {
        payment_document: String,
        payment_entry: String,
        gl_bank_account: String,
    },
    InvalidBankGlAmount {
        payment_document: String,
        payment_entry: String,
        gl_bank_account: String,
        amount: f64,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClearanceDetails {
    pub allocable_amount: f64,
    pub should_clear: bool,
    pub clearance_date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinkedBankTransaction {
    pub unallocated_amount: f64,
    pub gl_bank_account: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BankGlAllocation {
    pub total: f64,
    pub latest_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RelatedBankGlEntryRow {
    pub doctype: String,
    pub docname: String,
    pub gl_account: String,
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TotalAllocatedAmountRow {
    pub payment_document: String,
    pub payment_entry: String,
    pub gl_account: String,
    pub total: f64,
    pub latest_date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveFromBankTransactionPlan {
    pub bank_transaction_name: String,
    pub removed_entries: Vec<BankTransactionPayment>,
    pub remaining_entries: Vec<BankTransactionPayment>,
    pub save: bool,
}

impl BankTransaction {
    pub const DOCTYPE: &'static str = "Bank Transaction";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const FIELD_ORDER: [&'static str; 35] = [
        "naming_series",
        "date",
        "column_break_2",
        "status",
        "bank_account",
        "company",
        "amended_from",
        "section_break_4",
        "deposit",
        "withdrawal",
        "column_break_7",
        "currency",
        "section_break_10",
        "description",
        "reference_number",
        "column_break_10",
        "transaction_id",
        "transaction_type",
        "section_break_14",
        "column_break_oufv",
        "payment_entries",
        "section_break_18",
        "allocated_amount",
        "column_break_17",
        "unallocated_amount",
        "party_section",
        "party_type",
        "party",
        "column_break_3czf",
        "bank_party_name",
        "bank_party_account_number",
        "bank_party_iban",
        "extended_bank_statement_section",
        "included_fee",
        "excluded_fee",
    ];
    pub const ALLOW_IMPORT: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("naming_series", "Series")
                .options("ACC-BTN-.YYYY.-")
                .default("ACC-BTN-.YYYY.-")
                .required()
                .no_copy()
                .print_hide(),
            FieldSpec::date("date", "Date"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::select("status", "Status")
                .options("\nPending\nSettled\nUnreconciled\nReconciled\nCancelled")
                .default("Pending")
                .in_standard_filter(),
            FieldSpec::link("bank_account", "Bank Account")
                .options("Bank Account")
                .in_standard_filter(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .fetch_from("bank_account.company")
                .read_only()
                .in_standard_filter(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Bank Transaction")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::currency("deposit", "Deposit")
                .options("currency")
                .oldfield("debit", "Currency")
                .in_list_view(),
            FieldSpec::currency("withdrawal", "Withdrawal")
                .options("currency")
                .oldfield("credit", "Currency")
                .in_list_view(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("currency", "Currency").options("Currency"),
            FieldSpec::section_break("section_break_10"),
            FieldSpec::small_text("description", "Description").in_list_view(),
            FieldSpec::small_text("reference_number", "Reference Number").allow_on_submit(),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::data("transaction_id", "Transaction ID").read_only(),
            FieldSpec::data("transaction_type", "Transaction Type").length(50),
            FieldSpec::section_break("section_break_14"),
            FieldSpec::column_break("column_break_oufv"),
            FieldSpec::table("payment_entries", "Payment Entries")
                .options("Bank Transaction Payments")
                .allow_on_submit(),
            FieldSpec::section_break("section_break_18"),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("currency")
                .read_only()
                .allow_on_submit(),
            FieldSpec::column_break("column_break_17"),
            FieldSpec::currency("unallocated_amount", "Unallocated Amount")
                .options("currency")
                .read_only()
                .allow_on_submit(),
            FieldSpec::section_break("party_section").label("Payment From / To"),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .allow_on_submit(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .allow_on_submit(),
            FieldSpec::column_break("column_break_3czf"),
            FieldSpec::data(
                "bank_party_name",
                "Party Name/Account Holder (Bank Statement)",
            ),
            FieldSpec::data(
                "bank_party_account_number",
                "Party Account No. (Bank Statement)",
            ),
            FieldSpec::data("bank_party_iban", "Party IBAN (Bank Statement)").options("IBAN"),
            FieldSpec::section_break("extended_bank_statement_section")
                .label("Extended Bank Statement"),
            FieldSpec::currency("included_fee", "Included Fee").options("currency"),
            FieldSpec::currency("excluded_fee", "Excluded Fee")
                .options("currency")
                .description("On save, the Excluded Fee will be converted to an Included Fee."),
        ]
    }

    pub fn validate_duplicate_references(&self) -> Result<(), BankTransactionError> {
        let mut references = HashSet::new();

        for row in &self.payment_entries {
            let reference = (&row.payment_document, &row.payment_entry);
            if !references.insert(reference) {
                return Err(BankTransactionError::DuplicateReference {
                    payment_document: row.payment_document.clone(),
                    payment_entry: row.payment_entry.clone(),
                });
            }
        }

        Ok(())
    }

    pub fn update_allocated_amount(&mut self) {
        let allocated_amount: f64 = self
            .payment_entries
            .iter()
            .map(|entry| entry.allocated_amount)
            .sum();
        let unallocated_amount = (self.withdrawal - self.deposit).abs() - allocated_amount;

        self.allocated_amount = allocated_amount;
        self.unallocated_amount = unallocated_amount;
    }

    pub fn set_status(&mut self) {
        if self.docstatus == 2 {
            self.status = BankTransactionStatus::Cancelled;
        } else if self.docstatus == 1 {
            if self.unallocated_amount > 0.0 {
                self.status = BankTransactionStatus::Unreconciled;
            } else if self.unallocated_amount <= 0.0 {
                self.status = BankTransactionStatus::Reconciled;
            }
        }
    }

    pub fn add_payment_entries(
        &mut self,
        vouchers: &[(&str, &str)],
    ) -> Result<(), BankTransactionError> {
        if 0.0 >= self.unallocated_amount {
            return Err(BankTransactionError::AlreadyFullyReconciled {
                name: self.name.clone().unwrap_or_default(),
            });
        }

        for (payment_document, payment_entry) in vouchers {
            self.payment_entries.push(BankTransactionPayment::new(
                *payment_document,
                *payment_entry,
                0.0,
            ));
        }

        Ok(())
    }

    pub fn validate_included_fee(&self) -> Result<(), BankTransactionError> {
        if self.included_fee != 0.0 && self.withdrawal != 0.0 && self.included_fee > self.withdrawal
        {
            return Err(BankTransactionError::IncludedFeeBiggerThanWithdrawal);
        }

        Ok(())
    }

    pub fn handle_excluded_fee(&mut self) -> Result<(), BankTransactionError> {
        let excluded_fee = self.excluded_fee;
        if excluded_fee <= 0.0 {
            return Ok(());
        }

        if self.deposit > 0.0 && (self.deposit - excluded_fee) < 0.0 {
            return Err(BankTransactionError::ExcludedFeeBiggerThanDeposit);
        }

        if self.deposit > 0.0 && self.withdrawal > 0.0 {
            return Err(BankTransactionError::DepositAndWithdrawalWithExcludedFee);
        }

        if self.deposit > 0.0 {
            self.deposit -= excluded_fee;
        } else if self.withdrawal >= 0.0 {
            self.withdrawal += excluded_fee;
        }

        self.included_fee += excluded_fee;
        self.excluded_fee = 0.0;

        Ok(())
    }
}

impl BankTransactionPayment {
    pub fn new(
        payment_document: impl Into<String>,
        payment_entry: impl Into<String>,
        allocated_amount: f64,
    ) -> Self {
        Self {
            payment_document: payment_document.into(),
            payment_entry: payment_entry.into(),
            allocated_amount,
        }
    }
}

impl RelatedBankGlEntryRow {
    pub fn new(
        doctype: impl Into<String>,
        docname: impl Into<String>,
        gl_account: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            doctype: doctype.into(),
            docname: docname.into(),
            gl_account: gl_account.into(),
            amount,
        }
    }
}

impl TotalAllocatedAmountRow {
    pub fn new(
        payment_document: impl Into<String>,
        payment_entry: impl Into<String>,
        gl_account: impl Into<String>,
        total: f64,
        latest_date: impl Into<String>,
    ) -> Self {
        Self {
            payment_document: payment_document.into(),
            payment_entry: payment_entry.into(),
            gl_account: gl_account.into(),
            total,
            latest_date: latest_date.into(),
        }
    }
}

impl DocumentController for BankTransaction {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "before_validate",
            "validate",
            "before_submit",
            "before_update_after_submit",
            "on_cancel",
            "on_discard",
        ]
    }
}

pub const fn get_payment_doctypes() -> [&'static str; 5] {
    [
        "Payment Entry",
        "Journal Entry",
        "Sales Invoice",
        "Purchase Invoice",
        "Bank Transaction",
    ]
}

pub fn get_clearance_details(
    transaction: &BankTransaction,
    payment_entry: &BankTransactionPayment,
    mut bt_allocations: BTreeMap<String, BankGlAllocation>,
    mut gl_entries: BTreeMap<String, f64>,
    gl_bank_account: &str,
    linked_bank_transaction: Option<LinkedBankTransaction>,
) -> Result<ClearanceDetails, BankTransactionError> {
    let transaction_date = transaction.date.clone().unwrap_or_default();

    if payment_entry.payment_document == "Bank Transaction" {
        let linked = linked_bank_transaction.unwrap_or_default();
        if linked.gl_bank_account != gl_bank_account {
            return Err(BankTransactionError::LinkedBankAccountMismatch {
                linked_bank_account: linked.gl_bank_account,
                payment_entry: payment_entry.payment_entry.clone(),
                gl_bank_account: gl_bank_account.to_string(),
            });
        }

        return Ok(ClearanceDetails {
            allocable_amount: linked.unallocated_amount.abs(),
            should_clear: true,
            clearance_date: transaction_date,
        });
    }

    if !gl_entries.contains_key(gl_bank_account) {
        return Err(BankTransactionError::VoucherNotAffectingBankAccount {
            payment_document: payment_entry.payment_document.clone(),
            payment_entry: payment_entry.payment_entry.clone(),
            gl_bank_account: gl_bank_account.to_string(),
        });
    }

    let mut allocable_amount = gl_entries.remove(gl_bank_account).unwrap_or(0.0);
    if allocable_amount <= 0.0 {
        return Err(BankTransactionError::InvalidBankGlAmount {
            payment_document: payment_entry.payment_document.clone(),
            payment_entry: payment_entry.payment_entry.clone(),
            gl_bank_account: gl_bank_account.to_string(),
            amount: allocable_amount,
        });
    }

    let matching_bt_allocation = bt_allocations.remove(gl_bank_account).unwrap_or_default();
    allocable_amount -= matching_bt_allocation.total;

    let should_clear = gl_entries.iter().all(|(account, amount)| {
        *amount == bt_allocations.get(account).map_or(0.0, |row| row.total)
    });

    let clearance_date =
        matching_bt_allocation
            .latest_date
            .map_or(transaction_date.clone(), |latest_date| {
                if transaction_date.as_str() > latest_date.as_str() {
                    transaction_date.clone()
                } else {
                    latest_date
                }
            });

    Ok(ClearanceDetails {
        allocable_amount,
        should_clear,
        clearance_date,
    })
}

pub fn group_related_bank_gl_entries(
    rows: &[RelatedBankGlEntryRow],
) -> BTreeMap<(String, String), BTreeMap<String, f64>> {
    let mut entries = BTreeMap::new();

    for row in rows {
        entries
            .entry((row.doctype.clone(), row.docname.clone()))
            .or_insert_with(BTreeMap::new)
            .insert(row.gl_account.clone(), row.amount);
    }

    entries
}

pub fn group_total_allocated_amount(
    rows: &[TotalAllocatedAmountRow],
) -> BTreeMap<(String, String), BTreeMap<String, BankGlAllocation>> {
    let mut payment_allocation_details = BTreeMap::new();

    for row in rows {
        payment_allocation_details
            .entry((row.payment_document.clone(), row.payment_entry.clone()))
            .or_insert_with(BTreeMap::new)
            .insert(
                row.gl_account.clone(),
                BankGlAllocation {
                    total: row.total,
                    latest_date: Some(row.latest_date.clone()),
                },
            );
    }

    payment_allocation_details
}

pub fn remove_from_bank_transaction_plan(
    doctype: &str,
    docname: &str,
    bank_transactions: &[BankTransaction],
) -> Vec<RemoveFromBankTransactionPlan> {
    let mut plans = Vec::new();

    for bank_transaction in bank_transactions {
        if bank_transaction.docstatus == 2 {
            continue;
        }

        let mut modified = false;
        let mut removed_entries = Vec::new();
        let mut remaining_entries = Vec::new();

        for payment_entry in &bank_transaction.payment_entries {
            if payment_entry.payment_document == doctype && payment_entry.payment_entry == docname {
                removed_entries.push(payment_entry.clone());
                modified = true;
            } else {
                remaining_entries.push(payment_entry.clone());
            }
        }

        if modified {
            plans.push(RemoveFromBankTransactionPlan {
                bank_transaction_name: bank_transaction.name.clone().unwrap_or_default(),
                removed_entries,
                remaining_entries,
                save: true,
            });
        }
    }

    plans
}

impl Default for LinkedBankTransaction {
    fn default() -> Self {
        Self {
            unallocated_amount: 0.0,
            gl_bank_account: String::new(),
        }
    }
}
