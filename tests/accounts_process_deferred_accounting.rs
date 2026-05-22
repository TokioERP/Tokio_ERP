use tokio_erp::erpnext::accounts::doctype::process_deferred_accounting::process_deferred_accounting::{
    DeferredAccountingConversion, ProcessDeferredAccounting,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_deferred_accounting_matches_erpnext_metadata() {
    assert_eq!(
        ProcessDeferredAccounting::DOCTYPE,
        "Process Deferred Accounting"
    );
    assert_eq!(ProcessDeferredAccounting::MODULE, "Accounts");
    assert_eq!(ProcessDeferredAccounting::AUTONAME, "ACC-PDA-.#####");
    assert_eq!(
        ProcessDeferredAccounting::FIELD_ORDER,
        [
            "company",
            "type",
            "account",
            "column_break_3",
            "posting_date",
            "start_date",
            "end_date",
            "amended_from",
        ]
    );
    assert!(ProcessDeferredAccounting::EDITABLE_GRID);
    assert!(ProcessDeferredAccounting::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ProcessDeferredAccounting::IS_SUBMITTABLE);

    assert_eq!(
        ProcessDeferredAccounting::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::select("type", "Type")
                .options("\nIncome\nExpense")
                .required()
                .in_list_view(),
            FieldSpec::link("account", "Account")
                .options("Account")
                .depends_on("eval: doc.type"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .required()
                .in_list_view(),
            FieldSpec::date("start_date", "Service Start Date")
                .required()
                .in_list_view(),
            FieldSpec::date("end_date", "Service End Date")
                .required()
                .in_list_view(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Deferred Accounting")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn process_deferred_accounting_preserves_controller_hooks_and_validate() {
    let doc = ProcessDeferredAccounting::new(
        "PDA-0001",
        "2026-05-22",
        "2026-04-01",
        "2026-04-30",
        "Income",
        "_Test Company",
    );

    assert_eq!(doc.doctype(), "Process Deferred Accounting");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), ["validate", "on_submit", "on_cancel"]);
    assert!(doc.validate().is_ok());

    let invalid = ProcessDeferredAccounting::new(
        "PDA-0002",
        "2026-05-22",
        "2026-05-31",
        "2026-05-01",
        "Income",
        "_Test Company",
    );
    assert_eq!(
        invalid.validate(),
        Err("End date cannot be before start date")
    );
}

#[test]
fn process_deferred_accounting_on_submit_matches_erpnext_branching() {
    let mut income = ProcessDeferredAccounting::new(
        "PDA-0001",
        "2026-05-22",
        "2026-04-01",
        "2026-04-30",
        "Income",
        "_Test Company",
    );
    income.account = Some("Deferred Revenue - TC".to_string());

    assert_eq!(
        income.on_submit(),
        DeferredAccountingConversion::RevenueToIncome {
            process_name: "PDA-0001".to_string(),
            start_date: "2026-04-01".to_string(),
            end_date: "2026-04-30".to_string(),
            accounting_type: "Income".to_string(),
            account: Some("Deferred Revenue - TC".to_string()),
            company: "_Test Company".to_string(),
        }
    );

    let expense = ProcessDeferredAccounting::new(
        "PDA-0002",
        "2026-05-22",
        "2026-04-01",
        "2026-04-30",
        "Expense",
        "_Test Company",
    );
    assert_eq!(
        expense.on_submit(),
        DeferredAccountingConversion::ExpenseToExpense {
            process_name: "PDA-0002".to_string(),
            start_date: "2026-04-01".to_string(),
            end_date: "2026-04-30".to_string(),
            accounting_type: "Expense".to_string(),
            account: None,
            company: "_Test Company".to_string(),
        }
    );
}

#[test]
fn process_deferred_accounting_on_cancel_matches_erpnext_gl_cancellation() {
    let doc = ProcessDeferredAccounting::new(
        "PDA-0001",
        "2026-05-22",
        "2026-04-01",
        "2026-04-30",
        "Income",
        "_Test Company",
    );
    let cancellation = doc.on_cancel();

    assert_eq!(cancellation.ignore_linked_doctypes, ["GL Entry"]);
    assert_eq!(
        cancellation.against_voucher_type,
        "Process Deferred Accounting"
    );
    assert_eq!(cancellation.against_voucher, "PDA-0001");
    assert!(cancellation.cancel_gl_entries);
}
