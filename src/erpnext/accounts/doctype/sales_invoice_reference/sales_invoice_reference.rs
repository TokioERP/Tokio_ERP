use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesInvoiceReference {
    pub sales_invoice: Option<String>,
    pub posting_date: Option<String>,
    pub customer: Option<String>,
    pub grand_total: Option<String>,
    pub is_return: bool,
    pub return_against: Option<String>,
}

impl SalesInvoiceReference {
    pub const DOCTYPE: &'static str = "Sales Invoice Reference";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "sales_invoice",
        "posting_date",
        "column_break_fear",
        "customer",
        "grand_total",
        "is_return",
        "return_against",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(sales_invoice: impl Into<String>, posting_date: impl Into<String>) -> Self {
        Self {
            sales_invoice: Some(sales_invoice.into()),
            posting_date: Some(posting_date.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("sales_invoice", "Sales Invoice")
                .options("Sales Invoice")
                .required()
                .in_list_view(),
            FieldSpec::date("posting_date", "Date")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_fear"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .fetch_from("sales_invoice.customer")
                .read_only()
                .required(),
            FieldSpec::currency("grand_total", "Amount")
                .fetch_from("sales_invoice.grand_total")
                .required()
                .in_list_view(),
            FieldSpec::check("is_return", "Is Return")
                .default("0")
                .fetch_from("sales_invoice.is_return")
                .read_only(),
            FieldSpec::link("return_against", "Return Against")
                .options("Sales Invoice")
                .fetch_from("sales_invoice.return_against")
                .read_only(),
        ]
    }
}

impl DocumentController for SalesInvoiceReference {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
