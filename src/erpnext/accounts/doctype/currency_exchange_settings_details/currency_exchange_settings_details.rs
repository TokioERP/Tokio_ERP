use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CurrencyExchangeSettingsDetails {
    pub key: Option<String>,
    pub value: Option<String>,
}

impl CurrencyExchangeSettingsDetails {
    pub const DOCTYPE: &'static str = "Currency Exchange Settings Details";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["key", "value"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: Some(key.into()),
            value: Some(value.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("key", "Key").required().in_list_view(),
            FieldSpec::data("value", "Value").required().in_list_view(),
        ]
    }
}

impl DocumentController for CurrencyExchangeSettingsDetails {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
