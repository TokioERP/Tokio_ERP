use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentOrderReference {
    pub reference_doctype: Option<String>,
    pub reference_name: Option<String>,
    pub amount: f64,
    pub bank_account: Option<String>,
}

impl PaymentOrderReference {
    pub const DOCTYPE: &'static str = "Payment Order Reference";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 12] = [
        "reference_doctype",
        "reference_name",
        "amount",
        "column_break_4",
        "supplier",
        "payment_request",
        "mode_of_payment",
        "bank_account_details",
        "bank_account",
        "column_break_10",
        "account",
        "payment_reference",
    ];

    pub fn new(
        reference_doctype: impl Into<String>,
        reference_name: impl Into<String>,
        amount: f64,
        bank_account: impl Into<String>,
    ) -> Self {
        Self {
            reference_doctype: Some(reference_doctype.into()),
            reference_name: Some(reference_name.into()),
            amount,
            bank_account: Some(bank_account.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("reference_doctype", "Type")
                .options("DocType")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::dynamic_link("reference_name")
                .label("Name")
                .options("reference_doctype")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .read_only()
                .in_standard_filter(),
            FieldSpec::link("payment_request", "Payment Request")
                .options("Payment Request")
                .read_only(),
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .fetch_from("payment_request.mode_of_payment")
                .read_only(),
            FieldSpec::section_break("bank_account_details").label("Bank Account Details"),
            FieldSpec::link("bank_account", "Bank Account")
                .options("Bank Account")
                .required()
                .read_only(),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .read_only(),
            FieldSpec::data("payment_reference", "Payment Reference")
                .read_only()
                .no_copy()
                .print_hide(),
        ]
    }
}

impl DocumentController for PaymentOrderReference {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
