use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostPaymentLedgerItems {
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
}

impl RepostPaymentLedgerItems {
    pub const DOCTYPE: &'static str = "Repost Payment Ledger Items";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["voucher_type", "voucher_no"];
    pub const IS_TABLE: bool = true;

    pub fn new(voucher_type: impl Into<String>, voucher_no: impl Into<String>) -> Self {
        Self {
            voucher_type: Some(voucher_type.into()),
            voucher_no: Some(voucher_no.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("voucher_type", "Voucher Type").options("DocType"),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type"),
        ]
    }
}

impl DocumentController for RepostPaymentLedgerItems {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
