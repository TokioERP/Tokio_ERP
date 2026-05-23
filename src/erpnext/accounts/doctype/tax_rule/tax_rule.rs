use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxRule {
    pub name: String,
    pub tax_type: String,
    pub use_for_shopping_cart: bool,
    pub sales_tax_template: Option<String>,
    pub purchase_tax_template: Option<String>,
    pub customer: Option<String>,
    pub supplier: Option<String>,
    pub item: Option<String>,
    pub billing_city: Option<String>,
    pub billing_county: Option<String>,
    pub billing_state: Option<String>,
    pub billing_zipcode: Option<String>,
    pub billing_country: Option<String>,
    pub tax_category: Option<String>,
    pub customer_group: Option<String>,
    pub supplier_group: Option<String>,
    pub item_group: Option<String>,
    pub shipping_city: Option<String>,
    pub shipping_county: Option<String>,
    pub shipping_state: Option<String>,
    pub shipping_zipcode: Option<String>,
    pub shipping_country: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub priority: i32,
    pub company: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaxRuleError {
    MandatoryTaxTemplate(String),
    ConflictingTaxRule(String),
    InvalidDateRange(String),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxRuleLookupArgs {
    values: BTreeMap<String, String>,
}

impl TaxRuleLookupArgs {
    pub fn from_pairs<const N: usize>(pairs: [(&str, &str); N]) -> Self {
        Self {
            values: pairs
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values
            .get(key)
            .map(String::as_str)
            .filter(|value| !value.is_empty())
    }

    pub fn iter_without_tax_category(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .filter(|(key, _)| key.as_str() != "tax_category")
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

impl TaxRule {
    pub const DOCTYPE: &'static str = "Tax Rule";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-TAX-RULE-.YYYY.-.#####";
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const FIELD_ORDER: [&'static str; 32] = [
        "tax_type",
        "use_for_shopping_cart",
        "column_break_1",
        "sales_tax_template",
        "purchase_tax_template",
        "filters",
        "customer",
        "supplier",
        "item",
        "billing_city",
        "billing_county",
        "billing_state",
        "billing_zipcode",
        "billing_country",
        "tax_category",
        "column_break_2",
        "customer_group",
        "supplier_group",
        "item_group",
        "shipping_city",
        "shipping_county",
        "shipping_state",
        "shipping_zipcode",
        "shipping_country",
        "section_break_4",
        "from_date",
        "column_break_7",
        "to_date",
        "section_break_6",
        "priority",
        "column_break_20",
        "company",
    ];

    pub fn new_sales(sales_tax_template: impl Into<String>) -> Self {
        Self {
            tax_type: "Sales".to_string(),
            use_for_shopping_cart: true,
            sales_tax_template: Some(sales_tax_template.into()),
            priority: 1,
            company: Some("_Test Company".to_string()),
            ..Self::default()
        }
    }

    pub fn new_purchase(purchase_tax_template: impl Into<String>) -> Self {
        Self {
            tax_type: "Purchase".to_string(),
            use_for_shopping_cart: true,
            purchase_tax_template: Some(purchase_tax_template.into()),
            priority: 1,
            company: Some("_Test Company".to_string()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "tax_type" => FieldSpec::select("tax_type", "Tax Type")
                .options("Sales\nPurchase")
                .default("Sales")
                .in_list_view()
                .in_standard_filter(),
            "use_for_shopping_cart" => {
                FieldSpec::check("use_for_shopping_cart", "Use for Shopping Cart").default("1")
            }
            "sales_tax_template" => FieldSpec::link("sales_tax_template", "Sales Tax Template")
                .options("Sales Taxes and Charges Template")
                .depends_on("eval:doc.tax_type==\"Sales\""),
            "purchase_tax_template" => {
                FieldSpec::link("purchase_tax_template", "Purchase Tax Template")
                    .options("Purchase Taxes and Charges Template")
                    .depends_on("eval:doc.tax_type==\"Purchase\"")
            }
            "filters" => FieldSpec::section_break("filters").label("Filters"),
            "customer" => FieldSpec::link("customer", "Customer")
                .options("Customer")
                .depends_on("eval:doc.tax_type==\"Sales\""),
            "supplier" => FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .depends_on("eval:doc.tax_type==\"Purchase\""),
            "item" => FieldSpec::link("item", "Item").options("Item"),
            "billing_city" => FieldSpec::data("billing_city", "Billing City"),
            "billing_county" => FieldSpec::data("billing_county", "Billing County"),
            "billing_state" => FieldSpec::data("billing_state", "Billing State"),
            "billing_zipcode" => FieldSpec::data("billing_zipcode", "Billing Zipcode"),
            "billing_country" => {
                FieldSpec::link("billing_country", "Billing Country").options("Country")
            }
            "tax_category" => {
                FieldSpec::link("tax_category", "Tax Category").options("Tax Category")
            }
            "customer_group" => FieldSpec::link("customer_group", "Customer Group")
                .options("Customer Group")
                .fetch_from("customer.customer_group")
                .depends_on("eval:doc.tax_type==\"Sales\""),
            "supplier_group" => FieldSpec::link("supplier_group", "Supplier Group")
                .options("Supplier Group")
                .fetch_from("supplier.supplier_group")
                .depends_on("eval:doc.tax_type==\"Purchase\""),
            "item_group" => FieldSpec::link("item_group", "Item Group").options("Item Group"),
            "shipping_city" => FieldSpec::data("shipping_city", "Shipping City"),
            "shipping_county" => FieldSpec::data("shipping_county", "Shipping County"),
            "shipping_state" => FieldSpec::data("shipping_state", "Shipping State"),
            "shipping_zipcode" => FieldSpec::data("shipping_zipcode", "Shipping Zipcode"),
            "shipping_country" => {
                FieldSpec::link("shipping_country", "Shipping Country").options("Country")
            }
            "section_break_4" => FieldSpec::section_break("section_break_4").label("Validity"),
            "from_date" => FieldSpec::date("from_date", "From Date"),
            "to_date" => FieldSpec::date("to_date", "To Date"),
            "priority" => FieldSpec::int("priority", "Priority").default("1"),
            "company" => FieldSpec::link("company", "Company").options("Company"),
            field if field.starts_with("column_break") => FieldSpec::column_break(fieldname),
            field if field.starts_with("section_break") => FieldSpec::section_break(fieldname),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }

    pub fn validate_tax_template(&mut self) -> Result<(), TaxRuleError> {
        if self.tax_type == "Sales" {
            self.purchase_tax_template = None;
            self.supplier = None;
            self.supplier_group = None;
            if option_has_value(&self.customer) {
                self.customer_group = None;
            }
        } else {
            self.sales_tax_template = None;
            self.customer = None;
            self.customer_group = None;
            if option_has_value(&self.supplier) {
                self.supplier_group = None;
            }
        }

        if !option_has_value(&self.sales_tax_template)
            && !option_has_value(&self.purchase_tax_template)
        {
            return Err(TaxRuleError::MandatoryTaxTemplate(
                "Tax Template is mandatory.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_filters(&self, existing_rules: &[TaxRule]) -> Result<(), TaxRuleError> {
        for rule in existing_rules {
            if rule.name == self.name {
                continue;
            }
            if self.priority == rule.priority
                && self.filter_signature() == rule.filter_signature()
                && dates_overlap(self.from_date.as_deref(), self.to_date.as_deref(), rule)
            {
                return Err(TaxRuleError::ConflictingTaxRule(format!(
                    "Tax Rule Conflicts with {}",
                    rule.name
                )));
            }
        }
        Ok(())
    }

    fn filter_signature(&self) -> Vec<(&'static str, String)> {
        FILTER_FIELDS
            .iter()
            .map(|field| {
                (
                    *field,
                    self.value_for(field).unwrap_or_default().to_string(),
                )
            })
            .collect()
    }

    fn value_for(&self, field: &str) -> Option<&str> {
        match field {
            "tax_type" => non_empty(&self.tax_type),
            "customer" => option_str(&self.customer),
            "customer_group" => option_str(&self.customer_group),
            "supplier" => option_str(&self.supplier),
            "supplier_group" => option_str(&self.supplier_group),
            "item" => option_str(&self.item),
            "item_group" => option_str(&self.item_group),
            "billing_city" => option_str(&self.billing_city),
            "billing_county" => option_str(&self.billing_county),
            "billing_state" => option_str(&self.billing_state),
            "billing_zipcode" => option_str(&self.billing_zipcode),
            "billing_country" => option_str(&self.billing_country),
            "shipping_city" => option_str(&self.shipping_city),
            "shipping_county" => option_str(&self.shipping_county),
            "shipping_state" => option_str(&self.shipping_state),
            "shipping_zipcode" => option_str(&self.shipping_zipcode),
            "shipping_country" => option_str(&self.shipping_country),
            "tax_category" => option_str(&self.tax_category),
            "company" => option_str(&self.company),
            "use_for_shopping_cart" => Some(if self.use_for_shopping_cart { "1" } else { "0" }),
            _ => None,
        }
    }

    fn selected_template(&self) -> Option<&str> {
        option_str(&self.sales_tax_template).or_else(|| option_str(&self.purchase_tax_template))
    }
}

impl DocumentController for TaxRule {
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

const FILTER_FIELDS: [&str; 19] = [
    "tax_type",
    "customer",
    "customer_group",
    "supplier",
    "supplier_group",
    "item",
    "item_group",
    "billing_city",
    "billing_county",
    "billing_state",
    "billing_zipcode",
    "billing_country",
    "shipping_city",
    "shipping_county",
    "shipping_state",
    "shipping_zipcode",
    "shipping_country",
    "tax_category",
    "company",
];

pub fn get_tax_template(
    posting_date: Option<&str>,
    args: &TaxRuleLookupArgs,
    rules: &[TaxRule],
    disabled_templates: &BTreeSet<String>,
    customer_group_parents: &BTreeMap<String, Vec<String>>,
) -> Option<String> {
    let tax_category = args.get("tax_category").unwrap_or("");
    let mut matching_rules = rules
        .iter()
        .filter(|rule| posting_date_matches(posting_date, rule))
        .filter(|rule| rule.value_for("tax_category").unwrap_or("") == tax_category)
        .filter(|rule| lookup_args_match(rule, args, customer_group_parents))
        .collect::<Vec<_>>();

    if matching_rules.is_empty() {
        return None;
    }

    matching_rules.sort_by(|left, right| {
        let left_matches = no_of_keys_matched(left, args);
        let right_matches = no_of_keys_matched(right, args);
        right_matches
            .cmp(&left_matches)
            .then(right.priority.cmp(&left.priority))
    });

    let template = matching_rules[0].selected_template()?;
    if disabled_templates.contains(template) {
        None
    } else {
        Some(template.to_string())
    }
}

pub fn tax_rule_js_hooks() -> [(&'static str, &'static str); 2] {
    [
        (
            "customer",
            "erpnext.accounts.doctype.tax_rule.tax_rule.get_party_details",
        ),
        (
            "supplier",
            "erpnext.accounts.doctype.tax_rule.tax_rule.get_party_details",
        ),
    ]
}

fn lookup_args_match(
    rule: &TaxRule,
    args: &TaxRuleLookupArgs,
    customer_group_parents: &BTreeMap<String, Vec<String>>,
) -> bool {
    for (key, value) in args.iter_without_tax_category() {
        if key == "use_for_shopping_cart" {
            let expected = if value == "1" || value.eq_ignore_ascii_case("true") {
                "1"
            } else {
                "0"
            };
            if rule.value_for(key).unwrap_or("") != expected {
                return false;
            }
        } else if key == "customer_group" {
            let group = if value.is_empty() {
                "All Customer Groups"
            } else {
                value
            };
            let parents = customer_group_parents
                .get(group)
                .cloned()
                .unwrap_or_else(|| vec![group.to_string()]);
            let rule_value = rule.value_for(key).unwrap_or("");
            if !rule_value.is_empty() && !parents.iter().any(|parent| parent == rule_value) {
                return false;
            }
        } else {
            let rule_value = rule.value_for(key).unwrap_or("");
            if !rule_value.is_empty() && rule_value != value {
                return false;
            }
        }
    }
    true
}

fn no_of_keys_matched(rule: &TaxRule, args: &TaxRuleLookupArgs) -> usize {
    args.iter_without_tax_category()
        .filter(|(key, _)| rule.value_for(key).is_some())
        .count()
}

fn posting_date_matches(posting_date: Option<&str>, rule: &TaxRule) -> bool {
    if let Some(posting_date) = posting_date {
        option_str(&rule.from_date).map_or(true, |from| from <= posting_date)
            && option_str(&rule.to_date).map_or(true, |to| to >= posting_date)
    } else {
        !option_has_value(&rule.from_date) && !option_has_value(&rule.to_date)
    }
}

fn dates_overlap(from_date: Option<&str>, to_date: Option<&str>, rule: &TaxRule) -> bool {
    match (from_date, to_date) {
        (Some(from), Some(to)) => {
            option_str(&rule.from_date).is_some_and(|rule_from| rule_from > from && rule_from < to)
                || option_str(&rule.to_date).is_some_and(|rule_to| rule_to > from && rule_to < to)
                || (option_str(&rule.from_date).is_some_and(|rule_from| from > rule_from)
                    && option_str(&rule.to_date).is_some_and(|rule_to| from < rule_to))
                || (option_str(&rule.from_date) == Some(from)
                    && option_str(&rule.to_date) == Some(to))
        }
        (Some(from), None) => option_str(&rule.to_date).is_some_and(|rule_to| rule_to > from),
        (None, Some(to)) => option_str(&rule.from_date).is_some_and(|rule_from| rule_from < to),
        (None, None) => true,
    }
}

fn option_has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}

fn option_str(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|value| !value.is_empty())
}

fn non_empty(value: &str) -> Option<&str> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}
