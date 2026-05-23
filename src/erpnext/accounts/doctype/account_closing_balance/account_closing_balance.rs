use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountClosingBalance {
    pub closing_date: Option<String>,
    pub account: Option<String>,
    pub account_currency: Option<String>,
    pub company: Option<String>,
    pub cost_center: Option<String>,
    pub credit: f64,
    pub credit_in_account_currency: f64,
    pub credit_in_reporting_currency: f64,
    pub debit: f64,
    pub debit_in_account_currency: f64,
    pub debit_in_reporting_currency: f64,
    pub finance_book: Option<String>,
    pub is_period_closing_voucher_entry: bool,
    pub period_closing_voucher: Option<String>,
    pub project: Option<String>,
    pub reporting_currency_exchange_rate: f64,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountClosingEntry {
    pub company: String,
    pub account: String,
    pub account_currency: String,
    pub cost_center: String,
    pub project: String,
    pub finance_book: String,
    pub is_period_closing_voucher_entry: bool,
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountClosingEntryKey {
    pub parts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AggregatedClosingEntry {
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviousClosingEntriesPlan {
    pub period_closing_voucher_filters: BTreeMap<String, String>,
    pub period_closing_voucher_order_by: &'static str,
    pub period_closing_voucher_limit: usize,
    pub account_closing_balance_fields: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportingCurrencyError {
    pub title: &'static str,
    pub message: String,
}

impl AccountClosingBalance {
    pub const DOCTYPE: &'static str = "Account Closing Balance";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 16] = [
        "closing_date",
        "account",
        "cost_center",
        "debit",
        "credit",
        "reporting_currency_exchange_rate",
        "debit_in_reporting_currency",
        "credit_in_reporting_currency",
        "account_currency",
        "debit_in_account_currency",
        "credit_in_account_currency",
        "project",
        "company",
        "finance_book",
        "period_closing_voucher",
        "is_period_closing_voucher_entry",
    ];
    pub const DEFAULT_VIEW: &'static str = "List";
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const IN_CREATE: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("closing_date", "Closing Date")
                .in_filter()
                .in_list_view()
                .oldfield("posting_date", "Date")
                .search_index(),
            FieldSpec::link("account", "Account")
                .options("Account")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .oldfield("account", "Link")
                .search_index(),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .in_filter()
                .in_list_view()
                .oldfield("cost_center", "Link"),
            FieldSpec::currency("debit", "Debit Amount")
                .options("Company:company:default_currency")
                .oldfield("debit", "Currency"),
            FieldSpec::currency("credit", "Credit Amount")
                .options("Company:company:default_currency")
                .oldfield("credit", "Currency"),
            FieldSpec::float(
                "reporting_currency_exchange_rate",
                "Reporting Currency Exchange Rate",
            )
            .precision("9"),
            FieldSpec::currency(
                "debit_in_reporting_currency",
                "Debit Amount in Reporting Currency",
            )
            .options("Company:company:reporting_currency"),
            FieldSpec::currency(
                "credit_in_reporting_currency",
                "Credit Amount in Reporting Currency",
            )
            .options("Company:company:reporting_currency"),
            FieldSpec::link("account_currency", "Account Currency").options("Currency"),
            FieldSpec::currency(
                "debit_in_account_currency",
                "Debit Amount in Account Currency",
            )
            .options("account_currency"),
            FieldSpec::currency(
                "credit_in_account_currency",
                "Credit Amount in Account Currency",
            )
            .options("account_currency"),
            FieldSpec::link("project", "Project").options("Project"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .oldfield("company", "Link")
                .search_index(),
            FieldSpec::link("finance_book", "Finance Book").options("Finance Book"),
            FieldSpec::link("period_closing_voucher", "Period Closing Voucher")
                .options("Period Closing Voucher")
                .in_standard_filter()
                .search_index(),
            FieldSpec::check(
                "is_period_closing_voucher_entry",
                "Is Period Closing Voucher Entry",
            )
            .default("0"),
        ]
    }
}

impl AccountClosingEntry {
    pub fn new(
        company: impl Into<String>,
        account: impl Into<String>,
        account_currency: impl Into<String>,
    ) -> Self {
        Self {
            company: company.into(),
            account: account.into(),
            account_currency: account_currency.into(),
            ..Self::default()
        }
    }
}

impl DocumentController for AccountClosingBalance {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn make_closing_entries(
    closing_entries: &[AccountClosingEntry],
    previous_closing_entries: &[AccountClosingEntry],
    voucher_name: &str,
    _company: &str,
    closing_date: &str,
    accounting_dimensions: &[String],
    reporting_currency_exchange_rate: Option<f64>,
) -> Result<Vec<AccountClosingBalance>, ReportingCurrencyError> {
    let mut combined_entries = closing_entries.to_vec();
    combined_entries.extend_from_slice(previous_closing_entries);
    let merged_entries =
        aggregate_with_last_account_closing_balance(&combined_entries, accounting_dimensions);
    let mut entries = Vec::new();

    for value in merged_entries.values() {
        let mut closing_balance = AccountClosingBalance {
            company: value.dimensions.get("company").cloned(),
            account: value.dimensions.get("account").cloned(),
            account_currency: value.dimensions.get("account_currency").cloned(),
            cost_center: value.dimensions.get("cost_center").cloned(),
            project: value.dimensions.get("project").cloned(),
            finance_book: value.dimensions.get("finance_book").cloned(),
            is_period_closing_voucher_entry: value
                .dimensions
                .get("is_period_closing_voucher_entry")
                .map(|value| value == "1")
                .unwrap_or(false),
            debit: value.debit,
            credit: value.credit,
            debit_in_account_currency: value.debit_in_account_currency,
            credit_in_account_currency: value.credit_in_account_currency,
            dimensions: value.dimensions.clone(),
            period_closing_voucher: Some(voucher_name.to_string()),
            closing_date: Some(closing_date.to_string()),
            ..AccountClosingBalance::default()
        };
        set_amount_in_reporting_currency(
            &mut closing_balance,
            "",
            "",
            closing_date,
            reporting_currency_exchange_rate,
        )?;
        entries.push(closing_balance);
    }

    Ok(entries)
}

pub fn aggregate_with_last_account_closing_balance(
    entries: &[AccountClosingEntry],
    accounting_dimensions: &[String],
) -> BTreeMap<AccountClosingEntryKey, AggregatedClosingEntry> {
    let mut merged_entries = BTreeMap::new();

    for entry in entries {
        let (key, key_values) = generate_key(entry, accounting_dimensions);
        let merged_entry = merged_entries.entry(key).or_insert(AggregatedClosingEntry {
            debit: 0.0,
            credit: 0.0,
            debit_in_account_currency: 0.0,
            credit_in_account_currency: 0.0,
            dimensions: BTreeMap::new(),
        });

        merged_entry.dimensions = key_values;
        merged_entry.debit += entry.debit;
        merged_entry.credit += entry.credit;
        merged_entry.debit_in_account_currency += entry.debit_in_account_currency;
        merged_entry.credit_in_account_currency += entry.credit_in_account_currency;
    }

    merged_entries
}

pub fn generate_key(
    entry: &AccountClosingEntry,
    accounting_dimensions: &[String],
) -> (AccountClosingEntryKey, BTreeMap<String, String>) {
    let is_period_closing_voucher_entry = if entry.is_period_closing_voucher_entry {
        "1"
    } else {
        "0"
    };
    let mut parts = vec![
        entry.account.clone(),
        entry.account_currency.clone(),
        entry.cost_center.clone(),
        entry.project.clone(),
        entry.finance_book.clone(),
        is_period_closing_voucher_entry.to_string(),
    ];
    let mut key_values = BTreeMap::from([
        ("company".to_string(), entry.company.clone()),
        ("account".to_string(), entry.account.clone()),
        (
            "account_currency".to_string(),
            entry.account_currency.clone(),
        ),
        ("cost_center".to_string(), entry.cost_center.clone()),
        ("project".to_string(), entry.project.clone()),
        ("finance_book".to_string(), entry.finance_book.clone()),
        (
            "is_period_closing_voucher_entry".to_string(),
            is_period_closing_voucher_entry.to_string(),
        ),
    ]);

    for dimension in accounting_dimensions {
        let value = entry.dimensions.get(dimension).cloned().unwrap_or_default();
        parts.push(value.clone());
        key_values.insert(dimension.clone(), value);
    }

    (AccountClosingEntryKey { parts }, key_values)
}

pub fn previous_closing_entries_plan(
    company: &str,
    closing_date: &str,
    accounting_dimensions: &[String],
) -> PreviousClosingEntriesPlan {
    let mut account_closing_balance_fields = vec![
        "company".to_string(),
        "account".to_string(),
        "account_currency".to_string(),
        "debit".to_string(),
        "credit".to_string(),
        "debit_in_account_currency".to_string(),
        "credit_in_account_currency".to_string(),
        "cost_center".to_string(),
        "project".to_string(),
        "finance_book".to_string(),
        "is_period_closing_voucher_entry".to_string(),
    ];
    account_closing_balance_fields.extend(accounting_dimensions.iter().cloned());

    PreviousClosingEntriesPlan {
        period_closing_voucher_filters: BTreeMap::from([
            ("company".to_string(), company.to_string()),
            ("docstatus".to_string(), "1".to_string()),
            ("period_end_date".to_string(), format!("<{}", closing_date)),
        ]),
        period_closing_voucher_order_by: "period_end_date desc",
        period_closing_voucher_limit: 1,
        account_closing_balance_fields,
    }
}

pub fn set_amount_in_reporting_currency(
    closing_balance: &mut AccountClosingBalance,
    default_currency: &str,
    reporting_currency: &str,
    closing_date: &str,
    reporting_currency_exchange_rate: Option<f64>,
) -> Result<(), ReportingCurrencyError> {
    let Some(reporting_currency_exchange_rate) = reporting_currency_exchange_rate else {
        return Err(ReportingCurrencyError {
            title: "Reporting Currency Exchange Not Found",
            message: format!(
                "Unable to find exchange rate for {} to {} for key date {}. Please create a Currency Exchange record manually.",
                default_currency, reporting_currency, closing_date
            ),
        });
    };

    closing_balance.reporting_currency_exchange_rate = reporting_currency_exchange_rate;
    closing_balance.debit_in_reporting_currency =
        closing_balance.debit * reporting_currency_exchange_rate;
    closing_balance.credit_in_reporting_currency =
        closing_balance.credit * reporting_currency_exchange_rate;

    Ok(())
}
