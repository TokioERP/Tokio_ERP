use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosClosingEntryDetail {
    pub mode_of_payment: Option<String>,
    pub opening_amount: Option<String>,
    pub expected_amount: Option<String>,
    pub closing_amount: Option<String>,
    pub difference: Option<String>,
}

impl PosClosingEntryDetail {
    pub const DOCTYPE: &'static str = "POS Closing Entry Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 5] = [
        "mode_of_payment",
        "opening_amount",
        "expected_amount",
        "closing_amount",
        "difference",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(
        mode_of_payment: impl Into<String>,
        opening_amount: impl Into<String>,
        expected_amount: impl Into<String>,
        closing_amount: impl Into<String>,
        difference: impl Into<String>,
    ) -> Self {
        Self {
            mode_of_payment: Some(mode_of_payment.into()),
            opening_amount: Some(opening_amount.into()),
            expected_amount: Some(expected_amount.into()),
            closing_amount: Some(closing_amount.into()),
            difference: Some(difference.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
            FieldSpec::currency("opening_amount", "Opening Amount")
                .options("company:company_currency")
                .required()
                .in_list_view()
                .read_only(),
            FieldSpec::currency("expected_amount", "Expected Amount")
                .options("company:company_currency")
                .in_list_view()
                .read_only(),
            FieldSpec::currency("closing_amount", "Closing Amount")
                .options("company:company_currency")
                .required()
                .in_list_view()
                .default("0"),
            FieldSpec::currency("difference", "Difference")
                .options("company:company_currency")
                .in_list_view()
                .read_only(),
        ]
    }
}

impl DocumentController for PosClosingEntryDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
