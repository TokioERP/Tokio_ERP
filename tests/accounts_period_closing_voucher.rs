use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::period_closing_voucher::period_closing_voucher::{
    get_period_start_end_date, process_cancellation_plan, process_gl_and_closing_entries_plan,
    AccountBalance, ClosingProcessPlan, LifecycleAction, PcvGlEntryRow, PeriodClosingVoucher,
    PeriodClosingVoucherContext, PeriodClosingVoucherError, ProcessCancellationPlan,
    ProcessPcvDocumentPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_doc() -> PeriodClosingVoucher {
    PeriodClosingVoucher {
        name: Some("PCV-0001".to_string()),
        company: Some("_Test Company".to_string()),
        fiscal_year: Some("FY2026".to_string()),
        period_start_date: Some("2026-04-01".to_string()),
        period_end_date: Some("2026-04-30".to_string()),
        closing_account_head: Some("Retained Earnings - TC".to_string()),
        remarks: Some("Close April".to_string()),
        ..PeriodClosingVoucher::default()
    }
}

fn base_context() -> PeriodClosingVoucherContext {
    PeriodClosingVoucherContext {
        fy_start_date: "2026-04-01".to_string(),
        fy_end_date: "2027-03-31".to_string(),
        previous_closed_period_end_date: None,
        previous_fiscal_year: Some(("FY2025".to_string(), "2025-04-01".to_string())),
        previous_fiscal_year_closed: true,
        gle_exists_in_previous_year: true,
        future_closing_voucher: None,
        closing_account_root_type: Some("Equity".to_string()),
        closing_account_currency: Some("USD".to_string()),
        company_currency: Some("USD".to_string()),
        use_legacy_controller_for_pcv: false,
        gl_entry_estimated_count: 80_000,
        gle_count_against_current_pcv: 100,
        closing_account_currency_from_account: Some("USD".to_string()),
        accounting_dimensions: vec![
            "cost_center".to_string(),
            "finance_book".to_string(),
            "project".to_string(),
        ],
    }
}

fn balance(
    debit: f64,
    credit: f64,
    debit_in_account_currency: f64,
    credit_in_account_currency: f64,
    currency: &str,
) -> AccountBalance {
    AccountBalance {
        debit,
        credit,
        debit_in_account_currency,
        credit_in_account_currency,
        account_currency: currency.to_string(),
        ..AccountBalance::default()
    }
}

#[test]
fn period_closing_voucher_metadata_matches_erpnext_json() {
    assert_eq!(PeriodClosingVoucher::DOCTYPE, "Period Closing Voucher");
    assert_eq!(PeriodClosingVoucher::MODULE, "Accounts");
    assert_eq!(PeriodClosingVoucher::AUTONAME, "ACC-PCV-.YYYY.-.#####");
    assert_eq!(
        PeriodClosingVoucher::FIELD_ORDER,
        [
            "transaction_date",
            "company",
            "fiscal_year",
            "period_start_date",
            "period_end_date",
            "amended_from",
            "column_break1",
            "closing_account_head",
            "gle_processing_status",
            "remarks",
            "error_message",
        ]
    );
    assert!(PeriodClosingVoucher::IS_SUBMITTABLE);
    assert_eq!(
        PeriodClosingVoucher::SEARCH_FIELDS,
        "fiscal_year, period_start_date, period_end_date"
    );

    let doc = PeriodClosingVoucher::default();
    assert_eq!(doc.doctype(), "Period Closing Voucher");
    assert_eq!(doc.module(), "Accounts");
    assert!(PeriodClosingVoucher::fields().contains(
        &FieldSpec::link("closing_account_head", "Closing Account Head")
            .options("Account")
            .description(
                "The account head under Liability or Equity, in which Profit/Loss will be booked"
            )
            .required()
    ));
    assert!(PeriodClosingVoucher::fields().contains(
        &FieldSpec::select("gle_processing_status", "GL Entry Processing Status")
            .options("In Progress\nCompleted\nFailed")
            .depends_on("eval:doc.docstatus!=0")
            .read_only()
    ));
}

#[test]
fn period_closing_voucher_validate_matches_erpnext_date_previous_future_and_account_guards() {
    let mut doc = base_doc();
    let mut ctx = base_context();
    doc.validate(&ctx).unwrap();
    assert_eq!(doc.fy_start_date.as_deref(), Some("2026-04-01"));
    assert_eq!(doc.fy_end_date.as_deref(), Some("2027-03-31"));

    doc.period_start_date = Some("2026-04-02".to_string());
    assert_eq!(
        doc.validate(&ctx).unwrap_err(),
        PeriodClosingVoucherError::Validation("Period Start Date must be 2026-04-01".to_string())
    );

    doc = base_doc();
    ctx.previous_fiscal_year_closed = false;
    assert_eq!(
        doc.validate(&ctx).unwrap_err(),
        PeriodClosingVoucherError::Validation(
            "Previous Year is not closed, please close it first".to_string()
        )
    );

    ctx = base_context();
    ctx.future_closing_voucher = Some("PCV-0002".to_string());
    assert_eq!(
        doc.validate(&ctx).unwrap_err(),
        PeriodClosingVoucherError::Validation(
            "You cannot create this document because another Period Closing Entry PCV-0002 exists after 2026-04-30"
                .to_string()
        )
    );

    ctx = base_context();
    ctx.closing_account_root_type = Some("Asset".to_string());
    assert_eq!(
        doc.validate(&ctx).unwrap_err(),
        PeriodClosingVoucherError::Validation(
            "Closing Account Retained Earnings - TC must be of type Liability / Equity".to_string()
        )
    );

    ctx = base_context();
    ctx.closing_account_currency = Some("EUR".to_string());
    assert_eq!(
        doc.validate(&ctx).unwrap_err(),
        PeriodClosingVoucherError::Validation(
            "Currency of the Closing Account must be USD".to_string()
        )
    );
}

#[test]
fn period_closing_voucher_lifecycle_plans_match_erpnext_submit_cancel_branches() {
    let doc = base_doc();
    let mut ctx = base_context();

    assert_eq!(
        doc.on_submit(&ctx),
        LifecycleAction::CreateAndSubmitProcessPeriodClosingVoucher {
            parent_pcv: "PCV-0001".to_string()
        }
    );

    ctx.use_legacy_controller_for_pcv = true;
    assert_eq!(
        doc.on_submit(&ctx),
        LifecycleAction::ProcessGlAndClosingEntriesInline
    );

    ctx.gl_entry_estimated_count = 100_001;
    assert_eq!(
        doc.make_gl_entries(&ctx),
        LifecycleAction::EnqueueGlAndClosingEntries {
            timeout_seconds: 1800
        }
    );

    ctx.gle_count_against_current_pcv = 5001;
    assert_eq!(
        doc.cancel_gl_entries(&ctx),
        LifecycleAction::EnqueueCancellation {
            queue: "long",
            enqueue_after_commit: true
        }
    );
    ctx.use_legacy_controller_for_pcv = false;
    assert_eq!(
        doc.on_cancel(&ctx),
        vec![
            LifecycleAction::CancelProcessPeriodClosingVoucherDocs,
            LifecycleAction::SetGleProcessingStatus("In Progress"),
            LifecycleAction::EnqueueCancellation {
                queue: "long",
                enqueue_after_commit: true
            },
        ]
    );
    assert_eq!(
        PeriodClosingVoucher::ignore_linked_doctypes(),
        [
            "GL Entry",
            "Stock Ledger Entry",
            "Payment Ledger Entry",
            "Account Closing Balance",
            "Process Period Closing Voucher",
        ]
    );
    assert_eq!(
        doc.cancel_process_pcv_docs_plan(),
        ProcessPcvDocumentPlan {
            doctype: "Process Period Closing Voucher",
            parent_pcv: "PCV-0001".to_string(),
            docstatus: vec![1],
            action: "cancel",
        }
    );
    assert_eq!(
        doc.on_trash_delete_process_pcv_docs_plan(),
        ProcessPcvDocumentPlan {
            doctype: "Process Period Closing Voucher",
            parent_pcv: "PCV-0001".to_string(),
            docstatus: vec![1, 2],
            action: "delete",
        }
    );
}

#[test]
fn period_closing_voucher_gl_and_closing_entries_match_erpnext_balance_logic() {
    let doc = base_doc();
    let dimensions = vec![
        "Main - TC".to_string(),
        "IFRS".to_string(),
        "PROJ-1".to_string(),
    ];
    let mut balances = BTreeMap::new();
    balances.insert(
        "Sales - TC".to_string(),
        balance(0.0, 500.0, 0.0, 500.0, "USD"),
    );
    balances.insert(
        "Expense - TC".to_string(),
        balance(125.0, 25.0, 125.0, 25.0, "USD"),
    );
    balances.insert(
        "balances".to_string(),
        AccountBalance {
            balance_in_account_currency: -400.0,
            balance_in_company_currency: -400.0,
            ..AccountBalance::default()
        },
    );

    let rows = doc.get_pcv_gl_entries(&[(dimensions.clone(), balances)], "USD");
    assert_eq!(rows.len(), 3);
    assert_eq!(
        rows[0],
        PcvGlEntryRow {
            closing_date: None,
            company: "_Test Company".to_string(),
            posting_date: "2026-04-30".to_string(),
            account: "Expense - TC".to_string(),
            account_currency: "USD".to_string(),
            debit_in_account_currency: 0.0,
            debit: 0.0,
            credit_in_account_currency: 100.0,
            credit: 100.0,
            is_period_closing_voucher_entry: true,
            voucher_type: "Period Closing Voucher".to_string(),
            voucher_no: "PCV-0001".to_string(),
            fiscal_year: "FY2026".to_string(),
            remarks: "Close April".to_string(),
            is_opening: "No".to_string(),
            dimensions: BTreeMap::from([
                ("cost_center".to_string(), "Main - TC".to_string()),
                ("finance_book".to_string(), "IFRS".to_string()),
                ("project".to_string(), "PROJ-1".to_string()),
            ]),
            period_closing_voucher: None,
        }
    );
    assert_eq!(rows[2].account, "Retained Earnings - TC");
    assert_eq!(rows[2].debit, 0.0);
    assert_eq!(rows[2].credit, 400.0);

    let closing_entries = doc.get_closing_entries_for_pl_accounts(&rows[0..2]);
    assert_eq!(closing_entries.len(), 4);
    assert_eq!(closing_entries[2].debit, rows[0].credit);
    assert_eq!(closing_entries[2].credit, rows[0].debit);
    assert!(!closing_entries[2].is_period_closing_voucher_entry);
    assert_eq!(
        closing_entries[2].period_closing_voucher.as_deref(),
        Some("PCV-0001")
    );
    assert_eq!(
        doc.get_closing_entries_for_closing_account(&rows[2..])
            .len(),
        1
    );

    assert_eq!(
        PeriodClosingVoucher::accounting_dimension_fields(&["branch".to_string()]),
        vec![
            "cost_center".to_string(),
            "finance_book".to_string(),
            "project".to_string(),
            "branch".to_string(),
        ]
    );

    let mut acc_bal_dict = BTreeMap::new();
    doc.set_account_balance_dict(&rows[0], &mut acc_bal_dict);
    doc.set_account_balance_dict(&rows[1], &mut acc_bal_dict);
    let key = vec![
        "Main - TC".to_string(),
        "IFRS".to_string(),
        "PROJ-1".to_string(),
    ];
    assert_eq!(
        acc_bal_dict[&key]["balances"].balance_in_company_currency,
        rows[0].debit - rows[0].credit + rows[1].debit - rows[1].credit
    );

    let bs_entries = doc.get_closing_entries_for_balance_sheet_accounts(&[(
        key,
        acc_bal_dict.values().next().unwrap().clone(),
    )]);
    assert_eq!(bs_entries.len(), 2);
    assert_eq!(bs_entries[0].closing_date.as_deref(), Some("2026-04-30"));
    assert!(!bs_entries[0].is_period_closing_voucher_entry);
}

#[test]
fn period_closing_voucher_queries_processing_and_period_helpers_match_erpnext() {
    let doc = base_doc();
    let ctx = base_context();
    assert_eq!(
        doc.gl_entries_for_current_period_query("Profit and Loss", false, false, &ctx.accounting_dimensions),
        "SELECT name, posting_date, account, account_currency, debit_in_account_currency, credit_in_account_currency, debit, credit, cost_center, finance_book, project FROM `tabGL Entry` WHERE posting_date BETWEEN '2026-04-01' AND '2026-04-30' and is_opening = 'No' AND company = _Test Company AND voucher_type != 'Period Closing Voucher' AND account report_type = Profit and Loss AND is_cancelled = 0"
    );
    assert_eq!(
        doc.gl_entries_for_current_period_query("Balance Sheet", true, true, &ctx.accounting_dimensions),
        "SELECT name, posting_date, account, account_currency, debit_in_account_currency, credit_in_account_currency, debit, credit, cost_center, finance_book, project FROM `tabGL Entry` WHERE is_opening = 'Yes' AND company = _Test Company AND voucher_type != 'Period Closing Voucher' AND account report_type = Balance Sheet AND is_cancelled = 0 AS ITERATOR"
    );
    assert_eq!(
        get_period_start_end_date("2026-04-01", "2027-03-31", Some("2026-06-30")),
        ("2026-07-01".to_string(), "2027-03-31".to_string())
    );
    assert_eq!(
        doc.get_future_closing_voucher_filter(),
        BTreeMap::from([
            ("period_end_date".to_string(), ">2026-04-30".to_string()),
            ("docstatus".to_string(), "1".to_string()),
            ("company".to_string(), "_Test Company".to_string()),
        ])
    );
    assert!(doc.is_first_period_closing_voucher(None));
    assert!(doc.is_first_period_closing_voucher(Some("PCV-0001")));
    assert!(!doc.is_first_period_closing_voucher(Some("PCV-0000")));
    assert_eq!(
        process_gl_and_closing_entries_plan("PCV-0001"),
        ClosingProcessPlan {
            voucher_name: "PCV-0001".to_string(),
            make_gl_entries_merge_entries: false,
            make_closing_entries: true,
            success_status: "Completed",
            failure_status: "Failed",
        }
    );
    assert_eq!(
        process_cancellation_plan("PCV-0001"),
        ProcessCancellationPlan {
            voucher_type: "Period Closing Voucher",
            voucher_no: "PCV-0001".to_string(),
            delete_closing_entries_doctype: "Account Closing Balance",
            success_status: "Completed",
            failure_status: "Failed",
        }
    );
    assert_eq!(
        PeriodClosingVoucher::delete_closing_entries_filter("PCV-0001"),
        ("period_closing_voucher", "PCV-0001")
    );
}
