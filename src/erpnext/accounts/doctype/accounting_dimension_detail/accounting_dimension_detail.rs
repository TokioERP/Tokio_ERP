use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimensionDetail {
    pub company: Option<String>,
    pub reference_document: Option<String>,
    pub default_dimension: Option<String>,
    pub mandatory_for_bs: bool,
    pub mandatory_for_pl: bool,
    pub automatically_post_balancing_accounting_entry: bool,
    pub offsetting_account: Option<String>,
}

impl AccountingDimensionDetail {
    pub const DOCTYPE: &'static str = "Accounting Dimension Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 8] = [
        "company",
        "reference_document",
        "default_dimension",
        "mandatory_for_bs",
        "mandatory_for_pl",
        "column_break_lqns",
        "automatically_post_balancing_accounting_entry",
        "offsetting_account",
    ];
    pub const IS_TABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        company: impl Into<String>,
        reference_document: impl Into<String>,
        default_dimension: impl Into<String>,
    ) -> Self {
        Self {
            company: Some(company.into()),
            reference_document: Some(reference_document.into()),
            default_dimension: Some(default_dimension.into()),
            mandatory_for_bs: false,
            mandatory_for_pl: false,
            automatically_post_balancing_accounting_entry: false,
            offsetting_account: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .columns(2)
                .in_list_view(),
            FieldSpec::link("reference_document", "Reference Document")
                .options("DocType")
                .hidden()
                .read_only(),
            FieldSpec::dynamic_link("default_dimension")
                .label("Default Dimension")
                .options("reference_document")
                .columns(2)
                .in_list_view(),
            FieldSpec::check("mandatory_for_bs", "Mandatory For Balance Sheet")
                .default("0")
                .columns(3)
                .in_list_view(),
            FieldSpec::check("mandatory_for_pl", "Mandatory For Profit and Loss Account")
                .default("0")
                .columns(3)
                .in_list_view(),
            FieldSpec::check(
                "automatically_post_balancing_accounting_entry",
                "Automatically post balancing accounting entry",
            )
            .default("0"),
            FieldSpec::link("offsetting_account", "Offsetting Account")
                .options("Account")
                .mandatory_depends_on("eval: doc.automatically_post_balancing_accounting_entry"),
            FieldSpec::column_break("column_break_lqns"),
        ]
    }
}

impl DocumentController for AccountingDimensionDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
