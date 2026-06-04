use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PricingRule {
    pub name: Option<String>,
    pub title: Option<String>,
    pub disable: bool,
    pub apply_on: String,
    pub price_or_product_discount: String,
    pub items: Vec<PricingRuleChild>,
    pub item_groups: Vec<PricingRuleChild>,
    pub brands: Vec<PricingRuleChild>,
    pub mixed_conditions: bool,
    pub is_cumulative: bool,
    pub coupon_code_based: bool,
    pub apply_rule_on_other: Option<String>,
    pub other_item_code: Option<String>,
    pub other_item_group: Option<String>,
    pub other_brand: Option<String>,
    pub selling: bool,
    pub buying: bool,
    pub applicable_for: Option<String>,
    pub customer: Option<String>,
    pub customer_group: Option<String>,
    pub territory: Option<String>,
    pub sales_partner: Option<String>,
    pub campaign: Option<String>,
    pub supplier: Option<String>,
    pub supplier_group: Option<String>,
    pub min_qty: f64,
    pub max_qty: f64,
    pub min_amt: f64,
    pub max_amt: f64,
    pub same_item: bool,
    pub free_item: Option<String>,
    pub free_qty: f64,
    pub free_item_rate: f64,
    pub free_item_uom: Option<String>,
    pub round_free_qty: bool,
    pub dont_enforce_free_item_qty: bool,
    pub is_recursive: bool,
    pub recurse_for: f64,
    pub apply_recursion_over: f64,
    pub valid_from: Option<String>,
    pub valid_upto: Option<String>,
    pub company: Option<String>,
    pub currency: Option<String>,
    pub margin_type: Option<String>,
    pub margin_rate_or_amount: f64,
    pub rate_or_discount: Option<String>,
    pub apply_discount_on: Option<String>,
    pub rate: f64,
    pub discount_amount: f64,
    pub discount_percentage: f64,
    pub for_price_list: Option<String>,
    pub condition: Option<String>,
    pub apply_multiple_pricing_rules: bool,
    pub apply_discount_on_rate: bool,
    pub threshold_percentage: f64,
    pub validate_applied_rule: bool,
    pub rule_description: Option<String>,
    pub has_priority: bool,
    pub priority: Option<String>,
    pub promotional_scheme_id: Option<String>,
    pub promotional_scheme: Option<String>,
    pub selected_uom: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleChild {
    pub item_code: Option<String>,
    pub item_group: Option<String>,
    pub brand: Option<String>,
    pub uom: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PricingRuleArgs {
    pub doctype: String,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub parenttype: Option<String>,
    pub child_docname: Option<String>,
    pub item_code: Option<String>,
    pub item_group: Option<String>,
    pub brand: Option<String>,
    pub customer: Option<String>,
    pub customer_group: Option<String>,
    pub territory: Option<String>,
    pub supplier: Option<String>,
    pub supplier_group: Option<String>,
    pub quotation_to: Option<String>,
    pub transaction_type: Option<String>,
    pub currency: Option<String>,
    pub price_list_rate: f64,
    pub conversion_factor: f64,
    pub uom: Option<String>,
    pub ignore_pricing_rule: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PricingItemDetails {
    pub pricing_rule_for: Option<String>,
    pub has_margin: bool,
    pub margin_type: Option<String>,
    pub margin_rate_or_amount: Option<f64>,
    pub price_list_rate: Option<f64>,
    pub item_code: Option<String>,
    pub discount_percentage: f64,
    pub discount_amount: f64,
    pub rate: f64,
    pub pricing_rules: String,
    pub pricing_rule_removed: bool,
    pub remove_free_item: Option<String>,
    pub apply_on: Option<String>,
    pub applied_on_items: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PricingRuleRemovalInput {
    pub pricing_rule: PricingRule,
    pub exists: bool,
}

pub type PricingRuleRemovalState = PricingItemDetails;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PricingRuleDetail {
    pub pricing_rule: String,
    pub rate_or_discount: Option<String>,
    pub margin_type: Option<String>,
    pub item_code: Option<String>,
    pub child_docname: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplyPricingRulePlan {
    pub skip: bool,
    pub item_codes: Vec<String>,
    pub transaction_type: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemUomQueryPlan {
    pub parent_items: Vec<String>,
    pub uom_like: String,
    pub fields: Vec<&'static str>,
    pub distinct: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PricingRuleError {
    Mandatory(String),
    Validation(String),
}

impl PricingRule {
    pub const DOCTYPE: &'static str = "Pricing Rule";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const FIELD_ORDER: [&'static str; 88] = [
        "applicability_section",
        "naming_series",
        "title",
        "disable",
        "apply_on",
        "price_or_product_discount",
        "warehouse",
        "column_break_7",
        "items",
        "item_groups",
        "brands",
        "mixed_conditions",
        "is_cumulative",
        "coupon_code_based",
        "section_break_18",
        "apply_rule_on_other",
        "column_break_17",
        "other_item_code",
        "other_item_group",
        "other_brand",
        "section_break_7",
        "selling",
        "buying",
        "column_break_11",
        "applicable_for",
        "customer",
        "customer_group",
        "territory",
        "sales_partner",
        "campaign",
        "supplier",
        "supplier_group",
        "section_break_19",
        "min_qty",
        "max_qty",
        "column_break_21",
        "min_amt",
        "max_amt",
        "product_discount_scheme_section",
        "same_item",
        "free_item",
        "free_qty",
        "free_item_rate",
        "column_break_42",
        "free_item_uom",
        "round_free_qty",
        "dont_enforce_free_item_qty",
        "is_recursive",
        "recurse_for",
        "apply_recursion_over",
        "section_break_23",
        "valid_from",
        "valid_upto",
        "col_break1",
        "company",
        "currency",
        "margin",
        "margin_type",
        "column_break_33",
        "margin_rate_or_amount",
        "price_discount_scheme_section",
        "rate_or_discount",
        "apply_discount_on",
        "col_break2",
        "rate",
        "discount_amount",
        "discount_percentage",
        "for_price_list",
        "dynamic_condition_tab",
        "condition",
        "section_break_13",
        "apply_multiple_pricing_rules",
        "apply_discount_on_rate",
        "column_break_66",
        "threshold_percentage",
        "validate_pricing_rule_section",
        "validate_applied_rule",
        "column_break_texp",
        "rule_description",
        "priority_section",
        "has_priority",
        "column_break_sayg",
        "priority",
        "help_section",
        "pricing_rule_help",
        "reference_section",
        "promotional_scheme_id",
        "promotional_scheme",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("applicability_section"),
            FieldSpec::data("naming_series", "Series").default("PRLE-.####"),
            FieldSpec::data("title", "Title").required().no_copy(),
            FieldSpec::check("disable", "Disable").default("0"),
            FieldSpec::select("apply_on", "Apply On")
                .options("Item Code\nItem Group\nBrand\nTransaction")
                .default("Item Code")
                .required()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::select("price_or_product_discount", "Price or Product Discount")
                .options("Price\nProduct")
                .required(),
            FieldSpec::link("warehouse", "Warehouse")
                .options("Warehouse")
                .depends_on("eval:doc.apply_on != 'Transaction'")
                .search_index(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::table("items", "Apply Rule On Item Code")
                .options("Pricing Rule Item Code")
                .depends_on("eval:doc.apply_on == 'Item Code'"),
            FieldSpec::table("item_groups", "Apply Rule On Item Group")
                .options("Pricing Rule Item Group")
                .depends_on("eval:doc.apply_on == 'Item Group'"),
            FieldSpec::table("brands", "Apply Rule On Brand")
                .options("Pricing Rule Brand")
                .depends_on("eval:doc.apply_on == 'Brand'"),
        ]
    }

    pub fn validate(&mut self) -> Result<(), PricingRuleError> {
        self.validate_mandatory()?;
        self.validate_duplicate_apply_on()?;
        self.validate_applicable_for_selling_or_buying()?;
        self.validate_min_max_amt()?;
        self.validate_min_max_qty()?;
        self.validate_recursion()?;
        self.cleanup_fields_value();
        self.validate_rate_or_discount()?;
        self.validate_dates()?;
        self.validate_condition()?;
        self.validate_mixed_with_recursion()?;

        if self.margin_type.as_deref().unwrap_or_default().is_empty() {
            self.margin_rate_or_amount = 0.0;
        }

        Ok(())
    }

    pub fn validate_duplicate_apply_on(&self) -> Result<(), PricingRuleError> {
        let values = match self.apply_on.as_str() {
            "Item Code" => self
                .items
                .iter()
                .filter_map(|row| row.item_code.as_deref())
                .collect::<Vec<_>>(),
            "Item Group" => self
                .item_groups
                .iter()
                .filter_map(|row| row.item_group.as_deref())
                .collect::<Vec<_>>(),
            "Brand" => self
                .brands
                .iter()
                .filter_map(|row| row.brand.as_deref())
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        };

        let unique_count = values
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        if values.len() != unique_count {
            return Err(PricingRuleError::Validation(format!(
                "Duplicate {} found in the table",
                self.apply_on
            )));
        }

        Ok(())
    }

    pub fn validate_mandatory(&mut self) -> Result<(), PricingRuleError> {
        if self.has_priority && !has_value(&self.priority) {
            return Err(PricingRuleError::Mandatory(
                "Priority is mandatory".to_string(),
            ));
        }

        if has_value(&self.priority) && !self.has_priority {
            self.has_priority = true;
        }

        if self.apply_on == "Item Code" && self.items.is_empty() {
            return Err(PricingRuleError::Mandatory(
                "Item Code is not added in the table".to_string(),
            ));
        }
        if self.apply_on == "Item Group" && self.item_groups.is_empty() {
            return Err(PricingRuleError::Mandatory(
                "Item Group is not added in the table".to_string(),
            ));
        }
        if self.apply_on == "Brand" && self.brands.is_empty() {
            return Err(PricingRuleError::Mandatory(
                "Brand is not added in the table".to_string(),
            ));
        }

        if let Some(applicable_for) = self.applicable_for.as_deref() {
            if !applicable_for.is_empty() && !self.applicable_for_value_is_set(applicable_for) {
                return Err(PricingRuleError::Mandatory(format!(
                    "{applicable_for} is required"
                )));
            }
        }

        if let Some(apply_rule_on_other) = self.apply_rule_on_other.as_deref() {
            let missing = match apply_rule_on_other {
                "Item Code" => !has_value(&self.other_item_code),
                "Item Group" => !has_value(&self.other_item_group),
                "Brand" => !has_value(&self.other_brand),
                _ => false,
            };
            if missing {
                return Err(PricingRuleError::Validation(format!(
                    "For the 'Apply Rule On Other' condition the field <b>{apply_rule_on_other}</b> is mandatory"
                )));
            }
        }

        if self.price_or_product_discount == "Price" && !has_value(&self.rate_or_discount) {
            return Err(PricingRuleError::Mandatory(
                "Rate or Discount is required for the price discount.".to_string(),
            ));
        }

        if self.apply_discount_on_rate {
            if !has_value(&self.priority) {
                return Err(PricingRuleError::Validation(
                    "As the field <b>Apply Discount on Discounted Rate</b> is enabled, the field <b>Priority</b> is mandatory."
                        .to_string(),
                ));
            }
            if self
                .priority
                .as_deref()
                .and_then(|value| value.parse::<i32>().ok())
                == Some(1)
            {
                return Err(PricingRuleError::Validation(
                    "As the field <b>Apply Discount on Discounted Rate</b> is enabled, the value of the field <b>Priority</b> should be more than 1."
                        .to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn validate_applicable_for_selling_or_buying(&self) -> Result<(), PricingRuleError> {
        if !self.selling && !self.buying {
            return Err(PricingRuleError::Validation(
                "At least one of the Selling or Buying must be selected".to_string(),
            ));
        }

        let applicable_for = self.applicable_for.as_deref().unwrap_or_default();
        if !self.selling
            && matches!(
                applicable_for,
                "Customer" | "Customer Group" | "Territory" | "Sales Partner" | "Campaign"
            )
        {
            return Err(PricingRuleError::Validation(format!(
                "Selling must be checked, if Applicable For is selected as {applicable_for}"
            )));
        }

        if !self.buying && matches!(applicable_for, "Supplier" | "Supplier Group") {
            return Err(PricingRuleError::Validation(format!(
                "Buying must be checked, if Applicable For is selected as {applicable_for}"
            )));
        }

        Ok(())
    }

    pub fn validate_min_max_qty(&self) -> Result<(), PricingRuleError> {
        if self.min_qty != 0.0 && self.max_qty != 0.0 && self.min_qty > self.max_qty {
            return Err(PricingRuleError::Validation(
                "Min Qty can not be greater than Max Qty".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_min_max_amt(&self) -> Result<(), PricingRuleError> {
        if self.min_amt != 0.0 && self.max_amt != 0.0 && self.min_amt > self.max_amt {
            return Err(PricingRuleError::Validation(
                "Min Amt can not be greater than Max Amt".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_recursion(&mut self) -> Result<(), PricingRuleError> {
        if self.price_or_product_discount != "Product" {
            return Ok(());
        }
        if (has_value(&self.free_item) || self.same_item) && self.recurse_for <= 0.0 {
            self.recurse_for = 1.0;
        }
        if self.is_recursive {
            if self.apply_recursion_over > self.min_qty {
                return Err(PricingRuleError::Validation(
                    "Min Qty should be greater than Recurse Over Qty".to_string(),
                ));
            }
            if self.apply_recursion_over < 0.0 {
                return Err(PricingRuleError::Validation(
                    "Recurse Over Qty cannot be less than 0".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn cleanup_fields_value(&mut self) {
        if self.apply_on != "Item Code" {
            self.items.clear();
        }
        if self.apply_on != "Item Group" {
            self.item_groups.clear();
        }
        if self.apply_on != "Brand" {
            self.brands.clear();
        }

        match self.applicable_for.as_deref().unwrap_or_default() {
            "Customer" => {
                self.customer_group = None;
                self.territory = None;
                self.sales_partner = None;
                self.campaign = None;
                self.supplier = None;
                self.supplier_group = None;
            }
            "Supplier" => {
                self.customer = None;
                self.customer_group = None;
                self.territory = None;
                self.sales_partner = None;
                self.campaign = None;
                self.supplier_group = None;
            }
            _ => {}
        }

        if self.mixed_conditions && self.same_item {
            self.same_item = false;
        }

        match self.apply_rule_on_other.as_deref().unwrap_or_default() {
            "Item Code" => {
                self.other_item_group = None;
                self.other_brand = None;
            }
            "Item Group" => {
                self.other_item_code = None;
                self.other_brand = None;
            }
            "Brand" => {
                self.other_item_code = None;
                self.other_item_group = None;
            }
            _ => {
                self.other_item_code = None;
                self.other_item_group = None;
                self.other_brand = None;
            }
        }
    }

    pub fn validate_rate_or_discount(&mut self) -> Result<(), PricingRuleError> {
        if self.rate < 0.0 {
            return Err(PricingRuleError::Validation(
                "Rate can not be negative".to_string(),
            ));
        }

        if self.price_or_product_discount == "Product" && !has_value(&self.free_item) {
            if self.mixed_conditions {
                return Err(PricingRuleError::Validation(
                    "Free item code is not selected".to_string(),
                ));
            }
            self.same_item = true;
        }

        Ok(())
    }

    pub fn validate_price_list_with_currency(
        &self,
        price_list_currency: Option<&str>,
    ) -> Result<(), PricingRuleError> {
        if let (Some(currency), Some(_price_list), Some(price_list_currency)) = (
            self.currency.as_deref(),
            self.for_price_list.as_deref(),
            price_list_currency,
        ) {
            if currency != price_list_currency {
                return Err(PricingRuleError::Validation(format!(
                    "Currency should be same as Price List Currency: {price_list_currency}"
                )));
            }
        }
        Ok(())
    }

    pub fn validate_max_discount(
        &self,
        item_max_discounts: &[(String, f64)],
    ) -> Result<(), PricingRuleError> {
        if self.rate_or_discount.as_deref() != Some("Discount Percentage") {
            return Ok(());
        }

        for row in &self.items {
            let Some(item_code) = row.item_code.as_deref() else {
                continue;
            };
            let max_discount = item_max_discounts
                .iter()
                .find(|(code, _)| code == item_code)
                .map(|(_, value)| *value)
                .unwrap_or(0.0);
            if max_discount != 0.0 && self.discount_percentage > max_discount {
                return Err(PricingRuleError::Validation(format!(
                    "Max discount allowed for item: {item_code} is {}%",
                    format_number(max_discount)
                )));
            }
        }

        Ok(())
    }

    pub fn validate_dates(&self) -> Result<(), PricingRuleError> {
        if self.is_cumulative && !(has_value(&self.valid_from) && has_value(&self.valid_upto)) {
            return Err(PricingRuleError::Validation(
                "Valid from and valid upto fields are mandatory for the cumulative".to_string(),
            ));
        }
        if let (Some(valid_from), Some(valid_upto)) =
            (self.valid_from.as_deref(), self.valid_upto.as_deref())
        {
            if valid_from > valid_upto {
                return Err(PricingRuleError::Validation(
                    "Valid From cannot be greater than Valid Upto".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn validate_condition(&self) -> Result<(), PricingRuleError> {
        let Some(condition) = self.condition.as_deref() else {
            return Ok(());
        };
        if looks_like_single_equals_condition(condition) {
            return Err(PricingRuleError::Validation(
                "Invalid condition expression".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_mixed_with_recursion(&self) -> Result<(), PricingRuleError> {
        if self.mixed_conditions && self.is_recursive {
            return Err(PricingRuleError::Validation(
                "Recursive Discounts with Mixed condition is not supported by the system"
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn applicable_for_value_is_set(&self, applicable_for: &str) -> bool {
        match applicable_for {
            "Customer" => has_value(&self.customer),
            "Customer Group" => has_value(&self.customer_group),
            "Territory" => has_value(&self.territory),
            "Sales Partner" => has_value(&self.sales_partner),
            "Campaign" => has_value(&self.campaign),
            "Supplier" => has_value(&self.supplier),
            "Supplier Group" => has_value(&self.supplier_group),
            _ => true,
        }
    }
}

impl DocumentController for PricingRule {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

pub fn apply_price_discount_rule(
    pricing_rule: &PricingRule,
    item_details: &mut PricingItemDetails,
    args: &PricingRuleArgs,
) {
    let rate_or_discount = pricing_rule.rate_or_discount.as_deref().unwrap_or_default();
    item_details.pricing_rule_for = Some(rate_or_discount.to_string());

    if matches!(
        pricing_rule.margin_type.as_deref(),
        Some("Amount" | "Percentage")
    ) && (pricing_rule.currency == args.currency
        || pricing_rule.margin_type.as_deref() == Some("Percentage"))
    {
        item_details.margin_type = pricing_rule.margin_type.clone();
        item_details.has_margin = true;
        if pricing_rule.apply_multiple_pricing_rules && item_details.margin_rate_or_amount.is_some()
        {
            item_details.margin_rate_or_amount = Some(
                item_details.margin_rate_or_amount.unwrap_or(0.0)
                    + pricing_rule.margin_rate_or_amount,
            );
        } else {
            item_details.margin_rate_or_amount = Some(pricing_rule.margin_rate_or_amount);
        }
    }

    if rate_or_discount == "Rate" {
        let pricing_rule_rate = if pricing_rule.currency == args.currency {
            pricing_rule.rate
        } else {
            0.0
        };
        if pricing_rule_rate != 0.0 {
            let is_blank_uom = pricing_rule
                .items
                .first()
                .and_then(|row| row.uom.as_deref())
                != args.uom.as_deref();
            item_details.price_list_rate = Some(
                pricing_rule_rate
                    * if is_blank_uom {
                        non_zero(args.conversion_factor, 1.0)
                    } else {
                        1.0
                    },
            );
        }
        item_details.discount_percentage = 0.0;
    }

    for apply_on in ["Discount Amount", "Discount Percentage"] {
        if rate_or_discount != apply_on {
            continue;
        }
        let mut field = scrub(apply_on);
        if pricing_rule.apply_discount_on_rate && item_details.discount_percentage != 0.0 {
            let existing = item_details.discount_percentage;
            item_details.discount_percentage +=
                (100.0 - existing) * (pricing_rule.discount_percentage / 100.0);
        } else if args.price_list_rate != 0.0 {
            let mut value = if field == "discount_percentage" {
                pricing_rule.discount_percentage
            } else {
                pricing_rule.discount_amount
            };
            let calculate_discount_percentage = if field == "discount_percentage" {
                field = "discount_amount".to_string();
                value = args.price_list_rate * (value / 100.0);
                true
            } else {
                false
            };

            if field == "discount_amount" {
                item_details.discount_amount += value;
            } else {
                item_details.discount_percentage += value;
            }

            if calculate_discount_percentage && item_details.discount_amount != 0.0 {
                item_details.discount_percentage =
                    (item_details.discount_amount / args.price_list_rate) * 100.0;
            }
        } else if field == "discount_amount" {
            item_details.discount_amount += pricing_rule.discount_amount;
        } else {
            item_details.discount_percentage += pricing_rule.discount_percentage;
        }
    }
}

pub fn update_pricing_rule_uom(pricing_rule: &mut PricingRule, args: &PricingRuleArgs) {
    let apply_on_field = scrub(&pricing_rule.apply_on);
    let matching_uom = match apply_on_field.as_str() {
        "item_code" => pricing_rule
            .items
            .iter()
            .find(|row| row.item_code == args.item_code)
            .and_then(|row| row.uom.clone()),
        "item_group" => pricing_rule
            .item_groups
            .iter()
            .find(|row| row.item_group == args.item_group)
            .and_then(|row| row.uom.clone()),
        "brand" => pricing_rule
            .brands
            .iter()
            .find(|row| row.brand == args.brand)
            .and_then(|row| row.uom.clone()),
        _ => None,
    };
    pricing_rule.selected_uom = matching_uom;
}

pub fn get_pricing_rule_details(
    args: &PricingRuleArgs,
    pricing_rule: &PricingRule,
) -> PricingRuleDetail {
    PricingRuleDetail {
        pricing_rule: pricing_rule.name.clone().unwrap_or_default(),
        rate_or_discount: pricing_rule.rate_or_discount.clone(),
        margin_type: pricing_rule.margin_type.clone(),
        item_code: args.item_code.clone(),
        child_docname: args.child_docname.clone(),
    }
}

pub fn apply_pricing_rule_plan(
    item_list: &[PricingRuleArgs],
    doctype: &str,
) -> ApplyPricingRulePlan {
    let mut context = PricingRuleArgs {
        doctype: doctype.to_string(),
        ..PricingRuleArgs::default()
    };
    set_transaction_type(&mut context);

    ApplyPricingRulePlan {
        skip: doctype == "Material Request",
        item_codes: item_list
            .iter()
            .filter_map(|item| item.item_code.clone())
            .collect(),
        transaction_type: match context.transaction_type.as_deref() {
            Some("selling") => "selling",
            _ => "buying",
        },
    }
}

pub fn remove_pricing_rule_for_item(
    pricing_rules: &[PricingRuleRemovalInput],
    mut item_details: PricingRuleRemovalState,
    item_code: Option<&str>,
    rate: Option<f64>,
) -> PricingRuleRemovalState {
    for input in pricing_rules {
        if !input.exists {
            continue;
        }
        let pricing_rule = &input.pricing_rule;

        if pricing_rule.price_or_product_discount == "Price" {
            match pricing_rule.rate_or_discount.as_deref() {
                Some("Discount Percentage") => {
                    item_details.discount_percentage = 0.0;
                    item_details.discount_amount = 0.0;
                    item_details.rate = rate.unwrap_or(0.0);
                }
                Some("Discount Amount") => {
                    item_details.discount_amount = 0.0;
                }
                _ => {}
            }

            if matches!(
                pricing_rule.margin_type.as_deref(),
                Some("Percentage" | "Amount")
            ) {
                item_details.margin_rate_or_amount = Some(0.0);
                item_details.margin_type = None;
            }
        } else if has_value(&pricing_rule.free_item) && !pricing_rule.dont_enforce_free_item_qty {
            item_details.remove_free_item = Some(if pricing_rule.same_item {
                item_code.unwrap_or_default().to_string()
            } else {
                pricing_rule.free_item.clone().unwrap_or_default()
            });
        }

        if pricing_rule.mixed_conditions || has_value(&pricing_rule.apply_rule_on_other) {
            item_details.apply_on = Some(if has_value(&pricing_rule.apply_rule_on_other) {
                scrub(
                    pricing_rule
                        .apply_rule_on_other
                        .as_deref()
                        .unwrap_or_default(),
                )
            } else {
                scrub(&pricing_rule.apply_on)
            });
        }
    }

    item_details.pricing_rules.clear();
    item_details.pricing_rule_removed = true;
    item_details
}

pub fn remove_pricing_rules(
    item_list: Vec<PricingRuleRemovalState>,
) -> Vec<PricingRuleRemovalState> {
    item_list
        .into_iter()
        .filter(|item| !item.pricing_rules.is_empty())
        .map(|item| {
            let item_code = item.item_code.clone();
            let rate = item.price_list_rate;
            remove_pricing_rule_for_item(&[], item, item_code.as_deref(), rate)
        })
        .collect()
}

pub fn get_item_uoms_plan(
    txt: &str,
    apply_on: &str,
    value: &str,
    matching_items: Vec<String>,
) -> ItemUomQueryPlan {
    let parent_items = if apply_on == "Item Code" {
        vec![value.to_string()]
    } else {
        matching_items
    };

    ItemUomQueryPlan {
        parent_items,
        uom_like: format!("{txt}%"),
        fields: vec!["uom"],
        distinct: true,
    }
}

pub fn set_transaction_type(pricing_ctx: &mut PricingRuleArgs) {
    if matches!(
        pricing_ctx.transaction_type.as_deref(),
        Some("buying" | "selling")
    ) {
        return;
    }

    if matches!(
        pricing_ctx.doctype.as_str(),
        "Opportunity" | "Quotation" | "Sales Order" | "Delivery Note" | "Sales Invoice"
    ) {
        pricing_ctx.transaction_type = Some("selling".to_string());
    } else if matches!(
        pricing_ctx.doctype.as_str(),
        "Material Request"
            | "Supplier Quotation"
            | "Purchase Order"
            | "Purchase Receipt"
            | "Purchase Invoice"
    ) {
        pricing_ctx.transaction_type = Some("buying".to_string());
    } else if has_value(&pricing_ctx.customer) {
        pricing_ctx.transaction_type = Some("selling".to_string());
    } else {
        pricing_ctx.transaction_type = Some("buying".to_string());
    }
}

pub fn update_args_for_pricing_rule(
    args: &mut PricingRuleArgs,
    item_defaults: Option<(String, String)>,
    customer_defaults: Option<(String, String)>,
    supplier_group: Option<String>,
) -> Result<(), PricingRuleError> {
    if !(has_value(&args.item_group) && has_value(&args.brand)) {
        if let Some((item_group, brand)) = item_defaults {
            args.item_group = Some(item_group);
            args.brand = Some(brand);
            if !has_value(&args.item_group) {
                return Err(PricingRuleError::Validation(format!(
                    "Item Group not mentioned in item master for item {}",
                    args.item_code.as_deref().unwrap_or_default()
                )));
            }
        }
    }

    if args.transaction_type.as_deref() == Some("selling") {
        if has_value(&args.customer)
            && !(has_value(&args.customer_group) && has_value(&args.territory))
            && args.quotation_to.as_deref().unwrap_or("Customer") == "Customer"
        {
            if let Some((customer_group, territory)) = customer_defaults {
                args.customer_group = Some(customer_group);
                args.territory = Some(territory);
            }
        }
        args.supplier = None;
        args.supplier_group = None;
    } else if has_value(&args.supplier) && !has_value(&args.supplier_group) {
        args.supplier_group = supplier_group;
        args.customer = None;
        args.customer_group = None;
        args.territory = None;
    }

    Ok(())
}

fn has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}

fn scrub(value: &str) -> String {
    value.trim().to_lowercase().replace(' ', "_")
}

fn non_zero(value: f64, default_value: f64) -> f64 {
    if value == 0.0 {
        default_value
    } else {
        value
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn looks_like_single_equals_condition(condition: &str) -> bool {
    let trimmed = condition.trim();
    if !trimmed.contains('=') || trimmed.contains("==") || trimmed.contains("!=") {
        return false;
    }

    let mut parts = trimmed.split('=');
    let left = parts.next().unwrap_or_default().trim();
    let right = parts.next().unwrap_or_default().trim();
    !left.is_empty()
        && left
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | ':'))
        && !right.is_empty()
        && right.chars().next().is_some_and(|ch| {
            ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '@' | '\'' | '"')
        })
}
