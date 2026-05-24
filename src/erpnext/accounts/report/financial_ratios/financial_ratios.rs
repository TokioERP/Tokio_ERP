use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FinancialRatioError {
    FiscalYearNotFound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialRatioFilters {
    pub from_fiscal_year: String,
    pub to_fiscal_year: String,
    pub company: String,
    pub period_start_date: Option<String>,
    pub period_end_date: Option<String>,
    pub filter_based_on: Option<String>,
    pub periodicity: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYearSpan {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Period {
    pub key: String,
    pub label: String,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RatioColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountEntry {
    pub account_name: String,
    pub parent_account: Option<String>,
    pub is_group: bool,
    pub account_type: Option<String>,
    pub values: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RatioRow {
    pub ratio: String,
    pub values: Vec<(String, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinancialRatioInput {
    pub precision: u32,
    pub assets: Vec<AccountEntry>,
    pub liabilities: Vec<AccountEntry>,
    pub income: Vec<AccountEntry>,
    pub expense: Vec<AccountEntry>,
    pub avg_debtors: BTreeMap<String, f64>,
    pub avg_creditors: BTreeMap<String, f64>,
    pub avg_stock: BTreeMap<String, f64>,
}

impl FinancialRatioFilters {
    pub fn new(
        from_fiscal_year: impl Into<String>,
        to_fiscal_year: impl Into<String>,
        company: impl Into<String>,
    ) -> Self {
        Self {
            from_fiscal_year: from_fiscal_year.into(),
            to_fiscal_year: to_fiscal_year.into(),
            company: company.into(),
            period_start_date: None,
            period_end_date: None,
            filter_based_on: None,
            periodicity: None,
        }
    }
}

impl FiscalYearSpan {
    pub fn new(
        name: impl Into<String>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            start_date: start_date.into(),
            end_date: end_date.into(),
        }
    }
}

impl Period {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            from_date: from_date.into(),
            to_date: to_date.into(),
        }
    }
}

impl RatioColumn {
    pub fn data(label: impl Into<String>, fieldname: impl Into<String>, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname: fieldname.into(),
            fieldtype: "Data",
            width,
        }
    }

    pub fn float(label: impl Into<String>, fieldname: impl Into<String>, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname: fieldname.into(),
            fieldtype: "Float",
            width,
        }
    }
}

impl AccountEntry {
    pub fn root(account_name: impl Into<String>) -> Self {
        Self {
            account_name: account_name.into(),
            parent_account: None,
            is_group: true,
            account_type: None,
            values: BTreeMap::new(),
        }
    }

    pub fn group(account_name: impl Into<String>, account_type: impl Into<String>) -> Self {
        Self {
            account_name: account_name.into(),
            parent_account: Some(String::new()),
            is_group: true,
            account_type: Some(account_type.into()),
            values: BTreeMap::new(),
        }
    }

    pub fn leaf(account_name: impl Into<String>, account_type: impl Into<String>) -> Self {
        Self {
            account_name: account_name.into(),
            parent_account: Some(String::new()),
            is_group: false,
            account_type: Some(account_type.into()),
            values: BTreeMap::new(),
        }
    }

    pub fn with_amount(mut self, period: impl Into<String>, amount: f64) -> Self {
        self.values.insert(period.into(), amount);
        self
    }

    fn value(&self, period: &str) -> f64 {
        self.values.get(period).copied().unwrap_or(0.0)
    }
}

impl RatioRow {
    pub fn section(label: impl Into<String>) -> Self {
        Self {
            ratio: label.into(),
            values: Vec::new(),
        }
    }

    pub fn with_values(
        label: impl Into<String>,
        years: &[String],
        values: &BTreeMap<String, f64>,
    ) -> Self {
        Self {
            ratio: label.into(),
            values: years
                .iter()
                .map(|year| (year.clone(), values.get(year).copied().unwrap_or(0.0)))
                .collect(),
        }
    }

    pub fn value(&self, year: &str) -> Option<f64> {
        self.values
            .iter()
            .find(|(key, _)| key == year)
            .map(|(_, value)| *value)
    }
}

pub fn setup_filters(
    filters: &mut FinancialRatioFilters,
    fiscal_years: &[FiscalYearSpan],
) -> Result<(), FinancialRatioError> {
    filters.filter_based_on = Some("Fiscal Year".to_string());

    if filters.period_start_date.is_none() {
        filters.period_start_date = Some(
            fiscal_years
                .iter()
                .find(|fiscal_year| fiscal_year.name == filters.from_fiscal_year)
                .ok_or(FinancialRatioError::FiscalYearNotFound)?
                .start_date
                .clone(),
        );
    }

    if filters.period_end_date.is_none() {
        filters.period_end_date = Some(
            fiscal_years
                .iter()
                .find(|fiscal_year| fiscal_year.name == filters.to_fiscal_year)
                .ok_or(FinancialRatioError::FiscalYearNotFound)?
                .end_date
                .clone(),
        );
    }

    Ok(())
}

pub fn get_columns(period_list: &[Period]) -> (Vec<RatioColumn>, Vec<String>) {
    let mut columns = vec![RatioColumn::data("Ratios", "ratio", 200)];
    let mut years = Vec::new();
    for period in period_list {
        columns.push(RatioColumn::float(&period.label, &period.key, 150));
        years.push(period.key.clone());
    }
    (columns, years)
}

pub fn get_ratios_data_from_gl(
    period_list: &[Period],
    years: &[String],
    input: FinancialRatioInput,
) -> Vec<RatioRow> {
    let mut data = Vec::new();
    let mut current_asset = BTreeMap::new();
    let mut total_asset = BTreeMap::new();
    let mut current_liability = BTreeMap::new();
    let mut total_liability = BTreeMap::new();
    let mut net_sales = BTreeMap::new();
    let mut total_income = BTreeMap::new();
    let mut cogs = BTreeMap::new();
    let mut total_expense = BTreeMap::new();
    let mut quick_asset = BTreeMap::new();
    let mut direct_expense = BTreeMap::new();

    for year in years {
        update_balances(
            &mut current_asset,
            &mut total_asset,
            "Current Asset",
            year,
            &input.assets,
            "Asset",
            Some(&mut quick_asset),
        );
        update_balances(
            &mut current_liability,
            &mut total_liability,
            "Current Liability",
            year,
            &input.liabilities,
            "Liability",
            None,
        );
        update_balances(
            &mut cogs,
            &mut total_expense,
            "Cost of Goods Sold",
            year,
            &input.expense,
            "Expense",
            None,
        );
        update_balances(
            &mut direct_expense,
            &mut BTreeMap::new(),
            "Direct Expense",
            year,
            &input.expense,
            "Expense",
            None,
        );
        update_balances(
            &mut net_sales,
            &mut total_income,
            "Direct Income",
            year,
            &input.income,
            "Income",
            None,
        );
    }

    add_liquidity_ratios(
        &mut data,
        years,
        &current_asset,
        &current_liability,
        &quick_asset,
        input.precision,
    );
    add_solvency_ratios(
        &mut data,
        years,
        &total_asset,
        &total_liability,
        &net_sales,
        &cogs,
        &total_income,
        &total_expense,
        input.precision,
    );
    add_turnover_ratios(
        &mut data,
        years,
        period_list,
        &total_asset,
        &net_sales,
        &cogs,
        &direct_expense,
        &input.avg_debtors,
        &input.avg_creditors,
        &input.avg_stock,
        input.precision,
    );

    data
}

pub fn update_balances(
    ratio_dict: &mut BTreeMap<String, f64>,
    total_dict: &mut BTreeMap<String, f64>,
    account_type: &str,
    year: &str,
    root_type_data: &[AccountEntry],
    root_type: &str,
    mut net_dict: Option<&mut BTreeMap<String, f64>>,
) {
    let mut total_net = 0.0;
    for entry in root_type_data {
        if entry.parent_account.is_none() && entry.is_group {
            let mut total = entry.value(year);
            if account_type == "Direct Expense" {
                total *= -1.0;
            }
            total_dict.insert(year.to_string(), total);
        }

        if root_type == "Asset" || root_type == "Liability" {
            if entry.account_type.as_deref() == Some(account_type) && entry.is_group {
                ratio_dict.insert(year.to_string(), entry.value(year));
            }
            if matches!(
                entry.account_type.as_deref(),
                Some("Bank") | Some("Cash") | Some("Receivable")
            ) && !entry.is_group
            {
                total_net += entry.value(year);
                if let Some(net_dict) = net_dict.as_deref_mut() {
                    net_dict.insert(year.to_string(), total_net);
                }
            }
        } else if root_type == "Income" {
            if entry.account_type.as_deref() == Some(account_type) && entry.is_group {
                total_net += entry.value(year);
                ratio_dict.insert(year.to_string(), total_net);
            }
        } else if root_type == "Expense" && account_type == "Cost of Goods Sold" {
            if entry.account_type.as_deref() == Some(account_type) {
                total_net += entry.value(year);
                ratio_dict.insert(year.to_string(), total_net);
            }
        } else if entry.account_type.as_deref() == Some(account_type) && entry.is_group {
            ratio_dict.insert(year.to_string(), entry.value(year));
        }
    }
}

pub fn add_liquidity_ratios(
    data: &mut Vec<RatioRow>,
    years: &[String],
    current_asset: &BTreeMap<String, f64>,
    current_liability: &BTreeMap<String, f64>,
    quick_asset: &BTreeMap<String, f64>,
    precision: u32,
) {
    data.push(RatioRow::section("Liquidity Ratios"));
    data.push(ratio_row(
        "Current Ratio",
        years,
        current_asset,
        current_liability,
        precision,
    ));
    data.push(ratio_row(
        "Quick Ratio",
        years,
        quick_asset,
        current_liability,
        precision,
    ));
}

pub fn add_solvency_ratios(
    data: &mut Vec<RatioRow>,
    years: &[String],
    total_asset: &BTreeMap<String, f64>,
    total_liability: &BTreeMap<String, f64>,
    net_sales: &BTreeMap<String, f64>,
    cogs: &BTreeMap<String, f64>,
    total_income: &BTreeMap<String, f64>,
    total_expense: &BTreeMap<String, f64>,
    precision: u32,
) {
    data.push(RatioRow::section("Solvency Ratios"));
    let mut debt_equity_ratio = BTreeMap::new();
    let mut gross_profit_ratio = BTreeMap::new();
    let mut net_profit_ratio = BTreeMap::new();
    let mut return_on_asset_ratio = BTreeMap::new();
    let mut return_on_equity_ratio = BTreeMap::new();

    for year in years {
        let profit_after_tax = value(total_income, year) - value(total_expense, year);
        let share_holder_fund = value(total_asset, year) - value(total_liability, year);
        debt_equity_ratio.insert(
            year.clone(),
            calculate_ratio(value(total_liability, year), share_holder_fund, precision),
        );
        return_on_equity_ratio.insert(
            year.clone(),
            calculate_ratio(profit_after_tax, share_holder_fund, precision),
        );
        net_profit_ratio.insert(
            year.clone(),
            calculate_ratio(profit_after_tax, value(net_sales, year), precision),
        );
        gross_profit_ratio.insert(
            year.clone(),
            calculate_ratio(
                value(net_sales, year) - value(cogs, year),
                value(net_sales, year),
                precision,
            ),
        );
        return_on_asset_ratio.insert(
            year.clone(),
            calculate_ratio(profit_after_tax, value(total_asset, year), precision),
        );
    }

    data.push(RatioRow::with_values(
        "Debt Equity Ratio",
        years,
        &debt_equity_ratio,
    ));
    data.push(RatioRow::with_values(
        "Gross Profit Ratio",
        years,
        &gross_profit_ratio,
    ));
    data.push(RatioRow::with_values(
        "Net Profit Ratio",
        years,
        &net_profit_ratio,
    ));
    data.push(RatioRow::with_values(
        "Return on Asset Ratio",
        years,
        &return_on_asset_ratio,
    ));
    data.push(RatioRow::with_values(
        "Return on Equity Ratio",
        years,
        &return_on_equity_ratio,
    ));
}

pub fn add_turnover_ratios(
    data: &mut Vec<RatioRow>,
    years: &[String],
    _period_list: &[Period],
    total_asset: &BTreeMap<String, f64>,
    net_sales: &BTreeMap<String, f64>,
    cogs: &BTreeMap<String, f64>,
    direct_expense: &BTreeMap<String, f64>,
    avg_debtors: &BTreeMap<String, f64>,
    avg_creditors: &BTreeMap<String, f64>,
    avg_stock: &BTreeMap<String, f64>,
    precision: u32,
) {
    data.push(RatioRow::section("Turnover Ratios"));
    data.push(ratio_row(
        "Fixed Asset Turnover Ratio",
        years,
        net_sales,
        total_asset,
        precision,
    ));
    data.push(ratio_row(
        "Debtor Turnover Ratio",
        years,
        net_sales,
        avg_debtors,
        precision,
    ));
    data.push(ratio_row(
        "Creditor Turnover Ratio",
        years,
        direct_expense,
        avg_creditors,
        precision,
    ));
    data.push(ratio_row(
        "Inventory Turnover Ratio",
        years,
        cogs,
        avg_stock,
        precision,
    ));
}

pub fn calculate_ratio(value: f64, denominator: f64, precision: u32) -> f64 {
    if denominator != 0.0 {
        round_to(value / denominator, precision)
    } else {
        0.0
    }
}

fn ratio_row(
    label: &str,
    years: &[String],
    numerator: &BTreeMap<String, f64>,
    denominator: &BTreeMap<String, f64>,
    precision: u32,
) -> RatioRow {
    RatioRow {
        ratio: label.to_string(),
        values: years
            .iter()
            .map(|year| {
                (
                    year.clone(),
                    calculate_ratio(value(numerator, year), value(denominator, year), precision),
                )
            })
            .collect(),
    }
}

fn value(map: &BTreeMap<String, f64>, year: &str) -> f64 {
    map.get(year).copied().unwrap_or(0.0)
}

fn round_to(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
