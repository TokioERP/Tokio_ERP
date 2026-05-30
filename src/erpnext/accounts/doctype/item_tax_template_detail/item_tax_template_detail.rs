use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemTaxTemplateDetail {
    pub tax_type: Option<String>,
    pub tax_rate: f64,
    pub not_applicable: bool,
}

impl ItemTaxTemplateDetail {
    pub const DOCTYPE: &'static str = "Item Tax Template Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["tax_type", "tax_rate", "not_applicable"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("tax_type", "Tax")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::float("tax_rate", "Tax Rate").in_list_view(),
            FieldSpec::check("not_applicable", "Not Applicable")
                .default("0")
                .in_list_view(),
        ]
    }
}

impl DocumentController for ItemTaxTemplateDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
