use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYearData {
    pub year_start_date: String,
    pub year_end_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialStatementFilters {
    pub company: String,
    pub presentation_currency: Option<String>,
    pub accumulated_values: bool,
    pub show_zero_values: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Period {
    pub key: String,
    pub label: String,
    pub from_date: String,
    pub to_date: String,
    pub year_start_date: String,
    pub year_end_date: String,
    pub to_date_fiscal_year: Option<String>,
    pub from_date_fiscal_year_start_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountRow {
    pub name: String,
    pub account_number: Option<String>,
    pub parent_account: Option<String>,
    pub account_name: String,
    pub root_type: String,
    pub report_type: String,
    pub include_in_gross: bool,
    pub account_type: Option<String>,
    pub is_group: bool,
    pub lft: i32,
    pub rgt: i32,
    pub indent: i32,
    pub opening_balance: f64,
    pub values: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinancialStatementGlEntry {
    pub account: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub fiscal_year: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: String,
    pub label: String,
    pub fieldtype: &'static str,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinancialStatementRow {
    pub account: Option<String>,
    pub parent_account: Option<String>,
    pub indent: f64,
    pub year_start_date: Option<String>,
    pub year_end_date: Option<String>,
    pub currency: Option<String>,
    pub include_in_gross: bool,
    pub account_type: Option<String>,
    pub is_group: bool,
    pub opening_balance: f64,
    pub account_name: Option<String>,
    pub acc_name: Option<String>,
    pub acc_number: Option<String>,
    pub values: BTreeMap<String, f64>,
    pub total: f64,
    pub has_value: bool,
    pub is_blank: bool,
}

impl Period {
    pub fn new(key: &str, label: &str, from_date: &str, to_date: &str) -> Self {
        let year_start_date = format!("{}-01-01", &from_date[0..4]);
        let year_end_date = format!("{}-12-31", &to_date[0..4]);
        Self {
            key: key.to_string(),
            label: label.to_string(),
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
            year_start_date: year_start_date.clone(),
            year_end_date,
            to_date_fiscal_year: Some(format!("FY{}", &to_date[0..4])),
            from_date_fiscal_year_start_date: Some(year_start_date),
        }
    }
}

impl AccountRow {
    pub fn with_value(mut self, key: &str, value: f64) -> Self {
        self.values.insert(key.to_string(), value);
        self
    }

    pub fn to_statement_row(
        self,
        currency: &str,
        periods: &[Period],
        values: &[(&str, f64)],
    ) -> FinancialStatementRow {
        let mut row = prepare_data(&[self], "Debit", periods, currency, true)
            .into_iter()
            .next()
            .expect("single account should produce one row");
        for (key, value) in values {
            row.values.insert((*key).to_string(), *value);
        }
        row.total = values.iter().map(|(_, value)| *value).sum();
        row.has_value = values.iter().any(|(_, value)| value.abs() >= 0.005);
        row
    }
}

impl FinancialStatementGlEntry {
    pub fn new(
        account: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
        fiscal_year: &str,
    ) -> Self {
        Self {
            account: account.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
            fiscal_year: fiscal_year.to_string(),
        }
    }
}

impl ReportColumn {
    pub fn link(label: &str, fieldname: &str, options: &str, width: u16, hidden: bool) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Link",
            options: Some(options.to_string()),
            width,
            hidden,
        }
    }

    pub fn data(label: &str, fieldname: &str, width: u16, hidden: bool) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Data",
            options: None,
            width,
            hidden,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Currency",
            options: Some("currency".to_string()),
            width,
            hidden: false,
        }
    }
}

pub fn get_period_list(
    from_fiscal_year: &str,
    to_fiscal_year: &str,
    period_start_date: Option<&str>,
    period_end_date: Option<&str>,
    filter_based_on: &str,
    periodicity: &str,
    accumulated_values: bool,
    reset_period_on_fy_change: bool,
    ignore_fiscal_year: bool,
    _company: bool,
    fiscal_years: &BTreeMap<String, FiscalYearData>,
) -> Result<Vec<Period>, String> {
    let (year_start_date, year_end_date) = if filter_based_on == "Fiscal Year" {
        let fiscal_year = get_fiscal_year_data(from_fiscal_year, to_fiscal_year, fiscal_years)?;
        validate_fiscal_year(&fiscal_year)?;
        (
            parse_date(&fiscal_year.year_start_date)?,
            parse_date(&fiscal_year.year_end_date)?,
        )
    } else {
        let from_date =
            period_start_date.ok_or_else(|| "From Date and To Date are mandatory".to_string())?;
        let to_date =
            period_end_date.ok_or_else(|| "From Date and To Date are mandatory".to_string())?;
        validate_dates(from_date, to_date)?;
        (parse_date(from_date)?, parse_date(to_date)?)
    };

    let months_to_add = match periodicity {
        "Yearly" => 12,
        "Half-Yearly" => 6,
        "Quarterly" => 3,
        "Monthly" => 1,
        _ => return Err(format!("Unsupported periodicity {periodicity}")),
    };

    let mut period_list = Vec::new();
    let mut start_date = year_start_date;
    let months = get_months(&format_date(year_start_date), &format_date(year_end_date))?;
    let iterations = (months + months_to_add - 1) / months_to_add;

    for i in 0..iterations {
        let from_date = start_date;
        let mut to_date = if i == 0 && filter_based_on == "Date Range" {
            add_months(first_day(start_date), months_to_add)
        } else {
            add_months(start_date, months_to_add)
        };
        start_date = to_date;
        to_date = add_days(to_date, -1);
        if to_date > year_end_date {
            to_date = year_end_date;
        }

        let to_date_fiscal_year = (!ignore_fiscal_year).then(|| fiscal_year_for(to_date));
        let from_date_fiscal_year_start_date =
            (!ignore_fiscal_year).then(|| fiscal_year_start_for(from_date));

        period_list.push(Period {
            key: String::new(),
            label: String::new(),
            from_date: format_date(from_date),
            to_date: format_date(to_date),
            year_start_date: format_date(year_start_date),
            year_end_date: format_date(year_end_date),
            to_date_fiscal_year,
            from_date_fiscal_year_start_date,
        });

        if to_date == year_end_date {
            break;
        }
    }

    let first_from_date = period_list
        .first()
        .map(|period| period.from_date.clone())
        .unwrap_or_else(|| format_date(year_start_date));
    for period in &mut period_list {
        let to_date = parse_date(&period.to_date)?;
        period.key = format!(
            "{}_{}",
            month_abbr(to_date.month).to_lowercase(),
            to_date.year
        );
        period.label = if periodicity == "Monthly" && !accumulated_values {
            format!("{} {}", month_abbr(to_date.month), to_date.year)
        } else if !accumulated_values {
            get_label(periodicity, &period.from_date, &period.to_date)?
        } else if reset_period_on_fy_change {
            get_label(
                periodicity,
                period
                    .from_date_fiscal_year_start_date
                    .as_deref()
                    .unwrap_or(&period.from_date),
                &period.to_date,
            )?
        } else {
            get_label(periodicity, &first_from_date, &period.to_date)?
        };
    }

    Ok(period_list)
}

pub fn get_fiscal_year_data(
    from_fiscal_year: &str,
    to_fiscal_year: &str,
    fiscal_years: &BTreeMap<String, FiscalYearData>,
) -> Result<FiscalYearData, String> {
    let from = fiscal_years
        .get(from_fiscal_year)
        .ok_or_else(|| "Start Year and End Year are mandatory".to_string())?;
    let to = fiscal_years
        .get(to_fiscal_year)
        .ok_or_else(|| "Start Year and End Year are mandatory".to_string())?;

    Ok(FiscalYearData {
        year_start_date: from.year_start_date.clone(),
        year_end_date: to.year_end_date.clone(),
    })
}

pub fn validate_fiscal_year(fiscal_year: &FiscalYearData) -> Result<(), String> {
    if fiscal_year.year_start_date.is_empty() || fiscal_year.year_end_date.is_empty() {
        return Err("Start Year and End Year are mandatory".to_string());
    }
    if fiscal_year.year_end_date < fiscal_year.year_start_date {
        return Err("End Year cannot be before Start Year".to_string());
    }
    Ok(())
}

pub fn validate_dates(from_date: &str, to_date: &str) -> Result<(), String> {
    if from_date.is_empty() || to_date.is_empty() {
        return Err("From Date and To Date are mandatory".to_string());
    }
    if to_date < from_date {
        return Err("To Date cannot be less than From Date".to_string());
    }
    Ok(())
}

pub fn get_months(start_date: &str, end_date: &str) -> Result<i32, String> {
    let start_date = parse_date(start_date)?;
    let end_date = parse_date(end_date)?;
    Ok((12 * end_date.year + end_date.month as i32)
        - (12 * start_date.year + start_date.month as i32)
        + 1)
}

pub fn get_label(periodicity: &str, from_date: &str, to_date: &str) -> Result<String, String> {
    let from_date = parse_date(from_date)?;
    let to_date = parse_date(to_date)?;
    if periodicity == "Yearly" {
        if from_date.year == to_date.year {
            Ok(from_date.year.to_string())
        } else {
            Ok(format!("{}-{}", from_date.year, to_date.year))
        }
    } else {
        Ok(format!(
            "{} {:02}-{} {:02}",
            month_abbr(from_date.month),
            from_date.year % 100,
            month_abbr(to_date.month),
            to_date.year % 100
        ))
    }
}

pub fn calculate_values(
    accounts_by_name: &mut BTreeMap<String, AccountRow>,
    gl_entries_by_account: &BTreeMap<String, Vec<FinancialStatementGlEntry>>,
    period_list: &[Period],
    accumulated_values: bool,
    ignore_accumulated_values_for_fy: bool,
) -> Result<(), String> {
    for account in accounts_by_name.values_mut() {
        account.opening_balance = 0.0;
        account.values.clear();
    }

    for entries in gl_entries_by_account.values() {
        for entry in entries {
            let account = accounts_by_name
                .get_mut(&entry.account)
                .ok_or_else(|| format!("Could not retrieve information for {}.", entry.account))?;
            let amount = entry.debit - entry.credit;

            for period in period_list {
                if entry.posting_date <= period.to_date
                    && (accumulated_values || entry.posting_date >= period.from_date)
                    && (!ignore_accumulated_values_for_fy
                        || period.to_date_fiscal_year.as_deref()
                            == Some(entry.fiscal_year.as_str()))
                {
                    *account.values.entry(period.key.clone()).or_default() += amount;
                }
            }

            if let Some(first_period) = period_list.first() {
                if entry.posting_date < first_period.year_start_date {
                    account.opening_balance += amount;
                }
            }
        }
    }

    Ok(())
}

pub fn accumulate_values_into_parents(
    accounts: &[AccountRow],
    accounts_by_name: &mut BTreeMap<String, AccountRow>,
    period_list: &[Period],
) {
    for account in accounts.iter().rev() {
        if let Some(parent_account) = account.parent_account.as_ref() {
            let child = accounts_by_name.get(&account.name).cloned();
            if let (Some(child), Some(parent)) = (child, accounts_by_name.get_mut(parent_account)) {
                for period in period_list {
                    *parent.values.entry(period.key.clone()).or_default() +=
                        child.values.get(&period.key).copied().unwrap_or_default();
                }
                parent.opening_balance += child.opening_balance;
            }
        }
    }
}

pub fn prepare_data(
    accounts: &[AccountRow],
    balance_must_be: &str,
    period_list: &[Period],
    company_currency: &str,
    accumulated_values: bool,
) -> Vec<FinancialStatementRow> {
    let year_start_date = period_list
        .first()
        .map(|period| period.year_start_date.clone())
        .unwrap_or_default();
    let year_end_date = period_list
        .last()
        .map(|period| period.year_end_date.clone())
        .unwrap_or_default();

    accounts
        .iter()
        .map(|account| {
            let mut has_value = false;
            let mut total = 0.0;
            let mut values = BTreeMap::new();

            for period in period_list {
                let mut value = account.values.get(&period.key).copied().unwrap_or_default();
                if value != 0.0 && balance_must_be == "Credit" {
                    value *= -1.0;
                }
                value = round_to(value, 3);
                if value.abs() >= zero_cutoff() {
                    has_value = true;
                    total += value;
                }
                values.insert(period.key.clone(), value);
            }

            let total = if accumulated_values {
                period_list
                    .last()
                    .and_then(|period| account.values.get(&period.key))
                    .copied()
                    .unwrap_or_default()
            } else {
                total
            };

            FinancialStatementRow {
                account: Some(account.name.clone()),
                parent_account: account.parent_account.clone(),
                indent: account.indent as f64,
                year_start_date: Some(year_start_date.clone()),
                year_end_date: Some(year_end_date.clone()),
                currency: Some(company_currency.to_string()),
                include_in_gross: account.include_in_gross,
                account_type: account.account_type.clone(),
                is_group: account.is_group,
                opening_balance: account.opening_balance
                    * if balance_must_be == "Debit" {
                        1.0
                    } else {
                        -1.0
                    },
                account_name: Some(match account.account_number.as_deref() {
                    Some(account_number) => {
                        format!("{} - {}", account_number, account.account_name)
                    }
                    None => account.account_name.clone(),
                }),
                acc_name: Some(account.account_name.clone()),
                acc_number: account.account_number.clone(),
                values,
                total: round_to(total, 3),
                has_value,
                is_blank: false,
            }
        })
        .collect()
}

pub fn filter_out_zero_value_rows(
    data: Vec<FinancialStatementRow>,
    parent_children_map: &BTreeMap<Option<String>, Vec<String>>,
    show_zero_values: bool,
) -> Vec<FinancialStatementRow> {
    let mut accounts_to_show = BTreeSet::<String>::new();

    for row in &data {
        if show_zero_values || row.has_value {
            if let Some(account) = row.account.as_deref() {
                accounts_to_show.insert(account.to_string());
                collect_all_parents(account, parent_children_map, &mut accounts_to_show);
            }
        }
    }

    data.into_iter()
        .filter(|row| {
            row.account
                .as_ref()
                .map(|account| accounts_to_show.contains(account))
                .unwrap_or(false)
        })
        .collect()
}

pub fn add_total_row(
    out: &mut Vec<FinancialStatementRow>,
    root_type: &str,
    balance_must_be: &str,
    period_list: &[Period],
    company_currency: &str,
) {
    let mut total_row = FinancialStatementRow::total(
        &format!("'Total {} ({})'", root_type, balance_must_be),
        company_currency,
    );

    for row in out.iter() {
        if row.parent_account.is_none() {
            for period in period_list {
                *total_row.values.entry(period.key.clone()).or_default() +=
                    row.values.get(&period.key).copied().unwrap_or_default();
            }
            total_row.total += row.total;
            total_row.opening_balance += row.opening_balance;
        }
    }

    if !total_row.values.is_empty() {
        out.push(total_row);
        out.push(FinancialStatementRow::blank());
    }
}

pub fn filter_accounts(
    mut accounts: Vec<AccountRow>,
) -> (
    Vec<AccountRow>,
    BTreeMap<String, AccountRow>,
    BTreeMap<Option<String>, Vec<String>>,
) {
    accounts.sort_by_key(|account| account.lft);
    let account_map = accounts
        .into_iter()
        .map(|account| (account.name.clone(), account))
        .collect::<BTreeMap<_, _>>();
    let mut parent_children_map = BTreeMap::<Option<String>, Vec<String>>::new();

    for account in account_map.values() {
        parent_children_map
            .entry(account.parent_account.clone())
            .or_default()
            .push(account.name.clone());
    }

    let mut filtered_accounts = Vec::new();
    add_accounts_to_list(
        None,
        0,
        &account_map,
        &parent_children_map,
        &mut filtered_accounts,
    );
    let accounts_by_name = filtered_accounts
        .iter()
        .cloned()
        .map(|account| (account.name.clone(), account))
        .collect();

    (filtered_accounts, accounts_by_name, parent_children_map)
}

pub fn get_columns(
    periodicity: &str,
    period_list: &[Period],
    accumulated_values: bool,
    company: Option<&str>,
    cash_flow: bool,
) -> Vec<ReportColumn> {
    let mut columns = vec![ReportColumn::link(
        if cash_flow { "Section" } else { "Account" },
        if cash_flow { "section" } else { "account" },
        "Account",
        300,
        false,
    )];

    if !cash_flow {
        columns.push(ReportColumn::data("Account Name", "acc_name", 250, true));
        columns.push(ReportColumn::data(
            "Account Number",
            "acc_number",
            120,
            true,
        ));
    }

    if company.is_some() {
        columns.push(ReportColumn::link(
            "Currency", "currency", "Currency", 0, true,
        ));
    }

    for period in period_list {
        columns
            .push(ReportColumn::currency(&period.label, "period", 150).with_fieldname(&period.key));
    }

    if periodicity != "Yearly" && !accumulated_values {
        columns.push(ReportColumn::currency("Total", "total", 150));
    }

    columns
}

pub fn get_filtered_list_for_consolidated_report(
    filters: &BTreeMap<String, String>,
    period_list: &[String],
) -> Vec<String> {
    period_list
        .iter()
        .filter(|period| filters.get("company") == Some(*period))
        .cloned()
        .collect()
}

pub fn compute_growth_view_data(data: &mut [FinancialStatementRow], columns: &[Period]) {
    let original = data.to_vec();
    for row_idx in 0..original.len() {
        for column_idx in 1..columns.len() {
            let previous_period_key = &columns[column_idx - 1].key;
            let current_period_key = &columns[column_idx].key;
            let current_period_value = original[row_idx].values.get(current_period_key).copied();
            let previous_period_value = original[row_idx]
                .values
                .get(previous_period_key)
                .copied()
                .unwrap_or_default();
            let Some(current_period_value) = current_period_value else {
                continue;
            };

            let annual_growth = if previous_period_value == 0.0 && current_period_value > 0.0 {
                1.0
            } else if previous_period_value > 0.0 {
                (current_period_value - previous_period_value) / previous_period_value
            } else {
                0.0
            };
            data[row_idx].values.insert(
                current_period_key.clone(),
                round_to(annual_growth * 100.0, 2),
            );
        }
    }
}

pub fn compute_margin_view_data(
    data: &mut [FinancialStatementRow],
    columns: &[Period],
    accumulated_values: bool,
) {
    if columns.is_empty() {
        return;
    }

    let original = data.to_vec();
    let Some(base_row) = original
        .iter()
        .find(|row| row.account_name.as_deref() == Some("Income"))
    else {
        return;
    };

    let mut keys = columns
        .iter()
        .map(|period| period.key.clone())
        .collect::<Vec<_>>();
    if !accumulated_values {
        keys.push("total".to_string());
    }

    for (row_idx, row) in original.iter().enumerate() {
        if row.is_blank {
            continue;
        }
        for key in &keys {
            let base_value = if key == "total" {
                base_row.total
            } else {
                base_row.values.get(key).copied().unwrap_or_default()
            };
            let current_value = if key == "total" {
                row.total
            } else {
                row.values.get(key).copied().unwrap_or_default()
            };

            if base_value <= 0.0 {
                data[row_idx].values.remove(key);
                continue;
            }
            data[row_idx].values.insert(
                key.clone(),
                round_to((current_value / base_value) * 100.0, 2),
            );
        }
    }
}

impl ReportColumn {
    fn with_fieldname(mut self, fieldname: &str) -> Self {
        self.fieldname = fieldname.to_string();
        self
    }
}

impl FinancialStatementRow {
    fn total(label: &str, company_currency: &str) -> Self {
        Self {
            account: Some(label.to_string()),
            parent_account: None,
            indent: 0.0,
            year_start_date: None,
            year_end_date: None,
            currency: Some(company_currency.to_string()),
            include_in_gross: false,
            account_type: None,
            is_group: false,
            opening_balance: 0.0,
            account_name: Some(label.to_string()),
            acc_name: None,
            acc_number: None,
            values: BTreeMap::new(),
            total: 0.0,
            has_value: true,
            is_blank: false,
        }
    }

    fn blank() -> Self {
        Self {
            account: None,
            parent_account: None,
            indent: 0.0,
            year_start_date: None,
            year_end_date: None,
            currency: None,
            include_in_gross: false,
            account_type: None,
            is_group: false,
            opening_balance: 0.0,
            account_name: None,
            acc_name: None,
            acc_number: None,
            values: BTreeMap::new(),
            total: 0.0,
            has_value: false,
            is_blank: true,
        }
    }
}

fn add_accounts_to_list(
    parent: Option<String>,
    level: i32,
    account_map: &BTreeMap<String, AccountRow>,
    parent_children_map: &BTreeMap<Option<String>, Vec<String>>,
    filtered_accounts: &mut Vec<AccountRow>,
) {
    if let Some(children) = parent_children_map.get(&parent) {
        let mut children = children.clone();
        children.sort_by(|left, right| {
            let left = account_map.get(left).unwrap();
            let right = account_map.get(right).unwrap();
            if parent.is_none() {
                root_sort_key(left).cmp(&root_sort_key(right))
            } else {
                left.name.cmp(&right.name)
            }
        });

        for child_name in children {
            let mut child = account_map.get(&child_name).unwrap().clone();
            child.indent = level;
            filtered_accounts.push(child.clone());
            add_accounts_to_list(
                Some(child.name),
                level + 1,
                account_map,
                parent_children_map,
                filtered_accounts,
            );
        }
    }
}

fn root_sort_key(account: &AccountRow) -> (i32, String) {
    let priority = match account.root_type.as_str() {
        "Asset" => 0,
        "Liability" => 1,
        "Equity" => 2,
        "Income" => 3,
        "Expense" => 4,
        _ => 5,
    };
    let report_priority = if account.report_type == "Balance Sheet" {
        0
    } else {
        1
    };
    (report_priority * 10 + priority, account.name.clone())
}

fn collect_all_parents(
    account: &str,
    parent_children_map: &BTreeMap<Option<String>, Vec<String>>,
    accounts_to_show: &mut BTreeSet<String>,
) {
    for (parent, children) in parent_children_map {
        if children.iter().any(|child| child == account) {
            if let Some(parent) = parent {
                accounts_to_show.insert(parent.clone());
                collect_all_parents(parent, parent_children_map, accounts_to_show);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SimpleDate {
    year: i32,
    month: u32,
    day: u32,
}

fn parse_date(value: &str) -> Result<SimpleDate, String> {
    let parts = value.split('-').collect::<Vec<_>>();
    if parts.len() != 3 {
        return Err(format!("Invalid date {value}"));
    }
    Ok(SimpleDate {
        year: parts[0]
            .parse()
            .map_err(|_| format!("Invalid date {value}"))?,
        month: parts[1]
            .parse()
            .map_err(|_| format!("Invalid date {value}"))?,
        day: parts[2]
            .parse()
            .map_err(|_| format!("Invalid date {value}"))?,
    })
}

fn format_date(date: SimpleDate) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

fn first_day(date: SimpleDate) -> SimpleDate {
    SimpleDate { day: 1, ..date }
}

fn add_months(date: SimpleDate, months: i32) -> SimpleDate {
    let total_months = date.year * 12 + date.month as i32 - 1 + months;
    let year = total_months / 12;
    let month = (total_months % 12 + 1) as u32;
    let day = date.day.min(days_in_month(year, month));
    SimpleDate { year, month, day }
}

fn add_days(date: SimpleDate, days: i32) -> SimpleDate {
    if days == -1 && date.day > 1 {
        return SimpleDate {
            day: date.day - 1,
            ..date
        };
    }
    if days == -1 && date.month > 1 {
        let month = date.month - 1;
        return SimpleDate {
            year: date.year,
            month,
            day: days_in_month(date.year, month),
        };
    }
    if days == -1 {
        return SimpleDate {
            year: date.year - 1,
            month: 12,
            day: 31,
        };
    }

    let mut date = date;
    for _ in 0..days {
        let dim = days_in_month(date.year, date.month);
        if date.day < dim {
            date.day += 1;
        } else if date.month < 12 {
            date.month += 1;
            date.day = 1;
        } else {
            date.year += 1;
            date.month = 1;
            date.day = 1;
        }
    }
    date
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn month_abbr(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

fn fiscal_year_for(date: SimpleDate) -> String {
    format!("FY{}", date.year)
}

fn fiscal_year_start_for(date: SimpleDate) -> String {
    format!("{:04}-01-01", date.year)
}

fn round_to(value: f64, precision: i32) -> f64 {
    let factor = 10_f64.powi(precision);
    (value * factor).round() / factor
}

fn zero_cutoff() -> f64 {
    0.005
}
