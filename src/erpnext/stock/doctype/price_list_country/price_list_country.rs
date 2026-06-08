use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PriceListCountry {
    pub country: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl PriceListCountry {
    pub const DOCTYPE: &'static str = "Price List Country";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 1] = ["country"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        country: Option<&str>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            country: country.map(ToOwned::to_owned),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("country", "Country")
            .options("Country")
            .in_list_view()
            .required()]
    }
}

impl DocumentController for PriceListCountry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
