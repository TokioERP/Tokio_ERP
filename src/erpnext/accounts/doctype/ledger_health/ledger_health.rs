use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LedgerHealth {
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub checked_on: Option<String>,
    pub debit_credit_mismatch: bool,
    pub general_and_payment_ledger_mismatch: bool,
}

impl LedgerHealth {
    pub const DOCTYPE: &'static str = "Ledger Health";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 5] = [
        "voucher_type",
        "voucher_no",
        "checked_on",
        "debit_credit_mismatch",
        "general_and_payment_ledger_mismatch",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("voucher_type", "Voucher Type"),
            FieldSpec::data("voucher_no", "Voucher No"),
            FieldSpec::datetime("checked_on", "Checked On"),
            FieldSpec::check("debit_credit_mismatch", "Debit-Credit mismatch").default("0"),
            FieldSpec::check(
                "general_and_payment_ledger_mismatch",
                "General and Payment Ledger mismatch",
            )
            .default("0"),
        ]
    }
}

impl DocumentController for LedgerHealth {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
