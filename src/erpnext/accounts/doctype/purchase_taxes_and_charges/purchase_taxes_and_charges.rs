use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PurchaseTaxesAndCharges {
    pub account_head: Option<String>,
    pub charge_type: Option<String>,
    pub rate: Option<String>,
}

impl PurchaseTaxesAndCharges {
    pub const DOCTYPE: &'static str = "Purchase Taxes and Charges";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "category",
        "add_deduct_tax",
        "charge_type",
        "row_id",
        "included_in_print_rate",
        "included_in_paid_amount",
        "col_break1",
        "account_head",
        "description",
        "is_tax_withholding_account",
        "set_by_item_tax_template",
        "section_break_10",
        "rate",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "section_break_9",
        "account_currency",
        "net_amount",
        "tax_amount",
        "tax_amount_after_discount_amount",
        "total",
        "column_break_14",
        "base_net_amount",
        "base_tax_amount",
        "base_total",
        "base_tax_amount_after_discount_amount",
        "dont_recompute_tax",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

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
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "category" => FieldSpec::select("category", "Consider Tax or Charge for")
                .options("Valuation and Total\nValuation\nTotal")
                .default("Total")
                .required(),
            "add_deduct_tax" => FieldSpec::select("add_deduct_tax", "Add or Deduct")
                .options("Add\nDeduct")
                .default("Add")
                .required(),
            "charge_type" => FieldSpec::select("charge_type", "Type")
                .options(
                    "\nActual\nOn Net Total\nOn Previous Row Amount\nOn Previous Row Total\nOn Item Quantity",
                )
                .default("On Net Total")
                .required()
                .in_list_view()
                .columns(2),
            "account_head" => FieldSpec::link("account_head", "Account Head")
                .options("Account")
                .required()
                .in_list_view()
                .columns(2),
            "tax_amount" => FieldSpec::currency("tax_amount", "Amount")
                .options("currency")
                .in_list_view()
                .columns(2),
            "col_break1" | "column_break_14" | "dimension_col_break" => {
                FieldSpec::column_break(fieldname)
            }
            field if field.contains("section") => FieldSpec::section_break(fieldname),
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }
}

impl DocumentController for PurchaseTaxesAndCharges {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
