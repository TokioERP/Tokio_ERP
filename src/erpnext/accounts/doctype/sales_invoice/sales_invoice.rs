use crate::erpnext::DocumentController;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SalesInvoiceStatusUpdaterSpec {
    pub source_dt: &'static str,
    pub target_field: &'static str,
    pub target_ref_field: &'static str,
    pub target_dt: &'static str,
    pub join_field: &'static str,
    pub target_parent_dt: &'static str,
    pub target_parent_field: &'static str,
    pub source_field: &'static str,
    pub percent_join_field: &'static str,
    pub status_field: &'static str,
    pub keyword: &'static str,
    pub overflow_type: &'static str,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomerTaxMeta {
    pub tax_withholding_category: Option<String>,
    pub tax_withholding_group: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OnloadPlan {
    pub apply_tds: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Indicator {
    pub title: String,
    pub color: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentRow {
    pub idx: usize,
    pub mode_of_payment: Option<String>,
    pub amount: f64,
    pub base_amount: f64,
    pub account: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SalesInvoiceItemRow {
    pub sales_order: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SalesInvoiceError {
    PaidAndWriteOffGreaterThanGrandTotal,
    PaymentAmountMustBePositive { row: usize },
    PaymentAmountMustBeNegative { row: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub struct SalesInvoice {
    pub name: Option<String>,
    pub is_new: bool,
    pub amended_from: Option<String>,
    pub docstatus: i32,
    pub customer: Option<String>,
    pub company: Option<String>,
    pub currency: String,
    pub party_account_currency: Option<String>,
    pub is_pos: bool,
    pub is_return: i32,
    pub is_discounted: bool,
    pub is_opening: String,
    pub is_debit_note: bool,
    pub is_created_using_pos: bool,
    pub is_consolidated: bool,
    pub update_stock: i32,
    pub has_subcontracted: bool,
    pub po_no: Option<String>,
    pub po_date: Option<String>,
    pub write_off_account: Option<String>,
    pub paid_amount: f64,
    pub base_paid_amount: f64,
    pub write_off_amount: f64,
    pub grand_total: f64,
    pub rounded_total: Option<f64>,
    pub base_grand_total: f64,
    pub base_rounded_total: Option<f64>,
    pub disable_rounded_total: bool,
    pub outstanding_amount: f64,
    pub due_date: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub internal_transfer: bool,
    pub has_submitted_credit_note: bool,
    pub conversion_rate: f64,
    pub payments: Vec<PaymentRow>,
    pub items: Vec<SalesInvoiceItemRow>,
}

impl Default for SalesInvoice {
    fn default() -> Self {
        Self {
            name: None,
            is_new: false,
            amended_from: None,
            docstatus: 0,
            customer: None,
            company: None,
            currency: String::new(),
            party_account_currency: None,
            is_pos: false,
            is_return: 0,
            is_discounted: false,
            is_opening: String::new(),
            is_debit_note: false,
            is_created_using_pos: false,
            is_consolidated: false,
            update_stock: 0,
            has_subcontracted: false,
            po_no: None,
            po_date: None,
            write_off_account: None,
            paid_amount: 0.0,
            base_paid_amount: 0.0,
            write_off_amount: 0.0,
            grand_total: 0.0,
            rounded_total: None,
            base_grand_total: 0.0,
            base_rounded_total: None,
            disable_rounded_total: false,
            outstanding_amount: 0.0,
            due_date: None,
            remarks: None,
            status: String::new(),
            internal_transfer: false,
            has_submitted_credit_note: false,
            conversion_rate: 0.0,
            payments: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl SalesInvoice {
    pub const DOCTYPE: &'static str = "Sales Invoice";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const TITLE_FIELD: &'static str = "customer_name";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const IS_SUBMITTABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "customer_section",
        "naming_series",
        "customer",
        "customer_name",
        "tax_id",
        "project",
        "is_pos",
        "pos_profile",
        "is_return",
        "column_break1",
        "company",
        "cost_center",
        "posting_date",
        "posting_time",
        "set_posting_time",
        "due_date",
        "amended_from",
        "return_against",
        "update_billed_amount_in_sales_order",
        "customer_po_details",
        "po_no",
        "column_break_23",
        "po_date",
        "address_and_contact",
        "customer_address",
        "address_display",
        "contact_person",
        "contact_display",
        "contact_mobile",
        "contact_email",
        "territory",
        "col_break4",
        "shipping_address_name",
        "shipping_address",
        "company_address",
        "company_address_display",
        "currency_and_price_list",
        "currency",
        "conversion_rate",
        "column_break2",
        "selling_price_list",
        "price_list_currency",
        "plc_conversion_rate",
        "ignore_pricing_rule",
        "set_warehouse",
        "items_section",
        "update_stock",
        "scan_barcode",
        "items",
        "pricing_rule_details",
        "pricing_rules",
        "packing_list",
        "packed_items",
        "product_bundle_help",
        "time_sheet_list",
        "timesheets",
        "total_billing_amount",
        "section_break_30",
        "total_qty",
        "base_total",
        "base_net_total",
        "column_break_32",
        "total",
        "net_total",
        "total_net_weight",
        "taxes_section",
        "taxes_and_charges",
        "column_break_38",
        "shipping_rule",
        "tax_category",
        "section_break_40",
        "taxes",
        "sec_tax_breakup",
        "other_charges_calculation",
        "section_break_43",
        "base_total_taxes_and_charges",
        "column_break_47",
        "total_taxes_and_charges",
        "loyalty_points_redemption",
        "loyalty_points",
        "loyalty_amount",
        "redeem_loyalty_points",
        "column_break_77",
        "loyalty_program",
        "loyalty_redemption_account",
        "loyalty_redemption_cost_center",
        "apply_discount_on",
        "base_discount_amount",
        "column_break_51",
        "additional_discount_percentage",
        "discount_amount",
        "base_grand_total",
        "base_rounding_adjustment",
        "base_rounded_total",
        "base_in_words",
        "column_break5",
        "grand_total",
        "rounding_adjustment",
        "rounded_total",
        "in_words",
        "total_advance",
        "outstanding_amount",
        "advances_section",
        "allocate_advances_automatically",
        "get_advances",
        "advances",
        "payment_schedule_section",
        "payment_terms_template",
        "payment_schedule",
        "payments_section",
        "cash_bank_account",
        "payments",
        "section_break_84",
        "base_paid_amount",
        "column_break_86",
        "paid_amount",
        "section_break_88",
        "base_change_amount",
        "column_break_90",
        "change_amount",
        "account_for_change_amount",
        "write_off_amount",
        "base_write_off_amount",
        "write_off_outstanding_amount_automatically",
        "column_break_74",
        "write_off_account",
        "write_off_cost_center",
        "terms_section_break",
        "tc_name",
        "terms",
        "edit_printing_settings",
        "letter_head",
        "group_same_items",
        "language",
        "column_break_84",
        "select_print_heading",
        "more_information",
        "inter_company_invoice_reference",
        "customer_group",
        "is_discounted",
        "status",
        "more_info",
        "debit_to",
        "party_account_currency",
        "is_opening",
        "column_break8",
        "remarks",
        "sales_partner",
        "column_break10",
        "commission_rate",
        "total_commission",
        "sales_team",
        "subscription_section",
        "from_date",
        "to_date",
        "column_break_140",
        "auto_repeat",
        "update_auto_repeat_reference",
        "against_income_account",
        "accounting_dimensions_section",
        "dimension_col_break",
        "is_consolidated",
        "is_internal_customer",
        "company_tax_id",
        "unrealized_profit_loss_account",
        "represents_company",
        "set_target_warehouse",
        "is_debit_note",
        "disable_rounded_total",
        "additional_discount_account",
        "dispatch_address_name",
        "dispatch_address",
        "ignore_default_payment_terms_template",
        "total_billing_hours",
        "amount_eligible_for_commission",
        "subscription",
        "is_cash_or_non_trade_discount",
        "contact_and_address_tab",
        "payments_tab",
        "terms_tab",
        "more_info_tab",
        "connections_tab",
        "column_break_14",
        "column_break_39",
        "section_break_42",
        "column_break_55",
        "shipping_address_section",
        "company_address_section",
        "shipping_addr_col_break",
        "company_addr_col_break",
        "column_break_52",
        "section_break_104",
        "column_break_106",
        "write_off_section",
        "incoterm",
        "named_place",
        "only_include_allocated_payments",
        "use_company_roundoff_cost_center",
        "update_billed_amount_in_delivery_note",
        "dont_create_loyalty_points",
        "coupon_code",
        "update_outstanding_for_self",
        "column_break_imbx",
        "utm_medium",
        "utm_content",
        "utm_campaign",
        "utm_source",
        "company_contact_person",
        "is_created_using_pos",
        "pos_closing_entry",
        "last_scanned_warehouse",
        "has_subcontracted",
        "item_wise_tax_details",
        "apply_tds",
        "section_tax_withholding_entry",
        "tax_withholding_group",
        "tax_withholding_entries",
        "ignore_tax_withholding_threshold",
        "override_tax_withholding_entries",
        "totals_section",
        "base_totals_section",
        "column_break_xjag",
        "additional_discount_section",
        "sales_team_section",
        "commission_section",
        "column_break_ixxw",
        "utm_analytics_section",
        "automation_section",
        "section_break_pxwz",
        "column_break_rdke",
        "column_break_rdiw",
        "column_break_iaso",
        "section_break_qllv",
        "title",
    ];

    pub fn status_updater() -> [SalesInvoiceStatusUpdaterSpec; 1] {
        [SalesInvoiceStatusUpdaterSpec {
            source_dt: "Sales Invoice Item",
            target_field: "billed_amt",
            target_ref_field: "amount",
            target_dt: "Sales Order Item",
            join_field: "so_detail",
            target_parent_dt: "Sales Order",
            target_parent_field: "per_billed",
            source_field: "amount",
            percent_join_field: "sales_order",
            status_field: "billing_status",
            keyword: "Billed",
            overflow_type: "billing",
        }]
    }

    pub fn onload_plan(&self, customer_tax: Option<CustomerTaxMeta>) -> OnloadPlan {
        let apply_tds = self.customer.is_some()
            && customer_tax
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
        OnloadPlan { apply_tds }
    }

    pub fn set_indicator(&self, today: &str) -> Indicator {
        if self.outstanding_amount < 0.0 {
            Indicator::new("Credit Note Issued", "gray")
        } else if self.outstanding_amount > 0.0
            && self.due_date.as_deref().unwrap_or(today) >= today
        {
            Indicator::new("Unpaid", "orange")
        } else if self.outstanding_amount > 0.0 && self.due_date.as_deref().unwrap_or(today) < today
        {
            Indicator::new("Overdue", "red")
        } else if self.is_return == 1 {
            Indicator::new("Return", "gray")
        } else {
            Indicator::new("Paid", "green")
        }
    }

    pub fn set_paid_amount(&mut self, base_paid_amount_precision: u32) {
        let mut paid_amount = 0.0;
        let mut base_paid_amount = 0.0;
        for payment in &mut self.payments {
            payment.base_amount = round_to(
                payment.amount * self.conversion_rate,
                base_paid_amount_precision,
            );
            paid_amount += payment.amount;
            base_paid_amount += payment.base_amount;
        }
        self.paid_amount = paid_amount;
        self.base_paid_amount = base_paid_amount;
    }

    pub fn add_remarks(&mut self) {
        if self.remarks.is_none() {
            if let Some(po_no) = self.po_no.as_deref() {
                let mut remarks = format!("Against Customer Order {po_no}");
                if let Some(po_date) = self.po_date.as_deref() {
                    remarks.push_str(&format!(" dated {po_date}"));
                }
                self.remarks = Some(remarks);
            }
        }
    }

    pub fn validate_pos_return_totals(
        &self,
        grand_total_precision: u32,
    ) -> Result<(), SalesInvoiceError> {
        if self.is_return == 1 {
            let invoice_total = non_zero_or(self.rounded_total, self.grand_total);
            let tolerance = 1.0 / 10_f64.powf(grand_total_precision as f64 + 1.0);
            if self.paid_amount.abs() + self.write_off_amount.abs() - invoice_total.abs()
                > tolerance
            {
                return Err(SalesInvoiceError::PaidAndWriteOffGreaterThanGrandTotal);
            }
        }
        Ok(())
    }

    pub fn allow_write_off_only_on_pos(&mut self) {
        if !self.is_pos && self.write_off_account.is_some() {
            self.write_off_account = None;
        }
    }

    pub fn is_subcontracted(&mut self, has_subcontracted_sales_order: bool) -> bool {
        if !self.has_subcontracted {
            self.has_subcontracted = has_subcontracted_sales_order
                && self.items.iter().any(|item| item.sales_order.is_some());
        }
        if self.has_subcontracted {
            self.update_stock = 0;
        }
        self.has_subcontracted
    }

    pub fn verify_payment_amount_is_positive(&self) -> Result<(), SalesInvoiceError> {
        for payment in &self.payments {
            if payment.amount < 0.0 {
                return Err(SalesInvoiceError::PaymentAmountMustBePositive { row: payment.idx });
            }
        }
        Ok(())
    }

    pub fn verify_payment_amount_is_negative(&self) -> Result<(), SalesInvoiceError> {
        for payment in &self.payments {
            if payment.amount > 0.0 {
                return Err(SalesInvoiceError::PaymentAmountMustBeNegative { row: payment.idx });
            }
        }
        Ok(())
    }

    pub fn set_status(
        &mut self,
        status: Option<&str>,
        today: &str,
        discounting_status: Option<&str>,
    ) -> String {
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
            } else if self.is_return == 0 && self.has_submitted_credit_note {
                "Credit Note Issued".to_string()
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

        if matches!(self.status.as_str(), "Unpaid" | "Partly Paid" | "Overdue")
            && self.is_discounted
            && discounting_status == Some("Disbursed")
        {
            self.status.push_str(" and Discounted");
        }

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
}

impl Indicator {
    fn new(title: &str, color: &str) -> Self {
        Self {
            title: title.to_string(),
            color: color.to_string(),
        }
    }
}

impl DocumentController for SalesInvoice {
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

fn round_to(value: f64, precision: u32) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).round() / multiplier
}
