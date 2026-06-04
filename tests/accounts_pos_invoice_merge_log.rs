use tokio_erp::erpnext::accounts::doctype::pos_invoice_merge_log::pos_invoice_merge_log::{
    split_invoices, MergeInvoicesBasedOn, PosInvoiceMergeLog, PosInvoiceMergeLogError,
    PosInvoiceMergeLogInvoice, PosInvoiceMergeLogSplitInvoice,
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
fn pos_invoice_merge_log_preserves_controller_hooks() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    assert_eq!(log.doctype(), "POS Invoice Merge Log");
    assert_eq!(log.module(), "Accounts");
    assert_eq!(log.custom_hooks(), vec!["validate", "on_submit", "on_cancel"]);
}
