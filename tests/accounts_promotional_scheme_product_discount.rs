use tokio_erp::erpnext::accounts::doctype::promotional_scheme_product_discount::promotional_scheme_product_discount::PromotionalSchemeProductDiscount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn promotional_scheme_product_discount_matches_erpnext_metadata() {
    assert_eq!(
        PromotionalSchemeProductDiscount::DOCTYPE,
        "Promotional Scheme Product Discount"
    );
    assert_eq!(PromotionalSchemeProductDiscount::MODULE, "Accounts");
    assert_eq!(PromotionalSchemeProductDiscount::FIELD_ORDER.len(), 26);
    assert_eq!(PromotionalSchemeProductDiscount::FIELD_ORDER[0], "disable");
    assert_eq!(
        PromotionalSchemeProductDiscount::FIELD_ORDER[25],
        "apply_recursion_over"
    );
    assert!(PromotionalSchemeProductDiscount::IS_TABLE);
    assert!(PromotionalSchemeProductDiscount::EDITABLE_GRID);

    let fields = PromotionalSchemeProductDiscount::fields();
    assert_eq!(fields.len(), 26);
    assert!(fields.contains(&FieldSpec::check("disable", "Disable").default("0")));
    assert!(
        fields.contains(&FieldSpec::small_text("rule_description", "Rule Description").required())
    );
    assert!(fields.contains(
        &FieldSpec::check("same_item", "Same Item")
            .default("0")
            .depends_on("eval:!parent.mixed_conditions")
    ));
    assert!(fields.contains(
        &FieldSpec::link("free_item", "Item Code")
            .options("Item")
            .depends_on("eval:!doc.same_item || parent.mixed_conditions")
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::float("recurse_for", "Recurse Every (As Per Transaction UOM)")
            .default("0")
            .depends_on("is_recursive")
            .mandatory_depends_on("is_recursive")
    ));
}

#[test]
fn promotional_scheme_product_discount_preserves_pass_controller_behavior() {
    let blank = PromotionalSchemeProductDiscount::default();
    assert_eq!(blank.rule_description, None);
    assert_eq!(blank.free_item, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PromotionalSchemeProductDiscount::new("12+1");
    assert_eq!(row.rule_description.as_deref(), Some("12+1"));
    assert_eq!(row.doctype(), "Promotional Scheme Product Discount");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
