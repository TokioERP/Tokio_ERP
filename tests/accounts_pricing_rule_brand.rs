use tokio_erp::erpnext::accounts::doctype::pricing_rule_brand::pricing_rule_brand::PricingRuleBrand;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pricing_rule_brand_matches_erpnext_metadata() {
    assert_eq!(PricingRuleBrand::DOCTYPE, "Pricing Rule Brand");
    assert_eq!(PricingRuleBrand::MODULE, "Accounts");
    assert_eq!(PricingRuleBrand::FIELD_ORDER, ["brand", "uom"]);
    assert!(PricingRuleBrand::IS_TABLE);
    assert!(PricingRuleBrand::EDITABLE_GRID);

    assert_eq!(
        PricingRuleBrand::fields(),
        vec![
            FieldSpec::link("brand", "Brand")
                .options("Brand")
                .in_list_view()
                .depends_on("eval:parent.apply_on == 'Brand'"),
            FieldSpec::link("uom", "UOM").options("UOM").in_list_view(),
        ]
    );
}

#[test]
fn pricing_rule_brand_preserves_pass_controller_behavior() {
    let blank = PricingRuleBrand::default();
    assert_eq!(blank.brand, None);
    assert_eq!(blank.uom, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PricingRuleBrand::new("Acme", "Nos");
    assert_eq!(row.brand.as_deref(), Some("Acme"));
    assert_eq!(row.uom.as_deref(), Some("Nos"));
    assert_eq!(row.doctype(), "Pricing Rule Brand");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
