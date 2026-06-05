use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoice {
    pub name: Option<String>,
    pub company: Option<String>,
    pub customer: Option<String>,
    pub is_pos: bool,
    pub is_return: bool,
    pub docstatus: i32,
    pub status: String,
    pub due_date: Option<String>,
    pub posting_date: Option<String>,
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
    pub return_against: Option<String>,
    pub coupon_code: Option<String>,
    pub items: Vec<PosInvoiceItem>,
    pub payments: Vec<PosInvoicePayment>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoiceItem {
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
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoicePayment {
    pub idx: i32,
    pub mode_of_payment: Option<String>,
    pub payment_type: String,
    pub account: Option<String>,
    pub amount: f64,
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
            {
                "Overdue".to_string()
            } else if self.outstanding_amount > 0.0 && self.outstanding_amount < total {
                "Partly Paid".to_string()
            } else if self.outstanding_amount > 0.0 {
                "Unpaid".to_string()
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
            "set_status:Cancelled".to_string(),
        ];
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

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
