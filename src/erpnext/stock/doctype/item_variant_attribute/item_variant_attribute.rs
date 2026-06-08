use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemVariantAttribute {
    pub variant_of: Option<String>,
    pub attribute: Option<String>,
    pub attribute_value: Option<String>,
    pub numeric_values: bool,
    pub disabled: bool,
    pub from_range: Option<f64>,
    pub increment: Option<f64>,
    pub to_range: Option<f64>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemVariantAttribute {
    pub const DOCTYPE: &'static str = "Item Variant Attribute";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 11] = [
        "variant_of",
        "attribute",
        "column_break_2",
        "attribute_value",
        "numeric_values",
        "disabled",
        "section_break_4",
        "from_range",
        "increment",
        "column_break_8",
        "to_range",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        variant_of: Option<&str>,
        attribute: impl Into<String>,
        attribute_value: Option<&str>,
        numeric_values: bool,
        disabled: bool,
        from_range: Option<f64>,
        increment: Option<f64>,
        to_range: Option<f64>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            variant_of: variant_of.map(ToOwned::to_owned),
            attribute: Some(attribute.into()),
            attribute_value: attribute_value.map(ToOwned::to_owned),
            numeric_values,
            disabled,
            from_range,
            increment,
            to_range,
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("variant_of", "Variant Of")
                .options("Item")
                .search_index(),
            FieldSpec::link("attribute", "Attribute")
                .options("Item Attribute")
                .in_list_view()
                .required()
                .search_index(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::data("attribute_value", "Attribute Value").in_list_view(),
            FieldSpec::check("numeric_values", "Numeric Values")
                .default("0")
                .depends_on("has_variants"),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .fetch_from("attribute.disabled"),
            FieldSpec::section_break("section_break_4").depends_on("numeric_values"),
            FieldSpec::float("from_range", "From Range"),
            FieldSpec::float("increment", "Increment"),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::float("to_range", "To Range"),
        ]
    }
}

impl DocumentController for ItemVariantAttribute {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
