use crate::erpnext::accounts::doctype::ledger_health_monitor_company::ledger_health_monitor_company::LedgerHealthMonitorCompany;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LedgerHealthMonitor {
    pub enable_health_monitor: bool,
    pub monitor_for_last_x_days: i32,
    pub debit_credit_mismatch: bool,
    pub general_and_payment_ledger_mismatch: bool,
    pub companies: Vec<LedgerHealthMonitorCompany>,
}

impl LedgerHealthMonitor {
    pub const DOCTYPE: &'static str = "Ledger Health Monitor";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "enable_health_monitor",
        "monitor_section",
        "monitor_for_last_x_days",
        "debit_credit_mismatch",
        "general_and_payment_ledger_mismatch",
        "section_break_xdsp",
        "companies",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const HIDE_TOOLBAR: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SINGLE: bool = true;
    pub const SORT_FIELD: &'static str = "modified";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::check("enable_health_monitor", "Enable Health Monitor").default("0"),
            FieldSpec::section_break("monitor_section").label("Configuration"),
            FieldSpec::int("monitor_for_last_x_days", "Monitor for Last 'X' days")
                .default("60")
                .in_list_view()
                .required(),
            FieldSpec::check("debit_credit_mismatch", "Debit-Credit Mismatch").default("0"),
            FieldSpec::check(
                "general_and_payment_ledger_mismatch",
                "Discrepancy between General and Payment Ledger",
            )
            .default("0"),
            FieldSpec::section_break("section_break_xdsp").label("Companies"),
            FieldSpec::table_unlabeled("companies").options("Ledger Health Monitor Company"),
        ]
    }
}

impl DocumentController for LedgerHealthMonitor {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
