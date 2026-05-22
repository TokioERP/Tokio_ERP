use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ApplicableOnAccount {
    pub applicable_on_account: Option<String>,
    pub is_mandatory: bool,
}

impl ApplicableOnAccount {
    pub const DOCTYPE: &'static str = "Applicable On Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["applicable_on_account", "is_mandatory"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(applicable_on_account: impl Into<String>, is_mandatory: bool) -> Self {
        Self {
            applicable_on_account: Some(applicable_on_account.into()),
            is_mandatory,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("applicable_on_account", "Accounts")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::check("is_mandatory", "Is Mandatory")
                .default("0")
                .columns(2)
                .in_list_view(),
        ]
    }
}

impl DocumentController for ApplicableOnAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
