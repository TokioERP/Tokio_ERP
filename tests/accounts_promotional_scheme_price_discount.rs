use tokio_erp::erpnext::accounts::doctype::promotional_scheme_price_discount::promotional_scheme_price_discount::PromotionalSchemePriceDiscount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn promotional_scheme_price_discount_matches_erpnext_metadata() {
    assert_eq!(
        PromotionalSchemePriceDiscount::DOCTYPE,
        "Promotional Scheme Price Discount"
    );
    assert_eq!(PromotionalSchemePriceDiscount::MODULE, "Accounts");
    assert_eq!(PromotionalSchemePriceDiscount::FIELD_ORDER.len(), 24);
    assert_eq!(PromotionalSchemePriceDiscount::FIELD_ORDER[0], "disable");
    assert_eq!(
        PromotionalSchemePriceDiscount::FIELD_ORDER[23],
        "apply_discount_on_rate"
    );
    assert!(PromotionalSchemePriceDiscount::IS_TABLE);
    assert!(PromotionalSchemePriceDiscount::EDITABLE_GRID);

    let fields = PromotionalSchemePriceDiscount::fields();
    assert_eq!(fields.len(), 24);
    assert!(fields.contains(&FieldSpec::check("disable", "Disable").default("0")));
    assert!(
        fields.contains(&FieldSpec::small_text("rule_description", "Rule Description").required())
    );
    assert!(fields.contains(
        &FieldSpec::float("min_qty", "Min Qty")
            .default("0")
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::select("rate_or_discount", "Discount Type")
            .options("\nRate\nDiscount Percentage\nDiscount Amount")
            .default("Discount Percentage")
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::check("apply_discount_on_rate", "Apply Discount on Rate")
            .default("0")
            .depends_on("eval:in_list(['Discount Percentage', 'Discount Amount'], doc.rate_or_discount) && doc.apply_multiple_pricing_rules")
    ));
}

#[test]
fn promotional_scheme_price_discount_preserves_pass_controller_behavior() {
    let blank = PromotionalSchemePriceDiscount::default();
    assert_eq!(blank.rule_description, None);
    assert_eq!(blank.min_qty, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PromotionalSchemePriceDiscount::new("Test");
    assert_eq!(row.rule_description.as_deref(), Some("Test"));
    assert_eq!(row.doctype(), "Promotional Scheme Price Discount");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
