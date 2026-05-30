use tokio_erp::erpnext::accounts::doctype::ledger_health_monitor::ledger_health_monitor::LedgerHealthMonitor;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn ledger_health_monitor_matches_erpnext_metadata() {
    assert_eq!(LedgerHealthMonitor::DOCTYPE, "Ledger Health Monitor");
    assert_eq!(LedgerHealthMonitor::MODULE, "Accounts");
    assert_eq!(
        LedgerHealthMonitor::FIELD_ORDER,
        [
            "enable_health_monitor",
            "monitor_section",
            "monitor_for_last_x_days",
            "debit_credit_mismatch",
            "general_and_payment_ledger_mismatch",
            "section_break_xdsp",
            "companies",
        ]
    );
    assert!(LedgerHealthMonitor::ALLOW_RENAME);
    assert!(LedgerHealthMonitor::HIDE_TOOLBAR);
    assert!(LedgerHealthMonitor::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(LedgerHealthMonitor::IS_SINGLE);
    assert_eq!(LedgerHealthMonitor::SORT_FIELD, "modified");
    assert_eq!(LedgerHealthMonitor::SORT_ORDER, "DESC");
    assert!(LedgerHealthMonitor::TRACK_CHANGES);

    assert_eq!(
        LedgerHealthMonitor::fields(),
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
    );
}

#[test]
fn ledger_health_monitor_preserves_pass_controller_behavior() {
    let monitor = LedgerHealthMonitor::default();

    assert!(!monitor.enable_health_monitor);
    assert_eq!(monitor.monitor_for_last_x_days, 0);
    assert!(!monitor.debit_credit_mismatch);
    assert!(!monitor.general_and_payment_ledger_mismatch);
    assert!(monitor.companies.is_empty());
    assert_eq!(monitor.doctype(), "Ledger Health Monitor");
    assert_eq!(monitor.module(), "Accounts");
    assert!(monitor.custom_hooks().is_empty());
}
