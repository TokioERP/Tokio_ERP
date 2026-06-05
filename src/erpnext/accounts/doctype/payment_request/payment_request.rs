use std::collections::BTreeMap;

use serde_json::Value;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentRequest {
    pub name: Option<String>,
    pub company: Option<String>,
    pub currency: Option<String>,
    pub party_account_currency: Option<String>,
    pub grand_total: f64,
    pub outstanding_amount: f64,
    pub payment_request_type: String,
    pub payment_channel: String,
    pub status: String,
    pub reference_doctype: Option<String>,
    pub reference_name: Option<String>,
    pub payment_reference: Vec<PaymentReferenceRow>,
    pub payment_account: Option<String>,
    pub payment_gateway: Option<String>,
    pub mute_email: bool,
    pub flags_mute_email: bool,
    pub make_sales_invoice: bool,
    pub payment_url: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub email_to: Option<String>,
    pub print_format: Option<String>,
    pub mode_of_payment: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReferenceRow {
    pub payment_term: Option<String>,
    pub payment_schedule: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentSubmitPlan {
    pub outstanding_amount: f64,
    pub status: String,
    pub request_phone_payment: bool,
    pub set_payment_request_url: bool,
    pub send_email: bool,
    pub make_communication_entry: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentRequestAmountSource {
    pub doctype: String,
    pub rounded_total: Option<f64>,
    pub grand_total: Option<f64>,
    pub advance_paid: f64,
    pub outstanding_amount: f64,
    pub party_account_currency: Option<String>,
    pub currency: Option<String>,
    pub conversion_rate: f64,
    pub payments: Vec<PaymentRequestPaymentRow>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentRequestPaymentRow {
    pub payment_type: String,
    pub account: String,
    pub amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentRequestUpdate {
    pub payment_request: String,
    pub grand_total: f64,
    pub outstanding_amount: f64,
    pub payment_request_type: String,
    pub allocated_amount: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentRequestError {
    Validation(String),
    InvalidRequestAmountJson(String),
}

impl Default for PaymentRequest {
    fn default() -> Self {
        Self {
            name: None,
            company: None,
            currency: None,
            party_account_currency: None,
            grand_total: 0.0,
            outstanding_amount: 0.0,
            payment_request_type: "Inward".to_string(),
            payment_channel: "Email".to_string(),
            status: String::new(),
            reference_doctype: None,
            reference_name: None,
            payment_reference: Vec::new(),
            payment_account: None,
            payment_gateway: None,
            mute_email: false,
            flags_mute_email: false,
            make_sales_invoice: false,
            payment_url: None,
            subject: None,
            message: None,
            email_to: None,
            print_format: None,
            mode_of_payment: None,
            cost_center: None,
            project: None,
            dimensions: BTreeMap::new(),
        }
    }
}

impl PaymentRequest {
    pub const DOCTYPE: &'static str = "Payment Request";
    pub const MODULE: &'static str = "Accounts";
    pub const ALLOWED_DOCTYPES_FOR_PAYMENT_REQUEST: [&'static str; 6] = [
        "Sales Order",
        "Purchase Order",
        "Sales Invoice",
        "Purchase Invoice",
        "POS Invoice",
        "Fees",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("naming_series", "Series").default("ACC-PRQ-.YYYY.-"),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::dynamic_link("reference_name").options("reference_doctype"),
            FieldSpec::currency("grand_total", "Grand Total"),
            FieldSpec::currency("outstanding_amount", "Outstanding Amount"),
            FieldSpec::select("payment_request_type", "Payment Request Type")
                .options("\nOutward\nInward"),
            FieldSpec::select("status", "Status").options(
                "\nDraft\nRequested\nInitiated\nPartially Paid\nPayment Ordered\nPaid\nFailed\nCancelled",
            ),
            FieldSpec::table("payment_reference", "Payment Reference").options("Payment Reference"),
        ]
    }

    pub fn validate_reference_document(&self) -> Result<(), PaymentRequestError> {
        if self
            .reference_doctype
            .as_deref()
            .unwrap_or_default()
            .is_empty()
            || self
                .reference_name
                .as_deref()
                .unwrap_or_default()
                .is_empty()
        {
            return Err(PaymentRequestError::Validation(
                "To create a Payment Request reference document is required".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_against_payment_reference(
        &self,
        precision: u32,
    ) -> Result<(), PaymentRequestError> {
        if self.payment_reference.is_empty() {
            return Ok(());
        }

        let expected: f64 = self.payment_reference.iter().map(|row| row.amount).sum();
        if round_to_precision(expected, precision)
            != round_to_precision(self.grand_total, precision)
        {
            return Err(PaymentRequestError::Validation(
                "Grand Total must match sum of Payment References".to_string(),
            ));
        }

        let mut seen = Vec::new();
        for row in &self.payment_reference {
            let Some(schedule) = row.payment_schedule.as_deref() else {
                continue;
            };
            if seen.contains(&schedule) {
                return Err(PaymentRequestError::Validation(
                    "Duplicate Payment Schedule selected".to_string(),
                ));
            }
            seen.push(schedule);
        }
        Ok(())
    }

    pub fn get_request_amount(
        &self,
        completed_request_data: &[&str],
    ) -> Result<f64, PaymentRequestError> {
        if completed_request_data.is_empty() {
            return Ok(self.grand_total);
        }

        completed_request_data.iter().try_fold(0.0, |total, data| {
            let parsed: Value = serde_json::from_str(data)
                .map_err(|err| PaymentRequestError::InvalidRequestAmountJson(err.to_string()))?;
            Ok(total
                + parsed
                    .get("request_amount")
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0))
        })
    }

    pub fn before_submit_plan(
        &self,
        company_currency: &str,
        invoice_totals: Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>,
        payment_gateway_validation: bool,
    ) -> PaymentSubmitPlan {
        let outstanding_amount = if self.currency.as_deref()
            != self.party_account_currency.as_deref()
            && self.party_account_currency.as_deref() == Some(company_currency)
        {
            if let Some((rounded_total, grand_total, base_rounded_total, base_grand_total)) =
                invoice_totals
            {
                let invoice_grand_total = rounded_total.or(grand_total).unwrap_or(0.0);
                let invoice_base_grand_total =
                    base_rounded_total.or(base_grand_total).unwrap_or(0.0);
                if invoice_grand_total != 0.0 {
                    self.grand_total / invoice_grand_total * invoice_base_grand_total
                } else {
                    self.grand_total
                }
            } else {
                self.grand_total
            }
        } else {
            self.grand_total
        };

        let status = if self.payment_request_type == "Outward" {
            "Initiated"
        } else {
            "Requested"
        };
        let is_inward = self.payment_request_type == "Inward";
        let request_phone_payment = is_inward && self.payment_channel == "Phone";
        let set_payment_request_url = is_inward
            && !request_phone_payment
            && self.payment_account.is_some()
            && self.payment_gateway.is_some()
            && payment_gateway_validation;
        let send_email = set_payment_request_url && !(self.mute_email || self.flags_mute_email);

        PaymentSubmitPlan {
            outstanding_amount,
            status: status.to_string(),
            request_phone_payment,
            set_payment_request_url,
            send_email,
            make_communication_entry: send_email,
        }
    }
}

impl DocumentController for PaymentRequest {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[]
    }
}

pub fn get_amount(
    ref_doc: &PaymentRequestAmountSource,
    payment_account: Option<&str>,
    currency_precision: u32,
) -> f64 {
    let grand_total = match ref_doc.doctype.as_str() {
        "Sales Order" | "Purchase Order" => {
            let mut advance_amount = ref_doc.advance_paid;
            if ref_doc.party_account_currency != ref_doc.currency {
                advance_amount = safe_div(ref_doc.advance_paid, ref_doc.conversion_rate);
            }
            ref_doc.rounded_total.or(ref_doc.grand_total).unwrap_or(0.0) - advance_amount
        }
        "Sales Invoice" | "Purchase Invoice" => {
            let phone_amount = if ref_doc.doctype == "Sales Invoice" {
                phone_payment_amount(ref_doc, payment_account)
            } else {
                0.0
            };
            if phone_amount > 0.0 {
                phone_amount
            } else if ref_doc.party_account_currency == ref_doc.currency {
                ref_doc.outstanding_amount
            } else {
                safe_div(ref_doc.outstanding_amount, ref_doc.conversion_rate)
            }
        }
        "POS Invoice" => phone_payment_amount(ref_doc, payment_account),
        "Fees" => ref_doc.outstanding_amount,
        _ => 0.0,
    };

    if grand_total > 0.0 {
        round_to_precision(grand_total, currency_precision)
    } else {
        0.0
    }
}

pub fn update_payment_requests_as_per_pe_references(
    references: &[PaymentRequestUpdate],
    cancel: bool,
    precision: u32,
) -> Result<Vec<(String, f64, String)>, PaymentRequestError> {
    let mut state: BTreeMap<String, (f64, f64, String)> = BTreeMap::new();
    for reference in references {
        let (_, outstanding, _) = state.entry(reference.payment_request.clone()).or_insert((
            reference.grand_total,
            reference.outstanding_amount,
            reference.payment_request_type.clone(),
        ));
        let new_outstanding_amount = if cancel {
            *outstanding + reference.allocated_amount
        } else {
            *outstanding - reference.allocated_amount
        };
        let new_outstanding_amount = round_to_precision(new_outstanding_amount, precision);
        if !cancel && new_outstanding_amount < 0.0 {
            return Err(PaymentRequestError::Validation(format!(
                "The allocated amount is greater than the outstanding amount of Payment Request {}",
                reference.payment_request
            )));
        }
        *outstanding = new_outstanding_amount;
    }

    Ok(state
        .into_iter()
        .map(
            |(name, (grand_total, outstanding_amount, payment_request_type))| {
                let status = if outstanding_amount == grand_total {
                    if payment_request_type == "Outward" {
                        "Initiated"
                    } else {
                        "Requested"
                    }
                } else if outstanding_amount == 0.0 {
                    "Paid"
                } else {
                    "Partially Paid"
                };
                (name, outstanding_amount, status.to_string())
            },
        )
        .collect())
}

fn phone_payment_amount(
    ref_doc: &PaymentRequestAmountSource,
    payment_account: Option<&str>,
) -> f64 {
    ref_doc
        .payments
        .iter()
        .filter(|payment| {
            payment.payment_type == "Phone" && Some(payment.account.as_str()) == payment_account
        })
        .map(|payment| payment.amount)
        .sum()
}

fn safe_div(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
