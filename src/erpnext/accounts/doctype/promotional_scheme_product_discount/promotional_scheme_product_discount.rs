use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PromotionalSchemeProductDiscount {
    pub rule_description: Option<String>,
    pub free_item: Option<String>,
}

impl PromotionalSchemeProductDiscount {
    pub const DOCTYPE: &'static str = "Promotional Scheme Product Discount";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 26] = [
        "disable",
        "apply_multiple_pricing_rules",
        "column_break_2",
        "rule_description",
        "section_break_1",
        "min_qty",
        "max_qty",
        "column_break_3",
        "min_amount",
        "max_amount",
        "section_break_6",
        "same_item",
        "free_item",
        "free_qty",
        "column_break_9",
        "free_item_uom",
        "free_item_rate",
        "round_free_qty",
        "section_break_12",
        "warehouse",
        "threshold_percentage",
        "column_break_15",
        "priority",
        "is_recursive",
        "recurse_for",
        "apply_recursion_over",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(rule_description: impl Into<String>) -> Self {
        Self {
            rule_description: Some(rule_description.into()),
            free_item: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::check("disable", "Disable").default("0"),
            FieldSpec::check(
                "apply_multiple_pricing_rules",
                "Apply Multiple Pricing Rules",
            )
            .default("0"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::small_text("rule_description", "Rule Description").required(),
            FieldSpec::section_break("section_break_1"),
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
            FieldSpec::section_break("section_break_6").label("Free Item"),
            FieldSpec::check("same_item", "Same Item")
                .default("0")
                .depends_on("eval:!parent.mixed_conditions"),
            FieldSpec::link("free_item", "Item Code")
                .options("Item")
                .depends_on("eval:!doc.same_item || parent.mixed_conditions")
                .in_list_view(),
            FieldSpec::float("free_qty", "Qty").in_list_view(),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::link("free_item_uom", "UOM").options("UOM"),
            FieldSpec::currency("free_item_rate", "Rate"),
            FieldSpec::check("round_free_qty", "Round Free Qty").default("0"),
            FieldSpec::section_break("section_break_12"),
            FieldSpec::link("warehouse", "Warehouse").options("Warehouse"),
            FieldSpec::percent("threshold_percentage", "Threshold for Suggestion"),
            FieldSpec::column_break("column_break_15"),
            FieldSpec::select("priority", "Priority")
                .options("\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20"),
            FieldSpec::check("is_recursive", "Is Recursive").default("0"),
            FieldSpec::float("recurse_for", "Recurse Every (As Per Transaction UOM)")
                .default("0")
                .depends_on("is_recursive")
                .mandatory_depends_on("is_recursive"),
            FieldSpec::float(
                "apply_recursion_over",
                "Apply Recursion Over (As Per Transaction UOM)",
            )
            .default("0")
            .depends_on("is_recursive")
            .mandatory_depends_on("is_recursive"),
        ]
    }
}

impl DocumentController for PromotionalSchemeProductDiscount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
