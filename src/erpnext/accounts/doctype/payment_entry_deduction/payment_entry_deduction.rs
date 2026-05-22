use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentEntryDeduction {
    pub account: Option<String>,
    pub cost_center: Option<String>,
    pub amount: Option<String>,
    pub is_exchange_gain_loss: bool,
    pub description: Option<String>,
}

impl PaymentEntryDeduction {
    pub const DOCTYPE: &'static str = "Payment Entry Deduction";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "account",
        "cost_center",
        "amount",
        "column_break_2",
        "is_exchange_gain_loss",
        "description",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const ROW_FORMAT: Option<&'static str> = Some("Dynamic");

    pub fn new(
        account: impl Into<String>,
        cost_center: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            account: Some(account.into()),
            cost_center: Some(cost_center.into()),
            amount: Some(amount.into()),
            is_exchange_gain_loss: false,
            description: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .required()
                .allow_on_submit()
                .print_hide()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::check("is_exchange_gain_loss", "Is Exchange Gain / Loss?")
                .default("0")
                .depends_on("eval:doc.is_exchange_gain_loss")
                .read_only(),
            FieldSpec::small_text("description", "Description"),
        ]
    }
}

impl DocumentController for PaymentEntryDeduction {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
