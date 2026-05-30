use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeRateRevaluationAccount;

impl ExchangeRateRevaluationAccount {
    pub const DOCTYPE: &'static str = "Exchange Rate Revaluation Account";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 20] = [
        "account",
        "party_type",
        "party",
        "column_break_2",
        "account_currency",
        "account_balances",
        "balance_in_account_currency",
        "column_break_46yz",
        "new_balance_in_account_currency",
        "balances",
        "current_exchange_rate",
        "column_break_xown",
        "new_exchange_rate",
        "column_break_9",
        "balance_in_base_currency",
        "column_break_ukce",
        "new_balance_in_base_currency",
        "section_break_ngrs",
        "gain_loss",
        "zero_balance",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("party_type", "Party Type").options("DocType"),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .read_only(),
            FieldSpec::currency("balance_in_account_currency", "Balance In Account Currency")
                .options("account_currency")
                .read_only(),
            FieldSpec::section_break("balances"),
            FieldSpec::float("current_exchange_rate", "Current Exchange Rate")
                .read_only()
                .precision("9"),
            FieldSpec::currency("balance_in_base_currency", "Balance In Base Currency")
                .options("Company:company:default_currency")
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("column_break_9"),
            FieldSpec::float("new_exchange_rate", "New Exchange Rate")
                .required()
                .in_list_view()
                .precision("9"),
            FieldSpec::currency(
                "new_balance_in_base_currency",
                "New Balance In Base Currency",
            )
            .options("Company:company:default_currency")
            .read_only()
            .in_list_view(),
            FieldSpec::currency("gain_loss", "Gain/Loss")
                .options("Company:company:default_currency")
                .read_only()
                .in_list_view(),
            FieldSpec::check("zero_balance", "Zero Balance")
                .default("0")
                .description(
                    "This Account has '0' balance in either Base Currency or Account Currency",
                ),
            FieldSpec::currency(
                "new_balance_in_account_currency",
                "New Balance In Account Currency",
            )
            .options("account_currency")
            .read_only(),
            FieldSpec::section_break("account_balances"),
            FieldSpec::column_break("column_break_46yz"),
            FieldSpec::column_break("column_break_xown"),
            FieldSpec::column_break("column_break_ukce"),
            FieldSpec::section_break("section_break_ngrs"),
        ]
    }
}

impl DocumentController for ExchangeRateRevaluationAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
