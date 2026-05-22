use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentTermsTemplateDetail {
    pub idx: u32,
    pub payment_term: Option<String>,
    pub description: Option<String>,
    pub invoice_portion: Option<String>,
    pub mode_of_payment: Option<String>,
    pub due_date_based_on: Option<String>,
    pub credit_days: Option<String>,
    pub credit_months: Option<String>,
    pub discount_type: Option<String>,
    pub discount: Option<String>,
    pub discount_validity_based_on: Option<String>,
    pub discount_validity: Option<String>,
}

impl PaymentTermsTemplateDetail {
    pub const DOCTYPE: &'static str = "Payment Terms Template Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 16] = [
        "payment_term",
        "section_break_13",
        "description",
        "section_break_4",
        "invoice_portion",
        "mode_of_payment",
        "column_break_3",
        "due_date_based_on",
        "credit_days",
        "credit_months",
        "section_break_8",
        "discount_type",
        "discount",
        "column_break_11",
        "discount_validity_based_on",
        "discount_validity",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(
        payment_term: Option<impl Into<String>>,
        invoice_portion: impl Into<String>,
        due_date_based_on: impl Into<String>,
        idx: u32,
    ) -> Self {
        Self {
            idx,
            payment_term: payment_term.map(Into::into),
            invoice_portion: Some(invoice_portion.into()),
            due_date_based_on: Some(due_date_based_on.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_13").label("Description"),
            FieldSpec::small_text("description", "Description")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::float("invoice_portion", "Invoice Portion (%)")
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::link("mode_of_payment", "Mode of Payment").options("Mode of Payment"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::select("due_date_based_on", "Due Date Based On")
                .options(
                    "Day(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::int("credit_days", "Credit Days")
                .in_list_view()
                .columns(2)
                .default("0")
                .depends_on(
                    "eval:in_list(['Day(s) after invoice date', 'Day(s) after the end of the invoice month'], doc.due_date_based_on)",
                ),
            FieldSpec::int("credit_months", "Credit Months")
                .default("0")
                .depends_on("eval:doc.due_date_based_on=='Month(s) after the end of the invoice month'"),
            FieldSpec::section_break("section_break_8").label("Discount Settings"),
            FieldSpec::select("discount_type", "Discount Type")
                .options("Percentage\nAmount")
                .default("Percentage"),
            FieldSpec::float("discount", "Discount"),
            FieldSpec::column_break("column_break_11"),
            FieldSpec::select("discount_validity_based_on", "Discount Validity Based On")
                .options(
                    "Day(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .default("Day(s) after invoice date")
                .depends_on("discount"),
            FieldSpec::int("discount_validity", "Discount Validity").depends_on("discount"),
        ]
    }
}

impl DocumentController for PaymentTermsTemplateDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
