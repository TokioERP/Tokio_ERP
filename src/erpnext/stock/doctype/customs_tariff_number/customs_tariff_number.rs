use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomsTariffNumber {
    pub tariff_number: Option<String>,
    pub description: Option<String>,
}

impl CustomsTariffNumber {
    pub const DOCTYPE: &'static str = "Customs Tariff Number";
    pub const MODULE: &'static str = "Stock";
    pub const AUTONAME: &'static str = "field:tariff_number";
    pub const FIELD_ORDER: [&'static str; 2] = ["tariff_number", "description"];
    pub const ALLOW_RENAME: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(tariff_number: impl Into<String>, description: Option<&str>) -> Self {
        Self {
            tariff_number: Some(tariff_number.into()),
            description: description.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("tariff_number", "Tariff Number")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::data("description", "Description").in_list_view(),
        ]
    }
}

impl DocumentController for CustomsTariffNumber {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
