use tokio_erp::erpnext::accounts::doctype::pricing_rule_item_group::pricing_rule_item_group::PricingRuleItemGroup;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pricing_rule_item_group_matches_erpnext_metadata() {
    assert_eq!(PricingRuleItemGroup::DOCTYPE, "Pricing Rule Item Group");
    assert_eq!(PricingRuleItemGroup::MODULE, "Accounts");
    assert_eq!(PricingRuleItemGroup::FIELD_ORDER, ["item_group", "uom"]);
    assert!(PricingRuleItemGroup::IS_TABLE);
    assert!(PricingRuleItemGroup::EDITABLE_GRID);
    assert!(PricingRuleItemGroup::TRACK_CHANGES);
    assert_eq!(PricingRuleItemGroup::ROW_FORMAT, "Dynamic");

    assert_eq!(
        PricingRuleItemGroup::fields(),
        vec![
            FieldSpec::link("item_group", "Item Group")
                .options("Item Group")
                .in_list_view()
                .search_index()
                .depends_on("eval:parent.apply_on == 'Item Group'"),
            FieldSpec::link("uom", "UOM").options("UOM").in_list_view(),
        ]
    );
}

#[test]
fn pricing_rule_item_group_preserves_pass_controller_behavior() {
    let blank = PricingRuleItemGroup::default();
    assert_eq!(blank.item_group, None);
    assert_eq!(blank.uom, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PricingRuleItemGroup::new("Products", "Nos");
    assert_eq!(row.item_group.as_deref(), Some("Products"));
    assert_eq!(row.uom.as_deref(), Some("Nos"));
    assert_eq!(row.doctype(), "Pricing Rule Item Group");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
