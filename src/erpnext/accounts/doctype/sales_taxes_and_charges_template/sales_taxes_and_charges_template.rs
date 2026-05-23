use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesTaxesAndChargesTemplate {
    pub name: Option<String>,
    pub title: String,
    pub company: String,
    pub is_default: bool,
    pub disabled: bool,
    pub tax_category: Option<String>,
    pub taxes: Vec<SalesTaxRow>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesTaxRow {
    pub charge_type: String,
    pub rate: Option<String>,
    pub account_head: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxCategoryTemplate {
    pub name: String,
    pub company: String,
    pub tax_category: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateDashboard {
    pub fieldname: &'static str,
    pub non_standard_fieldnames: Vec<(&'static str, &'static str)>,
    pub transactions: Vec<(&'static str, Vec<&'static str>)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SalesTaxesTemplateJsHooks {
    pub tax_table: &'static str,
    pub tax_validations_doctype: &'static str,
    pub tax_filters_doctype: &'static str,
}

impl TaxCategoryTemplate {
    pub fn new(
        name: impl Into<String>,
        company: impl Into<String>,
        tax_category: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            company: company.into(),
            tax_category: tax_category.into(),
            disabled: false,
        }
    }
}

impl SalesTaxesAndChargesTemplate {
    pub const DOCTYPE: &'static str = "Sales Taxes and Charges Template";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 8] = [
        "title",
        "is_default",
        "disabled",
        "column_break_3",
        "company",
        "tax_category",
        "section_break_5",
        "taxes",
    ];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const SHOW_TITLE_FIELD_IN_LINK: bool = true;
    pub const TITLE_FIELD: &'static str = "title";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(title: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            company: company.into(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("title", "Title")
                .required()
                .no_copy()
                .oldfield("title", "Data"),
            FieldSpec::check("is_default", "Default")
                .default("0")
                .in_list_view(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view()
                .in_standard_filter()
                .oldfield("company", "Link"),
            FieldSpec::link("tax_category", "Tax Category").options("Tax Category"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("taxes", "Sales Taxes and Charges")
                .options("Sales Taxes and Charges")
                .description("* Will be calculated in the transaction.")
                .oldfield("other_charges", "Table"),
        ]
    }

    pub fn validate(&self, existing_templates: &[TaxCategoryTemplate]) -> Result<(), String> {
        validate_disabled(self)?;
        validate_for_tax_category(self, existing_templates)
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

    pub fn set_missing_values(&mut self, account_tax_rates: &[(String, String)]) {
        for tax in &mut self.taxes {
            if tax.charge_type == "On Net Total"
                && tax
                    .rate
                    .as_deref()
                    .unwrap_or_default()
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    == 0.0
            {
                if let Some((_, rate)) = account_tax_rates
                    .iter()
                    .find(|(account_head, _)| account_head == &tax.account_head)
                {
                    tax.rate = Some(rate.clone());
                }
            }
        }
    }
}

impl DocumentController for SalesTaxesAndChargesTemplate {
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

pub fn validate_disabled(doc: &SalesTaxesAndChargesTemplate) -> Result<(), String> {
    if doc.is_default && doc.disabled {
        Err("Disabled template must not be default template".to_string())
    } else {
        Ok(())
    }
}

pub fn validate_for_tax_category(
    doc: &SalesTaxesAndChargesTemplate,
    existing_templates: &[TaxCategoryTemplate],
) -> Result<(), String> {
    let Some(tax_category) = doc.tax_category.as_deref() else {
        return Ok(());
    };

    let duplicate = existing_templates.iter().any(|existing| {
        existing.company == doc.company
            && existing.tax_category == tax_category
            && !existing.disabled
            && Some(existing.name.as_str()) != doc.name.as_deref()
    });

    if duplicate {
        Err(format!(
            "A template with tax category <b>{tax_category}</b> already exists. Only one template is allowed with each tax category"
        ))
    } else {
        Ok(())
    }
}

pub fn sales_taxes_and_charges_template_dashboard() -> TemplateDashboard {
    TemplateDashboard {
        fieldname: "taxes_and_charges",
        non_standard_fieldnames: vec![
            ("Tax Rule", "sales_tax_template"),
            ("Subscription", "sales_tax_template"),
            ("Restaurant", "default_tax_template"),
        ],
        transactions: vec![
            (
                "Transactions",
                vec!["Sales Invoice", "Sales Order", "Delivery Note"],
            ),
            (
                "References",
                vec!["POS Profile", "Subscription", "Restaurant", "Tax Rule"],
            ),
        ],
    }
}

pub fn sales_taxes_and_charges_template_js_hooks() -> SalesTaxesTemplateJsHooks {
    SalesTaxesTemplateJsHooks {
        tax_table: "Sales Taxes and Charges",
        tax_validations_doctype: "Sales Taxes and Charges Template",
        tax_filters_doctype: "Sales Taxes and Charges",
    }
}
