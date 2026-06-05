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
    pub party: Option<String>,
    pub bank_account: Option<String>,
    pub account: Option<String>,
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
    pub phone_number: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentEntrySourceDoc {
    pub doctype: String,
    pub name: String,
    pub debit_to: Option<String>,
    pub credit_to: Option<String>,
    pub party_account: Option<String>,
    pub party_account_currency: Option<String>,
    pub company_currency: String,
    pub conversion_rate: f64,
    pub paid_from_account_currency: Option<String>,
    pub paid_to_account_currency: Option<String>,
    pub target_exchange_rate: f64,
    pub received_amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentEntryRequestPlan {
    pub reference_doctype: String,
    pub reference_name: String,
    pub party_account: String,
    pub party_account_currency: String,
    pub party_amount: f64,
    pub bank_account: Option<String>,
    pub bank_amount: f64,
    pub mode_of_payment: Option<String>,
    pub reference_no: String,
    pub remarks: String,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub submit: bool,
    pub created_from_payment_request: bool,
    pub paid_amount_override: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentEntryReferenceRow {
    pub idx: i32,
    pub allocated_amount: f64,
    pub payment_request: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscriptionPlanInput {
    pub name: String,
    pub plan: String,
    pub qty: f64,
    pub payment_gateway: Option<String>,
    pub rate: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubscriptionValidationPlan {
    pub calculated_amount: f64,
    pub grand_total: f64,
    pub warning: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PhonePaymentRequestPlan {
    pub reference_doctype: String,
    pub reference_docname: String,
    pub payment_reference: String,
    pub request_amount: f64,
    pub sender: Option<String>,
    pub currency: Option<String>,
    pub payment_gateway: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentUrlPlan {
    pub amount: f64,
    pub title: String,
    pub description: Option<String>,
    pub reference_doctype: String,
    pub reference_docname: String,
    pub payer_email: String,
    pub payer_name: Option<String>,
    pub order_id: String,
    pub currency: Option<String>,
    pub payment_gateway: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SendEmailPlan {
    pub recipients: Option<String>,
    pub sender: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub reference_doctype: Option<String>,
    pub reference_name: Option<String>,
    pub print_format: Option<String>,
    pub queue: String,
    pub timeout: i32,
    pub enqueue_after_commit: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentOrderFromRequestPlan {
    pub payment_order_type: String,
    pub reference_doctype: String,
    pub reference_name: String,
    pub amount: f64,
    pub supplier: Option<String>,
    pub payment_request: String,
    pub mode_of_payment: Option<String>,
    pub bank_account: Option<String>,
    pub account: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OpenPaymentRequestsQueryPlan {
    pub doctype: String,
    pub text_filter: Option<String>,
    pub searchfield: String,
    pub start: i32,
    pub page_len: i32,
    pub reference_doctype: String,
    pub reference_name: String,
    pub order_by: String,
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
            party: None,
            bank_account: None,
            account: None,
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
            phone_number: None,
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

    pub fn validate_payment_request_amount_plan(
        grand_total: f64,
        has_payment_reference: bool,
        ref_amount: f64,
        existing_payment_request_amount: f64,
        grand_total_precision: u32,
        currency_precision: u32,
    ) -> Result<(), PaymentRequestError> {
        if has_payment_reference {
            return Ok(());
        }
        if grand_total == 0.0 {
            return Err(PaymentRequestError::Validation(
                "Grand Total cannot be zero".to_string(),
            ));
        }
        if ref_amount == 0.0 {
            return Err(PaymentRequestError::Validation(
                "Payment Entry is already created".to_string(),
            ));
        }
        let total = round_to_precision(
            existing_payment_request_amount
                + round_to_precision(grand_total, grand_total_precision),
            currency_precision,
        );
        if total > ref_amount {
            return Err(PaymentRequestError::Validation(
                "Total Payment Request amount cannot be greater than Sales Invoice amount"
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_currency(
        payment_account: Option<&str>,
        transaction_currency: Option<&str>,
        payment_account_currency: Option<&str>,
    ) -> Result<(), PaymentRequestError> {
        if payment_account.is_some() && transaction_currency != payment_account_currency {
            return Err(PaymentRequestError::Validation(
                "Transaction currency must be same as Payment Gateway currency".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_subscription_details_plan(
        &self,
        is_a_subscription: bool,
        subscription_plans: &[SubscriptionPlanInput],
    ) -> Result<SubscriptionValidationPlan, PaymentRequestError> {
        if !is_a_subscription {
            return Ok(SubscriptionValidationPlan {
                calculated_amount: 0.0,
                grand_total: self.grand_total,
                warning: None,
            });
        }

        let mut amount = 0.0;
        for subscription_plan in subscription_plans {
            if subscription_plan.payment_gateway != self.payment_gateway {
                return Err(PaymentRequestError::Validation(format!(
                    "The payment gateway account in plan {} is different from the payment gateway account in this payment request",
                    subscription_plan.name
                )));
            }
            amount += subscription_plan.rate * subscription_plan.qty;
        }
        let warning = (amount != self.grand_total).then(|| {
            format!(
                "The amount of {} set in this payment request is different from the calculated amount of all payment plans: {}. Make sure this is correct before submitting the document.",
                self.grand_total, amount
            )
        });
        Ok(SubscriptionValidationPlan {
            calculated_amount: amount,
            grand_total: self.grand_total,
            warning,
        })
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

    pub fn request_phone_payment_plan(
        &self,
    ) -> Result<PhonePaymentRequestPlan, PaymentRequestError> {
        Ok(PhonePaymentRequestPlan {
            reference_doctype: "Payment Request".to_string(),
            reference_docname: self.name.clone().unwrap_or_default(),
            payment_reference: self.reference_name.clone().unwrap_or_default(),
            request_amount: self.get_request_amount(&[])?,
            sender: self.email_to.clone(),
            currency: self.currency.clone(),
            payment_gateway: self.payment_gateway.clone(),
            phone_number: self.phone_number.clone(),
        })
    }

    pub fn get_payment_url_plan(
        &self,
        reference_party_data: Option<(&str, Option<&str>)>,
        fallback_user: Option<&str>,
        precision: u32,
    ) -> PaymentUrlPlan {
        let (title, payer_name) = reference_party_data.unwrap_or(("", None));
        let reference_docname = self.name.clone().unwrap_or_default();
        PaymentUrlPlan {
            amount: round_to_precision(self.grand_total, precision),
            title: title.to_string(),
            description: self.subject.clone(),
            reference_doctype: "Payment Request".to_string(),
            reference_docname: reference_docname.clone(),
            payer_email: self
                .email_to
                .clone()
                .or_else(|| fallback_user.map(ToOwned::to_owned))
                .unwrap_or_default(),
            payer_name: payer_name.map(ToOwned::to_owned),
            order_id: reference_docname,
            currency: self.currency.clone(),
            payment_gateway: self.payment_gateway.clone(),
        }
    }

    pub fn send_email_plan(&self) -> SendEmailPlan {
        SendEmailPlan {
            recipients: self.email_to.clone(),
            sender: None,
            subject: self.subject.clone(),
            message: self.message.clone(),
            reference_doctype: self.reference_doctype.clone(),
            reference_name: self.reference_name.clone(),
            print_format: self.print_format.clone(),
            queue: "short".to_string(),
            timeout: 300,
            enqueue_after_commit: true,
        }
    }

    pub fn create_payment_entry_plan(
        &self,
        ref_doc: &PaymentEntrySourceDoc,
        submit: bool,
        precision: u32,
    ) -> PaymentEntryRequestPlan {
        let party_account = match ref_doc.doctype.as_str() {
            "Sales Invoice" | "POS Invoice" => ref_doc.debit_to.clone(),
            "Purchase Invoice" => ref_doc.credit_to.clone(),
            _ => ref_doc.party_account.clone(),
        }
        .unwrap_or_default();

        let party_account_currency = self
            .party_account_currency
            .clone()
            .or_else(|| ref_doc.party_account_currency.clone())
            .unwrap_or_default();
        let party_amount = self.outstanding_amount;
        let mut bank_amount = self.outstanding_amount;
        if party_account_currency == ref_doc.company_currency
            && self.currency.as_deref() != Some(party_account_currency.as_str())
        {
            bank_amount = round_to_precision(
                safe_div(self.outstanding_amount, ref_doc.conversion_rate),
                precision,
            );
        }

        let paid_amount_override = if self.currency.as_deref()
            != Some(ref_doc.company_currency.as_str())
            && self.payment_request_type == "Outward"
            && ref_doc.paid_from_account_currency.as_deref()
                == Some(ref_doc.company_currency.as_str())
            && ref_doc.paid_from_account_currency != ref_doc.paid_to_account_currency
        {
            Some(ref_doc.target_exchange_rate * ref_doc.received_amount)
        } else {
            None
        };

        let reference_doctype = self.reference_doctype.clone().unwrap_or_default();
        let reference_name = self.reference_name.clone().unwrap_or_default();
        let reference_no = self.name.clone().unwrap_or_default();
        PaymentEntryRequestPlan {
            reference_doctype: reference_doctype.clone(),
            reference_name: reference_name.clone(),
            party_account,
            party_account_currency,
            party_amount,
            bank_account: self.payment_account.clone(),
            bank_amount,
            mode_of_payment: self.mode_of_payment.clone(),
            reference_no: reference_no.clone(),
            remarks: format!(
                "Payment Entry against {reference_doctype} {reference_name} via Payment Request {reference_no}"
            ),
            cost_center: self.cost_center.clone(),
            project: self.project.clone(),
            submit,
            created_from_payment_request: true,
            paid_amount_override,
        }
    }

    pub fn allocate_payment_request_to_pe_references(
        payment_request: &str,
        mut outstanding_amount: f64,
        references: Vec<PaymentEntryReferenceRow>,
        precision: u32,
    ) -> Vec<PaymentEntryReferenceRow> {
        if references.len() == 1 {
            let mut only = references[0].clone();
            only.payment_request = Some(payment_request.to_string());
            return vec![only];
        }

        let mut output = Vec::new();
        for mut row in references {
            row.idx = output.len() as i32 + 1;
            if outstanding_amount == 0.0 {
                output.push(row);
                continue;
            }

            row.payment_request = Some(payment_request.to_string());
            if row.allocated_amount <= outstanding_amount {
                outstanding_amount =
                    round_to_precision(outstanding_amount - row.allocated_amount, precision);
                output.push(row);
            } else {
                let remaining_allocated_amount =
                    round_to_precision(row.allocated_amount - outstanding_amount, precision);
                row.allocated_amount = outstanding_amount;
                outstanding_amount = 0.0;
                output.push(row);

                let mut new_row = output.last().cloned().unwrap_or_default();
                new_row.idx = output.len() as i32 + 1;
                new_row.payment_request = None;
                new_row.allocated_amount = remaining_allocated_amount;
                output.push(new_row);
            }
        }
        output
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

pub fn set_payment_references(
    payment_schedules: &str,
) -> Result<Vec<PaymentReferenceRow>, PaymentRequestError> {
    if payment_schedules.trim().is_empty() {
        return Ok(Vec::new());
    }

    let rows: Value = serde_json::from_str(payment_schedules)
        .map_err(|err| PaymentRequestError::InvalidRequestAmountJson(err.to_string()))?;
    let rows = rows.as_array().ok_or_else(|| {
        PaymentRequestError::InvalidRequestAmountJson("expected JSON array".to_string())
    })?;

    Ok(rows
        .iter()
        .map(|row| PaymentReferenceRow {
            payment_term: row
                .get("payment_term")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            payment_schedule: row
                .get("name")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            description: row
                .get("description")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            due_date: row
                .get("due_date")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            amount: row
                .get("payment_amount")
                .and_then(Value::as_f64)
                .unwrap_or(0.0),
        })
        .collect())
}

pub fn apply_payment_references(
    existing_refs: Vec<PaymentReferenceRow>,
    payment_reference: Vec<PaymentReferenceRow>,
) -> (Vec<PaymentReferenceRow>, f64) {
    let mut merged = existing_refs;
    let existing_ids: Vec<String> = merged
        .iter()
        .filter_map(|row| row.payment_schedule.clone())
        .collect();
    merged.extend(payment_reference.into_iter().filter(|row| {
        row.payment_schedule
            .as_ref()
            .is_none_or(|schedule| !existing_ids.contains(schedule))
    }));
    let grand_total = merged.iter().map(|row| row.amount).sum();
    (merged, grand_total)
}

pub fn get_print_format_list(_ref_doctype: &str, custom_formats: &[String]) -> Vec<String> {
    let mut print_format_list = vec!["Standard".to_string()];
    print_format_list.extend(custom_formats.iter().cloned());
    print_format_list
}

pub fn validate_payment(
    reference_doctype: &str,
    reference_docname: &str,
    payment_request_status: Option<&str>,
) -> Result<(), PaymentRequestError> {
    if reference_doctype == "Payment Request" && payment_request_status == Some("Paid") {
        return Err(PaymentRequestError::Validation(format!(
            "The Payment Request {reference_docname} is already paid, cannot process payment twice"
        )));
    }
    Ok(())
}

pub fn make_payment_order_plan(source: &PaymentRequest) -> PaymentOrderFromRequestPlan {
    PaymentOrderFromRequestPlan {
        payment_order_type: "Payment Request".to_string(),
        reference_doctype: source.reference_doctype.clone().unwrap_or_default(),
        reference_name: source.reference_name.clone().unwrap_or_default(),
        amount: source.grand_total,
        supplier: source.party.clone(),
        payment_request: source.name.clone().unwrap_or_default(),
        mode_of_payment: source.mode_of_payment.clone(),
        bank_account: source.bank_account.clone(),
        account: source.account.clone(),
    }
}

pub fn get_open_payment_requests_query_plan(
    doctype: &str,
    txt: &str,
    searchfield: &str,
    start: i32,
    page_len: i32,
    reference_doctype: Option<&str>,
    reference_name: Option<&str>,
) -> Option<OpenPaymentRequestsQueryPlan> {
    Some(OpenPaymentRequestsQueryPlan {
        doctype: doctype.to_string(),
        text_filter: (!txt.is_empty()).then(|| txt.to_string()),
        searchfield: searchfield.to_string(),
        start,
        page_len,
        reference_doctype: reference_doctype?.to_string(),
        reference_name: reference_name?.to_string(),
        order_by: "transaction_date ASC,creation ASC".to_string(),
    })
}

pub fn get_dummy_message() -> &'static str {
    r#"
        {% if doc.contact_person -%}
        <p>Dear {{ doc.contact_person }},</p>
        {%- else %}<p>Hello,</p>{% endif %}

        <p>
            {{ _("Requesting payment against {0} {1} for amount {2}").format(
                doc.doctype,
                doc.name,
                payment_request.get_formatted("grand_total")
            ) }}
        </p>

        <a href="{{ payment_url }}">{{ _("Make Payment") }}</a>

        <p>{{ _("If you have any questions, please get back to us.") }}</p>

        <p>{{ _("Thank you for your business!") }}</p>
    "#
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
