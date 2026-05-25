use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetVarianceFilters {
    pub company: String,
    pub budget_against: String,
    pub from_fiscal_year: String,
    pub to_fiscal_year: String,
    pub period: String,
    pub show_cumulative: bool,
    pub budget_against_filter: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYear {
    pub name: String,
    pub year_start_date: String,
    pub year_end_date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetRecord {
    pub name: String,
    pub account: String,
    pub dimension: String,
    pub budget_amount: f64,
    pub from_fiscal_year: String,
    pub to_fiscal_year: String,
    pub budget_start_date: String,
    pub budget_end_date: String,
    pub distributions: Vec<BudgetDistribution>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetDistribution {
    pub start_date: String,
    pub end_date: String,
    pub amount: f64,
    pub percent: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub account: String,
    pub budget_against: String,
    pub fiscal_year: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldtype: &'static str,
    pub fieldname: String,
    pub options: String,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetVarianceRow {
    pub budget_against: String,
    pub account: String,
    pub values: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetVarianceReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<BudgetVarianceRow>,
    pub chart_data: Option<ChartData>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub budget_values: Vec<f64>,
    pub actual_values: Vec<f64>,
    pub budget_dataset_name: &'static str,
    pub actual_dataset_name: &'static str,
    pub chart_type: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetRecordsQueryPlan {
    pub doctype: &'static str,
    pub budget_against_field: String,
    pub filters: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetDimensionsQueryPlan {
    pub doctype: String,
    pub filters: Vec<String>,
    pub order_by: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Period {
    fiscal_year: String,
    from_date: ErpDate,
    to_date: ErpDate,
    label_suffix: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ErpDate {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetMap {
    order: Vec<(String, String)>,
    values: BTreeMap<String, BTreeMap<String, BTreeMap<String, BTreeMap<String, BudgetActual>>>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BudgetActual {
    pub budget: f64,
    pub actual: f64,
}

impl GlEntry {
    pub fn new(
        account: &str,
        budget_against: &str,
        fiscal_year: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
    ) -> Self {
        Self {
            account: account.to_string(),
            budget_against: budget_against.to_string(),
            fiscal_year: fiscal_year.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
        }
    }
}

impl ReportColumn {
    pub fn link(label: &str, fieldname: &str, options: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldtype: "Link",
            fieldname: fieldname.to_string(),
            options: options.to_string(),
            width,
        }
    }

    pub fn float(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldtype: "Float",
            fieldname: fieldname.to_string(),
            options: String::new(),
            width,
        }
    }
}

pub fn execute(
    filters: &BudgetVarianceFilters,
    fiscal_years: &[FiscalYear],
    dimensions: &[String],
    budget_records: &[BudgetRecord],
    actuals: &[GlEntry],
) -> BudgetVarianceReport {
    let columns = get_columns(filters, fiscal_years);
    let selected_dimensions = filters
        .budget_against_filter
        .clone()
        .unwrap_or_else(|| dimensions.to_vec());

    if selected_dimensions.is_empty() {
        return BudgetVarianceReport {
            columns,
            rows: Vec::new(),
            chart_data: None,
        };
    }

    let budget_records: Vec<BudgetRecord> = budget_records
        .iter()
        .filter(|budget| selected_dimensions.contains(&budget.dimension))
        .cloned()
        .collect();
    let budget_map = build_budget_map(&budget_records, actuals, fiscal_years);
    let rows = build_report_data(&budget_map, filters, fiscal_years);
    let chart_data = build_comparison_chart_data(&columns, &rows);

    BudgetVarianceReport {
        columns,
        rows,
        chart_data,
    }
}

pub fn get_columns(
    filters: &BudgetVarianceFilters,
    fiscal_years: &[FiscalYear],
) -> Vec<ReportColumn> {
    let mut columns = vec![
        ReportColumn::link(
            &filters.budget_against,
            "budget_against",
            &filters.budget_against,
            150,
        ),
        ReportColumn::link("Account", "account", "Account", 150),
    ];

    let group_months = filters.period != "Monthly";
    for fiscal_year in fiscal_years {
        for period in get_period_date_ranges(filters, fiscal_year) {
            if filters.period == "Yearly" {
                for label in [
                    format!("Budget {}", fiscal_year.name),
                    format!("Actual {}", fiscal_year.name),
                    format!("Variance {}", fiscal_year.name),
                ] {
                    columns.push(ReportColumn::float(&label, &scrub(&label), 150));
                }
            } else {
                let suffix = if group_months {
                    format!(
                        "{}-{}",
                        month_abbr(period.from_date.month),
                        month_abbr(period.to_date.month)
                    )
                } else {
                    month_abbr(period.from_date.month).to_string()
                };

                for label in [
                    format!("Budget ({suffix}) {}", fiscal_year.name),
                    format!("Actual ({suffix}) {}", fiscal_year.name),
                    format!("Variance ({suffix}) {}", fiscal_year.name),
                ] {
                    columns.push(ReportColumn::float(&label, &scrub(&label), 150));
                }
            }
        }
    }

    if filters.period != "Yearly" {
        for label in ["Total Budget", "Total Actual", "Total Variance"] {
            columns.push(ReportColumn::float(label, &scrub(label), 150));
        }
    }

    columns
}

pub fn build_budget_map(
    budget_records: &[BudgetRecord],
    actuals: &[GlEntry],
    fiscal_years: &[FiscalYear],
) -> BudgetMap {
    let mut budget_map = BudgetMap::default();

    for budget in budget_records {
        let order_key = (budget.dimension.clone(), budget.account.clone());
        if !budget_map.order.contains(&order_key) {
            budget_map.order.push(order_key);
        }
        budget_map
            .values
            .entry(budget.dimension.clone())
            .or_insert_with(BTreeMap::new)
            .entry(budget.account.clone())
            .or_insert_with(BTreeMap::new);

        for row in &budget.distributions {
            let months = get_months_in_range(&row.start_date, &row.end_date);
            if months.is_empty() {
                continue;
            }

            let monthly_budget = row.amount / months.len() as f64;
            for month_date in months {
                let fiscal_year = fiscal_year_for_date(month_date, fiscal_years);
                let month = month_name(month_date.month).to_string();
                let values = budget_map
                    .values
                    .entry(budget.dimension.clone())
                    .or_insert_with(BTreeMap::new)
                    .entry(budget.account.clone())
                    .or_insert_with(BTreeMap::new)
                    .entry(fiscal_year.clone())
                    .or_insert_with(BTreeMap::new)
                    .entry(month.clone())
                    .or_insert_with(BudgetActual::default);

                values.budget += monthly_budget;

                for actual in actuals.iter().filter(|actual| {
                    actual.account == budget.account
                        && actual.budget_against == budget.dimension
                        && actual.fiscal_year == fiscal_year
                        && month_name(parse_date(&actual.posting_date).month) == month
                }) {
                    values.actual += actual.debit - actual.credit;
                }
            }
        }
    }

    budget_map
}

pub fn build_report_data(
    budget_map: &BudgetMap,
    filters: &BudgetVarianceFilters,
    fiscal_years: &[FiscalYear],
) -> Vec<BudgetVarianceRow> {
    let mut data = Vec::new();
    let show_cumulative = filters.show_cumulative && filters.period != "Yearly";
    let periods = get_periods(filters, fiscal_years);

    for (dimension, account) in &budget_map.order {
        let fiscal_year_map = budget_map
            .values
            .get(dimension)
            .and_then(|accounts| accounts.get(account));

        let mut row = BudgetVarianceRow {
            budget_against: dimension.clone(),
            account: account.clone(),
            values: BTreeMap::new(),
        };
        let mut running_budget = 0.0;
        let mut running_actual = 0.0;
        let mut total_budget = 0.0;
        let mut total_actual = 0.0;

        for period in &periods {
            let months = get_months_between(period.from_date, period.to_date);
            let month_map = fiscal_year_map
                .and_then(|fiscal_year_map| fiscal_year_map.get(&period.fiscal_year));
            let mut period_budget = 0.0;
            let mut period_actual = 0.0;

            for month in months {
                if let Some(values) = month_map.and_then(|month_map| month_map.get(&month)) {
                    period_budget += values.budget;
                    period_actual += values.actual;
                }
            }

            let (display_budget, display_actual) = if show_cumulative {
                running_budget += period_budget;
                running_actual += period_actual;
                (running_budget, running_actual)
            } else {
                (period_budget, period_actual)
            };

            total_budget += period_budget;
            total_actual += period_actual;

            let (budget_label, actual_label, variance_label) = if filters.period == "Yearly" {
                (
                    format!("Budget {}", period.fiscal_year),
                    format!("Actual {}", period.fiscal_year),
                    format!("Variance {}", period.fiscal_year),
                )
            } else {
                (
                    format!("Budget ({}) {}", period.label_suffix, period.fiscal_year),
                    format!("Actual ({}) {}", period.label_suffix, period.fiscal_year),
                    format!("Variance ({}) {}", period.label_suffix, period.fiscal_year),
                )
            };

            row.values.insert(scrub(&budget_label), display_budget);
            row.values.insert(scrub(&actual_label), display_actual);
            row.values
                .insert(scrub(&variance_label), display_budget - display_actual);
        }

        if filters.period != "Yearly" {
            row.values.insert("total_budget".to_string(), total_budget);
            row.values.insert("total_actual".to_string(), total_actual);
            row.values
                .insert("total_variance".to_string(), total_budget - total_actual);
        }

        data.push(row);
    }

    data
}

pub fn build_comparison_chart_data(
    columns: &[ReportColumn],
    data: &[BudgetVarianceRow],
) -> Option<ChartData> {
    if data.is_empty() {
        return None;
    }

    let budget_fields: Vec<&str> = columns
        .iter()
        .map(|column| column.fieldname.as_str())
        .filter(|fieldname| fieldname.starts_with("budget_"))
        .collect();
    let actual_fields: Vec<&str> = columns
        .iter()
        .map(|column| column.fieldname.as_str())
        .filter(|fieldname| fieldname.starts_with("actual_"))
        .collect();

    if budget_fields.is_empty() || actual_fields.is_empty() {
        return None;
    }

    let labels = columns
        .iter()
        .filter(|column| column.fieldname.starts_with("budget_"))
        .map(|column| column.label.replace("Budget", "").trim().to_string())
        .collect();
    let mut budget_values = vec![0.0; budget_fields.len()];
    let mut actual_values = vec![0.0; actual_fields.len()];

    for row in data {
        for (index, field) in budget_fields.iter().enumerate() {
            budget_values[index] += row.values.get(*field).copied().unwrap_or(0.0);
        }
        for (index, field) in actual_fields.iter().enumerate() {
            actual_values[index] += row.values.get(*field).copied().unwrap_or(0.0);
        }
    }

    Some(ChartData {
        labels,
        budget_values,
        actual_values,
        budget_dataset_name: "Budget",
        actual_dataset_name: "Actual Expense",
        chart_type: "bar",
    })
}

pub fn get_budget_records_query(
    filters: &BudgetVarianceFilters,
    dimensions: &[&str],
) -> BudgetRecordsQueryPlan {
    let budget_against_field = scrub(&filters.budget_against);
    BudgetRecordsQueryPlan {
        doctype: "Budget",
        budget_against_field: budget_against_field.clone(),
        filters: vec![
            format!("b.company = {}", filters.company),
            "b.docstatus = 1".to_string(),
            format!("b.budget_against = {}", filters.budget_against),
            format!("b.{budget_against_field} in [{}]", dimensions.join(", ")),
            format!("b.from_fiscal_year <= {}", filters.to_fiscal_year),
            format!("b.to_fiscal_year >= {}", filters.from_fiscal_year),
        ],
    }
}

pub fn get_budget_dimensions_query(filters: &BudgetVarianceFilters) -> BudgetDimensionsQueryPlan {
    let mut query = BudgetDimensionsQueryPlan {
        doctype: filters.budget_against.clone(),
        filters: Vec::new(),
        order_by: None,
    };

    if matches!(filters.budget_against.as_str(), "Cost Center" | "Project") {
        query.filters.push(format!("company = {}", filters.company));
    }

    if filters.budget_against == "Cost Center" {
        query.order_by = Some("lft".to_string());
    }

    query
}

fn get_periods(filters: &BudgetVarianceFilters, fiscal_years: &[FiscalYear]) -> Vec<Period> {
    fiscal_years
        .iter()
        .flat_map(|fiscal_year| get_period_date_ranges(filters, fiscal_year))
        .collect()
}

fn get_period_date_ranges(
    filters: &BudgetVarianceFilters,
    fiscal_year: &FiscalYear,
) -> Vec<Period> {
    let start = parse_date(&fiscal_year.year_start_date);
    let end = parse_date(&fiscal_year.year_end_date);
    let step_months = match filters.period.as_str() {
        "Yearly" => 12,
        "Half-Yearly" => 6,
        "Quarterly" => 3,
        _ => 1,
    };
    let mut periods = Vec::new();
    let mut from_date = start;

    while from_date <= end {
        let mut to_date = from_date.add_months(step_months).add_days(-1);
        if to_date > end {
            to_date = end;
        }
        let label_suffix = if filters.period == "Yearly" {
            fiscal_year.name.clone()
        } else if filters.period == "Monthly" {
            month_abbr(from_date.month).to_string()
        } else {
            format!(
                "{}-{}",
                month_abbr(from_date.month),
                month_abbr(to_date.month)
            )
        };

        periods.push(Period {
            fiscal_year: fiscal_year.name.clone(),
            from_date,
            to_date,
            label_suffix,
        });

        from_date = to_date.add_days(1);
    }

    periods
}

fn get_months_in_range(start_date: &str, end_date: &str) -> Vec<ErpDate> {
    let mut months = Vec::new();
    let mut current = parse_date(start_date);
    let end = parse_date(end_date);

    while current <= end {
        months.push(current);
        current = current.add_months(1);
    }

    months
}

fn get_months_between(from_date: ErpDate, to_date: ErpDate) -> Vec<String> {
    let mut months = Vec::new();
    let mut current = from_date;

    while current <= to_date {
        months.push(month_name(current.month).to_string());
        current = current.add_months(1);
    }

    months
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

fn parse_date(value: &str) -> ErpDate {
    let mut parts = value.split('-');
    ErpDate {
        year: parts.next().unwrap_or("0").parse().unwrap_or(0),
        month: parts.next().unwrap_or("1").parse().unwrap_or(1),
        day: parts.next().unwrap_or("1").parse().unwrap_or(1),
    }
}

fn fiscal_year_for_date(date: ErpDate, fiscal_years: &[FiscalYear]) -> String {
    fiscal_years
        .iter()
        .find(|fiscal_year| {
            parse_date(&fiscal_year.year_start_date) <= date
                && date <= parse_date(&fiscal_year.year_end_date)
        })
        .map(|fiscal_year| fiscal_year.name.clone())
        .unwrap_or_else(|| date.year.to_string())
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
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

impl ErpDate {
    fn add_days(self, days: i32) -> Self {
        let mut date = self;
        if days >= 0 {
            for _ in 0..days {
                date = date.next_day();
            }
        } else {
            for _ in days..0 {
                date = date.previous_day();
            }
        }
        date
    }

    fn add_months(self, months: i32) -> Self {
        let zero_based = self.month as i32 - 1 + months;
        let year = self.year + zero_based.div_euclid(12);
        let month = zero_based.rem_euclid(12) as u32 + 1;
        let day = self.day.min(last_day_of_month(year, month));

        Self { year, month, day }
    }

    fn next_day(self) -> Self {
        let month_last_day = last_day_of_month(self.year, self.month);
        if self.day < month_last_day {
            Self {
                day: self.day + 1,
                ..self
            }
        } else if self.month < 12 {
            Self {
                year: self.year,
                month: self.month + 1,
                day: 1,
            }
        } else {
            Self {
                year: self.year + 1,
                month: 1,
                day: 1,
            }
        }
    }

    fn previous_day(self) -> Self {
        if self.day > 1 {
            Self {
                day: self.day - 1,
                ..self
            }
        } else if self.month > 1 {
            let month = self.month - 1;
            Self {
                year: self.year,
                month,
                day: last_day_of_month(self.year, month),
            }
        } else {
            Self {
                year: self.year - 1,
                month: 12,
                day: 31,
            }
        }
    }
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
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
