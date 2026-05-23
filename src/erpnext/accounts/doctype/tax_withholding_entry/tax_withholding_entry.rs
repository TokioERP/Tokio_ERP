use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaxWithholdingEntry {
    pub idx: usize,
    pub docstatus: i32,
    pub parent: String,
    pub parenttype: String,
    pub company: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub tax_id: Option<String>,
    pub tax_withholding_category: Option<String>,
    pub tax_withholding_group: Option<String>,
    pub tax_rate: f64,
    pub taxable_amount: f64,
    pub withholding_amount: f64,
    pub taxable_doctype: Option<String>,
    pub taxable_name: Option<String>,
    pub taxable_date: Option<String>,
    pub currency: Option<String>,
    pub conversion_rate: f64,
    pub under_withheld_reason: Option<String>,
    pub lower_deduction_certificate: Option<String>,
    pub withholding_doctype: Option<String>,
    pub withholding_name: Option<String>,
    pub withholding_date: Option<String>,
    pub status: Option<String>,
    pub created_by_migration: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaxWithholdingUpdateValues {
    pub taxable_amount: Option<f64>,
    pub withholding_amount: Option<f64>,
    pub taxable_doctype: Option<String>,
    pub taxable_name: Option<String>,
    pub taxable_date: Option<String>,
    pub withholding_doctype: Option<String>,
    pub withholding_name: Option<String>,
    pub withholding_date: Option<String>,
    pub tax_rate: Option<f64>,
    pub status: Option<String>,
    pub currency: Option<String>,
    pub conversion_rate: Option<f64>,
    pub under_withheld_reason: Option<String>,
    pub lower_deduction_certificate: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaxWithholdingEntryError {
    DifferentTaxableAndWithholdingLinks(String),
    MismatchedWithholdingAmount(String),
}

impl TaxWithholdingEntry {
    pub const DOCTYPE: &'static str = "Tax Withholding Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const ALLOW_RENAME: bool = true;
    pub const IS_TABLE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const FIELD_ORDER: [&'static str; 29] = [
        "section_break_krko",
        "company",
        "party_type",
        "party",
        "tax_id",
        "column_break_egzm",
        "tax_withholding_category",
        "tax_withholding_group",
        "taxable_amount",
        "tax_rate",
        "withholding_amount",
        "target_section",
        "taxable_doctype",
        "taxable_name",
        "taxable_date",
        "currency",
        "conversion_rate",
        "column_break_fqoe",
        "under_withheld_reason",
        "lower_deduction_certificate",
        "source_section",
        "withholding_doctype",
        "withholding_name",
        "withholding_date",
        "column_break_dahw",
        "section_break_ggna",
        "status",
        "column_break_jfjf",
        "created_by_migration",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "company" => FieldSpec::link("company", "Company").options("Company"),
            "party_type" => FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .read_only(),
            "party" => FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .read_only(),
            "tax_id" => FieldSpec::data("tax_id", "Tax ID").read_only(),
            "tax_withholding_category" => {
                FieldSpec::link("tax_withholding_category", "Tax Withholding Category")
                    .options("Tax Withholding Category")
                    .read_only()
                    .in_list_view()
            }
            "tax_withholding_group" => {
                FieldSpec::link("tax_withholding_group", "Tax Withholding Group")
                    .options("Tax Withholding Group")
                    .read_only()
                    .in_list_view()
                    .columns(1)
            }
            "taxable_amount" => FieldSpec::currency("taxable_amount", "Base Taxable Amount")
                .options("Company:company:default_currency")
                .in_list_view()
                .columns(1),
            "tax_rate" => FieldSpec::percent("tax_rate", "Tax Rate")
                .in_list_view()
                .columns(1),
            "withholding_amount" => FieldSpec::currency("withholding_amount", "Base Tax Withheld")
                .options("Company:company:default_currency")
                .read_only()
                .in_list_view()
                .columns(1),
            "target_section" => FieldSpec::section_break("target_section")
                .label("Applicable For")
                .description("Transaction for which tax is withheld"),
            "taxable_doctype" => {
                FieldSpec::link("taxable_doctype", "Taxable Document Type").options("DocType")
            }
            "taxable_name" => FieldSpec::dynamic_link("taxable_name")
                .label("Taxable Document Name")
                .options("taxable_doctype")
                .in_list_view(),
            "taxable_date" => FieldSpec::date("taxable_date", "Taxable Date").read_only(),
            "currency" => FieldSpec::link("currency", "Currency")
                .options("Currency")
                .read_only(),
            "conversion_rate" => FieldSpec::float("conversion_rate", "Exchange Rate")
                .precision("9")
                .read_only(),
            "under_withheld_reason" => {
                FieldSpec::select("under_withheld_reason", "Under Withheld Reason")
                    .options("\nThreshold Exemption\nLower Deduction Certificate")
                    .read_only()
            }
            "lower_deduction_certificate" => {
                FieldSpec::link("lower_deduction_certificate", "Lower Deduction Certificate")
                    .options("Lower Deduction Certificate")
                    .read_only()
            }
            "source_section" => FieldSpec::section_break("source_section")
                .label("Deducted From")
                .description("Transaction from which tax is withheld"),
            "withholding_doctype" => {
                FieldSpec::link("withholding_doctype", "Withholding Document Type")
                    .options("DocType")
            }
            "withholding_name" => FieldSpec::dynamic_link("withholding_name")
                .label("Withholding Document Name")
                .options("withholding_doctype"),
            "withholding_date" => {
                FieldSpec::date("withholding_date", "Withholding Date").read_only()
            }
            "status" => FieldSpec::select("status", "Status")
                .options("\nSettled\nUnder Withheld\nOver Withheld\nDuplicate\nCancelled")
                .read_only(),
            "created_by_migration" => {
                FieldSpec::check("created_by_migration", "Created By Migration")
                    .default("0")
                    .hidden()
                    .read_only()
            }
            "section_break_krko" | "section_break_ggna" => FieldSpec::section_break(fieldname),
            "column_break_egzm" | "column_break_fqoe" | "column_break_dahw"
            | "column_break_jfjf" => FieldSpec::column_break(fieldname),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }

    pub fn set_status(&mut self, status: Option<&str>) {
        self.status = Some(status.unwrap_or_else(|| self.get_status()).to_string());
    }

    pub fn get_status(&self) -> &'static str {
        if self.docstatus == 2 {
            "Cancelled"
        } else if !has_value(&self.withholding_name) && !has_value(&self.under_withheld_reason) {
            "Under Withheld"
        } else if !has_value(&self.taxable_name) {
            "Over Withheld"
        } else {
            "Settled"
        }
    }

    pub fn validate_adjustments(&self) -> Result<(), TaxWithholdingEntryError> {
        if self.is_taxable_different() && self.is_withholding_different() {
            return Err(TaxWithholdingEntryError::DifferentTaxableAndWithholdingLinks(
                format!(
                    "Row #{}: Cannot create entry with different taxable AND withholding document links.",
                    self.idx
                ),
            ));
        }
        Ok(())
    }

    pub fn validate_tax_withheld_amount(
        &self,
        precision: u32,
    ) -> Result<(), TaxWithholdingEntryError> {
        if !has_value(&self.withholding_name) || has_value(&self.under_withheld_reason) {
            return Ok(());
        }

        let tax_to_withheld =
            round_to_precision(self.taxable_amount * (self.tax_rate / 100.0), precision);
        let diff = (tax_to_withheld - self.withholding_amount).abs();
        if diff > 0.5 {
            return Err(TaxWithholdingEntryError::MismatchedWithholdingAmount(
                format!(
                    "Row #{}: Withholding Amount {} does not match calculated amount {}.",
                    self.idx,
                    format_number(self.withholding_amount),
                    format_number(tax_to_withheld)
                ),
            ));
        }
        Ok(())
    }

    pub fn is_taxable_different(&self) -> bool {
        self.taxable_doctype.as_deref().unwrap_or("") != self.parenttype
            || self.taxable_name.as_deref().unwrap_or("") != self.parent
    }

    pub fn is_withholding_different(&self) -> bool {
        self.withholding_doctype.as_deref().unwrap_or("") != self.parenttype
            || self.withholding_name.as_deref().unwrap_or("") != self.parent
    }

    pub fn values_to_update(
        &self,
        proportion: f64,
        field_type: &str,
    ) -> TaxWithholdingUpdateValues {
        if field_type == "taxable" {
            TaxWithholdingUpdateValues {
                withholding_amount: Some(self.withholding_amount * proportion),
                withholding_doctype: self.withholding_doctype.clone(),
                withholding_name: self.withholding_name.clone(),
                withholding_date: self.withholding_date.clone(),
                tax_rate: Some(self.tax_rate),
                status: Some("Duplicate".to_string()),
                under_withheld_reason: None,
                ..TaxWithholdingUpdateValues::default()
            }
        } else {
            TaxWithholdingUpdateValues {
                taxable_amount: Some(self.taxable_amount * proportion),
                taxable_doctype: self.taxable_doctype.clone(),
                taxable_name: self.taxable_name.clone(),
                taxable_date: self.taxable_date.clone(),
                tax_rate: Some(self.tax_rate),
                status: Some("Duplicate".to_string()),
                currency: self.currency.clone(),
                conversion_rate: Some(self.conversion_rate),
                under_withheld_reason: self.under_withheld_reason.clone(),
                lower_deduction_certificate: self.lower_deduction_certificate.clone(),
                ..TaxWithholdingUpdateValues::default()
            }
        }
    }

    pub fn balance_values_to_update(
        &self,
        proportion: f64,
        field_type: &str,
        precision: u32,
    ) -> TaxWithholdingUpdateValues {
        let balance_proportion = 1.0 - proportion;
        if field_type == "taxable" {
            TaxWithholdingUpdateValues {
                withholding_amount: Some(round_to_precision(
                    self.withholding_amount * balance_proportion,
                    precision,
                )),
                ..TaxWithholdingUpdateValues::default()
            }
        } else {
            TaxWithholdingUpdateValues {
                taxable_amount: Some(round_to_precision(
                    self.taxable_amount * balance_proportion,
                    precision,
                )),
                ..TaxWithholdingUpdateValues::default()
            }
        }
    }
}

impl DocumentController for TaxWithholdingEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn compute_withheld_amount(
    taxable_amount: f64,
    tax_rate: f64,
    round_off_tax_amount: bool,
    precision: u32,
) -> f64 {
    let amount = taxable_amount * tax_rate / 100.0;
    if round_off_tax_amount {
        round_to_precision(amount, 0)
    } else {
        round_to_precision(amount, precision)
    }
}

fn has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        value.to_string()
    }
}
