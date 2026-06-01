use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, PartialEq)]
pub struct ExchangeRateRevaluation {
    pub name: Option<String>,
    pub company: Option<String>,
    pub posting_date: Option<String>,
    pub rounding_loss_allowance: f64,
    pub accounts: Vec<ExchangeRateRevaluationRow>,
    pub gain_loss_unbooked: f64,
    pub gain_loss_booked: f64,
    pub total_gain_loss: f64,
    pub amended_from: Option<String>,
    pub ignore_linked_doctypes: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeRateRevaluationRow {
    pub account: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub account_currency: String,
    pub balance_in_account_currency: f64,
    pub new_balance_in_account_currency: f64,
    pub balance_in_base_currency: f64,
    pub new_balance_in_base_currency: f64,
    pub current_exchange_rate: f64,
    pub new_exchange_rate: f64,
    pub gain_loss: f64,
    pub zero_balance: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RevaluationAccountBalance {
    pub account: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub account_currency: String,
    pub balance_in_account_currency: f64,
    pub balance: f64,
    pub zero_balance: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RevaluationAccountDetail {
    pub account_currency: Option<String>,
    pub balance_in_base_currency: Option<f64>,
    pub balance_in_account_currency: Option<f64>,
    pub current_exchange_rate: Option<f64>,
    pub new_exchange_rate: Option<f64>,
    pub new_balance_in_base_currency: Option<f64>,
    pub new_balance_in_account_currency: Option<f64>,
    pub zero_balance: Option<bool>,
    pub gain_loss: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeRateRevaluationGlEntry {
    pub company: String,
    pub account: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub account_currency: String,
    pub posting_date: String,
    pub is_cancelled: bool,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub debit: f64,
    pub credit: f64,
    pub voucher_type: String,
    pub voucher_no: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExchangeRateRevaluationJournalEntryAccount {
    pub parent: String,
    pub reference_type: String,
    pub reference_name: String,
    pub docstatus: i32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeRateRevaluationJournalEntryPlan {
    pub voucher_type: String,
    pub company: Option<String>,
    pub posting_date: Option<String>,
    pub multi_currency: bool,
    pub accounts: Vec<ExchangeRateRevaluationJournalAccount>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeRateRevaluationJournalAccount {
    pub account: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub account_currency: Option<String>,
    pub balance: f64,
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub exchange_rate: f64,
    pub cost_center: Option<String>,
    pub reference_type: Option<String>,
    pub reference_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExchangeRateRevaluationError {
    RoundingLossAllowanceOutOfRange,
    MissingCompanyOrPostingDate,
    NoAccountsWithGainLoss,
    MissingUnrealizedGainLossAccount { company: String },
    MissingPartyForAccountType { account_type: String },
}

impl Default for ExchangeRateRevaluation {
    fn default() -> Self {
        Self {
            rounding_loss_allowance: 0.05,
            ..Self::empty()
        }
    }
}

impl ExchangeRateRevaluation {
    pub const DOCTYPE: &'static str = "Exchange Rate Revaluation";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-ERR-.YYYY.-.#####";
    pub const IS_SUBMITTABLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 13] = [
        "posting_date",
        "rounding_loss_allowance",
        "column_break_2",
        "company",
        "section_break_4",
        "get_entries",
        "accounts",
        "section_break_6",
        "gain_loss_unbooked",
        "gain_loss_booked",
        "column_break_10",
        "total_gain_loss",
        "amended_from",
    ];

    fn empty() -> Self {
        Self {
            name: None,
            company: None,
            posting_date: None,
            rounding_loss_allowance: 0.0,
            accounts: Vec::new(),
            gain_loss_unbooked: 0.0,
            gain_loss_booked: 0.0,
            total_gain_loss: 0.0,
            amended_from: None,
            ignore_linked_doctypes: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .required()
                .in_list_view(),
            FieldSpec::float("rounding_loss_allowance", "Rounding Loss Allowance")
                .default("0.05")
                .description("Only values between [0,1) are allowed. Like {0.00, 0.04, 0.09, ...}\nEx: If allowance is set at 0.07, accounts that have balance of 0.07 in either of the currencies will be considered as zero balance account")
                .precision("9"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::button("get_entries", "Get Entries"),
            FieldSpec::table("accounts", "Exchange Rate Revaluation Account")
                .options("Exchange Rate Revaluation Account")
                .required()
                .no_copy(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::currency("gain_loss_unbooked", "Gain/Loss from Revaluation")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::currency("gain_loss_booked", "Gain/Loss already booked")
                .options("Company:company:default_currency")
                .description("Gain/Loss accumulated in foreign currency account. Accounts with '0' balance in either Base or Account currency")
                .read_only(),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::currency("total_gain_loss", "Total Gain/Loss")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Exchange Rate Revaluation")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(&mut self) -> Result<(), ExchangeRateRevaluationError> {
        self.validate_rounding_loss_allowance()?;
        self.set_total_gain_loss();
        Ok(())
    }

    pub fn validate_rounding_loss_allowance(&self) -> Result<(), ExchangeRateRevaluationError> {
        if self.rounding_loss_allowance < 0.0 || self.rounding_loss_allowance >= 1.0 {
            return Err(ExchangeRateRevaluationError::RoundingLossAllowanceOutOfRange);
        }

        Ok(())
    }

    pub fn set_total_gain_loss(&mut self) {
        let mut total_gain_loss = 0.0;
        let mut gain_loss_booked = 0.0;
        let mut gain_loss_unbooked = 0.0;

        for row in &mut self.accounts {
            if !row.zero_balance {
                row.gain_loss =
                    flt(row.new_balance_in_base_currency, 2) - flt(row.balance_in_base_currency, 2);
            }

            if row.zero_balance {
                gain_loss_booked += flt(row.gain_loss, 2);
            } else {
                gain_loss_unbooked += flt(row.gain_loss, 2);
            }

            total_gain_loss += flt(row.gain_loss, 2);
        }

        self.gain_loss_booked = gain_loss_booked;
        self.gain_loss_unbooked = gain_loss_unbooked;
        self.total_gain_loss = flt(total_gain_loss, 2);
    }

    pub fn validate_mandatory(&self) -> Result<(), ExchangeRateRevaluationError> {
        if self.company.as_deref().unwrap_or_default().is_empty()
            || self.posting_date.as_deref().unwrap_or_default().is_empty()
        {
            return Err(ExchangeRateRevaluationError::MissingCompanyOrPostingDate);
        }

        Ok(())
    }

    pub fn before_submit(&mut self) -> Result<(), ExchangeRateRevaluationError> {
        self.remove_accounts_without_gain_loss()
    }

    pub fn remove_accounts_without_gain_loss(
        &mut self,
    ) -> Result<(), ExchangeRateRevaluationError> {
        self.accounts.retain(|account| account.gain_loss != 0.0);

        if self.accounts.is_empty() {
            return Err(ExchangeRateRevaluationError::NoAccountsWithGainLoss);
        }

        Ok(())
    }

    pub fn on_cancel(&mut self) {
        self.ignore_linked_doctypes = Some("GL Entry".to_string());
    }

    pub fn check_journal_entry_condition(
        &self,
        journal_accounts: &[ExchangeRateRevaluationJournalEntryAccount],
        gl_entries: &[ExchangeRateRevaluationGlEntry],
        exchange_gain_loss_account: &str,
    ) -> bool {
        let Some(name) = self.name.as_deref() else {
            return true;
        };
        let journals = journal_accounts
            .iter()
            .filter(|row| {
                row.reference_type == "Exchange Rate Revaluation"
                    && row.reference_name == name
                    && row.docstatus == 1
            })
            .map(|row| row.parent.as_str())
            .collect::<BTreeSet<_>>();

        if journals.is_empty() {
            return true;
        }

        let total_amount = gl_entries
            .iter()
            .filter(|row| {
                row.voucher_type == "Journal Entry"
                    && journals.contains(row.voucher_no.as_str())
                    && row.account == exchange_gain_loss_account
                    && !row.is_cancelled
            })
            .map(|row| row.credit - row.debit)
            .sum::<f64>();

        total_amount != self.total_gain_loss
    }

    pub fn fetch_and_calculate_accounts_data(&mut self, accounts: Vec<ExchangeRateRevaluationRow>) {
        self.accounts.extend(
            accounts
                .into_iter()
                .filter(|account| account.gain_loss != 0.0),
        );
    }

    pub fn get_for_unrealized_gain_loss_account(
        &self,
        account: Option<&str>,
    ) -> Result<String, ExchangeRateRevaluationError> {
        if let Some(account) = account.filter(|account| !account.is_empty()) {
            return Ok(account.to_string());
        }

        Err(
            ExchangeRateRevaluationError::MissingUnrealizedGainLossAccount {
                company: self.company.clone().unwrap_or_default(),
            },
        )
    }

    pub fn get_account_balance_from_gle(
        accounts: &[String],
        posting_date: &str,
        party_type: Option<&str>,
        party: Option<&str>,
        rounding_loss_allowance: f64,
        currency_precision: u32,
        gl_entries: &[ExchangeRateRevaluationGlEntry],
    ) -> Vec<RevaluationAccountBalance> {
        let account_set = accounts.iter().map(String::as_str).collect::<BTreeSet<_>>();
        let mut grouped: BTreeMap<
            (String, Option<String>, Option<String>),
            RevaluationAccountBalance,
        > = BTreeMap::new();

        for entry in gl_entries {
            if !account_set.contains(entry.account.as_str())
                || entry.posting_date.as_str() > posting_date
                || entry.is_cancelled
                || party_type.is_some_and(|expected| entry.party_type.as_deref() != Some(expected))
                || party.is_some_and(|expected| entry.party.as_deref() != Some(expected))
            {
                continue;
            }

            let key = (
                entry.account.clone(),
                normalize_optional_text(entry.party_type.as_deref()),
                normalize_optional_text(entry.party.as_deref()),
            );
            let row = grouped
                .entry(key)
                .or_insert_with(|| RevaluationAccountBalance {
                    account: entry.account.clone(),
                    party_type: normalize_optional_text(entry.party_type.as_deref()),
                    party: normalize_optional_text(entry.party.as_deref()),
                    account_currency: entry.account_currency.clone(),
                    ..Default::default()
                });
            row.balance_in_account_currency +=
                entry.debit_in_account_currency - entry.credit_in_account_currency;
            row.balance += entry.debit - entry.credit;
        }

        let mut rows = grouped
            .into_values()
            .filter_map(|mut row| {
                let raw_balance_in_account_currency = row.balance_in_account_currency;
                let raw_balance = row.balance;
                if raw_balance == raw_balance_in_account_currency
                    || (raw_balance_in_account_currency == 0.0 && raw_balance == 0.0)
                {
                    return None;
                }

                row.balance_in_account_currency =
                    flt(row.balance_in_account_currency, currency_precision);
                if row.balance_in_account_currency.abs() <= rounding_loss_allowance {
                    row.balance_in_account_currency = 0.0;
                }
                row.balance = flt(row.balance, currency_precision);
                if row.balance.abs() <= rounding_loss_allowance {
                    row.balance = 0.0;
                }
                row.zero_balance = row.balance == 0.0 || row.balance_in_account_currency == 0.0;
                Some(row)
            })
            .collect::<Vec<_>>();
        rows.sort_by(|left, right| left.account.cmp(&right.account));
        rows
    }

    pub fn calculate_new_account_balance(
        company_currency: &str,
        account_details: &[RevaluationAccountBalance],
        exchange_rates: &BTreeMap<String, f64>,
        last_exchange_rates: &BTreeMap<String, f64>,
        precision: u32,
    ) -> Vec<ExchangeRateRevaluationRow> {
        let mut accounts = Vec::new();

        for detail in account_details.iter().filter(|row| !row.zero_balance) {
            let current_exchange_rate = if detail.balance_in_account_currency != 0.0 {
                detail.balance / detail.balance_in_account_currency
            } else {
                0.0
            };
            let new_exchange_rate = exchange_rates
                .get(&detail.account_currency)
                .copied()
                .unwrap_or_else(|| {
                    if detail.account_currency == company_currency {
                        1.0
                    } else {
                        0.0
                    }
                });
            let new_balance_in_base_currency = flt(
                detail.balance_in_account_currency * new_exchange_rate,
                precision,
            );
            let gain_loss =
                flt(new_balance_in_base_currency, precision) - flt(detail.balance, precision);

            accounts.push(ExchangeRateRevaluationRow {
                account: detail.account.clone(),
                party_type: detail.party_type.clone(),
                party: detail.party.clone(),
                account_currency: detail.account_currency.clone(),
                balance_in_base_currency: detail.balance,
                balance_in_account_currency: detail.balance_in_account_currency,
                zero_balance: detail.zero_balance,
                current_exchange_rate,
                new_exchange_rate,
                new_balance_in_base_currency,
                new_balance_in_account_currency: detail.balance_in_account_currency,
                gain_loss,
            });
        }

        for detail in account_details.iter().filter(|row| row.zero_balance) {
            let (
                current_exchange_rate,
                new_exchange_rate,
                new_balance_in_account_currency,
                new_balance_in_base_currency,
                gain_loss,
            ) = if detail.balance != 0.0 {
                (
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    flt(0.0, precision) - flt(detail.balance, precision),
                )
            } else {
                let current_exchange_rate = last_exchange_rates
                    .get(&last_exchange_rate_key(
                        &detail.account,
                        detail.party_type.as_deref(),
                        detail.party.as_deref(),
                    ))
                    .copied()
                    .unwrap_or(0.0);
                (
                    current_exchange_rate,
                    0.0,
                    0.0,
                    0.0,
                    0.0 - current_exchange_rate * detail.balance_in_account_currency,
                )
            };

            accounts.push(ExchangeRateRevaluationRow {
                account: detail.account.clone(),
                party_type: detail.party_type.clone(),
                party: detail.party.clone(),
                account_currency: detail.account_currency.clone(),
                balance_in_base_currency: detail.balance,
                balance_in_account_currency: detail.balance_in_account_currency,
                zero_balance: detail.zero_balance,
                current_exchange_rate,
                new_exchange_rate,
                new_balance_in_base_currency,
                new_balance_in_account_currency,
                gain_loss,
            });
        }

        accounts
    }

    pub fn make_jv_for_zero_balance_plan(
        &self,
        unrealized_exchange_gain_loss_account: &str,
        default_cost_center: &str,
    ) -> Option<ExchangeRateRevaluationJournalEntryPlan> {
        if self.gain_loss_booked == 0.0 {
            return None;
        }

        let accounts = self
            .accounts
            .iter()
            .filter(|account| account.zero_balance)
            .collect::<Vec<_>>();
        if accounts.is_empty() {
            return None;
        }

        let mut journal_entry_accounts = Vec::new();
        for account in accounts {
            let mut journal_account =
                self.journal_account_from_row(account, default_cost_center, 0.0);
            if account.balance_in_account_currency != 0.0
                && account.new_balance_in_account_currency == 0.0
            {
                if account.balance_in_account_currency > 0.0 {
                    journal_account.credit_in_account_currency =
                        flt(account.balance_in_account_currency.abs(), 2);
                } else {
                    journal_account.debit_in_account_currency =
                        flt(account.balance_in_account_currency.abs(), 2);
                }
                journal_entry_accounts.push(journal_account);
                journal_entry_accounts.push(unrealized_journal_account(
                    unrealized_exchange_gain_loss_account,
                    default_cost_center,
                    self.name.as_deref(),
                    0.0,
                    0.0,
                    if account.gain_loss < 0.0 {
                        account.gain_loss.abs()
                    } else {
                        0.0
                    },
                    if account.gain_loss > 0.0 {
                        account.gain_loss.abs()
                    } else {
                        0.0
                    },
                ));
            } else if account.balance_in_base_currency != 0.0
                && account.new_balance_in_base_currency == 0.0
            {
                if account.balance_in_base_currency > 0.0 {
                    journal_account.credit = flt(account.balance_in_base_currency.abs(), 2);
                } else {
                    journal_account.debit = flt(account.balance_in_base_currency.abs(), 2);
                }
                journal_entry_accounts.push(journal_account);
                journal_entry_accounts.push(unrealized_journal_account(
                    unrealized_exchange_gain_loss_account,
                    default_cost_center,
                    self.name.as_deref(),
                    if account.gain_loss < 0.0 {
                        account.gain_loss.abs()
                    } else {
                        0.0
                    },
                    if account.gain_loss > 0.0 {
                        account.gain_loss.abs()
                    } else {
                        0.0
                    },
                    0.0,
                    0.0,
                ));
            }
        }

        Some(ExchangeRateRevaluationJournalEntryPlan {
            voucher_type: "Exchange Gain Or Loss".to_string(),
            company: self.company.clone(),
            posting_date: self.posting_date.clone(),
            multi_currency: true,
            accounts: journal_entry_accounts,
        })
    }

    pub fn make_jv_for_revaluation_plan(
        &self,
        unrealized_exchange_gain_loss_account: &str,
        default_cost_center: &str,
        journal_difference: f64,
    ) -> Option<ExchangeRateRevaluationJournalEntryPlan> {
        if self.gain_loss_unbooked == 0.0 {
            return None;
        }

        let accounts = self
            .accounts
            .iter()
            .filter(|account| !account.zero_balance)
            .collect::<Vec<_>>();
        if accounts.is_empty() {
            return None;
        }

        let mut journal_entry_accounts = Vec::new();
        for account in accounts {
            if flt(account.balance_in_account_currency, 2) == 0.0 {
                continue;
            }

            let debit_first = account.balance_in_account_currency > 0.0;
            let mut new_rate_row = self.journal_account_from_row(
                account,
                default_cost_center,
                account.new_exchange_rate,
            );
            if debit_first {
                new_rate_row.debit_in_account_currency =
                    flt(account.balance_in_account_currency.abs(), 2);
            } else {
                new_rate_row.credit_in_account_currency =
                    flt(account.balance_in_account_currency.abs(), 2);
            }
            journal_entry_accounts.push(new_rate_row);

            let mut current_rate_row = self.journal_account_from_row(
                account,
                default_cost_center,
                account.current_exchange_rate,
            );
            if debit_first {
                current_rate_row.credit_in_account_currency =
                    flt(account.balance_in_account_currency.abs(), 2);
            } else {
                current_rate_row.debit_in_account_currency =
                    flt(account.balance_in_account_currency.abs(), 2);
            }
            journal_entry_accounts.push(current_rate_row);
        }

        let gain_loss_unbooked = journal_difference;
        journal_entry_accounts.push(unrealized_journal_account(
            unrealized_exchange_gain_loss_account,
            default_cost_center,
            self.name.as_deref(),
            0.0,
            0.0,
            if gain_loss_unbooked < 0.0 {
                gain_loss_unbooked.abs()
            } else {
                0.0
            },
            if gain_loss_unbooked > 0.0 {
                gain_loss_unbooked
            } else {
                0.0
            },
        ));

        Some(ExchangeRateRevaluationJournalEntryPlan {
            voucher_type: "Exchange Rate Revaluation".to_string(),
            company: self.company.clone(),
            posting_date: self.posting_date.clone(),
            multi_currency: true,
            accounts: journal_entry_accounts,
        })
    }

    fn journal_account_from_row(
        &self,
        account: &ExchangeRateRevaluationRow,
        default_cost_center: &str,
        exchange_rate: f64,
    ) -> ExchangeRateRevaluationJournalAccount {
        ExchangeRateRevaluationJournalAccount {
            account: account.account.clone(),
            party_type: account.party_type.clone(),
            party: account.party.clone(),
            account_currency: Some(account.account_currency.clone()),
            balance: flt(account.balance_in_account_currency, 2),
            exchange_rate,
            cost_center: Some(default_cost_center.to_string()),
            reference_type: Some("Exchange Rate Revaluation".to_string()),
            reference_name: self.name.clone(),
            ..Default::default()
        }
    }
}

impl DocumentController for ExchangeRateRevaluation {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "before_submit", "on_cancel"]
    }
}

pub fn calculate_exchange_rate_using_last_gle(
    company: &str,
    account: &str,
    party_type: Option<&str>,
    party: Option<&str>,
    gl_entries: &[ExchangeRateRevaluationGlEntry],
) -> Option<f64> {
    let latest = gl_entries
        .iter()
        .filter(|entry| {
            entry.company == company
                && entry.account == account
                && !entry.is_cancelled
                && (entry.debit > 0.0 || entry.credit > 0.0)
                && (entry.debit_in_account_currency > 0.0 || entry.credit_in_account_currency > 0.0)
                && party_type.is_none_or(|expected| entry.party_type.as_deref() == Some(expected))
                && party.is_none_or(|expected| entry.party.as_deref() == Some(expected))
        })
        .max_by(|left, right| left.posting_date.cmp(&right.posting_date))?;

    gl_entries
        .iter()
        .filter(|entry| {
            entry.voucher_type == latest.voucher_type
                && entry.voucher_no == latest.voucher_no
                && entry.account == account
        })
        .max_by(|left, right| left.posting_date.cmp(&right.posting_date))
        .and_then(|entry| {
            let denominator = entry.debit_in_account_currency - entry.credit_in_account_currency;
            if denominator == 0.0 {
                None
            } else {
                Some((entry.debit - entry.credit) / denominator)
            }
        })
}

#[allow(clippy::too_many_arguments)]
pub fn get_account_details(
    company: Option<&str>,
    posting_date: Option<&str>,
    account: &str,
    account_currency: &str,
    account_type: &str,
    party_type: Option<&str>,
    party: Option<&str>,
    account_balances: &[RevaluationAccountBalance],
    exchange_rates: &BTreeMap<String, f64>,
    last_exchange_rates: &BTreeMap<String, f64>,
) -> Result<RevaluationAccountDetail, ExchangeRateRevaluationError> {
    if company.unwrap_or_default().is_empty() || posting_date.unwrap_or_default().is_empty() {
        return Err(ExchangeRateRevaluationError::MissingCompanyOrPostingDate);
    }

    if matches!(account_type, "Receivable" | "Payable") && (party_type.is_none() || party.is_none())
    {
        return Err(ExchangeRateRevaluationError::MissingPartyForAccountType {
            account_type: account_type.to_string(),
        });
    }

    let mut account_details = RevaluationAccountDetail {
        account_currency: Some(account_currency.to_string()),
        ..Default::default()
    };
    let matching_balances = account_balances
        .iter()
        .filter(|row| row.account == account)
        .cloned()
        .collect::<Vec<_>>();

    if let Some(row) = matching_balances
        .first()
        .filter(|row| row.balance != 0.0 || row.balance_in_account_currency != 0.0)
        .and_then(|_| {
            ExchangeRateRevaluation::calculate_new_account_balance(
                "",
                &matching_balances,
                exchange_rates,
                last_exchange_rates,
                2,
            )
            .into_iter()
            .next()
        })
    {
        account_details.balance_in_base_currency = Some(row.balance_in_base_currency);
        account_details.balance_in_account_currency = Some(row.balance_in_account_currency);
        account_details.current_exchange_rate = Some(row.current_exchange_rate);
        account_details.new_exchange_rate = Some(row.new_exchange_rate);
        account_details.new_balance_in_base_currency = Some(row.new_balance_in_base_currency);
        account_details.new_balance_in_account_currency = Some(row.new_balance_in_account_currency);
        account_details.zero_balance = Some(row.zero_balance);
        account_details.gain_loss = Some(row.gain_loss);
    }

    Ok(account_details)
}

fn unrealized_journal_account(
    account: &str,
    default_cost_center: &str,
    reference_name: Option<&str>,
    debit: f64,
    credit: f64,
    debit_in_account_currency: f64,
    credit_in_account_currency: f64,
) -> ExchangeRateRevaluationJournalAccount {
    ExchangeRateRevaluationJournalAccount {
        account: account.to_string(),
        debit,
        credit,
        debit_in_account_currency,
        credit_in_account_currency,
        cost_center: Some(default_cost_center.to_string()),
        exchange_rate: 1.0,
        reference_type: Some("Exchange Rate Revaluation".to_string()),
        reference_name: reference_name.map(str::to_string),
        ..Default::default()
    }
}

fn normalize_optional_text(value: Option<&str>) -> Option<String> {
    value.filter(|value| !value.is_empty()).map(str::to_string)
}

fn last_exchange_rate_key(account: &str, party_type: Option<&str>, party: Option<&str>) -> String {
    format!(
        "{}|{}|{}",
        account,
        party_type.unwrap_or_default(),
        party.unwrap_or_default()
    )
}

fn flt(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
