use std::collections::{BTreeMap, BTreeSet};

const VALUE_FIELDS: [&str; 6] = [
    "opening_debit",
    "opening_credit",
    "debit",
    "credit",
    "closing_debit",
    "closing_credit",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrialBalanceFilters {
    pub company: String,
    pub fiscal_year: String,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub year_start_date: Option<String>,
    pub year_end_date: Option<String>,
    pub presentation_currency: Option<String>,
    pub show_net_values: bool,
    pub show_group_accounts: bool,
    pub show_zero_values: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYear {
    pub year_start_date: String,
    pub year_end_date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountRow {
    pub company: String,
    pub name: String,
    pub account_number: Option<String>,
    pub parent_account: Option<String>,
    pub account_name: String,
    pub root_type: String,
    pub report_type: String,
    pub account_type: Option<String>,
    pub is_group: bool,
    pub lft: i32,
    pub rgt: i32,
    pub opening_debit: f64,
    pub opening_credit: f64,
    pub debit: f64,
    pub credit: f64,
    pub closing_debit: f64,
    pub closing_credit: f64,
    pub indent: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpeningBalance {
    pub account: String,
    pub opening_debit: f64,
    pub opening_credit: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub company: String,
    pub account: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub is_opening: String,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialBalanceInput {
    pub company_currency: String,
    pub fiscal_years: BTreeMap<String, FiscalYear>,
    pub accounts: Vec<AccountRow>,
    pub opening_balances: Vec<OpeningBalance>,
    pub gl_entries: Vec<GlEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: &'static str,
    pub label: String,
    pub fieldtype: &'static str,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialBalanceRow {
    pub account: Option<String>,
    pub parent_account: Option<String>,
    pub indent: i32,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub currency: Option<String>,
    pub is_group_account: bool,
    pub acc_name: Option<String>,
    pub acc_number: Option<String>,
    pub account_name: Option<String>,
    pub warn_if_negative: bool,
    pub opening_debit: f64,
    pub opening_credit: f64,
    pub debit: f64,
    pub credit: f64,
    pub closing_debit: f64,
    pub closing_credit: f64,
    pub has_value: bool,
    pub is_blank: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialBalanceReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<TrialBalanceRow>,
}

impl AccountRow {
    pub fn with_amounts(
        mut self,
        opening_debit: f64,
        opening_credit: f64,
        debit: f64,
        credit: f64,
    ) -> Self {
        self.opening_debit = opening_debit;
        self.opening_credit = opening_credit;
        self.debit = debit;
        self.credit = credit;
        self.closing_debit = opening_debit + debit;
        self.closing_credit = opening_credit + credit;
        self
    }
}

impl OpeningBalance {
    pub fn new(account: &str, opening_debit: f64, opening_credit: f64) -> Self {
        Self {
            account: account.to_string(),
            opening_debit,
            opening_credit,
        }
    }
}

impl GlEntry {
    pub fn new(
        company: &str,
        account: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
        is_opening: &str,
        is_cancelled: bool,
    ) -> Self {
        Self {
            company: company.to_string(),
            account: account.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
            is_opening: is_opening.to_string(),
            is_cancelled,
        }
    }
}

impl ReportColumn {
    pub fn link(
        label: &str,
        fieldname: &'static str,
        options: &str,
        width: u16,
        hidden: bool,
    ) -> Self {
        Self {
            fieldname,
            label: label.to_string(),
            fieldtype: "Link",
            options: Some(options.to_string()),
            width,
            hidden,
        }
    }

    pub fn data(label: &str, fieldname: &'static str, width: u16, hidden: bool) -> Self {
        Self {
            fieldname,
            label: label.to_string(),
            fieldtype: "Data",
            options: None,
            width,
            hidden,
        }
    }

    pub fn currency(label: &str, fieldname: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label: label.to_string(),
            fieldtype: "Currency",
            options: Some("currency".to_string()),
            width,
            hidden: false,
        }
    }
}

pub fn execute(
    mut filters: TrialBalanceFilters,
    input: TrialBalanceInput,
) -> Result<TrialBalanceReport, String> {
    validate_filters(&mut filters, &input)?;
    let rows = get_data(&filters, &input);
    Ok(TrialBalanceReport {
        columns: get_columns(),
        rows,
    })
}

pub fn validate_filters(
    filters: &mut TrialBalanceFilters,
    input: &TrialBalanceInput,
) -> Result<(), String> {
    if filters.fiscal_year.is_empty() {
        return Err("Fiscal Year is required".to_string());
    }

    let fiscal_year = input
        .fiscal_years
        .get(&filters.fiscal_year)
        .ok_or_else(|| format!("Fiscal Year {} does not exist", filters.fiscal_year))?;

    filters.year_start_date = Some(fiscal_year.year_start_date.clone());
    filters.year_end_date = Some(fiscal_year.year_end_date.clone());

    if filters.from_date.is_none() {
        filters.from_date = filters.year_start_date.clone();
    }
    if filters.to_date.is_none() {
        filters.to_date = filters.year_end_date.clone();
    }

    if filters.from_date > filters.to_date {
        return Err("From Date cannot be greater than To Date".to_string());
    }

    if filters.from_date < filters.year_start_date || filters.from_date > filters.year_end_date {
        filters.from_date = filters.year_start_date.clone();
    }
    if filters.to_date < filters.year_start_date || filters.to_date > filters.year_end_date {
        filters.to_date = filters.year_end_date.clone();
    }

    Ok(())
}

pub fn get_data(filters: &TrialBalanceFilters, input: &TrialBalanceInput) -> Vec<TrialBalanceRow> {
    let (mut accounts, accounts_by_name, parent_children_map) =
        filter_accounts(company_accounts(filters, input));
    let opening_balances = get_opening_balances(input);
    let gl_entries_by_account = set_gl_entries_by_account(filters, input);

    calculate_values(
        &mut accounts,
        &gl_entries_by_account,
        &opening_balances,
        filters.show_net_values,
    );
    accumulate_values_into_parents(&mut accounts, &accounts_by_name);

    let currency = filters
        .presentation_currency
        .clone()
        .unwrap_or_else(|| input.company_currency.clone());
    let mut data = prepare_data(&accounts, filters, &currency);
    if !filters.show_group_accounts {
        data = hide_group_accounts(data);
    }

    let total_row = calculate_total_row(&data, &currency, filters.show_group_accounts);
    data.push(TrialBalanceRow::blank());
    data.push(total_row);
    data = filter_out_zero_value_rows(data, &parent_children_map, filters.show_zero_values);
    data
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Account", "account", "Account", 300, false),
        ReportColumn::data("Account Name", "acc_name", 250, true),
        ReportColumn::data("Account Number", "acc_number", 120, true),
        ReportColumn::link("Currency", "currency", "Currency", 0, true),
        ReportColumn::currency("Opening (Dr)", "opening_debit", 120),
        ReportColumn::currency("Opening (Cr)", "opening_credit", 120),
        ReportColumn::currency("Debit", "debit", 120),
        ReportColumn::currency("Credit", "credit", 120),
        ReportColumn::currency("Closing (Dr)", "closing_debit", 120),
        ReportColumn::currency("Closing (Cr)", "closing_credit", 120),
    ]
}

pub fn prepare_opening_closing(row: &mut AccountRow) {
    let dr_or_cr = if matches!(row.root_type.as_str(), "Asset" | "Equity" | "Expense") {
        "debit"
    } else {
        "credit"
    };

    net_opening_closing(row, "opening", dr_or_cr);
    net_opening_closing(row, "closing", dr_or_cr);
}

fn company_accounts(filters: &TrialBalanceFilters, input: &TrialBalanceInput) -> Vec<AccountRow> {
    input
        .accounts
        .iter()
        .filter(|account| account.company == filters.company)
        .cloned()
        .collect()
}

fn filter_accounts(
    mut accounts: Vec<AccountRow>,
) -> (
    Vec<AccountRow>,
    BTreeMap<String, usize>,
    BTreeMap<Option<String>, Vec<String>>,
) {
    accounts.sort_by_key(|account| account.lft);
    let mut accounts_by_name = BTreeMap::new();
    let mut parent_children_map = BTreeMap::<Option<String>, Vec<String>>::new();
    for (idx, account) in accounts.iter().enumerate() {
        accounts_by_name.insert(account.name.clone(), idx);
        parent_children_map
            .entry(account.parent_account.clone())
            .or_default()
            .push(account.name.clone());
    }

    let account_map = accounts
        .into_iter()
        .map(|account| (account.name.clone(), account))
        .collect::<BTreeMap<_, _>>();
    let mut filtered_accounts = Vec::new();
    add_to_list(
        None,
        0,
        &account_map,
        &parent_children_map,
        &mut filtered_accounts,
    );
    let filtered_by_name = filtered_accounts
        .iter()
        .enumerate()
        .map(|(idx, account)| (account.name.clone(), idx))
        .collect();

    (filtered_accounts, filtered_by_name, parent_children_map)
}

fn add_to_list(
    parent: Option<String>,
    level: i32,
    account_map: &BTreeMap<String, AccountRow>,
    parent_children_map: &BTreeMap<Option<String>, Vec<String>>,
    out: &mut Vec<AccountRow>,
) {
    if let Some(children) = parent_children_map.get(&parent) {
        let mut sorted = children.clone();
        sorted.sort_by(|left, right| {
            let left_account = account_map.get(left).unwrap();
            let right_account = account_map.get(right).unwrap();
            left_account.lft.cmp(&right_account.lft)
        });
        for child_name in sorted {
            let mut child = account_map.get(&child_name).unwrap().clone();
            child.indent = level;
            out.push(child.clone());
            add_to_list(
                Some(child.name),
                level + 1,
                account_map,
                parent_children_map,
                out,
            );
        }
    }
}

fn get_opening_balances(input: &TrialBalanceInput) -> BTreeMap<String, OpeningBalance> {
    let mut opening = BTreeMap::<String, OpeningBalance>::new();
    for balance in &input.opening_balances {
        let entry = opening
            .entry(balance.account.clone())
            .or_insert_with(|| OpeningBalance::new(&balance.account, 0.0, 0.0));
        entry.opening_debit += balance.opening_debit;
        entry.opening_credit += balance.opening_credit;
    }
    opening
}

fn set_gl_entries_by_account(
    filters: &TrialBalanceFilters,
    input: &TrialBalanceInput,
) -> BTreeMap<String, Vec<GlEntry>> {
    let from_date = filters.from_date.as_deref().unwrap_or_default();
    let to_date = filters.to_date.as_deref().unwrap_or_default();
    let mut entries = BTreeMap::<String, Vec<GlEntry>>::new();

    for entry in input.gl_entries.iter().filter(|entry| {
        entry.company == filters.company
            && !entry.is_cancelled
            && entry.is_opening != "Yes"
            && entry.posting_date.as_str() >= from_date
            && entry.posting_date.as_str() <= to_date
    }) {
        entries
            .entry(entry.account.clone())
            .or_default()
            .push(entry.clone());
    }

    entries
}

fn calculate_values(
    accounts: &mut [AccountRow],
    gl_entries_by_account: &BTreeMap<String, Vec<GlEntry>>,
    opening_balances: &BTreeMap<String, OpeningBalance>,
    show_net_values: bool,
) {
    for account in accounts {
        account.opening_debit = opening_balances
            .get(&account.name)
            .map(|balance| balance.opening_debit)
            .unwrap_or_default();
        account.opening_credit = opening_balances
            .get(&account.name)
            .map(|balance| balance.opening_credit)
            .unwrap_or_default();
        account.debit = 0.0;
        account.credit = 0.0;

        for entry in gl_entries_by_account
            .get(&account.name)
            .into_iter()
            .flatten()
        {
            account.debit += entry.debit;
            account.credit += entry.credit;
        }

        account.closing_debit = account.opening_debit + account.debit;
        account.closing_credit = account.opening_credit + account.credit;

        if show_net_values {
            prepare_opening_closing(account);
        }
    }
}

fn accumulate_values_into_parents(
    accounts: &mut [AccountRow],
    accounts_by_name: &BTreeMap<String, usize>,
) {
    for idx in (0..accounts.len()).rev() {
        if let Some(parent_account) = accounts[idx].parent_account.clone() {
            if let Some(parent_idx) = accounts_by_name.get(&parent_account).copied() {
                accounts[parent_idx].opening_debit += accounts[idx].opening_debit;
                accounts[parent_idx].opening_credit += accounts[idx].opening_credit;
                accounts[parent_idx].debit += accounts[idx].debit;
                accounts[parent_idx].credit += accounts[idx].credit;
                accounts[parent_idx].closing_debit += accounts[idx].closing_debit;
                accounts[parent_idx].closing_credit += accounts[idx].closing_credit;
            }
        }
    }
}

fn prepare_data(
    accounts: &[AccountRow],
    filters: &TrialBalanceFilters,
    company_currency: &str,
) -> Vec<TrialBalanceRow> {
    accounts
        .iter()
        .cloned()
        .map(|mut account| {
            if account.is_group && filters.show_net_values {
                prepare_opening_closing(&mut account);
            }
            TrialBalanceRow::from_account(&account, filters, company_currency)
        })
        .collect()
}

fn filter_out_zero_value_rows(
    data: Vec<TrialBalanceRow>,
    parent_children_map: &BTreeMap<Option<String>, Vec<String>>,
    show_zero_values: bool,
) -> Vec<TrialBalanceRow> {
    if show_zero_values {
        return data;
    }

    let mut accounts_to_show = BTreeSet::<String>::new();
    for row in &data {
        if row.has_value {
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

fn hide_group_accounts(data: Vec<TrialBalanceRow>) -> Vec<TrialBalanceRow> {
    data.into_iter()
        .filter_map(|mut row| {
            if row.is_group_account {
                None
            } else {
                row.indent = 0;
                Some(row)
            }
        })
        .collect()
}

fn calculate_total_row(
    data: &[TrialBalanceRow],
    company_currency: &str,
    show_group_accounts: bool,
) -> TrialBalanceRow {
    let mut total_row = TrialBalanceRow::total(company_currency);

    for row in data {
        if !show_group_accounts || row.parent_account.is_none() {
            for field in VALUE_FIELDS {
                total_row.add_field(field, row.get_field(field));
            }
        }
    }

    total_row
}

impl TrialBalanceRow {
    fn from_account(
        account: &AccountRow,
        filters: &TrialBalanceFilters,
        company_currency: &str,
    ) -> Self {
        let has_value = [
            account.opening_debit,
            account.opening_credit,
            account.debit,
            account.credit,
            account.closing_debit,
            account.closing_credit,
        ]
        .iter()
        .any(|value| value.abs() >= 0.005);

        Self {
            account: Some(account.name.clone()),
            parent_account: account.parent_account.clone(),
            indent: account.indent,
            from_date: filters.from_date.clone(),
            to_date: filters.to_date.clone(),
            currency: Some(company_currency.to_string()),
            is_group_account: account.is_group,
            acc_name: Some(account.account_name.clone()),
            acc_number: account.account_number.clone(),
            account_name: Some(match account.account_number.as_deref() {
                Some(account_number) => format!("{} - {}", account_number, account.account_name),
                None => account.account_name.clone(),
            }),
            warn_if_negative: false,
            opening_debit: account.opening_debit,
            opening_credit: account.opening_credit,
            debit: account.debit,
            credit: account.credit,
            closing_debit: account.closing_debit,
            closing_credit: account.closing_credit,
            has_value,
            is_blank: false,
        }
    }

    fn blank() -> Self {
        Self {
            account: None,
            parent_account: None,
            indent: 0,
            from_date: None,
            to_date: None,
            currency: None,
            is_group_account: false,
            acc_name: None,
            acc_number: None,
            account_name: None,
            warn_if_negative: false,
            opening_debit: 0.0,
            opening_credit: 0.0,
            debit: 0.0,
            credit: 0.0,
            closing_debit: 0.0,
            closing_credit: 0.0,
            has_value: false,
            is_blank: true,
        }
    }

    fn total(company_currency: &str) -> Self {
        Self {
            account: Some("'Total'".to_string()),
            parent_account: None,
            indent: 0,
            from_date: None,
            to_date: None,
            currency: Some(company_currency.to_string()),
            is_group_account: false,
            acc_name: None,
            acc_number: None,
            account_name: Some("'Total'".to_string()),
            warn_if_negative: true,
            opening_debit: 0.0,
            opening_credit: 0.0,
            debit: 0.0,
            credit: 0.0,
            closing_debit: 0.0,
            closing_credit: 0.0,
            has_value: true,
            is_blank: false,
        }
    }

    fn get_field(&self, field: &str) -> f64 {
        match field {
            "opening_debit" => self.opening_debit,
            "opening_credit" => self.opening_credit,
            "debit" => self.debit,
            "credit" => self.credit,
            "closing_debit" => self.closing_debit,
            "closing_credit" => self.closing_credit,
            _ => 0.0,
        }
    }

    fn add_field(&mut self, field: &str, value: f64) {
        match field {
            "opening_debit" => self.opening_debit += value,
            "opening_credit" => self.opening_credit += value,
            "debit" => self.debit += value,
            "credit" => self.credit += value,
            "closing_debit" => self.closing_debit += value,
            "closing_credit" => self.closing_credit += value,
            _ => {}
        }
    }
}

fn net_opening_closing(row: &mut AccountRow, col_type: &str, dr_or_cr: &str) {
    let debit = if col_type == "opening" {
        &mut row.opening_debit
    } else {
        &mut row.closing_debit
    };
    let credit = if col_type == "opening" {
        &mut row.opening_credit
    } else {
        &mut row.closing_credit
    };

    if dr_or_cr == "debit" {
        *debit -= *credit;
        if *debit < 0.0 {
            *credit = debit.abs();
            *debit = 0.0;
        } else {
            *credit = 0.0;
        }
    } else {
        *credit -= *debit;
        if *credit < 0.0 {
            *debit = credit.abs();
            *credit = 0.0;
        } else {
            *debit = 0.0;
        }
    }
}
