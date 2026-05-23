use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostAccountingLedgerItems {
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
}

impl RepostAccountingLedgerItems {
    pub const DOCTYPE: &'static str = "Repost Accounting Ledger Items";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["voucher_type", "voucher_no"];
    pub const ALLOW_RENAME: bool = true;
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(voucher_type: impl Into<String>, voucher_no: impl Into<String>) -> Self {
        Self {
            voucher_type: Some(voucher_type.into()),
            voucher_no: Some(voucher_no.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .in_list_view(),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .in_list_view(),
        ]
    }
}

impl DocumentController for RepostAccountingLedgerItems {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
