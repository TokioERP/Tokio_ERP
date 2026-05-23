use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxCategory {
    pub title: Option<String>,
    pub disabled: bool,
}

const PRE_SALES_ITEMS: [&str; 2] = ["Quotation", "Supplier Quotation"];
const SALES_ITEMS: [&str; 3] = ["Sales Invoice", "Delivery Note", "Sales Order"];
const PURCHASE_ITEMS: [&str; 2] = ["Purchase Invoice", "Purchase Receipt"];
const PARTY_ITEMS: [&str; 2] = ["Customer", "Supplier"];
const TAXES_ITEMS: [&str; 2] = ["Item", "Tax Rule"];

impl TaxCategory {
    pub const DOCTYPE: &'static str = "Tax Category";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:title";
    pub const FIELD_ORDER: [&'static str; 2] = ["title", "disabled"];
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            disabled: false,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("title", "Title")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::check("disabled", "Disabled").default("0"),
        ]
    }
}

impl DocumentController for TaxCategory {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn tax_category_dashboard_groups() -> [(&'static str, &'static [&'static str]); 5] {
    [
        ("Pre Sales", &PRE_SALES_ITEMS),
        ("Sales", &SALES_ITEMS),
        ("Purchase", &PURCHASE_ITEMS),
        ("Party", &PARTY_ITEMS),
        ("Taxes", &TAXES_ITEMS),
    ]
}
