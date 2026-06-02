use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentEntryReference {
    pub payment_request: Option<String>,
}

impl PaymentEntryReference {
    pub const DOCTYPE: &'static str = "Payment Entry Reference";
    pub const MODULE: &'static str = "Accounts";
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 20] = [
        "reference_doctype",
        "reference_name",
        "due_date",
        "bill_no",
        "payment_term",
        "payment_term_outstanding",
        "account_type",
        "payment_type",
        "reconcile_effect_on",
        "column_break_4",
        "total_amount",
        "outstanding_amount",
        "allocated_amount",
        "exchange_rate",
        "exchange_gain_loss",
        "account",
        "payment_request",
        "payment_request_outstanding",
        "advance_voucher_type",
        "advance_voucher_no",
    ];

    pub fn payment_request_outstanding_with(
        &self,
        fetch_outstanding_amount: impl FnOnce(&str) -> Option<f64>,
    ) -> Option<f64> {
        let payment_request = self.payment_request.as_deref()?;
        fetch_outstanding_amount(payment_request)
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("reference_doctype", "Type")
                .options("DocType")
                .columns(2)
                .required()
                .in_list_view()
                .search_index(),
            FieldSpec::dynamic_link("reference_name")
                .label("Name")
                .options("reference_doctype")
                .columns(4)
                .required()
                .in_global_search()
                .in_list_view()
                .search_index(),
            FieldSpec::date("due_date", "Due Date")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::data("bill_no", "Supplier Invoice No")
                .read_only()
                .no_copy(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::currency("total_amount", "Grand Total")
                .columns(2)
                .read_only()
                .print_hide()
                .in_list_view(),
            FieldSpec::currency("outstanding_amount", "Outstanding")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated")
                .columns(2)
                .in_list_view(),
            FieldSpec::float("exchange_rate", "Exchange Rate")
                .depends_on("eval:(doc.reference_doctype=='Purchase Invoice')")
                .read_only()
                .print_hide(),
            FieldSpec::link("payment_term", "Payment Term").options("Payment Term"),
            FieldSpec::currency("exchange_gain_loss", "Exchange Gain/Loss")
                .options("Company:company:default_currency")
                .depends_on("exchange_gain_loss")
                .read_only(),
            FieldSpec::link("account", "Account").options("Account"),
            FieldSpec::data("account_type", "Account Type"),
            FieldSpec::data("payment_type", "Payment Type"),
            FieldSpec::link("payment_request", "Payment Request").options("Payment Request"),
            FieldSpec::float("payment_term_outstanding", "Payment Term Outstanding")
                .depends_on("eval: doc.payment_term")
                .read_only(),
            FieldSpec::float("payment_request_outstanding", "Payment Request Outstanding")
                .depends_on("eval: doc.payment_request && doc.payment_request_outstanding")
                .read_only()
                .is_virtual(),
            FieldSpec::date("reconcile_effect_on", "Reconcile Effect On").read_only(),
            FieldSpec::link("advance_voucher_type", "Advance Voucher Type")
                .options("DocType")
                .columns(2)
                .read_only(),
            FieldSpec::dynamic_link("advance_voucher_no")
                .label("Advance Voucher No")
                .options("advance_voucher_type")
                .columns(2)
                .read_only(),
        ]
    }
}

impl DocumentController for PaymentEntryReference {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
