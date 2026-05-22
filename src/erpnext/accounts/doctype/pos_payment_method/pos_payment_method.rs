use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosPaymentMethod {
    pub default: bool,
    pub allow_in_returns: bool,
    pub mode_of_payment: Option<String>,
}

impl PosPaymentMethod {
    pub const DOCTYPE: &'static str = "POS Payment Method";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["default", "allow_in_returns", "mode_of_payment"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(mode_of_payment: impl Into<String>) -> Self {
        Self {
            default: false,
            allow_in_returns: false,
            mode_of_payment: Some(mode_of_payment.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::check("default", "Default")
                .in_list_view()
                .default("0")
                .depends_on("eval:parent.doctype == 'POS Profile'"),
            FieldSpec::check("allow_in_returns", "Allow In Returns")
                .in_list_view()
                .default("0"),
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
        ]
    }
}

impl DocumentController for PosPaymentMethod {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
