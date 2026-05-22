use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosInvoiceReference {
    pub pos_invoice: Option<String>,
    pub posting_date: Option<String>,
    pub customer: Option<String>,
    pub grand_total: Option<String>,
    pub is_return: bool,
    pub return_against: Option<String>,
}

impl PosInvoiceReference {
    pub const DOCTYPE: &'static str = "POS Invoice Reference";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "pos_invoice",
        "posting_date",
        "column_break_3",
        "customer",
        "grand_total",
        "is_return",
        "return_against",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(
        pos_invoice: impl Into<String>,
        posting_date: impl Into<String>,
        customer: impl Into<String>,
        grand_total: impl Into<String>,
    ) -> Self {
        Self {
            pos_invoice: Some(pos_invoice.into()),
            posting_date: Some(posting_date.into()),
            customer: Some(customer.into()),
            grand_total: Some(grand_total.into()),
            is_return: false,
            return_against: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("pos_invoice", "POS Invoice")
                .options("POS Invoice")
                .required()
                .in_list_view(),
            FieldSpec::date("posting_date", "Date")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .required()
                .read_only(),
            FieldSpec::currency("grand_total", "Amount")
                .required()
                .in_list_view(),
            FieldSpec::check("is_return", "Is Return")
                .read_only()
                .default("0"),
            FieldSpec::link("return_against", "Return Against")
                .options("POS Invoice")
                .read_only(),
        ]
    }
}

impl DocumentController for PosInvoiceReference {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
