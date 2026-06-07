use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::erpnext::DocumentController;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountMeta {
    pub account_type: Option<String>,
    pub account_currency: Option<String>,
    pub root_type: Option<String>,
    pub company: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyTypeMeta {
    pub account_type: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceDoc {
    pub party: String,
    pub account: String,
    pub docstatus: i32,
    pub outstanding_amount: f64,
    pub grand_total: f64,
    pub advance_paid: f64,
    pub per_billed: f64,
    pub status: Option<String>,
    pub company_currency: String,
    pub conversion_rate: f64,
    pub due_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryAccountRow {
    pub idx: usize,
    pub account: String,
    pub account_type: Option<String>,
    pub bank_account: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub account_currency: Option<String>,
    pub exchange_rate: f64,
    pub debit_in_account_currency: f64,
    pub debit: f64,
    pub credit_in_account_currency: f64,
    pub credit: f64,
    pub reference_type: Option<String>,
    pub reference_name: Option<String>,
    pub reference_due_date: Option<String>,
    pub reference_detail_no: Option<String>,
    pub advance_voucher_type: Option<String>,
    pub advance_voucher_no: Option<String>,
    pub is_advance: String,
    pub user_remark: Option<String>,
    pub against_account: Option<String>,
}

impl Default for JournalEntryAccountRow {
    fn default() -> Self {
        Self {
            idx: 0,
            account: String::new(),
            account_type: None,
            bank_account: None,
            party_type: None,
            party: None,
            cost_center: None,
            project: None,
            account_currency: None,
            exchange_rate: 0.0,
            debit_in_account_currency: 0.0,
            debit: 0.0,
            credit_in_account_currency: 0.0,
            credit: 0.0,
            reference_type: None,
            reference_name: None,
            reference_due_date: None,
            reference_detail_no: None,
            advance_voucher_type: None,
            advance_voucher_no: None,
            is_advance: "No".to_string(),
            user_remark: None,
            against_account: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntry {
    pub name: Option<String>,
    pub title: Option<String>,
    pub voucher_type: String,
    pub naming_series: String,
    pub posting_date: String,
    pub company: String,
    pub company_currency: String,
    pub finance_book: Option<String>,
    pub accounts: Vec<JournalEntryAccountRow>,
    pub cheque_no: Option<String>,
    pub cheque_date: Option<String>,
    pub user_remark: Option<String>,
    pub total_debit: f64,
    pub total_credit: f64,
    pub difference: f64,
    pub multi_currency: bool,
    pub total_amount_currency: Option<String>,
    pub total_amount: f64,
    pub total_amount_in_words: Option<String>,
    pub clearance_date: Option<String>,
    pub remark: Option<String>,
    pub inter_company_journal_entry_reference: Option<String>,
    pub bill_no: Option<String>,
    pub bill_date: Option<String>,
    pub due_date: Option<String>,
    pub write_off_based_on: Option<String>,
    pub write_off_amount: f64,
    pub pay_to_recd_from: Option<String>,
    pub mode_of_payment: Option<String>,
    pub payment_order: Option<String>,
    pub is_opening: String,
    pub stock_entry: Option<String>,
    pub auto_repeat: Option<String>,
    pub amended_from: Option<String>,
    pub from_template: Option<String>,
    pub tax_withholding_category: Option<String>,
    pub apply_tds: bool,
    pub reversal_of: Option<String>,
    pub process_deferred_accounting: Option<String>,
    pub is_system_generated: bool,
    pub periodic_entry_difference_account: Option<String>,
    pub for_all_stock_asset_accounts: bool,
    pub stock_asset_account: Option<String>,
    pub party_not_required: bool,
    pub tax_withholding_group: Option<String>,
    pub ignore_tax_withholding_threshold: bool,
    pub override_tax_withholding_entries: bool,
    pub custom_remark: bool,
}

impl Default for JournalEntry {
    fn default() -> Self {
        Self {
            name: None,
            title: None,
            voucher_type: "Journal Entry".to_string(),
            naming_series: "ACC-JV-.YYYY.-".to_string(),
            posting_date: String::new(),
            company: String::new(),
            company_currency: String::new(),
            finance_book: None,
            accounts: Vec::new(),
            cheque_no: None,
            cheque_date: None,
            user_remark: None,
            total_debit: 0.0,
            total_credit: 0.0,
            difference: 0.0,
            multi_currency: false,
            total_amount_currency: None,
            total_amount: 0.0,
            total_amount_in_words: None,
            clearance_date: None,
            remark: None,
            inter_company_journal_entry_reference: None,
            bill_no: None,
            bill_date: None,
            due_date: None,
            write_off_based_on: None,
            write_off_amount: 0.0,
            pay_to_recd_from: None,
            mode_of_payment: None,
            payment_order: None,
            is_opening: String::new(),
            stock_entry: None,
            auto_repeat: None,
            amended_from: None,
            from_template: None,
            tax_withholding_category: None,
            apply_tds: false,
            reversal_of: None,
            process_deferred_accounting: None,
            is_system_generated: false,
            periodic_entry_difference_account: None,
            for_all_stock_asset_accounts: false,
            stock_asset_account: None,
            party_not_required: false,
            tax_withholding_group: None,
            ignore_tax_withholding_threshold: false,
            override_tax_withholding_entries: false,
            custom_remark: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlMapRow {
    pub account: String,
    pub party_type: Option<String>,
    pub due_date: Option<String>,
    pub party: Option<String>,
    pub against: Option<String>,
    pub debit: f64,
    pub credit: f64,
    pub account_currency: String,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub transaction_currency: String,
    pub transaction_exchange_rate: f64,
    pub debit_in_transaction_currency: f64,
    pub credit_in_transaction_currency: f64,
    pub against_voucher_type: Option<String>,
    pub against_voucher: Option<String>,
    pub remarks: Option<String>,
    pub voucher_detail_no: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub finance_book: Option<String>,
    pub advance_voucher_type: Option<String>,
    pub advance_voucher_no: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JournalEntryError {
    AccountsTableBlank,
    PartyRequired {
        row: usize,
        account: String,
    },
    PartyAccountTypeMismatch {
        row: usize,
        account: String,
        party_type: String,
    },
    MultiCurrencyRequired,
    ExchangeRateMandatory {
        row: usize,
    },
    ZeroDebitCredit {
        row: usize,
    },
    DebitAndCreditSameRow,
    DifferenceNotZero,
    CustomerAdvanceMustBeCredit {
        row: usize,
    },
    SupplierAdvanceMustBeDebit {
        row: usize,
    },
    InvalidReference {
        row: usize,
        reference_name: String,
    },
    PartyAccountMismatch {
        row: usize,
        reference_type: String,
        reference_name: String,
    },
    DebitLinkedWithSalesOrder {
        row: usize,
    },
    CreditLinkedWithPurchaseOrder {
        row: usize,
    },
    PaymentGreaterThanOutstanding {
        reference_type: String,
        reference_name: String,
    },
    OrderNotSubmitted {
        reference_type: String,
        reference_name: String,
    },
    OrderFullyBilled {
        reference_type: String,
        reference_name: String,
    },
    OrderClosed {
        reference_type: String,
        reference_name: String,
    },
    AdvanceGreaterThanGrandTotal {
        reference_type: String,
        reference_name: String,
    },
    BankEntryReferenceRequired,
    ChequeNoRequired,
    StockAccountInvalidTransaction {
        account: String,
    },
    CreditLimitCrossed {
        customer: String,
        outstanding: String,
        credit_limit: String,
    },
}

impl JournalEntry {
    pub const DOCTYPE: &'static str = "Journal Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const IS_SUBMITTABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const TITLE_FIELD: &'static str = "title";
    pub const FIELD_ORDER: [&'static str; 74] = [
        "entry_type_and_date",
        "title",
        "voucher_type",
        "naming_series",
        "column_break1",
        "posting_date",
        "company",
        "finance_book",
        "2_add_edit_gl_entries",
        "accounts",
        "section_break99",
        "cheque_no",
        "cheque_date",
        "user_remark",
        "total_debit",
        "total_credit",
        "difference",
        "get_balance",
        "multi_currency",
        "total_amount_currency",
        "total_amount",
        "total_amount_in_words",
        "reference",
        "clearance_date",
        "remark",
        "inter_company_journal_entry_reference",
        "column_break98",
        "bill_no",
        "bill_date",
        "due_date",
        "write_off",
        "write_off_based_on",
        "get_outstanding_invoices",
        "column_break_30",
        "write_off_amount",
        "printing_settings",
        "pay_to_recd_from",
        "column_break_35",
        "letter_head",
        "select_print_heading",
        "addtional_info",
        "mode_of_payment",
        "payment_order",
        "column_break3",
        "is_opening",
        "stock_entry",
        "auto_repeat",
        "amended_from",
        "from_template",
        "tax_withholding_category",
        "apply_tds",
        "reversal_of",
        "process_deferred_accounting",
        "is_system_generated",
        "periodic_entry_difference_account",
        "section_break_tcvw",
        "for_all_stock_asset_accounts",
        "stock_asset_account",
        "column_break_wpau",
        "get_balance_for_periodic_accounting",
        "party_not_required",
        "section_tax_withholding_entry",
        "tax_withholding_group",
        "ignore_tax_withholding_threshold",
        "override_tax_withholding_entries",
        "tax_withholding_entries",
        "more_info_tab",
        "section_break_ouaq",
        "column_break_cixu",
        "column_break_oizh",
        "column_break_isfa",
        "tax_withholding_tab",
        "auto_repeat_section",
        "custom_remark",
    ];

    pub fn validate(
        &mut self,
        accounts: &HashMap<String, AccountMeta>,
        party_types: &HashMap<String, PartyTypeMeta>,
        references: &HashMap<(String, String), ReferenceDoc>,
    ) -> Result<(), JournalEntryError> {
        if self.voucher_type == "Opening Entry" {
            self.is_opening = "Yes".to_string();
        }
        if self.is_opening.is_empty() {
            self.is_opening = "No".to_string();
        }
        self.clearance_date = None;

        self.validate_empty_accounts_table()?;
        self.validate_party(accounts, party_types)?;
        self.validate_entries_for_advance()?;
        self.validate_multi_currency(accounts)?;
        self.set_amounts_in_company_currency();
        self.validate_debit_credit_amount()?;
        self.set_total_debit_credit()?;
        self.validate_reference_doc(references)?;
        self.set_against_account();
        self.create_remarks()?;
        self.set_print_format_fields(accounts);
        if self.title.is_none() {
            self.title = self.get_title();
        }
        Ok(())
    }

    pub fn validate_empty_accounts_table(&self) -> Result<(), JournalEntryError> {
        if self.accounts.is_empty() {
            Err(JournalEntryError::AccountsTableBlank)
        } else {
            Ok(())
        }
    }

    pub fn validate_party(
        &self,
        accounts: &HashMap<String, AccountMeta>,
        party_types: &HashMap<String, PartyTypeMeta>,
    ) -> Result<(), JournalEntryError> {
        for row in &self.accounts {
            let account_type = accounts
                .get(&row.account)
                .and_then(|account| account.account_type.as_deref())
                .or(row.account_type.as_deref());
            if matches!(account_type, Some("Receivable" | "Payable")) {
                if (row.party_type.is_none() || row.party.is_none()) && !self.party_not_required {
                    return Err(JournalEntryError::PartyRequired {
                        row: row.idx,
                        account: row.account.clone(),
                    });
                }
                if let Some(party_type) = row.party_type.as_deref() {
                    let party_account_type = party_types
                        .get(party_type)
                        .map(|meta| meta.account_type.as_str());
                    if party_account_type != account_type && party_type != "Employee" {
                        return Err(JournalEntryError::PartyAccountTypeMismatch {
                            row: row.idx,
                            account: row.account.clone(),
                            party_type: party_type.to_string(),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    pub fn validate_entries_for_advance(&self) -> Result<(), JournalEntryError> {
        for row in &self.accounts {
            if matches!(
                row.reference_type.as_deref(),
                Some("Sales Invoice" | "Purchase Invoice" | "Journal Entry")
            ) {
                continue;
            }

            if row.is_advance == "Yes" {
                if row.party_type.as_deref() == Some("Customer") && row.debit > 0.0 {
                    return Err(JournalEntryError::CustomerAdvanceMustBeCredit { row: row.idx });
                }
                if row.party_type.as_deref() == Some("Supplier") && row.credit > 0.0 {
                    return Err(JournalEntryError::SupplierAdvanceMustBeDebit { row: row.idx });
                }
            }
        }
        Ok(())
    }

    pub fn system_generated_gain_loss(&self) -> bool {
        self.voucher_type == "Exchange Gain Or Loss"
            && self.multi_currency
            && self.is_system_generated
    }

    pub fn validate_multi_currency(
        &mut self,
        accounts: &HashMap<String, AccountMeta>,
    ) -> Result<(), JournalEntryError> {
        let mut alternate_currency = BTreeSet::new();
        for row in &mut self.accounts {
            if let Some(account) = accounts.get(&row.account) {
                row.account_currency = account.account_currency.clone();
                row.account_type = account.account_type.clone();
            }
            if row.account_currency.is_none() {
                row.account_currency = Some(self.company_currency.clone());
            }
            if row.account_currency.as_deref() != Some(self.company_currency.as_str()) {
                alternate_currency.insert(row.account_currency.clone().unwrap_or_default());
            }
        }
        if !alternate_currency.is_empty() && !self.multi_currency {
            return Err(JournalEntryError::MultiCurrencyRequired);
        }
        self.set_exchange_rate()
    }

    pub fn set_exchange_rate(&mut self) -> Result<(), JournalEntryError> {
        for row in &mut self.accounts {
            if row.account_currency.as_deref() == Some(self.company_currency.as_str()) {
                row.exchange_rate = 1.0;
            }
            if row.exchange_rate == 0.0 {
                return Err(JournalEntryError::ExchangeRateMandatory { row: row.idx });
            }
        }
        Ok(())
    }

    pub fn set_amounts_in_company_currency(&mut self) {
        if self.voucher_type == "Exchange Gain Or Loss" && self.multi_currency {
            return;
        }
        for row in &mut self.accounts {
            row.debit = flt(row.debit_in_account_currency * row.exchange_rate);
            row.credit = flt(row.credit_in_account_currency * row.exchange_rate);
        }
    }

    pub fn validate_debit_credit_amount(&self) -> Result<(), JournalEntryError> {
        if self.voucher_type == "Exchange Gain Or Loss" && self.multi_currency {
            return Ok(());
        }
        for row in &self.accounts {
            if row.debit == 0.0 && row.credit == 0.0 {
                return Err(JournalEntryError::ZeroDebitCredit { row: row.idx });
            }
        }
        Ok(())
    }

    pub fn set_total_debit_credit(&mut self) -> Result<(), JournalEntryError> {
        self.total_debit = 0.0;
        self.total_credit = 0.0;
        self.difference = 0.0;

        for row in &self.accounts {
            if row.debit > 0.0 && row.credit > 0.0 {
                return Err(JournalEntryError::DebitAndCreditSameRow);
            }
            self.total_debit += flt(row.debit);
            self.total_credit += flt(row.credit);
        }
        self.total_debit = flt(self.total_debit);
        self.total_credit = flt(self.total_credit);
        self.difference = flt(self.total_debit - self.total_credit);
        Ok(())
    }

    pub fn validate_total_debit_and_credit(&self) -> Result<(), JournalEntryError> {
        if self.voucher_type == "Exchange Gain Or Loss" && self.multi_currency {
            return Ok(());
        }
        if self.difference != 0.0 {
            Err(JournalEntryError::DifferenceNotZero)
        } else {
            Ok(())
        }
    }

    pub fn validate_reference_doc(
        &mut self,
        references: &HashMap<(String, String), ReferenceDoc>,
    ) -> Result<(), JournalEntryError> {
        let mut totals: BTreeMap<String, f64> = BTreeMap::new();
        let mut types: BTreeMap<String, String> = BTreeMap::new();

        for row in &mut self.accounts {
            if row.reference_type.is_none() {
                row.reference_name = None;
            }
            if row.reference_name.is_none() {
                row.reference_type = None;
            }

            let (Some(reference_type), Some(reference_name)) =
                (row.reference_type.as_deref(), row.reference_name.as_deref())
            else {
                continue;
            };

            if !matches!(
                reference_type,
                "Sales Invoice" | "Purchase Invoice" | "Sales Order" | "Purchase Order"
            ) {
                continue;
            }

            if reference_type == "Sales Order" && row.debit > 0.0 {
                return Err(JournalEntryError::DebitLinkedWithSalesOrder { row: row.idx });
            }
            if reference_type == "Purchase Order" && row.credit > 0.0 {
                return Err(JournalEntryError::CreditLinkedWithPurchaseOrder { row: row.idx });
            }

            let reference = references
                .get(&(reference_type.to_string(), reference_name.to_string()))
                .ok_or_else(|| JournalEntryError::InvalidReference {
                    row: row.idx,
                    reference_name: reference_name.to_string(),
                })?;

            if matches!(reference_type, "Sales Invoice" | "Purchase Invoice")
                && self.voucher_type != "Exchange Gain Or Loss"
                && (row.party.as_deref() != Some(reference.party.as_str())
                    || row.account != reference.account)
            {
                return Err(JournalEntryError::PartyAccountMismatch {
                    row: row.idx,
                    reference_type: reference_type.to_string(),
                    reference_name: reference_name.to_string(),
                });
            }
            if matches!(reference_type, "Sales Order" | "Purchase Order")
                && row.party.as_deref() != Some(reference.party.as_str())
            {
                return Err(JournalEntryError::PartyAccountMismatch {
                    row: row.idx,
                    reference_type: reference_type.to_string(),
                    reference_name: reference_name.to_string(),
                });
            }

            let amount = if matches!(reference_type, "Sales Order" | "Sales Invoice") {
                row.credit_in_account_currency
            } else {
                row.debit_in_account_currency
            };
            if !matches!(
                self.voucher_type.as_str(),
                "Deferred Revenue" | "Deferred Expense"
            ) {
                *totals.entry(reference_name.to_string()).or_default() += amount;
            }
            types.insert(reference_name.to_string(), reference_type.to_string());
            row.reference_due_date = reference.due_date.clone();
        }

        for (reference_name, total) in totals {
            let reference_type = types.get(&reference_name).cloned().unwrap_or_default();
            let reference = references
                .get(&(reference_type.clone(), reference_name.clone()))
                .expect("reference already validated above");
            if matches!(reference_type.as_str(), "Sales Order" | "Purchase Order") {
                if reference.docstatus != 1 {
                    return Err(JournalEntryError::OrderNotSubmitted {
                        reference_type,
                        reference_name,
                    });
                }
                if reference.per_billed >= 100.0 {
                    return Err(JournalEntryError::OrderFullyBilled {
                        reference_type,
                        reference_name,
                    });
                }
                if reference.status.as_deref() == Some("Closed") {
                    return Err(JournalEntryError::OrderClosed {
                        reference_type,
                        reference_name,
                    });
                }
                if reference.grand_total < reference.advance_paid + total {
                    return Err(JournalEntryError::AdvanceGreaterThanGrandTotal {
                        reference_type,
                        reference_name,
                    });
                }
            } else if matches!(
                reference_type.as_str(),
                "Sales Invoice" | "Purchase Invoice"
            ) && !matches!(self.voucher_type.as_str(), "Debit Note" | "Credit Note")
            {
                if reference.docstatus != 1 {
                    return Err(JournalEntryError::OrderNotSubmitted {
                        reference_type,
                        reference_name,
                    });
                }
                if total != 0.0 && flt(reference.outstanding_amount) < flt(total) {
                    return Err(JournalEntryError::PaymentGreaterThanOutstanding {
                        reference_type,
                        reference_name,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn set_against_account(&mut self) {
        let mut accounts_debited = BTreeSet::new();
        let mut accounts_credited = BTreeSet::new();

        for row in &self.accounts {
            if row.debit > 0.0 {
                accounts_debited.insert(row.party.clone().unwrap_or_else(|| row.account.clone()));
            }
            if row.credit > 0.0 {
                accounts_credited.insert(row.party.clone().unwrap_or_else(|| row.account.clone()));
            }
        }

        let credited = join_set(&accounts_credited);
        let debited = join_set(&accounts_debited);
        for row in &mut self.accounts {
            if row.debit > 0.0 {
                row.against_account = Some(credited.clone());
            }
            if row.credit > 0.0 {
                row.against_account = Some(debited.clone());
            }
        }
    }

    pub fn create_remarks(&mut self) -> Result<(), JournalEntryError> {
        if self.custom_remark {
            return Ok(());
        }
        let mut remarks = Vec::new();
        if let Some(cheque_no) = self.cheque_no.as_deref() {
            let cheque_date = self
                .cheque_date
                .as_deref()
                .ok_or(JournalEntryError::BankEntryReferenceRequired)?;
            remarks.push(format!("Reference #{cheque_no} dated {cheque_date}"));
        }
        for row in &self.accounts {
            match (row.reference_type.as_deref(), row.credit, row.debit) {
                (Some("Sales Invoice"), credit, _) if credit > 0.0 => remarks.push(format!(
                    "{} against Sales Invoice {}",
                    fmt_money(credit, &self.company_currency),
                    row.reference_name.clone().unwrap_or_default()
                )),
                (Some("Sales Order"), credit, _) if credit > 0.0 => remarks.push(format!(
                    "{} against Sales Order {}",
                    fmt_money(credit, &self.company_currency),
                    row.reference_name.clone().unwrap_or_default()
                )),
                (Some("Purchase Invoice"), _, debit) if debit > 0.0 => remarks.push(format!(
                    "{} against Purchase Invoice {}",
                    fmt_money(debit, &self.company_currency),
                    row.reference_name.clone().unwrap_or_default()
                )),
                (Some("Purchase Order"), _, debit) if debit > 0.0 => remarks.push(format!(
                    "{} against Purchase Order {}",
                    fmt_money(debit, &self.company_currency),
                    row.reference_name.clone().unwrap_or_default()
                )),
                _ => {}
            }
        }
        if !remarks.is_empty() {
            self.remark = Some(remarks.join("\n"));
        }
        Ok(())
    }

    pub fn set_print_format_fields(&mut self, accounts: &HashMap<String, AccountMeta>) {
        let mut bank_amount = 0.0;
        let mut party_amount = 0.0;
        let mut currency = None;
        let mut bank_account_currency = None;
        let mut party_account_currency = None;
        let mut pay_to_recd_from = None;
        let mut party_type = None;

        for row in &self.accounts {
            if matches!(row.party_type.as_deref(), Some("Customer" | "Supplier"))
                && row.party.is_some()
            {
                party_type = row.party_type.clone();
                if pay_to_recd_from.is_none() {
                    pay_to_recd_from = row.party.clone();
                }
                if pay_to_recd_from == row.party {
                    party_amount += non_zero(
                        row.debit_in_account_currency,
                        row.credit_in_account_currency,
                    );
                    party_account_currency = row.account_currency.clone();
                }
            } else if matches!(
                accounts
                    .get(&row.account)
                    .and_then(|account| account.account_type.as_deref())
                    .or(row.account_type.as_deref()),
                Some("Bank" | "Cash")
            ) {
                bank_amount += non_zero(
                    row.debit_in_account_currency,
                    row.credit_in_account_currency,
                );
                bank_account_currency = row.account_currency.clone();
            }
        }

        if party_type.is_some() && pay_to_recd_from.is_some() {
            self.pay_to_recd_from = pay_to_recd_from;
            if bank_amount != 0.0 {
                self.total_amount = bank_amount;
                currency = bank_account_currency;
            } else {
                self.total_amount = party_amount;
                currency = party_account_currency;
            }
        }
        self.total_amount_currency = currency;
        self.total_amount_in_words = Some(format_amount(self.total_amount));
    }

    pub fn get_title(&self) -> Option<String> {
        self.accounts
            .iter()
            .find_map(|row| row.party.clone())
            .or_else(|| self.accounts.first().map(|row| row.account.clone()))
    }

    pub fn get_balance(
        &mut self,
        difference_account: Option<&str>,
        default_cost_center: Option<&str>,
    ) -> Result<(), JournalEntryError> {
        self.validate_empty_accounts_table()?;
        let diff = flt(self.difference);
        if diff != 0.0 {
            let blank_index = self.accounts.iter().position(|row| {
                row.credit_in_account_currency == 0.0 && row.debit_in_account_currency == 0.0
            });
            let index = if let Some(index) = blank_index {
                index
            } else {
                self.accounts.push(JournalEntryAccountRow {
                    idx: self.accounts.len() + 1,
                    account: difference_account.unwrap_or_default().to_string(),
                    cost_center: default_cost_center.map(str::to_string),
                    account_currency: Some(self.company_currency.clone()),
                    ..Default::default()
                });
                self.accounts.len() - 1
            };
            let row = &mut self.accounts[index];
            row.exchange_rate = 1.0;
            if diff > 0.0 {
                row.credit_in_account_currency = diff;
                row.credit = diff;
            } else {
                row.debit_in_account_currency = diff.abs();
                row.debit = diff.abs();
            }
        }
        self.set_total_debit_credit()?;
        self.validate_total_debit_and_credit()
    }

    pub fn build_gl_map(&self) -> Vec<GlMapRow> {
        let mut transaction_currency = self.company_currency.clone();
        let mut transaction_exchange_rate = 1.0;
        if self.multi_currency {
            if let Some(row) = self
                .accounts
                .iter()
                .find(|row| row.account_currency.as_deref() != Some(self.company_currency.as_str()))
            {
                transaction_currency = row.account_currency.clone().unwrap_or_default();
                transaction_exchange_rate = row.exchange_rate;
            }
        }

        self.accounts
            .iter()
            .filter(|row| {
                row.debit != 0.0
                    || row.credit != 0.0
                    || self.voucher_type == "Exchange Gain Or Loss"
            })
            .map(|row| {
                let account_currency = row
                    .account_currency
                    .clone()
                    .unwrap_or_else(|| self.company_currency.clone());
                let mut against_voucher_type = row.reference_type.clone();
                let mut against_voucher = row.reference_name.clone();
                let mut advance_voucher_type = row.advance_voucher_type.clone();
                let mut advance_voucher_no = row.advance_voucher_no.clone();
                if matches!(
                    row.reference_type.as_deref(),
                    Some("Sales Order" | "Purchase Order" | "Employee Advance")
                ) {
                    against_voucher_type = Some(Self::DOCTYPE.to_string());
                    against_voucher = self.name.clone();
                    advance_voucher_type = row.reference_type.clone();
                    advance_voucher_no = row.reference_name.clone();
                }
                let remarks = [row.user_remark.clone(), self.remark.clone()]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                GlMapRow {
                    account: row.account.clone(),
                    party_type: row.party_type.clone(),
                    due_date: self.due_date.clone(),
                    party: row.party.clone(),
                    against: row.against_account.clone(),
                    debit: flt(row.debit),
                    credit: flt(row.credit),
                    account_currency: account_currency.clone(),
                    debit_in_account_currency: flt(row.debit_in_account_currency),
                    credit_in_account_currency: flt(row.credit_in_account_currency),
                    transaction_currency: transaction_currency.clone(),
                    transaction_exchange_rate,
                    debit_in_transaction_currency: if transaction_currency == account_currency {
                        flt(row.debit_in_account_currency)
                    } else {
                        flt(row.debit / transaction_exchange_rate)
                    },
                    credit_in_transaction_currency: if transaction_currency == account_currency {
                        flt(row.credit_in_account_currency)
                    } else {
                        flt(row.credit / transaction_exchange_rate)
                    },
                    against_voucher_type,
                    against_voucher,
                    remarks: if remarks.is_empty() {
                        None
                    } else {
                        Some(remarks.join("\n"))
                    },
                    voucher_detail_no: row.reference_detail_no.clone(),
                    cost_center: row.cost_center.clone(),
                    project: row.project.clone(),
                    finance_book: self.finance_book.clone(),
                    advance_voucher_type,
                    advance_voucher_no,
                }
            })
            .collect()
    }
}

impl DocumentController for JournalEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "validate",
            "before_submit",
            "on_submit",
            "on_update_after_submit",
            "before_cancel",
            "on_cancel",
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentEntryError {
    OrderAlreadyBilled,
    MissingPartyAccount,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankCashAccount {
    pub account: String,
    pub account_currency: String,
    pub account_type: String,
    pub balance: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderRef {
    pub doctype: String,
    pub name: String,
    pub company: String,
    pub company_currency: String,
    pub party: String,
    pub grand_total: f64,
    pub base_grand_total: f64,
    pub advance_paid: f64,
    pub per_billed: f64,
    pub transaction_date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvoiceRef {
    pub doctype: String,
    pub name: String,
    pub company: String,
    pub company_currency: String,
    pub party: String,
    pub party_account: String,
    pub party_account_currency: String,
    pub outstanding_amount: f64,
    pub conversion_rate: f64,
    pub remarks: String,
    pub posting_date: String,
}

pub fn make_payment_entry_against_order(
    order: &OrderRef,
    amount: Option<f64>,
    party_accounts: &HashMap<(String, String), (String, String)>,
    bank_account: Option<BankCashAccount>,
) -> Result<JournalEntry, PaymentEntryError> {
    if flt(order.per_billed) > 0.0 {
        return Err(PaymentEntryError::OrderAlreadyBilled);
    }
    let (party_type, amount_field_party_is_credit) = if order.doctype == "Sales Order" {
        ("Customer", true)
    } else {
        ("Supplier", false)
    };
    let (party_account, party_account_currency) = party_accounts
        .get(&(party_type.to_string(), order.party.clone()))
        .cloned()
        .ok_or(PaymentEntryError::MissingPartyAccount)?;
    let amount = amount.unwrap_or_else(|| {
        if party_account_currency == order.company_currency {
            order.base_grand_total - order.advance_paid
        } else {
            order.grand_total - order.advance_paid
        }
    });
    Ok(get_payment_entry(
        &PaymentRef {
            doctype: order.doctype.clone(),
            name: order.name.clone(),
            company: order.company.clone(),
            company_currency: order.company_currency.clone(),
            party: order.party.clone(),
            cost_center: None,
            posting_date: order.transaction_date.clone(),
        },
        PaymentEntryArgs {
            party_type: party_type.to_string(),
            party_account,
            party_account_currency,
            amount_field_party_is_credit,
            amount,
            debit_in_account_currency: None,
            remarks: format!(
                "Advance Payment received against {} {}",
                order.doctype, order.name
            ),
            is_advance: "Yes".to_string(),
            bank_account,
        },
    ))
}

pub fn make_payment_entry_against_invoice(
    invoice: &InvoiceRef,
    amount: Option<f64>,
    bank_account: Option<BankCashAccount>,
) -> Result<JournalEntry, PaymentEntryError> {
    let (party_type, party_amount_is_credit) = if invoice.doctype == "Sales Invoice" {
        ("Customer", invoice.outstanding_amount > 0.0)
    } else {
        ("Supplier", invoice.outstanding_amount <= 0.0)
    };
    Ok(get_payment_entry(
        &PaymentRef {
            doctype: invoice.doctype.clone(),
            name: invoice.name.clone(),
            company: invoice.company.clone(),
            company_currency: invoice.company_currency.clone(),
            party: invoice.party.clone(),
            cost_center: None,
            posting_date: invoice.posting_date.clone(),
        },
        PaymentEntryArgs {
            party_type: party_type.to_string(),
            party_account: invoice.party_account.clone(),
            party_account_currency: invoice.party_account_currency.clone(),
            amount_field_party_is_credit: party_amount_is_credit,
            amount: amount.unwrap_or_else(|| invoice.outstanding_amount.abs()),
            debit_in_account_currency: None,
            remarks: format!(
                "Payment received against {} {}. {}",
                invoice.doctype, invoice.name, invoice.remarks
            ),
            is_advance: "No".to_string(),
            bank_account,
        },
    ))
}

#[derive(Clone, Debug, PartialEq)]
struct PaymentRef {
    doctype: String,
    name: String,
    company: String,
    company_currency: String,
    party: String,
    cost_center: Option<String>,
    posting_date: String,
}

#[derive(Clone, Debug, PartialEq)]
struct PaymentEntryArgs {
    party_type: String,
    party_account: String,
    party_account_currency: String,
    amount_field_party_is_credit: bool,
    amount: f64,
    debit_in_account_currency: Option<f64>,
    remarks: String,
    is_advance: String,
    bank_account: Option<BankCashAccount>,
}

fn get_payment_entry(ref_doc: &PaymentRef, args: PaymentEntryArgs) -> JournalEntry {
    let mut je = JournalEntry {
        voucher_type: "Bank Entry".to_string(),
        company: ref_doc.company.clone(),
        company_currency: ref_doc.company_currency.clone(),
        posting_date: ref_doc.posting_date.clone(),
        remark: Some(args.remarks),
        ..Default::default()
    };

    let mut party_row = JournalEntryAccountRow {
        idx: 1,
        account: args.party_account,
        party_type: Some(args.party_type.clone()),
        party: Some(ref_doc.party.clone()),
        cost_center: ref_doc.cost_center.clone(),
        account_currency: Some(args.party_account_currency.clone()),
        exchange_rate: 1.0,
        is_advance: args.is_advance,
        reference_type: Some(ref_doc.doctype.clone()),
        reference_name: Some(ref_doc.name.clone()),
        ..Default::default()
    };
    if args.amount_field_party_is_credit {
        party_row.credit_in_account_currency = args.amount;
    } else {
        party_row.debit_in_account_currency = args.amount;
    }

    let mut bank_row = JournalEntryAccountRow {
        idx: 2,
        cost_center: ref_doc.cost_center.clone(),
        exchange_rate: 1.0,
        ..Default::default()
    };
    if let Some(bank) = args.bank_account {
        bank_row.account = bank.account;
        bank_row.account_currency = Some(bank.account_currency);
        bank_row.account_type = Some(bank.account_type);
    }
    let bank_amount = args.debit_in_account_currency.unwrap_or(args.amount);
    if args.amount_field_party_is_credit {
        bank_row.debit_in_account_currency = bank_amount;
    } else {
        bank_row.credit_in_account_currency = bank_amount;
    }

    je.accounts = vec![party_row, bank_row];
    if je
        .accounts
        .iter()
        .any(|row| row.account_currency.as_deref() != Some(ref_doc.company_currency.as_str()))
    {
        je.multi_currency = true;
    }
    je.set_amounts_in_company_currency();
    let _ = je.set_total_debit_credit();
    je
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutstandingArgs {
    pub doctype: String,
    pub docname: String,
    pub account: String,
    pub party: Option<String>,
    pub account_currency: String,
    pub company: String,
}

pub fn get_outstanding(
    args: &OutstandingArgs,
    journal_amounts: &BTreeMap<String, f64>,
    invoices: &HashMap<(String, String), ReferenceDoc>,
) -> Option<HashMap<String, String>> {
    if args.doctype == "Journal Entry" {
        let amount = *journal_amounts.get(&args.docname).unwrap_or(&0.0);
        let amount_field = if amount > 0.0 {
            "credit_in_account_currency"
        } else {
            "debit_in_account_currency"
        };
        return Some(HashMap::from([(
            amount_field.to_string(),
            format_amount(amount.abs()),
        )]));
    }

    if matches!(args.doctype.as_str(), "Sales Invoice" | "Purchase Invoice") {
        let invoice = invoices.get(&(args.doctype.clone(), args.docname.clone()))?;
        let party_type = if args.doctype == "Sales Invoice" {
            "Customer"
        } else {
            "Supplier"
        };
        let exchange_rate = if args.account_currency != invoice.company_currency {
            invoice.conversion_rate
        } else {
            1.0
        };
        let amount_field = if args.doctype == "Sales Invoice" {
            if invoice.outstanding_amount > 0.0 {
                "credit_in_account_currency"
            } else {
                "debit_in_account_currency"
            }
        } else if invoice.outstanding_amount > 0.0 {
            "debit_in_account_currency"
        } else {
            "credit_in_account_currency"
        };
        return Some(HashMap::from([
            (
                amount_field.to_string(),
                format_amount(invoice.outstanding_amount.abs()),
            ),
            ("exchange_rate".to_string(), format_amount(exchange_rate)),
            ("party_type".to_string(), party_type.to_string()),
            ("party".to_string(), invoice.party.clone()),
            (
                "reference_due_date".to_string(),
                invoice.due_date.clone().unwrap_or_default(),
            ),
        ]));
    }
    None
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountDetails {
    pub account_type: String,
    pub account_currency: Option<String>,
    pub bank_account: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountDetailsResult {
    pub party_type: Option<String>,
    pub account_type: String,
    pub account_currency: String,
    pub bank_account: Option<String>,
    pub exchange_rate: f64,
    pub party: Option<String>,
}

pub fn get_account_details_and_party_type(
    account: &str,
    _date: &str,
    company_currency: &str,
    account_details: &HashMap<String, AccountDetails>,
) -> Option<AccountDetailsResult> {
    let details = account_details.get(account)?;
    let party_type = match details.account_type.as_str() {
        "Receivable" => Some("Customer".to_string()),
        "Payable" => Some("Supplier".to_string()),
        _ => None,
    };
    Some(AccountDetailsResult {
        party_type: party_type.clone(),
        account_type: details.account_type.clone(),
        account_currency: details
            .account_currency
            .clone()
            .unwrap_or_else(|| company_currency.to_string()),
        bank_account: details.bank_account.clone(),
        exchange_rate: 1.0,
        party: if party_type.is_none() {
            Some(String::new())
        } else {
            None
        },
    })
}

pub fn validate_stock_account_transaction(
    perpetual_inventory_enabled: bool,
    voucher_type: &str,
    account: &str,
    account_balance: f64,
    stock_balance: f64,
) -> Result<(), JournalEntryError> {
    if !perpetual_inventory_enabled || voucher_type == "Periodic Accounting Entry" {
        return Ok(());
    }
    if flt(account_balance) == flt(stock_balance) {
        return Err(JournalEntryError::StockAccountInvalidTransaction {
            account: account.to_string(),
        });
    }
    Ok(())
}

pub fn check_customer_credit_limits(
    journal_entry: &JournalEntry,
    credit_limits: &HashMap<String, f64>,
    customer_outstanding: &HashMap<String, f64>,
    bypass_credit_limit_check: &HashMap<String, bool>,
) -> Result<(), JournalEntryError> {
    let mut customers = journal_entry
        .accounts
        .iter()
        .filter(|row| {
            row.party_type.as_deref() == Some("Customer")
                && row.party.as_deref().is_some_and(|party| !party.is_empty())
                && flt(row.debit) > 0.0
        })
        .filter_map(|row| row.party.clone())
        .collect::<Vec<_>>();
    customers.sort();
    customers.dedup();

    for customer in customers {
        if bypass_credit_limit_check
            .get(&customer)
            .copied()
            .unwrap_or(false)
        {
            continue;
        }
        let credit_limit = credit_limits.get(&customer).copied().unwrap_or(0.0);
        if credit_limit == 0.0 {
            continue;
        }
        let outstanding = customer_outstanding.get(&customer).copied().unwrap_or(0.0);
        if credit_limit > 0.0 && flt(outstanding) > credit_limit {
            return Err(JournalEntryError::CreditLimitCrossed {
                customer,
                outstanding: format_amount(flt(outstanding)),
                credit_limit: format_amount(credit_limit),
            });
        }
    }

    Ok(())
}

pub fn make_inter_company_journal_entry(
    name: &str,
    voucher_type: &str,
    company: &str,
) -> JournalEntry {
    JournalEntry {
        voucher_type: voucher_type.to_string(),
        company: company.to_string(),
        inter_company_journal_entry_reference: Some(name.to_string()),
        ..Default::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReverseEntryError {
    ReverseAlreadyExists,
}

pub fn make_reverse_journal_entry(
    source: &JournalEntry,
    existing_reverse: bool,
) -> Result<JournalEntry, ReverseEntryError> {
    if existing_reverse {
        return Err(ReverseEntryError::ReverseAlreadyExists);
    }
    let mut target = source.clone();
    target.reversal_of = source.name.clone();
    target.accounts = source
        .accounts
        .iter()
        .cloned()
        .map(|mut row| {
            std::mem::swap(
                &mut row.debit_in_account_currency,
                &mut row.credit_in_account_currency,
            );
            std::mem::swap(&mut row.debit, &mut row.credit);
            row
        })
        .collect();
    Ok(target)
}

fn flt(value: f64) -> f64 {
    let rounded = (value * 1_000_000_000.0).round() / 1_000_000_000.0;
    if rounded == -0.0 {
        0.0
    } else {
        rounded
    }
}

fn non_zero(first: f64, second: f64) -> f64 {
    if first != 0.0 {
        first
    } else {
        second
    }
}

fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn fmt_money(value: f64, currency: &str) -> String {
    format!("{} {}", format_amount(value), currency)
}

fn format_amount(value: f64) -> String {
    let value = flt(value);
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}
