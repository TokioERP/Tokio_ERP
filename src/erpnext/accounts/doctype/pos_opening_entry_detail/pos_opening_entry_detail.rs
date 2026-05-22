use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosOpeningEntryDetail {
    pub mode_of_payment: Option<String>,
    pub opening_amount: Option<String>,
}

impl PosOpeningEntryDetail {
    pub const DOCTYPE: &'static str = "POS Opening Entry Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["mode_of_payment", "opening_amount"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(mode_of_payment: impl Into<String>, opening_amount: impl Into<String>) -> Self {
        Self {
            mode_of_payment: Some(mode_of_payment.into()),
            opening_amount: Some(opening_amount.into()),
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
                .default("0"),
        ]
    }
}

impl DocumentController for PosOpeningEntryDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
