#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsolidatedTrialBalanceFilters {
    pub company: Vec<String>,
    pub from_date: String,
    pub to_date: String,
    pub presentation_currency: Option<String>,
    pub show_net_values: bool,
    pub show_group_accounts: bool,
    pub show_zero_values: bool,
    pub report_template: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidatedTrialBalanceReport {
    pub columns: Vec<ReportColumn>,
    pub data: Vec<ConsolidatedTrialBalanceRow>,
    pub delegated_to_financial_report_engine: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: &'static str,
    pub label: &'static str,
    pub fieldtype: &'static str,
    pub width: u16,
    pub hidden: bool,
    pub options: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompanyNode {
    pub name: String,
    pub parent_company: Option<String>,
    pub lft: i32,
    pub rgt: i32,
    pub default_currency: String,
    pub reporting_currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidatedTrialBalanceRow {
    pub company: Option<String>,
    pub account: String,
    pub account_name: String,
    pub acc_name: String,
    pub acc_number: Option<String>,
    pub parent_account: Option<String>,
    pub indent: i32,
    pub from_date: String,
    pub to_date: String,
    pub currency: String,
    pub is_group_account: bool,
    pub root_type: String,
    pub account_type: String,
    pub opening_debit: f64,
    pub opening_credit: f64,
    pub debit: f64,
    pub credit: f64,
    pub closing_debit: f64,
    pub closing_credit: f64,
    pub has_value: bool,
    pub warn_if_negative: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurrencyRate {
    pub from_currency: String,
    pub to_currency: String,
    pub date: String,
    pub rate: f64,
}

pub const VALUE_FIELDS: [&str; 6] = [
    "opening_debit",
    "opening_credit",
    "debit",
    "credit",
    "closing_debit",
    "closing_credit",
];

impl ReportColumn {
    pub const fn data(
        label: &'static str,
        fieldname: &'static str,
        width: u16,
        hidden: bool,
        options: Option<&'static str>,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Data",
            width,
            hidden,
            options,
        }
    }

    pub const fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
        hidden: bool,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Link",
            width,
            hidden,
            options: Some(options),
        }
    }

    pub const fn currency(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Currency",
            width,
            hidden: false,
            options: Some("currency"),
        }
    }
}

impl CompanyNode {
    pub fn new(
        name: &str,
        parent_company: Option<&str>,
        lft: i32,
        rgt: i32,
        default_currency: &str,
        reporting_currency: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            parent_company: parent_company.map(str::to_string),
            lft,
            rgt,
            default_currency: default_currency.to_string(),
            reporting_currency: reporting_currency.map(str::to_string),
        }
    }
}

impl CurrencyRate {
    pub fn new(from_currency: &str, to_currency: &str, date: &str, rate: f64) -> Self {
        Self {
            from_currency: from_currency.to_string(),
            to_currency: to_currency.to_string(),
            date: date.to_string(),
            rate,
        }
    }
}

pub fn execute(
    filters: &ConsolidatedTrialBalanceFilters,
    company_tree: Vec<CompanyNode>,
    company_rows: Vec<ConsolidatedTrialBalanceRow>,
    rates: Vec<CurrencyRate>,
) -> Result<ConsolidatedTrialBalanceReport, String> {
    if filters.report_template {
        return Ok(ConsolidatedTrialBalanceReport {
            columns: Vec::new(),
            data: Vec::new(),
            delegated_to_financial_report_engine: true,
        });
    }

    let sorted_companies = validate_companies(&filters.company, &company_tree)?;
    let (reporting_currency, ignore_reporting_currency) =
        get_reporting_currency(&sorted_companies, &company_tree)
            .ok_or_else(|| "Reporting currency could not be determined.".to_string())?;
    let mut data = Vec::new();

    for company in sorted_companies {
        let tb_data = company_rows
            .iter()
            .filter(|row| row_belongs_to_company(row, &company))
            .cloned()
            .collect::<Vec<_>>();
        consolidate_trial_balance_data(&mut data, &tb_data);
    }

    if filters.show_net_values {
        prepare_opening_closing_for_ctb(&mut data);
    }

    if !filters.show_group_accounts {
        data.retain(|row| !row.is_group_account);
    }

    let mut total_row = calculate_total_row(&data, &reporting_currency);
    calculate_foreign_currency_translation_reserve(&mut total_row, &mut data, filters);
    data.push(total_row);

    if let Some(to_currency) = filters.presentation_currency.as_deref() {
        update_to_presentation_currency(
            &mut data,
            &reporting_currency,
            to_currency,
            &filters.to_date,
            ignore_reporting_currency,
            &rates,
        );
    }

    Ok(ConsolidatedTrialBalanceReport {
        columns: get_columns(),
        data,
        delegated_to_financial_report_engine: false,
    })
}

pub fn validate_companies(
    companies: &[String],
    company_tree: &[CompanyNode],
) -> Result<Vec<String>, String> {
    if companies.is_empty() {
        return Ok(Vec::new());
    }

    let first = find_company(company_tree, &companies[0])
        .ok_or_else(|| format!("Company {} not found.", companies[0]))?;
    let root = root_company(first, company_tree);
    let mut subtree = company_tree
        .iter()
        .filter(|company| company.lft >= root.lft && company.rgt <= root.rgt)
        .collect::<Vec<_>>();
    subtree.sort_by_key(|company| company.lft);

    for company in companies {
        if !subtree.iter().any(|node| node.name == *company) {
            return Err(
                "Consolidated Trial Balance can be generated for Companies having same root Company."
                    .to_string(),
            );
        }
    }

    Ok(subtree
        .into_iter()
        .filter(|company| companies.contains(&company.name))
        .map(|company| company.name.clone())
        .collect())
}

pub fn get_reporting_currency(
    companies: &[String],
    company_tree: &[CompanyNode],
) -> Option<(String, bool)> {
    let first = find_company(company_tree, companies.first()?)?;
    let reporting_currency = first.reporting_currency.clone()?;
    let mut default_currency: Option<String> = None;

    for company in companies {
        let company_default_currency = find_company(company_tree, company)?
            .default_currency
            .clone();
        if default_currency.is_none() {
            default_currency = Some(company_default_currency.clone());
        }

        if Some(&company_default_currency) != default_currency.as_ref() {
            return Some((reporting_currency, false));
        }
    }

    default_currency.map(|currency| (currency, true))
}

pub fn prepare_companywise_tb_data(
    accounts: &[ConsolidatedTrialBalanceRow],
    from_date: &str,
    to_date: &str,
    reporting_currency: &str,
    zero_cutoff: f64,
) -> Vec<ConsolidatedTrialBalanceRow> {
    accounts
        .iter()
        .cloned()
        .map(|mut row| {
            row.from_date = from_date.to_string();
            row.to_date = to_date.to_string();
            row.currency = reporting_currency.to_string();
            row.account_name = row
                .acc_number
                .as_deref()
                .map(|number| format!("{number} - {}", row.acc_name))
                .unwrap_or_else(|| row.acc_name.clone());

            for field in VALUE_FIELDS {
                let amount = round_to(value(&row, field), 3);
                set_value(&mut row, field, amount);
            }

            row.has_value = VALUE_FIELDS
                .iter()
                .any(|field| value(&row, field).abs() >= zero_cutoff);
            row
        })
        .collect()
}

pub fn calculate_foreign_currency_translation_reserve(
    total_row: &mut ConsolidatedTrialBalanceRow,
    data: &mut Vec<ConsolidatedTrialBalanceRow>,
    filters: &ConsolidatedTrialBalanceFilters,
) {
    if data.is_empty() {
        return;
    }

    let opening_dr_cr_diff = total_row.opening_debit - total_row.opening_credit;
    let dr_cr_diff = total_row.debit - total_row.credit;
    let idx = get_fctr_root_row_index(data);
    let Some(parent) = data.get(idx).cloned() else {
        return;
    };
    let mut fctr_row = ConsolidatedTrialBalanceRow {
        company: None,
        account: "Foreign Currency Translation Reserve".to_string(),
        account_name: "Foreign Currency Translation Reserve".to_string(),
        acc_name: "Foreign Currency Translation Reserve".to_string(),
        acc_number: None,
        parent_account: Some(parent.account.clone()),
        indent: if filters.show_group_accounts {
            parent.indent + 1
        } else {
            0
        },
        from_date: total_row.from_date.clone(),
        to_date: total_row.to_date.clone(),
        currency: total_row.currency.clone(),
        is_group_account: false,
        root_type: parent.root_type.clone(),
        account_type: "Equity".to_string(),
        opening_debit: if opening_dr_cr_diff < 0.0 {
            opening_dr_cr_diff.abs()
        } else {
            0.0
        },
        opening_credit: if opening_dr_cr_diff > 0.0 {
            opening_dr_cr_diff.abs()
        } else {
            0.0
        },
        debit: if dr_cr_diff < 0.0 {
            dr_cr_diff.abs()
        } else {
            0.0
        },
        credit: if dr_cr_diff > 0.0 {
            dr_cr_diff.abs()
        } else {
            0.0
        },
        closing_debit: 0.0,
        closing_credit: 0.0,
        has_value: true,
        warn_if_negative: true,
    };
    fctr_row.closing_debit = fctr_row.opening_debit + fctr_row.debit;
    fctr_row.closing_credit = fctr_row.opening_credit + fctr_row.credit;

    if filters.show_net_values {
        prepare_opening_closing(&mut fctr_row);
    }

    data.insert(idx + 1, fctr_row.clone());

    for field in VALUE_FIELDS {
        set_value(
            total_row,
            field,
            value(total_row, field) + value(&fctr_row, field),
        );
    }
}

pub fn get_fctr_root_row_index(data: &[ConsolidatedTrialBalanceRow]) -> usize {
    let mut liabilities_idx = None;
    let mut equity_idx = None;

    for (index, row) in data.iter().enumerate() {
        if liabilities_idx.is_none() && row.root_type == "Liability" {
            liabilities_idx = Some(index);
        }
        if equity_idx.is_none() && row.root_type == "Equity" {
            equity_idx = Some(index);
        }
    }

    equity_idx.or(liabilities_idx).unwrap_or(0)
}

pub fn consolidate_trial_balance_data(
    data: &mut Vec<ConsolidatedTrialBalanceRow>,
    tb_data: &[ConsolidatedTrialBalanceRow],
) {
    if data.is_empty() {
        data.extend(tb_data.iter().cloned());
        return;
    }

    for entry in tb_data.iter().cloned() {
        consolidate_gle_data(data, entry, tb_data);
    }
}

pub fn update_to_presentation_currency(
    data: &mut [ConsolidatedTrialBalanceRow],
    from_currency: &str,
    to_currency: &str,
    date: &str,
    ignore_reporting_currency: bool,
    rates: &[CurrencyRate],
) {
    if from_currency == to_currency {
        return;
    }

    let exchange_rate = rates
        .iter()
        .find(|rate| {
            rate.from_currency == from_currency
                && rate.to_currency == to_currency
                && rate.date == date
        })
        .map_or(1.0, |rate| rate.rate);

    for row in data {
        if !ignore_reporting_currency {
            for field in VALUE_FIELDS {
                let current = value(row, field);
                if current != 0.0 {
                    set_value(row, field, current * exchange_rate);
                }
            }
        }
        row.currency = to_currency.to_string();
    }
}

pub fn prepare_opening_closing_for_ctb(data: &mut [ConsolidatedTrialBalanceRow]) {
    for row in data {
        prepare_opening_closing(row);
    }
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::data("Account", "account_name", 300, false, None),
        ReportColumn::data("Account Name", "acc_name", 250, true, None),
        ReportColumn::data("Account Number", "acc_number", 120, true, None),
        ReportColumn::link("Currency", "currency", "Currency", 0, true),
        ReportColumn::currency("Opening (Dr)", "opening_debit", 120),
        ReportColumn::currency("Opening (Cr)", "opening_credit", 120),
        ReportColumn::currency("Debit", "debit", 120),
        ReportColumn::currency("Credit", "credit", 120),
        ReportColumn::currency("Closing (Dr)", "closing_debit", 120),
        ReportColumn::currency("Closing (Cr)", "closing_credit", 120),
    ]
}

fn consolidate_gle_data(
    data: &mut Vec<ConsolidatedTrialBalanceRow>,
    mut entry: ConsolidatedTrialBalanceRow,
    tb_data: &[ConsolidatedTrialBalanceRow],
) {
    if let Some(existing) = data
        .iter_mut()
        .find(|row| row.account_name == entry.account_name)
    {
        existing.closing_credit += entry.closing_credit;
        existing.closing_debit += entry.closing_debit;
        existing.credit += entry.credit;
        existing.debit += entry.debit;
        existing.opening_credit += entry.opening_credit;
        existing.opening_debit += entry.opening_debit;
        existing.has_value = true;
        return;
    }

    let entry_parent_account = entry
        .parent_account
        .as_deref()
        .and_then(|parent| tb_data.iter().find(|row| row.account == parent));
    let parent_index = entry_parent_account.and_then(|parent| {
        data.iter()
            .position(|row| row.account_name == parent.account_name)
    });

    if let Some(index) = parent_index {
        entry.parent_account = Some(data[index].account.clone());
        entry.indent = data[index].indent + 1;
        data.insert(index + 1, entry);
    } else {
        entry.parent_account = None;
        entry.indent = 0;
        data.push(entry);
    }
}

fn calculate_total_row(
    data: &[ConsolidatedTrialBalanceRow],
    currency: &str,
) -> ConsolidatedTrialBalanceRow {
    let mut total = ConsolidatedTrialBalanceRow {
        company: None,
        account: "Total".to_string(),
        account_name: "Total".to_string(),
        acc_name: "Total".to_string(),
        acc_number: None,
        parent_account: None,
        indent: 0,
        from_date: String::new(),
        to_date: String::new(),
        currency: currency.to_string(),
        is_group_account: false,
        root_type: String::new(),
        account_type: String::new(),
        opening_debit: 0.0,
        opening_credit: 0.0,
        debit: 0.0,
        credit: 0.0,
        closing_debit: 0.0,
        closing_credit: 0.0,
        has_value: true,
        warn_if_negative: false,
    };

    for row in data.iter().filter(|row| !row.is_group_account) {
        for field in VALUE_FIELDS {
            let amount = value(&total, field) + value(row, field);
            set_value(&mut total, field, amount);
        }
    }

    total
}

fn prepare_opening_closing(row: &mut ConsolidatedTrialBalanceRow) {
    let opening = row.opening_debit - row.opening_credit;
    if opening >= 0.0 {
        row.opening_debit = opening;
        row.opening_credit = 0.0;
    } else {
        row.opening_debit = 0.0;
        row.opening_credit = opening.abs();
    }

    let closing = row.closing_debit - row.closing_credit;
    if closing >= 0.0 {
        row.closing_debit = closing;
        row.closing_credit = 0.0;
    } else {
        row.closing_debit = 0.0;
        row.closing_credit = closing.abs();
    }
}

fn find_company<'a>(company_tree: &'a [CompanyNode], name: &str) -> Option<&'a CompanyNode> {
    company_tree.iter().find(|company| company.name == name)
}

fn root_company<'a>(company: &'a CompanyNode, company_tree: &'a [CompanyNode]) -> &'a CompanyNode {
    let mut current = company;
    while let Some(parent) = current
        .parent_company
        .as_deref()
        .and_then(|parent| find_company(company_tree, parent))
    {
        current = parent;
    }
    current
}

fn company_suffix(company: &str) -> String {
    company
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .collect::<String>()
}

fn row_belongs_to_company(row: &ConsolidatedTrialBalanceRow, company: &str) -> bool {
    row.company.as_deref() == Some(company)
        || (row.company.is_none() && row.account.ends_with(company_suffix(company).as_str()))
}

fn value(row: &ConsolidatedTrialBalanceRow, field: &str) -> f64 {
    match field {
        "opening_debit" => row.opening_debit,
        "opening_credit" => row.opening_credit,
        "debit" => row.debit,
        "credit" => row.credit,
        "closing_debit" => row.closing_debit,
        "closing_credit" => row.closing_credit,
        _ => 0.0,
    }
}

fn set_value(row: &mut ConsolidatedTrialBalanceRow, field: &str, amount: f64) {
    match field {
        "opening_debit" => row.opening_debit = amount,
        "opening_credit" => row.opening_credit = amount,
        "debit" => row.debit = amount,
        "credit" => row.credit = amount,
        "closing_debit" => row.closing_debit = amount,
        "closing_credit" => row.closing_credit = amount,
        _ => {}
    }
}

fn round_to(value: f64, precision: i32) -> f64 {
    let multiplier = 10_f64.powi(precision);
    (value * multiplier).round() / multiplier
}
