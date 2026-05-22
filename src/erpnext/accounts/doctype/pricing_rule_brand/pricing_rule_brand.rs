use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleBrand {
    pub brand: Option<String>,
    pub uom: Option<String>,
}

impl PricingRuleBrand {
    pub const DOCTYPE: &'static str = "Pricing Rule Brand";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["brand", "uom"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(brand: impl Into<String>, uom: impl Into<String>) -> Self {
        Self {
            brand: Some(brand.into()),
            uom: Some(uom.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("brand", "Brand")
                .options("Brand")
                .in_list_view()
                .depends_on("eval:parent.apply_on == 'Brand'"),
            FieldSpec::link("uom", "UOM").options("UOM").in_list_view(),
        ]
    }
}

impl DocumentController for PricingRuleBrand {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
