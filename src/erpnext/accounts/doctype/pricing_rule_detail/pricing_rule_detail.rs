use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PricingRuleDetail {
    pub pricing_rule: Option<String>,
    pub item_code: Option<String>,
    pub margin_type: Option<String>,
    pub rate_or_discount: Option<String>,
    pub child_docname: Option<String>,
    pub rule_applied: bool,
}

impl Default for PricingRuleDetail {
    fn default() -> Self {
        Self {
            pricing_rule: None,
            item_code: None,
            margin_type: None,
            rate_or_discount: None,
            child_docname: None,
            rule_applied: true,
        }
    }
}

impl PricingRuleDetail {
    pub const DOCTYPE: &'static str = "Pricing Rule Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "pricing_rule",
        "item_code",
        "margin_type",
        "rate_or_discount",
        "child_docname",
        "rule_applied",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        pricing_rule: impl Into<String>,
        item_code: impl Into<String>,
        child_docname: impl Into<String>,
    ) -> Self {
        Self {
            pricing_rule: Some(pricing_rule.into()),
            item_code: Some(item_code.into()),
            child_docname: Some(child_docname.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("pricing_rule", "Pricing Rule")
                .options("Pricing Rule")
                .in_list_view()
                .read_only(),
            FieldSpec::data("item_code", "Item Code")
                .in_list_view()
                .read_only(),
            FieldSpec::data("margin_type", "Margin Type")
                .hidden()
                .read_only(),
            FieldSpec::data("rate_or_discount", "Rate or Discount")
                .hidden()
                .read_only(),
            FieldSpec::data("child_docname", "Child Docname")
                .hidden()
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::check("rule_applied", "Rule Applied")
                .default("1")
                .read_only(),
        ]
    }
}

impl DocumentController for PricingRuleDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
