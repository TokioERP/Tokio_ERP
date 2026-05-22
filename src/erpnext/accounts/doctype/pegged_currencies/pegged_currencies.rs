use crate::erpnext::accounts::doctype::pegged_currency_details::pegged_currency_details::PeggedCurrencyDetails;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeggedCurrencies {
    pub pegged_currency_item: Vec<PeggedCurrencyDetails>,
}

impl PeggedCurrencies {
    pub const DOCTYPE: &'static str = "Pegged Currencies";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] =
        ["pegged_currencies_item_section", "pegged_currency_item"];
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(pegged_currency_item: Vec<PeggedCurrencyDetails>) -> Self {
        Self {
            pegged_currency_item,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("pegged_currencies_item_section"),
            FieldSpec::table_unlabeled("pegged_currency_item").options("Pegged Currency Details"),
        ]
    }
}

impl DocumentController for PeggedCurrencies {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
