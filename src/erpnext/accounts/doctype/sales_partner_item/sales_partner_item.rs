use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesPartnerItem {
    pub sales_partner: Option<String>,
}

impl SalesPartnerItem {
    pub const DOCTYPE: &'static str = "Sales Partner Item";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["sales_partner"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(sales_partner: impl Into<String>) -> Self {
        Self {
            sales_partner: Some(sales_partner.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("sales_partner", "Sales Partner ")
            .options("Sales Partner")
            .in_list_view()]
    }
}

impl DocumentController for SalesPartnerItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
