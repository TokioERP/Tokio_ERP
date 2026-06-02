use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoyaltyProgramCollection {
    pub tier_name: Option<String>,
    pub min_spent: f64,
    pub collection_factor: f64,
}

impl LoyaltyProgramCollection {
    pub const DOCTYPE: &'static str = "Loyalty Program Collection";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] = [
        "tier_name",
        "min_spent",
        "column_break_3",
        "collection_factor",
    ];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("tier_name", "Tier Name")
                .required()
                .columns(3)
                .in_list_view(),
            FieldSpec::currency("min_spent", "Minimum Total Spent")
                .columns(3)
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::currency("collection_factor", "Collection Factor (=1 LP)")
                .required()
                .columns(3)
                .description("For how much spent = 1 Loyalty Point")
                .in_list_view(),
        ]
    }
}

impl DocumentController for LoyaltyProgramCollection {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
