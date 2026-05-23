use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionalScheme {
    pub name: String,
    pub apply_on: String,
    pub disable: bool,
    pub items: Vec<AppliedItem>,
    pub item_groups: Vec<AppliedItem>,
    pub brands: Vec<AppliedItem>,
    pub mixed_conditions: bool,
    pub is_cumulative: bool,
    pub apply_rule_on_other: Option<String>,
    pub other_item_code: Option<String>,
    pub other_item_group: Option<String>,
    pub other_brand: Option<String>,
    pub selling: bool,
    pub buying: bool,
    pub applicable_for: Option<String>,
    pub customer: Vec<String>,
    pub customer_group: Vec<String>,
    pub territory: Vec<String>,
    pub sales_partner: Vec<String>,
    pub campaign: Vec<String>,
    pub supplier: Vec<String>,
    pub supplier_group: Vec<String>,
    pub valid_from: Option<String>,
    pub valid_upto: Option<String>,
    pub company: String,
    pub currency: Option<String>,
    pub price_discount_slabs: Vec<PromotionalSchemePriceDiscount>,
    pub product_discount_slabs: Vec<PromotionalSchemeProductDiscount>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AppliedItem {
    pub value: String,
    pub uom: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PromotionalSchemePriceDiscount {
    pub name: String,
    pub disable: bool,
    pub min_qty: Option<i32>,
    pub max_qty: Option<i32>,
    pub min_amount: Option<i32>,
    pub max_amount: Option<i32>,
    pub priority: Option<i32>,
    pub warehouse: Option<String>,
    pub threshold_percentage: Option<i32>,
    pub rule_description: Option<String>,
    pub rate_or_discount: Option<String>,
    pub apply_discount_on: Option<String>,
    pub apply_discount_on_rate: Option<i32>,
    pub rate: Option<i32>,
    pub discount_amount: Option<i32>,
    pub discount_percentage: Option<i32>,
    pub validate_applied_rule: bool,
    pub apply_multiple_pricing_rules: bool,
    pub for_price_list: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PromotionalSchemeProductDiscount {
    pub name: String,
    pub disable: bool,
    pub min_qty: Option<i32>,
    pub max_qty: Option<i32>,
    pub min_amount: Option<i32>,
    pub max_amount: Option<i32>,
    pub priority: Option<i32>,
    pub warehouse: Option<String>,
    pub threshold_percentage: Option<i32>,
    pub rule_description: Option<String>,
    pub free_item: Option<String>,
    pub free_qty: Option<i32>,
    pub free_item_uom: Option<String>,
    pub free_item_rate: Option<i32>,
    pub same_item: bool,
    pub is_recursive: bool,
    pub recurse_for: Option<i32>,
    pub apply_recursion_over: Option<String>,
    pub apply_multiple_pricing_rules: bool,
    pub round_free_qty: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleArgs {
    pub promotional_scheme: String,
    pub apply_on: String,
    pub applicable_for: Option<String>,
    pub customer: Vec<String>,
    pub customer_group: Vec<String>,
    pub territory: Vec<String>,
    pub sales_partner: Vec<String>,
    pub campaign: Vec<String>,
    pub supplier: Vec<String>,
    pub supplier_group: Vec<String>,
    pub company: String,
    pub currency: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleDraft {
    pub name: Option<String>,
    pub title: String,
    pub promotional_scheme: String,
    pub promotional_scheme_id: String,
    pub price_or_product_discount: String,
    pub apply_on: String,
    pub applicable_for: Option<String>,
    pub customer: Option<String>,
    pub customer_group: Option<String>,
    pub territory: Option<String>,
    pub sales_partner: Option<String>,
    pub campaign: Option<String>,
    pub supplier: Option<String>,
    pub supplier_group: Option<String>,
    pub company: String,
    pub min_qty: Option<i32>,
    pub max_qty: Option<i32>,
    pub min_amt: Option<i32>,
    pub max_amt: Option<i32>,
    pub discount_percentage: Option<i32>,
    pub rule_description: Option<String>,
    pub free_item: Option<String>,
    pub free_qty: Option<i32>,
    pub is_recursive: bool,
    pub recurse_for: Option<i32>,
    pub disable: bool,
    pub items: Vec<AppliedItem>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExistingPricingRule {
    pub promotional_scheme_id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleUpdatePlan {
    pub rules: Vec<PricingRuleDraft>,
    pub created_count: usize,
    pub updated_names: Vec<String>,
}

impl AppliedItem {
    pub fn new(value: impl Into<String>, uom: Option<&str>) -> Self {
        Self {
            value: value.into(),
            uom: uom.map(str::to_string),
        }
    }
}

impl PromotionalScheme {
    pub const DOCTYPE: &'static str = "Promotional Scheme";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "Prompt";
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 37] = [
        "section_break_1",
        "apply_on",
        "disable",
        "column_break_3",
        "items",
        "item_groups",
        "brands",
        "mixed_conditions",
        "is_cumulative",
        "section_break_10",
        "apply_rule_on_other",
        "column_break_11",
        "other_item_code",
        "other_item_group",
        "other_brand",
        "section_break_8",
        "selling",
        "buying",
        "column_break_12",
        "applicable_for",
        "customer",
        "customer_group",
        "territory",
        "sales_partner",
        "campaign",
        "supplier",
        "supplier_group",
        "period_settings_section",
        "valid_from",
        "valid_upto",
        "column_break_26",
        "company",
        "currency",
        "section_break_14",
        "price_discount_slabs",
        "section_break_15",
        "product_discount_slabs",
    ];

    pub fn new(name: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            apply_on: "Item Code".to_string(),
            disable: false,
            items: Vec::new(),
            item_groups: Vec::new(),
            brands: Vec::new(),
            mixed_conditions: false,
            is_cumulative: false,
            apply_rule_on_other: None,
            other_item_code: None,
            other_item_group: None,
            other_brand: None,
            selling: true,
            buying: false,
            applicable_for: None,
            customer: Vec::new(),
            customer_group: Vec::new(),
            territory: Vec::new(),
            sales_partner: Vec::new(),
            campaign: Vec::new(),
            supplier: Vec::new(),
            supplier_group: Vec::new(),
            valid_from: Some("Today".to_string()),
            valid_upto: None,
            company: company.into(),
            currency: None,
            price_discount_slabs: vec![PromotionalSchemePriceDiscount::default()],
            product_discount_slabs: Vec::new(),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("section_break_1"),
            FieldSpec::select("apply_on", "Apply On")
                .options("\nItem Code\nItem Group\nBrand\nTransaction")
                .default("Item Code")
                .required()
                .in_list_view(),
            FieldSpec::check("disable", "Disable").default("0"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::table("items", "Pricing Rule Item Code")
                .options("Pricing Rule Item Code")
                .depends_on("eval:doc.apply_on == 'Item Code'"),
            FieldSpec::table("item_groups", "Pricing Rule Item Group")
                .options("Pricing Rule Item Group")
                .depends_on("eval:doc.apply_on == 'Item Group'"),
            FieldSpec::table("brands", "Pricing Rule Brand")
                .options("Pricing Rule Brand")
                .depends_on("eval:doc.apply_on == 'Brand'"),
            FieldSpec::check("mixed_conditions", "Mixed Conditions")
                .default("0")
                .depends_on("eval:doc.apply_on != 'Transaction'"),
            FieldSpec::check("is_cumulative", "Is Cumulative")
                .default("0")
                .depends_on("eval:doc.apply_on != 'Transaction'"),
            FieldSpec::section_break("section_break_10").label("Discount on Other Item"),
            FieldSpec::select("apply_rule_on_other", "Apply Rule On Other")
                .options("\nItem Code\nItem Group\nBrand"),
            FieldSpec::column_break("column_break_11"),
            FieldSpec::link("other_item_code", "Item Code")
                .options("Item")
                .depends_on("eval:doc.apply_rule_on_other == 'Item Code'"),
            FieldSpec::link("other_item_group", "Item Group")
                .options("Item Group")
                .depends_on("eval:doc.apply_rule_on_other == 'Item Group'"),
            FieldSpec::link("other_brand", "Brand")
                .options("Brand")
                .depends_on("eval:doc.apply_rule_on_other == 'Brand'"),
            FieldSpec::section_break("section_break_8").label("Party Information"),
            FieldSpec::check("selling", "Selling").default("0"),
            FieldSpec::check("buying", "Buying").default("0"),
            FieldSpec::column_break("column_break_12"),
            FieldSpec::select("applicable_for", "Applicable For")
                .options(
                    "\nCustomer\nCustomer Group\nTerritory\nSales Partner\nCampaign\nSupplier\nSupplier Group",
                )
                .depends_on("eval: doc.buying || doc.selling"),
            FieldSpec::table_multiselect("customer", "Customer")
                .options("Customer Item")
                .depends_on("eval:doc.applicable_for=='Customer'"),
            FieldSpec::table_multiselect("customer_group", "Customer Group")
                .options("Customer Group Item")
                .depends_on("eval:doc.applicable_for==\"Customer Group\""),
            FieldSpec::table_multiselect("territory", "Territory")
                .options("Territory Item")
                .depends_on("eval:doc.applicable_for==\"Territory\""),
            FieldSpec::table_multiselect("sales_partner", "Sales Partner")
                .options("Sales Partner Item")
                .depends_on("eval:doc.applicable_for==\"Sales Partner\""),
            FieldSpec::table_multiselect("campaign", "Campaign")
                .options("Campaign Item")
                .depends_on("eval:doc.applicable_for==\"Campaign\""),
            FieldSpec::table_multiselect("supplier", "Supplier")
                .options("Supplier Item")
                .depends_on("eval:doc.applicable_for=='Supplier'"),
            FieldSpec::table_multiselect("supplier_group", "Supplier Group")
                .options("Supplier Group Item")
                .depends_on("eval:doc.applicable_for==\"Supplier Group\""),
            FieldSpec::section_break("period_settings_section").label("Period Settings"),
            FieldSpec::date("valid_from", "Valid From").default("Today"),
            FieldSpec::date("valid_upto", "Valid Up To"),
            FieldSpec::column_break("column_break_26"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("currency", "Currency").options("Currency"),
            FieldSpec::section_break("section_break_14").label("Price Discount Slabs"),
            FieldSpec::table("price_discount_slabs", "Promotional Scheme Price Discount")
                .options("Promotional Scheme Price Discount"),
            FieldSpec::section_break("section_break_15").label("Product Discount Slabs"),
            FieldSpec::table(
                "product_discount_slabs",
                "Promotional Scheme Product Discount",
            )
            .options("Promotional Scheme Product Discount"),
        ]
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.selling && !self.buying {
            return Err("Either 'Selling' or 'Buying' must be selected".to_string());
        }
        if self.price_discount_slabs.is_empty() && self.product_discount_slabs.is_empty() {
            return Err("Price or product discount slabs are required".to_string());
        }
        self.validate_applicable_for()?;
        self.validate_mixed_with_recursion()
    }

    pub fn validate_applicable_for(&self) -> Result<(), String> {
        if let Some(applicable_for) = &self.applicable_for {
            if self.applicable_values(applicable_for).is_empty() {
                return Err(format!("The field {applicable_for} is required"));
            }
        }
        Ok(())
    }

    pub fn validate_mixed_with_recursion(&self) -> Result<(), String> {
        if self.mixed_conditions
            && self
                .product_discount_slabs
                .iter()
                .any(|slab| slab.is_recursive)
        {
            return Err(
                "Recursive Discounts with Mixed condition is not supported by the system"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub fn update_pricing_rules(&self, _existing: &[ExistingPricingRule]) -> PricingRuleUpdatePlan {
        let rules = get_pricing_rules(self);
        PricingRuleUpdatePlan {
            created_count: rules.iter().filter(|rule| rule.name.is_none()).count(),
            updated_names: rules.iter().filter_map(|rule| rule.name.clone()).collect(),
            rules,
        }
    }

    pub fn on_trash(&self, existing_pricing_rules: &[String]) -> Vec<String> {
        existing_pricing_rules.to_vec()
    }

    fn applicable_values(&self, applicable_for: &str) -> &[String] {
        match scrub(applicable_for).as_str() {
            "customer" => &self.customer,
            "customer_group" => &self.customer_group,
            "territory" => &self.territory,
            "sales_partner" => &self.sales_partner,
            "campaign" => &self.campaign,
            "supplier" => &self.supplier,
            "supplier_group" => &self.supplier_group,
            _ => &[],
        }
    }
}

impl DocumentController for PromotionalScheme {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_update", "on_trash"]
    }
}

pub fn get_args_for_pricing_rule(doc: &PromotionalScheme) -> PricingRuleArgs {
    PricingRuleArgs {
        promotional_scheme: doc.name.clone(),
        apply_on: doc.apply_on.clone(),
        applicable_for: doc.applicable_for.clone(),
        customer: doc.customer.clone(),
        customer_group: doc.customer_group.clone(),
        territory: doc.territory.clone(),
        sales_partner: doc.sales_partner.clone(),
        campaign: doc.campaign.clone(),
        supplier: doc.supplier.clone(),
        supplier_group: doc.supplier_group.clone(),
        company: doc.company.clone(),
        currency: doc.currency.clone(),
    }
}

pub fn get_pricing_rules(doc: &PromotionalScheme) -> Vec<PricingRuleDraft> {
    let mut rules = Vec::new();

    for slab in &doc.price_discount_slabs {
        extend_rules_for_slab(doc, &mut rules, SlabRef::Price(slab));
    }
    for slab in &doc.product_discount_slabs {
        extend_rules_for_slab(doc, &mut rules, SlabRef::Product(slab));
    }

    rules
}

pub fn raise_for_transaction_exists(name: &str) -> String {
    format!(
        "You can't change the Applicable For because transactions are present against the Promotional Scheme {name}. Kindly disable this Promotional Scheme and create new for new Applicable For."
    )
}

enum SlabRef<'a> {
    Price(&'a PromotionalSchemePriceDiscount),
    Product(&'a PromotionalSchemeProductDiscount),
}

fn extend_rules_for_slab(
    doc: &PromotionalScheme,
    rules: &mut Vec<PricingRuleDraft>,
    slab: SlabRef,
) {
    if let Some(applicable_for) = &doc.applicable_for {
        for value in doc.applicable_values(applicable_for) {
            let mut rule = prepare_pricing_rule(doc, &slab);
            set_applicable_value(&mut rule, applicable_for, value);
            rules.push(rule);
        }
    } else {
        rules.push(prepare_pricing_rule(doc, &slab));
    }
}

fn prepare_pricing_rule(doc: &PromotionalScheme, slab: &SlabRef) -> PricingRuleDraft {
    let mut rule = PricingRuleDraft {
        title: doc.name.clone(),
        promotional_scheme: doc.name.clone(),
        apply_on: doc.apply_on.clone(),
        applicable_for: doc.applicable_for.clone(),
        company: doc.company.clone(),
        disable: doc.disable,
        items: doc.items.clone(),
        ..Default::default()
    };

    match slab {
        SlabRef::Price(slab) => {
            rule.price_or_product_discount = "Price".to_string();
            rule.promotional_scheme_id = slab.name.clone();
            rule.disable = slab.disable || doc.disable;
            rule.min_qty = slab.min_qty;
            rule.max_qty = slab.max_qty;
            rule.min_amt = slab.min_amount;
            rule.max_amt = slab.max_amount;
            rule.discount_percentage = slab.discount_percentage;
            rule.rule_description = slab.rule_description.clone();
        }
        SlabRef::Product(slab) => {
            rule.price_or_product_discount = "Product".to_string();
            rule.promotional_scheme_id = slab.name.clone();
            rule.disable = slab.disable || doc.disable;
            rule.min_qty = slab.min_qty;
            rule.max_qty = slab.max_qty;
            rule.min_amt = slab.min_amount;
            rule.max_amt = slab.max_amount;
            rule.rule_description = slab.rule_description.clone();
            rule.free_item = slab.free_item.clone();
            rule.free_qty = slab.free_qty;
            rule.is_recursive = slab.is_recursive;
            rule.recurse_for = slab.recurse_for;
        }
    }

    rule
}

fn set_applicable_value(rule: &mut PricingRuleDraft, applicable_for: &str, value: &str) {
    rule.applicable_for = Some(applicable_for.to_string());
    match scrub(applicable_for).as_str() {
        "customer" => rule.customer = Some(value.to_string()),
        "customer_group" => rule.customer_group = Some(value.to_string()),
        "territory" => rule.territory = Some(value.to_string()),
        "sales_partner" => rule.sales_partner = Some(value.to_string()),
        "campaign" => rule.campaign = Some(value.to_string()),
        "supplier" => rule.supplier = Some(value.to_string()),
        "supplier_group" => rule.supplier_group = Some(value.to_string()),
        _ => {}
    }
}

fn scrub(value: &str) -> String {
    value.trim().to_lowercase().replace(' ', "_")
}
