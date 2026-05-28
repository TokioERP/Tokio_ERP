use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrossNetFilters {
    pub company: String,
    pub presentation_currency: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Period {
    pub key: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrossNetRow {
    pub is_none: bool,
    pub account_name: Option<String>,
    pub account: Option<String>,
    pub parent_account: String,
    pub is_group: bool,
    pub include_in_gross: i32,
    pub indent: f64,
    pub values: BTreeMap<String, f64>,
    pub total: Option<f64>,
    pub warn_if_negative: bool,
    pub currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GrossNetReport {
    pub columns: Vec<String>,
    pub rows: Vec<GrossNetRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrossNetExecutionPlan {
    pub period_source: &'static str,
    pub columns_source: &'static str,
    pub income_data_call: FinancialStatementDataCall,
    pub expense_data_call: FinancialStatementDataCall,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialStatementDataCall {
    pub root_type: &'static str,
    pub balance_must_be: &'static str,
    pub ignore_closing_entries: bool,
    pub ignore_accumulated_values_for_fy: bool,
    pub total: bool,
}

impl Period {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

impl GrossNetRow {
    #[allow(clippy::too_many_arguments)]
    pub fn account(
        account_name: &str,
        account: &str,
        parent_account: &str,
        is_group: bool,
        include_in_gross: i32,
        indent: f64,
        values: &[(&str, f64)],
        total: f64,
    ) -> Self {
        Self {
            is_none: false,
            account_name: Some(account_name.to_string()),
            account: Some(account.to_string()),
            parent_account: parent_account.to_string(),
            is_group,
            include_in_gross,
            indent,
            values: values
                .iter()
                .map(|(key, value)| ((*key).to_string(), *value))
                .collect(),
            total: Some(total),
            warn_if_negative: false,
            currency: None,
        }
    }

    pub fn message(message: &str) -> Self {
        Self {
            is_none: false,
            account_name: Some(message.to_string()),
            account: Some(message.to_string()),
            parent_account: String::new(),
            is_group: false,
            include_in_gross: 0,
            indent: 0.0,
            values: BTreeMap::new(),
            total: None,
            warn_if_negative: false,
            currency: None,
        }
    }

    pub fn blank() -> Self {
        Self {
            is_none: false,
            account_name: None,
            account: None,
            parent_account: String::new(),
            is_group: false,
            include_in_gross: 0,
            indent: 0.0,
            values: BTreeMap::new(),
            total: None,
            warn_if_negative: false,
            currency: None,
        }
    }

    pub fn profit(profit_type: &str, currency: &str) -> Self {
        let label = format!("'{profit_type}'");
        Self {
            is_none: false,
            account_name: Some(label.clone()),
            account: Some(label),
            parent_account: String::new(),
            is_group: false,
            include_in_gross: 0,
            indent: 0.0,
            values: BTreeMap::new(),
            total: None,
            warn_if_negative: true,
            currency: Some(currency.to_string()),
        }
    }

    pub fn account_value(&self) -> &str {
        self.account.as_deref().unwrap_or("")
    }

    pub fn none() -> Self {
        Self {
            is_none: true,
            account_name: None,
            account: None,
            parent_account: String::new(),
            is_group: false,
            include_in_gross: 0,
            indent: 0.0,
            values: BTreeMap::new(),
            total: None,
            warn_if_negative: false,
            currency: None,
        }
    }

    pub fn value(&self, key: &str) -> f64 {
        *self.values.get(key).unwrap_or(&0.0)
    }

    pub fn is_blank(&self) -> bool {
        self.account.is_none()
            && self.account_name.is_none()
            && self.values.is_empty()
            && self.total.is_none()
            && !self.is_none
    }
}

impl Default for GrossNetExecutionPlan {
    fn default() -> Self {
        Self {
            period_source: "erpnext.accounts.report.financial_statements.get_period_list",
            columns_source: "erpnext.accounts.report.financial_statements.get_columns",
            income_data_call: FinancialStatementDataCall {
                root_type: "Income",
                balance_must_be: "Credit",
                ignore_closing_entries: true,
                ignore_accumulated_values_for_fy: true,
                total: false,
            },
            expense_data_call: FinancialStatementDataCall {
                root_type: "Expense",
                balance_must_be: "Debit",
                ignore_closing_entries: true,
                ignore_accumulated_values_for_fy: true,
                total: false,
            },
        }
    }
}

pub fn execute(
    filters: GrossNetFilters,
    columns: Vec<String>,
    period_list: Vec<Period>,
    income: Vec<GrossNetRow>,
    expense: Vec<GrossNetRow>,
) -> GrossNetReport {
    let mut data = Vec::new();
    let mut gross_income = get_revenue(&income, &period_list, 1);
    let mut gross_expense = get_revenue(&expense, &period_list, 1);

    if gross_income.is_empty() && gross_expense.is_empty() {
        data.push(GrossNetRow::message("'Nothing is included in gross'"));
        return GrossNetReport {
            columns,
            rows: data,
        };
    }

    if gross_income.is_empty() {
        gross_income = vec![GrossNetRow::blank()];
    }
    if gross_expense.is_empty() {
        gross_expense = vec![GrossNetRow::blank()];
    }

    data.push(GrossNetRow::message("'Included in Gross Profit'"));
    data.push(GrossNetRow::blank());
    data.extend(gross_income.clone());

    data.push(GrossNetRow::blank());
    data.extend(gross_expense.clone());

    data.push(GrossNetRow::blank());
    let gross_profit = get_profit(
        &gross_income,
        &gross_expense,
        &period_list,
        &filters.company,
        "Gross Profit",
        filters.presentation_currency.as_deref(),
    )
    .unwrap_or_else(GrossNetRow::none);
    data.push(gross_profit);

    let non_gross_income = get_revenue(&income, &period_list, 0);
    data.push(GrossNetRow::blank());
    data.extend(non_gross_income.clone());

    let non_gross_expense = get_revenue(&expense, &period_list, 0);
    data.push(GrossNetRow::blank());
    data.extend(non_gross_expense.clone());

    let net_profit = get_net_profit(
        &non_gross_income,
        &gross_income,
        &gross_expense,
        &non_gross_expense,
        &period_list,
        &filters.company,
        filters.presentation_currency.as_deref(),
    )
    .unwrap_or_else(GrossNetRow::none);
    data.push(GrossNetRow::blank());
    data.push(net_profit);

    GrossNetReport {
        columns,
        rows: data,
    }
}

pub fn get_revenue(
    data: &[GrossNetRow],
    period_list: &[Period],
    include_in_gross: i32,
) -> Vec<GrossNetRow> {
    let mut revenue: Vec<GrossNetRow> = data
        .iter()
        .filter(|item| item.include_in_gross == include_in_gross || item.is_group)
        .cloned()
        .collect();

    loop {
        let (next, removed) = remove_parent_with_no_child(revenue);
        revenue = next;
        if !removed {
            break;
        }
    }

    adjust_account_totals(&mut revenue, period_list);
    revenue
}

pub fn remove_parent_with_no_child(data: Vec<GrossNetRow>) -> (Vec<GrossNetRow>, bool) {
    let mut output = Vec::new();
    let mut removed = false;

    for parent in &data {
        if parent.is_group {
            let have_child = data.iter().any(|child| {
                child.parent_account == parent.account_value() && !child.parent_account.is_empty()
            });

            if !have_child {
                removed = true;
                continue;
            }
        }

        output.push(parent.clone());
    }

    (output, removed)
}

pub fn adjust_account_totals(data: &mut [GrossNetRow], period_list: &[Period]) {
    let mut totals = BTreeMap::new();

    for index in (0..data.len()).rev() {
        if data[index].is_group {
            let account = data[index].account_value().to_string();
            for period in period_list {
                let sum = data
                    .iter()
                    .filter(|item| item.parent_account == account)
                    .map(|item| item.value(&period.key))
                    .sum();
                data[index].values.insert(period.key.clone(), sum);
            }
        } else {
            set_total(index, data[index].total.unwrap_or(0.0), data, &mut totals);
        }

        let account = data[index].account_value().to_string();
        data[index].total = Some(*totals.get(&account).unwrap_or(&0.0));
    }
}

pub fn get_profit(
    gross_income: &[GrossNetRow],
    gross_expense: &[GrossNetRow],
    period_list: &[Period],
    company: &str,
    profit_type: &str,
    currency: Option<&str>,
) -> Option<GrossNetRow> {
    let mut profit_loss = GrossNetRow::profit(profit_type, currency.unwrap_or(company));
    let mut has_value = false;

    for period in period_list {
        let gross_income_for_period = gross_income
            .first()
            .map_or(0.0, |row| flt(row.value(&period.key)));
        let gross_expense_for_period = gross_expense
            .first()
            .map_or(0.0, |row| flt(row.value(&period.key)));
        let value = gross_income_for_period - gross_expense_for_period;
        profit_loss.values.insert(period.key.clone(), value);

        if value != 0.0 {
            has_value = true;
            *profit_loss.total.get_or_insert(0.0) += value;
        }
    }

    has_value.then_some(profit_loss)
}

pub fn get_net_profit(
    non_gross_income: &[GrossNetRow],
    gross_income: &[GrossNetRow],
    gross_expense: &[GrossNetRow],
    non_gross_expense: &[GrossNetRow],
    period_list: &[Period],
    company: &str,
    currency: Option<&str>,
) -> Option<GrossNetRow> {
    let mut profit_loss = GrossNetRow::profit("Net Profit", currency.unwrap_or(company));
    let mut has_value = false;

    let gross_income_roots = root_rows(gross_income);
    let non_gross_income_roots = root_rows(non_gross_income);
    let gross_expense_roots = root_rows(gross_expense);
    let non_gross_expense_roots = root_rows(non_gross_expense);

    for period in period_list {
        let gross_income_for_period = sum_period(&gross_income_roots, &period.key);
        let non_gross_income_for_period = sum_period(&non_gross_income_roots, &period.key);
        let gross_expense_for_period = sum_period(&gross_expense_roots, &period.key);
        let non_gross_expense_for_period = sum_period(&non_gross_expense_roots, &period.key);

        let total_income = gross_income_for_period + non_gross_income_for_period;
        let total_expense = gross_expense_for_period + non_gross_expense_for_period;
        let value = flt(total_income) - flt(total_expense);
        profit_loss.values.insert(period.key.clone(), value);

        if value != 0.0 {
            has_value = true;
            *profit_loss.total.get_or_insert(0.0) += value;
        }
    }

    has_value.then_some(profit_loss)
}

fn set_total(
    index: usize,
    value: f64,
    complete_list: &mut [GrossNetRow],
    totals: &mut BTreeMap<String, f64>,
) {
    let account = complete_list[index].account_value().to_string();
    *totals.entry(account).or_insert(0.0) += value;

    let parent = complete_list[index].parent_account.clone();
    if !parent.is_empty() {
        if let Some(parent_index) = complete_list
            .iter()
            .position(|item| item.account_value() == parent)
        {
            set_total(parent_index, value, complete_list, totals);
        }
    }
}

fn root_rows(rows: &[GrossNetRow]) -> Vec<&GrossNetRow> {
    rows.iter().filter(|row| flt(row.indent) == 0.0).collect()
}

fn sum_period(rows: &[&GrossNetRow], key: &str) -> f64 {
    rows.iter().map(|row| flt(row.value(key))).sum()
}

fn flt(value: f64) -> f64 {
    value
}
