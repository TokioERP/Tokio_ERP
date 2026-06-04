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
    pub fn submitted(
        idx: usize,
        pos_invoice: impl Into<String>,
        customer: impl Into<String>,
    ) -> Self {
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
pub struct PosInvoiceMergeLogUpdateInvoice {
    pub name: String,
    pub is_return: bool,
}

impl PosInvoiceMergeLogUpdateInvoice {
    pub fn sale(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_return: false,
        }
    }

    pub fn return_invoice(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_return: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLogClosingEntry {
    pub name: String,
    pub posting_date: String,
    pub posting_time: String,
    pub company: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLogCancelCandidate {
    pub name: String,
    pub docstatus: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLogReturnInvoice {
    pub name: String,
    pub return_against: String,
    pub return_against_consolidated_invoice: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewSalesInvoicePlan {
    pub customer: Option<String>,
    pub is_pos: bool,
    pub posting_date: Option<String>,
    pub posting_time: Option<String>,
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
    SchedulerInactive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnqueueJobKind {
    CreateMergeLogs,
    CancelMergeLogs,
}

impl EnqueueJobKind {
    fn message(self) -> &'static str {
        match self {
            Self::CreateMergeLogs => "POS Invoices will be consolidated in a background process",
            Self::CancelMergeLogs => "POS Invoices will be unconsolidated in a background process",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosInvoiceMergeLogAction {
    SetClosingEntryStatus {
        status: String,
    },
    UpdatePosInvoiceConsolidatedInvoice {
        pos_invoice: String,
        consolidated_invoice: Option<String>,
    },
    RefreshPosInvoiceStatus {
        pos_invoice: String,
    },
    SavePosInvoice {
        pos_invoice: String,
    },
    SetSerialAndBatchBundle {
        pos_invoice: String,
        table_name: String,
    },
    CancelLinkedInvoice {
        sales_invoice: String,
        ignore_validate: bool,
    },
    DelinkCancelledStockLedgerBundles {
        bundles: Vec<String>,
    },
    CreateMergeLogDocument {
        posting_date: String,
        posting_time: String,
        company: Option<String>,
        customer: String,
        pos_closing_entry: Option<String>,
        pos_invoices: Vec<String>,
        ignore_permissions: bool,
    },
    SubmitCreatedMergeLog {
        pos_invoices: Vec<String>,
    },
    CancelMergeLog {
        merge_log: String,
        ignore_permissions: bool,
    },
    SetClosingEntryErrorMessage {
        error_message: String,
    },
    UpdateOpeningEntry {
        for_cancel: bool,
    },
    CreateMergeLogs,
    CancelMergeLogs,
    EnqueueJob {
        kind: EnqueueJobKind,
        job_id: String,
        message: String,
    },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchedulerStatus {
    pub developer_mode: bool,
    pub in_test: bool,
    pub scheduler_inactive: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorMessage {
    Dict(BTreeMap<String, String>),
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosInvoiceMergeLogSourceInvoice {
    pub pos_invoice: String,
    pub customer: String,
    pub accounting_dimensions: BTreeMap<String, String>,
}

impl PosInvoiceMergeLogSourceInvoice {
    pub fn new(pos_invoice: impl Into<String>, customer: impl Into<String>) -> Self {
        Self {
            pos_invoice: pos_invoice.into(),
            customer: customer.into(),
            accounting_dimensions: BTreeMap::new(),
        }
    }

    pub fn dimension(mut self, fieldname: impl Into<String>, value: impl Into<String>) -> Self {
        self.accounting_dimensions
            .insert(fieldname.into(), value.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosInvoiceMergeLogItem {
    pub name: String,
    pub net_rate: f64,
    pub net_amount: f64,
    pub base_net_amount: f64,
    pub pos_invoice_item: Option<String>,
    pub serial_and_batch_bundle: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosInvoiceMergeLogTax {
    pub name: String,
    pub account_head: String,
    pub cost_center: String,
    pub tax_amount_after_discount_amount: f64,
    pub base_tax_amount_after_discount_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosInvoiceMergeLogPayment {
    pub account: String,
    pub mode_of_payment: String,
    pub amount: f64,
    pub base_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosInvoiceMergeLogItemWiseTaxDetail {
    pub item_row: String,
    pub tax_row: String,
    pub amount: f64,
    pub rate: f64,
    pub taxable_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosInvoiceMergeLogDocument {
    pub name: String,
    pub posting_date: Option<String>,
    pub posting_time: Option<String>,
    pub is_return: bool,
    pub return_against: Option<String>,
    pub redeem_loyalty_points: bool,
    pub loyalty_redemption_account: Option<String>,
    pub loyalty_redemption_cost_center: Option<String>,
    pub loyalty_points: i64,
    pub loyalty_amount: f64,
    pub items: Vec<PosInvoiceMergeLogItem>,
    pub taxes: Vec<PosInvoiceMergeLogTax>,
    pub payments: Vec<PosInvoiceMergeLogPayment>,
    pub rounding_adjustment: f64,
    pub rounded_total: f64,
    pub base_rounding_adjustment: f64,
    pub base_rounded_total: f64,
    pub item_wise_tax_details: Vec<PosInvoiceMergeLogItemWiseTaxDetail>,
    pub accounting_dimensions: BTreeMap<String, String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
}

impl PosInvoiceMergeLogDocument {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            posting_date: None,
            posting_time: None,
            is_return: false,
            return_against: None,
            redeem_loyalty_points: false,
            loyalty_redemption_account: None,
            loyalty_redemption_cost_center: None,
            loyalty_points: 0,
            loyalty_amount: 0.0,
            items: Vec::new(),
            taxes: Vec::new(),
            payments: Vec::new(),
            rounding_adjustment: 0.0,
            rounded_total: 0.0,
            base_rounding_adjustment: 0.0,
            base_rounded_total: 0.0,
            item_wise_tax_details: Vec::new(),
            accounting_dimensions: BTreeMap::new(),
            cost_center: None,
            project: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoiceMergeLogProfileDefaults {
    pub pos_profile: String,
    pub accounting_dimensions: BTreeMap<String, String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MergedPosInvoiceItem {
    pub row_id: String,
    pub pos_invoice: String,
    pub pos_invoice_item: String,
    pub sales_invoice_item: Option<String>,
    pub rate: f64,
    pub amount: f64,
    pub base_amount: f64,
    pub price_list_rate: f64,
    pub serial_and_batch_bundle: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MergedPosInvoiceTax {
    pub row_id: String,
    pub account_head: String,
    pub cost_center: String,
    pub charge_type: String,
    pub idx: usize,
    pub included_in_print_rate: bool,
    pub tax_amount: f64,
    pub base_tax_amount: f64,
    pub dont_recompute_tax: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MergedPosInvoicePayment {
    pub account: String,
    pub mode_of_payment: String,
    pub amount: f64,
    pub base_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MergedItemWiseTaxDetail {
    pub item_row: String,
    pub tax_row: String,
    pub amount: f64,
    pub rate: f64,
    pub taxable_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MergedPosInvoicePlan {
    pub posting_date: Option<String>,
    pub posting_time: Option<String>,
    pub items: Vec<MergedPosInvoiceItem>,
    pub payments: Vec<MergedPosInvoicePayment>,
    pub taxes: Vec<MergedPosInvoiceTax>,
    pub item_wise_tax_details: Vec<MergedItemWiseTaxDetail>,
    pub redeem_loyalty_points: bool,
    pub loyalty_redemption_account: Option<String>,
    pub loyalty_redemption_cost_center: Option<String>,
    pub loyalty_points: i64,
    pub loyalty_amount: f64,
    pub rounding_adjustment: f64,
    pub rounded_total: f64,
    pub base_rounding_adjustment: f64,
    pub base_rounded_total: f64,
    pub additional_discount_percentage: f64,
    pub discount_amount: f64,
    pub taxes_and_charges: Option<String>,
    pub ignore_pricing_rule: bool,
    pub customer: Option<String>,
    pub disable_rounded_total: bool,
    pub accounting_dimensions: BTreeMap<String, String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub ignore_pos_profile: bool,
    pub pos_profile: String,
    pub sales_partner: Option<String>,
    pub commission_rate: f64,
    pub total_commission: f64,
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
            FieldSpec::link("pos_closing_entry", "POS Closing Entry").options("POS Closing Entry"),
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

    pub fn get_new_sales_invoice_plan(&self) -> NewSalesInvoicePlan {
        NewSalesInvoicePlan {
            customer: self.customer.clone(),
            is_pos: true,
            posting_date: None,
            posting_time: None,
        }
    }

    pub fn merge_pos_invoice_into_plan(
        &self,
        data: &[PosInvoiceMergeLogDocument],
        profile_defaults: &PosInvoiceMergeLogProfileDefaults,
        disable_rounded_total: bool,
        accounting_dimension_fields: &[String],
    ) -> MergedPosInvoicePlan {
        let mut items = Vec::new();
        let mut payments: Vec<MergedPosInvoicePayment> = Vec::new();
        let mut taxes: Vec<MergedPosInvoiceTax> = Vec::new();
        let mut item_tax_details = Vec::new();
        let mut loyalty_amount_sum = 0.0;
        let mut loyalty_points_sum = 0;
        let mut loyalty_redemption_account = None;
        let mut loyalty_redemption_cost_center = None;
        let mut rounding_adjustment = 0.0;
        let mut rounded_total = 0.0;
        let mut base_rounding_adjustment = 0.0;
        let mut base_rounded_total = 0.0;
        let mut posting_date = None;
        let mut posting_time = None;

        for doc in data {
            let mut old_new_item_map = BTreeMap::new();
            let mut old_new_tax_map = BTreeMap::new();

            if doc.posting_date.is_some() {
                posting_date = doc.posting_date.clone();
                posting_time = doc.posting_time.clone();
            }

            if doc.redeem_loyalty_points {
                loyalty_redemption_account = doc.loyalty_redemption_account.clone();
                loyalty_redemption_cost_center = doc.loyalty_redemption_cost_center.clone();
                loyalty_points_sum += doc.loyalty_points;
                loyalty_amount_sum += doc.loyalty_amount;
            }

            for item in &doc.items {
                let row_id = format!("{}:{}", doc.name, item.name);
                let sales_invoice_item = doc.is_return.then(|| {
                    format!(
                        "{}:{}",
                        doc.return_against.as_deref().unwrap_or_default(),
                        item.pos_invoice_item
                            .as_deref()
                            .unwrap_or(item.name.as_str())
                    )
                });
                items.push(MergedPosInvoiceItem {
                    row_id: row_id.clone(),
                    pos_invoice: doc.name.clone(),
                    pos_invoice_item: item.name.clone(),
                    sales_invoice_item,
                    rate: item.net_rate,
                    amount: item.net_amount,
                    base_amount: item.base_net_amount,
                    price_list_rate: 0.0,
                    serial_and_batch_bundle: item.serial_and_batch_bundle.clone(),
                });
                old_new_item_map.insert(item.name.clone(), row_id);
            }

            for tax in &doc.taxes {
                let tax_key = (tax.account_head.clone(), tax.cost_center.clone());
                if let Some(existing_tax) = taxes
                    .iter_mut()
                    .find(|row| row.account_head == tax_key.0 && row.cost_center == tax_key.1)
                {
                    existing_tax.tax_amount += tax.tax_amount_after_discount_amount;
                    existing_tax.base_tax_amount += tax.base_tax_amount_after_discount_amount;
                    old_new_tax_map.insert(tax.name.clone(), existing_tax.row_id.clone());
                } else {
                    let row_id = format!("{}|{}", tax.account_head, tax.cost_center);
                    taxes.push(MergedPosInvoiceTax {
                        row_id: row_id.clone(),
                        account_head: tax.account_head.clone(),
                        cost_center: tax.cost_center.clone(),
                        charge_type: "Actual".to_string(),
                        idx: taxes.len() + 1,
                        included_in_print_rate: false,
                        tax_amount: tax.tax_amount_after_discount_amount,
                        base_tax_amount: tax.base_tax_amount_after_discount_amount,
                        dont_recompute_tax: true,
                    });
                    old_new_tax_map.insert(tax.name.clone(), row_id);
                }
            }

            for payment in &doc.payments {
                if let Some(existing_payment) = payments.iter_mut().find(|row| {
                    row.account == payment.account && row.mode_of_payment == payment.mode_of_payment
                }) {
                    existing_payment.amount += payment.amount;
                    existing_payment.base_amount += payment.base_amount;
                } else {
                    payments.push(MergedPosInvoicePayment {
                        account: payment.account.clone(),
                        mode_of_payment: payment.mode_of_payment.clone(),
                        amount: payment.amount,
                        base_amount: payment.base_amount,
                    });
                }
            }

            rounding_adjustment += doc.rounding_adjustment;
            rounded_total += doc.rounded_total;
            base_rounding_adjustment += doc.base_rounding_adjustment;
            base_rounded_total += doc.base_rounded_total;

            for detail in &doc.item_wise_tax_details {
                let item_row = old_new_item_map
                    .get(detail.item_row.as_str())
                    .unwrap_or_else(|| panic!("missing mapped item row {}", detail.item_row));
                let tax_row = old_new_tax_map
                    .get(detail.tax_row.as_str())
                    .unwrap_or_else(|| panic!("missing mapped tax row {}", detail.tax_row));
                item_tax_details.push(MergedItemWiseTaxDetail {
                    item_row: item_row.clone(),
                    tax_row: tax_row.clone(),
                    amount: detail.amount,
                    rate: detail.rate,
                    taxable_amount: detail.taxable_amount,
                });
            }
        }

        let first_doc = data.first();
        let mut accounting_dimensions = BTreeMap::new();
        for fieldname in accounting_dimension_fields {
            let value = first_doc
                .and_then(|doc| non_empty_map_value(&doc.accounting_dimensions, fieldname))
                .or_else(|| {
                    non_empty_map_value(&profile_defaults.accounting_dimensions, fieldname)
                });
            if let Some(value) = value {
                accounting_dimensions.insert(fieldname.clone(), value.to_string());
            }
        }

        let cost_center = first_doc
            .and_then(|doc| non_empty_option_value(&doc.cost_center))
            .or_else(|| non_empty_option_value(&profile_defaults.cost_center))
            .map(str::to_string);
        let project = first_doc
            .and_then(|doc| non_empty_option_value(&doc.project))
            .or_else(|| non_empty_option_value(&profile_defaults.project))
            .map(str::to_string);
        let ignore_pos_profile =
            self.merge_invoices_based_on == MergeInvoicesBasedOn::CustomerGroup;

        MergedPosInvoicePlan {
            posting_date,
            posting_time,
            items,
            payments,
            taxes,
            item_wise_tax_details: item_tax_details,
            redeem_loyalty_points: loyalty_points_sum != 0,
            loyalty_redemption_account,
            loyalty_redemption_cost_center,
            loyalty_points: loyalty_points_sum,
            loyalty_amount: loyalty_amount_sum,
            rounding_adjustment,
            rounded_total,
            base_rounding_adjustment,
            base_rounded_total,
            additional_discount_percentage: 0.0,
            discount_amount: 0.0,
            taxes_and_charges: None,
            ignore_pricing_rule: true,
            customer: self.customer.clone(),
            disable_rounded_total,
            accounting_dimensions,
            cost_center,
            project,
            ignore_pos_profile,
            pos_profile: if ignore_pos_profile {
                String::new()
            } else {
                profile_defaults.pos_profile.clone()
            },
            sales_partner: None,
            commission_rate: 0.0,
            total_commission: 0.0,
        }
    }

    pub fn update_pos_invoices_plan(
        &self,
        invoice_docs: &[PosInvoiceMergeLogUpdateInvoice],
        sales_invoice: Option<&str>,
        credit_notes: &BTreeMap<String, Vec<String>>,
        docstatus: i32,
    ) -> Vec<PosInvoiceMergeLogAction> {
        let mut actions = Vec::new();
        for doc in invoice_docs {
            let mut consolidated_invoice = sales_invoice.map(str::to_string);
            if doc.is_return {
                for (credit_note, pos_invoices) in credit_notes {
                    if pos_invoices.contains(&doc.name) {
                        consolidated_invoice = Some(credit_note.clone());
                        break;
                    }
                }
            }
            if docstatus == 2 {
                consolidated_invoice = None;
            }

            actions.push(
                PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                    pos_invoice: doc.name.clone(),
                    consolidated_invoice,
                },
            );
            actions.push(PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: doc.name.clone(),
            });
            actions.push(PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: doc.name.clone(),
            });
        }
        actions
    }

    pub fn on_cancel_side_effect_plan(&self, bundles: &[String]) -> Vec<PosInvoiceMergeLogAction> {
        let invoice_docs = self
            .pos_invoices
            .iter()
            .map(|invoice| PosInvoiceMergeLogUpdateInvoice {
                name: invoice.pos_invoice.clone(),
                is_return: invoice.is_return,
            })
            .collect::<Vec<_>>();
        let mut actions = self.update_pos_invoices_plan(&invoice_docs, None, &BTreeMap::new(), 2);

        for invoice in &self.pos_invoices {
            for table_name in ["items", "packed_items"] {
                actions.push(PosInvoiceMergeLogAction::SetSerialAndBatchBundle {
                    pos_invoice: invoice.pos_invoice.clone(),
                    table_name: table_name.to_string(),
                });
            }
        }

        let mut linked_invoices = vec![
            self.consolidated_invoice.clone(),
            self.consolidated_credit_note.clone(),
        ];
        linked_invoices.reverse();
        for sales_invoice in linked_invoices.into_iter().flatten() {
            actions.push(PosInvoiceMergeLogAction::CancelLinkedInvoice {
                sales_invoice,
                ignore_validate: true,
            });
        }

        if !bundles.is_empty() {
            actions.push(
                PosInvoiceMergeLogAction::DelinkCancelledStockLedgerBundles {
                    bundles: bundles.to_vec(),
                },
            );
        }

        actions
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

pub fn split_invoices_by_accounting_dimension(
    pos_invoices: &[PosInvoiceMergeLogSourceInvoice],
) -> BTreeMap<Vec<(String, String)>, Vec<String>> {
    let mut grouped = BTreeMap::new();
    for invoice in pos_invoices {
        let key = invoice
            .accounting_dimensions
            .iter()
            .map(|(fieldname, value)| (fieldname.clone(), value.clone()))
            .collect::<Vec<_>>();
        grouped
            .entry(key)
            .or_insert_with(Vec::new)
            .push(invoice.pos_invoice.clone());
    }
    grouped
}

pub fn get_invoice_customer_map(
    pos_invoices: &[PosInvoiceMergeLogSourceInvoice],
) -> BTreeMap<String, BTreeMap<Vec<(String, String)>, Vec<String>>> {
    let mut customer_map: BTreeMap<String, Vec<PosInvoiceMergeLogSourceInvoice>> = BTreeMap::new();
    for invoice in pos_invoices {
        customer_map
            .entry(invoice.customer.clone())
            .or_default()
            .push(invoice.clone());
    }

    customer_map
        .into_iter()
        .map(|(customer, invoices)| {
            (
                customer,
                split_invoices_by_accounting_dimension(invoices.as_slice()),
            )
        })
        .collect()
}

pub fn distinguish_return_pos_invoices_plan(
    data: &[PosInvoiceMergeLogReturnInvoice],
    sales_invoice_doc: Option<&str>,
) -> BTreeMap<Option<String>, Vec<String>> {
    let default_key = sales_invoice_doc.map(str::to_string);
    let mut return_invoices = BTreeMap::from([(default_key.clone(), Vec::new())]);

    for doc in data {
        if let Some(sales_invoice) = doc.return_against_consolidated_invoice.as_deref() {
            return_invoices
                .entry(Some(sales_invoice.to_string()))
                .or_default()
                .push(doc.name.clone());
        } else {
            return_invoices
                .entry(default_key.clone())
                .or_default()
                .push(doc.name.clone());
        }
    }

    return_invoices
}

pub fn consolidate_pos_invoices_plan(
    invoice_count: usize,
    closing_entry: Option<&str>,
    _invoice_by_customer: BTreeMap<String, BTreeMap<Vec<(String, String)>, Vec<String>>>,
) -> Vec<PosInvoiceMergeLogAction> {
    if invoice_count >= 10 {
        if let Some(closing_entry) = closing_entry {
            return vec![
                PosInvoiceMergeLogAction::SetClosingEntryStatus {
                    status: "Queued".to_string(),
                },
                enqueue_action(EnqueueJobKind::CreateMergeLogs, closing_entry),
            ];
        }
    }

    vec![PosInvoiceMergeLogAction::CreateMergeLogs]
}

pub fn unconsolidate_pos_invoices_plan(
    closing_entry_pos_invoice_count: usize,
    closing_entry: Option<&str>,
) -> Vec<PosInvoiceMergeLogAction> {
    if closing_entry_pos_invoice_count >= 10 {
        if let Some(closing_entry) = closing_entry {
            return vec![
                PosInvoiceMergeLogAction::SetClosingEntryStatus {
                    status: "Queued".to_string(),
                },
                enqueue_action(EnqueueJobKind::CancelMergeLogs, closing_entry),
            ];
        }
    }

    vec![PosInvoiceMergeLogAction::CancelMergeLogs]
}

pub fn create_merge_logs_plan(
    invoice_by_customer: &BTreeMap<String, Vec<Vec<PosInvoiceMergeLogSplitInvoice>>>,
    closing_entry: Option<&PosInvoiceMergeLogClosingEntry>,
) -> Vec<PosInvoiceMergeLogAction> {
    let mut actions = Vec::new();
    for (customer, invoice_groups) in invoice_by_customer {
        for invoices in invoice_groups {
            for pos_invoices in split_invoices(invoices) {
                let (posting_date, posting_time, company, pos_closing_entry) =
                    if let Some(closing_entry) = closing_entry {
                        (
                            closing_entry.posting_date.clone(),
                            closing_entry.posting_time.clone(),
                            Some(closing_entry.company.clone()),
                            Some(closing_entry.name.clone()),
                        )
                    } else {
                        (String::new(), String::new(), None, None)
                    };

                actions.push(PosInvoiceMergeLogAction::CreateMergeLogDocument {
                    posting_date,
                    posting_time,
                    company,
                    customer: customer.clone(),
                    pos_closing_entry,
                    pos_invoices: pos_invoices.clone(),
                    ignore_permissions: true,
                });
                actions.push(PosInvoiceMergeLogAction::SubmitCreatedMergeLog { pos_invoices });
            }
        }
    }

    if closing_entry.is_some() {
        actions.push(PosInvoiceMergeLogAction::SetClosingEntryStatus {
            status: "Submitted".to_string(),
        });
        actions.push(PosInvoiceMergeLogAction::SetClosingEntryErrorMessage {
            error_message: String::new(),
        });
        actions.push(PosInvoiceMergeLogAction::UpdateOpeningEntry { for_cancel: false });
    }

    actions
}

pub fn cancel_merge_logs_plan(
    merge_logs: &[PosInvoiceMergeLogCancelCandidate],
    closing_entry: Option<&PosInvoiceMergeLogClosingEntry>,
) -> Vec<PosInvoiceMergeLogAction> {
    let mut actions = Vec::new();
    for merge_log in merge_logs {
        if merge_log.docstatus == 2 {
            continue;
        }

        actions.push(PosInvoiceMergeLogAction::CancelMergeLog {
            merge_log: merge_log.name.clone(),
            ignore_permissions: true,
        });
    }

    if closing_entry.is_some() {
        actions.push(PosInvoiceMergeLogAction::SetClosingEntryStatus {
            status: "Cancelled".to_string(),
        });
        actions.push(PosInvoiceMergeLogAction::SetClosingEntryErrorMessage {
            error_message: String::new(),
        });
        actions.push(PosInvoiceMergeLogAction::UpdateOpeningEntry { for_cancel: true });
    }

    actions
}

pub fn enqueue_job_plan(
    kind: EnqueueJobKind,
    closing_entry: Option<&str>,
    is_job_enqueued: bool,
    status: SchedulerStatus,
) -> Result<Option<PosInvoiceMergeLogAction>, PosInvoiceMergeLogError> {
    check_scheduler_status(status.in_test, status.scheduler_inactive)?;
    if is_job_enqueued {
        return Ok(None);
    }

    Ok(Some(enqueue_action(
        kind,
        closing_entry.unwrap_or_default(),
    )))
}

pub fn check_scheduler_status(
    in_test: bool,
    scheduler_inactive: bool,
) -> Result<(), PosInvoiceMergeLogError> {
    if scheduler_inactive && !in_test {
        Err(PosInvoiceMergeLogError::SchedulerInactive)
    } else {
        Ok(())
    }
}

pub fn get_error_message(message: ErrorMessage) -> String {
    match message {
        ErrorMessage::Dict(values) => values
            .get("message")
            .cloned()
            .unwrap_or_else(|| format!("{values:?}")),
        ErrorMessage::Text(message) => message,
    }
}

fn enqueue_action(kind: EnqueueJobKind, closing_entry: &str) -> PosInvoiceMergeLogAction {
    PosInvoiceMergeLogAction::EnqueueJob {
        kind,
        job_id: format!("pos_invoice_merge::{closing_entry}"),
        message: kind.message().to_string(),
    }
}

fn non_empty_map_value<'a>(values: &'a BTreeMap<String, String>, key: &str) -> Option<&'a str> {
    values
        .get(key)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
}

fn non_empty_option_value(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|value| !value.is_empty())
}
