use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeggedCurrencyDetails {
    pub source_currency: Option<String>,
    pub pegged_against: Option<String>,
    pub pegged_exchange_rate: Option<String>,
}

impl PeggedCurrencyDetails {
    pub const DOCTYPE: &'static str = "Pegged Currency Details";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] =
        ["source_currency", "pegged_against", "pegged_exchange_rate"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(
        source_currency: impl Into<String>,
        pegged_against: impl Into<String>,
        pegged_exchange_rate: impl Into<String>,
    ) -> Self {
        Self {
            source_currency: Some(source_currency.into()),
            pegged_against: Some(pegged_against.into()),
            pegged_exchange_rate: Some(pegged_exchange_rate.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("source_currency", "Currency")
                .options("Currency")
                .in_list_view(),
            FieldSpec::link("pegged_against", "Pegged Against")
                .options("Currency")
                .in_list_view(),
            FieldSpec::data("pegged_exchange_rate", "Exchange Rate").in_list_view(),
        ]
    }
}

impl DocumentController for PeggedCurrencyDetails {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
