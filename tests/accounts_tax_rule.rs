use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::doctype::tax_rule::tax_rule::{
    get_tax_template, tax_rule_js_hooks, TaxRule, TaxRuleError, TaxRuleLookupArgs,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn tax_rule_matches_erpnext_metadata_and_js_hooks() {
    assert_eq!(TaxRule::DOCTYPE, "Tax Rule");
    assert_eq!(TaxRule::MODULE, "Accounts");
    assert_eq!(TaxRule::AUTONAME, "ACC-TAX-RULE-.YYYY.-.#####");
    assert_eq!(TaxRule::FIELD_ORDER.len(), 32);
    assert_eq!(TaxRule::FIELD_ORDER[0], "tax_type");
    assert_eq!(TaxRule::FIELD_ORDER[31], "company");
    assert!(TaxRule::ALLOW_IMPORT);
    assert!(TaxRule::ALLOW_RENAME);
    let doc = TaxRule::new_sales("_Test Sales Taxes and Charges Template - _TC");
    assert_eq!(doc.doctype(), "Tax Rule");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), ["validate"]);
    assert_eq!(
        tax_rule_js_hooks(),
        [
            (
                "customer",
                "erpnext.accounts.doctype.tax_rule.tax_rule.get_party_details"
            ),
            (
                "supplier",
                "erpnext.accounts.doctype.tax_rule.tax_rule.get_party_details"
            ),
        ]
    );

    let fields = TaxRule::fields();
    assert_eq!(fields.len(), 32);
    assert!(fields.contains(
        &FieldSpec::select("tax_type", "Tax Type")
            .options("Sales\nPurchase")
            .default("Sales")
            .in_list_view()
            .in_standard_filter()
    ));
    assert!(fields.contains(
        &FieldSpec::link("sales_tax_template", "Sales Tax Template")
            .options("Sales Taxes and Charges Template")
            .depends_on("eval:doc.tax_type==\"Sales\"")
    ));
    assert!(fields.contains(
        &FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .fetch_from("customer.customer_group")
            .depends_on("eval:doc.tax_type==\"Sales\"")
    ));
    assert!(fields.contains(&FieldSpec::int("priority", "Priority").default("1")));
}

#[test]
fn tax_rule_validate_tax_template_matches_erpnext_clearing_and_mandatory_rules() {
    let mut sales = TaxRule::new_sales("_Test Sales Taxes and Charges Template - _TC");
    sales.purchase_tax_template = Some("Purchase Template".to_string());
    sales.supplier = Some("_Test Supplier".to_string());
    sales.supplier_group = Some("Services".to_string());
    sales.customer = Some("_Test Customer".to_string());
    sales.customer_group = Some("All Customer Groups".to_string());
    sales.validate_tax_template().unwrap();
    assert_eq!(sales.purchase_tax_template, None);
    assert_eq!(sales.supplier, None);
    assert_eq!(sales.supplier_group, None);
    assert_eq!(sales.customer_group, None);

    let mut purchase = TaxRule::new_purchase("_Test Purchase Taxes and Charges Template - _TC");
    purchase.sales_tax_template = Some("Sales Template".to_string());
    purchase.customer = Some("_Test Customer".to_string());
    purchase.customer_group = Some("All Customer Groups".to_string());
    purchase.supplier = Some("_Test Supplier".to_string());
    purchase.supplier_group = Some("Services".to_string());
    purchase.validate_tax_template().unwrap();
    assert_eq!(purchase.sales_tax_template, None);
    assert_eq!(purchase.customer, None);
    assert_eq!(purchase.customer_group, None);
    assert_eq!(purchase.supplier_group, None);

    let mut missing = TaxRule::default();
    missing.tax_type = "Sales".to_string();
    assert_eq!(
        missing.validate_tax_template().unwrap_err(),
        TaxRuleError::MandatoryTaxTemplate("Tax Template is mandatory.".to_string())
    );
}

#[test]
fn tax_rule_conflict_detection_matches_exact_filters_and_date_overlap() {
    let mut existing = TaxRule::new_sales("_Test Sales Taxes and Charges Template - _TC");
    existing.name = "TR-1".to_string();
    existing.customer = Some("_Test Customer".to_string());
    existing.from_date = Some("2015-01-01".to_string());
    existing.to_date = Some("2015-01-05".to_string());
    existing.priority = 1;

    let mut overlapping = TaxRule::new_sales("_Test Sales Taxes and Charges Template - _TC");
    overlapping.name = "TR-2".to_string();
    overlapping.customer = Some("_Test Customer".to_string());
    overlapping.from_date = Some("2015-01-03".to_string());
    overlapping.to_date = Some("2015-01-09".to_string());
    overlapping.priority = 1;

    assert_eq!(
        overlapping
            .validate_filters(&[existing.clone()])
            .unwrap_err(),
        TaxRuleError::ConflictingTaxRule("Tax Rule Conflicts with TR-1".to_string())
    );

    let mut non_overlapping = overlapping.clone();
    non_overlapping.to_date = Some("2013-01-01".to_string());
    non_overlapping.from_date = None;
    assert!(non_overlapping
        .validate_filters(&[existing.clone()])
        .is_ok());

    let mut different_priority = overlapping;
    different_priority.priority = 2;
    assert!(different_priority.validate_filters(&[existing]).is_ok());
}

#[test]
fn tax_rule_template_lookup_matches_tax_category_specificity_and_priority() {
    let rules = vec![
        {
            let mut rule = TaxRule::new_sales("_Test Sales Taxes and Charges Template - _TC");
            rule.name = "base".to_string();
            rule.customer = Some("_Test Customer".to_string());
            rule.priority = 1;
            rule
        },
        {
            let mut rule = TaxRule::new_sales("_Test Sales Taxes and Charges Template 1 - _TC");
            rule.name = "city-low".to_string();
            rule.customer = Some("_Test Customer".to_string());
            rule.billing_city = Some("Test City".to_string());
            rule.priority = 1;
            rule
        },
        {
            let mut rule = TaxRule::new_sales("_Test Sales Taxes and Charges Template 2 - _TC");
            rule.name = "city-high".to_string();
            rule.customer = Some("_Test Customer".to_string());
            rule.billing_city = Some("Test City".to_string());
            rule.priority = 2;
            rule
        },
        {
            let mut rule = TaxRule::new_sales("_Test Sales Taxes and Charges Template 3 - _TC");
            rule.name = "category".to_string();
            rule.customer = Some("_Test Customer".to_string());
            rule.tax_category = Some("_Test Tax Category 1".to_string());
            rule.priority = 1;
            rule
        },
    ];

    let args = TaxRuleLookupArgs::from_pairs([
        ("customer", "_Test Customer"),
        ("billing_city", "Test City"),
    ]);
    assert_eq!(
        get_tax_template(
            Some("2015-01-01"),
            &args,
            &rules,
            &BTreeSet::new(),
            &BTreeMap::new()
        ),
        Some("_Test Sales Taxes and Charges Template 2 - _TC".to_string())
    );

    let category_args = TaxRuleLookupArgs::from_pairs([
        ("customer", "_Test Customer"),
        ("tax_category", "_Test Tax Category 1"),
    ]);
    assert_eq!(
        get_tax_template(
            Some("2015-01-01"),
            &category_args,
            &rules,
            &BTreeSet::new(),
            &BTreeMap::new()
        ),
        Some("_Test Sales Taxes and Charges Template 3 - _TC".to_string())
    );

    let disabled = BTreeSet::from(["_Test Sales Taxes and Charges Template 3 - _TC".to_string()]);
    assert_eq!(
        get_tax_template(
            Some("2015-01-01"),
            &category_args,
            &rules,
            &disabled,
            &BTreeMap::new()
        ),
        None
    );
}

#[test]
fn tax_rule_customer_group_lookup_matches_parent_group_condition() {
    let mut parent_rule = TaxRule::new_sales("_Test Sales Taxes and Charges Template - _TC");
    parent_rule.customer_group = Some("All Customer Groups".to_string());
    parent_rule.from_date = Some("2015-01-01".to_string());

    let parents = BTreeMap::from([(
        "Commercial".to_string(),
        vec!["Commercial".to_string(), "All Customer Groups".to_string()],
    )]);
    let args = TaxRuleLookupArgs::from_pairs([
        ("customer_group", "Commercial"),
        ("use_for_shopping_cart", "1"),
    ]);

    assert_eq!(
        get_tax_template(
            Some("2015-01-01"),
            &args,
            &[parent_rule],
            &BTreeSet::new(),
            &parents
        ),
        Some("_Test Sales Taxes and Charges Template - _TC".to_string())
    );
}
