use std::collections::{HashMap, HashSet};

use crate::erpnext::accounts::doctype::allowed_dimension::allowed_dimension::AllowedDimension;
use crate::erpnext::accounts::doctype::applicable_on_account::applicable_on_account::ApplicableOnAccount;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimensionFilter {
    pub name: Option<String>,
    pub accounting_dimension: Option<String>,
    pub fieldname: Option<String>,
    pub disabled: bool,
    pub company: Option<String>,
    pub apply_restriction_on_values: bool,
    pub allow_or_restrict: String,
    pub accounts: Vec<ApplicableOnAccount>,
    pub dimensions: Vec<AllowedDimension>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountingDimensionFilterError {
    DuplicateApplicableAccount {
        row: usize,
        account: String,
        accounting_dimension: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionFilterRow {
    pub applicable_on_account: String,
    pub dimension_value: Option<String>,
    pub accounting_dimension: String,
    pub allow_or_restrict: String,
    pub fieldname: String,
    pub is_mandatory: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionFilterInfo {
    pub allowed_dimensions: Vec<String>,
    pub is_mandatory: bool,
    pub allow_or_restrict: String,
}

impl AccountingDimensionFilter {
    pub const DOCTYPE: &'static str = "Accounting Dimension Filter";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 13] = [
        "accounting_dimension",
        "fieldname",
        "disabled",
        "column_break_2",
        "company",
        "apply_restriction_on_values",
        "allow_or_restrict",
        "section_break_4",
        "accounts",
        "column_break_6",
        "dimensions",
        "section_break_10",
        "dimension_filter_help",
    ];
    pub const AUTONAME: &'static str = "format:{accounting_dimension}-{#####}";
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("accounting_dimension", "Accounting Dimension")
                .in_list_view()
                .required(),
            FieldSpec::data("fieldname", "Fieldname").hidden(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::check(
                "apply_restriction_on_values",
                "Apply restriction on dimension values",
            )
            .default("1"),
            FieldSpec::select("allow_or_restrict", "Allow Or Restrict Dimension")
                .options("Allow\nRestrict")
                .depends_on("eval:doc.apply_restriction_on_values == 1;")
                .required(),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::table("accounts", "Applicable On Account")
                .options("Applicable On Account")
                .required(),
            FieldSpec::column_break("column_break_6"),
            FieldSpec::table("dimensions", "Applicable Dimension")
                .options("Allowed Dimension")
                .depends_on("eval:doc.accounting_dimension && doc.apply_restriction_on_values")
                .mandatory_depends_on("eval:doc.apply_restriction_on_values == 1;"),
            FieldSpec::section_break("section_break_10"),
            FieldSpec::html("dimension_filter_help", "Dimension Filter Help"),
        ]
    }

    pub fn before_save(&mut self) {
        if !self.apply_restriction_on_values {
            self.allow_or_restrict = "Restrict".to_string();
            self.dimensions.clear();
        }
    }

    pub fn validate(
        &mut self,
        accounting_dimension_fieldname: Option<&str>,
        existing_accounts: &[String],
    ) -> Result<(), AccountingDimensionFilterError> {
        self.fieldname = accounting_dimension_fieldname
            .map(str::to_string)
            .or_else(|| self.accounting_dimension.as_deref().map(scrub));
        self.validate_applicable_accounts(existing_accounts)
    }

    pub fn validate_applicable_accounts(
        &self,
        existing_accounts: &[String],
    ) -> Result<(), AccountingDimensionFilterError> {
        let existing: HashSet<&str> = existing_accounts.iter().map(String::as_str).collect();
        for (idx, account) in self.accounts.iter().enumerate() {
            if let Some(applicable_on_account) = account.applicable_on_account.as_deref() {
                if existing.contains(applicable_on_account) {
                    return Err(AccountingDimensionFilterError::DuplicateApplicableAccount {
                        row: idx + 1,
                        account: applicable_on_account.to_string(),
                        accounting_dimension: self.accounting_dimension.clone().unwrap_or_default(),
                    });
                }
            }
        }
        Ok(())
    }
}

impl DocumentController for AccountingDimensionFilter {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["before_save", "validate"]
    }
}

pub fn build_dimension_filter_map(
    filters: &[DimensionFilterRow],
) -> HashMap<(String, String), DimensionFilterInfo> {
    let mut dimension_filter_map = HashMap::new();
    for filter in filters {
        build_map(
            &mut dimension_filter_map,
            &filter.fieldname,
            &filter.applicable_on_account,
            filter.dimension_value.as_deref(),
            &filter.allow_or_restrict,
            filter.is_mandatory,
        );
    }
    dimension_filter_map
}

pub fn build_map(
    map_object: &mut HashMap<(String, String), DimensionFilterInfo>,
    dimension: &str,
    account: &str,
    filter_value: Option<&str>,
    allow_or_restrict: &str,
    is_mandatory: bool,
) {
    let entry = map_object
        .entry((dimension.to_string(), account.to_string()))
        .or_insert_with(|| DimensionFilterInfo {
            allowed_dimensions: Vec::new(),
            is_mandatory,
            allow_or_restrict: allow_or_restrict.to_string(),
        });
    if let Some(filter_value) = filter_value {
        entry.allowed_dimensions.push(filter_value.to_string());
    }
}

fn scrub(value: &str) -> String {
    let mut scrubbed = String::new();
    let mut last_was_separator = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            scrubbed.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && !scrubbed.is_empty() {
            scrubbed.push('_');
            last_was_separator = true;
        }
    }
    while scrubbed.ends_with('_') {
        scrubbed.pop();
    }
    scrubbed
}
