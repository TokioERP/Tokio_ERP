use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeferredDocType {
    SalesInvoice,
    PurchaseInvoice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeferredProcessType {
    Income,
    Expense,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceStopDateError {
    BeforeServiceStart { idx: u32 },
    AfterServiceEnd { idx: u32 },
    ChangedExistingStopDate { idx: u32 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredDoc {
    pub doctype: DeferredDocType,
    pub name: String,
    pub company: String,
    pub company_currency: String,
    pub currency: String,
    pub party: String,
    pub project: Option<String>,
    pub docstatus: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredItem {
    pub name: String,
    pub idx: u32,
    pub service_start_date: String,
    pub service_end_date: String,
    pub service_stop_date: Option<String>,
    pub enable_deferred_revenue: bool,
    pub enable_deferred_expense: bool,
    pub deferred_revenue_account: Option<String>,
    pub deferred_expense_account: Option<String>,
    pub income_account: Option<String>,
    pub expense_account: Option<String>,
    pub net_amount: f64,
    pub base_net_amount: f64,
    pub net_amount_precision: u32,
    pub base_net_amount_precision: u32,
    pub project: Option<String>,
    pub cost_center: Option<String>,
    pub accounting_dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AlreadyBookedAmounts {
    pub base: f64,
    pub account_currency: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BookingDates {
    pub start_date: String,
    pub end_date: String,
    pub last_gl_entry: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntryPlan {
    pub account: String,
    pub against: Option<String>,
    pub credit: f64,
    pub credit_in_account_currency: f64,
    pub debit: f64,
    pub debit_in_account_currency: f64,
    pub account_currency: String,
    pub voucher_detail_no: Option<String>,
    pub posting_date: String,
    pub project: Option<String>,
    pub cost_center: Option<String>,
    pub against_voucher_type: Option<String>,
    pub against_voucher: Option<String>,
    pub reference_name: Option<String>,
    pub reference_type: Option<String>,
    pub reference_detail_no: Option<String>,
    pub accounting_dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryPlan {
    pub posting_date: String,
    pub company: String,
    pub voucher_type: String,
    pub process_deferred_accounting: Option<String>,
    pub submit: bool,
    pub entries: Vec<GlEntryPlan>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredPostingPlan {
    pub posting_date: String,
    pub amount: f64,
    pub base_amount: f64,
    pub last_gl_entry: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredConversionPlan {
    pub deferred_process: String,
    pub invoice_doctype: &'static str,
    pub item_table: &'static str,
    pub parent_table: &'static str,
    pub enable_field: &'static str,
    pub date_params: (String, String),
    pub conditions: String,
    pub query: String,
    pub mail_if_error: Option<DeferredErrorMailPlan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredErrorMailPlan {
    pub title: String,
    pub doctype: String,
    pub docname: String,
    pub content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredProcessAccountingDocPlan {
    pub company: String,
    pub posting_date: String,
    pub start_date: String,
    pub end_date: String,
    pub process_type: DeferredProcessType,
}

pub fn validate_service_stop_dates(
    doctype: DeferredDocType,
    _doc_name: &str,
    items: &[DeferredItem],
    old_stop_dates: &BTreeMap<String, String>,
) -> Result<(), ServiceStopDateError> {
    for item in items {
        let enabled = match doctype {
            DeferredDocType::SalesInvoice => item.enable_deferred_revenue,
            DeferredDocType::PurchaseInvoice => item.enable_deferred_expense,
        };
        if !enabled {
            continue;
        }

        if let Some(stop_date) = item.service_stop_date.as_deref() {
            if date_diff_str(stop_date, &item.service_start_date) < 0 {
                return Err(ServiceStopDateError::BeforeServiceStart { idx: item.idx });
            }
            if date_diff_str(stop_date, &item.service_end_date) > 0 {
                return Err(ServiceStopDateError::AfterServiceEnd { idx: item.idx });
            }
        }

        if let (Some(old), Some(new)) = (
            old_stop_dates
                .get(&item.name)
                .filter(|value| !value.is_empty()),
            item.service_stop_date.as_ref(),
        ) {
            if parse_date(old) != parse_date(new) {
                return Err(ServiceStopDateError::ChangedExistingStopDate { idx: item.idx });
            }
        }
    }

    Ok(())
}

pub fn build_conditions(
    process_type: DeferredProcessType,
    account: Option<&str>,
    company: Option<&str>,
) -> String {
    let deferred_account = match process_type {
        DeferredProcessType::Income => "item.deferred_revenue_account",
        DeferredProcessType::Expense => "item.deferred_expense_account",
    };

    if let Some(account) = account {
        format!("AND {deferred_account}={}", sql_quote(account))
    } else if let Some(company) = company {
        format!("AND p.company = {}", sql_quote(company))
    } else {
        String::new()
    }
}

pub fn convert_deferred_expense_to_expense_plan(
    deferred_process: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
    today: &str,
    conditions: &str,
    deferred_accounting_error: bool,
) -> DeferredConversionPlan {
    conversion_plan(
        DeferredProcessType::Expense,
        deferred_process,
        start_date,
        end_date,
        today,
        conditions,
        deferred_accounting_error,
    )
}

pub fn convert_deferred_revenue_to_income_plan(
    deferred_process: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
    today: &str,
    conditions: &str,
    deferred_accounting_error: bool,
) -> DeferredConversionPlan {
    conversion_plan(
        DeferredProcessType::Income,
        deferred_process,
        start_date,
        end_date,
        today,
        conditions,
        deferred_accounting_error,
    )
}

pub fn process_deferred_accounting_plan(
    posting_date: Option<&str>,
    today: &str,
    automatically_process: bool,
    companies: &[&str],
) -> Vec<DeferredProcessAccountingDocPlan> {
    if !automatically_process {
        return Vec::new();
    }

    let posting_date = posting_date.unwrap_or(today).to_string();
    let start_date = parse_date(today).add_months(-1).to_string();
    let end_date = parse_date(today).add_days(-1).to_string();
    let mut docs = Vec::new();
    for company in companies {
        for process_type in [DeferredProcessType::Income, DeferredProcessType::Expense] {
            docs.push(DeferredProcessAccountingDocPlan {
                company: (*company).to_string(),
                posting_date: posting_date.clone(),
                start_date: start_date.clone(),
                end_date: end_date.clone(),
                process_type,
            });
        }
    }
    docs
}

pub fn send_mail_plan(deferred_process: &str) -> DeferredErrorMailPlan {
    let link = format!("Process Deferred Accounting/{deferred_process}");
    DeferredErrorMailPlan {
        title: format!("Error while processing deferred accounting for {deferred_process}"),
        doctype: "Process Deferred Accounting".to_string(),
        docname: deferred_process.to_string(),
        content: format!(
            "Deferred accounting failed for some invoices:\nPlease check Process Deferred Accounting {link} and submit manually after resolving errors."
        ),
    }
}

pub fn get_booking_dates(
    _doc: &DeferredDoc,
    item: &DeferredItem,
    posting_date: Option<&str>,
    prev_posting_date: Option<&str>,
    latest_existing_posting_date: Option<&str>,
) -> Option<BookingDates> {
    let posting_date = posting_date.unwrap_or_default();
    let start_date = if let Some(prev_posting_date) = prev_posting_date {
        parse_date(prev_posting_date).add_days(1)
    } else if let Some(latest_existing_posting_date) = latest_existing_posting_date {
        parse_date(latest_existing_posting_date).add_days(1)
    } else {
        parse_date(&item.service_start_date)
    };

    let mut end_date = start_date.last_day();
    let service_end_date = parse_date(&item.service_end_date);
    let mut last_gl_entry = false;

    if end_date >= service_end_date {
        end_date = service_end_date;
        last_gl_entry = true;
    } else if let Some(service_stop_date) = item.service_stop_date.as_deref() {
        let service_stop_date = parse_date(service_stop_date);
        if end_date >= service_stop_date {
            end_date = service_stop_date;
            last_gl_entry = true;
        }
    }

    if !posting_date.is_empty() {
        let posting_date = parse_date(posting_date);
        if end_date > posting_date {
            end_date = posting_date;
        }
    }

    (start_date <= end_date).then(|| BookingDates {
        start_date: start_date.to_string(),
        end_date: end_date.to_string(),
        last_gl_entry,
    })
}

pub fn calculate_amount(
    doc: &DeferredDoc,
    item: &DeferredItem,
    last_gl_entry: bool,
    total_days: i64,
    total_booking_days: i64,
    account_currency: &str,
    already_booked: AlreadyBookedAmounts,
) -> (f64, f64) {
    if last_gl_entry {
        let base_amount = flt(
            item.base_net_amount - already_booked.base,
            item.base_net_amount_precision,
        );
        let amount = if account_currency == doc.company_currency {
            base_amount
        } else {
            flt(
                item.net_amount - already_booked.account_currency,
                item.net_amount_precision,
            )
        };
        return (amount, base_amount);
    }

    let base_amount = flt(
        item.base_net_amount * total_booking_days as f64 / total_days as f64,
        item.base_net_amount_precision,
    );
    let amount = if account_currency == doc.company_currency {
        base_amount
    } else {
        flt(
            item.net_amount * total_booking_days as f64 / total_days as f64,
            item.net_amount_precision,
        )
    };
    (amount, base_amount)
}

pub fn calculate_monthly_amount(
    doc: &DeferredDoc,
    item: &DeferredItem,
    last_gl_entry: bool,
    start_date: &str,
    end_date: &str,
    account_currency: &str,
    already_booked: AlreadyBookedAmounts,
) -> (f64, f64) {
    if last_gl_entry {
        let base_amount = flt(
            item.base_net_amount - already_booked.base,
            item.base_net_amount_precision,
        );
        let amount = if account_currency == doc.company_currency {
            base_amount
        } else {
            flt(
                item.net_amount - already_booked.account_currency,
                item.net_amount_precision,
            )
        };
        return (amount, base_amount);
    }

    let service_start = parse_date(&item.service_start_date);
    let service_end = parse_date(&item.service_end_date);
    let total_months = ((service_end.year - service_start.year) * 12 + service_end.month as i32
        - service_start.month as i32
        + 1) as f64;
    let prorate_factor = date_diff(service_end, service_start) as f64
        / date_diff(service_end.last_day(), service_start.first_day()) as f64;
    let actual_months = rounded(total_months * prorate_factor, 1);

    let mut base_amount = flt(
        item.base_net_amount / actual_months,
        item.base_net_amount_precision,
    );
    if base_amount + already_booked.base > item.base_net_amount {
        base_amount = item.base_net_amount - already_booked.base;
    }

    let mut amount = if account_currency == doc.company_currency {
        base_amount
    } else {
        let mut amount = flt(item.net_amount / actual_months, item.net_amount_precision);
        if amount + already_booked.account_currency > item.net_amount {
            amount = item.net_amount - already_booked.account_currency;
        }
        amount
    };

    let start_date = parse_date(start_date);
    let end_date = parse_date(end_date);
    if start_date.first_day() != start_date || end_date.last_day() != end_date {
        let partial_month = date_diff(end_date, start_date) as f64
            / date_diff(end_date.last_day(), start_date.first_day()) as f64;
        let factor = rounded(partial_month, 1);
        base_amount *= factor;
        amount *= factor;
    }

    (amount, base_amount)
}

#[allow(clippy::too_many_arguments)]
pub fn make_gl_entries_plan(
    _doc: &DeferredDoc,
    credit_account: &str,
    debit_account: &str,
    against: &str,
    amount: f64,
    base_amount: f64,
    posting_date: &str,
    project: Option<&str>,
    account_currency: &str,
    cost_center: Option<&str>,
    item: &DeferredItem,
    deferred_process: Option<&str>,
) -> Option<Vec<GlEntryPlan>> {
    if amount == 0.0 {
        return None;
    }

    Some(vec![
        GlEntryPlan {
            account: credit_account.to_string(),
            against: Some(against.to_string()),
            credit: base_amount,
            credit_in_account_currency: amount,
            debit: 0.0,
            debit_in_account_currency: 0.0,
            account_currency: account_currency.to_string(),
            voucher_detail_no: Some(item.name.clone()),
            posting_date: posting_date.to_string(),
            project: project.map(str::to_string),
            cost_center: cost_center.map(str::to_string),
            against_voucher_type: Some("Process Deferred Accounting".to_string()),
            against_voucher: deferred_process.map(str::to_string),
            reference_name: None,
            reference_type: None,
            reference_detail_no: None,
            accounting_dimensions: BTreeMap::new(),
        },
        GlEntryPlan {
            account: debit_account.to_string(),
            against: Some(against.to_string()),
            credit: 0.0,
            credit_in_account_currency: 0.0,
            debit: base_amount,
            debit_in_account_currency: amount,
            account_currency: account_currency.to_string(),
            voucher_detail_no: Some(item.name.clone()),
            posting_date: posting_date.to_string(),
            project: project.map(str::to_string),
            cost_center: cost_center.map(str::to_string),
            against_voucher_type: Some("Process Deferred Accounting".to_string()),
            against_voucher: deferred_process.map(str::to_string),
            reference_name: None,
            reference_type: None,
            reference_detail_no: None,
            accounting_dimensions: BTreeMap::new(),
        },
    ])
}

#[allow(clippy::too_many_arguments)]
pub fn book_revenue_via_journal_entry_plan(
    doc: &DeferredDoc,
    credit_account: &str,
    debit_account: &str,
    amount: f64,
    base_amount: f64,
    posting_date: &str,
    project: Option<&str>,
    account_currency: &str,
    cost_center: Option<&str>,
    item: &DeferredItem,
    deferred_process: Option<&str>,
    submit: bool,
) -> Option<JournalEntryPlan> {
    if amount == 0.0 {
        return None;
    }

    Some(JournalEntryPlan {
        posting_date: posting_date.to_string(),
        company: doc.company.clone(),
        voucher_type: match doc.doctype {
            DeferredDocType::SalesInvoice => "Deferred Revenue",
            DeferredDocType::PurchaseInvoice => "Deferred Expense",
        }
        .to_string(),
        process_deferred_accounting: deferred_process.map(str::to_string),
        submit,
        entries: vec![
            GlEntryPlan {
                account: credit_account.to_string(),
                against: None,
                credit: base_amount,
                credit_in_account_currency: amount,
                debit: 0.0,
                debit_in_account_currency: 0.0,
                account_currency: account_currency.to_string(),
                voucher_detail_no: Some(item.name.clone()),
                posting_date: posting_date.to_string(),
                project: project.map(str::to_string),
                cost_center: cost_center.map(str::to_string),
                against_voucher_type: None,
                against_voucher: None,
                reference_name: Some(doc.name.clone()),
                reference_type: Some(doc.doctype.label().to_string()),
                reference_detail_no: Some(item.name.clone()),
                accounting_dimensions: item.accounting_dimensions.clone(),
            },
            GlEntryPlan {
                account: debit_account.to_string(),
                against: None,
                credit: 0.0,
                credit_in_account_currency: 0.0,
                debit: base_amount,
                debit_in_account_currency: amount,
                account_currency: account_currency.to_string(),
                voucher_detail_no: Some(item.name.clone()),
                posting_date: posting_date.to_string(),
                project: project.map(str::to_string),
                cost_center: cost_center.map(str::to_string),
                against_voucher_type: None,
                against_voucher: None,
                reference_name: Some(doc.name.clone()),
                reference_type: Some(doc.doctype.label().to_string()),
                reference_detail_no: Some(item.name.clone()),
                accounting_dimensions: item.accounting_dimensions.clone(),
            },
        ],
    })
}

pub fn get_deferred_booking_accounts(
    doctype: DeferredDocType,
    dr_or_cr: &str,
    normal_account: &str,
    deferred_account: &str,
) -> String {
    let (credit_account, debit_account) = match doctype {
        DeferredDocType::SalesInvoice => (normal_account, deferred_account),
        DeferredDocType::PurchaseInvoice => (deferred_account, normal_account),
    };

    if dr_or_cr == "Debit" {
        debit_account.to_string()
    } else {
        credit_account.to_string()
    }
}

impl DeferredDocType {
    fn label(self) -> &'static str {
        match self {
            Self::SalesInvoice => "Sales Invoice",
            Self::PurchaseInvoice => "Purchase Invoice",
        }
    }
}

fn sql_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

#[allow(clippy::too_many_arguments)]
fn conversion_plan(
    process_type: DeferredProcessType,
    deferred_process: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
    today: &str,
    conditions: &str,
    deferred_accounting_error: bool,
) -> DeferredConversionPlan {
    let start_date = start_date
        .map(str::to_string)
        .unwrap_or_else(|| parse_date(today).add_months(-1).to_string());
    let end_date = end_date
        .map(str::to_string)
        .unwrap_or_else(|| parse_date(today).add_days(-1).to_string());
    let (invoice_doctype, item_table, parent_table, enable_field) = match process_type {
        DeferredProcessType::Income => (
            "Sales Invoice",
            "tabSales Invoice Item",
            "tabSales Invoice",
            "enable_deferred_revenue",
        ),
        DeferredProcessType::Expense => (
            "Purchase Invoice",
            "tabPurchase Invoice Item",
            "tabPurchase Invoice",
            "enable_deferred_expense",
        ),
    };
    let conditions = conditions.trim().to_string();
    let query = if conditions.is_empty() {
        format!(
            "select distinct item.parent from `{item_table}` item, `{parent_table}` p where item.service_start_date<=%s and item.service_end_date>=%s and item.{enable_field} = 1 and item.parent=p.name and item.docstatus = 1 and ifnull(item.amount, 0) > 0"
        )
    } else {
        format!(
            "select distinct item.parent from `{item_table}` item, `{parent_table}` p where item.service_start_date<=%s and item.service_end_date>=%s and item.{enable_field} = 1 and item.parent=p.name and item.docstatus = 1 and ifnull(item.amount, 0) > 0 {conditions}"
        )
    };

    DeferredConversionPlan {
        deferred_process: deferred_process.to_string(),
        invoice_doctype,
        item_table,
        parent_table,
        enable_field,
        date_params: (end_date, start_date),
        conditions,
        query,
        mail_if_error: deferred_accounting_error.then(|| send_mail_plan(deferred_process)),
    }
}

fn flt(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn rounded(value: f64, precision: u32) -> f64 {
    flt(value, precision)
}

fn date_diff_str(end: &str, start: &str) -> i64 {
    date_diff(parse_date(end), parse_date(start))
}

fn date_diff(end: SimpleDate, start: SimpleDate) -> i64 {
    end.days_since_epoch() - start.days_since_epoch()
}

fn parse_date(value: &str) -> SimpleDate {
    SimpleDate::parse(value).unwrap_or_else(|| panic!("invalid date: {value}"))
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SimpleDate {
    year: i32,
    month: u32,
    day: u32,
}

impl SimpleDate {
    fn parse(value: &str) -> Option<Self> {
        let mut parts = value.split('-');
        let year = parts.next()?.parse().ok()?;
        let month = parts.next()?.parse().ok()?;
        let day = parts.next()?.parse().ok()?;
        if parts.next().is_some() || !(1..=12).contains(&month) {
            return None;
        }
        let last_day = days_in_month(year, month);
        if day == 0 || day > last_day {
            return None;
        }
        Some(Self { year, month, day })
    }

    fn first_day(self) -> Self {
        Self { day: 1, ..self }
    }

    fn last_day(self) -> Self {
        Self {
            day: days_in_month(self.year, self.month),
            ..self
        }
    }

    fn add_days(self, days: i64) -> Self {
        civil_from_days(self.days_since_epoch() + days)
    }

    fn add_months(self, months: i32) -> Self {
        let total_months = self.year * 12 + self.month as i32 - 1 + months;
        let year = total_months.div_euclid(12);
        let month = total_months.rem_euclid(12) as u32 + 1;
        let day = self.day.min(days_in_month(year, month));
        Self { year, month, day }
    }

    fn days_since_epoch(self) -> i64 {
        days_from_civil(self.year, self.month, self.day)
    }
}

impl std::fmt::Display for SimpleDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => unreachable!("validated month"),
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = year - i32::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month as i32;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era * 146_097 + doe - 719_468) as i64
}

fn civil_from_days(days: i64) -> SimpleDate {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let doe = days - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe as i32 + era as i32 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    SimpleDate {
        year: year + i32::from(month <= 2),
        month: month as u32,
        day: day as u32,
    }
}
