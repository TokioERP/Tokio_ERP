use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliation {
    pub company: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub receivable_payable_account: Option<String>,
    pub default_advance_account: Option<String>,
    pub from_invoice_date: Option<String>,
    pub to_invoice_date: Option<String>,
    pub invoice_limit: i32,
    pub from_payment_date: Option<String>,
    pub to_payment_date: Option<String>,
    pub payment_limit: i32,
    pub minimum_invoice_amount: Option<f64>,
    pub minimum_payment_amount: Option<f64>,
    pub maximum_invoice_amount: Option<f64>,
    pub maximum_payment_amount: Option<f64>,
    pub bank_cash_account: Option<String>,
    pub cost_center: Option<String>,
    pub payment_name: Option<String>,
    pub invoice_name: Option<String>,
    pub invoices: Vec<PaymentReconciliationInvoice>,
    pub payments: Vec<PaymentReconciliationPayment>,
    pub allocation: Vec<PaymentReconciliationAllocation>,
    pub dimensions: Vec<String>,
    pub dimension_values: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliationPayment {
    pub reference_type: String,
    pub reference_name: String,
    pub reference_row: Option<String>,
    pub amount: f64,
    pub unreconciled_amount: f64,
    pub difference_amount: f64,
    pub exchange_rate: Option<f64>,
    pub currency: Option<String>,
    pub cost_center: Option<String>,
    pub posting_date: Option<String>,
    pub is_advance: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliationInvoice {
    pub invoice_type: String,
    pub invoice_number: String,
    pub invoice_date: Option<String>,
    pub amount: f64,
    pub outstanding_amount: f64,
    pub exchange_rate: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliationAllocation {
    pub idx: i32,
    pub reference_type: String,
    pub reference_name: String,
    pub reference_row: Option<String>,
    pub invoice_type: String,
    pub invoice_number: String,
    pub unreconciled_amount: f64,
    pub amount: f64,
    pub allocated_amount: f64,
    pub difference_amount: f64,
    pub difference_account: Option<String>,
    pub gain_loss_posting_date: Option<String>,
    pub exchange_rate: Option<f64>,
    pub currency: Option<String>,
    pub cost_center: Option<String>,
    pub dimensions: BTreeMap<String, String>,
    pub is_advance: bool,
    pub debit_or_credit_note_posting_date: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentDetails {
    pub voucher_type: String,
    pub voucher_no: String,
    pub voucher_detail_no: Option<String>,
    pub against_voucher_type: String,
    pub against_voucher: String,
    pub account: String,
    pub exchange_rate: Option<f64>,
    pub party_type: String,
    pub party: String,
    pub is_advance: bool,
    pub dr_or_cr: String,
    pub unreconciled_amount: f64,
    pub unadjusted_amount: f64,
    pub allocated_amount: f64,
    pub difference_amount: f64,
    pub difference_account: Option<String>,
    pub difference_posting_date: Option<String>,
    pub debit_or_credit_note_posting_date: Option<String>,
    pub cost_center: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentReconciliationFilterPlan {
    pub common: Vec<String>,
    pub accounting_dimensions: Vec<String>,
    pub posting_date: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReconcileDrCrNotePlan {
    pub voucher_type: String,
    pub posting_date: String,
    pub company: String,
    pub multi_currency: bool,
    pub debit_or_credit_account_field: String,
    pub reverse_dr_or_cr: String,
    pub allocated_amount: f64,
    pub reference_type: String,
    pub reference_name: String,
    pub note_reference_type: String,
    pub note_reference_name: String,
    pub cost_center: String,
    pub dimensions: BTreeMap<String, String>,
    pub gain_loss_dr_or_cr: Option<String>,
    pub gain_loss_reverse_dr_or_cr: Option<String>,
    pub difference_amount: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentReconciliationError {
    Validation(String),
}

impl PaymentReconciliation {
    pub const DOCTYPE: &'static str = "Payment Reconciliation";
    pub const MODULE: &'static str = "Accounts";
    pub const ALLOW_COPY: bool = true;
    pub const FIELD_ORDER: [&'static str; 32] = [
        "company",
        "party_type",
        "column_break_4",
        "party",
        "receivable_payable_account",
        "default_advance_account",
        "col_break1",
        "from_invoice_date",
        "from_payment_date",
        "minimum_invoice_amount",
        "minimum_payment_amount",
        "column_break_11",
        "to_invoice_date",
        "to_payment_date",
        "maximum_invoice_amount",
        "maximum_payment_amount",
        "column_break_13",
        "invoice_limit",
        "payment_limit",
        "bank_cash_account",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "sec_break1",
        "invoice_name",
        "invoices",
        "column_break_15",
        "payment_name",
        "payments",
        "sec_break2",
        "allocation",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .required(),
            FieldSpec::dynamic_link("party")
                .options("party_type")
                .required(),
            FieldSpec::link("receivable_payable_account", "Receivable / Payable Account")
                .options("Account")
                .required(),
            FieldSpec::table("payments", "Payments").options("Payment Reconciliation Payment"),
            FieldSpec::table("invoices", "Invoices").options("Payment Reconciliation Invoice"),
            FieldSpec::table("allocation", "Allocation")
                .options("Payment Reconciliation Allocation"),
        ]
    }

    pub fn load_from_db_defaults() -> Self {
        Self {
            invoice_limit: 50,
            payment_limit: 50,
            ..Self::default()
        }
    }

    pub fn check_mandatory_to_fetch(&self) -> Result<(), PaymentReconciliationError> {
        for (fieldname, label) in [
            (&self.company, "Company"),
            (&self.party_type, "Party Type"),
            (&self.party, "Party"),
            (
                &self.receivable_payable_account,
                "Receivable / Payable Account",
            ),
        ] {
            if fieldname.as_deref().unwrap_or_default().is_empty() {
                return Err(PaymentReconciliationError::Validation(format!(
                    "Please select {label} first"
                )));
            }
        }
        Ok(())
    }

    pub fn validate_entries(&self) -> Result<(), PaymentReconciliationError> {
        if self.invoices.is_empty() {
            return Err(PaymentReconciliationError::Validation(
                "No records found in the Invoices table".to_string(),
            ));
        }
        if self.payments.is_empty() {
            return Err(PaymentReconciliationError::Validation(
                "No records found in the Payments table".to_string(),
            ));
        }
        Ok(())
    }

    pub fn get_difference_amount(
        &self,
        payment_entry: &PaymentReconciliationPayment,
        invoice: &PaymentReconciliationInvoice,
        allocated_amount: f64,
        account_type: &str,
        account_currency: &str,
        company_currency: &str,
        precision: u32,
    ) -> f64 {
        let mut difference_amount = 0.0;
        if account_currency != company_currency {
            if let Some(invoice_exchange_rate) = invoice.exchange_rate {
                let payment_exchange_rate = payment_entry.exchange_rate.unwrap_or(1.0);
                if payment_exchange_rate != invoice_exchange_rate {
                    let allocated_amount = round_to_precision(allocated_amount, precision);
                    let allocated_amount_in_ref_rate =
                        round_to_precision(payment_exchange_rate * allocated_amount, precision);
                    let allocated_amount_in_inv_rate =
                        round_to_precision(invoice_exchange_rate * allocated_amount, precision);
                    if account_type == "Payable"
                        && matches!(
                            invoice.invoice_type.as_str(),
                            "Payment Entry" | "Journal Entry"
                        )
                    {
                        difference_amount =
                            allocated_amount_in_inv_rate - allocated_amount_in_ref_rate;
                    } else {
                        difference_amount =
                            allocated_amount_in_ref_rate - allocated_amount_in_inv_rate;
                    }
                }
            }
        }
        difference_amount
    }

    pub fn calculate_difference_on_allocation_change(
        &self,
        payment_entry: &mut [PaymentReconciliationPayment],
        invoice: &mut [PaymentReconciliationInvoice],
        allocated_amount: f64,
        invoice_exchange_map: &BTreeMap<String, f64>,
        account_type: &str,
        account_currency: &str,
        company_currency: &str,
        precision: u32,
    ) -> f64 {
        if let Some(first_invoice) = invoice.first_mut() {
            first_invoice.exchange_rate = invoice_exchange_map
                .get(&first_invoice.invoice_number)
                .copied();
        }
        if let Some(first_payment) = payment_entry.first_mut() {
            if matches!(
                first_payment.reference_type.as_str(),
                "Sales Invoice" | "Purchase Invoice"
            ) {
                first_payment.exchange_rate = invoice_exchange_map
                    .get(&first_payment.reference_name)
                    .copied();
            }
        }
        self.get_difference_amount(
            &payment_entry[0],
            &invoice[0],
            allocated_amount,
            account_type,
            account_currency,
            company_currency,
            precision,
        )
    }

    pub fn allocate_entries(
        &self,
        mut payments: Vec<PaymentReconciliationPayment>,
        mut invoices: Vec<PaymentReconciliationInvoice>,
        invoice_exchange_map: &BTreeMap<String, f64>,
        default_exchange_gain_loss_account: Option<&str>,
        exchange_gain_loss_posting_date: &str,
        nowdate: &str,
        account_type: &str,
        account_currency: &str,
        company_currency: &str,
        precision: u32,
    ) -> Vec<PaymentReconciliationAllocation> {
        let mut entries = Vec::new();
        for pay in &mut payments {
            pay.unreconciled_amount = pay.amount;
            for inv in &mut invoices {
                let allocated_amount = if pay.amount >= inv.outstanding_amount {
                    inv.outstanding_amount
                } else {
                    pay.amount
                };

                inv.exchange_rate = invoice_exchange_map.get(&inv.invoice_number).copied();
                if matches!(
                    pay.reference_type.as_str(),
                    "Sales Invoice" | "Purchase Invoice"
                ) {
                    pay.exchange_rate = invoice_exchange_map.get(&pay.reference_name).copied();
                }

                let mut res = self.get_allocated_entry(pay, inv, allocated_amount);
                res.difference_amount = self.get_difference_amount(
                    pay,
                    inv,
                    res.allocated_amount,
                    account_type,
                    account_currency,
                    company_currency,
                    precision,
                );
                res.difference_account = default_exchange_gain_loss_account.map(ToOwned::to_owned);
                res.exchange_rate = inv.exchange_rate;
                res.gain_loss_posting_date = pay.posting_date.clone();
                if !pay.is_advance {
                    if exchange_gain_loss_posting_date == "Invoice" {
                        res.gain_loss_posting_date = inv.invoice_date.clone();
                    } else if exchange_gain_loss_posting_date == "Reconciliation Date" {
                        res.gain_loss_posting_date = Some(nowdate.to_string());
                    }
                }

                if pay.amount >= inv.outstanding_amount {
                    pay.amount = round_to_precision(pay.amount - inv.outstanding_amount, precision);
                    inv.outstanding_amount = 0.0;
                } else {
                    inv.outstanding_amount =
                        round_to_precision(inv.outstanding_amount - pay.amount, precision);
                    pay.amount = 0.0;
                }

                if pay.amount == 0.0 {
                    entries.push(res);
                    break;
                } else if inv.outstanding_amount == 0.0 {
                    entries.push(res);
                    continue;
                }
            }
        }
        entries
            .into_iter()
            .filter(|entry| entry.allocated_amount != 0.0)
            .collect()
    }

    pub fn update_dimension_values_in_allocated_entries(
        &self,
        allocation: &mut PaymentReconciliationAllocation,
    ) {
        for dimension in &self.dimensions {
            if let Some(value) = self.dimension_values.get(dimension) {
                allocation
                    .dimensions
                    .insert(dimension.clone(), value.clone());
            }
        }
    }

    pub fn get_allocated_entry(
        &self,
        pay: &PaymentReconciliationPayment,
        inv: &PaymentReconciliationInvoice,
        allocated_amount: f64,
    ) -> PaymentReconciliationAllocation {
        let mut allocation = PaymentReconciliationAllocation {
            reference_type: pay.reference_type.clone(),
            reference_name: pay.reference_name.clone(),
            reference_row: pay.reference_row.clone(),
            invoice_type: inv.invoice_type.clone(),
            invoice_number: inv.invoice_number.clone(),
            unreconciled_amount: pay.unreconciled_amount,
            amount: pay.amount,
            allocated_amount,
            difference_amount: pay.difference_amount,
            currency: inv.currency.clone(),
            cost_center: pay.cost_center.clone(),
            is_advance: pay.is_advance,
            ..PaymentReconciliationAllocation::default()
        };
        self.update_dimension_values_in_allocated_entries(&mut allocation);
        allocation
    }

    pub fn get_payment_details(
        &self,
        row: &PaymentReconciliationAllocation,
        dr_or_cr: &str,
    ) -> PaymentDetails {
        PaymentDetails {
            voucher_type: row.reference_type.clone(),
            voucher_no: row.reference_name.clone(),
            voucher_detail_no: row.reference_row.clone(),
            against_voucher_type: row.invoice_type.clone(),
            against_voucher: row.invoice_number.clone(),
            account: self.receivable_payable_account.clone().unwrap_or_default(),
            exchange_rate: row.exchange_rate,
            party_type: self.party_type.clone().unwrap_or_default(),
            party: self.party.clone().unwrap_or_default(),
            is_advance: row.is_advance,
            dr_or_cr: dr_or_cr.to_string(),
            unreconciled_amount: row.unreconciled_amount,
            unadjusted_amount: row.amount,
            allocated_amount: row.allocated_amount,
            difference_amount: row.difference_amount,
            difference_account: row.difference_account.clone(),
            difference_posting_date: row.gain_loss_posting_date.clone(),
            debit_or_credit_note_posting_date: row.debit_or_credit_note_posting_date.clone(),
            cost_center: row.cost_center.clone(),
            dimensions: row.dimensions.clone(),
        }
    }

    pub fn validate_allocation(&self) -> Result<(), PaymentReconciliationError> {
        let mut unreconciled_invoices: BTreeMap<(String, String), f64> = BTreeMap::new();
        for inv in &self.invoices {
            unreconciled_invoices
                .entry((inv.invoice_type.clone(), inv.invoice_number.clone()))
                .or_insert(inv.outstanding_amount);
        }

        let mut invoices_to_reconcile = Vec::new();
        for row in &self.allocation {
            if !row.invoice_type.is_empty()
                && !row.invoice_number.is_empty()
                && row.allocated_amount != 0.0
            {
                invoices_to_reconcile.push(row.invoice_number.clone());
                if row.amount - row.allocated_amount < 0.0 {
                    return Err(PaymentReconciliationError::Validation(format!(
                        "Row {}: Allocated amount {} must be less than or equal to remaining payment amount {}",
                        row.idx,
                        format_number(row.allocated_amount),
                        format_number(row.amount)
                    )));
                }

                let invoice_outstanding = unreconciled_invoices
                    .get(&(row.invoice_type.clone(), row.invoice_number.clone()))
                    .copied()
                    .unwrap_or(0.0);
                if row.allocated_amount - invoice_outstanding > 0.009 {
                    return Err(PaymentReconciliationError::Validation(format!(
                        "Row {}: Allocated amount {} must be less than or equal to invoice outstanding amount {}",
                        row.idx,
                        format_number(row.allocated_amount),
                        format_number(invoice_outstanding)
                    )));
                }
            }
        }
        if invoices_to_reconcile.is_empty() {
            return Err(PaymentReconciliationError::Validation(
                "No records found in Allocation table".to_string(),
            ));
        }
        Ok(())
    }

    pub fn build_qb_filter_conditions(
        &self,
        get_invoices: bool,
        get_return_invoices: bool,
    ) -> PaymentReconciliationFilterPlan {
        let mut plan = PaymentReconciliationFilterPlan::default();
        if let Some(company) = self.company.as_deref() {
            plan.common.push(format!("company = '{company}'"));
        }

        if self.cost_center.is_some() && (get_invoices || get_return_invoices) {
            plan.accounting_dimensions.push(format!(
                "cost_center = '{}'",
                self.cost_center.as_deref().unwrap_or_default()
            ));
        }

        if get_invoices {
            if let Some(from_invoice_date) = self.from_invoice_date.as_deref() {
                plan.posting_date
                    .push(format!("posting_date >= '{from_invoice_date}'"));
            }
            if let Some(to_invoice_date) = self.to_invoice_date.as_deref() {
                plan.posting_date
                    .push(format!("posting_date <= '{to_invoice_date}'"));
            }
        } else if get_return_invoices {
            if let Some(from_payment_date) = self.from_payment_date.as_deref() {
                plan.posting_date
                    .push(format!("posting_date >= '{from_payment_date}'"));
            }
            if let Some(to_payment_date) = self.to_payment_date.as_deref() {
                plan.posting_date
                    .push(format!("posting_date <= '{to_payment_date}'"));
            }
        }

        plan.accounting_dimensions
            .extend(self.build_dimensions_filter_conditions());
        plan
    }

    pub fn build_dimensions_filter_conditions(&self) -> Vec<String> {
        self.dimensions
            .iter()
            .filter_map(|dimension| {
                self.dimension_values
                    .get(dimension)
                    .map(|value| format!("{dimension} = '{value}'"))
            })
            .collect()
    }

    pub fn get_journal_filter_conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();
        if let Some(company) = self.company.as_deref() {
            conditions.push(format!("Journal Entry.company = '{company}'"));
        }
        if let Some(from_payment_date) = self.from_payment_date.as_deref() {
            conditions.push(format!(
                "Journal Entry.posting_date >= '{from_payment_date}'"
            ));
        }
        if let Some(to_payment_date) = self.to_payment_date.as_deref() {
            conditions.push(format!("Journal Entry.posting_date <= '{to_payment_date}'"));
        }
        if let Some(minimum_payment_amount) = self.minimum_payment_amount {
            conditions.push(format!(
                "Journal Entry.total_debit >= {}",
                format_number(minimum_payment_amount)
            ));
        }
        if let Some(maximum_payment_amount) = self.maximum_payment_amount {
            conditions.push(format!(
                "Journal Entry.total_debit <= {}",
                format_number(maximum_payment_amount)
            ));
        }
        conditions
    }

    pub fn reconcile_dr_cr_note_plans(
        dr_cr_notes: &mut [PaymentReconciliationAllocation],
        company: &str,
        company_currency: &str,
        default_cost_center: Option<&str>,
        outstanding_by_voucher: &BTreeMap<String, f64>,
    ) -> Result<Vec<ReconcileDrCrNotePlan>, PaymentReconciliationError> {
        let mut plans = Vec::new();
        for inv in dr_cr_notes {
            let outstanding = outstanding_by_voucher
                .get(&inv.reference_name)
                .copied()
                .unwrap_or(0.0);
            if outstanding.abs() < inv.allocated_amount {
                return Err(PaymentReconciliationError::Validation(format!(
                    "{} has been modified after you pulled it. Please pull it again.",
                    inv.reference_type
                )));
            }

            let voucher_type = if inv.reference_type == "Sales Invoice" {
                "Credit Note"
            } else {
                "Debit Note"
            };
            let reconcile_dr_or_cr = if inv.reference_type == "Sales Invoice" {
                "credit_in_account_currency"
            } else {
                "debit_in_account_currency"
            };
            let reverse_dr_or_cr = if reconcile_dr_or_cr == "credit_in_account_currency" {
                "debit"
            } else {
                "credit"
            };
            let (gain_loss_dr_or_cr, gain_loss_reverse_dr_or_cr) = if inv.difference_amount != 0.0 {
                if inv.invoice_type == "Sales Invoice" {
                    let dr_or_cr = if inv.difference_amount < 0.0 {
                        "credit"
                    } else {
                        "debit"
                    };
                    let reverse = if dr_or_cr == "credit" {
                        "debit"
                    } else {
                        "credit"
                    };
                    (Some(dr_or_cr.to_string()), Some(reverse.to_string()))
                } else {
                    let dr_or_cr = if inv.difference_amount < 0.0 {
                        "debit"
                    } else {
                        "credit"
                    };
                    let reverse = if dr_or_cr == "credit" {
                        "debit"
                    } else {
                        "credit"
                    };
                    (Some(dr_or_cr.to_string()), Some(reverse.to_string()))
                }
            } else {
                (None, None)
            };

            plans.push(ReconcileDrCrNotePlan {
                voucher_type: voucher_type.to_string(),
                posting_date: inv
                    .debit_or_credit_note_posting_date
                    .clone()
                    .unwrap_or_else(|| "2026-06-05".to_string()),
                company: company.to_string(),
                multi_currency: inv
                    .currency
                    .as_deref()
                    .is_some_and(|currency| currency != company_currency),
                debit_or_credit_account_field: reconcile_dr_or_cr.to_string(),
                reverse_dr_or_cr: reverse_dr_or_cr.to_string(),
                allocated_amount: inv.allocated_amount,
                reference_type: inv.invoice_type.clone(),
                reference_name: inv.invoice_number.clone(),
                note_reference_type: inv.reference_type.clone(),
                note_reference_name: inv.reference_name.clone(),
                cost_center: inv
                    .cost_center
                    .clone()
                    .or_else(|| default_cost_center.map(ToOwned::to_owned))
                    .unwrap_or_default(),
                dimensions: inv.dimensions.clone(),
                gain_loss_dr_or_cr,
                gain_loss_reverse_dr_or_cr,
                difference_amount: inv.difference_amount,
            });
        }
        Ok(plans)
    }
}

impl DocumentController for PaymentReconciliation {
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

pub fn get_queries_for_dimension_filters(
    company: Option<&str>,
    dimensions: &[(String, String, bool)],
) -> Vec<(String, BTreeMap<String, String>)> {
    dimensions
        .iter()
        .map(|(fieldname, _document_type, is_tree)| {
            let mut filters = BTreeMap::new();
            if let Some(company) = company {
                filters.insert("company".to_string(), company.to_string());
            }
            if *is_tree {
                filters.insert("is_group".to_string(), "0".to_string());
            }
            (fieldname.clone(), filters)
        })
        .collect()
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
