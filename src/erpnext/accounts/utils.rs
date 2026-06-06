use std::cmp::Ordering;
use std::collections::BTreeMap;

pub const GL_REPOSTING_CHUNK: usize = 100;
pub const OUTSTANDING_DOCTYPES: [&str; 3] = ["Fees", "Purchase Invoice", "Sales Invoice"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiscalYearRecord {
    pub name: String,
    pub year_start_date: String,
    pub year_end_date: String,
    pub disabled: bool,
    pub companies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiscalYearError {
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiscalYearFilterField {
    pub fieldtype: &'static str,
    pub options: Vec<FiscalYearFilterOption>,
    pub operator: &'static str,
    pub query_value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiscalYearFilterOption {
    pub label: String,
    pub value: String,
    pub query_value: [String; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountMeta {
    pub name: String,
    pub report_type: String,
    pub is_group: bool,
    pub lft: i64,
    pub rgt: i64,
    pub account_currency: String,
    pub company: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostCenterMeta {
    pub name: String,
    pub is_group: bool,
    pub lft: i64,
    pub rgt: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceOnInput {
    pub account_name: Option<String>,
    pub date: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub company: Option<String>,
    pub in_account_currency: bool,
    pub cost_center: Option<String>,
    pub ignore_account_permission: bool,
    pub account_type: Option<String>,
    pub start_date: Option<String>,
    pub finance_book: Option<String>,
    pub include_default_fb_balances: bool,
    pub default_finance_book: Option<String>,
    pub company_default_currency: Option<String>,
    pub account_meta: Option<AccountMeta>,
    pub cost_center_meta: Option<CostCenterMeta>,
    pub account_type_accounts: Vec<String>,
    pub today: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceOnPlan {
    pub conditions: Vec<String>,
    pub select_field: Option<String>,
    pub effective_date: String,
    pub in_account_currency: bool,
    pub requires_permission_check: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AllocatedAmountArgs {
    pub allocated_amount: f64,
    pub unadjusted_amount: f64,
    pub precision: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocatedAmountError {
    Negative,
    GreaterThanUnadjusted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconciliationEffectInput {
    pub reconciliation_takes_effect_on: Option<String>,
    pub against_voucher_type: String,
    pub against_voucher_date: Option<String>,
    pub posting_date: String,
    pub today: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlEntryLike {
    pub account: String,
    pub cost_center: Option<String>,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntryDraft {
    pub accounts: Vec<JournalEntryAccountDraft>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JournalEntryAccountDraft {
    pub account: String,
    pub debit_in_account_currency: Option<f64>,
    pub credit_in_account_currency: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvanceLedgerEntry {
    pub company: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub voucher_detail_no: String,
    pub advance_voucher_type: String,
    pub advance_voucher_no: String,
    pub amount: f64,
    pub currency: String,
    pub cancel: bool,
    pub base_amount: Option<f64>,
    pub exchange_rate: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancePaymentLedgerEntry {
    pub doctype: &'static str,
    pub company: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub voucher_detail_no: String,
    pub against_voucher_type: String,
    pub against_voucher_no: String,
    pub amount: f64,
    pub currency: String,
    pub event: String,
    pub delinked: bool,
    pub base_amount: Option<f64>,
    pub exchange_rate: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateVoucherOutstandingPlan {
    Noop,
    AdvancePayment {
        voucher_type: String,
        voucher_no: String,
    },
    OutstandingRecompute {
        voucher_type: String,
        voucher_no: String,
        account: Option<String>,
        party_type: String,
        party: String,
    },
}

pub fn get_fiscal_year(
    date: Option<&str>,
    fiscal_year: Option<&str>,
    company: Option<&str>,
    raise_on_missing: bool,
    truncate: bool,
    fiscal_years: &[FiscalYearRecord],
) -> Result<Option<FiscalYearRecord>, FiscalYearError> {
    let mut years = get_fiscal_years(date, fiscal_year, company, raise_on_missing, fiscal_years)?;
    if years.is_empty() {
        return Ok(None);
    }
    let mut year = years.remove(0);

    if truncate {
        year.name = year
            .name
            .split('-')
            .map(|part| {
                part.chars()
                    .rev()
                    .take(2)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("-");
    }

    Ok(Some(year))
}

pub fn get_fiscal_years(
    transaction_date: Option<&str>,
    fiscal_year: Option<&str>,
    company: Option<&str>,
    raise_on_missing: bool,
    fiscal_years: &[FiscalYearRecord],
) -> Result<Vec<FiscalYearRecord>, FiscalYearError> {
    let all_fiscal_years = active_fiscal_years_for_company(company, fiscal_years);

    if transaction_date.is_none() && fiscal_year.is_none() {
        return Ok(all_fiscal_years);
    }

    for fy in all_fiscal_years {
        if fiscal_year.is_some_and(|name| fy.name == name)
            || transaction_date.is_some_and(|date| {
                compare_dates(&fy.year_start_date, date) != Ordering::Greater
                    && compare_dates(&fy.year_end_date, date) != Ordering::Less
            })
        {
            return Ok(vec![fy]);
        }
    }

    if raise_on_missing {
        Err(FiscalYearError::Missing)
    } else {
        Ok(vec![])
    }
}

pub fn get_fiscal_year_filter_field(
    company: Option<&str>,
    fiscal_years: &[FiscalYearRecord],
) -> FiscalYearFilterField {
    let options = active_fiscal_years_for_company(company, fiscal_years)
        .into_iter()
        .map(|fiscal_year| FiscalYearFilterOption {
            label: fiscal_year.name.clone(),
            value: fiscal_year.name,
            query_value: [fiscal_year.year_start_date, fiscal_year.year_end_date],
        })
        .collect();

    FiscalYearFilterField {
        fieldtype: "Select",
        options,
        operator: "Between",
        query_value: true,
    }
}

pub fn get_balance_on_plan(input: BalanceOnInput) -> BalanceOnPlan {
    let mut conditions = vec!["is_cancelled=0".to_string()];
    if let Some(start_date) = input.start_date.as_deref().filter(|date| !date.is_empty()) {
        conditions.push(format!(
            "posting_date >= {}",
            escape_sql_literal(start_date)
        ));
    }

    let effective_date = input.date.clone().unwrap_or(input.today);
    if let Some(date) = input.date.as_deref().filter(|date| !date.is_empty()) {
        conditions.push(format!("posting_date <= {}", escape_sql_literal(date)));
    }

    let mut in_account_currency = input.in_account_currency;
    let report_type = input
        .account_meta
        .as_ref()
        .map(|account| account.report_type.as_str())
        .unwrap_or_default();

    if let (Some(cost_center), "Profit and Loss") = (input.cost_center.as_deref(), report_type) {
        if let Some(cc) = input.cost_center_meta.as_ref() {
            if cc.is_group {
                conditions.push(format!(
                    "exists (select 1 from `tabCost Center` cc where cc.name = gle.cost_center and cc.lft >= {} and cc.rgt <= {})",
                    cc.lft, cc.rgt
                ));
            } else {
                conditions.push(format!(
                    "gle.cost_center = {}",
                    escape_sql_literal(cost_center)
                ));
            }
        } else {
            conditions.push(format!(
                "gle.cost_center = {}",
                escape_sql_literal(cost_center)
            ));
        }
    }

    if let Some(account) = input.account_meta.as_ref() {
        if account.is_group {
            conditions.push(format!(
                "exists (select name from `tabAccount` ac where ac.name = gle.account and ac.lft >= {} and ac.rgt <= {})",
                account.lft, account.rgt
            ));

            if input
                .company_default_currency
                .as_deref()
                .is_some_and(|currency| currency == account.account_currency)
            {
                in_account_currency = false;
            }
        } else if let Some(account_name) = input.account_name.as_deref() {
            conditions.push(format!(
                "gle.account = {}",
                escape_sql_literal(account_name)
            ));
        }
    }

    if input.account_type.is_some() {
        let accounts = input
            .account_type_accounts
            .iter()
            .map(|account| escape_sql_literal(account))
            .collect::<Vec<_>>()
            .join(", ");
        conditions.push(format!("gle.account in ({accounts})"));
    }

    if let (Some(party_type), Some(party)) = (input.party_type.as_deref(), input.party.as_deref()) {
        conditions.push(format!(
            "gle.party_type = {} and gle.party = {}",
            escape_sql_literal(party_type),
            escape_sql_literal(party)
        ));
    }

    if let Some(company) = input.company.as_deref() {
        conditions.push(format!("gle.company = {}", escape_sql_literal(company)));
    }

    if let Some(finance_book) = input.finance_book.as_deref() {
        if let (Some(default_finance_book), true) = (
            input.default_finance_book.as_deref(),
            input.include_default_fb_balances,
        ) {
            conditions.push(format!(
                "(gle.finance_book IN ({}, {}) OR gle.finance_book IS NULL)",
                escape_sql_literal(finance_book),
                escape_sql_literal(default_finance_book)
            ));
        } else {
            conditions.push(format!(
                "(gle.finance_book = {} OR gle.finance_book IS NULL)",
                escape_sql_literal(finance_book)
            ));
        }
    } else if let (Some(default_finance_book), true) = (
        input.default_finance_book.as_deref(),
        input.include_default_fb_balances,
    ) {
        conditions.push(format!(
            "(gle.finance_book = {} OR gle.finance_book IS NULL)",
            escape_sql_literal(default_finance_book)
        ));
    }

    let should_query = input.account_name.is_some()
        || input.party_type.zip(input.party).is_some()
        || input.account_type.is_some();
    let select_field = should_query.then(|| {
        if in_account_currency {
            "sum(round(debit_in_account_currency, p)) - sum(round(credit_in_account_currency, p))"
                .to_string()
        } else {
            "sum(round(debit, p)) - sum(round(credit, p))".to_string()
        }
    });

    BalanceOnPlan {
        conditions,
        select_field,
        effective_date,
        in_account_currency,
        requires_permission_check: input.account_name.is_some() && !input.ignore_account_permission,
    }
}

pub fn build_dimensions_dict_for_exc_gain_loss(
    entry: &BTreeMap<String, String>,
    active_dimensions: &[&str],
) -> BTreeMap<String, String> {
    let mut dimensions = BTreeMap::new();
    for fieldname in active_dimensions {
        if let Some(value) = entry.get(*fieldname).filter(|value| !value.is_empty()) {
            dimensions.insert((*fieldname).to_string(), value.clone());
        }
    }
    dimensions
}

pub fn validate_allocated_amount(args: AllocatedAmountArgs) -> Result<(), AllocatedAmountError> {
    let precision = args.precision.unwrap_or(2);
    if args.allocated_amount < 0.0 {
        return Err(AllocatedAmountError::Negative);
    }
    if flt(args.allocated_amount, precision) > flt(args.unadjusted_amount, precision) {
        return Err(AllocatedAmountError::GreaterThanUnadjusted);
    }
    Ok(())
}

pub fn get_reconciliation_effect_date(input: ReconciliationEffectInput) -> String {
    match input.reconciliation_takes_effect_on.as_deref() {
        Some("Advance Payment Date") => input.posting_date,
        Some("Oldest Of Invoice Or Advance") => {
            let reconcile_on = input
                .against_voucher_date
                .unwrap_or(input.posting_date.clone());
            if compare_dates(&reconcile_on, &input.posting_date) == Ordering::Less {
                input.posting_date
            } else {
                reconcile_on
            }
        }
        Some("Reconciliation Date") => input.today,
        _ => input.posting_date,
    }
}

pub fn convert_to_list(result: &[Vec<String>]) -> Vec<String> {
    result
        .iter()
        .filter_map(|row| row.first().cloned())
        .collect()
}

pub fn get_currency_precision(currency_precision: Option<i32>, number_format: &str) -> u32 {
    if let Some(precision) = currency_precision.filter(|precision| *precision != 0) {
        return precision.max(0) as u32;
    }
    number_format
        .rsplit_once('.')
        .map(|(_, decimals)| {
            decimals
                .chars()
                .filter(|ch| *ch == '#' || *ch == '0')
                .count() as u32
        })
        .unwrap_or(0)
}

pub fn get_zero_cutoff(fraction_units: Option<i32>) -> f64 {
    0.5 / f64::from(fraction_units.unwrap_or(100).max(1))
}

pub fn get_autoname_with_number(
    number_value: Option<&str>,
    doc_title: &str,
    company_abbr: &str,
) -> String {
    let mut parts = vec![doc_title.trim().to_string(), company_abbr.to_string()];
    if let Some(number) = number_value
        .map(str::trim)
        .filter(|number| !number.is_empty())
    {
        parts.insert(0, number.to_string());
    }
    parts.join(" - ")
}

pub fn compare_existing_and_expected_gle(
    existing_gle: &[GlEntryLike],
    expected_gle: &[GlEntryLike],
    precision: u32,
) -> bool {
    if existing_gle.len() != expected_gle.len() {
        return false;
    }

    for entry in expected_gle {
        let mut account_existed = false;
        for existing in existing_gle {
            if entry.account == existing.account {
                account_existed = true;
            }
            let comparable_cost_center =
                entry.cost_center.as_deref().unwrap_or_default().is_empty()
                    || existing
                        .cost_center
                        .as_deref()
                        .unwrap_or_default()
                        .is_empty()
                    || entry.cost_center == existing.cost_center;
            if entry.account == existing.account
                && comparable_cost_center
                && (flt(entry.debit, precision) != flt(existing.debit, precision)
                    || flt(entry.credit, precision) != flt(existing.credit, precision))
            {
                return false;
            }
        }
        if !account_existed {
            return false;
        }
    }

    true
}

pub fn get_journal_entry(
    account: &str,
    stock_adjustment_account: &str,
    amount: f64,
) -> JournalEntryDraft {
    let abs_amount = amount.abs();
    let (warehouse_debit, warehouse_credit, adjustment_debit, adjustment_credit) = if amount < 0.0 {
        (None, Some(abs_amount), Some(abs_amount), None)
    } else {
        (Some(abs_amount), None, None, Some(abs_amount))
    };

    JournalEntryDraft {
        accounts: vec![
            JournalEntryAccountDraft {
                account: account.to_string(),
                debit_in_account_currency: warehouse_debit,
                credit_in_account_currency: warehouse_credit,
            },
            JournalEntryAccountDraft {
                account: stock_adjustment_account.to_string(),
                debit_in_account_currency: adjustment_debit,
                credit_in_account_currency: adjustment_credit,
            },
        ],
    }
}

pub fn get_advance_ledger_entry(
    gle: AdvanceLedgerEntry,
    against_voucher_type: &str,
    against_voucher_no: &str,
) -> AdvancePaymentLedgerEntry {
    let event = if against_voucher_type == gle.voucher_type && against_voucher_no == gle.voucher_no
    {
        "Submit"
    } else {
        "Adjustment"
    };

    AdvancePaymentLedgerEntry {
        doctype: "Advance Payment Ledger Entry",
        company: gle.company,
        voucher_type: gle.voucher_type,
        voucher_no: gle.voucher_no,
        voucher_detail_no: gle.voucher_detail_no,
        against_voucher_type: gle.advance_voucher_type,
        against_voucher_no: gle.advance_voucher_no,
        amount: gle.amount,
        currency: gle.currency,
        event: event.to_string(),
        delinked: gle.cancel,
        base_amount: gle.base_amount,
        exchange_rate: gle.exchange_rate,
    }
}

pub fn update_voucher_outstanding_plan(
    voucher_type: &str,
    voucher_no: &str,
    account: Option<&str>,
    party_type: Option<&str>,
    party: Option<&str>,
    advance_payment_doctypes: &[&str],
) -> UpdateVoucherOutstandingPlan {
    if voucher_type.is_empty() || voucher_no.is_empty() {
        return UpdateVoucherOutstandingPlan::Noop;
    }

    if advance_payment_doctypes.contains(&voucher_type) {
        return UpdateVoucherOutstandingPlan::AdvancePayment {
            voucher_type: voucher_type.to_string(),
            voucher_no: voucher_no.to_string(),
        };
    }

    if !OUTSTANDING_DOCTYPES.contains(&voucher_type) || party_type.is_none() || party.is_none() {
        return UpdateVoucherOutstandingPlan::Noop;
    }

    UpdateVoucherOutstandingPlan::OutstandingRecompute {
        voucher_type: voucher_type.to_string(),
        voucher_no: voucher_no.to_string(),
        account: account.map(ToString::to_string),
        party_type: party_type.unwrap().to_string(),
        party: party.unwrap().to_string(),
    }
}

fn active_fiscal_years_for_company(
    company: Option<&str>,
    fiscal_years: &[FiscalYearRecord],
) -> Vec<FiscalYearRecord> {
    let mut years = fiscal_years
        .iter()
        .filter(|fy| !fy.disabled)
        .filter(|fy| {
            company.is_none()
                || fy.companies.is_empty()
                || fy.companies.iter().any(|fy_company| {
                    company.is_some_and(|requested_company| requested_company == fy_company)
                })
        })
        .cloned()
        .collect::<Vec<_>>();
    years.sort_by(|left, right| right.year_start_date.cmp(&left.year_start_date));
    years
}

fn escape_sql_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn flt(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn compare_dates(left: &str, right: &str) -> Ordering {
    parse_date_tuple(left).cmp(&parse_date_tuple(right))
}

fn parse_date_tuple(value: &str) -> (i32, u32, u32) {
    let mut parts = value.split('-');
    let year = parts.next().and_then(|part| part.parse().ok()).unwrap_or(0);
    let month = parts.next().and_then(|part| part.parse().ok()).unwrap_or(0);
    let day = parts.next().and_then(|part| part.parse().ok()).unwrap_or(0);
    (year, month, day)
}
