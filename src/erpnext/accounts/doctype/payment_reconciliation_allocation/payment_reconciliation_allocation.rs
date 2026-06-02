use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentReconciliationAllocation;

impl PaymentReconciliationAllocation {
    pub const DOCTYPE: &'static str = "Payment Reconciliation Allocation";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const IS_VIRTUAL: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 23] = [
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
        "debit_or_credit_note_posting_date",
        "column_break_7",
        "difference_account",
        "exchange_rate",
        "currency",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
    ];

    pub fn get_list<T>(_args: T) {}

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::dynamic_link("invoice_number")
                .label("Invoice Number")
                .options("invoice_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("currency")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::link("difference_account", "Difference Account")
                .options("Account")
                .read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::currency("difference_amount", "Difference Amount")
                .options("Currency")
                .read_only()
                .in_list_view(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("is_advance", "Is Advance")
                .read_only()
                .hidden(),
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::link("invoice_type", "Invoice Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::currency("unreconciled_amount", "Unreconciled Amount")
                .options("currency")
                .read_only()
                .hidden(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .read_only()
                .hidden(),
            FieldSpec::data("reference_row", "Reference Row")
                .read_only()
                .hidden(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::float("exchange_rate", "Exchange Rate").read_only(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::date("gain_loss_posting_date", "Difference Posting Date"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::date(
                "debit_or_credit_note_posting_date",
                "Debit / Credit Note Posting Date",
            ),
        ]
    }
}

impl DocumentController for PaymentReconciliationAllocation {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
