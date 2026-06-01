use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InvoiceDiscountingStatus {
    Draft,
    Sanctioned,
    Disbursed,
    Settled,
    Cancelled,
}

impl Default for InvoiceDiscountingStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvoiceDiscounting {
    pub name: Option<String>,
    pub docstatus: i32,
    pub posting_date: String,
    pub loan_start_date: Option<String>,
    pub loan_period: i64,
    pub loan_end_date: Option<String>,
    pub status: InvoiceDiscountingStatus,
    pub company: String,
    pub invoices: Vec<DiscountedInvoiceRow>,
    pub total_amount: f64,
    pub bank_charges: f64,
    pub short_term_loan: String,
    pub bank_account: String,
    pub bank_charges_account: String,
    pub accounts_receivable_credit: String,
    pub accounts_receivable_discounted: String,
    pub accounts_receivable_unpaid: String,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiscountedInvoiceRow {
    pub idx: usize,
    pub sales_invoice: String,
    pub customer: String,
    pub outstanding_amount: f64,
    pub parent: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvoiceDiscountingGlInvoice {
    pub sales_invoice: String,
    pub debit_to: String,
    pub party_account_currency: String,
    pub conversion_rate: f64,
    pub cost_center: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlEntryPlan {
    pub account: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub against: Option<String>,
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub cost_center: Option<String>,
    pub against_voucher: Option<String>,
    pub against_voucher_type: Option<String>,
    pub account_currency: Option<String>,
    pub dimensions: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JournalEntryPlan {
    pub voucher_type: String,
    pub company: String,
    pub remark: String,
    pub accounts: Vec<JournalAccountPlan>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct JournalAccountPlan {
    pub account: String,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub cost_center: Option<String>,
    pub reference_type: Option<String>,
    pub reference_name: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SalesInvoiceCandidate {
    pub sales_invoice: String,
    pub customer: String,
    pub posting_date: String,
    pub outstanding_amount: f64,
    pub debit_to: String,
    pub docstatus: i32,
    pub base_grand_total: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceDiscountingError {
    MissingLoanStartDateOrPeriod,
    AlreadyDiscounted {
        row: usize,
        sales_invoice: String,
        parent: Option<String>,
    },
    OutstandingGreaterThanActual {
        row: usize,
        sales_invoice: String,
        actual_outstanding: f64,
    },
}

impl InvoiceDiscounting {
    pub const DOCTYPE: &'static str = "Invoice Discounting";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-INV-DISC-.YYYY.-.#####";
    pub const IS_SUBMITTABLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 22] = [
        "posting_date",
        "loan_start_date",
        "loan_period",
        "loan_end_date",
        "column_break_3",
        "status",
        "company",
        "section_break_5",
        "invoices",
        "section_break_7",
        "total_amount",
        "column_break_9",
        "bank_charges",
        "section_break_6",
        "short_term_loan",
        "bank_account",
        "bank_charges_account",
        "column_break_15",
        "accounts_receivable_credit",
        "accounts_receivable_discounted",
        "accounts_receivable_unpaid",
        "amended_from",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .required()
                .in_list_view(),
            FieldSpec::date("loan_start_date", "Loan Start Date"),
            FieldSpec::int("loan_period", "Loan Period (Days)"),
            FieldSpec::date("loan_end_date", "Loan End Date")
                .no_copy()
                .read_only(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::select("status", "Status")
                .options("Draft\nSanctioned\nDisbursed\nSettled\nCancelled")
                .no_copy()
                .read_only(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("invoices", "Invoices")
                .options("Discounted Invoice")
                .required(),
            FieldSpec::section_break("section_break_7"),
            FieldSpec::currency("total_amount", "Total Amount")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::currency("bank_charges", "Bank Charges")
                .options("Company:company:default_currency"),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::link("short_term_loan", "Short Term Loan Account")
                .options("Account")
                .required(),
            FieldSpec::link("bank_account", "Bank Account")
                .options("Account")
                .required(),
            FieldSpec::link("bank_charges_account", "Bank Charges Account")
                .options("Account")
                .required(),
            FieldSpec::column_break("column_break_15"),
            FieldSpec::link(
                "accounts_receivable_credit",
                "Accounts Receivable Credit Account",
            )
            .options("Account")
            .required(),
            FieldSpec::link(
                "accounts_receivable_discounted",
                "Accounts Receivable Discounted Account",
            )
            .options("Account")
            .required(),
            FieldSpec::link(
                "accounts_receivable_unpaid",
                "Accounts Receivable Unpaid Account",
            )
            .options("Account")
            .required(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Invoice Discounting")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(
        &mut self,
        discounted_invoices: &BTreeSet<String>,
        actual_outstanding: &BTreeMap<String, f64>,
    ) -> Result<(), InvoiceDiscountingError> {
        self.validate_mandatory()?;
        self.validate_invoices(discounted_invoices, actual_outstanding)?;
        self.calculate_total_amount();
        self.set_status(None, false);
        self.set_end_date();
        Ok(())
    }

    pub fn set_end_date(&mut self) {
        if let Some(start_date) = self.loan_start_date.as_deref() {
            if self.loan_period != 0 {
                self.loan_end_date = Some(add_days(start_date, self.loan_period));
            }
        }
    }

    pub fn validate_mandatory(&self) -> Result<(), InvoiceDiscountingError> {
        if self.docstatus == 1
            && (self
                .loan_start_date
                .as_deref()
                .unwrap_or_default()
                .is_empty()
                || self.loan_period == 0)
        {
            return Err(InvoiceDiscountingError::MissingLoanStartDateOrPeriod);
        }
        Ok(())
    }

    pub fn validate_invoices(
        &self,
        discounted_invoices: &BTreeSet<String>,
        actual_outstanding: &BTreeMap<String, f64>,
    ) -> Result<(), InvoiceDiscountingError> {
        for record in &self.invoices {
            if discounted_invoices.contains(&record.sales_invoice) {
                return Err(InvoiceDiscountingError::AlreadyDiscounted {
                    row: record.idx,
                    sales_invoice: record.sales_invoice.clone(),
                    parent: record.parent.clone(),
                });
            }

            let actual = actual_outstanding
                .get(&record.sales_invoice)
                .copied()
                .unwrap_or(0.0);
            if record.outstanding_amount > actual {
                return Err(InvoiceDiscountingError::OutstandingGreaterThanActual {
                    row: record.idx,
                    sales_invoice: record.sales_invoice.clone(),
                    actual_outstanding: actual,
                });
            }
        }
        Ok(())
    }

    pub fn calculate_total_amount(&mut self) {
        self.total_amount = self
            .invoices
            .iter()
            .map(|invoice| invoice.outstanding_amount)
            .sum();
    }

    pub fn set_status(
        &mut self,
        status: Option<InvoiceDiscountingStatus>,
        _cancel: bool,
    ) -> InvoiceDiscountingStatus {
        if let Some(status) = status {
            self.status = status;
        } else {
            self.status = match self.docstatus {
                1 => InvoiceDiscountingStatus::Sanctioned,
                2 => InvoiceDiscountingStatus::Cancelled,
                _ => InvoiceDiscountingStatus::Draft,
            };
        }
        self.status.clone()
    }

    pub fn update_sales_invoice_plan(
        &self,
        active_discounted_invoices: &BTreeSet<String>,
    ) -> Vec<(String, bool)> {
        self.invoices
            .iter()
            .map(|invoice| {
                let is_discounted = if self.docstatus == 1 {
                    true
                } else {
                    active_discounted_invoices.contains(&invoice.sales_invoice)
                };
                (invoice.sales_invoice.clone(), is_discounted)
            })
            .collect()
    }

    pub fn make_gl_entries_plan(
        &self,
        company_currency: &str,
        ar_credit_account_currency: &str,
        invoice_data: &[InvoiceDiscountingGlInvoice],
    ) -> Vec<GlEntryPlan> {
        let invoice_data = invoice_data
            .iter()
            .map(|invoice| (invoice.sales_invoice.as_str(), invoice))
            .collect::<BTreeMap<_, _>>();
        let mut entries = Vec::new();

        for invoice in &self.invoices {
            if invoice.outstanding_amount == 0.0 {
                continue;
            }
            let Some(inv) = invoice_data.get(invoice.sales_invoice.as_str()) else {
                continue;
            };
            let outstanding_in_company_currency =
                flt(invoice.outstanding_amount * inv.conversion_rate);

            entries.push(GlEntryPlan {
                account: inv.debit_to.clone(),
                party_type: Some("Customer".to_string()),
                party: Some(invoice.customer.clone()),
                against: Some(self.accounts_receivable_credit.clone()),
                credit: outstanding_in_company_currency,
                credit_in_account_currency: if inv.party_account_currency == company_currency {
                    outstanding_in_company_currency
                } else {
                    invoice.outstanding_amount
                },
                cost_center: inv.cost_center.clone(),
                against_voucher: Some(invoice.sales_invoice.clone()),
                against_voucher_type: Some("Sales Invoice".to_string()),
                account_currency: Some(inv.party_account_currency.clone()),
                dimensions: inv.dimensions.clone(),
                ..Default::default()
            });

            entries.push(GlEntryPlan {
                account: self.accounts_receivable_credit.clone(),
                party_type: Some("Customer".to_string()),
                party: Some(invoice.customer.clone()),
                against: Some(inv.debit_to.clone()),
                debit: outstanding_in_company_currency,
                debit_in_account_currency: if ar_credit_account_currency == company_currency {
                    outstanding_in_company_currency
                } else {
                    invoice.outstanding_amount
                },
                cost_center: inv.cost_center.clone(),
                against_voucher: Some(invoice.sales_invoice.clone()),
                against_voucher_type: Some("Sales Invoice".to_string()),
                account_currency: Some(ar_credit_account_currency.to_string()),
                dimensions: inv.dimensions.clone(),
                ..Default::default()
            });
        }

        entries
    }

    pub fn create_disbursement_entry_plan(&self, default_cost_center: &str) -> JournalEntryPlan {
        let mut accounts = vec![JournalAccountPlan {
            account: self.bank_account.clone(),
            debit_in_account_currency: flt(self.total_amount) - flt(self.bank_charges),
            cost_center: Some(default_cost_center.to_string()),
            ..Default::default()
        }];

        if self.bank_charges != 0.0 {
            accounts.push(JournalAccountPlan {
                account: self.bank_charges_account.clone(),
                debit_in_account_currency: flt(self.bank_charges),
                cost_center: Some(default_cost_center.to_string()),
                ..Default::default()
            });
        }

        accounts.push(JournalAccountPlan {
            account: self.short_term_loan.clone(),
            credit_in_account_currency: flt(self.total_amount),
            cost_center: Some(default_cost_center.to_string()),
            reference_type: Some(Self::DOCTYPE.to_string()),
            reference_name: self.name.clone(),
            ..Default::default()
        });

        for invoice in &self.invoices {
            accounts.push(JournalAccountPlan {
                account: self.accounts_receivable_discounted.clone(),
                debit_in_account_currency: flt(invoice.outstanding_amount),
                cost_center: Some(default_cost_center.to_string()),
                reference_type: Some(Self::DOCTYPE.to_string()),
                reference_name: self.name.clone(),
                party_type: Some("Customer".to_string()),
                party: Some(invoice.customer.clone()),
                ..Default::default()
            });
            accounts.push(JournalAccountPlan {
                account: self.accounts_receivable_credit.clone(),
                credit_in_account_currency: flt(invoice.outstanding_amount),
                cost_center: Some(default_cost_center.to_string()),
                reference_type: Some(Self::DOCTYPE.to_string()),
                reference_name: self.name.clone(),
                party_type: Some("Customer".to_string()),
                party: Some(invoice.customer.clone()),
                ..Default::default()
            });
        }

        JournalEntryPlan {
            voucher_type: "Journal Entry".to_string(),
            company: self.company.clone(),
            remark: format!(
                "Loan Disbursement entry against Invoice Discounting: {}",
                self.name.as_deref().unwrap_or_default()
            ),
            accounts,
        }
    }

    pub fn close_loan_plan(
        &self,
        default_cost_center: &str,
        today: &str,
        outstanding_amounts: &BTreeMap<String, f64>,
    ) -> JournalEntryPlan {
        let mut accounts = vec![
            JournalAccountPlan {
                account: self.short_term_loan.clone(),
                debit_in_account_currency: flt(self.total_amount),
                cost_center: Some(default_cost_center.to_string()),
                reference_type: Some(Self::DOCTYPE.to_string()),
                reference_name: self.name.clone(),
                ..Default::default()
            },
            JournalAccountPlan {
                account: self.bank_account.clone(),
                credit_in_account_currency: flt(self.total_amount),
                cost_center: Some(default_cost_center.to_string()),
                ..Default::default()
            },
        ];

        if self
            .loan_end_date
            .as_deref()
            .is_some_and(|loan_end_date| parse_date(loan_end_date) > parse_date(today))
        {
            for invoice in &self.invoices {
                let outstanding_amount = outstanding_amounts
                    .get(&invoice.sales_invoice)
                    .copied()
                    .unwrap_or(0.0);
                if outstanding_amount > 0.0 {
                    accounts.push(JournalAccountPlan {
                        account: self.accounts_receivable_discounted.clone(),
                        credit_in_account_currency: flt(outstanding_amount),
                        cost_center: Some(default_cost_center.to_string()),
                        reference_type: Some(Self::DOCTYPE.to_string()),
                        reference_name: self.name.clone(),
                        party_type: Some("Customer".to_string()),
                        party: Some(invoice.customer.clone()),
                        ..Default::default()
                    });
                    accounts.push(JournalAccountPlan {
                        account: self.accounts_receivable_unpaid.clone(),
                        debit_in_account_currency: flt(outstanding_amount),
                        cost_center: Some(default_cost_center.to_string()),
                        reference_type: Some(Self::DOCTYPE.to_string()),
                        reference_name: self.name.clone(),
                        party_type: Some("Customer".to_string()),
                        party: Some(invoice.customer.clone()),
                        ..Default::default()
                    });
                }
            }
        }

        JournalEntryPlan {
            voucher_type: "Journal Entry".to_string(),
            company: self.company.clone(),
            remark: format!(
                "Loan Settlement entry against Invoice Discounting: {}",
                self.name.as_deref().unwrap_or_default()
            ),
            accounts,
        }
    }
}

impl DocumentController for InvoiceDiscounting {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_submit", "on_cancel"]
    }
}

#[allow(clippy::too_many_arguments)]
pub fn get_invoices(
    invoices: &[SalesInvoiceCandidate],
    customer: Option<&str>,
    from_date: Option<&str>,
    to_date: Option<&str>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
    discounted_invoices: &BTreeSet<String>,
) -> Vec<SalesInvoiceCandidate> {
    invoices
        .iter()
        .filter(|invoice| invoice.docstatus == 1)
        .filter(|invoice| invoice.outstanding_amount > 0.0)
        .filter(|invoice| !discounted_invoices.contains(&invoice.sales_invoice))
        .filter(|invoice| customer.is_none_or(|customer| invoice.customer == customer))
        .filter(|invoice| {
            from_date.is_none_or(|from_date| invoice.posting_date.as_str() >= from_date)
        })
        .filter(|invoice| to_date.is_none_or(|to_date| invoice.posting_date.as_str() <= to_date))
        .filter(|invoice| min_amount.is_none_or(|min| invoice.base_grand_total >= min))
        .filter(|invoice| max_amount.is_none_or(|max| invoice.base_grand_total <= max))
        .cloned()
        .collect()
}

pub fn get_party_account_based_on_invoice_discounting(
    sales_invoice: &str,
    invoice_discounting: &[(String, String, String, InvoiceDiscountingStatus)],
) -> Option<String> {
    invoice_discounting
        .iter()
        .find(|(invoice, _, _, _)| invoice == sales_invoice)
        .and_then(|(_, discounted, unpaid, status)| match status {
            InvoiceDiscountingStatus::Disbursed => Some(discounted.clone()),
            InvoiceDiscountingStatus::Settled => Some(unpaid.clone()),
            _ => None,
        })
}

fn flt(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn add_days(date: &str, days: i64) -> String {
    civil_from_days(parse_date(date) + days)
}

fn parse_date(date: &str) -> i64 {
    let date_part = date.split_whitespace().next().unwrap_or_default();
    let parts = date_part
        .split('-')
        .filter_map(|part| part.parse::<i64>().ok())
        .collect::<Vec<_>>();
    if parts.len() != 3 {
        return 0;
    }
    days_from_civil(parts[0], parts[1], parts[2])
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month_prime = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month_prime + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn civil_from_days(days: i64) -> String {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}
