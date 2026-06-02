use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliationInvoice;

impl PaymentReconciliationInvoice {
    pub const DOCTYPE: &'static str = "Payment Reconciliation Invoice";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const IS_VIRTUAL: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 8] = [
        "invoice_type",
        "invoice_number",
        "invoice_date",
        "col_break1",
        "amount",
        "outstanding_amount",
        "currency",
        "exchange_rate",
    ];

    pub fn get_list<T>(_args: T) {}

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("invoice_type", "Invoice Type")
                .options("Sales Invoice\nPurchase Invoice\nJournal Entry")
                .read_only()
                .in_list_view(),
            FieldSpec::dynamic_link("invoice_number")
                .label("Invoice Number")
                .options("invoice_type")
                .read_only()
                .in_list_view(),
            FieldSpec::date("invoice_date", "Invoice Date")
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("col_break1"),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .read_only(),
            FieldSpec::currency("outstanding_amount", "Outstanding Amount")
                .options("currency")
                .read_only()
                .in_list_view(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::float("exchange_rate", "Exchange Rate").hidden(),
        ]
    }
}

impl DocumentController for PaymentReconciliationInvoice {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
