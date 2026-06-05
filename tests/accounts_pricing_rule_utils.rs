use tokio_erp::erpnext::accounts::doctype::pricing_rule::utils::{
    apply_multiple_pricing_rules, filter_pricing_rule_based_on_condition, filter_pricing_rules,
    filter_pricing_rules_for_qty_amount, get_applied_pricing_rules, get_other_conditions,
    get_product_discount_rule, sorted_by_priority, update_coupon_code_count, validate_coupon_code,
    validate_quantity_and_amount_for_suggestion, CouponCode, CouponCodeError, FreeItemData,
    MultiplePricingRuleConflict, PricingRuleCandidate, PricingRuleFilterArgs,
    PricingRuleFilterOutcome, PricingRuleOtherConditions, TreeConditionInput,
};

fn candidate(name: &str, priority: i32) -> PricingRuleCandidate {
    PricingRuleCandidate {
        name: name.to_string(),
        priority: Some(priority),
        apply_multiple_pricing_rules: true,
        currency: Some("USD".to_string()),
        rate_or_discount: Some("Discount Percentage".to_string()),
        min_qty: 1.0,
        max_qty: 10.0,
        min_amt: 0.0,
        max_amt: 500.0,
        title: Some("Seasonal".to_string()),
        ..PricingRuleCandidate::default()
    }
}

#[test]
fn pricing_rule_utils_priority_condition_and_apply_multiple_match_erpnext() {
    let mut first = candidate("PRLE-0001", 2);
    let mut second = candidate("PRLE-0002", 1);
    second.apply_multiple_pricing_rules = false;
    assert!(!apply_multiple_pricing_rules(&[
        first.clone(),
        second.clone()
    ]));

    second.apply_multiple_pricing_rules = true;
    first.condition_passes = Some(false);
    second.condition_passes = Some(true);
    assert_eq!(
        filter_pricing_rule_based_on_condition(&[first, second.clone()], true),
        vec![second.clone()]
    );

    let mut no_priority = second.clone();
    no_priority.priority = None;
    assert_eq!(
        sorted_by_priority(&[second, no_priority])
            .into_iter()
            .map(|rule| rule.name)
            .collect::<Vec<_>>(),
        vec!["PRLE-0002".to_string(), "PRLE-0002".to_string()]
    );
}

#[test]
fn pricing_rule_utils_conditions_and_qty_amount_filter_match_erpnext() {
    let tree = TreeConditionInput {
        parenttype: "Item Group",
        field: "item_group",
        table: "`tabPricing Rule Item Group`",
        value: Some("Products".to_string()),
        parent_groups: vec!["Products".to_string(), "All Item Groups".to_string()],
        allow_blank: true,
    };
    assert_eq!(
        tree.condition(),
        "ifnull(`tabPricing Rule Item Group`.item_group, '') in ('Products', 'All Item Groups', '')"
    );

    let cond = get_other_conditions(&PricingRuleOtherConditions {
        company: Some("_Test Company".to_string()),
        customer: None,
        supplier: None,
        campaign: None,
        sales_partner: None,
        customer_group_condition: Some(
            "ifnull(`tabPricing Rule`.customer_group, '') = ''".to_string(),
        ),
        territory_condition: None,
        supplier_group_condition: None,
        transaction_date: Some("2026-05-01".to_string()),
        doctype: "Sales Invoice".to_string(),
    });
    assert!(cond
        .sql
        .contains("ifnull(`tabPricing Rule`.company, '') in (%(company)s, '')"));
    assert!(cond
        .sql
        .contains("ifnull(`tabPricing Rule`.selling, 0) = 1"));
    assert_eq!(cond.values.get("company").unwrap(), "_Test Company");
    assert_eq!(cond.values.get("transaction_date").unwrap(), "2026-05-01");

    let rules = filter_pricing_rules_for_qty_amount(
        5.0,
        100.0,
        &[
            candidate("KEEP", 1),
            PricingRuleCandidate {
                name: "DROP".to_string(),
                min_qty: 6.0,
                ..candidate("DROP", 1)
            },
        ],
        Some("Nos"),
    );
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].name, "KEEP");
}

#[test]
fn pricing_rule_utils_filter_conflict_and_suggestion_match_erpnext() {
    let args = PricingRuleFilterArgs {
        stock_qty: 5.0,
        price_list_rate: 20.0,
        qty: 5.0,
        item_code: Some("ITEM-001".to_string()),
        transaction_type: Some("selling".to_string()),
        currency: Some("USD".to_string()),
        price_list: Some("Standard Selling".to_string()),
        ..PricingRuleFilterArgs::default()
    };
    assert_eq!(
        filter_pricing_rules(&args, &[candidate("LOW", 1), candidate("HIGH", 2)]).unwrap(),
        PricingRuleFilterOutcome::Selected(candidate("HIGH", 2))
    );

    let conflict = filter_pricing_rules(
        &PricingRuleFilterArgs {
            for_shopping_cart: false,
            ..args.clone()
        },
        &[candidate("A", 1), candidate("B", 1)],
    )
    .unwrap_err();
    assert_eq!(
        conflict,
        MultiplePricingRuleConflict(
            "Multiple Price Rules exists with same criteria, please resolve conflict by assigning priority. Price Rules: A\nB"
                .to_string()
        )
    );

    let mut suggestion = candidate("SUGGEST", 1);
    suggestion.min_qty = 10.0;
    suggestion.threshold_percentage = 50.0;
    assert_eq!(
        validate_quantity_and_amount_for_suggestion(&suggestion, 6.0, 0.0, "ITEM-001", "selling"),
        Some("If you sale 10 quantities of the item <b>ITEM-001</b>, the scheme <b>Seasonal</b> will be applied on the item.".to_string())
    );
}

#[test]
fn pricing_rule_utils_applied_rules_and_product_discount_match_erpnext() {
    assert_eq!(
        get_applied_pricing_rules("[\"PRLE-1\",\"PRLE-2\"]"),
        vec!["PRLE-1".to_string(), "PRLE-2".to_string()]
    );
    assert_eq!(
        get_applied_pricing_rules("PRLE-1,PRLE-2"),
        vec!["PRLE-1".to_string(), "PRLE-2".to_string()]
    );

    let mut rule = candidate("FREE-RULE", 1);
    rule.free_item = Some("FREE-ITEM".to_string());
    rule.free_qty = 2.0;
    rule.free_item_rate = 0.0;
    rule.free_item_uom = Some("Nos".to_string());
    assert_eq!(
        get_product_discount_rule(&rule, "Sales Order", 3.0, 1.0).unwrap(),
        Some(FreeItemData {
            item_code: "FREE-ITEM".to_string(),
            qty: 2.0,
            pricing_rules: "FREE-RULE".to_string(),
            rate: 0.0,
            price_list_rate: 0.0,
            is_free_item: true,
            uom: Some("Nos".to_string()),
            schedule_date: None,
            delivery_date: Some("today".to_string()),
        })
    );

    rule.same_item = true;
    rule.apply_on = Some("Item Code".to_string());
    rule.free_item = None;
    assert_eq!(
        get_product_discount_rule(&rule, "Purchase Order", 3.0, 1.0)
            .unwrap()
            .unwrap()
            .schedule_date,
        Some("today".to_string())
    );
}

#[test]
fn pricing_rule_utils_coupon_validation_and_count_match_erpnext() {
    let coupon = CouponCode {
        coupon_code: "NEWYEAR".to_string(),
        valid_from: Some("2026-06-06".to_string()),
        valid_upto: None,
        maximum_use: 0,
        used: 0,
    };
    assert_eq!(
        validate_coupon_code(&coupon, "2026-06-05").unwrap_err(),
        CouponCodeError::Validation(
            "Sorry, this coupon code's validity has not started".to_string()
        )
    );

    let mut exhausted = CouponCode {
        valid_from: None,
        valid_upto: None,
        maximum_use: 2,
        used: 2,
        ..coupon
    };
    assert_eq!(
        update_coupon_code_count(&mut exhausted, "used").unwrap_err(),
        CouponCodeError::Validation(
            "NEWYEAR Coupon used are 2. Allowed quantity is exhausted".to_string()
        )
    );
    update_coupon_code_count(&mut exhausted, "cancelled").unwrap();
    assert_eq!(exhausted.used, 1);
}
