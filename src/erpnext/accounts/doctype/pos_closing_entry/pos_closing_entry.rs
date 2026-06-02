use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct POSClosingEntry {
    pub name: String,
    pub owner: String,
    pub amended_from: Option<String>,
    pub company: String,
    pub error_message: Option<String>,
    pub grand_total: f64,
    pub net_total: f64,
    pub payment_reconciliation: Vec<PaymentReconciliationRow>,
    pub period_end_date: Option<String>,
    pub period_start_date: Option<String>,
    pub pos_invoices: Vec<POSInvoiceReferenceRow>,
    pub pos_opening_entry: String,
    pub pos_profile: String,
    pub posting_date: Option<String>,
    pub posting_time: Option<String>,
    pub sales_invoices: Vec<SalesInvoiceReferenceRow>,
    pub status: String,
    pub taxes: Vec<TaxRow>,
    pub total_quantity: f64,
    pub total_taxes_and_charges: f64,
    pub user: String,
    pub invoice_type: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct POSInvoiceReferenceRow {
    pub idx: usize,
    pub pos_invoice: String,
    pub posting_date: Option<String>,
    pub grand_total: f64,
    pub customer: Option<String>,
    pub is_return: bool,
    pub return_against: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SalesInvoiceReferenceRow {
    pub idx: usize,
    pub sales_invoice: String,
    pub posting_date: Option<String>,
    pub grand_total: f64,
    pub customer: Option<String>,
    pub is_return: bool,
    pub return_against: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliationRow {
    pub mode_of_payment: String,
    pub opening_amount: f64,
    pub expected_amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaxRow {
    pub account_head: String,
    pub amount: f64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSInvoiceRecord {
    pub consolidated_invoice: Option<String>,
    pub pos_profile: String,
    pub docstatus: i32,
    pub owner: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesInvoiceRecord {
    pub pos_profile: String,
    pub docstatus: i32,
    pub is_pos: bool,
    pub owner: String,
    pub is_created_using_pos: bool,
    pub is_consolidated: bool,
    pub pos_closing_entry: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvoiceCandidate {
    pub name: String,
    pub doctype: String,
    pub customer: String,
    pub posting_date: String,
    pub grand_total: f64,
    pub net_total: f64,
    pub total_qty: f64,
    pub total_taxes_and_charges: f64,
    pub change_amount: f64,
    pub account_for_change_amount: Option<String>,
    pub is_return: bool,
    pub return_against: Option<String>,
    pub timestamp: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvoicePayment {
    pub parent: String,
    pub parenttype: String,
    pub mode_of_payment: String,
    pub account: String,
    pub amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentSummary {
    pub mode_of_payment: String,
    pub account: String,
    pub amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvoiceTax {
    pub parent: String,
    pub parenttype: String,
    pub account_head: String,
    pub tax_amount_after_discount_amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaxSummary {
    pub account_head: String,
    pub tax_amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvoicesData {
    pub invoices: Vec<InvoiceCandidate>,
    pub payments: Vec<PaymentSummary>,
    pub taxes: Vec<TaxSummary>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OpeningEntrySnapshot {
    pub name: String,
    pub period_start_date: String,
    pub period_end_date: String,
    pub pos_profile: String,
    pub user: String,
    pub company: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LifecyclePlan {
    pub consolidate: bool,
    pub unconsolidate: bool,
    pub realtime_event: String,
    pub realtime_docname: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BuildInvoiceQueryPlan {
    pub invoice_doctype: String,
    pub user: String,
    pub pos_profile: String,
    pub start: String,
    pub end: String,
    pub require_unconsolidated_pos_invoice: bool,
    pub require_created_using_pos: bool,
    pub require_no_pos_closing_entry: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum POSClosingEntryError {
    InvalidOpeningEntry,
    PosInvoicesNotAllowedForSalesInvoiceMode,
    DuplicatePosInvoices(Vec<String>),
    InvalidPosInvoices(Vec<String>),
    DuplicateSalesInvoices(Vec<String>),
    InvalidSalesInvoices(Vec<String>),
    CannotCancelOpenProfile,
}

impl POSClosingEntry {
    pub const DOCTYPE: &'static str = "POS Closing Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "POS-CLO-.YYYY.-.#####";
    pub const FIELD_ORDER: [&'static str; 31] = [
        "period_details_section",
        "period_start_date",
        "period_end_date",
        "column_break_3",
        "posting_date",
        "posting_time",
        "pos_opening_entry",
        "status",
        "section_break_5",
        "company",
        "column_break_7",
        "pos_profile",
        "user",
        "section_break_12",
        "pos_invoices",
        "sales_invoices",
        "taxes_and_charges_section",
        "taxes",
        "section_break_13",
        "column_break_16",
        "total_quantity",
        "column_break_ywgl",
        "net_total",
        "total_taxes_and_charges",
        "grand_total",
        "section_break_11",
        "payment_reconciliation",
        "failure_description_section",
        "error_message",
        "section_break_14",
        "amended_from",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("period_details_section").label("Period Details"),
            FieldSpec::datetime("period_start_date", "Period Start Date")
                .fetch_from("pos_opening_entry.period_start_date")
                .in_list_view()
                .read_only()
                .required(),
            FieldSpec::datetime("period_end_date", "Period End Date")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::time("posting_time", "Posting Time")
                .default("Now")
                .no_copy()
                .required(),
            FieldSpec::link("pos_opening_entry", "POS Opening Entry")
                .options("POS Opening Entry")
                .print_hide()
                .required(),
            FieldSpec::select("status", "Status")
                .options("Draft\nSubmitted\nQueued\nFailed\nCancelled")
                .default("Draft")
                .allow_on_submit()
                .hidden()
                .print_hide()
                .read_only(),
            FieldSpec::section_break("section_break_5").label("User Details"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .fetch_from("pos_opening_entry.company")
                .fetch_if_empty()
                .read_only()
                .required(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("pos_profile", "POS Profile")
                .options("POS Profile")
                .fetch_from("pos_opening_entry.pos_profile")
                .fetch_if_empty()
                .in_list_view()
                .read_only()
                .required(),
            FieldSpec::link("user", "Cashier")
                .options("User")
                .fetch_from("pos_opening_entry.user")
                .read_only()
                .required(),
            FieldSpec::section_break("section_break_12").label("Linked Invoices"),
            FieldSpec::table("pos_invoices", "POS Transactions")
                .options("POS Invoice Reference")
                .print_hide()
                .read_only(),
            FieldSpec::table("sales_invoices", "Sales Invoice Transactions")
                .options("Sales Invoice Reference")
                .print_hide()
                .read_only(),
            FieldSpec::section_break("taxes_and_charges_section")
                .label("Taxes and Charges")
                .collapsible(),
            FieldSpec::table("taxes", "Taxes")
                .options("POS Closing Entry Taxes")
                .read_only(),
            FieldSpec::section_break("section_break_13").label("Totals"),
            FieldSpec::column_break("column_break_16"),
            FieldSpec::float("total_quantity", "Total Quantity").read_only(),
            FieldSpec::column_break("column_break_ywgl"),
            FieldSpec::currency("net_total", "Net Total")
                .default("0")
                .read_only(),
            FieldSpec::currency("total_taxes_and_charges", "Total Taxes and Charges").read_only(),
            FieldSpec::currency("grand_total", "Grand Total")
                .default("0")
                .read_only(),
            FieldSpec::section_break("section_break_11").label("Modes of Payment"),
            FieldSpec::table("payment_reconciliation", "Payment Reconciliation")
                .options("POS Closing Entry Detail"),
            FieldSpec::section_break("failure_description_section")
                .label("Failure Description")
                .depends_on("error_message")
                .collapsible(),
            FieldSpec::small_text("error_message", "Error")
                .depends_on("error_message")
                .read_only(),
            FieldSpec::section_break("section_break_14"),
            FieldSpec::link("amended_from", "Amended From")
                .options("POS Closing Entry")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(
        &mut self,
        opening_entry_status: &str,
        invoice_type: &str,
        nowdate: &str,
        nowtime: &str,
        pos_invoice_records: &BTreeMap<String, POSInvoiceRecord>,
        sales_invoice_records: &BTreeMap<String, SalesInvoiceRecord>,
    ) -> Result<(), POSClosingEntryError> {
        self.set_posting_date_and_time(nowdate, nowtime);
        self.invoice_type = invoice_type.to_string();
        self.validate_pos_opening_entry(opening_entry_status)?;
        self.validate_invoice_mode(pos_invoice_records, sales_invoice_records)
    }

    pub fn set_posting_date_and_time(&mut self, nowdate: &str, nowtime: &str) {
        if self.posting_date.is_some() {
            self.posting_date = Some(nowdate.to_string());
        }
        if self.posting_time.is_some() {
            self.posting_time = Some(nowtime.to_string());
        }
    }

    pub fn validate_pos_opening_entry(
        &self,
        opening_entry_status: &str,
    ) -> Result<(), POSClosingEntryError> {
        if opening_entry_status != "Open" {
            return Err(POSClosingEntryError::InvalidOpeningEntry);
        }
        Ok(())
    }

    pub fn validate_invoice_mode(
        &self,
        pos_invoice_records: &BTreeMap<String, POSInvoiceRecord>,
        sales_invoice_records: &BTreeMap<String, SalesInvoiceRecord>,
    ) -> Result<(), POSClosingEntryError> {
        if self.invoice_type == "POS Invoice" {
            self.validate_duplicate_pos_invoices()?;
            self.validate_pos_invoices(pos_invoice_records)?;
        }
        if self.invoice_type == "Sales Invoice" && !self.pos_invoices.is_empty() {
            return Err(POSClosingEntryError::PosInvoicesNotAllowedForSalesInvoiceMode);
        }
        self.validate_duplicate_sales_invoices()?;
        self.validate_sales_invoices(sales_invoice_records)
    }

    pub fn validate_duplicate_pos_invoices(&self) -> Result<(), POSClosingEntryError> {
        let messages = duplicate_messages(
            self.pos_invoices
                .iter()
                .map(|row| (row.pos_invoice.as_str(), row.idx)),
        );
        if messages.is_empty() {
            Ok(())
        } else {
            Err(POSClosingEntryError::DuplicatePosInvoices(messages))
        }
    }

    pub fn validate_pos_invoices(
        &self,
        records: &BTreeMap<String, POSInvoiceRecord>,
    ) -> Result<(), POSClosingEntryError> {
        let mut errors = Vec::new();
        for row in &self.pos_invoices {
            let Some(invoice) = records.get(&row.pos_invoice) else {
                continue;
            };
            if invoice.consolidated_invoice.is_some() {
                errors.push(format!(
                    "Row #{}: POS Invoice is already consolidated",
                    row.idx
                ));
                continue;
            }
            if invoice.pos_profile != self.pos_profile {
                errors.push(format!(
                    "Row #{}: POS Profile doesn't match {}",
                    row.idx, self.pos_profile
                ));
            }
            if invoice.docstatus != 1 {
                errors.push(format!("Row #{}: POS Invoice is not submitted", row.idx));
            }
            if invoice.owner != self.user {
                errors.push(format!(
                    "Row #{}: POS Invoice isn't created by user {}",
                    row.idx, self.owner
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(POSClosingEntryError::InvalidPosInvoices(errors))
        }
    }

    pub fn validate_duplicate_sales_invoices(&self) -> Result<(), POSClosingEntryError> {
        let messages = duplicate_messages(
            self.sales_invoices
                .iter()
                .map(|row| (row.sales_invoice.as_str(), row.idx)),
        );
        if messages.is_empty() {
            Ok(())
        } else {
            Err(POSClosingEntryError::DuplicateSalesInvoices(messages))
        }
    }

    pub fn validate_sales_invoices(
        &self,
        records: &BTreeMap<String, SalesInvoiceRecord>,
    ) -> Result<(), POSClosingEntryError> {
        let mut errors = Vec::new();
        for row in &self.sales_invoices {
            let Some(invoice) = records.get(&row.sales_invoice) else {
                continue;
            };
            if invoice.pos_closing_entry.is_some() {
                errors.push(format!(
                    "Row #{}: Sales Invoice is already consolidated",
                    row.idx
                ));
                continue;
            }
            if !invoice.is_pos {
                errors.push(format!(
                    "Row #{}: Sales Invoice does not have Payments",
                    row.idx
                ));
            }
            if !invoice.is_created_using_pos {
                errors.push(format!(
                    "Row #{}: Sales Invoice is not created using POS",
                    row.idx
                ));
            }
            if invoice.pos_profile != self.pos_profile {
                errors.push(format!(
                    "Row #{}: POS Profile doesn't match {}",
                    row.idx, self.pos_profile
                ));
            }
            if invoice.docstatus != 1 {
                errors.push(format!("Row #{}: Sales Invoice is not submitted", row.idx));
            }
            if invoice.owner != self.user {
                errors.push(format!(
                    "Row #{}: Sales Invoice isn't created by user {}",
                    row.idx, self.owner
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(POSClosingEntryError::InvalidSalesInvoices(errors))
        }
    }

    pub fn on_submit_plan(&self) -> LifecyclePlan {
        LifecyclePlan {
            consolidate: true,
            realtime_event: format!("poe_{}", self.pos_opening_entry),
            realtime_docname: format!("POS Opening Entry/{}", self.pos_opening_entry),
            ..Default::default()
        }
    }

    pub fn before_cancel_plan(
        &self,
        open_entry_for_profile_exists: bool,
    ) -> Result<(), POSClosingEntryError> {
        if open_entry_for_profile_exists {
            Err(POSClosingEntryError::CannotCancelOpenProfile)
        } else {
            Ok(())
        }
    }

    pub fn on_cancel_plan(&self) -> LifecyclePlan {
        LifecyclePlan {
            unconsolidate: true,
            ..Default::default()
        }
    }

    pub fn retry_plan(&self) -> LifecyclePlan {
        LifecyclePlan {
            consolidate: true,
            ..Default::default()
        }
    }

    pub fn update_opening_entry_plan(&self, for_cancel: bool) -> (String, Option<String>) {
        (
            self.pos_opening_entry.clone(),
            (!for_cancel).then(|| self.name.clone()),
        )
    }

    pub fn update_sales_invoices_closing_entry_plan(
        &self,
        cancel: bool,
    ) -> Vec<(String, Option<String>)> {
        self.sales_invoices
            .iter()
            .map(|row| {
                (
                    row.sales_invoice.clone(),
                    (!cancel).then(|| self.name.clone()),
                )
            })
            .collect()
    }
}

impl DocumentController for POSClosingEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "validate",
            "on_submit",
            "before_cancel",
            "on_cancel",
            "retry",
        ]
    }
}

pub fn get_cashiers(users: &[String]) -> Vec<String> {
    users.to_vec()
}

pub fn get_invoices(
    invoice_type: &str,
    sales_invoices: Vec<InvoiceCandidate>,
    pos_invoices: Vec<InvoiceCandidate>,
    payments: &[InvoicePayment],
    taxes: &[InvoiceTax],
) -> InvoicesData {
    let mut invoices = sales_invoices;
    if invoice_type == "POS Invoice" {
        invoices.extend(pos_invoices);
    }
    invoices.sort_by(|left, right| left.timestamp.cmp(&right.timestamp));
    InvoicesData {
        payments: get_payments(&invoices, payments),
        taxes: get_taxes(&invoices, taxes),
        invoices,
    }
}

pub fn get_payments(
    invoices: &[InvoiceCandidate],
    payments: &[InvoicePayment],
) -> Vec<PaymentSummary> {
    if invoices.is_empty() {
        return Vec::new();
    }
    let invoice_names = invoices
        .iter()
        .map(|invoice| invoice.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut grouped = BTreeMap::<(String, String), f64>::new();
    for payment in payments {
        if matches!(payment.parenttype.as_str(), "Sales Invoice" | "POS Invoice")
            && invoice_names.contains(payment.parent.as_str())
        {
            *grouped
                .entry((payment.mode_of_payment.clone(), payment.account.clone()))
                .or_default() += payment.amount;
        }
    }

    let mut change_amount_by_account = BTreeMap::<String, f64>::new();
    for invoice in invoices {
        if let Some(account) = invoice.account_for_change_amount.as_deref() {
            *change_amount_by_account
                .entry(account.to_string())
                .or_default() += invoice.change_amount;
        }
    }

    grouped
        .into_iter()
        .map(|((mode_of_payment, account), amount)| PaymentSummary {
            amount: amount
                - change_amount_by_account
                    .get(&account)
                    .copied()
                    .unwrap_or_default(),
            mode_of_payment,
            account,
        })
        .collect()
}

pub fn get_taxes(invoices: &[InvoiceCandidate], taxes: &[InvoiceTax]) -> Vec<TaxSummary> {
    if invoices.is_empty() {
        return Vec::new();
    }
    let invoice_names = invoices
        .iter()
        .map(|invoice| invoice.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut grouped = BTreeMap::<String, f64>::new();
    for tax in taxes {
        if matches!(tax.parenttype.as_str(), "Sales Invoice" | "POS Invoice")
            && invoice_names.contains(tax.parent.as_str())
        {
            *grouped.entry(tax.account_head.clone()).or_default() +=
                tax.tax_amount_after_discount_amount;
        }
    }
    grouped
        .into_iter()
        .map(|(account_head, tax_amount)| TaxSummary {
            account_head,
            tax_amount,
        })
        .collect()
}

pub fn make_closing_entry_from_opening(
    opening_entry: &OpeningEntrySnapshot,
    current_datetime: &str,
    invoices: Vec<InvoiceCandidate>,
    payments: Vec<PaymentSummary>,
    taxes: Vec<TaxSummary>,
) -> POSClosingEntry {
    let mut closing_entry = POSClosingEntry {
        pos_opening_entry: opening_entry.name.clone(),
        period_start_date: Some(opening_entry.period_start_date.clone()),
        period_end_date: Some(current_datetime.to_string()),
        pos_profile: opening_entry.pos_profile.clone(),
        user: opening_entry.user.clone(),
        company: opening_entry.company.clone(),
        grand_total: 0.0,
        net_total: 0.0,
        total_quantity: 0.0,
        total_taxes_and_charges: 0.0,
        payment_reconciliation: payments
            .into_iter()
            .map(|payment| PaymentReconciliationRow {
                mode_of_payment: payment.mode_of_payment,
                opening_amount: 0.0,
                expected_amount: payment.amount,
            })
            .collect(),
        taxes: taxes
            .into_iter()
            .map(|tax| TaxRow {
                account_head: tax.account_head,
                amount: tax.tax_amount,
            })
            .collect(),
        ..Default::default()
    };

    for invoice in invoices {
        let posting_date = Some(invoice.posting_date.clone());
        let customer = Some(invoice.customer.clone());
        if invoice.doctype == "POS Invoice" {
            closing_entry.pos_invoices.push(POSInvoiceReferenceRow {
                pos_invoice: invoice.name.clone(),
                posting_date,
                grand_total: invoice.grand_total,
                customer,
                is_return: invoice.is_return,
                return_against: invoice.return_against.clone(),
                ..Default::default()
            });
        } else {
            closing_entry.sales_invoices.push(SalesInvoiceReferenceRow {
                sales_invoice: invoice.name.clone(),
                posting_date,
                grand_total: invoice.grand_total,
                customer,
                is_return: invoice.is_return,
                return_against: invoice.return_against.clone(),
                ..Default::default()
            });
        }
        closing_entry.grand_total += invoice.grand_total;
        closing_entry.net_total += invoice.net_total;
        closing_entry.total_quantity += invoice.total_qty;
        closing_entry.total_taxes_and_charges += invoice.total_taxes_and_charges;
    }

    closing_entry
}

pub fn build_invoice_query_plan(
    invoice_doctype: &str,
    user: &str,
    pos_profile: &str,
    start: &str,
    end: &str,
) -> BuildInvoiceQueryPlan {
    BuildInvoiceQueryPlan {
        invoice_doctype: invoice_doctype.to_string(),
        user: user.to_string(),
        pos_profile: pos_profile.to_string(),
        start: start.to_string(),
        end: end.to_string(),
        require_unconsolidated_pos_invoice: invoice_doctype == "POS Invoice",
        require_created_using_pos: invoice_doctype != "POS Invoice",
        require_no_pos_closing_entry: invoice_doctype != "POS Invoice",
    }
}

fn duplicate_messages<'a>(rows: impl Iterator<Item = (&'a str, usize)>) -> Vec<String> {
    let mut occurrences = BTreeMap::<String, Vec<usize>>::new();
    for (name, idx) in rows {
        occurrences.entry(name.to_string()).or_default().push(idx);
    }
    occurrences
        .into_iter()
        .filter_map(|(name, indexes)| {
            (indexes.len() > 1)
                .then(|| format!("{name} is added multiple times on rows: {indexes:?}"))
        })
        .collect()
}
