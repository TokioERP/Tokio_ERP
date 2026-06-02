use std::collections::HashMap;

use crate::erpnext::accounts::doctype::payment_order_reference::payment_order_reference::PaymentOrderReference;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentOrder {
    pub name: Option<String>,
    pub naming_series: String,
    pub company: String,
    pub payment_order_type: String,
    pub party: Option<String>,
    pub posting_date: Option<String>,
    pub company_bank: Option<String>,
    pub company_bank_account: Option<String>,
    pub account: Option<String>,
    pub references: Vec<PaymentOrderReference>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentStatusUpdate {
    pub doctype: String,
    pub name: String,
    pub field: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryPlan {
    pub payment_order: String,
    pub posting_date: String,
    pub voucher_type: String,
    pub accounts: Vec<JournalEntryAccountPlan>,
    pub ignore_mandatory: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntryAccountPlan {
    pub account: String,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub reference_type: Option<String>,
    pub reference_name: Option<String>,
}

impl Default for PaymentOrder {
    fn default() -> Self {
        Self {
            name: None,
            naming_series: "PMO-".to_string(),
            company: String::new(),
            payment_order_type: String::new(),
            party: None,
            posting_date: None,
            company_bank: None,
            company_bank_account: None,
            account: None,
            references: Vec::new(),
            amended_from: None,
        }
    }
}

impl PaymentOrder {
    pub const DOCTYPE: &'static str = "Payment Order";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 12] = [
        "naming_series",
        "company",
        "payment_order_type",
        "party",
        "column_break_2",
        "posting_date",
        "company_bank",
        "company_bank_account",
        "account",
        "section_break_5",
        "references",
        "amended_from",
    ];
    pub const AUTONAME: &'static str = "naming_series:";
    pub const IS_SUBMITTABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("naming_series", "Series")
                .default("PMO-")
                .options("PMO-")
                .no_copy()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("party", "Supplier")
                .options("Supplier")
                .depends_on("eval: doc.payment_order_type=='Payment Request';")
                .in_list_view(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("references", "Payment Order Reference")
                .options("Payment Order Reference")
                .required(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Payment Order")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::select("payment_order_type", "Payment Order Type")
                .options("\nPayment Request\nPayment Entry")
                .read_only()
                .required(),
            FieldSpec::link("company_bank_account", "Company Bank Account")
                .options("Bank Account")
                .required(),
            FieldSpec::link("company_bank", "Bank")
                .options("Bank")
                .depends_on("company_bank_account")
                .fetch_from("company_bank_account.bank")
                .in_list_view(),
            FieldSpec::data("account", "Account")
                .depends_on("company_bank_account")
                .fetch_from("company_bank_account.account"),
        ]
    }

    pub fn update_payment_status(&self, cancel: bool) -> Vec<PaymentStatusUpdate> {
        let status = if cancel {
            "Initiated"
        } else {
            "Payment Ordered"
        };

        let (ref_field, ref_doc_field) = if self.payment_order_type == "Payment Request" {
            ("status", "payment_request")
        } else {
            ("payment_order_status", "reference_name")
        };

        self.references
            .iter()
            .filter_map(|row| {
                let name = match ref_doc_field {
                    "payment_request" => row.payment_request.as_ref(),
                    _ => row.reference_name.as_ref(),
                }?;
                Some(PaymentStatusUpdate {
                    doctype: self.payment_order_type.clone(),
                    name: name.clone(),
                    field: ref_field.to_string(),
                    status: status.to_string(),
                })
            })
            .collect()
    }
}

impl DocumentController for PaymentOrder {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["on_submit", "on_cancel"]
    }
}

pub fn make_journal_entry_plan(
    doc: &PaymentOrder,
    supplier: &str,
    mode_of_payment: Option<&str>,
    mode_of_payment_type: &HashMap<String, String>,
    party_account: &str,
    posting_date: &str,
) -> JournalEntryPlan {
    let voucher_type = if mode_of_payment
        .and_then(|mode| mode_of_payment_type.get(mode))
        .is_some_and(|mode_type| mode_type == "Cash")
    {
        "Cash Entry"
    } else {
        "Bank Entry"
    };

    let mut paid_amt = 0.0;
    let mut accounts = Vec::new();
    for row in &doc.references {
        if row.supplier.as_deref() == Some(supplier)
            && mode_of_payment.is_none_or(|mode| row.mode_of_payment.as_deref() == Some(mode))
        {
            accounts.push(JournalEntryAccountPlan {
                account: party_account.to_string(),
                debit_in_account_currency: row.amount,
                credit_in_account_currency: 0.0,
                party_type: Some("Supplier".to_string()),
                party: Some(supplier.to_string()),
                reference_type: row.reference_doctype.clone(),
                reference_name: row.reference_name.clone(),
            });
            paid_amt += row.amount;
        }
    }

    accounts.push(JournalEntryAccountPlan {
        account: doc.account.clone().unwrap_or_default(),
        debit_in_account_currency: 0.0,
        credit_in_account_currency: paid_amt,
        party_type: None,
        party: None,
        reference_type: None,
        reference_name: None,
    });

    JournalEntryPlan {
        payment_order: doc.name.clone().unwrap_or_default(),
        posting_date: posting_date.to_string(),
        voucher_type: voucher_type.to_string(),
        accounts,
        ignore_mandatory: true,
    }
}
