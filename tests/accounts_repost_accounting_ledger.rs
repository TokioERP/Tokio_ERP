use tokio_erp::erpnext::accounts::doctype::repost_accounting_ledger::repost_accounting_ledger::{
    get_allowed_types_from_settings, get_repost_allowed_types,
    validate_docs_for_deferred_accounting, validate_docs_for_voucher_types, RepostAccountingLedger,
    RepostAction, RepostDocCapability, RepostStartContext, RepostSubmitPlan,
    RepostValidationContext, RepostVoucher, VoucherPostingDate,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn repost_accounting_ledger_matches_erpnext_metadata() {
    assert_eq!(RepostAccountingLedger::DOCTYPE, "Repost Accounting Ledger");
    assert_eq!(RepostAccountingLedger::MODULE, "Accounts");
    assert_eq!(
        RepostAccountingLedger::FIELD_ORDER,
        [
            "company",
            "column_break_vpup",
            "delete_cancelled_entries",
            "section_break_metl",
            "vouchers",
            "amended_from",
        ]
    );
    assert!(RepostAccountingLedger::EDITABLE_GRID);
    assert!(RepostAccountingLedger::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(RepostAccountingLedger::IS_SUBMITTABLE);
    assert!(RepostAccountingLedger::TRACK_CHANGES);
    assert_eq!(
        RepostAccountingLedger::fields(),
        vec![
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::column_break("column_break_vpup"),
            FieldSpec::check(
                "delete_cancelled_entries",
                "Delete Cancelled Ledger Entries"
            )
            .default("0"),
            FieldSpec::section_break("section_break_metl"),
            FieldSpec::table("vouchers", "Vouchers").options("Repost Accounting Ledger Items"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Repost Accounting Ledger")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn repost_accounting_ledger_validation_matches_erpnext_guards() {
    let mut doc = RepostAccountingLedger::new("RAL-0001", "_Test Company");
    doc.vouchers = vec![
        RepostVoucher::new("Sales Invoice", "SINV-0001"),
        RepostVoucher::new("Payment Request", "PREQ-0001"),
    ];

    let voucher_error = doc
        .validate_vouchers(&["Sales Invoice", "Purchase Invoice"])
        .unwrap_err();
    assert!(voucher_error.contains("Payment Request"));
    assert!(voucher_error.contains("not allowed to be reposted"));
    assert_eq!(
        validate_docs_for_voucher_types(&["Payment Entry"], &["Sales Invoice"]).unwrap_err(),
        "<b>Payment Entry</b> is not allowed to be reposted. You can enable it by adding it '<b>Allowed Doctype</b>' table in Accounts Settings."
    );

    let no_purchase_docs: [&str; 0] = [];
    let no_deferred_purchase_docs: [&str; 0] = [];
    assert_eq!(
        validate_docs_for_deferred_accounting(
            &["SINV-0001"],
            &no_purchase_docs,
            &["SINV-0001"],
            &no_deferred_purchase_docs,
        )
        .unwrap_err(),
        "Documents: <b>SINV-0001</b> have deferred revenue/expense enabled for them. Cannot repost."
    );

    doc.vouchers = vec![RepostVoucher::new("Sales Invoice", "SINV-0001")];
    let closed_context = RepostValidationContext {
        allowed_types: vec!["Sales Invoice".to_string()],
        latest_period_closing_voucher: Some("2026-05-01".to_string()),
        voucher_posting_dates: vec![VoucherPostingDate::new(
            "Sales Invoice",
            "SINV-0001",
            "2026-04-30",
        )],
        deferred_sales_documents: vec![],
        deferred_purchase_documents: vec![],
    };
    assert_eq!(
        doc.validate(&closed_context).unwrap_err(),
        "Cannot Resubmit Ledger entries for vouchers in Closed fiscal year."
    );
}

#[test]
fn repost_accounting_ledger_submit_and_repost_plans_match_erpnext_branching() {
    let short = RepostAccountingLedger {
        vouchers: vec![RepostVoucher::new("Sales Invoice", "SINV-0001")],
        ..RepostAccountingLedger::new("RAL-0001", "_Test Company")
    };
    assert_eq!(
        short.on_submit(),
        RepostSubmitPlan::StartRepost {
            account_repost_doc: "RAL-0001".to_string()
        }
    );

    let long = RepostAccountingLedger {
        vouchers: (1..=6)
            .map(|idx| RepostVoucher::new("Sales Invoice", format!("SINV-{idx:04}")))
            .collect(),
        ..RepostAccountingLedger::new("RAL-0002", "_Test Company")
    };
    assert_eq!(
        long.on_submit(),
        RepostSubmitPlan::Enqueue {
            method: "erpnext.accounts.doctype.repost_accounting_ledger.repost_accounting_ledger.start_repost".to_string(),
            job_name: "repost_accounting_ledger_RAL-0002".to_string(),
            account_repost_doc: "RAL-0002".to_string(),
            enqueue_after_commit: true,
        }
    );

    let repost = RepostAccountingLedger {
        delete_cancelled_entries: false,
        vouchers: vec![
            RepostVoucher::new("Sales Invoice", "SINV-0001"),
            RepostVoucher::new("Purchase Receipt", "PREC-0001"),
            RepostVoucher::new("Payment Entry", "PAY-0001"),
            RepostVoucher::new("Custom Voucher", "CV-0001"),
        ],
        ..RepostAccountingLedger::new("RAL-0003", "_Test Company")
    };
    let plan = repost.start_repost(&RepostStartContext {
        repost_docstatus: 1,
        hook_allowed_doctypes: vec!["Custom Voucher".to_string()],
        doc_capabilities: vec![RepostDocCapability {
            doctype: "Custom Voucher".to_string(),
            has_make_gl_entries: true,
            make_gl_entries_supports_cancel_arg: false,
        }],
    });
    assert_eq!(plan[0], RepostAction::SetThroughRepostFlag);
    assert!(plan.contains(&RepostAction::MakeGlEntriesOnCancel {
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: "SINV-0001".to_string(),
        from_repost: true,
    }));
    assert!(plan.contains(&RepostAction::ForceSetAgainstIncomeAccount {
        voucher_no: "SINV-0001".to_string()
    }));
    assert!(plan.contains(&RepostAction::MakeGlEntriesFromRepost {
        voucher_type: "Purchase Receipt".to_string(),
        voucher_no: "PREC-0001".to_string(),
    }));
    assert!(plan.contains(&RepostAction::MakeGlEntriesCancelArg {
        voucher_type: "Payment Entry".to_string(),
        voucher_no: "PAY-0001".to_string(),
    }));
    assert!(plan.contains(&RepostAction::MakeReverseGlEntries {
        voucher_type: "Custom Voucher".to_string(),
        voucher_no: "CV-0001".to_string(),
    }));
}

#[test]
fn repost_accounting_ledger_allowed_type_helpers_match_erpnext_queries() {
    assert_eq!(
        get_allowed_types_from_settings(
            &["Sales Invoice".to_string(), "Purchase Invoice".to_string()],
            true,
            &[
                (
                    "Sales Invoice".to_string(),
                    vec!["Sales Invoice Item".to_string()]
                ),
                (
                    "Purchase Invoice".to_string(),
                    vec!["Purchase Invoice Item".to_string()]
                ),
            ],
        ),
        vec![
            "Sales Invoice",
            "Purchase Invoice",
            "Sales Invoice Item",
            "Purchase Invoice Item"
        ]
    );
    assert_eq!(
        get_repost_allowed_types(
            &[
                "Sales Invoice".to_string(),
                "Sales Invoice".to_string(),
                "Purchase Receipt".to_string(),
                "Payment Entry".to_string(),
            ],
            "Invoice"
        ),
        vec!["Sales Invoice"]
    );

    let empty = RepostAccountingLedger::new("RAL-0004", "_Test Company");
    assert_eq!(
        empty.generate_preview(),
        Err("Add vouchers to generate preview.".to_string())
    );
    assert_eq!(empty.custom_hooks(), ["validate", "on_submit"]);
    assert_eq!(empty.doctype(), "Repost Accounting Ledger");
    assert_eq!(empty.module(), "Accounts");
}
