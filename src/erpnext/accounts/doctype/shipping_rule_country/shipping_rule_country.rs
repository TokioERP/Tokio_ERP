use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShippingRuleCountry {
    pub country: Option<String>,
}

impl ShippingRuleCountry {
    pub const DOCTYPE: &'static str = "Shipping Rule Country";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["country"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(country: impl Into<String>) -> Self {
        Self {
            country: Some(country.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("country", "Country")
            .options("Country")
            .required()
            .in_list_view()]
    }
}

impl DocumentController for ShippingRuleCountry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
