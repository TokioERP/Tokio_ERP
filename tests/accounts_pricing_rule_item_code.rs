use tokio_erp::erpnext::accounts::doctype::pricing_rule_item_code::pricing_rule_item_code::PricingRuleItemCode;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pricing_rule_item_code_matches_erpnext_metadata() {
    assert_eq!(PricingRuleItemCode::DOCTYPE, "Pricing Rule Item Code");
    assert_eq!(PricingRuleItemCode::MODULE, "Accounts");
    assert_eq!(PricingRuleItemCode::FIELD_ORDER, ["item_code", "uom"]);
    assert!(PricingRuleItemCode::IS_TABLE);
    assert!(PricingRuleItemCode::EDITABLE_GRID);
    assert!(PricingRuleItemCode::TRACK_CHANGES);

    assert_eq!(
        PricingRuleItemCode::fields(),
        vec![
            FieldSpec::link("item_code", "Item Code")
                .options("Item")
                .in_list_view()
                .search_index()
                .depends_on("eval:parent.apply_on == 'Item Code'"),
            FieldSpec::link("uom", "UOM").options("UOM").in_list_view(),
        ]
    );
}

#[test]
fn pricing_rule_item_code_preserves_pass_controller_behavior() {
    let blank = PricingRuleItemCode::default();
    assert_eq!(blank.item_code, None);
    assert_eq!(blank.uom, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PricingRuleItemCode::new("ITEM-0001", "Nos");
    assert_eq!(row.item_code.as_deref(), Some("ITEM-0001"));
    assert_eq!(row.uom.as_deref(), Some("Nos"));
    assert_eq!(row.doctype(), "Pricing Rule Item Code");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
