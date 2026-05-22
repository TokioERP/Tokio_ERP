use std::collections::HashSet;

use crate::erpnext::accounts::doctype::payment_terms_template_detail::payment_terms_template_detail::PaymentTermsTemplateDetail;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    pub message: String,
    pub indicator: Option<&'static str>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentTermsTemplate {
    pub template_name: Option<String>,
    pub allocate_payment_based_on_payment_terms: bool,
    pub terms: Vec<PaymentTermsTemplateDetail>,
}

impl PaymentTermsTemplate {
    pub const DOCTYPE: &'static str = "Payment Terms Template";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: Option<&'static str> = Some("field:template_name");
    pub const FIELD_ORDER: [&'static str; 3] = [
        "template_name",
        "allocate_payment_based_on_payment_terms",
        "terms",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("template_name", "Template Name").unique(),
            FieldSpec::check(
                "allocate_payment_based_on_payment_terms",
                "Allocate Payment Based On Payment Terms",
            )
            .default("0"),
            FieldSpec::table("terms", "Payment Terms")
                .options("Payment Terms Template Detail")
                .required(),
        ]
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.validate_invoice_portion()?;
        self.validate_terms()
    }

    pub fn validate_invoice_portion(&self) -> Result<(), ValidationError> {
        let total_portion: f64 = self
            .terms
            .iter()
            .map(|term| flt(term.invoice_portion.as_deref().unwrap_or("0")))
            .sum();

        if round_to_precision(total_portion, 2) != 100.00 {
            return Err(ValidationError {
                message: "Combined invoice portion must equal 100%".to_string(),
                indicator: Some("red"),
            });
        }

        Ok(())
    }

    pub fn validate_terms(&self) -> Result<(), ValidationError> {
        let mut terms = HashSet::new();

        for term in &self.terms {
            if self.allocate_payment_based_on_payment_terms
                && term.payment_term.as_deref().unwrap_or("").is_empty()
            {
                return Err(ValidationError {
                    message: format!("Row {}: Payment Term is mandatory", term.idx),
                    indicator: None,
                });
            }

            let term_info = (
                term.payment_term.as_deref(),
                term.credit_days.as_deref(),
                term.credit_months.as_deref(),
                term.due_date_based_on.as_deref(),
            );

            if !terms.insert(term_info) {
                return Err(ValidationError {
                    message: format!(
                        "The Payment Term at row {} is possibly a duplicate.",
                        term.idx
                    ),
                    indicator: Some("red"),
                });
            }
        }

        Ok(())
    }
}

impl DocumentController for PaymentTermsTemplate {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

fn flt(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
