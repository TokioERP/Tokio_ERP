use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemTax {
    pub item_tax_template: Option<String>,
    pub tax_category: Option<String>,
    pub valid_from: Option<String>,
    pub minimum_net_rate: Option<f64>,
    pub maximum_net_rate: Option<f64>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemTax {
    pub const DOCTYPE: &'static str = "Item Tax";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 5] = [
        "item_tax_template",
        "tax_category",
        "valid_from",
        "minimum_net_rate",
        "maximum_net_rate",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        item_tax_template: impl Into<String>,
        tax_category: Option<&str>,
        valid_from: Option<&str>,
        minimum_net_rate: Option<f64>,
        maximum_net_rate: Option<f64>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            item_tax_template: Some(item_tax_template.into()),
            tax_category: tax_category.map(ToOwned::to_owned),
            valid_from: valid_from.map(ToOwned::to_owned),
            minimum_net_rate,
            maximum_net_rate,
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("item_tax_template", "Item Tax Template")
                .options("Item Tax Template")
                .oldfield("tax_type", "Link")
                .in_list_view()
                .required(),
            FieldSpec::link("tax_category", "Tax Category")
                .options("Tax Category")
                .oldfield("tax_rate", "Currency")
                .in_list_view(),
            FieldSpec::date("valid_from", "Valid From").in_list_view(),
            FieldSpec::float("minimum_net_rate", "Minimum Net Rate").in_list_view(),
            FieldSpec::float("maximum_net_rate", "Maximum Net Rate").in_list_view(),
        ]
    }
}

impl DocumentController for ItemTax {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
