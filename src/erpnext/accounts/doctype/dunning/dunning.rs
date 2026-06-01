use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DunningStatus {
    Draft,
    Resolved,
    Unresolved,
    Cancelled,
}

impl Default for DunningStatus {
    fn default() -> Self {
        Self::Unresolved
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dunning {
    pub name: Option<String>,
    pub customer: String,
    pub customer_name: Option<String>,
    pub company: String,
    pub posting_date: String,
    pub posting_time: Option<String>,
    pub status: DunningStatus,
    pub currency: Option<String>,
    pub conversion_rate: f64,
    pub dunning_type: Option<String>,
    pub rate_of_interest: f64,
    pub overdue_payments: Vec<OverduePayment>,
    pub total_interest: f64,
    pub dunning_fee: f64,
    pub dunning_amount: f64,
    pub base_dunning_amount: f64,
    pub total_outstanding: f64,
    pub grand_total: f64,
    pub language: Option<String>,
    pub body_text: Option<String>,
    pub letter_head: Option<String>,
    pub closing_text: Option<String>,
    pub income_account: Option<String>,
    pub cost_center: Option<String>,
    pub amended_from: Option<String>,
    pub customer_address: Option<String>,
    pub address_display: Option<String>,
    pub contact_person: Option<String>,
    pub contact_display: Option<String>,
    pub contact_mobile: Option<String>,
    pub contact_email: Option<String>,
    pub company_address: Option<String>,
    pub company_address_display: Option<String>,
    pub ignore_linked_doctypes: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OverduePayment {
    pub sales_invoice: String,
    pub payment_schedule: Option<String>,
    pub parent: Option<String>,
    pub due_date: String,
    pub outstanding: f64,
    pub overdue_days: i64,
    pub interest: f64,
    pub dunning_level: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyDetails {
    pub customer_address: Option<String>,
    pub address_display: Option<String>,
    pub company_address: Option<String>,
    pub contact_person: Option<String>,
    pub contact_display: Option<String>,
    pub contact_mobile: Option<String>,
    pub company_address_display: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DunningError {
    CurrencyMismatch {
        sales_invoice: String,
        invoice_currency: Option<String>,
        dunning_currency: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesInvoiceSnapshot {
    pub doctype: String,
    pub name: String,
    pub is_return: bool,
    pub outstanding_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InvoiceOutstanding {
    pub sales_invoice: String,
    pub outstanding_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentScheduleOutstanding {
    pub payment_schedule: String,
    pub outstanding: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DunningStatusUpdatePlan {
    pub dunning_name: String,
    pub new_status: DunningStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DunningLetterText {
    pub parent: String,
    pub body_text: String,
    pub closing_text: String,
    pub language: String,
    pub is_default_language: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedDunningLetterText {
    pub body_text: String,
    pub closing_text: String,
    pub language: String,
}

impl Dunning {
    pub const DOCTYPE: &'static str = "Dunning";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const IS_SUBMITTABLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 54] = [
        "naming_series",
        "customer",
        "customer_name",
        "column_break_3",
        "company",
        "posting_date",
        "posting_time",
        "status",
        "section_break_9",
        "currency",
        "column_break_11",
        "conversion_rate",
        "section_break_6",
        "dunning_type",
        "column_break_8",
        "rate_of_interest",
        "section_break_12",
        "overdue_payments",
        "section_break_28",
        "total_interest",
        "dunning_fee",
        "column_break_17",
        "dunning_amount",
        "base_dunning_amount",
        "section_break_32",
        "spacer",
        "column_break_33",
        "total_outstanding",
        "grand_total",
        "printing_settings_section",
        "language",
        "body_text",
        "column_break_22",
        "letter_head",
        "closing_text",
        "accounting_details_section",
        "income_account",
        "column_break_48",
        "cost_center",
        "amended_from",
        "address_and_contact_tab",
        "address_and_contact_section",
        "customer_address",
        "address_display",
        "column_break_vodj",
        "contact_person",
        "contact_display",
        "contact_mobile",
        "contact_email",
        "section_break_xban",
        "column_break_16",
        "company_address",
        "company_address_display",
        "column_break_lqmf",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("naming_series", "Series")
                .options("DUNN-.MM.-.YY.-")
                .default("DUNN-.MM.-.YY.-")
                .print_hide(),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .required(),
            FieldSpec::data("customer_name", "Customer Name")
                .fetch_from("customer.customer_name")
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::date("posting_date", "Date")
                .default("Today")
                .required(),
            FieldSpec::time("posting_time", "Posting Time"),
            FieldSpec::select("status", "Status")
                .options("Draft\nResolved\nUnresolved\nCancelled")
                .default("Unresolved")
                .read_only()
                .in_standard_filter()
                .allow_on_submit(),
            FieldSpec::section_break("section_break_9"),
            FieldSpec::link("currency", "Currency").options("Currency"),
            FieldSpec::column_break("column_break_11"),
            FieldSpec::float("conversion_rate", "Conversion Rate"),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::link("dunning_type", "Dunning Type")
                .options("Dunning Type")
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::float("rate_of_interest", "Rate of Interest (%) Yearly")
                .default("0")
                .fetch_from("dunning_type.rate_of_interest")
                .fetch_if_empty(),
            FieldSpec::section_break("section_break_12"),
            FieldSpec::table("overdue_payments", "Overdue Payments").options("Overdue Payment"),
            FieldSpec::section_break("section_break_28"),
            FieldSpec::currency("total_interest", "Total Interest")
                .options("currency")
                .default("0")
                .precision("2")
                .read_only(),
            FieldSpec::currency("dunning_fee", "Dunning Fee")
                .options("currency")
                .default("0")
                .precision("2")
                .fetch_from("dunning_type.dunning_fee")
                .fetch_if_empty(),
            FieldSpec::column_break("column_break_17"),
            FieldSpec::currency("dunning_amount", "Dunning Amount")
                .options("currency")
                .default("0")
                .read_only(),
            FieldSpec::currency("base_dunning_amount", "Dunning Amount (Company Currency)")
                .options("Company:company:default_currency")
                .default("0")
                .read_only(),
            FieldSpec::section_break("section_break_32"),
            FieldSpec::data("spacer", "")
                .hidden()
                .print_hide()
                .read_only()
                .report_hide(),
            FieldSpec::column_break("column_break_33"),
            FieldSpec::currency("total_outstanding", "Total Outstanding")
                .options("currency")
                .read_only(),
            FieldSpec::currency("grand_total", "Grand Total")
                .options("currency")
                .default("0")
                .precision("2")
                .read_only(),
            FieldSpec::section_break("printing_settings_section")
                .label("Printing Settings")
                .collapsible(),
            FieldSpec::link("language", "Print Language")
                .options("Language")
                .print_hide(),
            FieldSpec::text_editor("body_text", "Body Text"),
            FieldSpec::column_break("column_break_22"),
            FieldSpec::link("letter_head", "Letter Head")
                .options("Letter Head")
                .print_hide(),
            FieldSpec::text_editor("closing_text", "Closing Text"),
            FieldSpec::section_break("accounting_details_section")
                .label("Accounting Details")
                .collapsible(),
            FieldSpec::link("income_account", "Income Account")
                .options("Account")
                .fetch_from("dunning_type.income_account")
                .description("For dunning fee and interest")
                .print_hide(),
            FieldSpec::column_break("column_break_48"),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .fetch_from("dunning_type.cost_center")
                .print_hide(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Dunning")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::tab_break("address_and_contact_tab", "Address & Contact"),
            FieldSpec::section_break("address_and_contact_section"),
            FieldSpec::link("customer_address", "Customer Address")
                .options("Address")
                .print_hide(),
            FieldSpec::text_editor("address_display", "Address").read_only(),
            FieldSpec::column_break("column_break_vodj"),
            FieldSpec::link("contact_person", "Contact Person")
                .options("Contact")
                .print_hide(),
            FieldSpec::small_text("contact_display", "Contact").read_only(),
            FieldSpec::small_text("contact_mobile", "Mobile No")
                .options("Phone")
                .read_only(),
            FieldSpec::data("contact_email", "Contact Email")
                .options("Email")
                .read_only(),
            FieldSpec::section_break("section_break_xban"),
            FieldSpec::column_break("column_break_16"),
            FieldSpec::link("company_address", "Company Address")
                .options("Address")
                .print_hide(),
            FieldSpec::text_editor("company_address_display", "Company Address Display")
                .read_only(),
            FieldSpec::column_break("column_break_lqmf"),
        ]
    }

    pub fn validate(
        &mut self,
        invoice_currencies: &BTreeMap<String, String>,
        party_details: PartyDetails,
        past_dunning_counts: &BTreeMap<String, usize>,
    ) -> Result<(), DunningError> {
        self.validate_same_currency(invoice_currencies)?;
        self.validate_overdue_payments();
        self.validate_totals();
        self.set_party_details(party_details);
        self.set_dunning_level(past_dunning_counts);
        Ok(())
    }

    pub fn validate_same_currency(
        &self,
        invoice_currencies: &BTreeMap<String, String>,
    ) -> Result<(), DunningError> {
        for row in &self.overdue_payments {
            let invoice_currency = invoice_currencies.get(&row.sales_invoice).cloned();
            if invoice_currency != self.currency {
                return Err(DunningError::CurrencyMismatch {
                    sales_invoice: row.sales_invoice.clone(),
                    invoice_currency,
                    dunning_currency: self.currency.clone(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_overdue_payments(&mut self) {
        let daily_interest = self.rate_of_interest / 100.0 / 365.0;
        let posting_date = parse_date(&self.posting_date);

        for row in &mut self.overdue_payments {
            row.overdue_days = posting_date - parse_date(&row.due_date);
            row.interest = row.outstanding * daily_interest * row.overdue_days as f64;
        }
    }

    pub fn validate_totals(&mut self) {
        self.total_outstanding = self
            .overdue_payments
            .iter()
            .map(|row| row.outstanding)
            .sum();
        self.total_interest = self.overdue_payments.iter().map(|row| row.interest).sum();
        self.dunning_amount = self.total_interest + self.dunning_fee;
        self.base_dunning_amount = self.dunning_amount * self.conversion_rate;
        self.grand_total = self.total_outstanding + self.dunning_amount;
    }

    pub fn set_party_details(&mut self, party_details: PartyDetails) {
        self.customer_address = party_details.customer_address;
        self.address_display = party_details.address_display;
        self.company_address = party_details.company_address;
        self.contact_person = party_details.contact_person;
        self.contact_display = party_details.contact_display;
        self.contact_mobile = party_details.contact_mobile;
        self.company_address_display = party_details.company_address_display;
    }

    pub fn set_dunning_level(&mut self, past_dunning_counts: &BTreeMap<String, usize>) {
        for row in &mut self.overdue_payments {
            let count = row
                .payment_schedule
                .as_ref()
                .and_then(|payment_schedule| past_dunning_counts.get(payment_schedule))
                .copied()
                .unwrap_or(0);
            row.dunning_level = count + 1;
        }
    }

    pub fn on_cancel(&mut self) {
        self.ignore_linked_doctypes = [
            "GL Entry",
            "Stock Ledger Entry",
            "Repost Item Valuation",
            "Repost Payment Ledger",
            "Repost Payment Ledger Items",
            "Repost Accounting Ledger",
            "Repost Accounting Ledger Items",
            "Unreconcile Payment",
            "Unreconcile Payment Entries",
            "Payment Ledger Entry",
            "Serial and Batch Bundle",
        ]
        .iter()
        .map(|doctype| (*doctype).to_string())
        .collect();
    }
}

impl DocumentController for Dunning {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_cancel"]
    }
}

pub fn update_linked_dunnings_plan(
    doc: &SalesInvoiceSnapshot,
    previous_outstanding_amount: f64,
    linked_dunnings: &[Dunning],
    invoice_outstandings: &[InvoiceOutstanding],
    payment_schedule_outstandings: &[PaymentScheduleOutstanding],
) -> Vec<DunningStatusUpdatePlan> {
    if doc.doctype != "Sales Invoice"
        || doc.is_return
        || previous_outstanding_amount == doc.outstanding_amount
    {
        return Vec::new();
    }

    let to_resolve = doc.outstanding_amount < previous_outstanding_amount;
    let state = if to_resolve {
        DunningStatus::Unresolved
    } else {
        DunningStatus::Resolved
    };
    let invoice_outstandings = invoice_outstandings
        .iter()
        .map(|row| (row.sales_invoice.as_str(), row.outstanding_amount))
        .collect::<BTreeMap<_, _>>();
    let payment_schedule_outstandings = payment_schedule_outstandings
        .iter()
        .map(|row| (row.payment_schedule.as_str(), row.outstanding))
        .collect::<BTreeMap<_, _>>();
    let mut plans = Vec::new();

    for dunning in linked_dunnings
        .iter()
        .filter(|dunning| dunning.status == state)
    {
        let has_outstanding = dunning.overdue_payments.iter().any(|row| {
            let invoice_outstanding = invoice_outstandings
                .get(row.sales_invoice.as_str())
                .copied()
                .unwrap_or(0.0);
            let ps_outstanding = row
                .payment_schedule
                .as_deref()
                .and_then(|payment_schedule| payment_schedule_outstandings.get(payment_schedule))
                .copied()
                .unwrap_or(0.0);
            invoice_outstanding > 0.0 && ps_outstanding > 0.0
        });
        let new_status = if has_outstanding {
            DunningStatus::Unresolved
        } else {
            DunningStatus::Resolved
        };

        if dunning.status != new_status {
            if let Some(dunning_name) = dunning.name.clone() {
                plans.push(DunningStatusUpdatePlan {
                    dunning_name,
                    new_status,
                });
            }
        }
    }

    plans
}

pub fn get_dunning_letter_text(
    dunning_type: &str,
    doc: &BTreeMap<String, String>,
    language: Option<&str>,
    letter_texts: &[DunningLetterText],
) -> Option<RenderedDunningLetterText> {
    let selected_language = language.or_else(|| doc.get("language").map(String::as_str));
    let letter_text = selected_language
        .and_then(|language| {
            letter_texts
                .iter()
                .find(|row| row.parent == dunning_type && row.language == language)
        })
        .or_else(|| {
            letter_texts
                .iter()
                .find(|row| row.parent == dunning_type && row.is_default_language)
        })?;

    Some(RenderedDunningLetterText {
        body_text: render_template(&letter_text.body_text, doc),
        closing_text: render_template(&letter_text.closing_text, doc),
        language: letter_text.language.clone(),
    })
}

fn render_template(template: &str, context: &BTreeMap<String, String>) -> String {
    let mut rendered = template.to_string();
    for (key, value) in context {
        rendered = rendered.replace(&format!("{{{{ {key} }}}}"), value);
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
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
