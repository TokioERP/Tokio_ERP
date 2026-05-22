use tokio_erp::erpnext::accounts::doctype::pricing_rule_detail::pricing_rule_detail::PricingRuleDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pricing_rule_detail_matches_erpnext_metadata() {
    assert_eq!(PricingRuleDetail::DOCTYPE, "Pricing Rule Detail");
    assert_eq!(PricingRuleDetail::MODULE, "Accounts");
    assert_eq!(
        PricingRuleDetail::FIELD_ORDER,
        [
            "pricing_rule",
            "item_code",
            "margin_type",
            "rate_or_discount",
            "child_docname",
            "rule_applied",
        ]
    );
    assert!(PricingRuleDetail::IS_TABLE);
    assert!(PricingRuleDetail::EDITABLE_GRID);
    assert!(PricingRuleDetail::QUICK_ENTRY);
    assert!(PricingRuleDetail::TRACK_CHANGES);

    assert_eq!(
        PricingRuleDetail::fields(),
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
    );
}

#[test]
fn pricing_rule_detail_preserves_pass_controller_behavior() {
    let blank = PricingRuleDetail::default();
    assert_eq!(blank.pricing_rule, None);
    assert_eq!(blank.item_code, None);
    assert_eq!(blank.margin_type, None);
    assert_eq!(blank.rate_or_discount, None);
    assert_eq!(blank.child_docname, None);
    assert!(blank.rule_applied);
    assert!(blank.custom_hooks().is_empty());

    let row = PricingRuleDetail::new("PRLE-0001", "ITEM-0001", "child-row-1");
    assert_eq!(row.pricing_rule.as_deref(), Some("PRLE-0001"));
    assert_eq!(row.item_code.as_deref(), Some("ITEM-0001"));
    assert_eq!(row.child_docname.as_deref(), Some("child-row-1"));
    assert!(row.rule_applied);
    assert_eq!(row.doctype(), "Pricing Rule Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
