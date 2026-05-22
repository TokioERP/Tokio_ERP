use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CashierClosingPayments {
    pub mode_of_payment: Option<String>,
    pub amount: Option<String>,
}

impl CashierClosingPayments {
    pub const DOCTYPE: &'static str = "Cashier Closing Payments";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["mode_of_payment", "amount"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(mode_of_payment: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            mode_of_payment: Some(mode_of_payment.into()),
            amount: Some(amount.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::float("amount", "Amount")
                .default("0.00")
                .in_list_view(),
        ]
    }
}

impl DocumentController for CashierClosingPayments {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
