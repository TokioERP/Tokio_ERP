use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShippingRuleCondition {
    pub from_value: f64,
    pub to_value: f64,
    pub shipping_amount: f64,
    pub idx: usize,
}

impl ShippingRuleCondition {
    pub const DOCTYPE: &'static str = "Shipping Rule Condition";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["from_value", "to_value", "shipping_amount"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(from_value: f64, to_value: f64, shipping_amount: f64) -> Self {
        Self {
            from_value,
            to_value,
            shipping_amount,
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
            "from_value" => FieldSpec::float("from_value", "From Value")
                .required()
                .in_list_view(),
            "to_value" => FieldSpec::float("to_value", "To Value").in_list_view(),
            "shipping_amount" => FieldSpec::currency("shipping_amount", "Shipping Amount")
                .options("Company:company:default_currency")
                .required()
                .in_list_view(),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }
}

impl DocumentController for ShippingRuleCondition {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
