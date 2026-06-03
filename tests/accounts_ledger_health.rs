use tokio_erp::erpnext::accounts::doctype::ledger_health::ledger_health::{
    run_ledger_health_checks, LedgerHealth, LedgerHealthReportRow,
};
use tokio_erp::erpnext::accounts::doctype::ledger_health_monitor::ledger_health_monitor::LedgerHealthMonitor;
use tokio_erp::erpnext::accounts::doctype::ledger_health_monitor_company::ledger_health_monitor_company::LedgerHealthMonitorCompany;
use tokio_erp::erpnext::DocumentController;

#[test]
fn ledger_health_run_checks_matches_erpnext_test_rows() {
    let monitor = LedgerHealthMonitor {
        enable_health_monitor: true,
        monitor_for_last_x_days: 60,
        debit_credit_mismatch: true,
        general_and_payment_ledger_mismatch: true,
        companies: vec![LedgerHealthMonitorCompany::new("Test Company")],
    };

    let entries = run_ledger_health_checks(
        &monitor,
        "2026-06-03 14:30:00",
        &[LedgerHealthReportRow::new("Journal Entry", "ACC-JV-0001")],
        &[LedgerHealthReportRow::new("Journal Entry", "ACC-JV-0002")],
    );

    assert_eq!(
        entries,
        vec![
            LedgerHealth {
                voucher_type: Some("Journal Entry".to_string()),
                voucher_no: Some("ACC-JV-0001".to_string()),
                checked_on: Some("2026-06-03 14:30:00".to_string()),
                debit_credit_mismatch: true,
                general_and_payment_ledger_mismatch: false,
            },
            LedgerHealth {
                voucher_type: Some("Journal Entry".to_string()),
                voucher_no: Some("ACC-JV-0002".to_string()),
                checked_on: Some("2026-06-03 14:30:00".to_string()),
                debit_credit_mismatch: false,
                general_and_payment_ledger_mismatch: true,
            },
        ]
    );
}

#[test]
fn ledger_health_run_checks_preserves_disabled_monitor_noop() {
    let monitor = LedgerHealthMonitor {
        enable_health_monitor: false,
        monitor_for_last_x_days: 60,
        debit_credit_mismatch: true,
        general_and_payment_ledger_mismatch: true,
        companies: vec![LedgerHealthMonitorCompany::new("Test Company")],
    };

    assert!(run_ledger_health_checks(
        &monitor,
        "2026-06-03 14:30:00",
        &[LedgerHealthReportRow::new("Journal Entry", "ACC-JV-0001")],
        &[LedgerHealthReportRow::new("Journal Entry", "ACC-JV-0002")],
    )
    .is_empty());
}

#[test]
fn ledger_health_preserves_pass_controller_behavior() {
    let entry = LedgerHealth {
        voucher_type: Some("Journal Entry".to_string()),
        voucher_no: Some("ACC-JV-0001".to_string()),
        checked_on: Some("2026-06-03 14:30:00".to_string()),
        debit_credit_mismatch: true,
        general_and_payment_ledger_mismatch: false,
    };

    assert_eq!(entry.doctype(), "Ledger Health");
    assert_eq!(entry.module(), "Accounts");
    assert!(entry.custom_hooks().is_empty());
}
