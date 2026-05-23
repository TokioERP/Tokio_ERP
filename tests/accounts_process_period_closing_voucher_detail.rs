use tokio_erp::erpnext::accounts::doctype::process_period_closing_voucher_detail::process_period_closing_voucher_detail::ProcessPeriodClosingVoucherDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_period_closing_voucher_detail_matches_erpnext_metadata() {
    assert_eq!(
        ProcessPeriodClosingVoucherDetail::DOCTYPE,
        "Process Period Closing Voucher Detail"
    );
    assert_eq!(ProcessPeriodClosingVoucherDetail::MODULE, "Accounts");
    assert_eq!(
        ProcessPeriodClosingVoucherDetail::FIELD_ORDER,
        [
            "processing_date",
            "report_type",
            "status",
            "closing_balance",
        ]
    );
    assert!(ProcessPeriodClosingVoucherDetail::ALLOW_RENAME);
    assert!(ProcessPeriodClosingVoucherDetail::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ProcessPeriodClosingVoucherDetail::IS_TABLE);
    assert_eq!(ProcessPeriodClosingVoucherDetail::GRID_PAGE_LENGTH, 50);
    assert_eq!(ProcessPeriodClosingVoucherDetail::ROW_FORMAT, "Dynamic");
    assert_eq!(
        ProcessPeriodClosingVoucherDetail::ROWS_THRESHOLD_FOR_GRID_SEARCH,
        20
    );

    assert_eq!(
        ProcessPeriodClosingVoucherDetail::fields(),
        vec![
            FieldSpec::date("processing_date", "Processing Date").in_list_view(),
            FieldSpec::select("report_type", "Report Type")
                .options("Profit and Loss\nBalance Sheet")
                .default("Profit and Loss")
                .in_list_view(),
            FieldSpec::select("status", "Status")
                .options("Queued\nRunning\nPaused\nCompleted\nCancelled")
                .default("Queued")
                .in_list_view(),
            FieldSpec::json("closing_balance", "Closing Balance").in_list_view(),
        ]
    );
}

#[test]
fn process_period_closing_voucher_detail_preserves_pass_controller_behavior() {
    let blank = ProcessPeriodClosingVoucherDetail::default();
    assert_eq!(blank.processing_date, None);
    assert_eq!(blank.report_type.as_deref(), Some("Profit and Loss"));
    assert_eq!(blank.status.as_deref(), Some("Queued"));
    assert_eq!(blank.closing_balance, None);
    assert!(blank.custom_hooks().is_empty());

    let row = ProcessPeriodClosingVoucherDetail::new("2026-05-01", "Balance Sheet", "Running");
    assert_eq!(row.processing_date.as_deref(), Some("2026-05-01"));
    assert_eq!(row.report_type.as_deref(), Some("Balance Sheet"));
    assert_eq!(row.status.as_deref(), Some("Running"));
    assert_eq!(row.doctype(), "Process Period Closing Voucher Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
