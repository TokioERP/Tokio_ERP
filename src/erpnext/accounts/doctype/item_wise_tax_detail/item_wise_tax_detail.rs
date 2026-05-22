use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemWiseTaxDetail {
    pub item_row: Option<String>,
    pub tax_row: Option<String>,
    pub rate: Option<String>,
    pub amount: Option<String>,
    pub taxable_amount: Option<String>,
}

impl ItemWiseTaxDetail {
    pub const DOCTYPE: &'static str = "Item Wise Tax Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 5] =
        ["item_row", "tax_row", "rate", "amount", "taxable_amount"];
    pub const IS_TABLE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const GRID_PAGE_LENGTH: Option<u16> = Some(50);
    pub const ROW_FORMAT: Option<&'static str> = Some("Dynamic");

    pub fn new(item_row: impl Into<String>, tax_row: impl Into<String>) -> Self {
        Self {
            item_row: Some(item_row.into()),
            tax_row: Some(tax_row.into()),
            rate: None,
            amount: None,
            taxable_amount: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("item_row", "Item Row")
                .required()
                .in_list_view(),
            FieldSpec::data("tax_row", "Tax Row")
                .required()
                .in_list_view(),
            FieldSpec::float("rate", "Tax Rate").in_list_view(),
            FieldSpec::currency("amount", "Tax Amount")
                .options("Company:company:default_currency")
                .in_list_view(),
            FieldSpec::currency("taxable_amount", "Taxable Amount")
                .options("Company:company:default_currency")
                .in_list_view(),
        ]
    }
}

impl DocumentController for ItemWiseTaxDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
