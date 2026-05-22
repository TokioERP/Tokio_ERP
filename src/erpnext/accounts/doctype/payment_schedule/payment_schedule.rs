use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentSchedule {
    pub payment_term: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub invoice_portion: Option<String>,
    pub mode_of_payment: Option<String>,
    pub due_date_based_on: Option<String>,
    pub credit_days: Option<String>,
    pub credit_months: Option<String>,
    pub discount_date: Option<String>,
    pub discount: Option<String>,
    pub discount_type: Option<String>,
    pub discount_validity_based_on: Option<String>,
    pub discount_validity: Option<String>,
    pub payment_amount: Option<String>,
    pub outstanding: Option<String>,
    pub paid_amount: Option<String>,
    pub discounted_amount: Option<String>,
    pub base_payment_amount: Option<String>,
    pub base_outstanding: Option<String>,
    pub base_paid_amount: Option<String>,
}

impl PaymentSchedule {
    pub const DOCTYPE: &'static str = "Payment Schedule";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 27] = [
        "payment_term",
        "section_break_15",
        "description",
        "section_break_4",
        "due_date",
        "invoice_portion",
        "mode_of_payment",
        "column_break_5",
        "due_date_based_on",
        "credit_days",
        "credit_months",
        "section_break_6",
        "discount_date",
        "discount",
        "discount_type",
        "column_break_9",
        "discount_validity_based_on",
        "discount_validity",
        "section_break_9",
        "payment_amount",
        "outstanding",
        "paid_amount",
        "discounted_amount",
        "column_break_3",
        "base_payment_amount",
        "base_outstanding",
        "base_paid_amount",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const ROW_FORMAT: Option<&'static str> = Some("Dynamic");

    pub fn new(
        payment_term: impl Into<String>,
        due_date: impl Into<String>,
        payment_amount: impl Into<String>,
    ) -> Self {
        Self {
            payment_term: Some(payment_term.into()),
            due_date: Some(due_date.into()),
            payment_amount: Some(payment_amount.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_15").label("Description"),
            FieldSpec::small_text("description", "Description")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::date("due_date", "Due Date")
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::percent("invoice_portion", "Invoice Portion")
                .in_list_view()
                .columns(2),
            FieldSpec::link("mode_of_payment", "Mode of Payment").options("Mode of Payment"),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::select("due_date_based_on", "Due Date Based On")
                .options(
                    "\nDay(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .read_only(),
            FieldSpec::int("credit_days", "Credit Days")
                .read_only()
                .depends_on(
                    "eval:in_list(['Day(s) after invoice date', 'Day(s) after the end of the invoice month'], doc.due_date_based_on)",
                ),
            FieldSpec::int("credit_months", "Credit Months")
                .read_only()
                .depends_on("eval:doc.due_date_based_on=='Month(s) after the end of the invoice month'"),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::date("discount_date", "Discount Date").depends_on("discount"),
            FieldSpec::float("discount", "Discount"),
            FieldSpec::select("discount_type", "Discount Type")
                .options("Percentage\nAmount")
                .default("Percentage"),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::select("discount_validity_based_on", "Discount Validity Based On")
                .options(
                    "\nDay(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .read_only()
                .depends_on("discount"),
            FieldSpec::int("discount_validity", "Discount Validity")
                .read_only()
                .depends_on("discount_validity_based_on"),
            FieldSpec::section_break("section_break_9"),
            FieldSpec::currency("payment_amount", "Payment Amount")
                .options("currency")
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::currency("outstanding", "Outstanding")
                .options("currency")
                .read_only(),
            FieldSpec::currency("paid_amount", "Paid Amount")
                .options("currency")
                .depends_on("paid_amount"),
            FieldSpec::currency("discounted_amount", "Discounted Amount")
                .read_only()
                .default("0")
                .depends_on("discounted_amount"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::currency("base_payment_amount", "Payment Amount (Company Currency)")
                .options("Company:company:default_currency"),
            FieldSpec::currency("base_outstanding", "Outstanding (Company Currency)")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::currency("base_paid_amount", "Paid Amount (Company Currency)")
                .options("Company:company:default_currency")
                .read_only()
                .depends_on("base_paid_amount"),
        ]
    }
}

impl DocumentController for PaymentSchedule {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
