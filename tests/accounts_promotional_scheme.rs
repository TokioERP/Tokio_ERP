use tokio_erp::erpnext::accounts::doctype::promotional_scheme::promotional_scheme::{
    get_args_for_pricing_rule, get_pricing_rules, raise_for_transaction_exists, AppliedItem,
    PricingRuleDraft, PromotionalScheme, PromotionalSchemeProductDiscount,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn promotional_scheme_matches_erpnext_core_metadata() {
    assert_eq!(PromotionalScheme::DOCTYPE, "Promotional Scheme");
    assert_eq!(PromotionalScheme::MODULE, "Accounts");
    assert_eq!(PromotionalScheme::AUTONAME, "Prompt");
    assert_eq!(PromotionalScheme::FIELD_ORDER.len(), 37);
    assert_eq!(PromotionalScheme::FIELD_ORDER[0], "section_break_1");
    assert_eq!(PromotionalScheme::FIELD_ORDER[36], "product_discount_slabs");
    assert!(PromotionalScheme::ALLOW_RENAME);
    assert!(PromotionalScheme::EDITABLE_GRID);
    assert!(PromotionalScheme::TRACK_CHANGES);

    let fields = PromotionalScheme::fields();
    assert_eq!(fields.len(), 37);
    assert!(fields.contains(
        &FieldSpec::select("apply_on", "Apply On")
            .options("\nItem Code\nItem Group\nBrand\nTransaction")
            .default("Item Code")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::table_multiselect("customer", "Customer")
            .options("Customer Item")
            .depends_on("eval:doc.applicable_for=='Customer'")
    ));
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::table("price_discount_slabs", "Promotional Scheme Price Discount")
            .options("Promotional Scheme Price Discount")
    ));
}

#[test]
fn promotional_scheme_validate_matches_erpnext_errors() {
    let mut doc = PromotionalScheme::new("_Test Scheme", "_Test Company");
    doc.selling = false;
    assert_eq!(
        doc.validate(),
        Err("Either 'Selling' or 'Buying' must be selected".to_string())
    );

    let mut doc = PromotionalScheme::new("_Test Scheme", "_Test Company");
    doc.price_discount_slabs.clear();
    assert_eq!(
        doc.validate(),
        Err("Price or product discount slabs are required".to_string())
    );

    let mut doc = PromotionalScheme::new("_Test Scheme", "_Test Company");
    doc.applicable_for = Some("Customer".to_string());
    assert_eq!(
        doc.validate(),
        Err("The field Customer is required".to_string())
    );

    let mut doc = PromotionalScheme::new("_Test Scheme", "_Test Company");
    doc.price_discount_slabs.clear();
    doc.product_discount_slabs = vec![PromotionalSchemeProductDiscount {
        name: "PSPD-1".to_string(),
        rule_description: Some("12+1".to_string()),
        min_qty: Some(12),
        free_item: Some("_Test Item 2".to_string()),
        free_qty: Some(1),
        is_recursive: true,
        recurse_for: Some(12),
        ..Default::default()
    }];
    doc.mixed_conditions = true;
    assert_eq!(
        doc.validate(),
        Err("Recursive Discounts with Mixed condition is not supported by the system".to_string())
    );
}

#[test]
fn promotional_scheme_generates_customer_price_rules_like_erpnext() {
    let mut doc = PromotionalScheme::new("_Test Scheme", "_Test Company");
    doc.items = vec![AppliedItem::new("_Test Item", Some("Nos"))];
    doc.applicable_for = Some("Customer".to_string());
    doc.customer = vec!["_Test Customer".to_string(), "_Test Customer 2".to_string()];
    doc.price_discount_slabs[0].name = "SLAB-1".to_string();
    doc.price_discount_slabs[0].min_qty = Some(6);
    doc.price_discount_slabs[0].min_amount = Some(10);
    doc.price_discount_slabs[0].max_amount = Some(1000);
    doc.price_discount_slabs[0].discount_percentage = Some(20);
    doc.price_discount_slabs[0].rule_description = Some("Test".to_string());

    let args = get_args_for_pricing_rule(&doc);
    assert_eq!(args.applicable_for.as_deref(), Some("Customer"));
    assert_eq!(args.customer, vec!["_Test Customer", "_Test Customer 2"]);

    let rules = get_pricing_rules(&doc);
    assert_eq!(rules.len(), 2);
    assert_eq!(
        rules[0],
        PricingRuleDraft {
            title: "_Test Scheme".to_string(),
            promotional_scheme: "_Test Scheme".to_string(),
            promotional_scheme_id: "SLAB-1".to_string(),
            price_or_product_discount: "Price".to_string(),
            apply_on: "Item Code".to_string(),
            applicable_for: Some("Customer".to_string()),
            customer: Some("_Test Customer".to_string()),
            company: "_Test Company".to_string(),
            min_qty: Some(6),
            min_amt: Some(10),
            max_amt: Some(1000),
            discount_percentage: Some(20),
            rule_description: Some("Test".to_string()),
            items: vec![AppliedItem::new("_Test Item", Some("Nos"))],
            ..Default::default()
        }
    );
    assert_eq!(rules[1].customer.as_deref(), Some("_Test Customer 2"));
}

#[test]
fn promotional_scheme_generates_product_rules_and_delete_plan() {
    let mut doc = PromotionalScheme::new("_Test Scheme", "_Test Company");
    doc.price_discount_slabs.clear();
    doc.product_discount_slabs = vec![PromotionalSchemeProductDiscount {
        name: "PRODUCT-1".to_string(),
        rule_description: Some("12+1".to_string()),
        min_qty: Some(12),
        free_item: Some("_Test Item 2".to_string()),
        free_qty: Some(1),
        is_recursive: true,
        recurse_for: Some(12),
        ..Default::default()
    }];

    let rules = doc.update_pricing_rules(&[]);
    assert_eq!(rules.created_count, 1);
    assert_eq!(rules.rules[0].price_or_product_discount, "Product");
    assert_eq!(rules.rules[0].free_item.as_deref(), Some("_Test Item 2"));
    assert_eq!(rules.rules[0].free_qty, Some(1));
    assert_eq!(rules.rules[0].recurse_for, Some(12));
    assert_eq!(doc.on_trash(&["PR-0001".to_string()]), vec!["PR-0001"]);
    assert_eq!(
        raise_for_transaction_exists("_Test Scheme"),
        "You can't change the Applicable For because transactions are present against the Promotional Scheme _Test Scheme. Kindly disable this Promotional Scheme and create new for new Applicable For."
    );
    assert_eq!(doc.custom_hooks(), ["validate", "on_update", "on_trash"]);
}
