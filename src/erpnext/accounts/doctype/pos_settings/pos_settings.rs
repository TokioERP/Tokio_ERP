use std::collections::HashSet;

use crate::erpnext::accounts::doctype::pos_field::pos_field::PosField;
use crate::erpnext::accounts::doctype::pos_search_fields::pos_search_fields::PosSearchFields;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosSettings {
    pub invoice_type: String,
    pub post_change_gl_entries: bool,
    pub invoice_fields: Vec<PosField>,
    pub pos_search_fields: Vec<PosSearchFields>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosSettingsError {
    DuplicatePosField { fieldname: String },
    CannotChangeInvoiceTypeWithOpenEntries,
}

impl Default for PosSettings {
    fn default() -> Self {
        Self {
            invoice_type: "Sales Invoice".to_string(),
            post_change_gl_entries: false,
            invoice_fields: Vec::new(),
            pos_search_fields: Vec::new(),
        }
    }
}

impl PosSettings {
    pub const DOCTYPE: &'static str = "POS Settings";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "invoice_type",
        "column_break_vwwt",
        "post_change_gl_entries",
        "section_break_gyos",
        "invoice_fields",
        "pos_search_fields",
    ];
    pub const IS_SINGLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("invoice_type", "Invoice Type Created via POS Screen")
                .options("Sales Invoice\nPOS Invoice")
                .default("Sales Invoice")
                .description("The system will create a Sales Invoice or a POS Invoice from the POS interface based on this setting. For high-volume transactions, it is recommended to use POS Invoice."),
            FieldSpec::column_break("column_break_vwwt"),
            FieldSpec::check("post_change_gl_entries", "Create Ledger Entries for Change Amount")
                .options("1")
                .default("0")
                .description("If enabled, ledger entries will be posted for change amount in POS transactions"),
            FieldSpec::section_break("section_break_gyos"),
            FieldSpec::table("invoice_fields", "POS Additional Fields").options("POS Field"),
            FieldSpec::table("pos_search_fields", "POS Search Fields").options("POS Search Fields"),
        ]
    }

    pub fn validate(
        &self,
        old_invoice_type: Option<&str>,
        open_pos_opening_entries_count: usize,
    ) -> Result<(), PosSettingsError> {
        if old_invoice_type.is_some_and(|old| old != self.invoice_type) {
            self.validate_invoice_type(open_pos_opening_entries_count)?;
        }
        self.validate_invoice_fields()
    }

    pub fn validate_invoice_fields(&self) -> Result<(), PosSettingsError> {
        let mut seen = HashSet::new();
        for field in &self.invoice_fields {
            if let Some(fieldname) = field.fieldname.as_deref() {
                if !seen.insert(fieldname) {
                    return Err(PosSettingsError::DuplicatePosField {
                        fieldname: fieldname.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn validate_invoice_type(
        &self,
        open_pos_opening_entries_count: usize,
    ) -> Result<(), PosSettingsError> {
        if open_pos_opening_entries_count > 0 {
            return Err(PosSettingsError::CannotChangeInvoiceTypeWithOpenEntries);
        }
        Ok(())
    }
}

impl DocumentController for PosSettings {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}
