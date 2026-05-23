use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesInvoicePayment {
    pub default: bool,
    pub mode_of_payment: Option<String>,
    pub amount: Option<String>,
    pub reference_no: Option<String>,
    pub account: Option<String>,
    pub type_: Option<String>,
    pub base_amount: Option<String>,
    pub clearance_date: Option<String>,
}

impl SalesInvoicePayment {
    pub const DOCTYPE: &'static str = "Sales Invoice Payment";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 9] = [
        "default",
        "mode_of_payment",
        "amount",
        "reference_no",
        "column_break_3",
        "account",
        "type",
        "base_amount",
        "clearance_date",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;

    pub fn new(mode_of_payment: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            mode_of_payment: Some(mode_of_payment.into()),
            amount: Some(amount.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::check("default", "Default")
                .default("0")
                .hidden()
                .read_only(),
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .default("0")
                .required()
                .in_list_view()
                .depends_on("eval: [\"POS Invoice\", \"Sales Invoice\"].includes(parent.doctype)"),
            FieldSpec::data("reference_no", "Reference No"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .read_only()
                .print_hide(),
            FieldSpec::read_only_field("type", "Type").fetch_from("mode_of_payment.type"),
            FieldSpec::currency("base_amount", "Base Amount (Company Currency)")
                .options("Company:company:default_currency")
                .read_only()
                .print_hide()
                .no_copy(),
            FieldSpec::date("clearance_date", "Clearance Date")
                .read_only()
                .print_hide()
                .no_copy(),
        ]
    }
}

impl DocumentController for SalesInvoicePayment {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
