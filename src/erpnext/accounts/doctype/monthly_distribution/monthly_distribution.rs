use std::collections::BTreeMap;

use crate::erpnext::accounts::doctype::monthly_distribution_percentage::monthly_distribution_percentage::MonthlyDistributionPercentage;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MonthlyDistribution {
    pub distribution_id: Option<String>,
    pub fiscal_year: Option<String>,
    pub percentages: Vec<MonthlyDistributionPercentage>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MonthlyDistributionError {
    PercentageAllocationMismatch { total: f64 },
    InvalidDate(String),
    UnsupportedPeriodicity(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributionPeriod {
    pub key: String,
    pub from_date: String,
}

impl DistributionPeriod {
    pub fn new(key: impl Into<String>, from_date: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            from_date: from_date.into(),
        }
    }
}

impl MonthlyDistribution {
    pub const DOCTYPE: &'static str = "Monthly Distribution";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["distribution_id", "fiscal_year", "percentages"];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("distribution_id", "Distribution Name")
                .description("Name of the Monthly Distribution")
                .oldfield("distribution_id", "Data")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::link("fiscal_year", "Fiscal Year")
                .options("Fiscal Year")
                .oldfield("fiscal_year", "Select")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            FieldSpec::table("percentages", "Monthly Distribution Percentages")
                .options("Monthly Distribution Percentage")
                .oldfield("budget_distribution_details", "Table"),
        ]
    }

    pub fn get_months(&mut self) {
        self.percentages.clear();

        for (idx, month) in MONTH_NAMES.iter().enumerate() {
            let mut row = MonthlyDistributionPercentage::new(*month, 100.0 / 12.0);
            row.idx = Some((idx + 1) as u16);
            self.percentages.push(row);
        }
    }

    pub fn validate(&self) -> Result<(), MonthlyDistributionError> {
        let total: f64 = self
            .percentages
            .iter()
            .map(|row| row.percentage_allocation.unwrap_or(0.0))
            .sum();
        let total = flt2(total);

        if total != 100.0 {
            return Err(MonthlyDistributionError::PercentageAllocationMismatch { total });
        }

        Ok(())
    }
}

impl DocumentController for MonthlyDistribution {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn get_periodwise_distribution_data(
    doc: &MonthlyDistribution,
    period_list: &[DistributionPeriod],
    periodicity: &str,
) -> Result<BTreeMap<String, f64>, MonthlyDistributionError> {
    let months_to_add = match periodicity {
        "Yearly" => 12,
        "Half-Yearly" => 6,
        "Quarterly" => 3,
        "Monthly" => 1,
        _ => {
            return Err(MonthlyDistributionError::UnsupportedPeriodicity(
                periodicity.to_string(),
            ))
        }
    };

    let mut period_dict = BTreeMap::new();
    for period in period_list {
        period_dict.insert(
            period.key.clone(),
            get_percentage(doc, &period.from_date, months_to_add)?,
        );
    }
    Ok(period_dict)
}

pub fn get_percentage(
    doc: &MonthlyDistribution,
    start_date: &str,
    period: i32,
) -> Result<f64, MonthlyDistributionError> {
    let start_date = SimpleDate::parse(start_date)?;
    let mut months = Vec::new();

    for offset in 0..period {
        months.push(start_date.add_months(offset).month_name());
    }

    Ok(doc
        .percentages
        .iter()
        .filter(|row| {
            row.month
                .as_deref()
                .is_some_and(|month| months.contains(&month))
        })
        .map(|row| row.percentage_allocation.unwrap_or(0.0))
        .sum())
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SimpleDate {
    year: i32,
    month: u32,
    day: u32,
}

impl SimpleDate {
    fn parse(value: &str) -> Result<Self, MonthlyDistributionError> {
        let mut parts = value.split('-');
        let year = parts
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| MonthlyDistributionError::InvalidDate(value.to_string()))?;
        let month = parts
            .next()
            .and_then(|value| value.parse().ok())
            .filter(|month| (1..=12).contains(month))
            .ok_or_else(|| MonthlyDistributionError::InvalidDate(value.to_string()))?;
        let day = parts
            .next()
            .and_then(|value| value.parse().ok())
            .filter(|day| (1..=31).contains(day))
            .ok_or_else(|| MonthlyDistributionError::InvalidDate(value.to_string()))?;

        if parts.next().is_some() {
            return Err(MonthlyDistributionError::InvalidDate(value.to_string()));
        }

        Ok(Self { year, month, day })
    }

    fn add_months(self, months: i32) -> Self {
        let month_index = self.year * 12 + self.month as i32 - 1 + months;
        let year = month_index.div_euclid(12);
        let month = month_index.rem_euclid(12) as u32 + 1;

        Self {
            year,
            month,
            day: self.day,
        }
    }

    fn month_name(self) -> &'static str {
        MONTH_NAMES[(self.month - 1) as usize]
    }
}

fn flt2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
