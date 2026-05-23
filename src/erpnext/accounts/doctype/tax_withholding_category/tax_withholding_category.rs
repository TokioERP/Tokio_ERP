use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::accounts::doctype::tax_withholding_account::tax_withholding_account::TaxWithholdingAccount;
use crate::erpnext::accounts::doctype::tax_withholding_rate::tax_withholding_rate::TaxWithholdingRate;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaxWithholdingCategory {
    pub name: String,
    pub category_name: Option<String>,
    pub tax_deduction_basis: String,
    pub round_off_tax_amount: bool,
    pub tax_on_excess_amount: bool,
    pub disable_cumulative_threshold: bool,
    pub disable_transaction_threshold: bool,
    pub rates: Vec<TaxWithholdingRate>,
    pub accounts: Vec<TaxWithholdingAccount>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaxWithholdingCategoryError {
    InvalidDateRange(String),
    OverlappingDates(String),
    DuplicateCompany(String),
    DuplicateAccount(String),
    InvalidThreshold(String),
    MissingTaxRow(String),
    MissingAccount(String),
}

impl TaxWithholdingCategory {
    pub const DOCTYPE: &'static str = "Tax Withholding Category";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "Prompt";
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 12] = [
        "category_details_section",
        "category_name",
        "tax_deduction_basis",
        "column_break_2",
        "round_off_tax_amount",
        "tax_on_excess_amount",
        "disable_cumulative_threshold",
        "disable_transaction_threshold",
        "section_break_8",
        "rates",
        "section_break_7",
        "accounts",
    ];

    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tax_deduction_basis: "Net Total".to_string(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "category_details_section" => {
                FieldSpec::section_break("category_details_section").label("Category Details")
            }
            "category_name" => FieldSpec::data("category_name", "Category Name").in_list_view(),
            "tax_deduction_basis" => FieldSpec::select(
                "tax_deduction_basis",
                "Deduct Tax On Basis",
            )
            .options("\nGross Total\nNet Total")
            .default("Net Total")
            .required(),
            "column_break_2" => FieldSpec::column_break("column_break_2"),
            "round_off_tax_amount" => FieldSpec::check(
                "round_off_tax_amount",
                "Round Off Tax Amount",
            )
            .default("0")
            .description("Checking this will round off the tax amount to the nearest integer"),
            "tax_on_excess_amount" => FieldSpec::check(
                "tax_on_excess_amount",
                "Only Deduct Tax On Excess Amount ",
            )
            .default("0")
            .description("Tax withheld only for amount exceeding cumulative threshold"),
            "disable_cumulative_threshold" => FieldSpec::check(
                "disable_cumulative_threshold",
                "Disable Cumulative Threshold",
            )
            .default("0")
            .description("When checked, only transaction threshold will be applied for transaction individually"),
            "disable_transaction_threshold" => FieldSpec::check(
                "disable_transaction_threshold",
                "Disable Transaction Threshold",
            )
            .default("0")
            .description("When checked, only cumulative threshold will be applied"),
            "section_break_8" => {
                FieldSpec::section_break("section_break_8").label("Tax Withholding Rates")
            }
            "rates" => FieldSpec::table("rates", "Rates")
                .options("Tax Withholding Rate")
                .required(),
            "section_break_7" => {
                FieldSpec::section_break("section_break_7").label("Account Details")
            }
            "accounts" => FieldSpec::table("accounts", "Accounts")
                .options("Tax Withholding Account")
                .required(),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }

    pub fn validate(&mut self) -> Result<(), TaxWithholdingCategoryError> {
        self.assign_rate_indexes();
        self.validate_dates()?;
        self.validate_companies_and_accounts()?;
        self.validate_thresholds()
    }

    pub fn validate_dates(&mut self) -> Result<(), TaxWithholdingCategoryError> {
        self.assign_rate_indexes();
        let mut groups: BTreeMap<String, Vec<&TaxWithholdingRate>> = BTreeMap::new();
        for rate in &self.rates {
            if rate.from_date >= rate.to_date {
                return Err(TaxWithholdingCategoryError::InvalidDateRange(format!(
                    "Row #{}: From Date cannot be before To Date",
                    rate.idx
                )));
            }
            groups
                .entry(rate.tax_withholding_group.clone().unwrap_or_default())
                .or_default()
                .push(rate);
        }

        for (group, mut rates) in groups {
            rates.sort_by_key(|rate| rate.from_date.as_str());
            let mut last_to_date: Option<&str> = None;
            for rate in rates {
                if last_to_date.is_some_and(|to_date| rate.from_date.as_str() < to_date) {
                    return Err(TaxWithholdingCategoryError::OverlappingDates(format!(
                        "Row #{}: Dates overlapping with other row in group {}",
                        rate.idx,
                        if group.is_empty() { "Default" } else { &group }
                    )));
                }
                last_to_date = Some(rate.to_date.as_str());
            }
        }
        Ok(())
    }

    pub fn validate_companies_and_accounts(&self) -> Result<(), TaxWithholdingCategoryError> {
        let mut companies = BTreeSet::new();
        let mut accounts = BTreeSet::new();
        for row in &self.accounts {
            if let Some(company) = row.company.as_deref().filter(|value| !value.is_empty()) {
                if !companies.insert(company.to_string()) {
                    return Err(TaxWithholdingCategoryError::DuplicateCompany(format!(
                        "Company {} added multiple times",
                        company
                    )));
                }
            }

            if let Some(account) = row.account.as_deref().filter(|value| !value.is_empty()) {
                if !accounts.insert(account.to_string()) {
                    return Err(TaxWithholdingCategoryError::DuplicateAccount(format!(
                        "Account {} added multiple times",
                        account
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn validate_thresholds(&mut self) -> Result<(), TaxWithholdingCategoryError> {
        self.assign_rate_indexes();
        for rate in &self.rates {
            if rate.cumulative_threshold != 0.0
                && rate.single_threshold != 0.0
                && rate.cumulative_threshold < rate.single_threshold
            {
                return Err(TaxWithholdingCategoryError::InvalidThreshold(format!(
                    "Row #{}: Cumulative threshold cannot be less than Single Transaction threshold",
                    rate.idx
                )));
            }
        }
        Ok(())
    }

    pub fn get_applicable_tax_row(
        &self,
        posting_date: &str,
        tax_withholding_group: &str,
    ) -> Result<&TaxWithholdingRate, TaxWithholdingCategoryError> {
        self.rates
            .iter()
            .find(|row| {
                row.from_date.as_str() <= posting_date
                    && posting_date <= row.to_date.as_str()
                    && row.tax_withholding_group.as_deref().unwrap_or("") == tax_withholding_group
            })
            .ok_or_else(|| {
                TaxWithholdingCategoryError::MissingTaxRow(
                    "No Tax Withholding data found for the current posting date.".to_string(),
                )
            })
    }

    pub fn get_company_account(
        &self,
        company: &str,
    ) -> Result<String, TaxWithholdingCategoryError> {
        self.accounts
            .iter()
            .find(|row| row.company.as_deref() == Some(company))
            .and_then(|row| row.account.clone())
            .ok_or_else(|| {
                TaxWithholdingCategoryError::MissingAccount(format!(
                    "No Tax withholding account set for Company {} in Tax Withholding Category {}.",
                    company, self.name
                ))
            })
    }

    fn assign_rate_indexes(&mut self) {
        for (index, rate) in self.rates.iter_mut().enumerate() {
            if rate.idx == 0 {
                rate.idx = index + 1;
            }
        }
    }
}

impl DocumentController for TaxWithholdingCategory {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

pub fn tax_withholding_category_dashboard_items() -> [&'static str; 1] {
    ["Supplier"]
}
