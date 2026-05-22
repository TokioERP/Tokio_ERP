use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessPaymentReconciliationLogAllocations {
    pub allocated_amount: Option<String>,
    pub amount: Option<String>,
    pub currency: Option<String>,
    pub difference_account: Option<String>,
    pub difference_amount: Option<String>,
    pub exchange_rate: Option<String>,
    pub gain_loss_posting_date: Option<String>,
    pub invoice_number: Option<String>,
    pub invoice_type: Option<String>,
    pub is_advance: Option<String>,
    pub reconciled: bool,
    pub reference_name: Option<String>,
    pub reference_row: Option<String>,
    pub reference_type: Option<String>,
    pub unreconciled_amount: Option<String>,
}

impl ProcessPaymentReconciliationLogAllocations {
    pub const DOCTYPE: &'static str = "Process Payment Reconciliation Log Allocations";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 20] = [
        "reference_type",
        "reference_name",
        "reference_row",
        "column_break_3",
        "invoice_type",
        "invoice_number",
        "section_break_6",
        "allocated_amount",
        "unreconciled_amount",
        "column_break_8",
        "amount",
        "is_advance",
        "section_break_5",
        "difference_amount",
        "gain_loss_posting_date",
        "column_break_7",
        "difference_account",
        "exchange_rate",
        "currency",
        "reconciled",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        reference_type: impl Into<String>,
        reference_name: impl Into<String>,
        invoice_type: impl Into<String>,
        invoice_number: impl Into<String>,
        allocated_amount: impl Into<String>,
    ) -> Self {
        Self {
            reference_type: Some(reference_type.into()),
            reference_name: Some(reference_name.into()),
            invoice_type: Some(invoice_type.into()),
            invoice_number: Some(invoice_number.into()),
            allocated_amount: Some(allocated_amount.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("reference_row", "Reference Row")
                .hidden()
                .read_only(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("invoice_type", "Invoice Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::dynamic_link("invoice_number")
                .label("Invoice Number")
                .options("invoice_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("currency")
                .required()
                .in_list_view(),
            FieldSpec::currency("unreconciled_amount", "Unreconciled Amount")
                .options("currency")
                .hidden()
                .read_only(),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .hidden()
                .read_only(),
            FieldSpec::data("is_advance", "Is Advance")
                .hidden()
                .read_only(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::currency("difference_amount", "Difference Amount")
                .options("Currency")
                .read_only()
                .in_list_view(),
            FieldSpec::date("gain_loss_posting_date", "Difference Posting Date"),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("difference_account", "Difference Account")
                .options("Account")
                .read_only(),
            FieldSpec::float("exchange_rate", "Exchange Rate").read_only(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::check("reconciled", "Reconciled")
                .default("0")
                .in_list_view(),
        ]
    }
}

impl DocumentController for ProcessPaymentReconciliationLogAllocations {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
