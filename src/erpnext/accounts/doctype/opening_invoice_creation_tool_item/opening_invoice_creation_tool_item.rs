use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpeningInvoiceCreationToolItem;

impl OpeningInvoiceCreationToolItem {
    pub const DOCTYPE: &'static str = "Opening Invoice Creation Tool Item";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 17] = [
        "invoice_number",
        "party_type",
        "party",
        "party_name",
        "temporary_opening_account",
        "column_break_3",
        "posting_date",
        "due_date",
        "supplier_invoice_date",
        "section_break_5",
        "item_name",
        "outstanding_amount",
        "column_break_4",
        "qty",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .hidden()
                .read_only(),
            FieldSpec::dynamic_link("party")
                .label("Party ID")
                .options("party_type")
                .mandatory_depends_on("eval: !parent.create_missing_party")
                .in_list_view(),
            FieldSpec::link("temporary_opening_account", "Temporary Opening Account")
                .options("Account"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view(),
            FieldSpec::date("due_date", "Due Date")
                .default("Today")
                .in_list_view(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::data("item_name", "Item Name")
                .default("Opening Invoice Item")
                .in_list_view(),
            FieldSpec::currency("outstanding_amount", "Outstanding Amount")
                .default("0")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::data("qty", "Quantity").default("1"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::data("invoice_number", "Invoice Number")
                .description("Reference number of the invoice from the previous system"),
            FieldSpec::date("supplier_invoice_date", "Supplier Invoice Date")
                .depends_on("eval: parent.invoice_type == \"Purchase\""),
            FieldSpec::data("party_name", "Party Name").in_list_view(),
        ]
    }
}

impl DocumentController for OpeningInvoiceCreationToolItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
