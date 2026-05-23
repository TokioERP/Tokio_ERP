use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesInvoiceAdvance {
    pub reference_type: Option<String>,
    pub reference_name: Option<String>,
    pub remarks: Option<String>,
    pub reference_row: Option<String>,
    pub advance_amount: Option<String>,
    pub allocated_amount: Option<String>,
    pub exchange_gain_loss: Option<String>,
    pub ref_exchange_rate: Option<String>,
    pub difference_posting_date: Option<String>,
}

impl SalesInvoiceAdvance {
    pub const DOCTYPE: &'static str = "Sales Invoice Advance";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 10] = [
        "reference_type",
        "reference_name",
        "remarks",
        "reference_row",
        "col_break1",
        "advance_amount",
        "allocated_amount",
        "exchange_gain_loss",
        "ref_exchange_rate",
        "difference_posting_date",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const QUICK_ENTRY: bool = true;

    pub fn new(reference_type: impl Into<String>, reference_name: impl Into<String>) -> Self {
        Self {
            reference_type: Some(reference_type.into()),
            reference_name: Some(reference_name.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .read_only()
                .no_copy()
                .oldfield("journal_voucher", "Link")
                .width("250px"),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .read_only()
                .in_list_view()
                .columns(2)
                .print_hide()
                .no_copy(),
            FieldSpec::text("remarks", "Remarks")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy()
                .oldfield("remarks", "Small Text")
                .width("150px"),
            FieldSpec::data("reference_row", "Reference Row")
                .read_only()
                .hidden()
                .print_hide()
                .no_copy()
                .oldfield("jv_detail_no", "Data")
                .width("120px"),
            FieldSpec::column_break("col_break1"),
            FieldSpec::currency("advance_amount", "Advance amount")
                .options("party_account_currency")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy()
                .oldfield("advance_amount", "Currency")
                .width("120px"),
            FieldSpec::currency("allocated_amount", "Allocated amount")
                .options("party_account_currency")
                .in_list_view()
                .columns(2)
                .no_copy()
                .oldfield("allocated_amount", "Currency")
                .width("120px"),
            FieldSpec::currency("exchange_gain_loss", "Exchange Gain/Loss")
                .options("Company:company:default_currency")
                .read_only()
                .depends_on("exchange_gain_loss"),
            FieldSpec::float("ref_exchange_rate", "Reference Exchange Rate")
                .read_only()
                .depends_on("exchange_gain_loss"),
            FieldSpec::date("difference_posting_date", "Difference Posting Date")
                .in_list_view()
                .columns(2),
        ]
    }
}

impl DocumentController for SalesInvoiceAdvance {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
