use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentReference {
    pub payment_term: Option<String>,
    pub payment_schedule: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub amount: Option<String>,
}

impl PaymentReference {
    pub const DOCTYPE: &'static str = "Payment Reference";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 9] = [
        "payment_term",
        "column_break_lnjp",
        "payment_schedule",
        "section_break_fjhh",
        "description",
        "section_break_mjlv",
        "due_date",
        "column_break_qghl",
        "amount",
    ];
    pub const IS_TABLE: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const GRID_PAGE_LENGTH: Option<u16> = Some(50);
    pub const ROW_FORMAT: Option<&'static str> = Some("Dynamic");
    pub const ROWS_THRESHOLD_FOR_GRID_SEARCH: Option<u16> = Some(20);

    pub fn new(payment_term: impl Into<String>) -> Self {
        Self {
            payment_term: Some(payment_term.into()),
            payment_schedule: None,
            description: None,
            due_date: None,
            amount: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .in_list_view(),
            FieldSpec::column_break("column_break_lnjp"),
            FieldSpec::link("payment_schedule", "Payment Schedule")
                .options("Payment Schedule")
                .allow_on_submit()
                .read_only(),
            FieldSpec::section_break("section_break_fjhh")
                .label("Description")
                .collapsible(),
            FieldSpec::small_text("description", "Description").in_list_view(),
            FieldSpec::section_break("section_break_mjlv"),
            FieldSpec::date("due_date", "Due Date").in_list_view(),
            FieldSpec::column_break("column_break_qghl"),
            FieldSpec::currency("amount", "Amount")
                .precision("2")
                .in_list_view(),
        ]
    }
}

impl DocumentController for PaymentReference {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
