use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionWiseFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub dimension: String,
    pub finance_book: Option<String>,
    pub include_default_book_entries: bool,
    pub default_finance_book: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionMeta {
    pub has_company: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionRecord {
    pub name: String,
    pub company: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DimensionWiseAccount {
    pub name: String,
    pub account_number: Option<String>,
    pub parent_account: Option<String>,
    pub lft: i32,
    pub rgt: i32,
    pub root_type: String,
    pub report_type: String,
    pub account_name: String,
    pub is_group: bool,
    pub indent: usize,
    values: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DimensionWiseGlEntry {
    pub account: String,
    pub company: String,
    pub dimension_value: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: String,
    pub label: String,
    pub fieldtype: &'static str,
    pub options: String,
    pub width: Option<u16>,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DimensionWiseRow {
    pub account: String,
    pub is_group: bool,
    pub parent_account: Option<String>,
    pub indent: usize,
    pub from_date: String,
    pub to_date: String,
    pub currency: String,
    pub account_name: String,
    pub values: BTreeMap<String, f64>,
    pub has_value: bool,
    pub total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DimensionWiseReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Option<Vec<DimensionWiseRow>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionWiseQueryPlan {
    pub account_doctype: &'static str,
    pub account_fields: Vec<&'static str>,
    pub account_order_by: &'static str,
    pub account_filters: Vec<&'static str>,
    pub gl_doctype: &'static str,
    pub gl_fields: Vec<String>,
    pub gl_conditions: Vec<String>,
    pub gl_order_by: &'static str,
    pub finance_book: String,
    pub company_default_finance_book: Option<String>,
    pub dimensions: Vec<String>,
}

impl DimensionRecord {
    pub fn new(name: &str, company: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            company: company.map(str::to_string),
        }
    }
}

impl DimensionWiseAccount {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        account_number: Option<&str>,
        parent_account: Option<&str>,
        lft: i32,
        rgt: i32,
        root_type: &str,
        report_type: &str,
        account_name: &str,
        is_group: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            account_number: account_number.map(str::to_string),
            parent_account: parent_account.map(str::to_string),
            lft,
            rgt,
            root_type: root_type.to_string(),
            report_type: report_type.to_string(),
            account_name: account_name.to_string(),
            is_group,
            indent: 0,
            values: BTreeMap::new(),
        }
    }
}

impl DimensionWiseGlEntry {
    pub fn new(
        account: &str,
        company: &str,
        dimension_value: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
        is_cancelled: bool,
    ) -> Self {
        Self {
            account: account.to_string(),
            company: company.to_string(),
            dimension_value: dimension_value.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
            is_cancelled,
        }
    }
}

impl ReportColumn {
    pub fn link(label: &str, fieldname: &str, options: &str, width: u16) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Link",
            options: options.to_string(),
            width: Some(width),
            hidden: false,
        }
    }

    pub fn hidden_link(label: &str, fieldname: &str, options: &str) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Link",
            options: options.to_string(),
            width: None,
            hidden: true,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Currency",
            options: "currency".to_string(),
            width: Some(width),
            hidden: false,
        }
    }
}

impl DimensionWiseQueryPlan {
    pub fn for_filters(filters: &DimensionWiseFilters, dimensions: &[String]) -> Self {
        let dimension_field = scrub(&filters.dimension);
        let mut gl_fields = vec![
            "posting_date".to_string(),
            "account".to_string(),
            dimension_field.clone(),
            "debit".to_string(),
            "credit".to_string(),
            "is_opening".to_string(),
            "fiscal_year".to_string(),
            "debit_in_account_currency".to_string(),
            "credit_in_account_currency".to_string(),
            "account_currency".to_string(),
        ];
        gl_fields.shrink_to_fit();

        Self {
            account_doctype: "Account",
            account_fields: vec![
                "name",
                "account_number",
                "parent_account",
                "lft",
                "rgt",
                "root_type",
                "report_type",
                "account_name",
                "include_in_gross",
                "account_type",
                "is_group",
            ],
            account_order_by: "lft",
            account_filters: vec!["company = filters.company"],
            gl_doctype: "GL Entry",
            gl_fields,
            gl_conditions: vec![
                "company = filters.company".to_string(),
                format!("{dimension_field} in %(dimensions)s"),
                "account in selected account tree".to_string(),
                "posting_date >= filters.from_date".to_string(),
                "posting_date <= filters.to_date".to_string(),
                "is_cancelled = 0".to_string(),
            ],
            gl_order_by: "account, posting_date",
            finance_book: filters.finance_book.clone().unwrap_or_default(),
            company_default_finance_book: filters
                .include_default_book_entries
                .then(|| filters.default_finance_book.clone())
                .flatten(),
            dimensions: dimensions.to_vec(),
        }
    }
}

pub fn execute(
    filters: &DimensionWiseFilters,
    meta: &DimensionMeta,
    dimension_records: &[DimensionRecord],
    accounts: &[DimensionWiseAccount],
    gl_entries: &[DimensionWiseGlEntry],
    company_currency: &str,
) -> DimensionWiseReport {
    let dimension_list = get_dimensions(filters, meta, dimension_records);

    if dimension_list.is_empty() {
        return DimensionWiseReport {
            columns: Vec::new(),
            rows: Some(Vec::new()),
        };
    }

    DimensionWiseReport {
        columns: get_columns(&dimension_list),
        rows: get_data(
            filters,
            &dimension_list,
            accounts,
            gl_entries,
            company_currency,
        ),
    }
}

pub fn get_data(
    filters: &DimensionWiseFilters,
    dimension_list: &[String],
    accounts: &[DimensionWiseAccount],
    gl_entries: &[DimensionWiseGlEntry],
    company_currency: &str,
) -> Option<Vec<DimensionWiseRow>> {
    if accounts.is_empty() {
        return None;
    }

    let mut accounts = filter_accounts(accounts);
    let account_names: BTreeSet<String> = accounts
        .iter()
        .map(|account| account.name.clone())
        .collect();
    let gl_entries = filter_gl_entries(gl_entries, filters, dimension_list, &account_names);

    format_gl_entries(
        &mut accounts,
        &gl_entries,
        dimension_list,
        &filters.dimension,
    );
    accumulate_values_into_parents(&mut accounts, dimension_list);

    Some(prepare_data(
        &accounts,
        filters,
        company_currency,
        dimension_list,
    ))
}

pub fn get_dimensions(
    filters: &DimensionWiseFilters,
    meta: &DimensionMeta,
    records: &[DimensionRecord],
) -> Vec<String> {
    records
        .iter()
        .filter(|record| !meta.has_company || record.company.as_ref() == Some(&filters.company))
        .map(|record| record.name.clone())
        .collect()
}

pub fn get_columns(dimension_list: &[String]) -> Vec<ReportColumn> {
    let mut columns = vec![
        ReportColumn::link("Account", "account", "Account", 300),
        ReportColumn::hidden_link("Currency", "currency", "Currency"),
    ];

    for dimension in dimension_list {
        columns.push(ReportColumn::currency(dimension, &scrub(dimension), 150));
    }

    columns.push(ReportColumn::currency("Total", "total", 150));
    columns
}

pub fn get_condition(dimension: &str) -> String {
    format!(" and {} in %(dimensions)s", scrub(dimension))
}

pub fn format_gl_entries(
    accounts: &mut [DimensionWiseAccount],
    entries: &[DimensionWiseGlEntry],
    dimension_list: &[String],
    dimension_type: &str,
) {
    let dimension_field = scrub(dimension_type);

    for entry in entries {
        if entry.is_cancelled || !dimension_list.contains(&entry.dimension_value) {
            continue;
        }

        if let Some(account) = accounts
            .iter_mut()
            .find(|account| account.name == entry.account)
        {
            let current = account
                .values
                .entry(dimension_field_value(
                    &entry.dimension_value,
                    &dimension_field,
                ))
                .or_insert(0.0);
            *current += entry.debit - entry.credit;
        }
    }
}

pub fn accumulate_values_into_parents(
    accounts: &mut [DimensionWiseAccount],
    dimension_list: &[String],
) {
    for index in (0..accounts.len()).rev() {
        let Some(parent_name) = accounts[index].parent_account.clone() else {
            continue;
        };

        let values: Vec<(String, f64)> = dimension_list
            .iter()
            .map(|dimension| {
                let fieldname = scrub(dimension);
                (
                    fieldname.clone(),
                    *accounts[index].values.get(&fieldname).unwrap_or(&0.0),
                )
            })
            .collect();

        if let Some(parent) = accounts
            .iter_mut()
            .find(|account| account.name == parent_name)
        {
            for (fieldname, value) in values {
                *parent.values.entry(fieldname).or_insert(0.0) += value;
            }
        }
    }
}

pub fn prepare_data(
    accounts: &[DimensionWiseAccount],
    filters: &DimensionWiseFilters,
    company_currency: &str,
    dimension_list: &[String],
) -> Vec<DimensionWiseRow> {
    let mut rows = Vec::new();

    for account in accounts {
        let mut values = BTreeMap::new();
        let mut has_value = false;
        let mut total = 0.0;

        for dimension in dimension_list {
            let fieldname = scrub(dimension);
            let value = flt(*account.values.get(&fieldname).unwrap_or(&0.0), 3);

            if value.abs() >= zero_cutoff(company_currency) {
                has_value = true;
                total += value;
            }

            values.insert(fieldname, value);
        }

        rows.push(DimensionWiseRow {
            account: account.name.clone(),
            is_group: account.is_group,
            parent_account: account.parent_account.clone(),
            indent: account.indent,
            from_date: filters.from_date.clone(),
            to_date: filters.to_date.clone(),
            currency: company_currency.to_string(),
            account_name: match account
                .account_number
                .as_ref()
                .filter(|value| !value.is_empty())
            {
                Some(account_number) => format!("{account_number} - {}", account.account_name),
                None => account.account_name.clone(),
            },
            values,
            has_value,
            total,
        });
    }

    filter_out_zero_value_rows(rows)
}

fn filter_gl_entries(
    entries: &[DimensionWiseGlEntry],
    filters: &DimensionWiseFilters,
    dimension_list: &[String],
    account_names: &BTreeSet<String>,
) -> Vec<DimensionWiseGlEntry> {
    entries
        .iter()
        .filter(|entry| !entry.is_cancelled)
        .filter(|entry| entry.company == filters.company)
        .filter(|entry| account_names.contains(&entry.account))
        .filter(|entry| dimension_list.contains(&entry.dimension_value))
        .filter(|entry| entry.posting_date >= filters.from_date)
        .filter(|entry| entry.posting_date <= filters.to_date)
        .cloned()
        .collect()
}

fn filter_accounts(accounts: &[DimensionWiseAccount]) -> Vec<DimensionWiseAccount> {
    let mut children: BTreeMap<Option<String>, Vec<DimensionWiseAccount>> = BTreeMap::new();
    for account in accounts {
        children
            .entry(account.parent_account.clone())
            .or_default()
            .push(account.clone());
    }

    for (parent, accounts) in children.iter_mut() {
        sort_accounts(accounts, parent.is_none());
    }

    let mut output = Vec::new();
    add_accounts(None, 0, &children, &mut output);
    output
}

fn add_accounts(
    parent: Option<String>,
    indent: usize,
    children: &BTreeMap<Option<String>, Vec<DimensionWiseAccount>>,
    output: &mut Vec<DimensionWiseAccount>,
) {
    if let Some(child_accounts) = children.get(&parent) {
        for child in child_accounts {
            let mut child = child.clone();
            child.indent = indent;
            output.push(child.clone());
            add_accounts(Some(child.name), indent + 1, children, output);
        }
    }
}

fn sort_accounts(accounts: &mut [DimensionWiseAccount], is_root: bool) {
    accounts.sort_by(|a, b| {
        if starts_numbered(&a.name) {
            a.name.cmp(&b.name)
        } else if is_root {
            root_sort_key(a)
                .cmp(&root_sort_key(b))
                .then_with(|| a.name.cmp(&b.name))
        } else {
            a.name.cmp(&b.name)
        }
    });
}

fn root_sort_key(account: &DimensionWiseAccount) -> i32 {
    match (account.report_type.as_str(), account.root_type.as_str()) {
        ("Balance Sheet", "Asset") => 0,
        ("Balance Sheet", "Liability") => 1,
        ("Balance Sheet", "Equity") => 2,
        ("Profit and Loss", "Income") => 3,
        ("Profit and Loss", "Expense") => 4,
        _ => 5,
    }
}

fn starts_numbered(value: &str) -> bool {
    value
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .next()
        .is_some_and(|first| first.chars().all(|ch| ch.is_ascii_digit()))
}

fn filter_out_zero_value_rows(rows: Vec<DimensionWiseRow>) -> Vec<DimensionWiseRow> {
    let mut parent_children: BTreeMap<Option<String>, Vec<String>> = BTreeMap::new();
    for row in &rows {
        parent_children
            .entry(row.parent_account.clone())
            .or_default()
            .push(row.account.clone());
    }

    let mut accounts_to_show = BTreeSet::new();
    for row in &rows {
        if row.has_value {
            accounts_to_show.insert(row.account.clone());
            add_all_parents(&row.account, &parent_children, &mut accounts_to_show);
        }
    }

    rows.into_iter()
        .filter(|row| accounts_to_show.contains(&row.account))
        .collect()
}

fn add_all_parents(
    account: &str,
    parent_children: &BTreeMap<Option<String>, Vec<String>>,
    accounts_to_show: &mut BTreeSet<String>,
) {
    for (parent, children) in parent_children {
        if children.iter().any(|child| child == account) {
            if let Some(parent) = parent {
                accounts_to_show.insert(parent.clone());
                add_all_parents(parent, parent_children, accounts_to_show);
            }
        }
    }
}

fn dimension_field_value(dimension: &str, dimension_field: &str) -> String {
    let scrubbed = scrub(dimension);
    if scrubbed.is_empty() {
        dimension_field.to_string()
    } else {
        scrubbed
    }
}

fn scrub(value: &str) -> String {
    let mut output = String::new();
    let mut last_was_underscore = false;

    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_was_underscore = false;
        } else if !last_was_underscore {
            output.push('_');
            last_was_underscore = true;
        }
    }

    output.trim_matches('_').to_string()
}

fn flt(value: f64, precision: i32) -> f64 {
    let factor = 10_f64.powi(precision);
    (value * factor).round() / factor
}

fn zero_cutoff(currency: &str) -> f64 {
    match currency {
        "BHD" => 0.0005,
        _ => 0.005,
    }
}
