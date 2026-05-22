use tokio_erp::erpnext::accounts::doctype::process_payment_reconciliation::process_payment_reconciliation::{
    get_next_allocation, reconcile_job_name, Allocation, ProcessPaymentReconciliation,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_payment_reconciliation_matches_erpnext_metadata() {
    assert_eq!(
        ProcessPaymentReconciliation::DOCTYPE,
        "Process Payment Reconciliation"
    );
    assert_eq!(ProcessPaymentReconciliation::MODULE, "Accounts");
    assert_eq!(
        ProcessPaymentReconciliation::AUTONAME,
        "format:ACC-PPR-{#####}"
    );
    assert_eq!(ProcessPaymentReconciliation::TITLE_FIELD, "company");
    assert_eq!(
        ProcessPaymentReconciliation::FIELD_ORDER,
        [
            "company",
            "party_type",
            "column_break_io6c",
            "party",
            "receivable_payable_account",
            "default_advance_account",
            "filter_section",
            "from_invoice_date",
            "to_invoice_date",
            "column_break_kegk",
            "from_payment_date",
            "to_payment_date",
            "column_break_uj04",
            "cost_center",
            "bank_cash_account",
            "section_break_2n02",
            "status",
            "error_log",
            "section_break_a8yx",
            "amended_from",
        ]
    );
    assert!(ProcessPaymentReconciliation::EDITABLE_GRID);
    assert!(ProcessPaymentReconciliation::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ProcessPaymentReconciliation::IS_SUBMITTABLE);

    assert_eq!(
        ProcessPaymentReconciliation::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_io6c"),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .required()
                .in_list_view(),
            FieldSpec::link("receivable_payable_account", "Receivable/Payable Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("default_advance_account", "Default Advance Account")
                .options("Account")
                .required()
                .depends_on("eval:doc.party")
                .mandatory_depends_on("doc.party_type")
                .description("Only 'Payment Entries' made against this advance account are supported.")
                .documentation_url("https://docs.erpnext.com/docs/user/manual/en/advance-in-separate-party-account"),
            FieldSpec::section_break("filter_section").label("Filters"),
            FieldSpec::date("from_invoice_date", "From Invoice Date"),
            FieldSpec::date("to_invoice_date", "To Invoice Date"),
            FieldSpec::column_break("column_break_kegk"),
            FieldSpec::date("from_payment_date", "From Payment Date"),
            FieldSpec::date("to_payment_date", "To Payment Date"),
            FieldSpec::column_break("column_break_uj04"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::link("bank_cash_account", "Bank/Cash Account").options("Account"),
            FieldSpec::section_break("section_break_2n02").label("Status"),
            FieldSpec::select("status", "Status")
                .options(
                    "\nQueued\nRunning\nPaused\nCompleted\nPartially Reconciled\nFailed\nCancelled",
                )
                .read_only()
                .allow_on_submit(),
            FieldSpec::long_text("error_log", "Error Log").depends_on("eval:doc.error_log"),
            FieldSpec::section_break("section_break_a8yx"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Payment Reconciliation")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn process_payment_reconciliation_preserves_lifecycle_hooks() {
    let mut doc = ProcessPaymentReconciliation::new(
        "PPR-0001",
        "_Test Company",
        "Customer",
        "_Test Customer",
        "Debtors - TC",
        "Advances - TC",
    );
    doc.status = Some("Running".to_string());
    doc.error_log = Some("existing error".to_string());

    assert_eq!(doc.doctype(), "Process Payment Reconciliation");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        [
            "on_discard",
            "validate",
            "before_save",
            "on_submit",
            "on_cancel",
        ]
    );

    doc.before_save();
    assert_eq!(doc.status.as_deref(), Some(""));
    assert_eq!(doc.error_log.as_deref(), Some(""));

    doc.on_submit();
    assert_eq!(doc.status.as_deref(), Some("Queued"));
    assert_eq!(doc.error_log, None);

    let cancel = doc.on_cancel(Some("PPRLOG-0001"));
    assert_eq!(doc.status.as_deref(), Some("Cancelled"));
    assert_eq!(cancel.log_name.as_deref(), Some("PPRLOG-0001"));
    assert_eq!(cancel.log_status.as_deref(), Some("Cancelled"));

    doc.status = Some("Running".to_string());
    doc.on_discard();
    assert_eq!(doc.status.as_deref(), Some("Cancelled"));
}

#[test]
fn process_payment_reconciliation_validates_account_companies() {
    let doc = ProcessPaymentReconciliation::new(
        "PPR-0001",
        "_Test Company",
        "Customer",
        "_Test Customer",
        "Debtors - TC",
        "Advances - TC",
    )
    .with_bank_cash_account("Cash - TC");

    assert!(doc
        .validate(Some("_Test Company"), Some("_Test Company"))
        .is_ok());

    assert_eq!(
        doc.validate(Some("Other Company"), Some("_Test Company")),
        Err(
            "Receivable/Payable Account: Debtors - TC doesn't belong to company _Test Company"
                .to_string()
        )
    );
    assert_eq!(
        doc.validate(Some("_Test Company"), Some("Other Company")),
        Err("Bank/Cash Account Cash - TC doesn't belong to company _Test Company".to_string())
    );
}

#[test]
fn process_payment_reconciliation_dashboard_and_indicators_match_erpnext() {
    let dashboard = ProcessPaymentReconciliation::dashboard_data();
    assert_eq!(dashboard.fieldname, "process_pr");
    assert_eq!(dashboard.transactions[0].label, "Reconciliation Logs");
    assert_eq!(
        dashboard.transactions[0].items,
        ["Process Payment Reconciliation Log"]
    );

    let running = ProcessPaymentReconciliation::indicator_for_status("Running").unwrap();
    assert_eq!(running.status, "Running");
    assert_eq!(running.color, "blue");
    assert_eq!(running.filter, "status,=,Running");

    let failed = ProcessPaymentReconciliation::indicator_for_status("Failed").unwrap();
    assert_eq!(failed.color, "red");
    assert!(ProcessPaymentReconciliation::indicator_for_status("Cancelled").is_none());
}

#[test]
fn process_payment_reconciliation_progress_and_pr_instance_match_helpers() {
    assert_eq!(
        ProcessPaymentReconciliation::reconciled_count(Some((3, 10))),
        Some((3, 10))
    );
    assert_eq!(ProcessPaymentReconciliation::reconciled_count(None), None);

    let doc = ProcessPaymentReconciliation::new(
        "PPR-0001",
        "_Test Company",
        "Customer",
        "_Test Customer",
        "Debtors - TC",
        "Advances - TC",
    );
    let seed = doc.payment_reconciliation_seed();
    assert_eq!(seed.company.as_deref(), Some("_Test Company"));
    assert_eq!(seed.party_type.as_deref(), Some("Customer"));
    assert_eq!(seed.party.as_deref(), Some("_Test Customer"));
    assert_eq!(
        seed.receivable_payable_account.as_deref(),
        Some("Debtors - TC")
    );
    assert_eq!(
        seed.default_advance_account.as_deref(),
        Some("Advances - TC")
    );
    assert_eq!(seed.invoice_limit, 1000);
    assert_eq!(seed.payment_limit, 1000);
}

#[test]
fn process_payment_reconciliation_allocation_helpers_match_erpnext_ordering() {
    let allocations = vec![
        Allocation::new(1, "Payment Entry", "PE-001", false),
        Allocation::new(2, "Payment Entry", "PE-001", false),
        Allocation::new(3, "Journal Entry", "JV-001", false),
        Allocation::new(4, "Payment Entry", "PE-001", true),
    ];

    let next = get_next_allocation(&allocations);
    assert_eq!(next.len(), 2);
    assert_eq!(next[0].idx, 1);
    assert_eq!(next[1].idx, 2);
    assert_eq!(
        reconcile_job_name("PPR-0001", &next),
        "process_PPR-0001_reconcile_allocation_1_2"
    );
    assert_eq!(
        reconcile_job_name("PPR-0001", &[]),
        "process_PPR-0001_reconcile"
    );
    assert!(get_next_allocation(&[Allocation::new(1, "Payment Entry", "PE-001", true)]).is_empty());
}
