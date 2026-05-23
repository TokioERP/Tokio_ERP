use crate::erpnext::accounts::doctype::shipping_rule_condition::shipping_rule_condition::ShippingRuleCondition;
use crate::erpnext::accounts::doctype::shipping_rule_country::shipping_rule_country::ShippingRuleCountry;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShippingRule {
    pub label: String,
    pub disabled: bool,
    pub shipping_rule_type: String,
    pub company: String,
    pub account: String,
    pub cost_center: String,
    pub project: Option<String>,
    pub calculate_based_on: String,
    pub shipping_amount: f64,
    pub conditions: Vec<ShippingRuleCondition>,
    pub countries: Vec<ShippingRuleCountry>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShippingRuleError {
    FromGreaterThanTo(String),
    ManyBlankToValues(String),
    NegativeValue(String),
    OverlappingConditions(Vec<String>),
    Country(String),
    WrongTaxTable(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShippingRuleDocumentContext {
    pub has_shipping_address: bool,
    pub shipping_country: Option<String>,
    pub base_net_total: f64,
    pub total_net_weight: f64,
    pub currency: String,
    pub company_currency: String,
    pub conversion_rate: f64,
    pub taxes_options: String,
    pub has_stock_items: bool,
    pub has_asset_items: bool,
    pub existing_shipping_charge: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaxChargeDraft {
    pub doctype: String,
    pub charge_type: String,
    pub account_head: String,
    pub cost_center: String,
    pub tax_amount: f64,
    pub description: Option<String>,
    pub category: Option<String>,
    pub add_deduct_tax: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShippingRuleApplication {
    Append(TaxChargeDraft),
    UpdateExisting(TaxChargeDraft),
}

impl ShippingRuleApplication {
    pub fn tax_amount(&self) -> f64 {
        match self {
            Self::Append(charge) | Self::UpdateExisting(charge) => charge.tax_amount,
        }
    }
}

const PRE_SALES_ITEMS: [&str; 2] = ["Quotation", "Supplier Quotation"];
const SALES_ITEMS: [&str; 3] = ["Sales Order", "Delivery Note", "Sales Invoice"];
const PURCHASE_ITEMS: [&str; 3] = ["Purchase Invoice", "Purchase Order", "Purchase Receipt"];

impl ShippingRule {
    pub const DOCTYPE: &'static str = "Shipping Rule";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:label";
    pub const ALLOW_IMPORT: bool = true;
    pub const FIELD_ORDER: [&'static str; 20] = [
        "label",
        "disabled",
        "column_break_4",
        "shipping_rule_type",
        "section_break_10",
        "company",
        "column_break_12",
        "account",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "shipping_amount_section",
        "calculate_based_on",
        "column_break_8",
        "shipping_amount",
        "rule_conditions_section",
        "conditions",
        "section_break_6",
        "countries",
    ];

    pub fn new(
        label: impl Into<String>,
        shipping_rule_type: impl Into<String>,
        company: impl Into<String>,
        account: impl Into<String>,
        cost_center: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            shipping_rule_type: shipping_rule_type.into(),
            company: company.into(),
            account: account.into(),
            cost_center: cost_center.into(),
            calculate_based_on: "Fixed".to_string(),
            ..Self::default()
        }
    }

    pub fn new_net_total(
        label: impl Into<String>,
        shipping_rule_type: impl Into<String>,
        company: impl Into<String>,
        account: impl Into<String>,
        cost_center: impl Into<String>,
    ) -> Self {
        let mut rule = Self::new(label, shipping_rule_type, company, account, cost_center);
        rule.calculate_based_on = "Net Total".to_string();
        rule
    }

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "label" => FieldSpec::data("label", "Shipping Rule Label")
                .description("example: Next Day Shipping")
                .required()
                .unique(),
            "disabled" => FieldSpec::check("disabled", "Disabled").default("0"),
            "shipping_rule_type" => FieldSpec::select("shipping_rule_type", "Shipping Rule Type")
                .options("Selling\nBuying"),
            "section_break_10" => FieldSpec::section_break("section_break_10")
                .label("Accounting")
                .depends_on("eval:!doc.disabled"),
            "company" => FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            "account" => FieldSpec::link("account", "Shipping Account")
                .options("Account")
                .required(),
            "accounting_dimensions_section" => {
                FieldSpec::section_break("accounting_dimensions_section")
                    .label("Accounting Dimensions")
            }
            "cost_center" => FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .required(),
            "project" => FieldSpec::link("project", "Project").options("Project"),
            "calculate_based_on" => FieldSpec::select("calculate_based_on", "Calculate Based On")
                .options("Fixed\nNet Total\nNet Weight")
                .default("Fixed")
                .in_list_view()
                .in_standard_filter(),
            "shipping_amount" => FieldSpec::currency("shipping_amount", "Shipping Amount")
                .depends_on("eval:doc.calculate_based_on==='Fixed'"),
            "rule_conditions_section" => FieldSpec::section_break("rule_conditions_section")
                .label("Shipping Rule Conditions")
                .depends_on("eval:doc.calculate_based_on!=='Fixed'"),
            "conditions" => FieldSpec::table("conditions", "Shipping Rule Conditions")
                .options("Shipping Rule Condition"),
            "section_break_6" => {
                FieldSpec::section_break("section_break_6").label("Restrict to Countries")
            }
            "countries" => FieldSpec::table("countries", "Valid for Countries")
                .options("Shipping Rule Country"),
            "column_break_4" | "column_break_12" | "dimension_col_break" | "column_break_8" => {
                FieldSpec::column_break(fieldname)
            }
            "shipping_amount_section" => FieldSpec::section_break("shipping_amount_section"),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }

    pub fn validate(&mut self) -> Result<(), ShippingRuleError> {
        self.validate_from_to_values()?;
        self.sort_shipping_rule_conditions();
        self.validate_overlapping_shipping_rule_conditions()
    }

    fn validate_from_to_values(&mut self) -> Result<(), ShippingRuleError> {
        if self.calculate_based_on == "Fixed" {
            self.conditions.clear();
            return Ok(());
        }

        let mut zero_to_values = 0usize;
        for (index, condition) in self.conditions.iter_mut().enumerate() {
            if condition.idx == 0 {
                condition.idx = index + 1;
            }
            if condition.from_value < 0.0 {
                return Err(ShippingRuleError::NegativeValue(
                    "From Value cannot be less than 0.0".to_string(),
                ));
            }
            if condition.to_value < 0.0 {
                return Err(ShippingRuleError::NegativeValue(
                    "To Value cannot be less than 0.0".to_string(),
                ));
            }
            if condition.to_value == 0.0 {
                zero_to_values += 1;
            } else if condition.from_value >= condition.to_value {
                return Err(ShippingRuleError::FromGreaterThanTo(format!(
                    "From value must be less than to value in row {}",
                    condition.idx
                )));
            }
        }

        if zero_to_values >= 2 {
            return Err(ShippingRuleError::ManyBlankToValues(
                "There can only be one Shipping Rule Condition with 0 or blank value for \"To Value\""
                    .to_string(),
            ));
        }
        Ok(())
    }

    fn sort_shipping_rule_conditions(&mut self) {
        for (index, condition) in self.conditions.iter_mut().enumerate() {
            condition.idx = index + 1;
        }
    }

    fn validate_overlapping_shipping_rule_conditions(&self) -> Result<(), ShippingRuleError> {
        let mut overlaps = Vec::new();
        for i in 0..self.conditions.len() {
            for j in (i + 1)..self.conditions.len() {
                let first = &self.conditions[i];
                let second = &self.conditions[j];
                if first == second {
                    continue;
                }
                let range_a = (
                    first.from_value,
                    if first.to_value == 0.0 {
                        first.from_value
                    } else {
                        first.to_value
                    },
                );
                let range_b = (
                    second.from_value,
                    if second.to_value == 0.0 {
                        second.from_value
                    } else {
                        second.to_value
                    },
                );
                if ranges_overlap(range_a, range_b) {
                    overlaps.push(format!(
                        "{}-{} = {} and {}-{} = {}",
                        format_number(first.from_value),
                        format_number(first.to_value),
                        format_money(first.shipping_amount),
                        format_number(second.from_value),
                        format_number(second.to_value),
                        format_money(second.shipping_amount)
                    ));
                }
            }
        }

        if overlaps.is_empty() {
            Ok(())
        } else {
            Err(ShippingRuleError::OverlappingConditions(overlaps))
        }
    }

    pub fn apply(
        &self,
        context: &ShippingRuleDocumentContext,
    ) -> Result<ShippingRuleApplication, ShippingRuleError> {
        if context.has_shipping_address {
            self.validate_countries(context)?;
        }

        let mut shipping_amount = match self.calculate_based_on.as_str() {
            "Net Total" => self.get_shipping_amount_from_rules(context.base_net_total),
            "Net Weight" => self.get_shipping_amount_from_rules(context.total_net_weight),
            "Fixed" => self.shipping_amount,
            _ => 0.0,
        };

        if context.currency != context.company_currency {
            shipping_amount = round_to_precision(shipping_amount / context.conversion_rate, 2);
        }

        self.add_shipping_rule_to_tax_table(context, shipping_amount)
    }

    pub fn get_shipping_amount_from_rules(&self, value: f64) -> f64 {
        for condition in &self.conditions {
            if condition.to_value == 0.0
                || (condition.from_value <= value && value <= condition.to_value)
            {
                return condition.shipping_amount;
            }
        }
        0.0
    }

    fn validate_countries(
        &self,
        context: &ShippingRuleDocumentContext,
    ) -> Result<(), ShippingRuleError> {
        if self.countries.is_empty() {
            return Ok(());
        }

        let Some(shipping_country) = context.shipping_country.as_deref() else {
            return Err(ShippingRuleError::Country(
                "Shipping Address does not have country, which is required for this Shipping Rule"
                    .to_string(),
            ));
        };

        let allowed = self
            .countries
            .iter()
            .any(|row| row.country.as_deref() == Some(shipping_country));
        if allowed {
            Ok(())
        } else {
            Err(ShippingRuleError::Country(format!(
                "Shipping rule not applicable for country {} in Shipping Address",
                shipping_country
            )))
        }
    }

    fn add_shipping_rule_to_tax_table(
        &self,
        context: &ShippingRuleDocumentContext,
        shipping_amount: f64,
    ) -> Result<ShippingRuleApplication, ShippingRuleError> {
        let mut charge = TaxChargeDraft {
            doctype: String::new(),
            charge_type: "Actual".to_string(),
            account_head: self.account.clone(),
            cost_center: self.cost_center.clone(),
            tax_amount: shipping_amount,
            description: None,
            category: None,
            add_deduct_tax: None,
        };

        if self.shipping_rule_type == "Selling" {
            if context.taxes_options != "Sales Taxes and Charges" {
                return Err(ShippingRuleError::WrongTaxTable(
                    "Shipping rule only applicable for Selling".to_string(),
                ));
            }
            charge.doctype = "Sales Taxes and Charges".to_string();
        } else {
            if context.taxes_options != "Purchase Taxes and Charges" {
                return Err(ShippingRuleError::WrongTaxTable(
                    "Shipping rule only applicable for Buying".to_string(),
                ));
            }
            charge.doctype = "Purchase Taxes and Charges".to_string();
            charge.category = Some(
                if context.has_stock_items || context.has_asset_items {
                    "Valuation and Total"
                } else {
                    "Total"
                }
                .to_string(),
            );
            charge.add_deduct_tax = Some("Add".to_string());
        }

        if context.existing_shipping_charge {
            Ok(ShippingRuleApplication::UpdateExisting(charge))
        } else {
            charge.description = Some(self.label.clone());
            Ok(ShippingRuleApplication::Append(charge))
        }
    }
}

impl DocumentController for ShippingRule {
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

pub fn shipping_rule_js_hooks() -> [&'static str; 5] {
    [
        "onload",
        "company",
        "refresh",
        "calculate_based_on",
        "toggle_reqd",
    ]
}

pub fn shipping_rule_dashboard_groups() -> [(&'static str, &'static [&'static str]); 3] {
    [
        ("Pre Sales", &PRE_SALES_ITEMS),
        ("Sales", &SALES_ITEMS),
        ("Purchase", &PURCHASE_ITEMS),
    ]
}

fn ranges_overlap(first: (f64, f64), second: (f64, f64)) -> bool {
    let (x1, x2) = first;
    let (y1, y2) = second;
    let separate = (x1 <= x2 && x2 <= y1 && y1 <= y2) || (y1 <= y2 && y2 <= x1 && x1 <= x2);
    !separate
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn format_money(value: f64) -> String {
    format!("{:.2}", value)
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        value.to_string()
    }
}
