use std::collections::BTreeSet;

use crate::erpnext::DocumentController;

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntryReferenceRow {
    pub idx: usize,
    pub reference_doctype: String,
    pub reference_name: String,
    pub due_date: Option<String>,
    pub bill_no: Option<String>,
    pub payment_term: Option<String>,
    pub payment_term_outstanding: f64,
    pub account_type: Option<String>,
    pub payment_type: Option<String>,
    pub reconcile_effect_on: Option<String>,
    pub total_amount: f64,
    pub outstanding_amount: f64,
    pub allocated_amount: f64,
    pub exchange_rate: Option<f64>,
    pub exchange_gain_loss: f64,
    pub account: Option<String>,
    pub payment_request: Option<String>,
    pub advance_voucher_type: Option<String>,
    pub advance_voucher_no: Option<String>,
    pub on_hold: bool,
}

impl Default for PaymentEntryReferenceRow {
    fn default() -> Self {
        Self {
            idx: 0,
            reference_doctype: String::new(),
            reference_name: String::new(),
            due_date: None,
            bill_no: None,
            payment_term: None,
            payment_term_outstanding: 0.0,
            account_type: None,
            payment_type: None,
            reconcile_effect_on: None,
            total_amount: 0.0,
            outstanding_amount: 0.0,
            allocated_amount: 0.0,
            exchange_rate: None,
            exchange_gain_loss: 0.0,
            account: None,
            payment_request: None,
            advance_voucher_type: None,
            advance_voucher_no: None,
            on_hold: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntryDeductionRow {
    pub account: String,
    pub cost_center: Option<String>,
    pub amount: f64,
    pub is_exchange_gain_loss: bool,
    pub description: Option<String>,
}

impl Default for PaymentEntryDeductionRow {
    fn default() -> Self {
        Self {
            account: String::new(),
            cost_center: None,
            amount: 0.0,
            is_exchange_gain_loss: false,
            description: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupplierBlockStatus {
    pub on_hold: bool,
    pub hold_type: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntry {
    pub name: Option<String>,
    pub naming_series: String,
    pub payment_type: String,
    pub posting_date: String,
    pub company: String,
    pub company_currency: String,
    pub cost_center: Option<String>,
    pub mode_of_payment: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub party_name: Option<String>,
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
    pub paid_from: Option<String>,
    pub paid_from_account_currency: String,
    pub paid_from_account_type: Option<String>,
    pub paid_to: Option<String>,
    pub paid_to_account_currency: String,
    pub paid_to_account_type: Option<String>,
    pub paid_amount: f64,
    pub source_exchange_rate: f64,
    pub base_paid_amount: f64,
    pub received_amount: f64,
    pub target_exchange_rate: f64,
    pub base_received_amount: f64,
    pub references: Vec<PaymentEntryReferenceRow>,
    pub total_allocated_amount: f64,
    pub base_total_allocated_amount: f64,
    pub unallocated_amount: f64,
    pub difference_amount: f64,
    pub write_off_difference_amount: bool,
    pub deductions: Vec<PaymentEntryDeductionRow>,
    pub reference_no: Option<String>,
    pub reference_date: Option<String>,
    pub clearance_date: Option<String>,
    pub project: Option<String>,
    pub remarks: Option<String>,
    pub bank: Option<String>,
    pub bank_account_no: Option<String>,
    pub payment_order: Option<String>,
    pub auto_repeat: Option<String>,
    pub amended_from: Option<String>,
    pub title: Option<String>,
    pub bank_account: Option<String>,
    pub party_bank_account: Option<String>,
    pub payment_order_status: Option<String>,
    pub status: String,
    pub custom_remarks: bool,
    pub tax_withholding_category: Option<String>,
    pub base_total_taxes_and_charges: f64,
    pub total_taxes_and_charges: f64,
    pub paid_amount_after_tax: f64,
    pub base_paid_amount_after_tax: f64,
    pub received_amount_after_tax: f64,
    pub base_received_amount_after_tax: f64,
    pub book_advance_payments_in_separate_party_account: bool,
    pub base_in_words: Option<String>,
    pub in_words: Option<String>,
    pub is_opening: String,
    pub apply_tds: bool,
    pub supplier_block_status: Option<SupplierBlockStatus>,
    pub party_account_field: Option<String>,
    pub party_account: Option<String>,
    pub party_account_currency: Option<String>,
    pub transaction_currency: String,
    pub transaction_exchange_rate: f64,
}

impl Default for PaymentEntry {
    fn default() -> Self {
        Self {
            name: None,
            naming_series: "ACC-PAY-.YYYY.-".to_string(),
            payment_type: "Receive".to_string(),
            posting_date: String::new(),
            company: String::new(),
            company_currency: String::new(),
            cost_center: None,
            mode_of_payment: None,
            party_type: None,
            party: None,
            party_name: None,
            contact_person: None,
            contact_email: None,
            paid_from: None,
            paid_from_account_currency: String::new(),
            paid_from_account_type: None,
            paid_to: None,
            paid_to_account_currency: String::new(),
            paid_to_account_type: None,
            paid_amount: 0.0,
            source_exchange_rate: 0.0,
            base_paid_amount: 0.0,
            received_amount: 0.0,
            target_exchange_rate: 0.0,
            base_received_amount: 0.0,
            references: Vec::new(),
            total_allocated_amount: 0.0,
            base_total_allocated_amount: 0.0,
            unallocated_amount: 0.0,
            difference_amount: 0.0,
            write_off_difference_amount: false,
            deductions: Vec::new(),
            reference_no: None,
            reference_date: None,
            clearance_date: None,
            project: None,
            remarks: None,
            bank: None,
            bank_account_no: None,
            payment_order: None,
            auto_repeat: None,
            amended_from: None,
            title: None,
            bank_account: None,
            party_bank_account: None,
            payment_order_status: None,
            status: String::new(),
            custom_remarks: false,
            tax_withholding_category: None,
            base_total_taxes_and_charges: 0.0,
            total_taxes_and_charges: 0.0,
            paid_amount_after_tax: 0.0,
            base_paid_amount_after_tax: 0.0,
            received_amount_after_tax: 0.0,
            base_received_amount_after_tax: 0.0,
            book_advance_payments_in_separate_party_account: false,
            base_in_words: None,
            in_words: None,
            is_opening: "No".to_string(),
            apply_tds: false,
            supplier_block_status: None,
            party_account_field: None,
            party_account: None,
            party_account_currency: None,
            transaction_currency: String::new(),
            transaction_exchange_rate: 1.0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentEntryError {
    InvalidPaymentType,
    PartyTypeMandatory,
    PartyMandatory,
    MandatoryField(&'static str),
    ReceivedAmountGreaterThanPaidAmount,
    DuplicateReference {
        row: usize,
        reference_doctype: String,
        reference_name: String,
    },
    CannotReceiveFromCustomerAgainstNegativeOutstanding,
    AllocatedGreaterThanOutstanding {
        row: usize,
    },
    BankTransactionReferenceMandatory,
    SupplierBlocked {
        supplier: String,
    },
    ReferenceDocumentOnHold {
        reference_doctype: String,
        reference_name: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlEntryPlan {
    pub account: String,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub against: Option<String>,
    pub account_currency: String,
    pub cost_center: Option<String>,
    pub debit_in_account_currency: f64,
    pub debit: f64,
    pub debit_in_transaction_currency: f64,
    pub credit_in_account_currency: f64,
    pub credit: f64,
    pub credit_in_transaction_currency: f64,
    pub against_voucher_type: Option<String>,
    pub against_voucher: Option<String>,
    pub advance_voucher_type: Option<String>,
    pub advance_voucher_no: Option<String>,
    pub transaction_exchange_rate: f64,
    pub post_net_value: bool,
}

impl PaymentEntry {
    pub const DOCTYPE: &'static str = "Payment Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const IS_SUBMITTABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const TITLE_FIELD: &'static str = "title";
    pub const FIELD_ORDER: [&'static str; 93] = [
        "type_of_payment",
        "naming_series",
        "payment_type",
        "column_break_5",
        "posting_date",
        "company",
        "cost_center",
        "mode_of_payment",
        "party_section",
        "party_type",
        "party",
        "party_name",
        "column_break_11",
        "contact_person",
        "contact_email",
        "payment_accounts_section",
        "paid_from",
        "paid_from_account_currency",
        "column_break_18",
        "paid_to",
        "paid_to_account_currency",
        "payment_amounts_section",
        "paid_amount",
        "source_exchange_rate",
        "base_paid_amount",
        "column_break_21",
        "received_amount",
        "target_exchange_rate",
        "base_received_amount",
        "section_break_14",
        "references",
        "section_break_34",
        "total_allocated_amount",
        "base_total_allocated_amount",
        "column_break_36",
        "unallocated_amount",
        "difference_amount",
        "write_off_difference_amount",
        "deductions_or_loss_section",
        "deductions",
        "transaction_references",
        "reference_no",
        "column_break_23",
        "reference_date",
        "clearance_date",
        "section_break_12",
        "project",
        "remarks",
        "column_break_16",
        "letter_head",
        "print_heading",
        "bank",
        "bank_account_no",
        "payment_order",
        "auto_repeat",
        "amended_from",
        "title",
        "bank_account",
        "party_bank_account",
        "payment_order_status",
        "accounting_dimensions_section",
        "dimension_col_break",
        "status",
        "custom_remarks",
        "tax_withholding_category",
        "taxes_and_charges_section",
        "purchase_taxes_and_charges_template",
        "sales_taxes_and_charges_template",
        "taxes",
        "base_total_taxes_and_charges",
        "total_taxes_and_charges",
        "paid_amount_after_tax",
        "base_paid_amount_after_tax",
        "received_amount_after_tax",
        "base_received_amount_after_tax",
        "paid_from_account_type",
        "paid_to_account_type",
        "column_break_61",
        "section_break_60",
        "get_outstanding_invoices",
        "get_outstanding_orders",
        "book_advance_payments_in_separate_party_account",
        "base_in_words",
        "in_words",
        "reconcile_on_advance_payment_date",
        "is_opening",
        "apply_tds",
        "section_tax_withholding_entry",
        "tax_withholding_group",
        "ignore_tax_withholding_threshold",
        "tax_withholding_entries",
        "override_tax_withholding_entries",
        "auto_repeat_section",
    ];

    pub fn validate(&mut self) -> Result<(), PaymentEntryError> {
        self.setup_party_account_field();
        self.set_missing_values()?;
        self.validate_payment_type()?;
        self.set_exchange_rate();
        self.validate_mandatory()?;
        self.validate_reference_documents()?;
        self.validate_duplicate_entry()?;
        self.set_amounts()?;
        self.validate_amounts()?;
        self.clear_unallocated_reference_document_rows();
        self.validate_transaction_reference()?;
        self.set_title();
        self.set_remarks();
        self.validate_payment_type_with_outstanding()?;
        self.validate_allocated_amount()?;
        self.ensure_supplier_is_not_blocked()?;
        self.set_status(0);
        self.set_total_in_words();
        Ok(())
    }

    pub fn setup_party_account_field(&mut self) {
        self.party_account_field = None;
        self.party_account = None;
        self.party_account_currency = None;
        if self.payment_type == "Receive" {
            self.party_account_field = Some("paid_from".to_string());
            self.party_account = self.paid_from.clone();
            self.party_account_currency = Some(self.paid_from_account_currency.clone());
        } else if self.payment_type == "Pay" {
            self.party_account_field = Some("paid_to".to_string());
            self.party_account = self.paid_to.clone();
            self.party_account_currency = Some(self.paid_to_account_currency.clone());
        }
    }

    pub fn set_missing_values(&mut self) -> Result<(), PaymentEntryError> {
        if self.payment_type == "Internal Transfer" {
            self.party = None;
            self.total_allocated_amount = 0.0;
            self.base_total_allocated_amount = 0.0;
            self.unallocated_amount = 0.0;
            self.references.clear();
            return Ok(());
        }
        if self.party_type.is_none() {
            return Err(PaymentEntryError::PartyTypeMandatory);
        }
        if self.party.is_none() {
            return Err(PaymentEntryError::PartyMandatory);
        }
        self.party_account_currency = if self.payment_type == "Receive" {
            Some(self.paid_from_account_currency.clone())
        } else {
            Some(self.paid_to_account_currency.clone())
        };
        Ok(())
    }

    pub fn validate_payment_type(&self) -> Result<(), PaymentEntryError> {
        if matches!(
            self.payment_type.as_str(),
            "Receive" | "Pay" | "Internal Transfer"
        ) {
            Ok(())
        } else {
            Err(PaymentEntryError::InvalidPaymentType)
        }
    }

    pub fn validate_mandatory(&self) -> Result<(), PaymentEntryError> {
        for (field, value) in [
            ("paid_amount", self.paid_amount),
            ("received_amount", self.received_amount),
            ("source_exchange_rate", self.source_exchange_rate),
            ("target_exchange_rate", self.target_exchange_rate),
        ] {
            if value == 0.0 {
                return Err(PaymentEntryError::MandatoryField(field));
            }
        }
        Ok(())
    }

    pub fn validate_reference_documents(&self) -> Result<(), PaymentEntryError> {
        for row in &self.references {
            if row.reference_doctype == "Purchase Invoice" && row.on_hold {
                return Err(PaymentEntryError::ReferenceDocumentOnHold {
                    reference_doctype: row.reference_doctype.clone(),
                    reference_name: row.reference_name.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn set_exchange_rate(&mut self) {
        self.set_source_exchange_rate();
        self.set_target_exchange_rate();
    }

    pub fn set_source_exchange_rate(&mut self) {
        if self.paid_from.is_none() {
            return;
        }
        if self.paid_from_account_currency == self.company_currency {
            self.source_exchange_rate = 1.0;
        } else if self.source_exchange_rate == 0.0 {
            self.source_exchange_rate = 1.0;
        }
    }

    pub fn set_target_exchange_rate(&mut self) {
        if self.paid_from_account_currency == self.paid_to_account_currency {
            self.target_exchange_rate = self.source_exchange_rate;
        } else if self.paid_to.is_some() && self.target_exchange_rate == 0.0 {
            self.target_exchange_rate = 1.0;
        }
    }

    pub fn validate_duplicate_entry(&self) -> Result<(), PaymentEntryError> {
        let mut seen = BTreeSet::new();
        for (index, row) in self.references.iter().enumerate() {
            let key = (
                row.reference_doctype.clone(),
                row.reference_name.clone(),
                row.payment_term.clone(),
                row.payment_request.clone(),
            );
            if !seen.insert(key) {
                return Err(PaymentEntryError::DuplicateReference {
                    row: index + 1,
                    reference_doctype: row.reference_doctype.clone(),
                    reference_name: row.reference_name.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn set_amounts(&mut self) -> Result<(), PaymentEntryError> {
        self.set_received_amount();
        self.set_amounts_in_company_currency();
        self.set_total_allocated_amount();
        self.set_unallocated_amount();
        self.set_exchange_gain_loss();
        self.set_difference_amount();
        Ok(())
    }

    pub fn validate_amounts(&self) -> Result<(), PaymentEntryError> {
        if self.paid_from_account_currency == self.paid_to_account_currency
            && self.paid_amount < self.received_amount
        {
            Err(PaymentEntryError::ReceivedAmountGreaterThanPaidAmount)
        } else {
            Ok(())
        }
    }

    pub fn set_received_amount(&mut self) {
        self.base_received_amount = self.base_paid_amount;
        if self.paid_from_account_currency == self.paid_to_account_currency
            && self.payment_type != "Internal Transfer"
        {
            self.received_amount = self.paid_amount;
        }
    }

    pub fn set_amounts_in_company_currency(&mut self) {
        self.base_paid_amount = 0.0;
        self.base_received_amount = 0.0;
        self.difference_amount = 0.0;
        if self.paid_amount != 0.0 {
            self.base_paid_amount = flt(self.paid_amount * self.source_exchange_rate);
        }
        if self.received_amount != 0.0 {
            self.base_received_amount = flt(self.received_amount * self.target_exchange_rate);
        }
    }

    pub fn calculate_base_allocated_amount_for_reference(&mut self, index: usize) -> f64 {
        let exchange_rate = if self.payment_type == "Receive" {
            self.source_exchange_rate
        } else if self.payment_type == "Pay" {
            self.target_exchange_rate
        } else {
            1.0
        };
        let base_allocated = flt(self.references[index].allocated_amount * exchange_rate);
        if is_advance_doctype(&self.references[index].reference_doctype) {
            return base_allocated;
        }
        let reference_exchange_rate = self.references[index].exchange_rate.unwrap_or(1.0);
        let reference_base = flt(self.references[index].allocated_amount * reference_exchange_rate);
        self.references[index].exchange_gain_loss = flt(base_allocated - reference_base);
        base_allocated
    }

    pub fn set_total_allocated_amount(&mut self) {
        if self.payment_type == "Internal Transfer" {
            return;
        }
        let mut total_allocated = 0.0;
        let mut base_total_allocated = 0.0;
        for index in 0..self.references.len() {
            if self.references[index].allocated_amount != 0.0 {
                total_allocated += self.references[index].allocated_amount;
                base_total_allocated += self.calculate_base_allocated_amount_for_reference(index);
            }
        }
        self.total_allocated_amount = flt(total_allocated.abs());
        self.base_total_allocated_amount = flt(base_total_allocated.abs());
    }

    pub fn set_unallocated_amount(&mut self) {
        self.unallocated_amount = 0.0;
        if self.party.is_none() {
            return;
        }
        let deductions_to_consider: f64 = self
            .deductions
            .iter()
            .filter(|row| !row.is_exchange_gain_loss)
            .map(|row| row.amount)
            .sum();
        let included_taxes = 0.0;
        if self.payment_type == "Receive"
            && self.base_total_allocated_amount < self.base_paid_amount + deductions_to_consider
        {
            self.unallocated_amount = flt(self.base_paid_amount + deductions_to_consider
                - self.base_total_allocated_amount
                - included_taxes)
                / self.source_exchange_rate;
        } else if self.payment_type == "Pay"
            && self.base_total_allocated_amount < self.base_received_amount - deductions_to_consider
        {
            self.unallocated_amount = flt(self.base_received_amount
                - deductions_to_consider
                - self.base_total_allocated_amount
                - included_taxes)
                / self.target_exchange_rate;
        }
    }

    pub fn set_exchange_gain_loss(&mut self) {
        let exchange_gain_loss = flt(self.base_paid_amount - self.base_received_amount);
        self.deductions
            .retain(|row| !row.is_exchange_gain_loss || exchange_gain_loss != 0.0);
        if exchange_gain_loss == 0.0 {
            return;
        }
        if let Some(row) = self
            .deductions
            .iter_mut()
            .find(|row| row.is_exchange_gain_loss)
        {
            row.amount = exchange_gain_loss;
        } else {
            self.deductions.push(PaymentEntryDeductionRow {
                amount: exchange_gain_loss,
                is_exchange_gain_loss: true,
                ..Default::default()
            });
        }
    }

    pub fn set_difference_amount(&mut self) {
        let base_unallocated_amount = self.unallocated_amount
            * if self.payment_type == "Receive" {
                self.source_exchange_rate
            } else {
                self.target_exchange_rate
            };
        let base_party_amount = self.base_total_allocated_amount + flt(base_unallocated_amount);
        let included_taxes = 0.0;
        if self.payment_type == "Receive" {
            self.difference_amount = base_party_amount - self.base_received_amount + included_taxes;
        } else if self.payment_type == "Pay" {
            self.difference_amount = self.base_paid_amount - base_party_amount - included_taxes;
        } else {
            self.difference_amount =
                self.base_paid_amount - self.base_received_amount - included_taxes;
        }
        let total_deductions: f64 = self.deductions.iter().map(|row| row.amount).sum();
        self.difference_amount = flt(self.difference_amount - total_deductions);
    }

    pub fn clear_unallocated_reference_document_rows(&mut self) {
        self.references.retain(|row| row.allocated_amount != 0.0);
    }

    pub fn validate_transaction_reference(&self) -> Result<(), PaymentEntryError> {
        let bank_account_type = if self.payment_type == "Receive" {
            self.paid_to_account_type.as_deref()
        } else {
            self.paid_from_account_type.as_deref()
        };
        if bank_account_type == Some("Bank")
            && (self.reference_no.is_none() || self.reference_date.is_none())
        {
            Err(PaymentEntryError::BankTransactionReferenceMandatory)
        } else {
            Ok(())
        }
    }

    pub fn set_title(&mut self) {
        if matches!(self.payment_type.as_str(), "Receive" | "Pay") {
            self.title = self.party.clone();
        } else {
            self.title = Some(format!(
                "{} - {}",
                self.paid_from.clone().unwrap_or_default(),
                self.paid_to.clone().unwrap_or_default()
            ));
        }
    }

    pub fn set_remarks(&mut self) {
        if self.custom_remarks {
            return;
        }
        let mut remarks = if self.payment_type == "Internal Transfer" {
            vec![format!(
                "Amount {} {} transferred from {} to {}",
                self.paid_from_account_currency,
                format_amount(self.paid_amount),
                self.paid_from.clone().unwrap_or_default(),
                self.paid_to.clone().unwrap_or_default()
            )]
        } else {
            vec![format!(
                "Amount {} {} {} {}",
                if self.payment_type == "Receive" {
                    &self.paid_to_account_currency
                } else {
                    &self.paid_from_account_currency
                },
                format_amount(if self.payment_type == "Receive" {
                    self.paid_amount
                } else {
                    self.received_amount
                }),
                if self.payment_type == "Receive" {
                    "received from"
                } else {
                    "paid to"
                },
                self.party.clone().unwrap_or_default()
            )]
        };
        if let Some(reference_no) = self.reference_no.as_deref() {
            remarks.push(format!(
                "Transaction reference no {} dated {}",
                reference_no,
                self.reference_date.clone().unwrap_or_default()
            ));
        }
        if matches!(self.payment_type.as_str(), "Receive" | "Pay") {
            for row in &self.references {
                if row.allocated_amount != 0.0 {
                    remarks.push(format!(
                        "Amount {} {} against {} {}",
                        self.party_account_currency.clone().unwrap_or_default(),
                        format_amount(row.allocated_amount),
                        row.reference_doctype,
                        row.reference_name
                    ));
                }
            }
        }
        for row in &self.deductions {
            if row.amount != 0.0 {
                remarks.push(format!(
                    "Amount {} {} deducted against {}",
                    self.company_currency,
                    format_amount(row.amount),
                    row.account
                ));
            }
        }
        self.remarks = Some(remarks.join("\n"));
    }

    pub fn validate_payment_type_with_outstanding(&self) -> Result<(), PaymentEntryError> {
        let total_outstanding: f64 = self.references.iter().map(|row| row.allocated_amount).sum();
        if total_outstanding < 0.0
            && self.party_type.as_deref() == Some("Customer")
            && self.payment_type == "Receive"
        {
            Err(PaymentEntryError::CannotReceiveFromCustomerAgainstNegativeOutstanding)
        } else {
            Ok(())
        }
    }

    pub fn validate_allocated_amount(&self) -> Result<(), PaymentEntryError> {
        if self.payment_type == "Internal Transfer"
            || matches!(self.party_type.as_deref(), Some("Customer" | "Supplier"))
        {
            return Ok(());
        }
        for row in &self.references {
            if (row.allocated_amount > 0.0 && row.allocated_amount > row.outstanding_amount)
                || (row.allocated_amount < 0.0 && row.allocated_amount < row.outstanding_amount)
            {
                return Err(PaymentEntryError::AllocatedGreaterThanOutstanding { row: row.idx });
            }
        }
        Ok(())
    }

    pub fn ensure_supplier_is_not_blocked(&self) -> Result<(), PaymentEntryError> {
        if self.payment_type != "Pay" || self.party_type.as_deref() != Some("Supplier") {
            return Ok(());
        }
        let Some(status) = self.supplier_block_status.as_ref() else {
            return Ok(());
        };
        if !status.on_hold || !matches!(status.hold_type.as_deref(), Some("All" | "Payments")) {
            return Ok(());
        }
        if status
            .release_date
            .as_deref()
            .is_some_and(|release_date| release_date < self.posting_date.as_str())
        {
            return Ok(());
        }
        Err(PaymentEntryError::SupplierBlocked {
            supplier: self.party.clone().unwrap_or_default(),
        })
    }

    pub fn set_status(&mut self, docstatus: i32) {
        self.status = match docstatus {
            2 => "Cancelled",
            1 => "Submitted",
            _ => "Draft",
        }
        .to_string();
    }

    pub fn set_total_in_words(&mut self) {
        if matches!(self.payment_type.as_str(), "Pay" | "Internal Transfer") {
            self.base_in_words = Some(format!(
                "{} {}",
                format_amount(self.base_paid_amount.abs()),
                self.company_currency
            ));
            self.in_words = Some(format!(
                "{} {}",
                format_amount(self.paid_amount.abs()),
                self.paid_from_account_currency
            ));
        } else if self.payment_type == "Receive" {
            self.base_in_words = Some(format!(
                "{} {}",
                format_amount(self.base_received_amount.abs()),
                self.company_currency
            ));
            self.in_words = Some(format!(
                "{} {}",
                format_amount(self.received_amount.abs()),
                self.paid_to_account_currency
            ));
        }
    }

    pub fn set_transaction_currency_and_rate(&mut self) {
        self.transaction_currency = self.company_currency.clone();
        self.transaction_exchange_rate = 1.0;
        if self.paid_from_account_currency != self.company_currency {
            self.transaction_currency = self.paid_from_account_currency.clone();
            self.transaction_exchange_rate = self.source_exchange_rate;
        } else if self.paid_to_account_currency != self.company_currency {
            self.transaction_currency = self.paid_to_account_currency.clone();
            self.transaction_exchange_rate = self.target_exchange_rate;
        }
    }

    pub fn build_gl_map(&self) -> Vec<GlEntryPlan> {
        let mut doc = self.clone();
        if matches!(doc.payment_type.as_str(), "Receive" | "Pay")
            && doc.party_account_field.is_none()
        {
            doc.setup_party_account_field();
        }
        doc.set_transaction_currency_and_rate();
        let mut gl_entries = Vec::new();
        doc.add_party_gl_entries(&mut gl_entries);
        doc.add_bank_gl_entries(&mut gl_entries);
        doc.add_deductions_gl_entries(&mut gl_entries);
        gl_entries
    }

    pub fn add_party_gl_entries(&self, gl_entries: &mut Vec<GlEntryPlan>) {
        let Some(party_account) = self.party_account.as_deref() else {
            return;
        };
        let against_account = if self.payment_type == "Receive" {
            self.paid_to.clone()
        } else {
            self.paid_from.clone()
        };
        for row in &self.references {
            let dr_or_cr_credit = self.payment_type == "Receive";
            let allocated_base = if self.payment_type == "Receive" {
                flt(row.allocated_amount * self.source_exchange_rate)
            } else if self.payment_type == "Pay" {
                flt(row.allocated_amount * self.target_exchange_rate)
            } else {
                row.allocated_amount
            };
            let mut gle = GlEntryPlan {
                account: party_account.to_string(),
                party_type: self.party_type.clone(),
                party: self.party.clone(),
                against: against_account.clone(),
                account_currency: self.party_account_currency.clone().unwrap_or_default(),
                cost_center: self.cost_center.clone(),
                advance_voucher_type: row.advance_voucher_type.clone(),
                advance_voucher_no: row.advance_voucher_no.clone(),
                transaction_exchange_rate: self.target_exchange_rate,
                ..Default::default()
            };
            if dr_or_cr_credit {
                gle.credit_in_account_currency = row.allocated_amount;
                gle.credit = allocated_base;
                gle.credit_in_transaction_currency =
                    self.value_in_transaction_currency(row.allocated_amount, allocated_base);
            } else {
                gle.debit_in_account_currency = row.allocated_amount;
                gle.debit = allocated_base;
                gle.debit_in_transaction_currency =
                    self.value_in_transaction_currency(row.allocated_amount, allocated_base);
            }
            if is_advance_doctype(&row.reference_doctype)
                || self.book_advance_payments_in_separate_party_account
            {
                gle.against_voucher_type = Some(Self::DOCTYPE.to_string());
                gle.against_voucher = self.name.clone();
                if is_advance_doctype(&row.reference_doctype) {
                    gle.advance_voucher_type = Some(row.reference_doctype.clone());
                    gle.advance_voucher_no = Some(row.reference_name.clone());
                }
            } else {
                gle.against_voucher_type = Some(row.reference_doctype.clone());
                gle.against_voucher = Some(row.reference_name.clone());
            }
            gl_entries.push(gle);
        }
        if self.unallocated_amount != 0.0 {
            let dr_or_cr_credit = self.payment_type == "Receive";
            let exchange_rate = self.get_exchange_rate();
            let base_unallocated = flt(self.unallocated_amount * exchange_rate);
            let mut gle = GlEntryPlan {
                account: party_account.to_string(),
                party_type: self.party_type.clone(),
                party: self.party.clone(),
                against: against_account,
                account_currency: self.party_account_currency.clone().unwrap_or_default(),
                cost_center: self.cost_center.clone(),
                ..Default::default()
            };
            if dr_or_cr_credit {
                gle.credit_in_account_currency = self.unallocated_amount;
                gle.credit = base_unallocated;
                gle.credit_in_transaction_currency =
                    self.value_in_transaction_currency(self.unallocated_amount, base_unallocated);
            } else {
                gle.debit_in_account_currency = self.unallocated_amount;
                gle.debit = base_unallocated;
                gle.debit_in_transaction_currency =
                    self.value_in_transaction_currency(self.unallocated_amount, base_unallocated);
            }
            if self.book_advance_payments_in_separate_party_account {
                gle.against_voucher_type = Some(Self::DOCTYPE.to_string());
                gle.against_voucher = self.name.clone();
            }
            gl_entries.push(gle);
        }
    }

    pub fn add_bank_gl_entries(&self, gl_entries: &mut Vec<GlEntryPlan>) {
        if matches!(self.payment_type.as_str(), "Pay" | "Internal Transfer") {
            gl_entries.push(GlEntryPlan {
                account: self.paid_from.clone().unwrap_or_default(),
                account_currency: self.paid_from_account_currency.clone(),
                against: if self.payment_type == "Pay" {
                    self.party.clone()
                } else {
                    self.paid_to.clone()
                },
                credit_in_account_currency: self.paid_amount,
                credit_in_transaction_currency: if self.paid_from_account_currency
                    == self.transaction_currency
                {
                    self.paid_amount
                } else {
                    flt(self.base_paid_amount / self.transaction_exchange_rate)
                },
                credit: self.base_paid_amount,
                cost_center: self.cost_center.clone(),
                post_net_value: true,
                ..Default::default()
            });
        }
        if matches!(self.payment_type.as_str(), "Receive" | "Internal Transfer") {
            gl_entries.push(GlEntryPlan {
                account: self.paid_to.clone().unwrap_or_default(),
                account_currency: self.paid_to_account_currency.clone(),
                against: if self.payment_type == "Receive" {
                    self.party.clone()
                } else {
                    self.paid_from.clone()
                },
                debit_in_account_currency: self.received_amount,
                debit_in_transaction_currency: if self.paid_to_account_currency
                    == self.transaction_currency
                {
                    self.received_amount
                } else {
                    flt(self.base_received_amount / self.transaction_exchange_rate)
                },
                debit: self.base_received_amount,
                cost_center: self.cost_center.clone(),
                ..Default::default()
            });
        }
    }

    pub fn add_deductions_gl_entries(&self, gl_entries: &mut Vec<GlEntryPlan>) {
        for row in &self.deductions {
            if row.amount == 0.0 {
                continue;
            }
            gl_entries.push(GlEntryPlan {
                account: row.account.clone(),
                account_currency: self.company_currency.clone(),
                against: self.party.clone().or_else(|| self.paid_from.clone()),
                debit_in_account_currency: row.amount,
                debit_in_transaction_currency: flt(row.amount / self.transaction_exchange_rate),
                debit: row.amount,
                cost_center: row.cost_center.clone(),
                ..Default::default()
            });
        }
    }

    pub fn get_exchange_rate(&self) -> f64 {
        if self.payment_type == "Receive" {
            self.source_exchange_rate
        } else {
            self.target_exchange_rate
        }
    }

    fn value_in_transaction_currency(&self, account_amount: f64, base_amount: f64) -> f64 {
        if self.party_account_currency.as_deref() == Some(self.transaction_currency.as_str()) {
            account_amount
        } else {
            flt(base_amount / self.transaction_exchange_rate)
        }
    }
}

impl DocumentController for PaymentEntry {
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
            "on_submit",
            "validate_for_repost",
            "on_update_after_submit",
            "on_cancel",
        ]
    }
}

fn is_advance_doctype(doctype: &str) -> bool {
    matches!(
        doctype,
        "Sales Order" | "Purchase Order" | "Employee Advance"
    )
}

fn flt(value: f64) -> f64 {
    let rounded = (value * 1_000_000_000.0).round() / 1_000_000_000.0;
    if rounded == -0.0 {
        0.0
    } else {
        rounded
    }
}

fn format_amount(value: f64) -> String {
    let value = flt(value);
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}
