use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OverduePayment;

impl OverduePayment {
    pub const DOCTYPE: &'static str = "Overdue Payment";
    pub const MODULE: &'static str = "Accounts";
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 18] = [
        "sales_invoice",
        "payment_schedule",
        "dunning_level",
        "payment_term",
        "section_break_15",
        "description",
        "section_break_4",
        "due_date",
        "overdue_days",
        "mode_of_payment",
        "column_break_5",
        "invoice_portion",
        "section_break_16",
        "payment_amount",
        "outstanding",
        "paid_amount",
        "discounted_amount",
        "interest",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .columns(2)
                .read_only()
                .print_hide(),
            FieldSpec::section_break("section_break_15")
                .label("Description")
                .collapsible(),
            FieldSpec::small_text("description", "Description")
                .columns(2)
                .fetch_from("payment_term.description")
                .read_only(),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::date("due_date", "Due Date")
                .columns(2)
                .read_only(),
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .read_only(),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::percent("invoice_portion", "Invoice Portion")
                .columns(2)
                .read_only(),
            FieldSpec::currency("payment_amount", "Payment Amount")
                .options("currency")
                .columns(2)
                .read_only(),
            FieldSpec::currency("outstanding", "Outstanding")
                .options("currency")
                .fetch_from("payment_amount")
                .read_only()
                .in_list_view(),
            FieldSpec::currency("paid_amount", "Paid Amount")
                .options("currency")
                .depends_on("paid_amount"),
            FieldSpec::currency("discounted_amount", "Discounted Amount")
                .default("0")
                .depends_on("discounted_amount")
                .read_only()
                .print_hide(),
            FieldSpec::link("sales_invoice", "Sales Invoice")
                .options("Sales Invoice")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("payment_schedule", "Payment Schedule")
                .read_only()
                .print_hide(),
            FieldSpec::data("overdue_days", "Overdue Days")
                .read_only()
                .in_list_view(),
            FieldSpec::int("dunning_level", "Dunning Level")
                .default("1")
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_16"),
            FieldSpec::currency("interest", "Interest")
                .options("currency")
                .read_only()
                .in_list_view(),
        ]
    }
}

impl DocumentController for OverduePayment {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
