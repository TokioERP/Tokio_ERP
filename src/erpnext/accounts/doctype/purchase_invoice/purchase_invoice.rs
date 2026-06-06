use std::collections::HashMap;

use crate::erpnext::DocumentController;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusUpdaterSpec {
    pub source_dt: &'static str,
    pub target_dt: &'static str,
    pub join_field: &'static str,
    pub target_field: &'static str,
    pub target_parent_dt: &'static str,
    pub target_parent_field: &'static str,
    pub target_ref_field: &'static str,
    pub source_field: &'static str,
    pub percent_join_field: &'static str,
    pub overflow_type: &'static str,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupplierTaxMeta {
    pub tax_withholding_category: Option<String>,
    pub tax_withholding_group: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OnloadPlan {
    pub apply_tds: bool,
    pub clear_tax_withholding_entries: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountMeta {
    pub account_type: Option<String>,
    pub report_type: Option<String>,
    pub account_currency: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PurchaseInvoiceItemRow {
    pub qty: f64,
    pub received_qty: f64,
    pub purchase_receipt: Option<String>,
    pub pr_detail: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PurchaseInvoiceError {
    ReleaseDateMustBeFuture,
    CashBankAccountRequired,
    PaidAndWriteOffGreaterThanGrandTotal,
    CreditToAccountMissing(String),
    CreditToMustBeBalanceSheet,
    CreditToMustBePayable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseInvoice {
    pub name: Option<String>,
    pub is_new: bool,
    pub amended_from: Option<String>,
    pub docstatus: i32,
    pub supplier: Option<String>,
    pub company: Option<String>,
    pub currency: String,
    pub party_account_currency: Option<String>,
    pub is_paid: i32,
    pub is_return: i32,
    pub is_opening: String,
    pub on_hold: bool,
    pub release_date: Option<String>,
    pub hold_comment: Option<String>,
    pub bill_no: Option<String>,
    pub bill_date: Option<String>,
    pub cash_bank_account: Option<String>,
    pub paid_amount: f64,
    pub write_off_amount: f64,
    pub grand_total: f64,
    pub rounded_total: Option<f64>,
    pub base_grand_total: f64,
    pub base_rounded_total: Option<f64>,
    pub disable_rounded_total: bool,
    pub outstanding_amount: f64,
    pub due_date: Option<String>,
    pub credit_to: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub internal_transfer: bool,
    pub has_submitted_debit_note: bool,
    pub per_received: f64,
    pub items: Vec<PurchaseInvoiceItemRow>,
}

impl Default for PurchaseInvoice {
    fn default() -> Self {
        Self {
            name: None,
            is_new: false,
            amended_from: None,
            docstatus: 0,
            supplier: None,
            company: None,
            currency: String::new(),
            party_account_currency: None,
            is_paid: 0,
            is_return: 0,
            is_opening: String::new(),
            on_hold: false,
            release_date: None,
            hold_comment: None,
            bill_no: None,
            bill_date: None,
            cash_bank_account: None,
            paid_amount: 0.0,
            write_off_amount: 0.0,
            grand_total: 0.0,
            rounded_total: None,
            base_grand_total: 0.0,
            base_rounded_total: None,
            disable_rounded_total: false,
            outstanding_amount: 0.0,
            due_date: None,
            credit_to: None,
            remarks: None,
            status: String::new(),
            internal_transfer: false,
            has_submitted_debit_note: false,
            per_received: 0.0,
            items: Vec::new(),
        }
    }
}

impl PurchaseInvoice {
    pub const DOCTYPE: &'static str = "Purchase Invoice";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const TITLE_FIELD: &'static str = "supplier_name";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const IS_SUBMITTABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "naming_series",
        "supplier",
        "supplier_name",
        "tax_id",
        "due_date",
        "is_paid",
        "is_return",
        "apply_tds",
        "column_break1",
        "company",
        "cost_center",
        "posting_date",
        "posting_time",
        "set_posting_time",
        "amended_from",
        "sb_14",
        "on_hold",
        "release_date",
        "cb_17",
        "hold_comment",
        "supplier_invoice_details",
        "bill_no",
        "column_break_15",
        "bill_date",
        "return_against",
        "update_billed_amount_in_purchase_order",
        "update_billed_amount_in_purchase_receipt",
        "section_addresses",
        "supplier_address",
        "address_display",
        "contact_person",
        "contact_display",
        "contact_mobile",
        "contact_email",
        "col_break_address",
        "shipping_address",
        "shipping_address_display",
        "currency_and_price_list",
        "currency",
        "conversion_rate",
        "column_break2",
        "buying_price_list",
        "price_list_currency",
        "plc_conversion_rate",
        "ignore_pricing_rule",
        "sec_warehouse",
        "set_warehouse",
        "rejected_warehouse",
        "col_break_warehouse",
        "is_subcontracted",
        "items_section",
        "update_stock",
        "scan_barcode",
        "items",
        "pricing_rule_details",
        "pricing_rules",
        "raw_materials_supplied",
        "supplied_items",
        "section_break_26",
        "total_qty",
        "base_total",
        "base_net_total",
        "column_break_28",
        "total",
        "net_total",
        "total_net_weight",
        "taxes_section",
        "tax_category",
        "column_break_49",
        "shipping_rule",
        "section_break_51",
        "taxes_and_charges",
        "taxes",
        "sec_tax_breakup",
        "other_charges_calculation",
        "totals",
        "base_taxes_and_charges_added",
        "base_taxes_and_charges_deducted",
        "base_total_taxes_and_charges",
        "column_break_40",
        "taxes_and_charges_added",
        "taxes_and_charges_deducted",
        "total_taxes_and_charges",
        "section_break_44",
        "apply_discount_on",
        "base_discount_amount",
        "column_break_46",
        "additional_discount_percentage",
        "discount_amount",
        "base_grand_total",
        "base_rounding_adjustment",
        "base_rounded_total",
        "base_in_words",
        "column_break8",
        "grand_total",
        "rounding_adjustment",
        "rounded_total",
        "in_words",
        "total_advance",
        "outstanding_amount",
        "disable_rounded_total",
        "payments_section",
        "mode_of_payment",
        "cash_bank_account",
        "clearance_date",
        "col_br_payments",
        "paid_amount",
        "base_paid_amount",
        "write_off",
        "write_off_amount",
        "base_write_off_amount",
        "column_break_61",
        "write_off_account",
        "write_off_cost_center",
        "advances_section",
        "allocate_advances_automatically",
        "get_advances",
        "advances",
        "payment_schedule_section",
        "payment_terms_template",
        "payment_schedule",
        "terms_section_break",
        "tc_name",
        "terms",
        "printing_settings",
        "letter_head",
        "group_same_items",
        "column_break_112",
        "select_print_heading",
        "language",
        "is_internal_supplier",
        "credit_to",
        "party_account_currency",
        "is_opening",
        "against_expense_account",
        "column_break_63",
        "status",
        "inter_company_invoice_reference",
        "remarks",
        "subscription_section",
        "from_date",
        "to_date",
        "column_break_114",
        "auto_repeat",
        "update_auto_repeat_reference",
        "accounting_dimensions_section",
        "dimension_col_break",
        "billing_address",
        "billing_address_display",
        "project",
        "unrealized_profit_loss_account",
        "represents_company",
        "set_from_warehouse",
        "supplier_warehouse",
        "per_received",
        "ignore_default_payment_terms_template",
        "accounting_details_section",
        "column_break_147",
        "subscription",
        "is_old_subcontracting_flow",
        "payments_tab",
        "address_and_contact_tab",
        "terms_tab",
        "more_info_tab",
        "connections_tab",
        "column_break_6",
        "column_break_50",
        "column_break_58",
        "company_shipping_address_section",
        "column_break_126",
        "company_billing_address_section",
        "column_break_130",
        "status_section",
        "column_break_177",
        "additional_info_section",
        "incoterm",
        "named_place",
        "only_include_allocated_payments",
        "use_company_roundoff_cost_center",
        "use_transaction_date_exchange_rate",
        "supplier_group",
        "update_outstanding_for_self",
        "sender",
        "dispatch_address_display",
        "dispatch_address",
        "last_scanned_warehouse",
        "claimed_landed_cost_amount",
        "item_wise_tax_details",
        "section_tax_withholding_entry",
        "tax_withholding_group",
        "tax_withholding_entries",
        "ignore_tax_withholding_threshold",
        "override_tax_withholding_entries",
        "column_break_hcca",
        "section_break_ttrv",
        "column_break_peap",
        "base_totals_section",
        "totals_section",
        "automation_section",
        "section_break_hzux",
        "title",
    ];

    pub fn status_updater() -> [StatusUpdaterSpec; 1] {
        [StatusUpdaterSpec {
            source_dt: "Purchase Invoice Item",
            target_dt: "Purchase Order Item",
            join_field: "po_detail",
            target_field: "billed_amt",
            target_parent_dt: "Purchase Order",
            target_parent_field: "per_billed",
            target_ref_field: "amount",
            source_field: "amount",
            percent_join_field: "purchase_order",
            overflow_type: "billing",
        }]
    }

    pub fn onload_plan(&self, is_new: bool, supplier_tax: Option<SupplierTaxMeta>) -> OnloadPlan {
        let apply_tds = self.supplier.is_some()
            && supplier_tax
                .map(|tax| {
                    tax.tax_withholding_category
                        .filter(|value| !value.is_empty())
                        .is_some()
                        || tax
                            .tax_withholding_group
                            .filter(|value| !value.is_empty())
                            .is_some()
                })
                .unwrap_or(false);
        OnloadPlan {
            apply_tds,
            clear_tax_withholding_entries: is_new,
        }
    }

    pub fn before_save(&mut self) {
        if !self.on_hold {
            self.release_date = None;
        }
    }

    pub fn invoice_is_blocked(&self, today: &str) -> bool {
        self.on_hold
            && self
                .release_date
                .as_deref()
                .map(|release_date| release_date > today)
                .unwrap_or(true)
    }

    pub fn validate_core(
        &mut self,
        today: &str,
        base_grand_total_precision: u32,
        accounts: &HashMap<String, AccountMeta>,
    ) -> Result<(), PurchaseInvoiceError> {
        if self.is_opening.is_empty() {
            self.is_opening = "No".to_string();
        }
        self.validate_release_date(today)?;
        if self.is_paid == 1 {
            self.validate_cash(base_grand_total_precision)?;
        }
        self.validate_credit_to_acc(accounts)?;
        self.set_status(None, today);
        self.set_percentage_received();
        Ok(())
    }

    pub fn validate_release_date(&self, today: &str) -> Result<(), PurchaseInvoiceError> {
        if self
            .release_date
            .as_deref()
            .map(|release_date| today >= release_date)
            .unwrap_or(false)
        {
            return Err(PurchaseInvoiceError::ReleaseDateMustBeFuture);
        }
        Ok(())
    }

    pub fn validate_cash(
        &self,
        base_grand_total_precision: u32,
    ) -> Result<(), PurchaseInvoiceError> {
        if self.cash_bank_account.is_none() && self.paid_amount != 0.0 {
            return Err(PurchaseInvoiceError::CashBankAccountRequired);
        }

        let total = non_zero_or(self.rounded_total, self.grand_total);
        let tolerance = 1.0 / 10_f64.powi(base_grand_total_precision as i32 + 1);
        if self.paid_amount + self.write_off_amount - total > tolerance {
            return Err(PurchaseInvoiceError::PaidAndWriteOffGreaterThanGrandTotal);
        }
        Ok(())
    }

    pub fn validate_credit_to_acc(
        &mut self,
        accounts: &HashMap<String, AccountMeta>,
    ) -> Result<(), PurchaseInvoiceError> {
        let Some(credit_to) = self.credit_to.as_deref() else {
            return Ok(());
        };
        let Some(account) = accounts.get(credit_to) else {
            return Err(PurchaseInvoiceError::CreditToAccountMissing(
                credit_to.to_string(),
            ));
        };
        if account.report_type.as_deref() != Some("Balance Sheet") {
            return Err(PurchaseInvoiceError::CreditToMustBeBalanceSheet);
        }
        if self.supplier.is_some() && account.account_type.as_deref() != Some("Payable") {
            return Err(PurchaseInvoiceError::CreditToMustBePayable);
        }
        self.party_account_currency = account.account_currency.clone();
        Ok(())
    }

    pub fn create_remarks(&mut self) {
        if self.remarks.is_none() {
            if let Some(bill_no) = self.bill_no.as_deref() {
                let mut remarks = format!("Against Supplier Invoice {bill_no}");
                if let Some(bill_date) = self.bill_date.as_deref() {
                    remarks.push_str(&format!(" dated {bill_date}"));
                }
                self.remarks = Some(remarks);
            }
        }
    }

    pub fn set_percentage_received(&mut self) {
        let mut total_billed_qty = 0.0;
        let mut total_received_qty = 0.0;
        for row in &self.items {
            if row.purchase_receipt.is_some() && row.pr_detail.is_some() && row.received_qty != 0.0
            {
                total_billed_qty += row.qty;
                total_received_qty += row.received_qty;
            }
        }
        if total_billed_qty != 0.0 && total_received_qty != 0.0 {
            self.per_received = total_received_qty / total_billed_qty * 100.0;
        }
    }

    pub fn set_status(&mut self, status: Option<&str>, today: &str) -> String {
        if self.is_new {
            if self.amended_from.is_some() {
                self.status = "Draft".to_string();
            }
            return self.status.clone();
        }

        let outstanding_amount = self.outstanding_amount;
        let total = self.total_in_party_account_currency();

        if status.is_some() {
            return self.status.clone();
        }

        if self.docstatus == 2 {
            return self.status.clone();
        }

        self.status = if self.docstatus == 1 {
            if self.internal_transfer {
                "Internal Transfer".to_string()
            } else if self.is_overdue(today) {
                "Overdue".to_string()
            } else if outstanding_amount > 0.0 && outstanding_amount < total {
                "Partly Paid".to_string()
            } else if outstanding_amount > 0.0 && self.due_date.as_deref().unwrap_or(today) >= today
            {
                "Unpaid".to_string()
            } else if self.is_return == 0 && self.has_submitted_debit_note {
                "Debit Note Issued".to_string()
            } else if self.is_return == 1 {
                "Return".to_string()
            } else if outstanding_amount <= 0.0 {
                "Paid".to_string()
            } else {
                "Submitted".to_string()
            }
        } else {
            "Draft".to_string()
        };
        self.status.clone()
    }

    fn total_in_party_account_currency(&self) -> f64 {
        if self.party_account_currency.as_deref() != Some(self.currency.as_str()) {
            if self.disable_rounded_total {
                self.base_grand_total
            } else {
                self.base_rounded_total.unwrap_or(0.0)
            }
        } else if self.disable_rounded_total {
            self.grand_total
        } else {
            self.rounded_total.unwrap_or(0.0)
        }
    }

    fn is_overdue(&self, today: &str) -> bool {
        self.outstanding_amount > 0.0
            && self
                .due_date
                .as_deref()
                .map(|due_date| due_date < today)
                .unwrap_or(false)
    }

    pub fn block_invoice(&mut self, hold_comment: Option<String>, release_date: Option<String>) {
        self.on_hold = true;
        self.hold_comment = hold_comment;
        self.release_date = release_date;
    }

    pub fn unblock_invoice(&mut self) {
        self.on_hold = false;
        self.release_date = None;
    }
}

impl DocumentController for PurchaseInvoice {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "validate",
            "before_save",
            "before_submit",
            "on_submit",
            "on_update_after_submit",
            "before_cancel",
            "on_cancel",
        ]
    }
}

fn non_zero_or(value: Option<f64>, fallback: f64) -> f64 {
    match value {
        Some(value) if value != 0.0 => value,
        _ => fallback,
    }
}
