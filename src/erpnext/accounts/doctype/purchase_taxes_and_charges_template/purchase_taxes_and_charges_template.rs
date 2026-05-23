use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PurchaseTaxesAndChargesTemplate {
    pub name: Option<String>,
    pub title: String,
    pub company: String,
    pub is_default: bool,
    pub disabled: bool,
    pub tax_category: Option<String>,
}

impl PurchaseTaxesAndChargesTemplate {
    pub const DOCTYPE: &'static str = "Purchase Taxes and Charges Template";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 8] = [
        "title",
        "is_default",
        "disabled",
        "column_break4",
        "company",
        "tax_category",
        "section_break6",
        "taxes",
    ];
    pub const ALLOW_RENAME: bool = true;

    pub fn new(title: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            company: company.into(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("title", "Title").required().no_copy(),
            FieldSpec::check("is_default", "Default")
                .default("0")
                .in_list_view(),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .in_list_view(),
            FieldSpec::column_break("column_break4"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("tax_category", "Tax Category").options("Tax Category"),
            FieldSpec::section_break("section_break6"),
            FieldSpec::table("taxes", "Purchase Taxes and Charges")
                .options("Purchase Taxes and Charges"),
        ]
    }

    pub fn validate_delegate(&self) -> &'static str {
        "valdiate_taxes_and_charges_template"
    }

    pub fn autoname(&mut self, company_abbr: Option<&str>) {
        if !self.company.is_empty() && !self.title.is_empty() {
            if let Some(abbr) = company_abbr {
                self.name = Some(format!("{} - {}", self.title, abbr));
            }
        }
    }
}

impl DocumentController for PurchaseTaxesAndChargesTemplate {
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
