use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiscountedInvoice {
    pub sales_invoice: Option<String>,
    pub customer: Option<String>,
    pub posting_date: Option<String>,
    pub outstanding_amount: Option<String>,
    pub debit_to: Option<String>,
}

impl DiscountedInvoice {
    pub const DOCTYPE: &'static str = "Discounted Invoice";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "sales_invoice",
        "customer",
        "column_break_3",
        "posting_date",
        "outstanding_amount",
        "debit_to",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(sales_invoice: impl Into<String>) -> Self {
        Self {
            sales_invoice: Some(sales_invoice.into()),
            customer: None,
            posting_date: None,
            outstanding_amount: None,
            debit_to: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("sales_invoice", "Invoice")
                .options("Sales Invoice")
                .required()
                .in_list_view()
                .search_index(),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .fetch_from("sales_invoice.customer")
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Date")
                .fetch_from("sales_invoice.posting_date")
                .read_only()
                .in_list_view(),
            FieldSpec::currency("outstanding_amount", "Outstanding Amount")
                .options("Company:company:default_currency")
                .fetch_from("sales_invoice.outstanding_amount")
                .fetch_if_empty()
                .in_list_view(),
            FieldSpec::link("debit_to", "Debit to")
                .options("Account")
                .fetch_from("sales_invoice.debit_to")
                .read_only(),
        ]
    }
}

impl DocumentController for DiscountedInvoice {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
