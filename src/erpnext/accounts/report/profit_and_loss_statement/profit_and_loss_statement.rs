use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfitLossFilters {
    pub company: String,
    pub presentation_currency: Option<String>,
    pub accumulated_values: bool,
    pub periodicity: String,
    pub selected_view: Option<String>,
    pub accumulated_in_group_company: bool,
    pub report_template: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfitLossPeriod {
    pub key: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitLossRow {
    pub account_name: String,
    pub account: String,
    pub warn_if_negative: bool,
    pub currency: String,
    pub values: BTreeMap<String, f64>,
    pub total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitLossReportSummary {
    pub value: f64,
    pub label: String,
    pub datatype: String,
    pub currency: String,
    pub indicator: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitLossChartDataset {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitLossChart {
    pub labels: Vec<String>,
    pub datasets: Vec<ProfitLossChartDataset>,
    pub chart_type: String,
    pub fieldtype: String,
    pub options: String,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitLossReport {
    pub columns: Vec<String>,
    pub rows: Vec<ProfitLossRow>,
    pub chart: Option<ProfitLossChart>,
    pub report_summary: Vec<ProfitLossReportSummary>,
    pub primitive_summary: f64,
    pub delegated_to_financial_report_engine: bool,
}

impl ProfitLossPeriod {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
        }
    }
}

impl ProfitLossRow {
    pub fn account(account: &str, values: &[(&str, f64)]) -> Self {
        Self {
            account_name: account.to_string(),
            account: account.to_string(),
            warn_if_negative: false,
            currency: String::new(),
            values: values
                .iter()
                .map(|(key, value)| ((*key).to_string(), *value))
                .collect(),
            total: values.iter().map(|(_, value)| *value).sum(),
        }
    }

    pub fn profit(currency: &str) -> Self {
        Self {
            account_name: "'Profit for the year'".to_string(),
            account: "'Profit for the year'".to_string(),
            warn_if_negative: true,
            currency: currency.to_string(),
            values: BTreeMap::new(),
            total: 0.0,
        }
    }

    pub fn value(&self, key: &str) -> f64 {
        *self.values.get(key).unwrap_or(&0.0)
    }
}

impl ProfitLossReportSummary {
    pub fn currency(label: &str, value: f64, currency: &str) -> Self {
        Self {
            value,
            label: label.to_string(),
            datatype: "Currency".to_string(),
            currency: currency.to_string(),
            indicator: None,
        }
    }

    pub fn profit(label: &str, value: f64, indicator: &str, currency: &str) -> Self {
        Self {
            indicator: Some(indicator.to_string()),
            ..Self::currency(label, value, currency)
        }
    }
}

impl ProfitLossChartDataset {
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }
}

pub fn execute(
    filters: &ProfitLossFilters,
    columns: Vec<String>,
    period_list: Vec<ProfitLossPeriod>,
    income: Vec<ProfitLossRow>,
    expense: Vec<ProfitLossRow>,
) -> ProfitLossReport {
    if filters.report_template {
        return ProfitLossReport {
            columns: Vec::new(),
            rows: Vec::new(),
            chart: None,
            report_summary: Vec::new(),
            primitive_summary: 0.0,
            delegated_to_financial_report_engine: true,
        };
    }

    let currency = filters
        .presentation_currency
        .as_deref()
        .unwrap_or(&filters.company)
        .to_string();
    let net_profit_loss = get_net_profit_loss(
        &income,
        &expense,
        &period_list,
        &filters.company,
        Some(&currency),
        false,
    );

    let mut rows = Vec::new();
    rows.extend(income.clone());
    rows.extend(expense.clone());
    if let Some(net_profit_loss) = net_profit_loss.clone() {
        rows.push(net_profit_loss);
    }

    let chart = get_chart_data(
        filters,
        &period_list,
        &income,
        &expense,
        net_profit_loss.as_ref(),
        &currency,
    );
    let (report_summary, primitive_summary) = get_report_summary(
        &period_list,
        &filters.periodicity,
        &income,
        &expense,
        net_profit_loss.as_ref(),
        &currency,
        filters,
        false,
    );

    match filters.selected_view.as_deref() {
        Some("Growth") => compute_growth_view_data(&mut rows, &period_list),
        Some("Margin") => {
            compute_margin_view_data(&mut rows, &period_list, filters.accumulated_values)
        }
        _ => {}
    }

    ProfitLossReport {
        columns,
        rows,
        chart: Some(chart),
        report_summary,
        primitive_summary,
        delegated_to_financial_report_engine: false,
    }
}

pub fn get_net_profit_loss(
    income: &[ProfitLossRow],
    expense: &[ProfitLossRow],
    period_list: &[ProfitLossPeriod],
    company: &str,
    currency: Option<&str>,
    consolidated: bool,
) -> Option<ProfitLossRow> {
    let mut total = 0.0;
    let mut net_profit_loss = ProfitLossRow::profit(currency.unwrap_or(company));
    let mut has_value = false;

    for period in period_list {
        let key = if consolidated {
            period.label.as_str()
        } else {
            period.key.as_str()
        };
        let total_income = second_last(income).map_or(0.0, |row| round_to(row.value(key), 3));
        let total_expense = second_last(expense).map_or(0.0, |row| round_to(row.value(key), 3));
        let value = total_income - total_expense;

        net_profit_loss.values.insert(key.to_string(), value);
        if value != 0.0 {
            has_value = true;
        }
        total += value;
        net_profit_loss.total = total;
    }

    has_value.then_some(net_profit_loss)
}

#[allow(clippy::too_many_arguments)]
pub fn get_report_summary(
    period_list: &[ProfitLossPeriod],
    periodicity: &str,
    income: &[ProfitLossRow],
    expense: &[ProfitLossRow],
    net_profit_loss: Option<&ProfitLossRow>,
    currency: &str,
    filters: &ProfitLossFilters,
    consolidated: bool,
) -> (Vec<ProfitLossReportSummary>, f64) {
    let mut net_income = 0.0;
    let mut net_expense = 0.0;
    let mut net_profit = 0.0;

    if filters.accumulated_values {
        if let Some(period) = period_list.last() {
            let key = if consolidated {
                period.label.as_str()
            } else {
                period.key.as_str()
            };
            net_income = second_last(income).map_or(0.0, |row| row.value(key));
            net_expense = second_last(expense).map_or(0.0, |row| row.value(key));
            net_profit = net_profit_loss.map_or(0.0, |row| row.value(key));
        }
    } else {
        for period in period_list {
            let key = if consolidated {
                period.label.as_str()
            } else {
                period.key.as_str()
            };
            net_income += second_last(income).map_or(0.0, |row| row.value(key));
            net_expense += second_last(expense).map_or(0.0, |row| row.value(key));
            net_profit += net_profit_loss.map_or(0.0, |row| row.value(key));
        }
    }

    let (profit_label, income_label, expense_label) =
        if period_list.len() == 1 && periodicity == "Yearly" {
            (
                "Profit This Year",
                "Total Income This Year",
                "Total Expense This Year",
            )
        } else {
            ("Net Profit", "Total Income", "Total Expense")
        };

    (
        vec![
            ProfitLossReportSummary::currency(income_label, net_income, currency),
            ProfitLossReportSummary::currency(expense_label, net_expense, currency),
            ProfitLossReportSummary::profit(
                profit_label,
                net_profit,
                if net_profit > 0.0 { "Green" } else { "Red" },
                currency,
            ),
        ],
        net_profit,
    )
}

pub fn get_chart_data(
    filters: &ProfitLossFilters,
    chart_columns: &[ProfitLossPeriod],
    income: &[ProfitLossRow],
    expense: &[ProfitLossRow],
    net_profit_loss: Option<&ProfitLossRow>,
    currency: &str,
) -> ProfitLossChart {
    let labels = chart_columns
        .iter()
        .map(|period| period.label.clone())
        .collect::<Vec<_>>();
    let mut datasets = Vec::new();

    if let Some(row) = second_last(income) {
        datasets.push(ProfitLossChartDataset::new(
            "Income",
            chart_columns
                .iter()
                .map(|period| row.value(&period.key))
                .collect(),
        ));
    }

    if let Some(row) = second_last(expense) {
        datasets.push(ProfitLossChartDataset::new(
            "Expense",
            chart_columns
                .iter()
                .map(|period| row.value(&period.key))
                .collect(),
        ));
    }

    if let Some(row) = net_profit_loss {
        datasets.push(ProfitLossChartDataset::new(
            "Net Profit/Loss",
            chart_columns
                .iter()
                .map(|period| row.value(&period.key))
                .collect(),
        ));
    }

    ProfitLossChart {
        labels,
        datasets,
        chart_type: if filters.accumulated_values {
            "line"
        } else {
            "bar"
        }
        .to_string(),
        fieldtype: "Currency".to_string(),
        options: "currency".to_string(),
        currency: currency.to_string(),
    }
}

fn second_last(rows: &[ProfitLossRow]) -> Option<&ProfitLossRow> {
    rows.len().checked_sub(2).and_then(|index| rows.get(index))
}

fn round_to(value: f64, precision: i32) -> f64 {
    let multiplier = 10_f64.powi(precision);
    (value * multiplier).round() / multiplier
}

pub fn compute_growth_view_data(data: &mut [ProfitLossRow], columns: &[ProfitLossPeriod]) {
    let data_copy = data.to_vec();

    for (row_idx, row) in data.iter_mut().enumerate() {
        for column_idx in 1..columns.len() {
            let previous_period_key = &columns[column_idx - 1].key;
            let current_period_key = &columns[column_idx].key;
            let current_period_value = data_copy[row_idx].value(current_period_key);
            let previous_period_value = data_copy[row_idx].value(previous_period_key);
            let mut annual_growth = 0.0;

            if previous_period_value == 0.0 && current_period_value > 0.0 {
                annual_growth = 1.0;
            } else if previous_period_value > 0.0 {
                annual_growth =
                    (current_period_value - previous_period_value) / previous_period_value;
            }

            row.values.insert(
                current_period_key.clone(),
                round_to(annual_growth * 100.0, 2),
            );
        }
    }
}

pub fn compute_margin_view_data(
    data: &mut [ProfitLossRow],
    columns: &[ProfitLossPeriod],
    accumulated_values: bool,
) {
    let Some(base_row) = data
        .iter()
        .find(|row| row.account_name == "Income")
        .cloned()
    else {
        return;
    };
    let data_copy = data.to_vec();

    for (row_idx, row) in data.iter_mut().enumerate() {
        for column in columns {
            let base_value = base_row.value(&column.key);
            let current_value = data_copy[row_idx].value(&column.key);

            if base_value <= 0.0 {
                row.values.remove(&column.key);
                continue;
            }

            row.values.insert(
                column.key.clone(),
                round_to((current_value / base_value) * 100.0, 2),
            );
        }

        if !accumulated_values {
            let base_value = base_row.total;
            if base_value <= 0.0 {
                row.total = 0.0;
            } else {
                row.total = round_to((data_copy[row_idx].total / base_value) * 100.0, 2);
            }
        }
    }
}
