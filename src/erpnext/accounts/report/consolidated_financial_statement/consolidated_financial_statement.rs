use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsolidatedFilters {
    pub company: String,
    pub report: String,
    pub from_fiscal_year: String,
    pub to_fiscal_year: String,
    pub filter_based_on: String,
    pub period_start_date: Option<String>,
    pub period_end_date: Option<String>,
    pub presentation_currency: Option<String>,
    pub accumulated_in_group_company: bool,
    pub show_zero_values: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYearData {
    pub year_start_date: String,
    pub year_end_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompanyNode {
    pub name: String,
    pub parent_company: Option<String>,
    pub lft: i32,
    pub rgt: i32,
    pub default_currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountRow {
    pub name: String,
    pub company: String,
    pub parent_account: Option<String>,
    pub lft: i32,
    pub rgt: i32,
    pub root_type: String,
    pub report_type: String,
    pub account_name: String,
    pub account_number: Option<String>,
    pub is_group: bool,
    pub account_key: String,
    pub parent_account_name: Option<String>,
    pub indent: i32,
    pub company_wise_opening_bal: BTreeMap<String, f64>,
    pub opening_balance: f64,
    pub values: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidatedGlEntry {
    pub account_key: String,
    pub company: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: String,
    pub label: String,
    pub fieldtype: String,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
    pub apply_currency_formatter: bool,
    pub company_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidatedRow {
    pub account_name: Option<String>,
    pub account: Option<String>,
    pub parent_account: Option<String>,
    pub indent: f64,
    pub year_start_date: Option<String>,
    pub root_type: Option<String>,
    pub year_end_date: Option<String>,
    pub currency: Option<String>,
    pub company_wise_opening_bal: BTreeMap<String, f64>,
    pub opening_balance: f64,
    pub values: BTreeMap<String, f64>,
    pub total: f64,
    pub has_value: bool,
    pub is_blank: bool,
}

impl ConsolidatedGlEntry {
    pub fn new(
        account_key: &str,
        company: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
    ) -> Self {
        Self {
            account_key: account_key.to_string(),
            company: company.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
        }
    }
}

impl ReportColumn {
    pub fn link(label: &str, fieldname: &str, options: &str, width: u16, hidden: bool) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Link".to_string(),
            options: Some(options.to_string()),
            width,
            hidden,
            apply_currency_formatter: false,
            company_name: None,
        }
    }

    pub fn currency(
        label: &str,
        fieldname: &str,
        width: u16,
        apply_currency_formatter: bool,
        company_name: Option<&str>,
    ) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Currency".to_string(),
            options: Some("currency".to_string()),
            width,
            hidden: false,
            apply_currency_formatter,
            company_name: company_name.map(str::to_string),
        }
    }
}

pub fn get_companies(
    filters: &ConsolidatedFilters,
    company_tree: &[CompanyNode],
) -> Result<(Vec<String>, BTreeMap<String, Vec<String>>), String> {
    let all_companies = get_subsidiary_companies(&filters.company, company_tree)?;
    let mut companies = BTreeMap::new();
    companies.insert(filters.company.clone(), all_companies.clone());

    for company in &all_companies {
        if !companies.contains_key(company) {
            companies.insert(
                company.clone(),
                get_subsidiary_companies(company, company_tree)?,
            );
        }
    }

    Ok((all_companies, companies))
}

pub fn get_subsidiary_companies(
    company: &str,
    company_tree: &[CompanyNode],
) -> Result<Vec<String>, String> {
    let node = company_tree
        .iter()
        .find(|node| node.name == company)
        .ok_or_else(|| format!("Company {company} not found"))?;

    let mut companies = company_tree
        .iter()
        .filter(|candidate| candidate.lft >= node.lft && candidate.rgt <= node.rgt)
        .cloned()
        .collect::<Vec<_>>();
    companies.sort_by(|left, right| left.lft.cmp(&right.lft).then(left.rgt.cmp(&right.rgt)));
    Ok(companies.into_iter().map(|node| node.name).collect())
}

pub fn get_company_columns(
    companies: &[String],
    company_tree: &[CompanyNode],
    filters: &ConsolidatedFilters,
) -> Result<Vec<ReportColumn>, String> {
    companies
        .iter()
        .map(|company| {
            let node = company_tree
                .iter()
                .find(|node| node.name == *company)
                .ok_or_else(|| format!("Company {company} not found"))?;
            let apply_currency_formatter = filters.presentation_currency.is_none();
            let currency = filters
                .presentation_currency
                .as_deref()
                .unwrap_or(&node.default_currency);
            Ok(ReportColumn::currency(
                &format!("{company} ({currency})"),
                company,
                150,
                apply_currency_formatter,
                Some(company),
            ))
        })
        .collect()
}

pub fn get_columns(company_columns: &[ReportColumn]) -> Vec<ReportColumn> {
    let mut columns = vec![
        ReportColumn::link("Account", "account", "Account", 300, false),
        ReportColumn::link("Currency", "currency", "Currency", 0, true),
    ];
    columns.extend(company_columns.iter().cloned());
    columns
}

pub fn update_parent_account_names(accounts: &mut [AccountRow]) {
    let mut name_to_account_map = BTreeMap::new();

    for account in accounts.iter_mut() {
        account.account_key = account_key(account);
        name_to_account_map.insert(account.name.clone(), account.account_key.clone());
    }

    for account in accounts {
        if let Some(parent_account) = account.parent_account.as_ref() {
            account.parent_account_name = name_to_account_map.get(parent_account).cloned();
        }
    }
}

pub fn filter_accounts(
    accounts: Vec<AccountRow>,
) -> (
    Vec<AccountRow>,
    BTreeMap<String, AccountRow>,
    BTreeMap<Option<String>, Vec<String>>,
) {
    let mut parent_children_map = BTreeMap::<Option<String>, Vec<String>>::new();
    let mut accounts_by_name = BTreeMap::<String, AccountRow>::new();
    let mut added_accounts = BTreeSet::<String>::new();

    for mut account in accounts {
        if added_accounts.contains(&account.account_key) {
            continue;
        }
        added_accounts.insert(account.account_key.clone());
        account.company_wise_opening_bal = BTreeMap::new();
        parent_children_map
            .entry(account.parent_account_name.clone())
            .or_default()
            .push(account.account_key.clone());
        accounts_by_name.insert(account.account_key.clone(), account);
    }

    let mut filtered_accounts = Vec::new();
    add_to_list(
        None,
        0,
        &accounts_by_name,
        &parent_children_map,
        &mut filtered_accounts,
    );

    (filtered_accounts, accounts_by_name, parent_children_map)
}

pub fn calculate_values(
    accounts_by_name: &mut BTreeMap<String, AccountRow>,
    gl_entries_by_account: &BTreeMap<String, Vec<ConsolidatedGlEntry>>,
    companies: &BTreeMap<String, Vec<String>>,
    filters: &ConsolidatedFilters,
    fiscal_year: &FiscalYearData,
) {
    let start_date = if filters.filter_based_on == "Fiscal Year" {
        fiscal_year.year_start_date.as_str()
    } else {
        filters.period_start_date.as_deref().unwrap_or("")
    };

    for account in accounts_by_name.values_mut() {
        account.values.clear();
        account.opening_balance = 0.0;
        account.company_wise_opening_bal.clear();
    }

    for entries in gl_entries_by_account.values() {
        for entry in entries {
            if let Some(account) = accounts_by_name.get_mut(&entry.account_key) {
                let amount = entry.debit - entry.credit;
                for (company, subsidiaries) in companies {
                    if entry.company == *company
                        || (filters.accumulated_in_group_company
                            && subsidiaries
                                .iter()
                                .any(|subsidiary| subsidiary == &entry.company))
                    {
                        *account.values.entry(company.clone()).or_default() += amount;
                        if entry.posting_date.as_str() < start_date {
                            *account
                                .company_wise_opening_bal
                                .entry(company.clone())
                                .or_default() += amount;
                        }
                    }
                }
                if entry.posting_date.as_str() < start_date {
                    account.opening_balance += amount;
                }
            }
        }
    }
}

pub fn accumulate_values_into_parents(
    accounts: &[AccountRow],
    accounts_by_name: &mut BTreeMap<String, AccountRow>,
    companies: &BTreeMap<String, Vec<String>>,
) {
    for account in accounts.iter().rev() {
        if let Some(parent) = account.parent_account_name.as_ref() {
            let child = accounts_by_name.get(&account.account_key).cloned();
            if let (Some(child), Some(parent)) = (child, accounts_by_name.get_mut(parent)) {
                for company in companies.keys() {
                    *parent.values.entry(company.clone()).or_default() +=
                        child.values.get(company).copied().unwrap_or_default();
                    *parent
                        .company_wise_opening_bal
                        .entry(company.clone())
                        .or_default() += child
                        .company_wise_opening_bal
                        .get(company)
                        .copied()
                        .unwrap_or_default();
                }
                parent.opening_balance += child.opening_balance;
            }
        }
    }
}

pub fn prepare_data(
    accounts: &[AccountRow],
    start_date: Option<&str>,
    end_date: &str,
    balance_must_be: &str,
    companies: &BTreeMap<String, Vec<String>>,
    company_currency: &str,
    filters: &ConsolidatedFilters,
) -> Vec<ConsolidatedRow> {
    accounts
        .iter()
        .map(|account| {
            let mut has_value = false;
            let mut total = 0.0;
            let mut values = BTreeMap::new();

            for company in companies.keys() {
                let mut value = account.values.get(company).copied().unwrap_or_default();
                if value != 0.0 && balance_must_be == "Credit" {
                    value *= -1.0;
                }
                value = round_to(value, 3);
                if value.abs() >= zero_cutoff(filters) {
                    has_value = true;
                    total += value;
                }
                values.insert(company.clone(), value);
            }

            ConsolidatedRow {
                account_name: Some(account_label(account)),
                account: Some(account.name.clone()),
                parent_account: account.parent_account.clone(),
                indent: account.indent as f64,
                year_start_date: start_date.map(str::to_string),
                root_type: Some(account.root_type.clone()),
                year_end_date: Some(end_date.to_string()),
                currency: filters.presentation_currency.clone(),
                company_wise_opening_bal: account.company_wise_opening_bal.clone(),
                opening_balance: account.opening_balance
                    * if balance_must_be == "Debit" {
                        1.0
                    } else {
                        -1.0
                    },
                values,
                total,
                has_value,
                is_blank: false,
            }
        })
        .map(|mut row| {
            if row.currency.is_none() {
                row.currency = Some(company_currency.to_string());
            }
            row
        })
        .collect()
}

pub fn add_total_row(
    out: &mut Vec<ConsolidatedRow>,
    root_type: &str,
    balance_must_be: &str,
    companies: &BTreeMap<String, Vec<String>>,
    company_currency: &str,
) {
    let label = format!("'Total {} ({})'", root_type, balance_must_be);
    let mut total_row = ConsolidatedRow::total(&label, company_currency);

    for row in out.iter_mut() {
        if row.parent_account.is_none() {
            for company in companies.keys() {
                *total_row.values.entry(company.clone()).or_default() +=
                    row.values.get(company).copied().unwrap_or_default();
            }
            total_row.total += row.total;
            row.total = 0.0;
        }
    }

    if !total_row.values.is_empty() {
        out.push(total_row);
        out.push(ConsolidatedRow::blank());
    }
}

pub fn prepare_companywise_opening_balance(
    asset_data: &[AccountRow],
    liability_data: &[AccountRow],
    equity_data: &[AccountRow],
    companies: &BTreeMap<String, Vec<String>>,
) -> (Option<String>, BTreeMap<String, f64>) {
    let mut opening_balance = BTreeMap::new();

    for company in companies.keys() {
        let mut opening_value = 0.0;
        for data in [asset_data, liability_data, equity_data] {
            if let Some(first) = data.first() {
                if let Some(account_name) = get_root_account_name(&first.root_type, company, data) {
                    opening_value +=
                        get_opening_balance(&account_name, data, company).unwrap_or_default();
                }
            }
        }
        opening_balance.insert(company.clone(), opening_value);
    }

    if opening_balance.is_empty() {
        (None, BTreeMap::new())
    } else {
        (
            Some("Previous Financial Year is not closed".to_string()),
            opening_balance,
        )
    }
}

pub fn get_opening_balance(account_name: &str, data: &[AccountRow], company: &str) -> Option<f64> {
    data.iter()
        .find(|row| row.account_name == account_name)
        .and_then(|row| row.company_wise_opening_bal.get(company).copied())
}

pub fn get_root_account_name(
    root_type: &str,
    company: &str,
    accounts: &[AccountRow],
) -> Option<String> {
    accounts
        .iter()
        .find(|account| {
            account.root_type == root_type
                && account.is_group
                && account.company == company
                && account.parent_account.is_none()
        })
        .map(|account| account.account_name.clone())
}

fn add_to_list(
    parent: Option<String>,
    level: i32,
    accounts_by_name: &BTreeMap<String, AccountRow>,
    parent_children_map: &BTreeMap<Option<String>, Vec<String>>,
    filtered_accounts: &mut Vec<AccountRow>,
) {
    if let Some(children) = parent_children_map.get(&parent) {
        let mut children = children.clone();
        children.sort_by(|left, right| {
            let left = accounts_by_name.get(left).unwrap();
            let right = accounts_by_name.get(right).unwrap();
            if parent.is_none() {
                root_sort_key(left).cmp(&root_sort_key(right))
            } else {
                left.account_key.cmp(&right.account_key)
            }
        });

        for child_name in children {
            let mut child = accounts_by_name.get(&child_name).unwrap().clone();
            child.indent = level;
            filtered_accounts.push(child.clone());
            add_to_list(
                Some(child.account_key),
                level + 1,
                accounts_by_name,
                parent_children_map,
                filtered_accounts,
            );
        }
    }
}

fn account_key(account: &AccountRow) -> String {
    match account.account_number.as_deref() {
        Some(account_number) => format!("{} - {}", account_number, account.account_name),
        None => account.account_name.clone(),
    }
}

fn account_label(account: &AccountRow) -> String {
    match account.account_number.as_deref() {
        Some(account_number) => format!("{} - {}", account_number, account.account_name),
        None => account.account_name.clone(),
    }
}

fn root_sort_key(account: &AccountRow) -> (i32, String) {
    let root_priority = match account.root_type.as_str() {
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
    (
        report_priority * 10 + root_priority,
        account.account_key.clone(),
    )
}

fn zero_cutoff(_filters: &ConsolidatedFilters) -> f64 {
    0.005
}

fn round_to(value: f64, precision: i32) -> f64 {
    let factor = 10_f64.powi(precision);
    (value * factor).round() / factor
}

impl ConsolidatedRow {
    fn total(label: &str, company_currency: &str) -> Self {
        Self {
            account_name: Some(label.to_string()),
            account: Some(label.to_string()),
            parent_account: None,
            indent: 0.0,
            year_start_date: None,
            root_type: None,
            year_end_date: None,
            currency: Some(company_currency.to_string()),
            company_wise_opening_bal: BTreeMap::new(),
            opening_balance: 0.0,
            values: BTreeMap::new(),
            total: 0.0,
            has_value: true,
            is_blank: false,
        }
    }

    fn blank() -> Self {
        Self {
            account_name: None,
            account: None,
            parent_account: None,
            indent: 0.0,
            year_start_date: None,
            root_type: None,
            year_end_date: None,
            currency: None,
            company_wise_opening_bal: BTreeMap::new(),
            opening_balance: 0.0,
            values: BTreeMap::new(),
            total: 0.0,
            has_value: false,
            is_blank: true,
        }
    }
}
