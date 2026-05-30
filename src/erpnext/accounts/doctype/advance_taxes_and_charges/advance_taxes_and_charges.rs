use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AdvanceTaxesAndCharges {
    pub account_head: Option<String>,
    pub add_deduct_tax: Option<String>,
    pub charge_type: Option<String>,
    pub description: Option<String>,
    pub included_in_paid_amount: bool,
    pub is_tax_withholding_account: bool,
    pub set_by_item_tax_template: bool,
}

impl AdvanceTaxesAndCharges {
    pub const DOCTYPE: &'static str = "Advance Taxes and Charges";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 24] = [
        "add_deduct_tax",
        "charge_type",
        "row_id",
        "account_head",
        "col_break_1",
        "description",
        "included_in_paid_amount",
        "set_by_item_tax_template",
        "is_tax_withholding_account",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "section_break_8",
        "rate",
        "section_break_9",
        "currency",
        "net_amount",
        "tax_amount",
        "total",
        "column_break_13",
        "base_tax_amount",
        "base_net_amount",
        "base_total",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("charge_type", "Type")
                .options("\nActual\nOn Paid Amount\nOn Previous Row Amount\nOn Previous Row Total")
                .oldfield("charge_type", "Select")
                .columns(2)
                .required()
                .in_list_view(),
            FieldSpec::data("row_id", "Reference Row #")
                .oldfield("row_id", "Data")
                .depends_on(
                    "eval:[\"On Previous Row Amount\", \"On Previous Row Total\"].indexOf(doc.charge_type)!==-1"
                ),
            FieldSpec::link("account_head", "Account Head")
                .options("Account")
                .oldfield("account_head", "Link")
                .columns(2)
                .required()
                .in_list_view()
                .search_index(),
            FieldSpec::column_break("col_break_1").width("50%"),
            FieldSpec::small_text("description", "Description")
                .oldfield("description", "Small Text")
                .width("300px")
                .required(),
            FieldSpec::section_break("accounting_dimensions_section").label("Accounting Dimensions"),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .oldfield("cost_center_other_charges", "Link")
                .default(":Company"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::link("project", "Project")
                .options("Project")
                .allow_on_submit(),
            FieldSpec::section_break("section_break_8"),
            FieldSpec::float("rate", "Tax Rate")
                .oldfield("rate", "Currency")
                .columns(2)
                .in_list_view(),
            FieldSpec::section_break("section_break_9"),
            FieldSpec::currency("tax_amount", "Amount")
                .options("currency")
                .columns(2)
                .in_list_view(),
            FieldSpec::currency("total", "Total")
                .options("currency")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::currency("base_tax_amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .oldfield("tax_amount", "Currency")
                .read_only(),
            FieldSpec::currency("base_total", "Total (Company Currency)")
                .options("Company:company:default_currency")
                .oldfield("total", "Currency")
                .read_only(),
            FieldSpec::select("add_deduct_tax", "Add Or Deduct")
                .options("Add\nDeduct")
                .required(),
            FieldSpec::check("included_in_paid_amount", "Considered In Paid Amount").default("0"),
            FieldSpec::link("currency", "Account Currency")
                .options("Currency")
                .fetch_from("account_head.account_currency")
                .read_only(),
            FieldSpec::currency("net_amount", "Net Amount")
                .options("currency")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::currency("base_net_amount", "Net Amount (Company Currency)")
                .options("Company:company:default_currency")
                .oldfield("tax_amount", "Currency")
                .read_only(),
            FieldSpec::check("set_by_item_tax_template", "Set by Item Tax Template")
                .default("0")
                .read_only()
                .hidden()
                .print_hide()
                .report_hide(),
            FieldSpec::check("is_tax_withholding_account", "Is Tax Withholding Account")
                .default("0")
                .read_only(),
        ]
    }
}

impl DocumentController for AdvanceTaxesAndCharges {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
