use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosItemGroup {
    pub item_group: Option<String>,
}

impl PosItemGroup {
    pub const DOCTYPE: &'static str = "POS Item Group";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["item_group"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(item_group: impl Into<String>) -> Self {
        Self {
            item_group: Some(item_group.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("item_group", "Item Group")
            .options("Item Group")
            .required()
            .in_list_view()]
    }
}

impl DocumentController for PosItemGroup {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
