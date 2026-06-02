use tokio_erp::erpnext::accounts::doctype::ledger_merge::ledger_merge::{
    run_ledger_merge, LedgerMerge, LedgerMergeError, LedgerMergeRunResult, LedgerMergeStartContext,
};
use tokio_erp::erpnext::accounts::doctype::ledger_merge_accounts::ledger_merge_accounts::LedgerMergeAccounts;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn ledger_merge_matches_erpnext_metadata() {
    assert_eq!(LedgerMerge::DOCTYPE, "Ledger Merge");
    assert_eq!(LedgerMerge::MODULE, "Accounts");
    assert_eq!(
        LedgerMerge::FIELD_ORDER,
        [
            "section_break_1",
            "root_type",
            "account",
            "account_name",
            "column_break_3",
            "company",
            "status",
            "is_group",
            "section_break_5",
            "merge_accounts",
        ]
    );
    assert_eq!(
        LedgerMerge::AUTONAME,
        "format:{account_name} merger on {creation}"
    );
    assert_eq!(LedgerMerge::NAMING_RULE, "Expression");
    assert!(LedgerMerge::HIDE_TOOLBAR);
    assert_eq!(LedgerMerge::SORT_FIELD, "creation");
    assert_eq!(LedgerMerge::SORT_ORDER, "DESC");
    assert!(LedgerMerge::TRACK_CHANGES);

    assert_eq!(
        LedgerMerge::fields(),
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .depends_on("root_type")
                .required()
                .set_only_once(),
            FieldSpec::section_break("section_break_1"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::table("merge_accounts", "Accounts to Merge")
                .options("Ledger Merge Accounts")
                .required(),
            FieldSpec::section_break("section_break_5").depends_on("account"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .set_only_once(),
            FieldSpec::select("status", "Status")
                .options("Pending\nSuccess\nPartial Success\nError")
                .in_list_view()
                .read_only(),
            FieldSpec::select("root_type", "Root Type")
                .options("\nAsset\nLiability\nIncome\nExpense\nEquity")
                .required()
                .set_only_once(),
            FieldSpec::data("account_name", "Account Name")
                .depends_on("account")
                .fetch_from("account.account_name")
                .fetch_if_empty()
                .read_only()
                .required(),
            FieldSpec::check("is_group", "Is Group")
                .default("0")
                .depends_on("account")
                .fetch_from("account.is_group")
                .read_only(),
        ]
    );
}

#[test]
fn ledger_merge_start_merge_matches_erpnext_enqueue_rules() {
    let ledger_merge = LedgerMerge {
        name: Some("Cash merger on 2026-06-01".to_string()),
        ..Default::default()
    };

    assert_eq!(ledger_merge.doctype(), "Ledger Merge");
    assert_eq!(ledger_merge.module(), "Accounts");

    let plan = ledger_merge
        .start_merge_plan(LedgerMergeStartContext {
            scheduler_inactive: false,
            in_test: false,
            developer_mode: false,
            job_enqueued: false,
        })
        .expect("enqueue plan")
        .expect("not already enqueued");
    assert_eq!(plan.queue, "default");
    assert_eq!(plan.timeout, 6000);
    assert_eq!(plan.event, "ledger_merge");
    assert_eq!(plan.job_id, "ledger_merge::Cash merger on 2026-06-01");
    assert_eq!(plan.docname, "Cash merger on 2026-06-01");
    assert!(!plan.now);

    let test_plan = ledger_merge
        .start_merge_plan(LedgerMergeStartContext {
            scheduler_inactive: true,
            in_test: true,
            developer_mode: false,
            job_enqueued: false,
        })
        .expect("in-test enqueue plan")
        .expect("not already enqueued");
    assert!(test_plan.now);

    assert_eq!(
        ledger_merge.start_merge_plan(LedgerMergeStartContext {
            scheduler_inactive: false,
            in_test: false,
            developer_mode: false,
            job_enqueued: true,
        }),
        Ok(None)
    );
    assert_eq!(
        ledger_merge.start_merge_plan(LedgerMergeStartContext {
            scheduler_inactive: true,
            in_test: false,
            developer_mode: false,
            job_enqueued: false,
        }),
        Err(LedgerMergeError::SchedulerInactive)
    );
}

#[test]
fn ledger_merge_run_matches_erpnext_status_and_progress() {
    let mut ledger_merge = LedgerMerge {
        name: Some("Cash merger on 2026-06-01".to_string()),
        account: Some("Cash - AC".to_string()),
        merge_accounts: vec![
            LedgerMergeAccounts::new("Bank - AC", "Bank"),
            LedgerMergeAccounts::new("Vault - AC", "Vault"),
        ],
        ..Default::default()
    };

    assert_eq!(
        run_ledger_merge(&mut ledger_merge, &[Ok(()), Ok(())]),
        LedgerMergeRunResult {
            status: Some("Success".to_string()),
            successful_merges: 2,
            total: 2,
            progress_events: vec![(1, 2), (2, 2)],
            refresh_published: true,
            errors_logged: 0,
        }
    );
    assert_eq!(ledger_merge.status, "Success");
    assert!(ledger_merge.merge_accounts.iter().all(|row| row.merged));

    let mut partial = LedgerMerge {
        name: Some("Receivable merger on 2026-06-01".to_string()),
        account: Some("Debtors - AC".to_string()),
        merge_accounts: vec![
            LedgerMergeAccounts::new("Old Debtors - AC", "Old Debtors"),
            LedgerMergeAccounts::new("Bad Account - AC", "Bad Account"),
        ],
        ..Default::default()
    };
    let result = run_ledger_merge(&mut partial, &[Ok(()), Err(())]);
    assert_eq!(result.status.as_deref(), Some("Partial Success"));
    assert_eq!(result.successful_merges, 1);
    assert_eq!(result.errors_logged, 1);
    assert_eq!(partial.status, "Partial Success");
    assert!(partial.merge_accounts[0].merged);
    assert!(!partial.merge_accounts[1].merged);
}
