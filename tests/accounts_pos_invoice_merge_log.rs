use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::pos_invoice_merge_log::pos_invoice_merge_log::{
    check_scheduler_status, consolidate_pos_invoices_plan, enqueue_job_plan, get_error_message,
    get_invoice_customer_map, split_invoices, split_invoices_by_accounting_dimension,
    unconsolidate_pos_invoices_plan, EnqueueJobKind, ErrorMessage, MergeInvoicesBasedOn,
    PosInvoiceMergeLog, PosInvoiceMergeLogAction, PosInvoiceMergeLogError,
    PosInvoiceMergeLogInvoice, PosInvoiceMergeLogSourceInvoice, PosInvoiceMergeLogSplitInvoice,
    SchedulerStatus,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_invoice_merge_log_matches_erpnext_metadata() {
    assert_eq!(PosInvoiceMergeLog::DOCTYPE, "POS Invoice Merge Log");
    assert_eq!(PosInvoiceMergeLog::MODULE, "Accounts");
    assert_eq!(
        PosInvoiceMergeLog::FIELD_ORDER,
        [
            "company",
            "posting_date",
            "posting_time",
            "merge_invoices_based_on",
            "column_break_3",
            "pos_closing_entry",
            "customer",
            "customer_group",
            "section_break_3",
            "pos_invoices",
            "references_section",
            "consolidated_invoice",
            "column_break_7",
            "consolidated_credit_note",
            "amended_from",
        ]
    );
    assert!(PosInvoiceMergeLog::EDITABLE_GRID);
    assert!(PosInvoiceMergeLog::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(PosInvoiceMergeLog::IS_SUBMITTABLE);
    assert_eq!(PosInvoiceMergeLog::ROW_FORMAT, "Dynamic");
    assert_eq!(PosInvoiceMergeLog::SORT_FIELD, "creation");
    assert_eq!(PosInvoiceMergeLog::SORT_ORDER, "DESC");
    assert!(PosInvoiceMergeLog::TRACK_CHANGES);

    let fields = PosInvoiceMergeLog::fields();
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .in_standard_filter()
            .print_hide()
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::table("pos_invoices", "POS Invoices")
            .options("POS Invoice Reference")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::select("merge_invoices_based_on", "Merge Invoices Based On")
            .options("Customer\nCustomer Group")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .depends_on("eval:doc.merge_invoices_based_on == 'Customer Group'")
            .mandatory_depends_on("eval:doc.merge_invoices_based_on == 'Customer Group'")
    ));
}

#[test]
fn pos_invoice_merge_log_validates_duplicates_customer_and_status() {
    let mut log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");
    log.pos_invoices = vec![
        PosInvoiceMergeLogInvoice::submitted(1, "POS-001", "_Test Customer"),
        PosInvoiceMergeLogInvoice::submitted(2, "POS-001", "_Test Customer"),
    ];
    assert_eq!(
        log.validate_duplicate_pos_invoices(),
        Err(PosInvoiceMergeLogError::DuplicatePosInvoices {
            invoice: "POS-001".to_string(),
            rows: vec![1, 2],
        })
    );

    log.pos_invoices = vec![PosInvoiceMergeLogInvoice::submitted(3, "POS-002", "Other Customer")];
    assert_eq!(
        log.validate_customer(),
        Err(PosInvoiceMergeLogError::CustomerMismatch {
            row: 3,
            pos_invoice: "POS-002".to_string(),
            customer: "_Test Customer".to_string(),
        })
    );

    log.merge_invoices_based_on = MergeInvoicesBasedOn::CustomerGroup;
    assert!(log.validate_customer().is_ok());

    log.pos_invoices = vec![PosInvoiceMergeLogInvoice {
        idx: 4,
        pos_invoice: "POS-003".to_string(),
        customer: "_Test Customer".to_string(),
        status: "Draft".to_string(),
        docstatus: 0,
        is_return: false,
        return_against: None,
        return_against_status: None,
    }];
    assert_eq!(
        log.validate_pos_invoice_status(),
        Err(PosInvoiceMergeLogError::PosInvoiceNotSubmitted {
            row: 4,
            pos_invoice: "POS-003".to_string(),
        })
    );
}

#[test]
fn pos_invoice_merge_log_splits_serial_returns_before_following_sales() {
    let invoices = vec![
        PosInvoiceMergeLogSplitInvoice::sale("POS-SALE-1"),
        PosInvoiceMergeLogSplitInvoice::serial_return("POS-RETURN-1", "POS-SALE-1", false),
        PosInvoiceMergeLogSplitInvoice::sale("POS-SALE-2"),
    ];

    assert_eq!(
        split_invoices(&invoices),
        vec![
            vec!["POS-SALE-1".to_string()],
            vec!["POS-RETURN-1".to_string(), "POS-SALE-2".to_string()],
        ]
    );
}

#[test]
fn pos_invoice_merge_log_groups_customer_map_by_accounting_dimensions() {
    let invoices = vec![
        PosInvoiceMergeLogSourceInvoice::new("POS-001", "_Test Customer")
            .dimension("cost_center", "Main - TC")
            .dimension("project", "PROJ-1"),
        PosInvoiceMergeLogSourceInvoice::new("POS-002", "_Test Customer")
            .dimension("cost_center", "Main - TC")
            .dimension("project", "PROJ-1"),
        PosInvoiceMergeLogSourceInvoice::new("POS-003", "_Test Customer")
            .dimension("cost_center", "Other - TC")
            .dimension("project", "PROJ-2"),
        PosInvoiceMergeLogSourceInvoice::new("POS-004", "Second Customer")
            .dimension("cost_center", "Main - TC"),
    ];

    let customer_map = get_invoice_customer_map(&invoices);
    assert_eq!(customer_map.len(), 2);
    assert_eq!(customer_map["_Test Customer"].len(), 2);
    assert_eq!(customer_map["Second Customer"].len(), 1);

    let grouped = split_invoices_by_accounting_dimension(&invoices[..3]);
    assert_eq!(
        grouped
            .values()
            .map(|invoices| invoices.len())
            .collect::<Vec<_>>(),
        vec![2, 1]
    );
}

#[test]
fn pos_invoice_merge_log_plans_queue_or_inline_consolidation_like_erpnext_thresholds() {
    let grouped: BTreeMap<String, BTreeMap<Vec<(String, String)>, Vec<String>>> =
        BTreeMap::from([(
            "_Test Customer".to_string(),
            BTreeMap::from([(vec![("cost_center".to_string(), "Main - TC".to_string())], vec![
                "POS-001".to_string(),
            ])]),
        )]);

    assert_eq!(
        consolidate_pos_invoices_plan(10, Some("POS-CLOSE-0001"), grouped.clone()),
        vec![
            PosInvoiceMergeLogAction::SetClosingEntryStatus {
                status: "Queued".to_string(),
            },
            PosInvoiceMergeLogAction::EnqueueJob {
                kind: EnqueueJobKind::CreateMergeLogs,
                job_id: "pos_invoice_merge::POS-CLOSE-0001".to_string(),
                message: "POS Invoices will be consolidated in a background process".to_string(),
            },
        ]
    );
    assert_eq!(
        consolidate_pos_invoices_plan(9, Some("POS-CLOSE-0001"), grouped),
        vec![PosInvoiceMergeLogAction::CreateMergeLogs]
    );
    assert_eq!(
        unconsolidate_pos_invoices_plan(10, Some("POS-CLOSE-0001")),
        vec![
            PosInvoiceMergeLogAction::SetClosingEntryStatus {
                status: "Queued".to_string(),
            },
            PosInvoiceMergeLogAction::EnqueueJob {
                kind: EnqueueJobKind::CancelMergeLogs,
                job_id: "pos_invoice_merge::POS-CLOSE-0001".to_string(),
                message: "POS Invoices will be unconsolidated in a background process".to_string(),
            },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_enqueue_and_error_helpers_match_erpnext_branches() {
    assert_eq!(
        check_scheduler_status(false, true),
        Err(PosInvoiceMergeLogError::SchedulerInactive)
    );
    assert_eq!(check_scheduler_status(true, true), Ok(()));
    assert_eq!(
        enqueue_job_plan(
            EnqueueJobKind::CreateMergeLogs,
            Some("POS-CLOSE-0001"),
            false,
            SchedulerStatus {
                developer_mode: false,
                in_test: true,
                scheduler_inactive: false,
            },
        ),
        Ok(Some(PosInvoiceMergeLogAction::EnqueueJob {
            kind: EnqueueJobKind::CreateMergeLogs,
            job_id: "pos_invoice_merge::POS-CLOSE-0001".to_string(),
            message: "POS Invoices will be consolidated in a background process".to_string(),
        }))
    );
    assert_eq!(
        enqueue_job_plan(
            EnqueueJobKind::CancelMergeLogs,
            Some("POS-CLOSE-0001"),
            true,
            SchedulerStatus {
                developer_mode: false,
                in_test: false,
                scheduler_inactive: false,
            },
        ),
        Ok(None)
    );
    assert_eq!(
        get_error_message(ErrorMessage::Dict(BTreeMap::from([(
            "message".to_string(),
            "Detailed failure".to_string(),
        )]))),
        "Detailed failure"
    );
    assert_eq!(
        get_error_message(ErrorMessage::Text("fallback failure".to_string())),
        "fallback failure"
    );
}

#[test]
fn pos_invoice_merge_log_preserves_controller_hooks() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    assert_eq!(log.doctype(), "POS Invoice Merge Log");
    assert_eq!(log.module(), "Accounts");
    assert_eq!(log.custom_hooks(), vec!["validate", "on_submit", "on_cancel"]);
}
