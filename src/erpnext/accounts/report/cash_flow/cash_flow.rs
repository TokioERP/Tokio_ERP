use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashFlowFilters {
    pub company: String,
    pub company_currency: String,
    pub accumulated_values: bool,
    pub accumulated_in_group_company: bool,
    pub show_opening_and_closing_balance: bool,
    pub report_template: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashFlowPeriod {
    pub key: String,
    pub label: String,
    pub from_date: String,
    pub to_date: String,
    pub year_start_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashFlowSection {
    pub section_name: &'static str,
    pub section_footer: &'static str,
    pub section_header: &'static str,
    pub account_types: Vec<CashFlowAccountType>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashFlowAccountType {
    pub account_type: &'static str,
    pub label: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashFlowAccountAmount {
    pub account_type: String,
    pub period_key: String,
    pub amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashFlowRow {
    pub section_name: String,
    pub parent_section: Option<String>,
    pub indent: f64,
    pub section: String,
    pub accounts: Vec<String>,
    pub currency: String,
    pub values: BTreeMap<String, f64>,
    pub total: f64,
    pub is_empty: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashFlowReportSummary {
    pub value: f64,
    pub label: String,
    pub datatype: String,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashFlowChartDataset {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashFlowChart {
    pub labels: Vec<String>,
    pub datasets: Vec<CashFlowChartDataset>,
    pub chart_type: String,
    pub fieldtype: String,
    pub options: String,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashFlowReport {
    pub columns: Vec<String>,
    pub rows: Vec<CashFlowRow>,
    pub chart: CashFlowChart,
    pub report_summary: Vec<CashFlowReportSummary>,
    pub delegated_to_financial_report_engine: bool,
}

impl CashFlowPeriod {
    pub fn new(
        key: &str,
        label: &str,
        from_date: &str,
        to_date: &str,
        year_start_date: &str,
    ) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
            year_start_date: year_start_date.to_string(),
        }
    }
}

impl CashFlowAccountAmount {
    pub fn new(account_type: &str, period_key: &str, amount: f64) -> Self {
        Self {
            account_type: account_type.to_string(),
            period_key: period_key.to_string(),
            amount,
        }
    }
}

impl CashFlowRow {
    pub fn empty() -> Self {
        Self {
            section_name: String::new(),
            parent_section: None,
            indent: 0.0,
            section: String::new(),
            accounts: Vec::new(),
            currency: String::new(),
            values: BTreeMap::new(),
            total: 0.0,
            is_empty: true,
        }
    }

    pub fn section(section_header: &str) -> Self {
        Self {
            section_name: quoted(section_header),
            parent_section: None,
            indent: 0.0,
            section: section_header.to_string(),
            accounts: Vec::new(),
            currency: String::new(),
            values: BTreeMap::new(),
            total: 0.0,
            is_empty: false,
        }
    }

    pub fn amounts(section: &str, values: &[(&str, f64)]) -> Self {
        let values = values
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect::<BTreeMap<_, _>>();
        Self {
            section_name: section.to_string(),
            parent_section: None,
            indent: 0.0,
            section: section.to_string(),
            accounts: Vec::new(),
            currency: String::new(),
            total: values.values().sum(),
            values,
            is_empty: false,
        }
    }

    pub fn child(parent_section: &str, section: &str, values: &[(&str, f64)]) -> Self {
        Self {
            parent_section: Some(parent_section.to_string()),
            indent: 1.0,
            ..Self::amounts(section, values)
        }
    }

    pub fn section_total(section: &str, values: &[(&str, f64)], currency: &str) -> Self {
        Self {
            currency: currency.to_string(),
            ..Self::amounts(section, values)
        }
    }

    pub fn value(&self, key: &str) -> f64 {
        *self.values.get(key).unwrap_or(&0.0)
    }
}

impl CashFlowReportSummary {
    pub fn currency(label: &str, value: f64, currency: &str) -> Self {
        Self {
            value,
            label: label.to_string(),
            datatype: "Currency".to_string(),
            currency: currency.to_string(),
        }
    }
}

impl CashFlowChartDataset {
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn execute(
    filters: &CashFlowFilters,
    columns: Vec<String>,
    period_list: Vec<CashFlowPeriod>,
    net_profit_loss: Option<CashFlowRow>,
    account_amounts: &[CashFlowAccountAmount],
    opening_amount: f64,
    fiscal_year_start: &str,
) -> CashFlowReport {
    if filters.report_template {
        return CashFlowReport {
            columns: Vec::new(),
            rows: Vec::new(),
            chart: CashFlowChart {
                labels: Vec::new(),
                datasets: Vec::new(),
                chart_type: String::new(),
                fieldtype: String::new(),
                options: String::new(),
                currency: String::new(),
            },
            report_summary: Vec::new(),
            delegated_to_financial_report_engine: true,
        };
    }

    let cash_flow_sections = get_cash_flow_accounts();
    let company_currency = filters.company_currency.as_str();
    let mut data = Vec::new();
    let mut summary_data = BTreeMap::new();

    for (section_index, cash_flow_section) in cash_flow_sections.iter().enumerate() {
        let mut section_data = Vec::new();
        data.push(CashFlowRow::section(cash_flow_section.section_header));

        if section_index == 0 {
            if let Some(mut row) = net_profit_loss.clone() {
                row.indent = 1.0;
                row.parent_section = Some(cash_flow_section.section_header.to_string());
                row.section = row.section.clone();
                data.push(row.clone());
                section_data.push(row);
            }
        }

        for account in &cash_flow_section.account_types {
            let account_data = get_account_type_based_data(
                account.account_type,
                &period_list,
                filters.accumulated_values,
                fiscal_year_start,
                account_amounts,
            );
            let mut row = account_data;
            row.section_name = account.label.to_string();
            row.section = account.label.to_string();
            row.indent = 1.0;
            row.parent_section = Some(cash_flow_section.section_header.to_string());
            row.currency = company_currency.to_string();
            data.push(row.clone());
            section_data.push(row);
        }

        add_total_row_account(
            &mut data,
            &section_data,
            cash_flow_section.section_footer,
            &period_list,
            company_currency,
            &mut summary_data,
            filters,
            false,
        );
    }

    let net_change_in_cash = {
        let snapshot = data.clone();
        add_total_row_account(
            &mut data,
            &snapshot,
            "Net Change in Cash",
            &period_list,
            company_currency,
            &mut summary_data,
            filters,
            false,
        )
    };

    if filters.show_opening_and_closing_balance {
        show_opening_and_closing_balance(
            &mut data,
            &period_list,
            company_currency,
            &net_change_in_cash,
            opening_amount,
        );
    }

    let chart = get_chart_data(&period_list, &data, company_currency);
    let report_summary = get_report_summary(&summary_data, company_currency);

    CashFlowReport {
        columns,
        rows: data,
        chart,
        report_summary,
        delegated_to_financial_report_engine: false,
    }
}

pub fn get_cash_flow_accounts() -> Vec<CashFlowSection> {
    vec![
        CashFlowSection {
            section_name: "Operations",
            section_footer: "Net Cash from Operations",
            section_header: "Cash Flow from Operations",
            account_types: vec![
                CashFlowAccountType {
                    account_type: "Depreciation",
                    label: "Depreciation",
                },
                CashFlowAccountType {
                    account_type: "Receivable",
                    label: "Net Change in Accounts Receivable",
                },
                CashFlowAccountType {
                    account_type: "Payable",
                    label: "Net Change in Accounts Payable",
                },
                CashFlowAccountType {
                    account_type: "Stock",
                    label: "Net Change in Inventory",
                },
            ],
        },
        CashFlowSection {
            section_name: "Investing",
            section_footer: "Net Cash from Investing",
            section_header: "Cash Flow from Investing",
            account_types: vec![CashFlowAccountType {
                account_type: "Fixed Asset",
                label: "Net Change in Fixed Asset",
            }],
        },
        CashFlowSection {
            section_name: "Financing",
            section_footer: "Net Cash from Financing",
            section_header: "Cash Flow from Financing",
            account_types: vec![CashFlowAccountType {
                account_type: "Equity",
                label: "Net Change in Equity",
            }],
        },
    ]
}

pub fn get_account_type_based_data(
    account_type: &str,
    period_list: &[CashFlowPeriod],
    accumulated_values: bool,
    fiscal_year_start: &str,
    account_amounts: &[CashFlowAccountAmount],
) -> CashFlowRow {
    let mut row = CashFlowRow::empty();
    row.is_empty = false;
    let mut total = 0.0;

    for period in period_list {
        let _start_date = get_start_date(period, accumulated_values, fiscal_year_start);
        let mut amount = account_amounts
            .iter()
            .find(|amount| amount.account_type == account_type && amount.period_key == period.key)
            .map_or(0.0, |amount| amount.amount);

        if amount != 0.0 && account_type == "Depreciation" {
            amount *= -1.0;
        }

        total += amount;
        row.values.entry(period.key.clone()).or_insert(amount);
    }

    row.total = total;
    row
}

pub fn get_start_date(
    period: &CashFlowPeriod,
    accumulated_values: bool,
    fiscal_year_start: &str,
) -> String {
    if !accumulated_values && !period.from_date.is_empty() {
        return period.from_date.clone();
    }

    if accumulated_values {
        return fiscal_year_start.to_string();
    }

    period.year_start_date.clone()
}

#[allow(clippy::too_many_arguments)]
pub fn add_total_row_account(
    out: &mut Vec<CashFlowRow>,
    data: &[CashFlowRow],
    label: &str,
    period_list: &[CashFlowPeriod],
    currency: &str,
    summary_data: &mut BTreeMap<String, f64>,
    filters: &CashFlowFilters,
    consolidated: bool,
) -> CashFlowRow {
    let mut total_row = CashFlowRow {
        section_name: quoted(label),
        section: quoted(label),
        currency: currency.to_string(),
        is_empty: false,
        ..CashFlowRow::empty()
    };
    summary_data.insert(label.to_string(), 0.0);
    let mut selected_periods = period_list.iter().collect::<Vec<_>>();

    if filters.accumulated_in_group_company {
        selected_periods
            .retain(|period| period.key == filters.company || period.label == filters.company);
    }

    for row in data {
        if row.parent_section.is_some() {
            for period in &selected_periods {
                let key = if consolidated {
                    period.label.as_str()
                } else {
                    period.key.as_str()
                };
                let value = row.value(key);
                *total_row.values.entry(key.to_string()).or_insert(0.0) += value;
                *summary_data.entry(label.to_string()).or_insert(0.0) += value;
            }

            total_row.total += row.total;
        }
    }

    out.push(total_row.clone());
    out.push(CashFlowRow::empty());

    total_row
}

pub fn show_opening_and_closing_balance(
    out: &mut Vec<CashFlowRow>,
    period_list: &[CashFlowPeriod],
    currency: &str,
    net_change_in_cash: &CashFlowRow,
    opening_amount: f64,
) {
    let mut opening_balance = CashFlowRow {
        section_name: "Opening".to_string(),
        section: "Opening".to_string(),
        currency: currency.to_string(),
        is_empty: false,
        ..CashFlowRow::empty()
    };
    let mut closing_balance = CashFlowRow {
        section_name: "Closing (Opening + Total)".to_string(),
        section: "Closing (Opening + Total)".to_string(),
        currency: currency.to_string(),
        is_empty: false,
        ..CashFlowRow::empty()
    };
    let mut running_total = opening_amount;

    for (index, period) in period_list.iter().enumerate() {
        let change = net_change_in_cash.value(&period.key);
        opening_balance.values.insert(
            period.key.clone(),
            if index == 0 {
                opening_amount
            } else {
                running_total
            },
        );
        running_total += change;
        closing_balance
            .values
            .insert(period.key.clone(), running_total);
    }

    if let Some(first) = period_list.first() {
        opening_balance.total = opening_balance.value(&first.key);
    }
    if let Some(last) = period_list.last() {
        closing_balance.total = closing_balance.value(&last.key);
    }

    out.extend([
        opening_balance,
        net_change_in_cash.clone(),
        closing_balance,
        CashFlowRow::empty(),
    ]);
}

pub fn get_report_summary(
    summary_data: &BTreeMap<String, f64>,
    currency: &str,
) -> Vec<CashFlowReportSummary> {
    let ordered_labels = [
        "Net Cash from Operations",
        "Net Cash from Investing",
        "Net Cash from Financing",
        "Net Change in Cash",
    ];
    let mut report_summary = ordered_labels
        .iter()
        .filter_map(|label| {
            summary_data
                .get(*label)
                .map(|value| CashFlowReportSummary::currency(label, *value, currency))
        })
        .collect::<Vec<_>>();

    for (label, value) in summary_data {
        if !ordered_labels.contains(&label.as_str()) {
            report_summary.push(CashFlowReportSummary::currency(label, *value, currency));
        }
    }

    report_summary
}

pub fn get_chart_data(
    period_list: &[CashFlowPeriod],
    data: &[CashFlowRow],
    currency: &str,
) -> CashFlowChart {
    let labels = period_list
        .iter()
        .map(|period| period.label.clone())
        .collect::<Vec<_>>();
    let mut datasets = data
        .iter()
        .filter(|row| row.parent_section.is_none() && !row.currency.is_empty())
        .map(|row| {
            CashFlowChartDataset::new(
                row.section.replace('\'', "").as_str(),
                period_list
                    .iter()
                    .map(|period| row.value(&period.key))
                    .collect(),
            )
        })
        .collect::<Vec<_>>();

    let keep = datasets.len().saturating_sub(2);
    datasets.truncate(keep);

    CashFlowChart {
        labels,
        datasets,
        chart_type: "bar".to_string(),
        fieldtype: "Currency".to_string(),
        options: "currency".to_string(),
        currency: currency.to_string(),
    }
}

fn quoted(value: &str) -> String {
    format!("'{value}'")
}
