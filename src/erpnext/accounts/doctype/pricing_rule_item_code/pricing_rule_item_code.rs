use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleItemCode {
    pub item_code: Option<String>,
    pub uom: Option<String>,
}

impl PricingRuleItemCode {
    pub const DOCTYPE: &'static str = "Pricing Rule Item Code";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["item_code", "uom"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(item_code: impl Into<String>, uom: impl Into<String>) -> Self {
        Self {
            item_code: Some(item_code.into()),
            uom: Some(uom.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("item_code", "Item Code")
                .options("Item")
                .in_list_view()
                .search_index()
                .depends_on("eval:parent.apply_on == 'Item Code'"),
            FieldSpec::link("uom", "UOM").options("UOM").in_list_view(),
        ]
    }
}

impl DocumentController for PricingRuleItemCode {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
