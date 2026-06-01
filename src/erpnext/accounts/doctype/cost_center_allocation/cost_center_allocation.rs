use std::collections::HashSet;

use crate::erpnext::accounts::doctype::cost_center_allocation_percentage::cost_center_allocation_percentage::CostCenterAllocationPercentage;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CostCenterAllocation {
    pub name: Option<String>,
    pub main_cost_center: String,
    pub company: String,
    pub valid_from: String,
    pub allocation_percentages: Vec<CostCenterAllocationPercentage>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostCenterAllocationValidationContext {
    pub last_gle_date: Option<String>,
    pub future_allocation: Option<FutureAllocation>,
    pub main_cost_center_used_as_child_parent: Option<String>,
    pub active_main_cost_centers: HashSet<String>,
    pub skip_from_date_validation: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FutureAllocation {
    pub name: String,
    pub valid_from: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostCenterAllocationWarning {
    pub allocation_name: String,
    pub valid_from: String,
    pub applicable_upto: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CostCenterAllocationError {
    WrongPercentageAllocation,
    InvalidDate {
        last_gle_date: String,
        main_cost_center: String,
    },
    MainCostCenterCantBeChild {
        main_cost_center: String,
    },
    InvalidMainCostCenter {
        main_cost_center: String,
        parent: String,
    },
    InvalidChildCostCenter {
        cost_center: String,
    },
}

impl CostCenterAllocation {
    pub const DOCTYPE: &'static str = "Cost Center Allocation";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "CC-ALLOC-.#####";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "main_cost_center",
        "company",
        "column_break_2",
        "valid_from",
        "section_break_5",
        "allocation_percentages",
        "amended_from",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("main_cost_center", "Main Cost Center")
                .options("Cost Center")
                .in_list_view()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .fetch_from("main_cost_center.company")
                .required(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::date("valid_from", "Valid From")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table(
                "allocation_percentages",
                "Cost Center Allocation Percentages",
            )
            .options("Cost Center Allocation Percentage")
            .required(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Cost Center Allocation")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(
        &self,
        ctx: &CostCenterAllocationValidationContext,
    ) -> Result<Option<CostCenterAllocationWarning>, CostCenterAllocationError> {
        self.validate_total_allocation_percentage()?;
        if !ctx.skip_from_date_validation {
            self.validate_from_date_based_on_existing_gle(ctx)?;
        }
        let warning = self.validate_backdated_allocation(ctx);
        self.validate_main_cost_center(ctx)?;
        self.validate_child_cost_centers(ctx)?;
        Ok(warning)
    }

    pub fn validate_total_allocation_percentage(&self) -> Result<(), CostCenterAllocationError> {
        let total_percentage: f64 = self
            .allocation_percentages
            .iter()
            .map(|row| row.percentage.unwrap_or(0.0))
            .sum();
        if total_percentage != 100.0 {
            Err(CostCenterAllocationError::WrongPercentageAllocation)
        } else {
            Ok(())
        }
    }

    pub fn validate_from_date_based_on_existing_gle(
        &self,
        ctx: &CostCenterAllocationValidationContext,
    ) -> Result<(), CostCenterAllocationError> {
        if let Some(last_gle_date) = &ctx.last_gle_date {
            if date_to_ordinal(&self.valid_from) <= date_to_ordinal(last_gle_date) {
                return Err(CostCenterAllocationError::InvalidDate {
                    last_gle_date: last_gle_date.clone(),
                    main_cost_center: self.main_cost_center.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_backdated_allocation(
        &self,
        ctx: &CostCenterAllocationValidationContext,
    ) -> Option<CostCenterAllocationWarning> {
        ctx.future_allocation
            .as_ref()
            .map(|future| CostCenterAllocationWarning {
                allocation_name: future.name.clone(),
                valid_from: future.valid_from.clone(),
                applicable_upto: add_days(&future.valid_from, -1),
            })
    }

    pub fn validate_main_cost_center(
        &self,
        ctx: &CostCenterAllocationValidationContext,
    ) -> Result<(), CostCenterAllocationError> {
        if self
            .allocation_percentages
            .iter()
            .any(|row| row.cost_center.as_deref() == Some(self.main_cost_center.as_str()))
        {
            return Err(CostCenterAllocationError::MainCostCenterCantBeChild {
                main_cost_center: self.main_cost_center.clone(),
            });
        }

        if let Some(parent) = &ctx.main_cost_center_used_as_child_parent {
            return Err(CostCenterAllocationError::InvalidMainCostCenter {
                main_cost_center: self.main_cost_center.clone(),
                parent: parent.clone(),
            });
        }
        Ok(())
    }

    pub fn validate_child_cost_centers(
        &self,
        ctx: &CostCenterAllocationValidationContext,
    ) -> Result<(), CostCenterAllocationError> {
        for row in &self.allocation_percentages {
            if let Some(cost_center) = &row.cost_center {
                if ctx.active_main_cost_centers.contains(cost_center) {
                    return Err(CostCenterAllocationError::InvalidChildCostCenter {
                        cost_center: cost_center.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl DocumentController for CostCenterAllocation {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "clear_cache"]
    }
}

pub fn add_days(date: &str, days: i32) -> String {
    SimpleDate::parse(date).add_days(days).to_string()
}

fn date_to_ordinal(date: &str) -> i32 {
    let date = SimpleDate::parse(date);
    days_from_civil(date.year, date.month, date.day)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SimpleDate {
    year: i32,
    month: u8,
    day: u8,
}

impl SimpleDate {
    fn parse(value: &str) -> Self {
        let mut parts = value.split('-');
        let year = parts
            .next()
            .and_then(|part| part.parse::<i32>().ok())
            .expect("invalid year");
        let month = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .expect("invalid month");
        let day = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .expect("invalid day");
        Self { year, month, day }
    }

    fn add_days(self, days: i32) -> Self {
        civil_from_days(days_from_civil(self.year, self.month, self.day) + days)
    }
}

impl std::fmt::Display for SimpleDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn days_from_civil(year: i32, month: u8, day: u8) -> i32 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = i32::from(month);
    let day = i32::from(day);
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe
}

fn civil_from_days(days: i32) -> SimpleDate {
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };

    SimpleDate {
        year,
        month: month as u8,
        day: day as u8,
    }
}
