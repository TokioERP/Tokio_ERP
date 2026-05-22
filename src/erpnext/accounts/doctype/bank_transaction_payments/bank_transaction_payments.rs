use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankTransactionPayments {
    pub payment_document: Option<String>,
    pub payment_entry: Option<String>,
    pub allocated_amount: Option<String>,
    pub clearance_date: Option<String>,
}

impl BankTransactionPayments {
    pub const DOCTYPE: &'static str = "Bank Transaction Payments";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] = [
        "payment_document",
        "payment_entry",
        "allocated_amount",
        "clearance_date",
    ];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        payment_document: impl Into<String>,
        payment_entry: impl Into<String>,
        allocated_amount: impl Into<String>,
    ) -> Self {
        Self {
            payment_document: Some(payment_document.into()),
            payment_entry: Some(payment_entry.into()),
            allocated_amount: Some(allocated_amount.into()),
            clearance_date: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_document", "Payment Document")
                .options("DocType")
                .required()
                .in_list_view(),
            FieldSpec::dynamic_link("payment_entry")
                .label("Payment Entry")
                .options("payment_document")
                .required()
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .required()
                .in_list_view(),
            FieldSpec::date("clearance_date", "Clearance Date")
                .depends_on("eval:doc.docstatus==1")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }
}

impl DocumentController for BankTransactionPayments {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
