use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentTerm {
    pub payment_term_name: Option<String>,
    pub invoice_portion: Option<String>,
    pub mode_of_payment: Option<String>,
    pub due_date_based_on: Option<String>,
    pub credit_days: Option<String>,
    pub credit_months: Option<String>,
    pub discount_type: Option<String>,
    pub discount: Option<String>,
    pub discount_validity_based_on: Option<String>,
    pub discount_validity: Option<String>,
    pub description: Option<String>,
}

impl PaymentTerm {
    pub const DOCTYPE: &'static str = "Payment Term";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: Option<&'static str> = Some("field:payment_term_name");
    pub const FIELD_ORDER: [&'static str; 15] = [
        "payment_term_name",
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
        "section_break_6",
        "description",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(payment_term_name: impl Into<String>) -> Self {
        Self {
            payment_term_name: Some(payment_term_name.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("payment_term_name", "Payment Term Name").unique(),
            FieldSpec::float("invoice_portion", "Invoice Portion (%)"),
            FieldSpec::link("mode_of_payment", "Mode of Payment").options("Mode of Payment"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::select("due_date_based_on", "Due Date Based On").options(
                "Day(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
            ),
            FieldSpec::int("credit_days", "Credit Days").depends_on(
                "eval:in_list(['Day(s) after invoice date', 'Day(s) after the end of the invoice month'], doc.due_date_based_on)",
            ),
            FieldSpec::int("credit_months", "Credit Months")
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
            FieldSpec::section_break("section_break_6"),
            FieldSpec::small_text("description", "Description"),
        ]
    }
}

impl DocumentController for PaymentTerm {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
