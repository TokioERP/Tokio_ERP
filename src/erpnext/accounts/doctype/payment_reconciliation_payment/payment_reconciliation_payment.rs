use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentReconciliationPayment;

impl PaymentReconciliationPayment {
    pub const DOCTYPE: &'static str = "Payment Reconciliation Payment";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const IS_VIRTUAL: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const FIELD_ORDER: [&'static str; 13] = [
        "reference_type",
        "reference_name",
        "posting_date",
        "is_advance",
        "reference_row",
        "col_break1",
        "amount",
        "difference_amount",
        "sec_break1",
        "remarks",
        "currency",
        "exchange_rate",
        "cost_center",
    ];

    pub fn get_list<T>(_args: T) {}

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::date("posting_date", "Posting Date")
                .read_only()
                .in_list_view(),
            FieldSpec::data("is_advance", "Is Advance")
                .read_only()
                .hidden(),
            FieldSpec::data("reference_row", "Reference Row")
                .read_only()
                .hidden(),
            FieldSpec::column_break("col_break1"),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("sec_break1"),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::currency("difference_amount", "Difference Amount")
                .options("currency")
                .read_only(),
            FieldSpec::float("exchange_rate", "Exchange Rate").hidden(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::small_text("remarks", "Remarks").read_only(),
        ]
    }
}

impl DocumentController for PaymentReconciliationPayment {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
