use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoice {
    pub name: Option<String>,
    pub company: Option<String>,
    pub customer: Option<String>,
    pub is_pos: bool,
    pub is_return: bool,
    pub docstatus: i32,
    pub status: String,
    pub due_date: Option<String>,
    pub posting_date: Option<String>,
    pub grand_total: f64,
    pub rounded_total: Option<f64>,
    pub base_grand_total: f64,
    pub base_rounded_total: Option<f64>,
    pub conversion_rate: f64,
    pub paid_amount: f64,
    pub base_paid_amount: f64,
    pub outstanding_amount: f64,
    pub change_amount: f64,
    pub base_change_amount: f64,
    pub write_off_amount: f64,
    pub base_write_off_amount: f64,
    pub account_for_change_amount: Option<String>,
    pub consolidated_invoice: Option<String>,
    pub loyalty_program: Option<String>,
    pub return_against: Option<String>,
    pub coupon_code: Option<String>,
    pub payments: Vec<PosInvoicePayment>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoicePayment {
    pub idx: i32,
    pub mode_of_payment: Option<String>,
    pub payment_type: String,
    pub account: Option<String>,
    pub amount: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosInvoiceError {
    Validation(String),
}

impl PosInvoice {
    pub const DOCTYPE: &'static str = "POS Invoice";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const IS_SUBMITTABLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 6] = [
        "naming_series",
        "customer",
        "is_pos",
        "items",
        "payments",
        "status",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("naming_series", "Series").default("ACC-PSINV-.YYYY.-"),
            FieldSpec::link("customer", "Customer").options("Customer"),
            FieldSpec::check("is_pos", "Include Payment").default("1"),
            FieldSpec::table("items", "Items").options("POS Invoice Item"),
            FieldSpec::table("payments", "Payments").options("Sales Invoice Payment"),
            FieldSpec::select("status", "Status")
                .options("\nDraft\nReturn\nCredit Note Issued\nConsolidated\nSubmitted\nPaid\nPartly Paid\nUnpaid\nPartly Paid and Discounted\nUnpaid and Discounted\nOverdue and Discounted\nOverdue\nCancelled"),
        ]
    }

    pub fn validate_basic(&self) -> Result<(), PosInvoiceError> {
        if self.customer.as_deref().unwrap_or_default().is_empty() {
            return Err(PosInvoiceError::Validation(
                "Please select Customer first".to_string(),
            ));
        }
        if !self.is_pos {
            return Err(PosInvoiceError::Validation(
                "POS Invoice should have the field Include Payment checked.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_mode_of_payment(&self) -> Result<(), PosInvoiceError> {
        if self.payments.is_empty() {
            return Err(PosInvoiceError::Validation(
                "At least one mode of payment is required for POS invoice.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_payment_amount(&self, precision: u32) -> Result<(), PosInvoiceError> {
        let mut total_amount_in_payments = 0.0;
        for entry in &self.payments {
            total_amount_in_payments += entry.amount;
            if !self.is_return && entry.amount < 0.0 {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{} (Payment Table): Amount must be positive",
                    entry.idx
                )));
            }
            if self.is_return && entry.amount > 0.0 {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{} (Payment Table): Amount must be negative",
                    entry.idx
                )));
            }
        }

        if self.is_return && self.docstatus != 0 {
            let invoice_total = self.rounded_total.unwrap_or(self.grand_total);
            total_amount_in_payments = round_to_precision(total_amount_in_payments, precision);
            if total_amount_in_payments != 0.0 && total_amount_in_payments < invoice_total {
                return Err(PosInvoiceError::Validation(format!(
                    "Total payments amount can't be greater than {}",
                    -invoice_total
                )));
            }
        }
        Ok(())
    }

    pub fn validate_change_amount(&mut self) {
        let grand_total = self.rounded_total.unwrap_or(self.grand_total);
        let base_grand_total = self.base_rounded_total.unwrap_or(self.base_grand_total);
        if self.change_amount == 0.0 && grand_total < self.paid_amount {
            self.change_amount = self.paid_amount - grand_total + self.write_off_amount;
            self.base_change_amount =
                self.base_paid_amount - base_grand_total + self.base_write_off_amount;
        }
    }

    pub fn validate_change_account(
        &self,
        account_company: Option<&str>,
    ) -> Result<(), PosInvoiceError> {
        if self.change_amount != 0.0 && self.account_for_change_amount.is_none() {
            return Err(PosInvoiceError::Validation(
                "Please enter Account for Change Amount".to_string(),
            ));
        }
        if self.change_amount != 0.0
            && self.account_for_change_amount.is_some()
            && account_company != self.company.as_deref()
        {
            return Err(PosInvoiceError::Validation(format!(
                "The selected change account {} doesn't belongs to Company {}.",
                self.account_for_change_amount
                    .as_deref()
                    .unwrap_or_default(),
                self.company.as_deref().unwrap_or_default()
            )));
        }
        Ok(())
    }

    pub fn validate_company_with_pos_company(
        &self,
        company: Option<&str>,
        pos_company: Option<&str>,
    ) -> Result<(), PosInvoiceError> {
        if company != pos_company {
            return Err(PosInvoiceError::Validation(format!(
                "Company {} does not match with POS Profile Company {}",
                company.unwrap_or_default(),
                pos_company.unwrap_or_default()
            )));
        }
        Ok(())
    }

    pub fn set_outstanding_amount(&mut self) {
        let total = self.rounded_total.unwrap_or(self.grand_total);
        self.outstanding_amount = if total > self.paid_amount {
            total - self.paid_amount
        } else {
            0.0
        };
    }

    pub fn set_status(&mut self, _update: bool, status: Option<&str>, today: &str) -> String {
        if let Some(status) = status {
            self.status = status.to_string();
            return self.status.clone();
        }
        let total = self.rounded_total.unwrap_or(self.grand_total);
        self.status = if self.docstatus == 2 {
            "Cancelled".to_string()
        } else if self.docstatus == 1 {
            if self.consolidated_invoice.is_some() {
                "Consolidated".to_string()
            } else if self.outstanding_amount > 0.0
                && self.due_date.as_deref().unwrap_or(today) < today
            {
                "Overdue".to_string()
            } else if self.outstanding_amount > 0.0 && self.outstanding_amount < total {
                "Partly Paid".to_string()
            } else if self.outstanding_amount > 0.0 {
                "Unpaid".to_string()
            } else if self.is_return {
                "Return".to_string()
            } else if self.outstanding_amount <= 0.0 {
                "Paid".to_string()
            } else {
                "Submitted".to_string()
            }
        } else {
            "Draft".to_string()
        };
        self.status.clone()
    }

    pub fn before_submit_plan(&self) -> &'static str {
        "set_outstanding_amount"
    }

    pub fn before_cancel_plan(
        &self,
        consolidated_invoice_submitted: bool,
        pos_closing_entry: Option<&str>,
    ) -> Result<(), PosInvoiceError> {
        if self.consolidated_invoice.is_some() && consolidated_invoice_submitted {
            return Err(PosInvoiceError::Validation(format!(
                "You need to cancel POS Closing Entry {} to be able to cancel this document.",
                pos_closing_entry.unwrap_or_default()
            )));
        }
        Ok(())
    }

    pub fn on_cancel_plan(&self) -> Vec<String> {
        let mut plan = vec![
            "ignore_linked:Payment Ledger Entry,Serial and Batch Bundle".to_string(),
            "sales_invoice_on_cancel".to_string(),
            "set_status:Cancelled".to_string(),
        ];
        if self.coupon_code.is_some() {
            plan.push("update_coupon_code_count:cancelled".to_string());
        }
        plan.push("delink_serial_and_batch_bundle".to_string());
        plan
    }

    pub fn check_phone_payments(&self, paid_amount: Option<f64>) -> Result<(), PosInvoiceError> {
        for pay in &self.payments {
            if pay.payment_type == "Phone" && pay.amount >= 0.0 {
                if let Some(paid_amount) = paid_amount {
                    if pay.amount != paid_amount {
                        return Err(PosInvoiceError::Validation(format!(
                            "Payment related to {} is not completed",
                            pay.mode_of_payment.as_deref().unwrap_or_default()
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}

impl DocumentController for PosInvoice {
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

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
