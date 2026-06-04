use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MergeInvoicesBasedOn {
    Customer,
    CustomerGroup,
}

impl Default for MergeInvoicesBasedOn {
    fn default() -> Self {
        Self::Customer
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLogInvoice {
    pub idx: usize,
    pub pos_invoice: String,
    pub customer: String,
    pub status: String,
    pub docstatus: i32,
    pub is_return: bool,
    pub return_against: Option<String>,
    pub return_against_status: Option<String>,
}

impl PosInvoiceMergeLogInvoice {
    pub fn submitted(idx: usize, pos_invoice: impl Into<String>, customer: impl Into<String>) -> Self {
        Self {
            idx,
            pos_invoice: pos_invoice.into(),
            customer: customer.into(),
            status: "Submitted".to_string(),
            docstatus: 1,
            is_return: false,
            return_against: None,
            return_against_status: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLogSplitInvoice {
    pub pos_invoice: String,
    pub is_return: bool,
    pub return_against: Option<String>,
    pub has_serial_or_batch_item: bool,
    pub return_against_is_consolidated: bool,
}

impl PosInvoiceMergeLogSplitInvoice {
    pub fn sale(pos_invoice: impl Into<String>) -> Self {
        Self {
            pos_invoice: pos_invoice.into(),
            is_return: false,
            return_against: None,
            has_serial_or_batch_item: false,
            return_against_is_consolidated: false,
        }
    }

    pub fn serial_return(
        pos_invoice: impl Into<String>,
        return_against: impl Into<String>,
        return_against_is_consolidated: bool,
    ) -> Self {
        Self {
            pos_invoice: pos_invoice.into(),
            is_return: true,
            return_against: Some(return_against.into()),
            has_serial_or_batch_item: true,
            return_against_is_consolidated,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosInvoiceMergeLogError {
    DuplicatePosInvoices {
        invoice: String,
        rows: Vec<usize>,
    },
    CustomerMismatch {
        row: usize,
        pos_invoice: String,
        customer: String,
    },
    PosInvoiceNotSubmitted {
        row: usize,
        pos_invoice: String,
    },
    PosInvoiceAlreadyConsolidated {
        row: usize,
        pos_invoice: String,
        status: String,
    },
    ReturnOriginalNotConsolidated {
        row: usize,
        return_against: String,
        pos_invoice: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLog {
    pub company: Option<String>,
    pub posting_date: Option<String>,
    pub posting_time: Option<String>,
    pub merge_invoices_based_on: MergeInvoicesBasedOn,
    pub pos_closing_entry: Option<String>,
    pub customer: Option<String>,
    pub customer_group: Option<String>,
    pub pos_invoices: Vec<PosInvoiceMergeLogInvoice>,
    pub consolidated_invoice: Option<String>,
    pub consolidated_credit_note: Option<String>,
    pub amended_from: Option<String>,
}

impl Default for PosInvoiceMergeLog {
    fn default() -> Self {
        Self {
            company: None,
            posting_date: None,
            posting_time: None,
            merge_invoices_based_on: MergeInvoicesBasedOn::Customer,
            pos_closing_entry: None,
            customer: None,
            customer_group: None,
            pos_invoices: Vec::new(),
            consolidated_invoice: None,
            consolidated_credit_note: None,
            amended_from: None,
        }
    }
}

impl PosInvoiceMergeLog {
    pub const DOCTYPE: &'static str = "POS Invoice Merge Log";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 15] = [
        "company",
        "posting_date",
        "posting_time",
        "merge_invoices_based_on",
        "column_break_3",
        "pos_closing_entry",
        "customer",
        "customer_group",
        "section_break_3",
        "pos_invoices",
        "references_section",
        "consolidated_invoice",
        "column_break_7",
        "consolidated_credit_note",
        "amended_from",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        company: impl Into<String>,
        posting_date: impl Into<String>,
        posting_time: impl Into<String>,
        customer: impl Into<String>,
    ) -> Self {
        Self {
            company: Some(company.into()),
            posting_date: Some(posting_date.into()),
            posting_time: Some(posting_time.into()),
            customer: Some(customer.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_standard_filter()
                .print_hide()
                .required(),
            FieldSpec::date("posting_date", "Posting Date")
                .required()
                .in_list_view(),
            FieldSpec::time("posting_time", "Posting Time")
                .no_copy()
                .required(),
            FieldSpec::select("merge_invoices_based_on", "Merge Invoices Based On")
                .options("Customer\nCustomer Group")
                .required(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("pos_closing_entry", "POS Closing Entry")
                .options("POS Closing Entry"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .required()
                .in_list_view(),
            FieldSpec::link("customer_group", "Customer Group")
                .options("Customer Group")
                .depends_on("eval:doc.merge_invoices_based_on == 'Customer Group'")
                .mandatory_depends_on("eval:doc.merge_invoices_based_on == 'Customer Group'"),
            FieldSpec::section_break("section_break_3"),
            FieldSpec::table("pos_invoices", "POS Invoices")
                .options("POS Invoice Reference")
                .required(),
            FieldSpec::section_break("references_section")
                .label("References")
                .collapsible(),
            FieldSpec::link("consolidated_invoice", "Consolidated Sales Invoice")
                .options("Sales Invoice")
                .read_only()
                .allow_on_submit(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("consolidated_credit_note", "Consolidated Credit Note")
                .options("Sales Invoice")
                .read_only()
                .allow_on_submit(),
            FieldSpec::link("amended_from", "Amended From")
                .options("POS Invoice Merge Log")
                .read_only()
                .no_copy()
                .print_hide(),
        ]
    }

    pub fn validate(&self) -> Result<(), PosInvoiceMergeLogError> {
        self.validate_customer()?;
        self.validate_pos_invoice_status()?;
        self.validate_duplicate_pos_invoices()
    }

    pub fn validate_duplicate_pos_invoices(&self) -> Result<(), PosInvoiceMergeLogError> {
        let mut occurrences: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for invoice in &self.pos_invoices {
            occurrences
                .entry(invoice.pos_invoice.as_str())
                .or_default()
                .push(invoice.idx);
        }

        for (invoice, rows) in occurrences {
            if rows.len() > 1 {
                return Err(PosInvoiceMergeLogError::DuplicatePosInvoices {
                    invoice: invoice.to_string(),
                    rows,
                });
            }
        }

        Ok(())
    }

    pub fn validate_customer(&self) -> Result<(), PosInvoiceMergeLogError> {
        if self.merge_invoices_based_on == MergeInvoicesBasedOn::CustomerGroup {
            return Ok(());
        }

        let customer = self.customer.as_deref().unwrap_or_default();
        for invoice in &self.pos_invoices {
            if invoice.customer != customer {
                return Err(PosInvoiceMergeLogError::CustomerMismatch {
                    row: invoice.idx,
                    pos_invoice: invoice.pos_invoice.clone(),
                    customer: customer.to_string(),
                });
            }
        }

        Ok(())
    }

    pub fn validate_pos_invoice_status(&self) -> Result<(), PosInvoiceMergeLogError> {
        let current_invoices: BTreeSet<&str> = self
            .pos_invoices
            .iter()
            .map(|invoice| invoice.pos_invoice.as_str())
            .collect();

        for invoice in &self.pos_invoices {
            if invoice.docstatus != 1 {
                return Err(PosInvoiceMergeLogError::PosInvoiceNotSubmitted {
                    row: invoice.idx,
                    pos_invoice: invoice.pos_invoice.clone(),
                });
            }

            if invoice.status == "Consolidated" {
                return Err(PosInvoiceMergeLogError::PosInvoiceAlreadyConsolidated {
                    row: invoice.idx,
                    pos_invoice: invoice.pos_invoice.clone(),
                    status: invoice.status.clone(),
                });
            }

            if invoice.is_return {
                if let Some(return_against) = invoice.return_against.as_deref() {
                    if !current_invoices.contains(return_against)
                        && invoice.return_against_status.as_deref() != Some("Consolidated")
                    {
                        return Err(PosInvoiceMergeLogError::ReturnOriginalNotConsolidated {
                            row: invoice.idx,
                            return_against: return_against.to_string(),
                            pos_invoice: invoice.pos_invoice.clone(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

impl DocumentController for PosInvoiceMergeLog {
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

pub fn split_invoices(invoices: &[PosInvoiceMergeLogSplitInvoice]) -> Vec<Vec<String>> {
    let mut split: Vec<Vec<String>> = Vec::new();
    let mut special_invoices = BTreeSet::new();

    for invoice in invoices
        .iter()
        .filter(|invoice| invoice.is_return && invoice.return_against.is_some())
    {
        if !invoice.has_serial_or_batch_item {
            continue;
        }

        let return_against = invoice.return_against.as_deref().unwrap();
        let return_against_is_added = split
            .iter()
            .any(|group| group.first().is_some_and(|name| name == return_against));
        if return_against_is_added || invoice.return_against_is_consolidated {
            continue;
        }

        split.push(
            invoices
                .iter()
                .filter(|candidate| candidate.pos_invoice == return_against)
                .map(|candidate| candidate.pos_invoice.clone())
                .collect(),
        );
        special_invoices.insert(return_against.to_string());
    }

    split.push(
        invoices
            .iter()
            .filter(|invoice| !special_invoices.contains(invoice.pos_invoice.as_str()))
            .map(|invoice| invoice.pos_invoice.clone())
            .collect(),
    );

    split
}
