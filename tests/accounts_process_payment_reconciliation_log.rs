use tokio_erp::erpnext::accounts::doctype::process_payment_reconciliation_log::process_payment_reconciliation_log::ProcessPaymentReconciliationLog;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_payment_reconciliation_log_matches_erpnext_metadata() {
    assert_eq!(
        ProcessPaymentReconciliationLog::DOCTYPE,
        "Process Payment Reconciliation Log"
    );
    assert_eq!(ProcessPaymentReconciliationLog::MODULE, "Accounts");
    assert_eq!(
        ProcessPaymentReconciliationLog::AUTONAME,
        "format:PPR-LOG-{##}"
    );
    assert_eq!(
        ProcessPaymentReconciliationLog::SEARCH_FIELDS,
        "allocated, reconciled, total_allocations, reconciled_entries"
    );
    assert_eq!(
        ProcessPaymentReconciliationLog::FIELD_ORDER,
        [
            "process_pr",
            "section_break_fvdw",
            "status",
            "tasks_section",
            "allocated",
            "reconciled",
            "column_break_yhin",
            "total_allocations",
            "reconciled_entries",
            "section_break_4ywv",
            "error_log",
            "allocations_section",
            "allocations",
        ]
    );
    assert!(ProcessPaymentReconciliationLog::EDITABLE_GRID);
    assert!(ProcessPaymentReconciliationLog::IN_CREATE);
    assert!(ProcessPaymentReconciliationLog::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        ProcessPaymentReconciliationLog::fields(),
        vec![
            FieldSpec::link("process_pr", "Parent Document")
                .options("Process Payment Reconciliation")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_fvdw").label("Status"),
            FieldSpec::select("status", "Status")
                .options("Running\nPaused\nReconciled\nPartially Reconciled\nFailed\nCancelled")
                .read_only(),
            FieldSpec::section_break("tasks_section").label("Tasks"),
            FieldSpec::check("allocated", "Allocated")
                .default("0")
                .description("Invoices and Payments have been Fetched and Allocated")
                .read_only(),
            FieldSpec::check("reconciled", "Reconciled")
                .default("0")
                .description("All allocations have been successfully reconciled")
                .read_only(),
            FieldSpec::column_break("column_break_yhin"),
            FieldSpec::int("total_allocations", "Total Allocations")
                .read_only()
                .in_list_view(),
            FieldSpec::int("reconciled_entries", "Reconciled Entries")
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_4ywv"),
            FieldSpec::long_text("error_log", "Reconciliation Error Log")
                .depends_on("eval:doc.error_log")
                .read_only(),
            FieldSpec::section_break("allocations_section").label("Allocations"),
            FieldSpec::table("allocations", "Allocations")
                .options("Process Payment Reconciliation Log Allocations")
                .read_only(),
        ]
    );
}

#[test]
fn process_payment_reconciliation_log_preserves_pass_controller_behavior() {
    let blank = ProcessPaymentReconciliationLog::default();
    assert_eq!(blank.process_pr, None);
    assert_eq!(blank.status, None);
    assert!(!blank.allocated);
    assert!(!blank.reconciled);
    assert_eq!(blank.total_allocations, 0);
    assert_eq!(blank.reconciled_entries, 0);
    assert_eq!(blank.error_log, None);
    assert!(blank.allocations.is_empty());
    assert!(blank.custom_hooks().is_empty());

    let log = ProcessPaymentReconciliationLog::new("PPR-0001");
    assert_eq!(log.process_pr.as_deref(), Some("PPR-0001"));
    assert_eq!(log.doctype(), "Process Payment Reconciliation Log");
    assert_eq!(log.module(), "Accounts");
    assert!(log.custom_hooks().is_empty());
}

#[test]
fn process_payment_reconciliation_log_progress_matches_client_script() {
    assert_eq!(
        ProcessPaymentReconciliationLog::progress("Running", 4, 10),
        Some(40.0)
    );
    assert_eq!(
        ProcessPaymentReconciliationLog::progress("Completed", 0, 0),
        Some(100.0)
    );
    assert_eq!(
        ProcessPaymentReconciliationLog::progress("Reconciled", 10, 10),
        None
    );
    assert_eq!(
        ProcessPaymentReconciliationLog::progress("Failed", 3, 10),
        None
    );
}

#[test]
fn process_payment_reconciliation_log_indicators_match_list_script() {
    let running = ProcessPaymentReconciliationLog::indicator_for_status("Running").unwrap();
    assert_eq!(running.status, "Running");
    assert_eq!(running.color, "blue");
    assert_eq!(running.filter, "status,=,Running");

    assert_eq!(
        ProcessPaymentReconciliationLog::indicator_for_status("Partially Reconciled")
            .unwrap()
            .color,
        "orange"
    );
    assert_eq!(
        ProcessPaymentReconciliationLog::indicator_for_status("Cancelled")
            .unwrap()
            .color,
        "red"
    );
    assert!(ProcessPaymentReconciliationLog::indicator_for_status("Queued").is_none());
}
