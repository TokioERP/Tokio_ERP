use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PricingRuleCandidate {
    pub name: String,
    pub priority: Option<i32>,
    pub apply_multiple_pricing_rules: bool,
    pub condition_passes: Option<bool>,
    pub mixed_conditions: bool,
    pub is_cumulative: bool,
    pub apply_rule_on_other: bool,
    pub min_qty: f64,
    pub max_qty: f64,
    pub min_amt: f64,
    pub max_amt: f64,
    pub threshold_percentage: f64,
    pub title: Option<String>,
    pub currency: Option<String>,
    pub rate_or_discount: Option<String>,
    pub for_price_list: Option<String>,
    pub uom: Option<String>,
    pub item_code: Option<String>,
    pub variant_of: Option<String>,
    pub apply_rule_on_other_items: Vec<String>,
    pub free_item: Option<String>,
    pub same_item: bool,
    pub apply_on: Option<String>,
    pub free_qty: f64,
    pub is_recursive: bool,
    pub apply_recursion_over: f64,
    pub recurse_for: f64,
    pub round_free_qty: bool,
    pub free_item_rate: f64,
    pub free_item_uom: Option<String>,
    pub dont_enforce_free_item_qty: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PricingRuleFilterArgs {
    pub stock_qty: f64,
    pub price_list_rate: f64,
    pub qty: f64,
    pub item_code: Option<String>,
    pub transaction_type: Option<String>,
    pub currency: Option<String>,
    pub price_list: Option<String>,
    pub uom: Option<String>,
    pub for_shopping_cart: bool,
    pub variant_of: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeConditionInput {
    pub parenttype: &'static str,
    pub field: &'static str,
    pub table: &'static str,
    pub value: Option<String>,
    pub parent_groups: Vec<String>,
    pub allow_blank: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PricingRuleOtherConditions {
    pub company: Option<String>,
    pub customer: Option<String>,
    pub supplier: Option<String>,
    pub campaign: Option<String>,
    pub sales_partner: Option<String>,
    pub customer_group_condition: Option<String>,
    pub territory_condition: Option<String>,
    pub supplier_group_condition: Option<String>,
    pub transaction_date: Option<String>,
    pub doctype: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OtherConditionPlan {
    pub sql: String,
    pub values: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PricingRuleFilterOutcome {
    Selected(PricingRuleCandidate),
    Suggestion { message: String, item_code: String },
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiplePricingRuleConflict(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct FreeItemData {
    pub item_code: String,
    pub qty: f64,
    pub pricing_rules: String,
    pub rate: f64,
    pub price_list_rate: f64,
    pub is_free_item: bool,
    pub uom: Option<String>,
    pub schedule_date: Option<String>,
    pub delivery_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CouponCode {
    pub coupon_code: String,
    pub valid_from: Option<String>,
    pub valid_upto: Option<String>,
    pub maximum_use: i32,
    pub used: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CouponCodeError {
    Validation(String),
}

impl TreeConditionInput {
    pub fn condition(&self) -> String {
        if self.value.is_some() {
            let mut parent_groups = self.parent_groups.clone();
            if self.allow_blank {
                parent_groups.push(String::new());
            }
            if parent_groups.is_empty() {
                return String::new();
            }
            return format!(
                "ifnull({}.{}, '') in ({})",
                self.table,
                self.field,
                parent_groups
                    .iter()
                    .map(|value| format!("'{}'", value))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if self.allow_blank {
            format!("ifnull({}.{}, '') = ''", self.table, self.field)
        } else {
            String::new()
        }
    }
}

pub fn apply_multiple_pricing_rules(pricing_rules: &[PricingRuleCandidate]) -> bool {
    pricing_rules
        .iter()
        .all(|rule| rule.apply_multiple_pricing_rules)
}

pub fn sorted_by_priority(pricing_rules: &[PricingRuleCandidate]) -> Vec<PricingRuleCandidate> {
    let mut rules = pricing_rules
        .iter()
        .filter(|rule| rule.apply_multiple_pricing_rules)
        .cloned()
        .collect::<Vec<_>>();
    for rule in &mut rules {
        if rule.priority.is_none() {
            rule.priority = Some(1);
        }
    }
    rules.sort_by_key(|rule| rule.priority.unwrap_or(1));
    rules
}

pub fn filter_pricing_rule_based_on_condition(
    pricing_rules: &[PricingRuleCandidate],
    has_doc: bool,
) -> Vec<PricingRuleCandidate> {
    if !has_doc {
        return pricing_rules.to_vec();
    }

    pricing_rules
        .iter()
        .filter(|rule| rule.condition_passes.unwrap_or(true))
        .cloned()
        .collect()
}

pub fn get_other_conditions(input: &PricingRuleOtherConditions) -> OtherConditionPlan {
    let mut sql = String::new();
    let mut values = BTreeMap::new();

    for (field, value) in [
        ("company", &input.company),
        ("customer", &input.customer),
        ("supplier", &input.supplier),
        ("campaign", &input.campaign),
        ("sales_partner", &input.sales_partner),
    ] {
        if let Some(value) = value {
            sql.push_str(&format!(
                " and ifnull(`tabPricing Rule`.{field}, '') in (%({field})s, '')"
            ));
            values.insert(field.to_string(), value.clone());
        } else {
            sql.push_str(&format!(" and ifnull(`tabPricing Rule`.{field}, '') = ''"));
        }
    }

    for condition in [
        &input.customer_group_condition,
        &input.territory_condition,
        &input.supplier_group_condition,
    ]
    .into_iter()
    .flatten()
    {
        if !condition.is_empty() {
            sql.push_str(" and ");
            sql.push_str(condition);
        }
    }

    if let Some(date) = &input.transaction_date {
        sql.push_str(
            " and %(transaction_date)s between ifnull(`tabPricing Rule`.valid_from, '2000-01-01') and ifnull(`tabPricing Rule`.valid_upto, '2500-12-31')",
        );
        values.insert("transaction_date".to_string(), date.clone());
    }

    if is_selling_doctype(&input.doctype) {
        sql.push_str(" and ifnull(`tabPricing Rule`.selling, 0) = 1");
    } else {
        sql.push_str(" and ifnull(`tabPricing Rule`.buying, 0) = 1");
    }

    OtherConditionPlan { sql, values }
}

pub fn filter_pricing_rules(
    args: &PricingRuleFilterArgs,
    pricing_rules: &[PricingRuleCandidate],
) -> Result<PricingRuleFilterOutcome, MultiplePricingRuleConflict> {
    let stock_qty = args.stock_qty;
    let amount = args.price_list_rate * args.qty;
    let original = pricing_rules.to_vec();
    let mut pricing_rules =
        filter_pricing_rules_for_qty_amount(stock_qty, amount, pricing_rules, args.uom.as_deref());

    if pricing_rules.is_empty() {
        for rule in original {
            if rule.threshold_percentage == 0.0 {
                continue;
            }
            if let Some(message) = validate_quantity_and_amount_for_suggestion(
                &rule,
                stock_qty,
                amount,
                args.item_code.as_deref().unwrap_or_default(),
                args.transaction_type.as_deref().unwrap_or_default(),
            ) {
                return Ok(PricingRuleFilterOutcome::Suggestion {
                    message,
                    item_code: args.item_code.clone().unwrap_or_default(),
                });
            }
        }
    }

    for rule in &mut pricing_rules {
        if rule.item_code.is_some() && args.variant_of.is_some() {
            rule.variant_of = args.variant_of.clone();
        } else {
            rule.variant_of = None;
        }
    }

    if pricing_rules.len() > 1 {
        let currency_filtered = pricing_rules
            .iter()
            .filter(|rule| rule.currency == args.currency)
            .cloned()
            .collect::<Vec<_>>();
        if !currency_filtered.is_empty() {
            pricing_rules = currency_filtered;
        }
    }

    if !pricing_rules.is_empty() {
        let max_priority = pricing_rules
            .iter()
            .filter_map(|rule| rule.priority)
            .max()
            .unwrap_or(0);
        if max_priority != 0 {
            pricing_rules.retain(|rule| rule.priority.unwrap_or(0) == max_priority);
        }
    }

    if pricing_rules.len() > 1 {
        let rate_or_discount = pricing_rules
            .iter()
            .map(|rule| rule.rate_or_discount.clone())
            .collect::<std::collections::BTreeSet<_>>();
        if rate_or_discount.len() == 1
            && rate_or_discount.contains(&Some("Discount Percentage".to_string()))
        {
            let price_list_filtered = pricing_rules
                .iter()
                .filter(|rule| rule.for_price_list == args.price_list)
                .cloned()
                .collect::<Vec<_>>();
            if !price_list_filtered.is_empty() {
                pricing_rules = price_list_filtered;
            }
        }
    }

    if pricing_rules.len() > 1 && !args.for_shopping_cart {
        return Err(MultiplePricingRuleConflict(format!(
            "Multiple Price Rules exists with same criteria, please resolve conflict by assigning priority. Price Rules: {}",
            pricing_rules
                .iter()
                .map(|rule| rule.name.clone())
                .collect::<Vec<_>>()
                .join("\n")
        )));
    }

    Ok(pricing_rules
        .into_iter()
        .next()
        .map(PricingRuleFilterOutcome::Selected)
        .unwrap_or(PricingRuleFilterOutcome::None))
}

pub fn validate_quantity_and_amount_for_suggestion(
    rule: &PricingRuleCandidate,
    qty: f64,
    amount: f64,
    item_code: &str,
    transaction_type: &str,
) -> Option<String> {
    let transaction = if transaction_type == "buying" {
        "purchase"
    } else {
        "sale"
    };

    let mut fieldname = "";
    for (field, value, threshold) in [
        ("min_qty", qty, rule.min_qty),
        ("min_amt", amount, rule.min_amt),
    ] {
        if threshold != 0.0
            && value < threshold
            && threshold - (threshold * rule.threshold_percentage * 0.01).trunc() <= value
        {
            fieldname = field;
        }
    }
    for (field, value, threshold) in [
        ("max_qty", qty, rule.max_qty),
        ("max_amt", amount, rule.max_amt),
    ] {
        if threshold != 0.0
            && value > threshold
            && threshold + (threshold * rule.threshold_percentage * 0.01).trunc() >= value
        {
            fieldname = field;
        }
    }

    if fieldname.is_empty() {
        return None;
    }

    let title = rule.title.as_deref().unwrap_or_default();
    if matches!(fieldname, "min_amt" | "max_amt") {
        let value = if fieldname == "min_amt" {
            rule.min_amt
        } else {
            rule.max_amt
        };
        Some(format!(
            "If you {transaction} {} worth item <b>{item_code}</b>, the scheme <b>{title}</b> will be applied on the item.",
            format_money(value)
        ))
    } else {
        let value = if fieldname == "min_qty" {
            rule.min_qty
        } else {
            rule.max_qty
        };
        Some(format!(
            "If you {transaction} {} quantities of the item <b>{item_code}</b>, the scheme <b>{title}</b> will be applied on the item.",
            format_number(value)
        ))
    }
}

pub fn filter_pricing_rules_for_qty_amount(
    qty: f64,
    rate: f64,
    pricing_rules: &[PricingRuleCandidate],
    args_uom: Option<&str>,
) -> Vec<PricingRuleCandidate> {
    pricing_rules
        .iter()
        .filter(|rule| {
            let conversion_factor = if rule.uom.as_deref() == args_uom {
                1.0
            } else {
                1.0
            };
            let qty_ok = qty >= rule.min_qty * conversion_factor
                && if rule.max_qty != 0.0 {
                    qty <= rule.max_qty * conversion_factor
                } else {
                    true
                };
            let amount_ok = rate >= rule.min_amt * conversion_factor
                && if rule.max_amt != 0.0 {
                    rate <= rule.max_amt * conversion_factor
                } else {
                    true
                };
            qty_ok && amount_ok
        })
        .cloned()
        .collect()
}

pub fn get_applied_pricing_rules(pricing_rules: &str) -> Vec<String> {
    let pricing_rules = pricing_rules.trim();
    if pricing_rules.is_empty() {
        return Vec::new();
    }
    if pricing_rules.starts_with('[') {
        return pricing_rules
            .trim_matches(['[', ']'])
            .split(',')
            .map(|value| value.trim().trim_matches('"').to_string())
            .filter(|value| !value.is_empty())
            .collect();
    }
    pricing_rules
        .split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

pub fn get_product_discount_rule(
    pricing_rule: &PricingRuleCandidate,
    parenttype: &str,
    transaction_qty: f64,
    current_item_qty: f64,
) -> Result<Option<FreeItemData>, String> {
    let mut free_item = pricing_rule.free_item.clone();
    if pricing_rule.same_item && pricing_rule.apply_on.as_deref() != Some("Transaction") {
        free_item = pricing_rule
            .item_code
            .clone()
            .or_else(|| Some(String::new()));
    }

    let Some(free_item) = free_item else {
        return Err(format!(
            "Free item not set in the pricing rule Pricing Rule/{}",
            pricing_rule.name
        ));
    };

    let mut qty = if pricing_rule.free_qty == 0.0 {
        1.0
    } else {
        pricing_rule.free_qty
    };
    if pricing_rule.is_recursive {
        let transaction_qty = transaction_qty - pricing_rule.apply_recursion_over;
        if transaction_qty > 0.0 {
            qty = transaction_qty * qty / pricing_rule.recurse_for;
            if pricing_rule.round_free_qty {
                qty = (transaction_qty / pricing_rule.recurse_for).floor()
                    * if pricing_rule.free_qty == 0.0 {
                        1.0
                    } else {
                        pricing_rule.free_qty
                    };
            }
        }
    }

    if qty == 0.0 || current_item_qty == 0.0 {
        return Ok(None);
    }

    Ok(Some(FreeItemData {
        item_code: free_item,
        qty,
        pricing_rules: pricing_rule.name.clone(),
        rate: pricing_rule.free_item_rate,
        price_list_rate: pricing_rule.free_item_rate,
        is_free_item: true,
        uom: pricing_rule.free_item_uom.clone(),
        schedule_date: if parenttype == "Purchase Order" {
            Some("today".to_string())
        } else {
            None
        },
        delivery_date: if parenttype == "Sales Order" {
            Some("today".to_string())
        } else {
            None
        },
    }))
}

pub fn validate_coupon_code(coupon: &CouponCode, today: &str) -> Result<(), CouponCodeError> {
    if coupon
        .valid_from
        .as_deref()
        .is_some_and(|valid_from| valid_from > today)
    {
        return Err(CouponCodeError::Validation(
            "Sorry, this coupon code's validity has not started".to_string(),
        ));
    }
    if coupon
        .valid_upto
        .as_deref()
        .is_some_and(|valid_upto| valid_upto < today)
    {
        return Err(CouponCodeError::Validation(
            "Sorry, this coupon code's validity has expired".to_string(),
        ));
    }
    if coupon.maximum_use != 0 && coupon.used >= coupon.maximum_use {
        return Err(CouponCodeError::Validation(
            "Sorry, this coupon code is no longer valid".to_string(),
        ));
    }
    Ok(())
}

pub fn update_coupon_code_count(
    coupon: &mut CouponCode,
    transaction_type: &str,
) -> Result<(), CouponCodeError> {
    match transaction_type {
        "used" => {
            if coupon.maximum_use == 0 || coupon.used < coupon.maximum_use {
                coupon.used += 1;
            } else {
                return Err(CouponCodeError::Validation(format!(
                    "{} Coupon used are {}. Allowed quantity is exhausted",
                    coupon.coupon_code, coupon.used
                )));
            }
        }
        "cancelled" => {
            if coupon.used > 0 {
                coupon.used -= 1;
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_selling_doctype(doctype: &str) -> bool {
    matches!(
        doctype,
        "Quotation"
            | "Quotation Item"
            | "Sales Order"
            | "Sales Order Item"
            | "Delivery Note"
            | "Delivery Note Item"
            | "Sales Invoice"
            | "Sales Invoice Item"
            | "POS Invoice"
            | "POS Invoice Item"
    )
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn format_money(value: f64) -> String {
    format_number(value)
}
