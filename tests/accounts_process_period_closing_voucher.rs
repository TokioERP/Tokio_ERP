use tokio_erp::erpnext::accounts::doctype::process_period_closing_voucher::process_period_closing_voucher::{
    PeriodClosingVoucherContext, ProcessPeriodClosingVoucher, ProcessPeriodClosingVoucherDetail,
    ProcessPeriodClosingVoucherHook,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_period_closing_voucher_matches_erpnext_metadata() {
    assert_eq!(
        ProcessPeriodClosingVoucher::DOCTYPE,
        "Process Period Closing Voucher"
    );
    assert_eq!(ProcessPeriodClosingVoucher::MODULE, "Accounts");
    assert_eq!(
        ProcessPeriodClosingVoucher::AUTONAME,
        "format:Process-PCV-{###}"
    );
    assert_eq!(ProcessPeriodClosingVoucher::GRID_PAGE_LENGTH, 50);
    assert_eq!(ProcessPeriodClosingVoucher::ROW_FORMAT, "Dynamic");
    assert_eq!(
        ProcessPeriodClosingVoucher::FIELD_ORDER,
        [
            "parent_pcv",
            "status",
            "p_l_closing_balance",
            "normal_balances",
            "bs_closing_balance",
            "z_opening_balances",
            "amended_from",
        ]
    );
    assert!(ProcessPeriodClosingVoucher::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ProcessPeriodClosingVoucher::IS_SUBMITTABLE);

    assert_eq!(
        ProcessPeriodClosingVoucher::fields(),
        vec![
            FieldSpec::link("parent_pcv", "PCV")
                .options("Period Closing Voucher")
                .required()
                .in_list_view(),
            FieldSpec::select("status", "Status")
                .options("Queued\nRunning\nPaused\nCompleted\nCancelled")
                .default("Queued")
                .no_copy(),
            FieldSpec::json("p_l_closing_balance", "P&L Closing Balance").no_copy(),
            FieldSpec::table("normal_balances", "Dates to Process")
                .options("Process Period Closing Voucher Detail")
                .no_copy(),
            FieldSpec::json("bs_closing_balance", "Balance Sheet Closing Balance"),
            FieldSpec::table("z_opening_balances", "Opening Balances")
                .options("Process Period Closing Voucher Detail")
                .no_copy(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Period Closing Voucher")
                .no_copy()
                .print_hide()
                .read_only()
                .search_index(),
        ]
    );
}

#[test]
fn process_period_closing_voucher_validate_populates_processing_tables() {
    let context = PeriodClosingVoucherContext {
        period_start_date: "2026-05-01".to_string(),
        period_end_date: "2026-05-03".to_string(),
        is_first_period_closing_voucher: true,
        gl_min_posting_date: Some("2026-04-01".to_string()),
        gl_max_posting_date: Some("2026-04-02".to_string()),
    };
    let mut doc = ProcessPeriodClosingVoucher::new("PCV-0001");

    doc.validate(&context).unwrap();

    assert_eq!(doc.status.as_deref(), Some("Queued"));
    assert_eq!(doc.normal_balances.len(), 6);
    assert_eq!(
        doc.normal_balances[0],
        ProcessPeriodClosingVoucherDetail::new(
            1,
            "normal_balances",
            "2026-05-01",
            "Profit and Loss",
            "Queued",
        )
    );
    assert_eq!(
        doc.normal_balances[1],
        ProcessPeriodClosingVoucherDetail::new(
            2,
            "normal_balances",
            "2026-05-01",
            "Balance Sheet",
            "Queued",
        )
    );
    assert_eq!(doc.normal_balances[5].processing_date, "2026-05-03");
    assert_eq!(doc.normal_balances[5].report_type, "Balance Sheet");

    assert_eq!(doc.z_opening_balances.len(), 2);
    assert_eq!(doc.z_opening_balances[0].processing_date, "2026-04-01");
    assert_eq!(doc.z_opening_balances[0].report_type, "Balance Sheet");
    assert_eq!(doc.z_opening_balances[1].processing_date, "2026-04-02");
}

#[test]
fn process_period_closing_voucher_preserves_hooks_and_client_helpers() {
    let mut doc = ProcessPeriodClosingVoucher::new("PCV-0001");
    assert_eq!(doc.doctype(), "Process Period Closing Voucher");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        ["on_discard", "validate", "on_submit", "on_cancel"]
    );

    assert_eq!(
        doc.on_submit(),
        ProcessPeriodClosingVoucherHook::StartProcessing {
            docname: "PCV-0001".to_string(),
        }
    );
    assert_eq!(
        doc.on_cancel(),
        ProcessPeriodClosingVoucherHook::CancelProcessing {
            docname: "PCV-0001".to_string(),
        }
    );

    doc.status = Some("Running".to_string());
    doc.on_discard();
    assert_eq!(doc.status.as_deref(), Some("Cancelled"));

    assert_eq!(
        ProcessPeriodClosingVoucher::client_action(1, "Queued"),
        Some("Start")
    );
    assert_eq!(
        ProcessPeriodClosingVoucher::client_action(1, "Running"),
        Some("Pause")
    );
    assert_eq!(
        ProcessPeriodClosingVoucher::client_action(1, "Paused"),
        Some("Resume")
    );
    assert_eq!(
        ProcessPeriodClosingVoucher::client_action(0, "Queued"),
        None
    );
}

#[test]
fn process_period_closing_voucher_progress_matches_client_script() {
    let normal = vec![
        ProcessPeriodClosingVoucherDetail::new(
            1,
            "normal_balances",
            "2026-05-01",
            "Profit and Loss",
            "Completed",
        ),
        ProcessPeriodClosingVoucherDetail::new(
            2,
            "normal_balances",
            "2026-05-01",
            "Balance Sheet",
            "Queued",
        ),
    ];
    let opening = vec![
        ProcessPeriodClosingVoucherDetail::new(
            1,
            "z_opening_balances",
            "2026-04-01",
            "Balance Sheet",
            "Completed",
        ),
        ProcessPeriodClosingVoucherDetail::new(
            2,
            "z_opening_balances",
            "2026-04-02",
            "Balance Sheet",
            "Completed",
        ),
    ];

    assert_eq!(
        ProcessPeriodClosingVoucher::progress(&normal, &opening),
        Some(75.0)
    );
    assert_eq!(ProcessPeriodClosingVoucher::progress(&[], &[]), None);
}
