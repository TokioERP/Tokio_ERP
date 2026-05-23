use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PromotionalSchemePriceDiscount {
    pub rule_description: Option<String>,
    pub min_qty: Option<String>,
}

impl PromotionalSchemePriceDiscount {
    pub const DOCTYPE: &'static str = "Promotional Scheme Price Discount";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 24] = [
        "disable",
        "apply_multiple_pricing_rules",
        "column_break_2",
        "rule_description",
        "section_break_2",
        "min_qty",
        "max_qty",
        "column_break_3",
        "min_amount",
        "max_amount",
        "section_break_6",
        "rate_or_discount",
        "column_break_10",
        "rate",
        "discount_amount",
        "discount_percentage",
        "for_price_list",
        "section_break_11",
        "warehouse",
        "threshold_percentage",
        "validate_applied_rule",
        "column_break_14",
        "priority",
        "apply_discount_on_rate",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(rule_description: impl Into<String>) -> Self {
        Self {
            rule_description: Some(rule_description.into()),
            min_qty: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::check("disable", "Disable").default("0"),
            FieldSpec::check("apply_multiple_pricing_rules", "Apply Multiple Pricing Rules")
                .default("0")
                .depends_on("priority"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::small_text("rule_description", "Rule Description").required(),
            FieldSpec::section_break("section_break_2"),
            FieldSpec::float("min_qty", "Min Qty")
                .default("0")
                .in_list_view(),
            FieldSpec::float("max_qty", "Max Qty")
                .default("0")
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::currency("min_amount", "Min Amount")
                .default("0")
                .in_list_view(),
            FieldSpec::currency("max_amount", "Max Amount")
                .default("0")
                .in_list_view(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::select("rate_or_discount", "Discount Type")
                .options("\nRate\nDiscount Percentage\nDiscount Amount")
                .default("Discount Percentage")
                .in_list_view(),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::currency("rate", "Rate").depends_on("eval:doc.rate_or_discount==\"Rate\""),
            FieldSpec::currency("discount_amount", "Discount Amount")
                .depends_on("eval:doc.rate_or_discount==\"Discount Amount\""),
            FieldSpec::float("discount_percentage", "Discount Percentage")
                .depends_on("eval:doc.rate_or_discount==\"Discount Percentage\""),
            FieldSpec::link("for_price_list", "For Price List")
                .options("Price List")
                .depends_on("eval:doc.rate_or_discount!=\"Rate\""),
            FieldSpec::section_break("section_break_11"),
            FieldSpec::link("warehouse", "Warehouse").options("Warehouse"),
            FieldSpec::percent("threshold_percentage", "Threshold for Suggestion"),
            FieldSpec::check("validate_applied_rule", "Validate Applied Rule").default("0"),
            FieldSpec::column_break("column_break_14"),
            FieldSpec::select("priority", "Priority").options(
                "\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20",
            ),
            FieldSpec::check("apply_discount_on_rate", "Apply Discount on Rate")
                .default("0")
                .depends_on("eval:in_list(['Discount Percentage', 'Discount Amount'], doc.rate_or_discount) && doc.apply_multiple_pricing_rules"),
        ]
    }
}

impl DocumentController for PromotionalSchemePriceDiscount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
