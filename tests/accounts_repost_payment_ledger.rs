use tokio_erp::erpnext::accounts::doctype::repost_payment_ledger::repost_payment_ledger::{
    execute_repost_payment_ledger, repost_ple_for_voucher, start_payment_ledger_repost,
    PaymentLedgerRepostAction, PaymentLedgerRepostJob, PaymentLedgerRunContext,
    RepostPaymentLedger, RepostPaymentVoucher, VoucherSource, VOUCHER_TYPES,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn repost_payment_ledger_matches_erpnext_metadata() {
    assert_eq!(RepostPaymentLedger::DOCTYPE, "Repost Payment Ledger");
    assert_eq!(RepostPaymentLedger::MODULE, "Accounts");
    assert_eq!(
        RepostPaymentLedger::FIELD_ORDER,
        [
            "filters_section",
            "company",
            "posting_date",
            "column_break_4",
            "voucher_type",
            "add_manually",
            "status_section",
            "repost_status",
            "repost_error_log",
            "selected_vouchers_section",
            "repost_vouchers",
            "amended_from",
        ]
    );
    assert_eq!(
        VOUCHER_TYPES,
        [
            "Sales Invoice",
            "Purchase Invoice",
            "Payment Entry",
            "Journal Entry"
        ]
    );
    assert!(RepostPaymentLedger::EDITABLE_GRID);
    assert!(RepostPaymentLedger::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(RepostPaymentLedger::IS_SUBMITTABLE);
    assert!(RepostPaymentLedger::TRACK_CHANGES);
    assert_eq!(
        RepostPaymentLedger::fields(),
        vec![
            FieldSpec::section_break("filters_section").label("Filters"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .required(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("voucher_type", "Voucher Type").options("DocType"),
            FieldSpec::check("add_manually", "Add Manually")
                .default("0")
                .description("Ignore Voucher Type filter and Select Vouchers Manually"),
            FieldSpec::section_break("status_section").label("Status"),
            FieldSpec::select("repost_status", "Repost Status")
                .options("\nQueued\nFailed\nCompleted")
                .read_only(),
            FieldSpec::long_text("repost_error_log", "Repost Error Log")
                .depends_on("eval:doc.repost_error_log"),
            FieldSpec::section_break("selected_vouchers_section").label("Vouchers"),
            FieldSpec::table("repost_vouchers", "Selected Vouchers")
                .options("Repost Payment Ledger Items"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Repost Payment Ledger")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn repost_payment_ledger_before_validate_loads_filtered_vouchers_and_status() {
    let sources = vec![
        VoucherSource::new(
            "Sales Invoice",
            "SINV-0001",
            "_Test Company",
            "2026-05-23",
            1,
        ),
        VoucherSource::new(
            "Sales Invoice",
            "SINV-OLD",
            "_Test Company",
            "2026-05-22",
            1,
        ),
        VoucherSource::new(
            "Payment Entry",
            "PAY-0001",
            "_Test Company",
            "2026-05-24",
            1,
        ),
        VoucherSource::new(
            "Journal Entry",
            "JV-DRAFT",
            "_Test Company",
            "2026-05-24",
            0,
        ),
        VoucherSource::new(
            "Purchase Invoice",
            "PINV-0001",
            "Other Company",
            "2026-05-24",
            1,
        ),
    ];
    let mut doc = RepostPaymentLedger::new("RPL-0001", "_Test Company", "2026-05-23");
    doc.before_validate(&sources);

    assert_eq!(doc.repost_status.as_deref(), Some("Queued"));
    assert_eq!(
        doc.repost_vouchers,
        vec![
            RepostPaymentVoucher::new("Sales Invoice", "SINV-0001"),
            RepostPaymentVoucher::new("Payment Entry", "PAY-0001"),
        ]
    );

    let mut manual = RepostPaymentLedger::new("RPL-0002", "_Test Company", "2026-05-23");
    manual.add_manually = true;
    manual.repost_vouchers = vec![RepostPaymentVoucher::new("Payment Entry", "PAY-MANUAL")];
    manual.before_validate(&sources);
    assert_eq!(
        manual.repost_vouchers,
        vec![RepostPaymentVoucher::new("Payment Entry", "PAY-MANUAL")]
    );

    let mut only_payment = RepostPaymentLedger::new("RPL-0003", "_Test Company", "2026-05-23");
    only_payment.voucher_type = Some("Payment Entry".to_string());
    assert_eq!(
        only_payment.get_vouchers(&sources),
        vec![RepostPaymentVoucher::new("Payment Entry", "PAY-0001")]
    );
}

#[test]
fn repost_payment_ledger_submit_and_worker_plans_match_erpnext() {
    assert_eq!(
        execute_repost_payment_ledger("RPL-0001"),
        PaymentLedgerRepostJob {
            method: "erpnext.accounts.doctype.repost_payment_ledger.repost_payment_ledger.start_payment_ledger_repost".to_string(),
            docname: "RPL-0001".to_string(),
            job_name: "payment_ledger_repost_RPL-0001".to_string(),
            is_async: true,
        }
    );

    let mut doc = RepostPaymentLedger::new("RPL-0001", "_Test Company", "2026-05-23");
    doc.repost_vouchers = vec![
        RepostPaymentVoucher::new("Payment Entry", "PAY-0001"),
        RepostPaymentVoucher::new("Sales Invoice", "SINV-0001"),
    ];
    assert_eq!(doc.on_submit(), execute_repost_payment_ledger("RPL-0001"));

    let actions = start_payment_ledger_repost(&doc, &PaymentLedgerRunContext::submitted("Queued"));
    assert_eq!(
        actions,
        vec![
            PaymentLedgerRepostAction::BuildGlMap {
                voucher_type: "Payment Entry".to_string(),
                voucher_no: "PAY-0001".to_string(),
            },
            PaymentLedgerRepostAction::DeletePaymentLedgerEntries {
                voucher_type: "Payment Entry".to_string(),
                voucher_no: "PAY-0001".to_string(),
            },
            PaymentLedgerRepostAction::DeleteAdvancePaymentLedgerEntries {
                voucher_type: "Payment Entry".to_string(),
                voucher_no: "PAY-0001".to_string(),
            },
            PaymentLedgerRepostAction::CreatePaymentLedgerEntry { cancel: false },
            PaymentLedgerRepostAction::GetGlEntries {
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SINV-0001".to_string(),
            },
            PaymentLedgerRepostAction::DeletePaymentLedgerEntries {
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SINV-0001".to_string(),
            },
            PaymentLedgerRepostAction::DeleteAdvancePaymentLedgerEntries {
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SINV-0001".to_string(),
            },
            PaymentLedgerRepostAction::CreatePaymentLedgerEntry { cancel: false },
            PaymentLedgerRepostAction::SetRepostErrorLog("".to_string()),
            PaymentLedgerRepostAction::SetRepostStatus("Completed".to_string()),
        ]
    );

    assert_eq!(
        repost_ple_for_voucher("Sales Invoice", "SINV-0001", true),
        vec![
            PaymentLedgerRepostAction::DeletePaymentLedgerEntries {
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SINV-0001".to_string(),
            },
            PaymentLedgerRepostAction::DeleteAdvancePaymentLedgerEntries {
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SINV-0001".to_string(),
            },
            PaymentLedgerRepostAction::CreatePaymentLedgerEntry { cancel: false },
        ]
    );
    assert!(repost_ple_for_voucher("Sales Invoice", "SINV-0001", false).is_empty());
}

#[test]
fn repost_payment_ledger_worker_failure_matches_traceback_handling() {
    let mut doc = RepostPaymentLedger::new("RPL-0001", "_Test Company", "2026-05-23");
    doc.repost_vouchers = vec![RepostPaymentVoucher::new("Journal Entry", "JV-0001")];
    let actions = start_payment_ledger_repost(
        &doc,
        &PaymentLedgerRunContext {
            docstatus: 1,
            repost_status: "Failed".to_string(),
            fail_traceback: Some("boom".to_string()),
        },
    );
    assert_eq!(
        actions,
        vec![
            PaymentLedgerRepostAction::Rollback,
            PaymentLedgerRepostAction::SetRepostErrorLog("Traceback: <br>boom".to_string()),
            PaymentLedgerRepostAction::SetRepostStatus("Failed".to_string()),
        ]
    );

    let skipped = start_payment_ledger_repost(
        &doc,
        &PaymentLedgerRunContext {
            docstatus: 0,
            repost_status: "Queued".to_string(),
            fail_traceback: None,
        },
    );
    assert!(skipped.is_empty());
    assert_eq!(doc.custom_hooks(), ["before_validate", "on_submit"]);
    assert_eq!(doc.doctype(), "Repost Payment Ledger");
    assert_eq!(doc.module(), "Accounts");
}
