use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PricingRuleItemGroup {
    pub item_group: Option<String>,
    pub uom: Option<String>,
}

impl PricingRuleItemGroup {
    pub const DOCTYPE: &'static str = "Pricing Rule Item Group";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["item_group", "uom"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";

    pub fn new(item_group: impl Into<String>, uom: impl Into<String>) -> Self {
        Self {
            item_group: Some(item_group.into()),
            uom: Some(uom.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("item_group", "Item Group")
                .options("Item Group")
                .in_list_view()
                .search_index()
                .depends_on("eval:parent.apply_on == 'Item Group'"),
            FieldSpec::link("uom", "UOM").options("UOM").in_list_view(),
        ]
    }
}

impl DocumentController for PricingRuleItemGroup {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
