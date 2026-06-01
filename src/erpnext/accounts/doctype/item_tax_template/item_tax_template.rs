use std::collections::HashSet;

use crate::erpnext::accounts::doctype::item_tax_template_detail::item_tax_template_detail::ItemTaxTemplateDetail;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemTaxTemplate {
    pub name: Option<String>,
    pub title: Option<String>,
    pub company: Option<String>,
    pub disabled: bool,
    pub taxes: Vec<ItemTaxTemplateDetail>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemTaxAccount {
    pub name: String,
    pub account_type: String,
    pub company: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemTaxTemplateError {
    AccountBelongsToDifferentCompany { row: usize, company: String },
    InvalidAccountType { row: usize },
    DuplicateTaxType { tax_type: String },
}

impl ItemTaxTemplate {
    pub const DOCTYPE: &'static str = "Item Tax Template";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "title",
        "company",
        "column_break_3",
        "disabled",
        "section_break_5",
        "taxes",
    ];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const DOCUMENT_TYPE: &'static str = "Setup";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TITLE_FIELD: &'static str = "title";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("title", "Title")
                .in_filter()
                .in_list_view()
                .no_copy()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_filter()
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("taxes", "Tax Rates")
                .options("Item Tax Template Detail")
                .required(),
        ]
    }

    pub fn autoname_with_company_abbr(&mut self, company_abbr: &str) -> Option<String> {
        let (Some(title), Some(_company)) = (&self.title, &self.company) else {
            return None;
        };
        let name = format!("{title} - {company_abbr}");
        self.name = Some(name.clone());
        Some(name)
    }

    pub fn validate(&mut self, accounts: &[ItemTaxAccount]) -> Result<(), ItemTaxTemplateError> {
        self.set_zero_rate_for_not_applicable_tax();
        self.validate_tax_accounts(accounts)
    }

    pub fn set_zero_rate_for_not_applicable_tax(&mut self) {
        for row in &mut self.taxes {
            if row.not_applicable {
                row.tax_rate = 0.0;
            }
        }
    }

    pub fn validate_tax_accounts(
        &self,
        accounts: &[ItemTaxAccount],
    ) -> Result<(), ItemTaxTemplateError> {
        let mut checked_tax_types = HashSet::new();
        for (idx, row) in self.taxes.iter().enumerate() {
            let Some(tax_type) = row.tax_type.as_deref() else {
                continue;
            };
            let Some(account) = accounts.iter().find(|account| account.name == tax_type) else {
                continue;
            };

            if Some(account.company.as_str()) != self.company.as_deref() {
                return Err(ItemTaxTemplateError::AccountBelongsToDifferentCompany {
                    row: idx + 1,
                    company: self.company.clone().unwrap_or_default(),
                });
            }

            if !is_valid_item_tax_account_type(&account.account_type) {
                return Err(ItemTaxTemplateError::InvalidAccountType { row: idx + 1 });
            }

            if !checked_tax_types.insert(tax_type) {
                return Err(ItemTaxTemplateError::DuplicateTaxType {
                    tax_type: tax_type.to_string(),
                });
            }
        }
        Ok(())
    }
}

impl DocumentController for ItemTaxTemplate {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "autoname"]
    }
}

fn is_valid_item_tax_account_type(account_type: &str) -> bool {
    matches!(
        account_type,
        "Tax"
            | "Chargeable"
            | "Income Account"
            | "Expense Account"
            | "Expenses Included In Valuation"
    )
}
