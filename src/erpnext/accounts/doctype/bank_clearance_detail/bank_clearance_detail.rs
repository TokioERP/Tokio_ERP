use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankClearanceDetail {
    pub payment_document: Option<String>,
    pub payment_entry: Option<String>,
    pub against_account: Option<String>,
    pub amount: Option<String>,
    pub posting_date: Option<String>,
    pub cheque_number: Option<String>,
    pub cheque_date: Option<String>,
    pub clearance_date: Option<String>,
}

impl BankClearanceDetail {
    pub const DOCTYPE: &'static str = "Bank Clearance Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 9] = [
        "payment_document",
        "payment_entry",
        "against_account",
        "amount",
        "column_break_5",
        "posting_date",
        "cheque_number",
        "cheque_date",
        "clearance_date",
    ];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const GRID_PAGE_LENGTH: u16 = 50;
    pub const ROW_FORMAT: &'static str = "Dynamic";

    pub fn new(payment_document: impl Into<String>, payment_entry: impl Into<String>) -> Self {
        Self {
            payment_document: Some(payment_document.into()),
            payment_entry: Some(payment_entry.into()),
            against_account: None,
            amount: None,
            posting_date: None,
            cheque_number: None,
            cheque_date: None,
            clearance_date: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_document", "Payment Document").options("DocType"),
            FieldSpec::dynamic_link("payment_entry")
                .label("Payment Entry")
                .options("payment_document")
                .columns(2)
                .in_list_view()
                .oldfield("voucher_id", "Link"),
            FieldSpec::data("against_account", "Against Account")
                .columns(2)
                .in_list_view()
                .read_only()
                .oldfield("against_account", "Data")
                .width("15"),
            FieldSpec::data("amount", "Amount")
                .columns(2)
                .in_list_view()
                .read_only()
                .oldfield("debit", "Currency"),
            FieldSpec::column_break("column_break_5").width("50%"),
            FieldSpec::date("posting_date", "Posting Date")
                .columns(2)
                .read_only()
                .oldfield("posting_date", "Date"),
            FieldSpec::data("cheque_number", "Cheque Number")
                .columns(1)
                .in_list_view()
                .read_only()
                .oldfield("cheque_number", "Data"),
            FieldSpec::date("cheque_date", "Cheque Date")
                .columns(2)
                .in_list_view()
                .read_only()
                .oldfield("cheque_date", "Date"),
            FieldSpec::date("clearance_date", "Clearance Date")
                .columns(2)
                .in_list_view()
                .oldfield("clearance_date", "Date"),
        ]
    }
}

impl DocumentController for BankClearanceDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
