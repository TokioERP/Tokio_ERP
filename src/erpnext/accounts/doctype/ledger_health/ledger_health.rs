use crate::erpnext::accounts::doctype::ledger_health_monitor::ledger_health_monitor::LedgerHealthMonitor;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LedgerHealth {
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub checked_on: Option<String>,
    pub debit_credit_mismatch: bool,
    pub general_and_payment_ledger_mismatch: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerHealthReportRow {
    pub voucher_type: String,
    pub voucher_no: String,
}

impl LedgerHealthReportRow {
    pub fn new(voucher_type: impl Into<String>, voucher_no: impl Into<String>) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
        }
    }
}

impl LedgerHealth {
    pub const DOCTYPE: &'static str = "Ledger Health";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "autoincrement";
    pub const NAMING_RULE: &'static str = "Autoincrement";
    pub const IN_CREATE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const READ_ONLY: bool = true;
    pub const SORT_FIELD: &'static str = "modified";
    pub const SORT_ORDER: &'static str = "DESC";
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
            FieldSpec::check("debit_credit_mismatch", "Debit-Credit mismatch").default("0"),
            FieldSpec::datetime("checked_on", "Checked On"),
            FieldSpec::check(
                "general_and_payment_ledger_mismatch",
                "General and Payment Ledger mismatch",
            )
            .default("0"),
        ]
    }
}

pub fn run_ledger_health_checks(
    monitor: &LedgerHealthMonitor,
    checked_on: impl Into<String>,
    debit_credit_rows: &[LedgerHealthReportRow],
    general_payment_rows: &[LedgerHealthReportRow],
) -> Vec<LedgerHealth> {
    if !monitor.enable_health_monitor || monitor.companies.is_empty() {
        return Vec::new();
    }

    let checked_on = checked_on.into();
    let mut entries = Vec::new();

    if monitor.debit_credit_mismatch {
        entries.extend(debit_credit_rows.iter().map(|row| LedgerHealth {
            voucher_type: Some(row.voucher_type.clone()),
            voucher_no: Some(row.voucher_no.clone()),
            checked_on: Some(checked_on.clone()),
            debit_credit_mismatch: true,
            general_and_payment_ledger_mismatch: false,
        }));
    }

    if monitor.general_and_payment_ledger_mismatch {
        entries.extend(general_payment_rows.iter().map(|row| LedgerHealth {
            voucher_type: Some(row.voucher_type.clone()),
            voucher_no: Some(row.voucher_no.clone()),
            checked_on: Some(checked_on.clone()),
            debit_credit_mismatch: false,
            general_and_payment_ledger_mismatch: true,
        }));
    }

    entries
}

impl DocumentController for LedgerHealth {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
