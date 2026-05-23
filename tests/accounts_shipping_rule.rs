use tokio_erp::erpnext::accounts::doctype::shipping_rule::shipping_rule::{
    shipping_rule_dashboard_groups, shipping_rule_js_hooks, ShippingRule, ShippingRuleApplication,
    ShippingRuleDocumentContext, ShippingRuleError, TaxChargeDraft,
};
use tokio_erp::erpnext::accounts::doctype::shipping_rule_condition::shipping_rule_condition::ShippingRuleCondition;
use tokio_erp::erpnext::accounts::doctype::shipping_rule_country::shipping_rule_country::ShippingRuleCountry;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn shipping_rule_child_tables_match_erpnext_metadata() {
    assert_eq!(ShippingRuleCondition::DOCTYPE, "Shipping Rule Condition");
    assert_eq!(ShippingRuleCondition::MODULE, "Accounts");
    assert_eq!(
        ShippingRuleCondition::FIELD_ORDER,
        ["from_value", "to_value", "shipping_amount"]
    );
    assert!(ShippingRuleCondition::IS_TABLE);
    assert!(ShippingRuleCondition::EDITABLE_GRID);
    assert_eq!(
        ShippingRuleCondition::fields(),
        vec![
            FieldSpec::float("from_value", "From Value")
                .required()
                .in_list_view(),
            FieldSpec::float("to_value", "To Value").in_list_view(),
            FieldSpec::currency("shipping_amount", "Shipping Amount")
                .options("Company:company:default_currency")
                .required()
                .in_list_view(),
        ]
    );

    assert_eq!(ShippingRuleCountry::DOCTYPE, "Shipping Rule Country");
    assert_eq!(ShippingRuleCountry::MODULE, "Accounts");
    assert_eq!(ShippingRuleCountry::FIELD_ORDER, ["country"]);
    assert!(ShippingRuleCountry::IS_TABLE);
    assert!(ShippingRuleCountry::EDITABLE_GRID);
    assert_eq!(
        ShippingRuleCountry::fields(),
        vec![FieldSpec::link("country", "Country")
            .options("Country")
            .required()
            .in_list_view()]
    );
}

#[test]
fn shipping_rule_matches_erpnext_metadata_and_static_hooks() {
    assert_eq!(ShippingRule::DOCTYPE, "Shipping Rule");
    assert_eq!(ShippingRule::MODULE, "Accounts");
    assert_eq!(ShippingRule::AUTONAME, "field:label");
    assert!(ShippingRule::ALLOW_IMPORT);
    assert_eq!(ShippingRule::FIELD_ORDER.len(), 20);
    assert_eq!(ShippingRule::FIELD_ORDER[0], "label");
    assert_eq!(ShippingRule::FIELD_ORDER[19], "countries");

    let fields = ShippingRule::fields();
    assert_eq!(fields.len(), 20);
    assert!(fields.contains(
        &FieldSpec::data("label", "Shipping Rule Label")
            .description("example: Next Day Shipping")
            .required()
            .unique()
    ));
    assert!(fields.contains(
        &FieldSpec::select("calculate_based_on", "Calculate Based On")
            .options("Fixed\nNet Total\nNet Weight")
            .default("Fixed")
            .in_list_view()
            .in_standard_filter()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("shipping_amount", "Shipping Amount")
            .depends_on("eval:doc.calculate_based_on==='Fixed'")
    ));
    assert!(fields.contains(
        &FieldSpec::table("conditions", "Shipping Rule Conditions")
            .options("Shipping Rule Condition")
    ));

    let rule = ShippingRule::new(
        "Fast",
        "Selling",
        "_Test Company",
        "Freight - TC",
        "Main - TC",
    );
    assert_eq!(rule.doctype(), "Shipping Rule");
    assert_eq!(rule.module(), "Accounts");
    assert_eq!(rule.custom_hooks(), ["validate"]);
    assert_eq!(
        shipping_rule_js_hooks(),
        [
            "onload",
            "company",
            "refresh",
            "calculate_based_on",
            "toggle_reqd"
        ]
    );
    assert_eq!(
        shipping_rule_dashboard_groups(),
        [
            ("Pre Sales", ["Quotation", "Supplier Quotation"].as_slice()),
            (
                "Sales",
                ["Sales Order", "Delivery Note", "Sales Invoice"].as_slice()
            ),
            (
                "Purchase",
                ["Purchase Invoice", "Purchase Order", "Purchase Receipt"].as_slice()
            ),
        ]
    );
}

#[test]
fn shipping_rule_validation_matches_erpnext_condition_rules() {
    let mut fixed = ShippingRule::new(
        "Fixed",
        "Selling",
        "_Test Company",
        "Freight - TC",
        "Main - TC",
    );
    fixed.conditions = vec![ShippingRuleCondition::new(0.0, 100.0, 50.0)];
    fixed.validate().unwrap();
    assert!(fixed.conditions.is_empty());

    let mut inverted = ShippingRule::new_net_total(
        "Tiered",
        "Selling",
        "_Test Company",
        "Freight - TC",
        "Main - TC",
    );
    inverted.conditions = vec![ShippingRuleCondition::new(101.0, 100.0, 50.0)];
    assert_eq!(
        inverted.validate().unwrap_err(),
        ShippingRuleError::FromGreaterThanTo(
            "From value must be less than to value in row 1".to_string()
        )
    );

    let mut open_ended = ShippingRule::new_net_total(
        "Open",
        "Selling",
        "_Test Company",
        "Freight - TC",
        "Main - TC",
    );
    open_ended.conditions = vec![
        ShippingRuleCondition::new(0.0, 0.0, 50.0),
        ShippingRuleCondition::new(101.0, 0.0, 100.0),
    ];
    assert_eq!(
        open_ended.validate().unwrap_err(),
        ShippingRuleError::ManyBlankToValues(
            "There can only be one Shipping Rule Condition with 0 or blank value for \"To Value\""
                .to_string()
        )
    );

    for (range_a, range_b) in [
        ((50.0, 150.0), (0.0, 100.0)),
        ((50.0, 150.0), (100.0, 200.0)),
        ((50.0, 150.0), (75.0, 125.0)),
        ((50.0, 150.0), (25.0, 175.0)),
        ((50.0, 150.0), (50.0, 150.0)),
    ] {
        let mut rule = ShippingRule::new_net_total(
            "Overlap",
            "Selling",
            "_Test Company",
            "Freight - TC",
            "Main - TC",
        );
        rule.conditions = vec![
            ShippingRuleCondition::new(range_a.0, range_a.1, 50.0),
            ShippingRuleCondition::new(range_b.0, range_b.1, 100.0),
        ];
        assert!(matches!(
            rule.validate().unwrap_err(),
            ShippingRuleError::OverlappingConditions(_)
        ));
    }
}

#[test]
fn shipping_rule_apply_matches_erpnext_amount_country_and_tax_table_logic() {
    let mut selling = ShippingRule::new_net_total(
        "Tiered",
        "Selling",
        "_Test Company",
        "Freight - TC",
        "Main - TC",
    );
    selling.conditions = vec![
        ShippingRuleCondition::new(0.0, 100.0, 50.0),
        ShippingRuleCondition::new(101.0, 200.0, 100.0),
        ShippingRuleCondition::new(201.0, 0.0, 200.0),
    ];
    selling.countries = vec![ShippingRuleCountry::new("United States")];
    selling.validate().unwrap();

    let context = ShippingRuleDocumentContext {
        has_shipping_address: true,
        shipping_country: Some("United States".to_string()),
        base_net_total: 150.0,
        total_net_weight: 0.0,
        currency: "USD".to_string(),
        company_currency: "USD".to_string(),
        conversion_rate: 1.0,
        taxes_options: "Sales Taxes and Charges".to_string(),
        has_stock_items: false,
        has_asset_items: false,
        existing_shipping_charge: false,
    };
    assert_eq!(
        selling.apply(&context).unwrap(),
        ShippingRuleApplication::Append(TaxChargeDraft {
            doctype: "Sales Taxes and Charges".to_string(),
            charge_type: "Actual".to_string(),
            account_head: "Freight - TC".to_string(),
            cost_center: "Main - TC".to_string(),
            tax_amount: 100.0,
            description: Some("Tiered".to_string()),
            category: None,
            add_deduct_tax: None,
        })
    );

    let mut foreign = context.clone();
    foreign.currency = "EUR".to_string();
    foreign.conversion_rate = 2.0;
    assert_eq!(selling.apply(&foreign).unwrap().tax_amount(), 50.0);

    let mut buying = ShippingRule::new_net_total(
        "Buy",
        "Buying",
        "_Test Company",
        "Freight - TC",
        "Main - TC",
    );
    buying.conditions = vec![ShippingRuleCondition::new(0.0, 1000.0, 75.0)];
    let buying_context = ShippingRuleDocumentContext {
        taxes_options: "Purchase Taxes and Charges".to_string(),
        has_stock_items: true,
        ..context
    };
    assert_eq!(
        buying.apply(&buying_context).unwrap(),
        ShippingRuleApplication::Append(TaxChargeDraft {
            doctype: "Purchase Taxes and Charges".to_string(),
            charge_type: "Actual".to_string(),
            account_head: "Freight - TC".to_string(),
            cost_center: "Main - TC".to_string(),
            tax_amount: 75.0,
            description: Some("Buy".to_string()),
            category: Some("Valuation and Total".to_string()),
            add_deduct_tax: Some("Add".to_string()),
        })
    );

    let mut wrong_table = buying_context.clone();
    wrong_table.taxes_options = "Sales Taxes and Charges".to_string();
    assert_eq!(
        buying.apply(&wrong_table).unwrap_err(),
        ShippingRuleError::WrongTaxTable("Shipping rule only applicable for Buying".to_string())
    );

    let mut missing_country = buying_context.clone();
    missing_country.has_shipping_address = true;
    missing_country.shipping_country = None;
    assert_eq!(
        selling.apply(&missing_country).unwrap_err(),
        ShippingRuleError::Country(
            "Shipping Address does not have country, which is required for this Shipping Rule"
                .to_string()
        )
    );
}
