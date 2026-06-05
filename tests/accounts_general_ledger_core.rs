use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::general_ledger::{
    check_freezing_date, get_debit_credit_allowance, get_debit_credit_difference,
    get_merge_properties, make_acc_dimensions_offsetting_entry, make_entry_plan,
    make_gl_entries_plan, make_reverse_gl_entries_plan, process_debit_credit_difference,
    process_gl_map, validate_against_pcv, validate_allowed_dimensions, validate_disabled_accounts,
    AccountingDimensionOffset, DimensionFilterRule, DimensionPolicy, EntrySubmitPlan,
    GeneralLedgerContext, GeneralLedgerError, GlEntry, MakeGlEntriesPlan, ReverseGlPlan,
    RoundOffSettings, SaveEntriesPlan,
};

fn gle(account: &str, debit: f64, credit: f64, cost_center: &str) -> GlEntry {
    GlEntry {
        company: "TC".to_string(),
        account: account.to_string(),
        posting_date: "2026-05-15".to_string(),
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: "SINV-0001".to_string(),
        cost_center: Some(cost_center.to_string()),
        debit,
        credit,
        debit_in_account_currency: debit,
        credit_in_account_currency: credit,
        debit_in_transaction_currency: debit,
        credit_in_transaction_currency: credit,
        remarks: Some("row".to_string()),
        ..GlEntry::default()
    }
}

#[test]
fn general_ledger_process_map_distribution_merge_and_negative_toggle_match_erpnext() {
    let context = GeneralLedgerContext {
        precision: 2,
        round_off_account: Some("Round Off - TC".to_string()),
        cost_center_allocations: BTreeMap::from([(
            "Main - TC".to_string(),
            vec![
                ("North - TC".to_string(), 60.0),
                ("South - TC".to_string(), 40.0),
            ],
        )]),
        exchange_gain_loss_journal_entries: BTreeSet::from(["JE-EX".to_string()]),
        ..GeneralLedgerContext::default()
    };

    let mut income_a = gle("Income - TC", 0.0, 100.0, "Main - TC");
    income_a.party_type = Some("Customer".to_string());
    income_a.party = Some("CUST-001".to_string());
    let mut income_b = income_a.clone();
    income_b.credit = 50.0;
    income_b.credit_in_account_currency = 50.0;
    income_b.credit_in_transaction_currency = 50.0;
    let round_off = gle("Round Off - TC", 0.0, 1.0, "Main - TC");
    let negative = GlEntry {
        debit: -25.0,
        credit: 0.0,
        debit_in_account_currency: -25.0,
        debit_in_transaction_currency: -25.0,
        cost_center: Some("Direct - TC".to_string()),
        ..gle("Expense - TC", 0.0, 0.0, "Direct - TC")
    };

    let processed = process_gl_map(
        vec![income_a, income_b, round_off, negative],
        true,
        &context,
    );

    assert_eq!(processed.len(), 4);
    let north = processed
        .iter()
        .find(|entry| {
            entry.account == "Income - TC" && entry.cost_center.as_deref() == Some("North - TC")
        })
        .unwrap();
    assert_eq!(north.credit, 90.0);
    assert_eq!(north.credit_in_account_currency, 90.0);
    let south = processed
        .iter()
        .find(|entry| {
            entry.account == "Income - TC" && entry.cost_center.as_deref() == Some("South - TC")
        })
        .unwrap();
    assert_eq!(south.credit, 60.0);
    assert_eq!(south.credit_in_transaction_currency, 60.0);
    assert_eq!(
        processed
            .iter()
            .find(|entry| entry.account == "Round Off - TC")
            .unwrap()
            .cost_center
            .as_deref(),
        Some("North - TC")
    );
    let negative = processed
        .iter()
        .find(|entry| entry.account == "Expense - TC")
        .unwrap();
    assert_eq!((negative.debit, negative.credit), (0.0, 25.0));
    assert_eq!(
        get_merge_properties(&["department".to_string()]),
        vec![
            "account",
            "cost_center",
            "party",
            "party_type",
            "voucher_detail_no",
            "against_voucher",
            "against_voucher_type",
            "project",
            "finance_book",
            "voucher_no",
            "advance_voucher_type",
            "advance_voucher_no",
            "department",
        ]
    );
}

#[test]
fn general_ledger_offset_roundoff_reverse_and_validation_helpers_match_erpnext() {
    let mut gl_map = vec![gle("Expense - TC", 100.0, 0.0, "Main - TC")];
    make_acc_dimensions_offsetting_entry(
        &mut gl_map,
        &[AccountingDimensionOffset {
            fieldname: "department".to_string(),
            name: "Department".to_string(),
            offsetting_account: "Department Offset - TC".to_string(),
            account_currency: "USD".to_string(),
        }],
    );
    assert_eq!(gl_map.len(), 2);
    assert_eq!(gl_map[1].account, "Department Offset - TC");
    assert_eq!((gl_map[1].debit, gl_map[1].credit), (0.0, 100.0));
    assert_eq!(
        gl_map[1].remarks.as_deref(),
        Some("Offsetting for Accounting Dimension - Department")
    );
    assert_eq!(gl_map[1].party, None);

    assert_eq!(get_debit_credit_allowance("Journal Entry", 2), 0.05);
    assert_eq!(get_debit_credit_allowance("Sales Invoice", 2), 0.5);
    assert_eq!(get_debit_credit_difference(&mut gl_map, 2), (0.0, 200.0));

    let mut diff_map = vec![
        gle("Receivable - TC", 100.0, 0.0, "Main - TC"),
        gle("Income - TC", 0.0, 99.99, "Main - TC"),
    ];
    process_debit_credit_difference(
        &mut diff_map,
        2,
        RoundOffSettings {
            round_off_account: Some("Round Off - TC".to_string()),
            round_off_cost_center: Some("Main - TC".to_string()),
            round_off_for_opening: None,
            default_expense_account: None,
        },
        false,
    )
    .unwrap();
    let round_off = diff_map
        .iter()
        .find(|entry| entry.account == "Round Off - TC")
        .unwrap();
    assert_eq!((round_off.debit, round_off.credit), (0.0, 0.01));

    let reverse = make_reverse_gl_entries_plan(
        &[gle("Receivable - TC", 100.0, 0.0, "Main - TC")],
        false,
        None,
    );
    assert_eq!(
        reverse,
        ReverseGlPlan {
            cancel_original_entries: true,
            partial_cancel: false,
            reversed_entries: vec![GlEntry {
                name: None,
                debit: 0.0,
                credit: 100.0,
                debit_in_account_currency: 0.0,
                credit_in_account_currency: 100.0,
                debit_in_transaction_currency: 0.0,
                credit_in_transaction_currency: 100.0,
                remarks: Some("On cancellation of SINV-0001".to_string()),
                is_cancelled: 1,
                ..gle("Receivable - TC", 100.0, 0.0, "Main - TC")
            }],
        }
    );

    assert_eq!(
        validate_disabled_accounts(&gl_map, &BTreeSet::from(["Expense - TC".to_string()]))
            .unwrap_err(),
        GeneralLedgerError::Validation(
            "Cannot create accounting entries against disabled accounts: <br><b>Expense - TC</b>"
                .to_string()
        )
    );

    let mut entry = gle("Expense - TC", 10.0, 0.0, "Main - TC");
    assert_eq!(
        validate_allowed_dimensions(
            &entry,
            &BTreeMap::from([(
                ("department".to_string(), "Expense - TC".to_string()),
                DimensionFilterRule {
                    is_mandatory: true,
                    policy: DimensionPolicy::Allow,
                    allowed_dimensions: BTreeSet::from(["Sales".to_string()]),
                },
            )])
        )
        .unwrap_err(),
        GeneralLedgerError::MandatoryAccountDimension(
            "Department is mandatory for account Expense - TC".to_string()
        )
    );
    entry
        .dimensions
        .insert("department".to_string(), "Admin".to_string());
    assert!(matches!(
        validate_allowed_dimensions(
            &entry,
            &BTreeMap::from([(
                ("department".to_string(), "Expense - TC".to_string()),
                DimensionFilterRule {
                    is_mandatory: true,
                    policy: DimensionPolicy::Allow,
                    allowed_dimensions: BTreeSet::from(["Sales".to_string()]),
                },
            )])
        ),
        Err(GeneralLedgerError::InvalidAccountDimension(_))
    ));

    assert_eq!(
        check_freezing_date(
            "2026-01-01",
            Some("2026-01-31"),
            Some("Accounts Manager"),
            &BTreeSet::from(["Accounts User".to_string()]),
            "Administrator",
            false,
        )
        .unwrap_err(),
        GeneralLedgerError::Validation(
            "You are not authorized to add or update entries before 2026-01-31".to_string()
        )
    );
    assert_eq!(
        validate_against_pcv(true, "2026-05-01", true, Some("2026-04-30")).unwrap_err(),
        GeneralLedgerError::Validation(
            "Opening Entry can not be created after Period Closing Voucher is created.".to_string()
        )
    );
}

#[test]
fn general_ledger_make_entries_orchestration_plan_matches_erpnext() {
    let context = GeneralLedgerContext {
        precision: 2,
        round_off_settings: RoundOffSettings {
            round_off_account: Some("Round Off - TC".to_string()),
            round_off_cost_center: Some("Main - TC".to_string()),
            round_off_for_opening: None,
            default_expense_account: None,
        },
        ..GeneralLedgerContext::default()
    };

    let plan = make_gl_entries_plan(
        vec![
            gle("Receivable - TC", 100.0, 0.0, "Main - TC"),
            gle("Income - TC", 0.0, 100.0, "Main - TC"),
        ],
        false,
        false,
        true,
        "No",
        false,
        false,
        &context,
    )
    .unwrap();
    let MakeGlEntriesPlan::Save(save_plan) = plan else {
        panic!("expected save plan");
    };
    assert_eq!(save_plan.validate_budget, true);
    assert_eq!(save_plan.create_payment_ledger_entry, true);
    assert_eq!(save_plan.payment_ledger_cancel, 0);
    assert_eq!(save_plan.adv_adj, false);
    assert_eq!(save_plan.update_outstanding, "No");
    assert_eq!(save_plan.from_repost, false);
    assert_eq!(save_plan.entries.len(), 2);
    assert_eq!(save_plan.entries[0].account, "Receivable - TC");
    assert_eq!(save_plan.entries[0].merge_key[0], "Receivable - TC");
    assert_eq!(save_plan.entries[1].account, "Income - TC");

    let one_sided = make_gl_entries_plan(
        vec![gle("Receivable - TC", 100.0, 0.0, "Main - TC")],
        false,
        false,
        true,
        "Yes",
        false,
        false,
        &context,
    );
    assert_eq!(
        one_sided.unwrap_err(),
        GeneralLedgerError::Validation(
            "Incorrect number of General Ledger Entries found. You might have selected a wrong Account in the transaction."
                .to_string()
        )
    );

    let cancel_plan = make_gl_entries_plan(
        vec![gle("Receivable - TC", 100.0, 0.0, "Main - TC")],
        true,
        true,
        true,
        "Yes",
        false,
        false,
        &context,
    )
    .unwrap();
    assert!(matches!(
        cancel_plan,
        MakeGlEntriesPlan::Reverse(ReverseGlPlan { .. })
    ));

    let pcv = GlEntry {
        voucher_type: "Period Closing Voucher".to_string(),
        voucher_no: "PCV-0001".to_string(),
        ..gle("Retained Earnings - TC", 100.0, 0.0, "Main - TC")
    };
    let pcv_plan = make_gl_entries_plan(
        vec![
            pcv.clone(),
            GlEntry {
                debit: 0.0,
                credit: 100.0,
                debit_in_account_currency: 0.0,
                credit_in_account_currency: 100.0,
                debit_in_transaction_currency: 0.0,
                credit_in_transaction_currency: 100.0,
                account: "Closing Account - TC".to_string(),
                ..pcv.clone()
            },
        ],
        false,
        false,
        true,
        "",
        true,
        false,
        &context,
    )
    .unwrap();
    assert!(matches!(
        pcv_plan,
        MakeGlEntriesPlan::Save(SaveEntriesPlan {
            validate_budget: false,
            create_payment_ledger_entry: false,
            from_repost: true,
            ..
        })
    ));

    assert_eq!(
        make_entry_plan(
            &gle("Income - TC", 0.0, 100.0, "Main - TC"),
            true,
            "",
            false
        ),
        EntrySubmitPlan {
            ignore_permissions: true,
            from_repost: false,
            adv_adj: true,
            update_outstanding: "Yes".to_string(),
            notify_update: false,
            validate_expense_against_budget: true,
            entry: gle("Income - TC", 0.0, 100.0, "Main - TC"),
        }
    );
}
