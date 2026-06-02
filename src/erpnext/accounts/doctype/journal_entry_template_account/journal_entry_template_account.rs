use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct JournalEntryTemplateAccount {
    pub account: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
}

impl JournalEntryTemplateAccount {
    pub const DOCTYPE: &'static str = "Journal Entry Template Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "account",
        "party_type",
        "party",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
    ];
    pub const IS_TABLE: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(account: impl Into<String>) -> Self {
        Self {
            account: Some(account.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .in_list_view(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .in_list_view(),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions")
                .collapsible(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::link("project", "Project").options("Project"),
        ]
    }
}

impl DocumentController for JournalEntryTemplateAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
