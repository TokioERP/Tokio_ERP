use tokio_erp::erpnext::accounts::doctype::pricing_rule::pricing_rule::{
    apply_price_discount_rule, apply_pricing_rule_plan, get_item_uoms_plan,
    get_pricing_rule_details, remove_pricing_rule_for_item, remove_pricing_rules,
    set_transaction_type, update_args_for_pricing_rule, update_pricing_rule_uom,
    ApplyPricingRulePlan, ItemUomQueryPlan, PricingItemDetails, PricingRule, PricingRuleArgs,
    PricingRuleChild, PricingRuleDetail, PricingRuleError, PricingRuleRemovalInput,
    PricingRuleRemovalState,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_rule() -> PricingRule {
    PricingRule {
        title: Some("Seasonal discount".to_string()),
        apply_on: "Item Code".to_string(),
        price_or_product_discount: "Price".to_string(),
        selling: true,
        currency: Some("USD".to_string()),
        rate_or_discount: Some("Discount Percentage".to_string()),
        discount_percentage: 10.0,
        items: vec![PricingRuleChild {
            item_code: Some("ITEM-001".to_string()),
            uom: Some("Nos".to_string()),
            ..PricingRuleChild::default()
        }],
        ..PricingRule::default()
    }
}

#[test]
fn pricing_rule_metadata_matches_erpnext_json() {
    assert_eq!(PricingRule::DOCTYPE, "Pricing Rule");
    assert_eq!(PricingRule::MODULE, "Accounts");
    assert_eq!(PricingRule::AUTONAME, "naming_series:");
    assert_eq!(PricingRule::FIELD_ORDER.len(), 88);
    assert_eq!(PricingRule::FIELD_ORDER[0], "applicability_section");
    assert_eq!(PricingRule::FIELD_ORDER[87], "promotional_scheme");
    assert!(PricingRule::ALLOW_IMPORT);
    assert!(PricingRule::ALLOW_RENAME);

    let doc = PricingRule::default();
    assert_eq!(doc.doctype(), "Pricing Rule");
    assert_eq!(doc.module(), "Accounts");
    assert!(PricingRule::fields().contains(
        &FieldSpec::select("apply_on", "Apply On")
            .options("Item Code\nItem Group\nBrand\nTransaction")
            .default("Item Code")
            .required()
            .in_list_view()
            .in_standard_filter()
    ));
    assert!(PricingRule::fields().contains(
        &FieldSpec::table("items", "Apply Rule On Item Code")
            .options("Pricing Rule Item Code")
            .depends_on("eval:doc.apply_on == 'Item Code'")
    ));
}

#[test]
fn pricing_rule_validate_matches_mandatory_applicability_range_and_recursion_rules() {
    let mut rule = base_rule();
    rule.has_priority = true;
    assert_eq!(
        rule.validate().unwrap_err(),
        PricingRuleError::Mandatory("Priority is mandatory".to_string())
    );

    rule = base_rule();
    rule.items.push(PricingRuleChild {
        item_code: Some("ITEM-001".to_string()),
        ..PricingRuleChild::default()
    });
    assert_eq!(
        rule.validate().unwrap_err(),
        PricingRuleError::Validation("Duplicate Item Code found in the table".to_string())
    );

    rule = base_rule();
    rule.selling = false;
    rule.buying = false;
    assert_eq!(
        rule.validate().unwrap_err(),
        PricingRuleError::Validation(
            "At least one of the Selling or Buying must be selected".to_string()
        )
    );

    rule = base_rule();
    rule.min_qty = 10.0;
    rule.max_qty = 5.0;
    assert_eq!(
        rule.validate().unwrap_err(),
        PricingRuleError::Validation("Min Qty can not be greater than Max Qty".to_string())
    );

    rule = base_rule();
    rule.price_or_product_discount = "Product".to_string();
    rule.free_item = Some("FREE-ITEM".to_string());
    rule.is_recursive = true;
    rule.min_qty = 2.0;
    rule.apply_recursion_over = 3.0;
    assert_eq!(
        rule.validate().unwrap_err(),
        PricingRuleError::Validation("Min Qty should be greater than Recurse Over Qty".to_string())
    );
}

#[test]
fn pricing_rule_cleanup_rate_dates_condition_and_currency_match_erpnext() {
    let mut rule = base_rule();
    rule.priority = Some("3".to_string());
    rule.has_priority = false;
    rule.apply_rule_on_other = Some("Item Group".to_string());
    rule.other_item_code = Some("ITEM-OTHER".to_string());
    rule.other_item_group = Some("GROUP-OTHER".to_string());
    rule.other_brand = Some("BRAND-OTHER".to_string());
    rule.customer = Some("CUST-001".to_string());
    rule.applicable_for = Some("Customer".to_string());
    rule.margin_type = None;
    rule.validate().unwrap();

    assert!(rule.has_priority);
    assert_eq!(rule.margin_rate_or_amount, 0.0);
    assert_eq!(rule.other_item_code, None);
    assert_eq!(rule.other_item_group.as_deref(), Some("GROUP-OTHER"));
    assert_eq!(rule.other_brand, None);

    rule.rate = -1.0;
    assert_eq!(
        rule.validate_rate_or_discount().unwrap_err(),
        PricingRuleError::Validation("Rate can not be negative".to_string())
    );

    let mut cumulative = base_rule();
    cumulative.is_cumulative = true;
    assert_eq!(
        cumulative.validate_dates().unwrap_err(),
        PricingRuleError::Validation(
            "Valid from and valid upto fields are mandatory for the cumulative".to_string()
        )
    );

    let mut currency = base_rule();
    currency.currency = Some("USD".to_string());
    currency.for_price_list = Some("Standard Selling".to_string());
    assert_eq!(
        currency
            .validate_price_list_with_currency(Some("EUR"))
            .unwrap_err(),
        PricingRuleError::Validation(
            "Currency should be same as Price List Currency: EUR".to_string()
        )
    );

    let mut condition = base_rule();
    condition.condition = Some("item_code = 'ITEM-001'".to_string());
    assert_eq!(
        condition.validate_condition().unwrap_err(),
        PricingRuleError::Validation("Invalid condition expression".to_string())
    );
}

#[test]
fn pricing_rule_apply_and_remove_price_discount_match_erpnext() {
    let mut rule = base_rule();
    rule.name = Some("PRLE-0001".to_string());
    rule.margin_type = Some("Percentage".to_string());
    rule.margin_rate_or_amount = 5.0;
    rule.apply_multiple_pricing_rules = true;
    rule.apply_discount_on_rate = true;

    let args = PricingRuleArgs {
        doctype: "Sales Invoice".to_string(),
        item_code: Some("ITEM-001".to_string()),
        currency: Some("USD".to_string()),
        price_list_rate: 100.0,
        conversion_factor: 2.0,
        uom: Some("Box".to_string()),
        ..PricingRuleArgs::default()
    };
    let mut details = PricingItemDetails {
        discount_percentage: 20.0,
        margin_rate_or_amount: Some(2.0),
        ..PricingItemDetails::default()
    };

    apply_price_discount_rule(&rule, &mut details, &args);
    assert_eq!(
        details.pricing_rule_for.as_deref(),
        Some("Discount Percentage")
    );
    assert!(details.has_margin);
    assert_eq!(details.margin_rate_or_amount, Some(7.0));
    assert_eq!(details.discount_percentage, 28.0);

    rule.rate_or_discount = Some("Rate".to_string());
    rule.rate = 25.0;
    let mut rate_details = PricingItemDetails::default();
    apply_price_discount_rule(&rule, &mut rate_details, &args);
    assert_eq!(rate_details.price_list_rate, Some(50.0));
    assert_eq!(rate_details.discount_percentage, 0.0);

    let removed = remove_pricing_rule_for_item(
        &[PricingRuleRemovalInput {
            pricing_rule: rule.clone(),
            exists: true,
        }],
        PricingRuleRemovalState {
            discount_percentage: 15.0,
            discount_amount: 8.0,
            margin_type: Some("Percentage".to_string()),
            margin_rate_or_amount: Some(4.0),
            pricing_rules: "PRLE-0001".to_string(),
            ..PricingRuleRemovalState::default()
        },
        Some("ITEM-001"),
        Some(99.0),
    );
    assert!(removed.pricing_rule_removed);
    assert_eq!(removed.pricing_rules, "");
    assert_eq!(removed.margin_type, None);
    assert_eq!(removed.margin_rate_or_amount, Some(0.0));
}

#[test]
fn pricing_rule_transaction_type_and_args_update_match_erpnext() {
    let mut sales_args = PricingRuleArgs {
        doctype: "Sales Invoice".to_string(),
        customer: Some("CUST-001".to_string()),
        item_code: Some("ITEM-001".to_string()),
        ..PricingRuleArgs::default()
    };
    set_transaction_type(&mut sales_args);
    assert_eq!(sales_args.transaction_type.as_deref(), Some("selling"));

    update_args_for_pricing_rule(
        &mut sales_args,
        Some(("Products".to_string(), "Brand A".to_string())),
        Some(("Retail".to_string(), "Uzbekistan".to_string())),
        None,
    )
    .unwrap();
    assert_eq!(sales_args.item_group.as_deref(), Some("Products"));
    assert_eq!(sales_args.brand.as_deref(), Some("Brand A"));
    assert_eq!(sales_args.customer_group.as_deref(), Some("Retail"));
    assert_eq!(sales_args.territory.as_deref(), Some("Uzbekistan"));
    assert_eq!(sales_args.supplier, None);

    let mut buying_args = PricingRuleArgs {
        doctype: "Purchase Invoice".to_string(),
        supplier: Some("SUP-001".to_string()),
        item_code: Some("ITEM-002".to_string()),
        ..PricingRuleArgs::default()
    };
    set_transaction_type(&mut buying_args);
    update_args_for_pricing_rule(
        &mut buying_args,
        Some(("Raw".to_string(), "".to_string())),
        None,
        Some("Local Suppliers".to_string()),
    )
    .unwrap();
    assert_eq!(buying_args.transaction_type.as_deref(), Some("buying"));
    assert_eq!(
        buying_args.supplier_group.as_deref(),
        Some("Local Suppliers")
    );
    assert_eq!(buying_args.customer, None);
}

#[test]
fn pricing_rule_remaining_helpers_match_erpnext_orchestration_shapes() {
    let mut rule = base_rule();
    rule.name = Some("PRLE-0001".to_string());
    rule.margin_type = Some("Amount".to_string());
    rule.items[0].uom = Some("Box".to_string());
    assert_eq!(
        rule.validate_max_discount(&[("ITEM-001".to_string(), 5.0)])
            .unwrap_err(),
        PricingRuleError::Validation("Max discount allowed for item: ITEM-001 is 5%".to_string())
    );
    rule.rate_or_discount = Some("Discount Amount".to_string());

    let args = PricingRuleArgs {
        doctype: "Sales Invoice".to_string(),
        name: Some("SINV-0001".to_string()),
        parent: Some("SINV-0001".to_string()),
        parenttype: Some("Sales Invoice".to_string()),
        child_docname: Some("SINV-ITEM-1".to_string()),
        item_code: Some("ITEM-001".to_string()),
        price_list_rate: 100.0,
        ..PricingRuleArgs::default()
    };
    update_pricing_rule_uom(&mut rule, &args);
    assert_eq!(rule.selected_uom.as_deref(), Some("Box"));
    assert_eq!(
        get_pricing_rule_details(&args, &rule),
        PricingRuleDetail {
            pricing_rule: "PRLE-0001".to_string(),
            rate_or_discount: Some("Discount Amount".to_string()),
            margin_type: Some("Amount".to_string()),
            item_code: Some("ITEM-001".to_string()),
            child_docname: Some("SINV-ITEM-1".to_string()),
        }
    );

    assert_eq!(
        apply_pricing_rule_plan(&[args.clone()], "Sales Invoice"),
        ApplyPricingRulePlan {
            skip: false,
            item_codes: vec!["ITEM-001".to_string()],
            transaction_type: "selling",
        }
    );
    assert_eq!(
        apply_pricing_rule_plan(&[args], "Material Request"),
        ApplyPricingRulePlan {
            skip: true,
            item_codes: vec!["ITEM-001".to_string()],
            transaction_type: "buying",
        }
    );

    let removed = remove_pricing_rules(vec![PricingRuleRemovalState {
        pricing_rules: "PRLE-0001".to_string(),
        item_code: Some("ITEM-001".to_string()),
        price_list_rate: Some(88.0),
        ..PricingRuleRemovalState::default()
    }]);
    assert_eq!(removed.len(), 1);
    assert!(removed[0].pricing_rule_removed);

    assert_eq!(
        get_item_uoms_plan(
            "Kg",
            "Item Group",
            "Products",
            vec!["ITEM-001".to_string(), "ITEM-002".to_string()]
        ),
        ItemUomQueryPlan {
            parent_items: vec!["ITEM-001".to_string(), "ITEM-002".to_string()],
            uom_like: "Kg%".to_string(),
            fields: vec!["uom"],
            distinct: true,
        }
    );
}
