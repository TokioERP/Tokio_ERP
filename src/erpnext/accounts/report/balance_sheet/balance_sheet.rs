use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BalanceSheetFilters {
    pub company: String,
    pub presentation_currency: Option<String>,
    pub accumulated_values: bool,
    pub selected_view: Option<String>,
    pub accumulated_in_group_company: bool,
    pub report_template: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BalanceSheetPeriod {
    pub key: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BalanceSheetRow {
    pub account_name: String,
    pub account: String,
    pub warn_if_negative: bool,
    pub currency: String,
    pub values: BTreeMap<String, f64>,
    pub total: f64,
    pub opening_balance: f64,
    pub is_empty: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BalanceSheetReportSummary {
    pub value: f64,
    pub label: String,
    pub datatype: String,
    pub currency: String,
    pub indicator: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BalanceSheetChartDataset {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BalanceSheetChart {
    pub labels: Vec<String>,
    pub datasets: Vec<BalanceSheetChartDataset>,
    pub chart_type: String,
    pub fieldtype: String,
    pub options: String,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BalanceSheetReport {
    pub columns: Vec<String>,
    pub rows: Vec<BalanceSheetRow>,
    pub message: Option<String>,
    pub chart: Option<BalanceSheetChart>,
    pub report_summary: Vec<BalanceSheetReportSummary>,
    pub primitive_summary: f64,
    pub delegated_to_financial_report_engine: bool,
}

impl BalanceSheetPeriod {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
        }
    }
}

impl BalanceSheetRow {
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
            opening_balance: 0.0,
            is_empty: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            account_name: String::new(),
            account: String::new(),
            warn_if_negative: false,
            currency: String::new(),
            values: BTreeMap::new(),
            total: 0.0,
            opening_balance: 0.0,
            is_empty: true,
        }
    }

    pub fn opening_balance(opening_balance: f64) -> Self {
        Self {
            opening_balance,
            ..Self::empty()
        }
    }

    pub fn provisional(account: &str, currency: &str) -> Self {
        Self {
            account_name: account.to_string(),
            account: account.to_string(),
            warn_if_negative: true,
            currency: currency.to_string(),
            values: BTreeMap::new(),
            total: 0.0,
            opening_balance: 0.0,
            is_empty: false,
        }
    }

    pub fn value(&self, key: &str) -> f64 {
        *self.values.get(key).unwrap_or(&0.0)
    }
}

impl BalanceSheetReportSummary {
    pub fn currency(label: &str, value: f64, currency: &str) -> Self {
        Self {
            value,
            label: label.to_string(),
            datatype: "Currency".to_string(),
            currency: currency.to_string(),
            indicator: None,
        }
    }

    pub fn provisional(label: &str, value: f64, indicator: &str, currency: &str) -> Self {
        Self {
            indicator: Some(indicator.to_string()),
            ..Self::currency(label, value, currency)
        }
    }
}

impl BalanceSheetChartDataset {
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }
}

pub fn execute(
    filters: &BalanceSheetFilters,
    columns: Vec<String>,
    period_list: Vec<BalanceSheetPeriod>,
    asset: Vec<BalanceSheetRow>,
    liability: Vec<BalanceSheetRow>,
    equity: Vec<BalanceSheetRow>,
    float_precision: i32,
) -> BalanceSheetReport {
    if filters.report_template {
        return BalanceSheetReport {
            columns: Vec::new(),
            rows: Vec::new(),
            message: None,
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
    let (mut provisional_profit_loss, total_credit) = get_provisional_profit_loss(
        &asset,
        &liability,
        &equity,
        &period_list,
        &filters.company,
        Some(&currency),
        false,
    );
    let opening = check_opening_balance(&asset, &liability, &equity, float_precision);
    let message = opening.as_ref().map(|(message, _)| message.clone());

    let mut rows = Vec::new();
    rows.extend(asset.clone());
    rows.extend(liability.clone());
    rows.extend(equity.clone());

    if let Some((_, opening_balance)) = opening {
        if round_to(opening_balance, 2) != 0.0 {
            let mut unclosed = BalanceSheetRow::provisional(
                "'Unclosed Fiscal Years Profit / Loss (Credit)'",
                &currency,
            );

            for period in &period_list {
                unclosed.values.insert(period.key.clone(), opening_balance);
                if let Some(row) = provisional_profit_loss.as_mut() {
                    let adjusted = row.value(&period.key) - opening_balance;
                    row.values.insert(period.key.clone(), adjusted);
                }
            }

            unclosed.total = opening_balance;
            rows.push(unclosed);
        }
    }

    if let Some(row) = provisional_profit_loss.clone() {
        rows.push(row);
    }
    if !total_credit.values.is_empty() {
        rows.push(total_credit);
    }

    let chart = get_chart_data(
        filters,
        &period_list,
        &asset,
        &liability,
        &equity,
        &currency,
    );
    let (report_summary, primitive_summary) = get_report_summary(
        &period_list,
        &asset,
        &liability,
        &equity,
        provisional_profit_loss.as_ref(),
        &currency,
        filters,
        false,
    );

    match filters.selected_view.as_deref() {
        Some("Growth") => compute_growth_view_data(&mut rows, &period_list),
        _ => {}
    }

    BalanceSheetReport {
        columns,
        rows,
        message,
        chart: Some(chart),
        report_summary,
        primitive_summary,
        delegated_to_financial_report_engine: false,
    }
}

pub fn get_provisional_profit_loss(
    asset: &[BalanceSheetRow],
    liability: &[BalanceSheetRow],
    equity: &[BalanceSheetRow],
    period_list: &[BalanceSheetPeriod],
    company: &str,
    currency: Option<&str>,
    consolidated: bool,
) -> (Option<BalanceSheetRow>, BalanceSheetRow) {
    let mut total = 0.0;
    let mut total_row_total = 0.0;
    let mut has_value = false;
    let currency = currency.unwrap_or(company);
    let mut provisional =
        BalanceSheetRow::provisional("'Provisional Profit / Loss (Credit)'", currency);
    let mut total_row = BalanceSheetRow::provisional("'Total (Credit)'", currency);

    if asset.is_empty() {
        return (None, total_row);
    }

    for period in period_list {
        let key = if consolidated {
            period.label.as_str()
        } else {
            period.key.as_str()
        };
        let total_assets = second_last(asset).map_or(0.0, |row| row.value(key));
        let mut effective_liability = 0.0;

        if last_is_empty(liability) {
            effective_liability += second_last(liability).map_or(0.0, |row| row.value(key));
        }
        if last_is_empty(equity) {
            effective_liability += second_last(equity).map_or(0.0, |row| row.value(key));
        }

        let value = total_assets - effective_liability;
        provisional.values.insert(key.to_string(), value);
        total_row
            .values
            .insert(key.to_string(), value + effective_liability);

        if value != 0.0 {
            has_value = true;
        }

        total += value;
        provisional.total = total;

        total_row_total += value + effective_liability;
        total_row.total = total_row_total;
    }

    (has_value.then_some(provisional), total_row)
}

pub fn check_opening_balance(
    asset: &[BalanceSheetRow],
    liability: &[BalanceSheetRow],
    equity: &[BalanceSheetRow],
    float_precision: i32,
) -> Option<(String, f64)> {
    let mut opening_balance = 0.0;

    if let Some(row) = asset.last() {
        opening_balance = round_to(row.opening_balance, float_precision);
    }
    if let Some(row) = liability.last() {
        opening_balance -= round_to(row.opening_balance, float_precision);
    }
    if let Some(row) = equity.last() {
        opening_balance -= round_to(row.opening_balance, float_precision);
    }

    opening_balance = round_to(opening_balance, float_precision);
    (opening_balance != 0.0).then_some((
        "Previous Financial Year is not closed".to_string(),
        opening_balance,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn get_report_summary(
    period_list: &[BalanceSheetPeriod],
    asset: &[BalanceSheetRow],
    liability: &[BalanceSheetRow],
    equity: &[BalanceSheetRow],
    provisional_profit_loss: Option<&BalanceSheetRow>,
    currency: &str,
    filters: &BalanceSheetFilters,
    consolidated: bool,
) -> (Vec<BalanceSheetReportSummary>, f64) {
    let mut net_asset = 0.0;
    let mut net_liability = 0.0;
    let mut net_equity = 0.0;
    let mut net_provisional_profit_loss = 0.0;

    let mut selected_periods = if filters.accumulated_values {
        period_list
            .last()
            .map(|period| vec![period])
            .unwrap_or_default()
    } else {
        period_list.iter().collect::<Vec<_>>()
    };

    if filters.accumulated_in_group_company {
        selected_periods
            .retain(|period| period.key == filters.company || period.label == filters.company);
    }

    for period in selected_periods {
        let key = if consolidated {
            period.label.as_str()
        } else {
            period.key.as_str()
        };
        if let Some(row) = second_last(asset) {
            net_asset += row.value(key);
        }
        if last_is_empty(liability) {
            net_liability += second_last(liability).map_or(0.0, |row| row.value(key));
        }
        if last_is_empty(equity) {
            net_equity += second_last(equity).map_or(0.0, |row| row.value(key));
        }
        if let Some(row) = provisional_profit_loss {
            net_provisional_profit_loss += row.value(key);
        }
    }

    (
        vec![
            BalanceSheetReportSummary::currency("Total Asset", net_asset, currency),
            BalanceSheetReportSummary::currency("Total Liability", net_liability, currency),
            BalanceSheetReportSummary::currency("Total Equity", net_equity, currency),
            BalanceSheetReportSummary::provisional(
                "Provisional Profit / Loss (Credit)",
                net_provisional_profit_loss,
                if net_provisional_profit_loss > 0.0 {
                    "Green"
                } else {
                    "Red"
                },
                currency,
            ),
        ],
        net_asset - net_liability + net_equity,
    )
}

pub fn get_chart_data(
    filters: &BalanceSheetFilters,
    chart_columns: &[BalanceSheetPeriod],
    asset: &[BalanceSheetRow],
    liability: &[BalanceSheetRow],
    equity: &[BalanceSheetRow],
    currency: &str,
) -> BalanceSheetChart {
    let labels = chart_columns
        .iter()
        .map(|period| period.label.clone())
        .collect::<Vec<_>>();
    let mut datasets = Vec::new();

    if let Some(row) = second_last(asset) {
        datasets.push(BalanceSheetChartDataset::new(
            "Assets",
            chart_columns
                .iter()
                .map(|period| row.value(&period.key))
                .collect(),
        ));
    }

    if let Some(row) = second_last(liability) {
        datasets.push(BalanceSheetChartDataset::new(
            "Liabilities",
            chart_columns
                .iter()
                .map(|period| row.value(&period.key))
                .collect(),
        ));
    }

    if let Some(row) = second_last(equity) {
        datasets.push(BalanceSheetChartDataset::new(
            "Equity",
            chart_columns
                .iter()
                .map(|period| row.value(&period.key))
                .collect(),
        ));
    }

    BalanceSheetChart {
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

fn second_last(rows: &[BalanceSheetRow]) -> Option<&BalanceSheetRow> {
    rows.len().checked_sub(2).and_then(|index| rows.get(index))
}

fn last_is_empty(rows: &[BalanceSheetRow]) -> bool {
    rows.last().is_some_and(|row| row.is_empty)
}

fn round_to(value: f64, precision: i32) -> f64 {
    let multiplier = 10_f64.powi(precision);
    (value * multiplier).round() / multiplier
}

pub fn compute_growth_view_data(data: &mut [BalanceSheetRow], columns: &[BalanceSheetPeriod]) {
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
