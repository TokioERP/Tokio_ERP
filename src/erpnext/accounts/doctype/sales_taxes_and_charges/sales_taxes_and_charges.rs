use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesTaxesAndCharges {
    pub account_head: Option<String>,
    pub charge_type: Option<String>,
    pub rate: Option<String>,
}

impl SalesTaxesAndCharges {
    pub const DOCTYPE: &'static str = "Sales Taxes and Charges";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "charge_type",
        "row_id",
        "account_head",
        "col_break_1",
        "description",
        "included_in_print_rate",
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
        "account_currency",
        "net_amount",
        "tax_amount",
        "total",
        "tax_amount_after_discount_amount",
        "column_break_13",
        "base_net_amount",
        "base_tax_amount",
        "base_total",
        "base_tax_amount_after_discount_amount",
        "dont_recompute_tax",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(charge_type: impl Into<String>, account_head: impl Into<String>) -> Self {
        Self {
            charge_type: Some(charge_type.into()),
            account_head: Some(account_head.into()),
            rate: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|fieldname| Self::field_for(fieldname))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "charge_type" => FieldSpec::select("charge_type", "Type")
                .options(
                    "\nActual\nOn Net Total\nOn Previous Row Amount\nOn Previous Row Total\nOn Item Quantity",
                )
                .required()
                .in_list_view()
                .columns(2)
                .oldfield("charge_type", "Select"),
            "row_id" => FieldSpec::data("row_id", "Reference Row #")
                .depends_on(
                    "eval:[\"On Previous Row Amount\", \"On Previous Row Total\"].indexOf(doc.charge_type)!==-1",
                )
                .oldfield("row_id", "Data"),
            "account_head" => FieldSpec::link("account_head", "Account Head")
                .options("Account")
                .required()
                .in_list_view()
                .columns(2)
                .search_index()
                .allow_on_submit()
                .oldfield("account_head", "Link"),
            "description" => FieldSpec::small_text("description", "Description")
                .required()
                .oldfield("description", "Small Text")
                .width("300px"),
            "included_in_print_rate" => {
                FieldSpec::check("included_in_print_rate", "Is this Tax included in Basic Rate?")
                    .default("0")
                    .description(
                        "If checked, the tax amount will be considered as already included in the Print Rate / Print Amount",
                    )
                    .print_hide()
                    .report_hide()
                    .width("150px")
            }
            "included_in_paid_amount" => FieldSpec::check(
                "included_in_paid_amount",
                "Considered In Paid Amount",
            )
            .default("0")
            .depends_on("eval:['Sales Taxes and Charges Template', 'Payment Entry'].includes(parent.doctype)")
            .description(
                "If checked, the tax amount will be considered as already included in the Paid Amount in Payment Entry",
            ),
            "set_by_item_tax_template" => {
                FieldSpec::check("set_by_item_tax_template", "Set by Item Tax Template")
                    .default("0")
                    .hidden()
                    .print_hide()
                    .read_only()
                    .report_hide()
            }
            "is_tax_withholding_account" => {
                FieldSpec::check("is_tax_withholding_account", "Is Tax Withholding Account")
                    .default("0")
                    .read_only()
            }
            "cost_center" => FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .default(":Company")
                .allow_on_submit()
                .oldfield("cost_center_other_charges", "Link"),
            "project" => FieldSpec::link("project", "Project")
                .options("Project")
                .allow_on_submit(),
            "rate" => FieldSpec::float("rate", "Tax Rate")
                .in_list_view()
                .columns(2)
                .oldfield("rate", "Currency"),
            "account_currency" => FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .fetch_from("account_head.account_currency")
                .read_only(),
            "net_amount" => FieldSpec::currency("net_amount", "Net Amount")
                .options("currency")
                .read_only()
                .in_list_view()
                .columns(2),
            "tax_amount" => FieldSpec::currency("tax_amount", "Amount")
                .options("currency")
                .in_list_view()
                .columns(2),
            "total" => FieldSpec::currency("total", "Total")
                .options("currency")
                .read_only()
                .in_list_view()
                .columns(2),
            "tax_amount_after_discount_amount" => {
                FieldSpec::currency("tax_amount_after_discount_amount", "Tax Amount After Discount Amount")
                    .options("currency")
                    .read_only()
            }
            "base_net_amount" => FieldSpec::currency(
                "base_net_amount",
                "Net Amount (Company Currency)",
            )
            .options("Company:company:default_currency")
            .read_only()
            .oldfield("tax_amount", "Currency"),
            "base_tax_amount" => {
                FieldSpec::currency("base_tax_amount", "Amount (Company Currency)")
                    .options("Company:company:default_currency")
                    .read_only()
                    .oldfield("tax_amount", "Currency")
            }
            "base_total" => FieldSpec::currency("base_total", "Total (Company Currency)")
                .options("Company:company:default_currency")
                .read_only()
                .oldfield("total", "Currency"),
            "base_tax_amount_after_discount_amount" => FieldSpec::currency(
                "base_tax_amount_after_discount_amount",
                "Tax Amount After Discount Amount (Company Currency)",
            )
            .options("Company:company:default_currency")
            .read_only()
            .depends_on("eval:parent.discount_amount"),
            "dont_recompute_tax" => FieldSpec::check("dont_recompute_tax", "Don't Recompute Tax")
                .default("0")
                .hidden()
                .print_hide()
                .read_only(),
            "col_break_1" | "dimension_col_break" | "column_break_13" => {
                FieldSpec::column_break(fieldname)
            }
            "accounting_dimensions_section" => {
                FieldSpec::section_break("accounting_dimensions_section")
                    .label("Accounting Dimensions")
            }
            "section_break_8" | "section_break_9" => FieldSpec::section_break(fieldname),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }
}

impl DocumentController for SalesTaxesAndCharges {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
