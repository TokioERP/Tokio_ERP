use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AdvancePaymentLedgerEntry {
    pub company: Option<String>,
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub against_voucher_type: Option<String>,
    pub against_voucher_no: Option<String>,
    pub currency: Option<String>,
    pub exchange_rate: f64,
    pub amount: f64,
    pub base_amount: f64,
    pub event: Option<String>,
    pub delinked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvancePaymentOutstandingUpdate {
    pub voucher_type: String,
    pub voucher_no: Option<String>,
}

impl AdvancePaymentLedgerEntry {
    pub const DOCTYPE: &'static str = "Advance Payment Ledger Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 11] = [
        "company",
        "voucher_type",
        "voucher_no",
        "against_voucher_type",
        "against_voucher_no",
        "currency",
        "exchange_rate",
        "amount",
        "base_amount",
        "event",
        "delinked",
    ];
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .read_only(),
            FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .read_only(),
            FieldSpec::link("against_voucher_type", "Against Voucher Type")
                .options("DocType")
                .read_only(),
            FieldSpec::dynamic_link("against_voucher_no")
                .label("Against Voucher No")
                .options("against_voucher_type")
                .read_only(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .read_only(),
            FieldSpec::float("exchange_rate", "Exchange Rate")
                .depends_on("exchange_rate")
                .precision("9")
                .read_only(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .read_only(),
            FieldSpec::currency("base_amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .depends_on("base_amount")
                .read_only(),
            FieldSpec::data("event", "Event").read_only(),
            FieldSpec::check("delinked", "DeLinked")
                .default("0")
                .read_only(),
        ]
    }

    pub fn on_update_plan(
        &self,
        advance_payment_doctypes: &[String],
        update_outstanding_flag: Option<&str>,
        is_reverse_depr_entry: bool,
    ) -> Option<AdvancePaymentOutstandingUpdate> {
        let against_voucher_type = self.against_voucher_type.as_deref()?;
        if advance_payment_doctypes
            .iter()
            .any(|doctype| doctype == against_voucher_type)
            && update_outstanding_flag == Some("Yes")
            && !is_reverse_depr_entry
        {
            Some(AdvancePaymentOutstandingUpdate {
                voucher_type: against_voucher_type.to_string(),
                voucher_no: self.against_voucher_no.clone(),
            })
        } else {
            None
        }
    }
}

impl DocumentController for AdvancePaymentLedgerEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["on_update"]
    }
}

pub fn advance_payment_ledger_entry_indexes() -> [(&'static str, [&'static str; 2]); 2] {
    [
        (
            "Advance Payment Ledger Entry",
            ["against_voucher_type", "against_voucher_no"],
        ),
        (
            "Advance Payment Ledger Entry",
            ["voucher_type", "voucher_no"],
        ),
    ]
}
