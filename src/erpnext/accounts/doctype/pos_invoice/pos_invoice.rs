use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoice {
    pub name: Option<String>,
    pub company: Option<String>,
    pub customer: Option<String>,
    pub pos_profile: Option<String>,
    pub contact_mobile: Option<String>,
    pub debit_to: Option<String>,
    pub party_account_currency: Option<String>,
    pub is_pos: bool,
    pub is_return: bool,
    pub docstatus: i32,
    pub status: String,
    pub due_date: Option<String>,
    pub posting_date: Option<String>,
    pub is_discounted: bool,
    pub discounting_status: Option<String>,
    pub has_submitted_return: bool,
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
    pub redeem_loyalty_points: bool,
    pub loyalty_points: i32,
    pub loyalty_redemption_account: Option<String>,
    pub loyalty_redemption_cost_center: Option<String>,
    pub return_against: Option<String>,
    pub coupon_code: Option<String>,
    pub items: Vec<PosInvoiceItem>,
    pub payments: Vec<PosInvoicePayment>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoiceItem {
    pub name: Option<String>,
    pub idx: i32,
    pub item_code: String,
    pub warehouse: Option<String>,
    pub stock_qty: f64,
    pub qty: f64,
    pub has_serial_no: bool,
    pub has_batch_no: bool,
    pub use_serial_batch_fields: bool,
    pub serial_and_batch_bundle: Option<String>,
    pub serial_no: Option<String>,
    pub batch_no: Option<String>,
    pub pos_invoice_item: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoicePayment {
    pub idx: i32,
    pub mode_of_payment: Option<String>,
    pub payment_type: String,
    pub account: Option<String>,
    pub amount: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosProfile {
    pub name: String,
    pub company: String,
    pub customer: Option<String>,
    pub currency: Option<String>,
    pub warehouse: Option<String>,
    pub account_for_change_amount: Option<String>,
    pub print_format: Option<String>,
    pub allow_print_before_pay: bool,
    pub set_grand_total_to_default_mop: bool,
    pub utm_source: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_medium: Option<String>,
    pub selling_price_list: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MissingValuesPlan {
    pub pos_profile: String,
    pub company: Option<String>,
    pub customer: Option<String>,
    pub debit_to: Option<String>,
    pub party_account_currency: Option<String>,
    pub due_date: Option<String>,
    pub print_format: Option<String>,
    pub allow_print_before_pay: bool,
    pub set_default_payment: bool,
    pub utm_source: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_medium: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentRequestPlan {
    pub use_existing_request: bool,
    pub reference_doctype: String,
    pub reference_name: String,
    pub recipient_id: Option<String>,
    pub mode_of_payment: Option<String>,
    pub payment_account: Option<String>,
    pub payment_request_type: String,
    pub party_type: String,
    pub party: Option<String>,
    pub return_doc: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdatePaymentsPlan {
    pub new_paid_amount: f64,
    pub new_base_paid_amount: f64,
    pub new_outstanding_amount: f64,
    pub new_change_amount: f64,
    pub added_payment_count: usize,
    pub set_status_update: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MergeLogInvoiceInput {
    pub name: String,
    pub customer: String,
    pub posting_date: String,
    pub grand_total: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MergeLogPlan {
    pub posting_date_source: String,
    pub customer: Option<String>,
    pub invoices: Vec<MergeLogInvoiceInput>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemQueryPlan {
    pub doctype: String,
    pub txt: String,
    pub searchfield: String,
    pub start: i32,
    pub page_len: i32,
    pub item_groups: Option<Vec<String>>,
    pub as_dict: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SalesInvoicePaymentPlan {
    pub idx: i32,
    pub mode_of_payment: Option<String>,
    pub amount: f64,
    pub base_amount: f64,
    pub parent: Option<String>,
    pub parentfield: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClearUnallocatedPaymentsPlan {
    pub kept_payments: Vec<PosInvoicePayment>,
    pub delete_zero_payments_for_parent: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReturnSalesInvoiceItemPlan {
    pub source_name: Option<String>,
    pub pos_invoice: Option<String>,
    pub pos_invoice_item: Option<String>,
    pub sales_invoice_item: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReturnSalesInvoicePlan {
    pub source_pos_invoice: Option<String>,
    pub is_pos: bool,
    pub is_return: bool,
    pub is_created_using_pos: bool,
    pub is_consolidated: bool,
    pub return_against: Option<String>,
    pub items: Vec<ReturnSalesInvoiceItemPlan>,
    pub payment_count: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConsolidatedSalesInvoicePlan {
    pub sales_invoice_name: String,
    pub db_set_consolidated_invoice: Option<String>,
    pub set_status_update: bool,
    pub return_sales_invoice: ReturnSalesInvoicePlan,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SerialBatchBundlePlan {
    pub bundle: String,
    pub clear_voucher_no: bool,
    pub cancel_bundle: bool,
    pub clear_row_link: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SerialBatchBundleSubmitPlan {
    pub table_name: String,
    pub bundle: String,
    pub ignore_voucher_validation: bool,
    pub submit: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetPosFieldsPlan {
    pub pos_profile: Option<String>,
    pub company: Option<String>,
    pub customer: Option<String>,
    pub account_for_change_amount: Option<String>,
    pub set_warehouse: Option<String>,
    pub update_multi_mode_option: bool,
    pub add_return_modes: bool,
    pub selling_price_list: Option<String>,
    pub currency: Option<String>,
    pub item_defaults_count: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OnSubmitPlan {
    pub actions: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoyaltyTransactionPlan {
    pub loyalty_redemption_account: Option<String>,
    pub loyalty_redemption_cost_center: Option<String>,
    pub validate_loyalty_points: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosInvoiceError {
    Validation(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BundleAvailabilityRow {
    pub item_code: String,
    pub required: f64,
    pub available: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProductBundleItem {
    pub item_code: String,
    pub qty: f64,
    pub bin_qty: f64,
    pub is_stock_item: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StockAvailability {
    pub item_code: String,
    pub warehouse: String,
    pub available: f64,
    pub is_stock_item: bool,
    pub is_negative_stock_allowed: bool,
    pub bundle_items: Vec<BundleAvailabilityRow>,
}

impl StockAvailability {
    pub fn bundle(
        item_code: &str,
        warehouse: &str,
        bundle_items: Vec<BundleAvailabilityRow>,
        is_negative_stock_allowed: bool,
    ) -> Self {
        Self {
            item_code: item_code.to_string(),
            warehouse: warehouse.to_string(),
            available: 0.0,
            is_stock_item: true,
            is_negative_stock_allowed,
            bundle_items,
        }
    }

    pub fn bundle_error(&self, row_idx: i32) -> Option<String> {
        let errors = self
            .bundle_items
            .iter()
            .filter(|item| item.available < item.required)
            .map(|item| {
                format!(
                    "<li>Packed Item {}: Required {}, Available {}</li>",
                    item.item_code,
                    format_number(item.required),
                    format_number(item.available)
                )
            })
            .collect::<Vec<_>>();
        if errors.is_empty() {
            return None;
        }
        Some(format!(
            "<b>Row #{}:</b> Bundle {} in warehouse {} has insufficient packed items:<br><div style='margin-top: 15px;'><ul style='line-height: 0.8;'>{}</ul></div>",
            row_idx,
            self.item_code,
            self.warehouse,
            errors.join("<br>")
        ))
    }
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
                && self.is_discounted
                && self.discounting_status.as_deref() == Some("Disbursed")
            {
                "Overdue and Discounted".to_string()
            } else if self.outstanding_amount > 0.0
                && self.due_date.as_deref().unwrap_or(today) < today
            {
                "Overdue".to_string()
            } else if self.outstanding_amount > 0.0
                && self.outstanding_amount < total
                && self.is_discounted
                && self.discounting_status.as_deref() == Some("Disbursed")
            {
                "Partly Paid and Discounted".to_string()
            } else if self.outstanding_amount > 0.0 && self.outstanding_amount < total {
                "Partly Paid".to_string()
            } else if self.outstanding_amount > 0.0
                && self.due_date.as_deref().unwrap_or(today) >= today
                && self.is_discounted
                && self.discounting_status.as_deref() == Some("Disbursed")
            {
                "Unpaid and Discounted".to_string()
            } else if self.outstanding_amount > 0.0 {
                "Unpaid".to_string()
            } else if self.outstanding_amount <= 0.0 && !self.is_return && self.has_submitted_return
            {
                "Credit Note Issued".to_string()
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
        ];
        if !self.is_return && self.loyalty_program.is_some() {
            plan.push("delete_loyalty_point_entry".to_string());
        } else if self.is_return && self.return_against.is_some() && self.loyalty_program.is_some()
        {
            plan.push("return_against_delete_loyalty_point_entry".to_string());
            plan.push("return_against_make_loyalty_point_entry".to_string());
        }
        plan.push("set_status:Cancelled".to_string());
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

    pub fn validate_is_pos_using_sales_invoice(
        &self,
        invoice_type_in_pos: &str,
    ) -> Result<(), PosInvoiceError> {
        if invoice_type_in_pos == "Sales Invoice" && !self.is_return {
            return Err(PosInvoiceError::Validation(
                "Sales Invoice mode is activated in POS. Please create Sales Invoice instead."
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_serialised_or_batched_item(&self) -> Result<(), PosInvoiceError> {
        for item in &self.items {
            if item.has_serial_no
                && ((!item.use_serial_batch_fields && item.serial_and_batch_bundle.is_none())
                    || (item.use_serial_batch_fields && item.serial_no.is_none()))
            {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{}: Please select Serial No. for item {}",
                    item.idx, item.item_code
                )));
            }
            if item.has_batch_no
                && ((!item.use_serial_batch_fields && item.serial_and_batch_bundle.is_none())
                    || (item.use_serial_batch_fields && item.batch_no.is_none()))
            {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{}: Please select Batch No. for item {}",
                    item.idx, item.item_code
                )));
            }
        }
        Ok(())
    }

    pub fn validate_return_items_qty(
        &self,
        original_serial_nos: &[String],
    ) -> Result<(), PosInvoiceError> {
        if !self.is_return {
            return Ok(());
        }
        for item in &self.items {
            if item.qty > 0.0 {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{}: You cannot add positive quantities in a return invoice. Please remove item {} to complete the return.",
                    item.idx, item.item_code
                )));
            }
            if let Some(serial_no) = item.serial_no.as_deref() {
                for sr in serial_no.lines() {
                    if !original_serial_nos.iter().any(|existing| existing == sr) {
                        return Err(PosInvoiceError::Validation(format!(
                            "Row #{}: Serial No {} cannot be returned since it was not transacted in original invoice {}",
                            item.idx,
                            sr,
                            self.return_against.as_deref().unwrap_or_default()
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn validate_stock_availability(
        &self,
        validate_stock_on_save: bool,
        stock: &[StockAvailability],
    ) -> Result<(), PosInvoiceError> {
        if self.is_return || (self.docstatus == 0 && !validate_stock_on_save) {
            return Ok(());
        }
        for item in &self.items {
            if item.serial_and_batch_bundle.is_some() {
                continue;
            }
            let warehouse = item.warehouse.as_deref().unwrap_or_default();
            let availability = stock
                .iter()
                .find(|row| row.item_code == item.item_code && row.warehouse == warehouse);
            let Some(availability) = availability else {
                continue;
            };
            if availability.is_negative_stock_allowed {
                continue;
            }
            if let Some(message) = availability.bundle_error(item.idx) {
                return Err(PosInvoiceError::Validation(message));
            }
            if availability.is_stock_item && availability.available <= 0.0 {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{}: Item {} has no stock in warehouse {}.",
                    item.idx, item.item_code, warehouse
                )));
            }
            if availability.is_stock_item && availability.available < item.stock_qty {
                return Err(PosInvoiceError::Validation(format!(
                    "Row #{}: Item {} in warehouse {}: Available {}, Needed {}.",
                    item.idx,
                    item.item_code,
                    warehouse,
                    format_number(availability.available),
                    format_number(item.stock_qty)
                )));
            }
        }
        Ok(())
    }

    pub fn set_missing_values_plan(
        &self,
        profile: &PosProfile,
        pos_invoice_print_disabled: bool,
        party_account: Option<&str>,
        party_account_currency: Option<&str>,
        due_date: Option<&str>,
    ) -> MissingValuesPlan {
        let debit_to = self
            .debit_to
            .clone()
            .or_else(|| party_account.map(str::to_string));
        let party_account_currency = self
            .party_account_currency
            .clone()
            .or_else(|| party_account_currency.map(str::to_string));
        let due_date = self
            .due_date
            .clone()
            .or_else(|| due_date.map(str::to_string));
        let print_format = profile.print_format.clone().or_else(|| {
            if pos_invoice_print_disabled {
                None
            } else {
                Some(Self::DOCTYPE.to_string())
            }
        });

        MissingValuesPlan {
            pos_profile: profile.name.clone(),
            company: self
                .company
                .clone()
                .or_else(|| Some(profile.company.clone())),
            customer: self.customer.clone().or_else(|| profile.customer.clone()),
            debit_to,
            party_account_currency,
            due_date,
            print_format,
            allow_print_before_pay: profile.allow_print_before_pay,
            set_default_payment: profile.set_grand_total_to_default_mop,
            utm_source: profile.utm_source.clone(),
            utm_campaign: profile.utm_campaign.clone(),
            utm_medium: profile.utm_medium.clone(),
        }
    }

    pub fn reset_mode_of_payments_plan(&self) -> Option<&'static str> {
        self.pos_profile
            .as_ref()
            .map(|_| "update_multi_mode_option")
    }

    pub fn create_payment_request_plan(
        &self,
        existing_payment_request: bool,
    ) -> Result<PaymentRequestPlan, PosInvoiceError> {
        for pay in &self.payments {
            if pay.payment_type == "Phone" {
                if pay.amount <= 0.0 {
                    return Err(PosInvoiceError::Validation(
                        "Payment amount cannot be less than or equal to 0".to_string(),
                    ));
                }
                if self
                    .contact_mobile
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                {
                    return Err(PosInvoiceError::Validation(
                        "Please enter the phone number first".to_string(),
                    ));
                }

                return Ok(PaymentRequestPlan {
                    use_existing_request: existing_payment_request,
                    reference_doctype: Self::DOCTYPE.to_string(),
                    reference_name: self.name.clone().unwrap_or_default(),
                    recipient_id: self.contact_mobile.clone(),
                    mode_of_payment: pay.mode_of_payment.clone(),
                    payment_account: pay.account.clone(),
                    payment_request_type: "Inward".to_string(),
                    party_type: "Customer".to_string(),
                    party: self.customer.clone(),
                    return_doc: true,
                });
            }
        }
        Err(PosInvoiceError::Validation(
            "No phone payment found".to_string(),
        ))
    }

    pub fn update_payments_plan(
        &self,
        payments: &[PosInvoicePayment],
        precision: u32,
    ) -> Result<UpdatePaymentsPlan, PosInvoiceError> {
        if self.status == "Consolidated" {
            return Err(PosInvoiceError::Validation(
                "Create Payment Entry for Consolidated POS Invoices.".to_string(),
            ));
        }

        let total = self.rounded_total.unwrap_or(self.grand_total);
        if self.paid_amount >= total {
            return Err(PosInvoiceError::Validation(
                "This invoice has already been paid.".to_string(),
            ));
        }

        let mut paid_amount = self.paid_amount;
        for payment in payments {
            paid_amount += payment.amount;
        }

        let new_paid_amount = round_to_precision(paid_amount, precision);
        let new_base_paid_amount =
            round_to_precision(new_paid_amount * self.conversion_rate, precision);
        let new_outstanding_amount = if total > new_paid_amount {
            round_to_precision(total - new_paid_amount, precision)
        } else {
            0.0
        };
        let new_change_amount = if new_paid_amount > total {
            round_to_precision(new_paid_amount - total, precision)
        } else {
            0.0
        };

        Ok(UpdatePaymentsPlan {
            new_paid_amount,
            new_base_paid_amount,
            new_outstanding_amount,
            new_change_amount,
            added_payment_count: payments.len(),
            set_status_update: true,
        })
    }

    pub fn clear_unallocated_mode_of_payments_plan(&self) -> ClearUnallocatedPaymentsPlan {
        ClearUnallocatedPaymentsPlan {
            kept_payments: self
                .payments
                .iter()
                .filter(|payment| payment.amount != 0.0)
                .cloned()
                .collect(),
            delete_zero_payments_for_parent: self.name.clone(),
        }
    }

    pub fn create_return_sales_invoice_plan(
        &self,
        consolidated_return_against: Option<&str>,
        consolidated_item_lookup: &[(String, String)],
    ) -> ReturnSalesInvoicePlan {
        ReturnSalesInvoicePlan {
            source_pos_invoice: self.name.clone(),
            is_pos: true,
            is_return: true,
            is_created_using_pos: true,
            is_consolidated: true,
            return_against: consolidated_return_against.map(str::to_string),
            items: self
                .items
                .iter()
                .map(|item| ReturnSalesInvoiceItemPlan {
                    source_name: item.name.clone(),
                    pos_invoice: self.name.clone(),
                    pos_invoice_item: item.name.clone(),
                    sales_invoice_item: item.pos_invoice_item.as_ref().and_then(|source_item| {
                        consolidated_item_lookup
                            .iter()
                            .find(|(pos_invoice_item, _)| pos_invoice_item == source_item)
                            .map(|(_, sales_invoice_item)| sales_invoice_item.clone())
                    }),
                })
                .collect(),
            payment_count: self.payments.len(),
        }
    }

    pub fn create_and_add_consolidated_sales_invoice_plan(
        &self,
        sales_invoice_name: &str,
        consolidated_return_against: Option<&str>,
        consolidated_item_lookup: &[(String, String)],
    ) -> ConsolidatedSalesInvoicePlan {
        ConsolidatedSalesInvoicePlan {
            sales_invoice_name: sales_invoice_name.to_string(),
            db_set_consolidated_invoice: Some(sales_invoice_name.to_string()),
            set_status_update: true,
            return_sales_invoice: self.create_return_sales_invoice_plan(
                consolidated_return_against,
                consolidated_item_lookup,
            ),
        }
    }

    pub fn delink_serial_and_batch_bundle_plan(&self) -> Vec<SerialBatchBundlePlan> {
        self.items
            .iter()
            .filter_map(|item| {
                item.serial_and_batch_bundle
                    .as_ref()
                    .map(|bundle| SerialBatchBundlePlan {
                        bundle: bundle.clone(),
                        clear_voucher_no: self.consolidated_invoice.is_none(),
                        cancel_bundle: true,
                        clear_row_link: true,
                    })
            })
            .collect()
    }

    pub fn submit_serial_batch_bundle_plan(
        &self,
        table_name: &str,
        draft_bundles: &[String],
    ) -> Vec<SerialBatchBundleSubmitPlan> {
        self.items
            .iter()
            .filter_map(|item| item.serial_and_batch_bundle.as_ref())
            .filter(|bundle| draft_bundles.iter().any(|draft| draft == *bundle))
            .map(|bundle| SerialBatchBundleSubmitPlan {
                table_name: table_name.to_string(),
                bundle: bundle.clone(),
                ignore_voucher_validation: true,
                submit: true,
            })
            .collect()
    }

    pub fn set_pos_fields_plan(
        &self,
        profile: Option<&PosProfile>,
        for_validate: bool,
        default_cash_account: Option<&str>,
        customer_price_list: Option<&str>,
        customer_group_price_list: Option<&str>,
        customer_currency: Option<&str>,
    ) -> Result<SetPosFieldsPlan, PosInvoiceError> {
        let Some(profile) = profile else {
            return Err(PosInvoiceError::Validation(
                "No POS Profile found. Please create a New POS Profile first".to_string(),
            ));
        };

        let customer = if !for_validate && self.customer.is_none() {
            profile.customer.clone()
        } else {
            self.customer.clone()
        };
        let selling_price_list = customer_price_list
            .or(customer_group_price_list)
            .map(str::to_string)
            .or_else(|| profile.selling_price_list.clone());
        let currency = customer_currency
            .filter(|currency| Some(*currency) != profile.currency.as_deref())
            .map(str::to_string)
            .or_else(|| profile.currency.clone());

        Ok(SetPosFieldsPlan {
            pos_profile: Some(profile.name.clone()),
            company: Some(profile.company.clone()),
            customer,
            account_for_change_amount: profile
                .account_for_change_amount
                .clone()
                .or_else(|| self.account_for_change_amount.clone())
                .or_else(|| default_cash_account.map(str::to_string)),
            set_warehouse: profile.warehouse.clone(),
            update_multi_mode_option: self.payments.is_empty() && !for_validate,
            add_return_modes: self.is_return && !for_validate,
            selling_price_list,
            currency,
            item_defaults_count: self
                .items
                .iter()
                .filter(|item| !item.item_code.is_empty())
                .count(),
        })
    }

    pub fn on_submit_plan(&self, invoice_type_in_pos: &str) -> OnSubmitPlan {
        let mut actions = Vec::new();
        if !self.is_return && self.loyalty_program.is_some() {
            actions.push("make_loyalty_point_entry".to_string());
        } else if self.is_return && self.return_against.is_some() && self.loyalty_program.is_some()
        {
            actions.push("return_against_delete_loyalty_point_entry".to_string());
            actions.push("return_against_make_loyalty_point_entry".to_string());
        }
        if self.redeem_loyalty_points && self.loyalty_points != 0 {
            actions.push("apply_loyalty_points".to_string());
        }
        actions.push("check_phone_payments".to_string());
        actions.push("set_status:update".to_string());
        actions.push("make_bundle_for_sales_purchase_return".to_string());
        for table_name in ["items", "packed_items"] {
            actions.push(format!(
                "make_bundle_using_old_serial_batch_fields:{table_name}"
            ));
            actions.push(format!("submit_serial_batch_bundle:{table_name}"));
        }
        if self.coupon_code.is_some() {
            actions.push("update_coupon_code_count:used".to_string());
        }
        actions.push("clear_unallocated_mode_of_payments".to_string());
        if self.is_return && invoice_type_in_pos == "Sales Invoice" {
            actions.push("create_and_add_consolidated_sales_invoice".to_string());
        }
        OnSubmitPlan { actions }
    }

    pub fn validate_loyalty_transaction_plan(
        &self,
        loyalty_expense_account: Option<&str>,
        loyalty_cost_center: Option<&str>,
    ) -> LoyaltyTransactionPlan {
        let loyalty_redemption_account = if self.redeem_loyalty_points {
            self.loyalty_redemption_account
                .clone()
                .or_else(|| loyalty_expense_account.map(str::to_string))
        } else {
            self.loyalty_redemption_account.clone()
        };
        let loyalty_redemption_cost_center = if self.redeem_loyalty_points {
            self.loyalty_redemption_cost_center
                .clone()
                .or_else(|| loyalty_cost_center.map(str::to_string))
        } else {
            self.loyalty_redemption_cost_center.clone()
        };

        LoyaltyTransactionPlan {
            loyalty_redemption_account,
            loyalty_redemption_cost_center,
            validate_loyalty_points: self.redeem_loyalty_points
                && self.loyalty_program.is_some()
                && self.loyalty_points != 0,
        }
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

pub fn get_stock_availability(
    item_code: &str,
    warehouse: &str,
    is_stock_item: bool,
    is_active_bundle: bool,
    bin_qty: f64,
    pos_sales_qty: f64,
    is_negative_stock_allowed: bool,
    bundle_availability: Option<f64>,
) -> StockAvailability {
    if is_stock_item {
        StockAvailability {
            item_code: item_code.to_string(),
            warehouse: warehouse.to_string(),
            available: bin_qty - pos_sales_qty,
            is_stock_item: true,
            is_negative_stock_allowed,
            bundle_items: Vec::new(),
        }
    } else if is_active_bundle {
        StockAvailability {
            item_code: item_code.to_string(),
            warehouse: warehouse.to_string(),
            available: bundle_availability.unwrap_or(0.0),
            is_stock_item: true,
            is_negative_stock_allowed: false,
            bundle_items: Vec::new(),
        }
    } else {
        StockAvailability {
            item_code: item_code.to_string(),
            warehouse: warehouse.to_string(),
            available: 0.0,
            is_stock_item: false,
            is_negative_stock_allowed: false,
            bundle_items: Vec::new(),
        }
    }
}

pub fn get_bundle_availability(
    _bundle_item_code: &str,
    _warehouse: &str,
    items: &[ProductBundleItem],
    pos_sales_qty: f64,
) -> f64 {
    let mut bundle_bin_qty = 1_000_000.0;
    for item in items {
        if item.is_stock_item {
            let max_available_bundles = item.bin_qty / item.qty;
            if bundle_bin_qty > max_available_bundles {
                bundle_bin_qty = max_available_bundles;
            }
        }
    }
    bundle_bin_qty - pos_sales_qty
}

pub fn get_pos_reserved_qty(
    pos_invoice_item_reserved_qty: f64,
    packed_item_reserved_qty: f64,
) -> f64 {
    pos_invoice_item_reserved_qty + packed_item_reserved_qty
}

pub fn make_sales_return_plan(source_name: &str) -> (String, String) {
    (PosInvoice::DOCTYPE.to_string(), source_name.to_string())
}

pub fn make_merge_log_plan(
    invoices: Vec<MergeLogInvoiceInput>,
) -> Result<MergeLogPlan, PosInvoiceError> {
    if invoices.is_empty() {
        return Err(PosInvoiceError::Validation(
            "At least one invoice has to be selected.".to_string(),
        ));
    }
    Ok(MergeLogPlan {
        posting_date_source: "today".to_string(),
        customer: invoices.last().map(|invoice| invoice.customer.clone()),
        invoices,
    })
}

pub fn item_query_plan(
    doctype: &str,
    txt: &str,
    searchfield: &str,
    start: i32,
    page_len: i32,
    item_groups: Option<Vec<String>>,
    as_dict: bool,
) -> ItemQueryPlan {
    ItemQueryPlan {
        doctype: doctype.to_string(),
        txt: txt.to_string(),
        searchfield: searchfield.to_string(),
        start,
        page_len,
        item_groups,
        as_dict,
    }
}

pub fn create_payments_on_invoice(
    doc: &PosInvoice,
    idx: i32,
    payment_details: &PosInvoicePayment,
) -> SalesInvoicePaymentPlan {
    SalesInvoicePaymentPlan {
        idx,
        mode_of_payment: payment_details.mode_of_payment.clone(),
        amount: payment_details.amount,
        base_amount: payment_details.amount * doc.conversion_rate,
        parent: doc.name.clone(),
        parentfield: "payments".to_string(),
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
