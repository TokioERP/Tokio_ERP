use crate::erpnext::accounts::doctype::fiscal_year_company::fiscal_year_company::FiscalYearCompany;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FiscalYear {
    pub name: Option<String>,
    pub year: String,
    pub disabled: bool,
    pub is_short_year: bool,
    pub year_start_date: String,
    pub year_end_date: String,
    pub companies: Vec<FiscalYearCompany>,
    pub auto_created: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYearOverlap {
    pub name: String,
    pub companies: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiscalYearError {
    InvalidDate,
    FromDateAfterToDate,
    InvalidYearEndDate,
    OverlappingFiscalYear { fiscal_year: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SimpleDate {
    year: i32,
    month: u32,
    day: u32,
}

impl FiscalYear {
    pub const DOCTYPE: &'static str = "Fiscal Year";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "year",
        "disabled",
        "is_short_year",
        "year_start_date",
        "year_end_date",
        "companies",
        "auto_created",
    ];
    pub const SORT_FIELD: &'static str = "name";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("year", "Year Name")
                .description("For e.g. 2012, 2012-13")
                .oldfield("year", "Data")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::check("is_short_year", "Is Short/Long Year")
                .default("0")
                .description("More/Less than 12 months.")
                .set_only_once(),
            FieldSpec::date("year_start_date", "Year Start Date")
                .oldfield("year_start_date", "Date")
                .in_list_view()
                .no_copy()
                .required()
                .set_only_once(),
            FieldSpec::date("year_end_date", "Year End Date")
                .in_list_view()
                .no_copy()
                .required()
                .set_only_once(),
            FieldSpec::table("companies", "Companies").options("Fiscal Year Company"),
            FieldSpec::check("auto_created", "Auto Created")
                .default("0")
                .hidden()
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(
        &self,
        overlapping_fiscal_years: &[FiscalYearOverlap],
    ) -> Result<(), FiscalYearError> {
        self.validate_dates()?;
        self.validate_overlap(overlapping_fiscal_years)
    }

    pub fn validate_dates(&self) -> Result<(), FiscalYearError> {
        let start = SimpleDate::parse(&self.year_start_date)?;
        let end = SimpleDate::parse(&self.year_end_date)?;
        if start > end {
            return Err(FiscalYearError::FromDateAfterToDate);
        }
        if self.is_short_year {
            return Ok(());
        }

        let expected = start.add_years(1).add_days(-1);
        if end != expected {
            return Err(FiscalYearError::InvalidYearEndDate);
        }
        Ok(())
    }

    pub fn validate_overlap(
        &self,
        overlapping_fiscal_years: &[FiscalYearOverlap],
    ) -> Result<(), FiscalYearError> {
        for existing in overlapping_fiscal_years {
            let overlap = if self.companies.is_empty() && existing.companies.is_empty() {
                true
            } else {
                self.companies.iter().any(|company| {
                    company
                        .company
                        .as_ref()
                        .is_some_and(|company| existing.companies.contains(company))
                })
            };

            if overlap {
                return Err(FiscalYearError::OverlappingFiscalYear {
                    fiscal_year: existing.name.clone(),
                });
            }
        }
        Ok(())
    }
}

impl DocumentController for FiscalYear {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_update", "on_trash"]
    }
}

pub fn auto_create_fiscal_year_plan(
    current_fy: &FiscalYear,
) -> Result<FiscalYear, FiscalYearError> {
    let current_end = SimpleDate::parse(&current_fy.year_end_date)?;
    let start = current_end.add_days(1);
    let end = current_end.add_years(1);
    let start_year = start.year.to_string();
    let end_year = end.year.to_string();
    let year = if start_year == end_year {
        start_year
    } else {
        format!("{start_year}-{end_year}")
    };

    Ok(FiscalYear {
        year,
        disabled: current_fy.disabled,
        year_start_date: start.to_string(),
        year_end_date: end.to_string(),
        companies: current_fy.companies.clone(),
        auto_created: true,
        ..FiscalYear::default()
    })
}

pub fn get_from_and_to_date(from_date: &str, to_date: &str) -> (String, String) {
    (from_date.to_string(), to_date.to_string())
}

impl SimpleDate {
    fn parse(value: &str) -> Result<Self, FiscalYearError> {
        let mut parts = value.split('-');
        let year = parts
            .next()
            .and_then(|part| part.parse::<i32>().ok())
            .ok_or(FiscalYearError::InvalidDate)?;
        let month = parts
            .next()
            .and_then(|part| part.parse::<u32>().ok())
            .ok_or(FiscalYearError::InvalidDate)?;
        let day = parts
            .next()
            .and_then(|part| part.parse::<u32>().ok())
            .ok_or(FiscalYearError::InvalidDate)?;
        if parts.next().is_some()
            || month == 0
            || month > 12
            || day == 0
            || day > days_in_month(year, month)
        {
            return Err(FiscalYearError::InvalidDate);
        }
        Ok(Self { year, month, day })
    }

    fn add_years(self, years: i32) -> Self {
        let year = self.year + years;
        let day = self.day.min(days_in_month(year, self.month));
        Self {
            year,
            month: self.month,
            day,
        }
    }

    fn add_days(self, days: i32) -> Self {
        let mut serial = self.to_serial() + i64::from(days);
        let mut year = 1970;
        while serial < 0 {
            year -= 1;
            serial += i64::from(days_in_year(year));
        }
        loop {
            let len = i64::from(days_in_year(year));
            if serial < len {
                break;
            }
            serial -= len;
            year += 1;
        }
        let mut month = 1;
        loop {
            let len = i64::from(days_in_month(year, month));
            if serial < len {
                break;
            }
            serial -= len;
            month += 1;
        }
        Self {
            year,
            month,
            day: (serial + 1) as u32,
        }
    }

    fn to_serial(self) -> i64 {
        let mut days = 0_i64;
        if self.year >= 1970 {
            for year in 1970..self.year {
                days += i64::from(days_in_year(year));
            }
        } else {
            for year in self.year..1970 {
                days -= i64::from(days_in_year(year));
            }
        }
        for month in 1..self.month {
            days += i64::from(days_in_month(self.year, month));
        }
        days + i64::from(self.day) - 1
    }
}

impl std::fmt::Display for SimpleDate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}

impl PartialOrd for SimpleDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SimpleDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
