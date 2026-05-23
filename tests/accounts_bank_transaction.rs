use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::bank_transaction::bank_transaction::{
    get_clearance_details, get_payment_doctypes, group_related_bank_gl_entries,
    group_total_allocated_amount, remove_from_bank_transaction_plan, BankGlAllocation,
    BankTransaction, BankTransactionAllocationAction, BankTransactionError, BankTransactionPayment,
    BankTransactionStatus, ClearanceDetails, LinkedBankTransaction, RelatedBankGlEntryRow,
    RemoveFromBankTransactionPlan, TotalAllocatedAmountRow,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_transaction_matches_erpnext_metadata_and_client_payment_doctypes() {
    assert_eq!(BankTransaction::DOCTYPE, "Bank Transaction");
    assert_eq!(BankTransaction::MODULE, "Accounts");
    assert_eq!(BankTransaction::AUTONAME, "naming_series:");
    assert!(BankTransaction::ALLOW_IMPORT);
    assert!(BankTransaction::EDITABLE_GRID);
    assert_eq!(
        BankTransaction::FIELD_ORDER,
        [
            "naming_series",
            "date",
            "column_break_2",
            "status",
            "bank_account",
            "company",
            "amended_from",
            "section_break_4",
            "deposit",
            "withdrawal",
            "column_break_7",
            "currency",
            "section_break_10",
            "description",
            "reference_number",
            "column_break_10",
            "transaction_id",
            "transaction_type",
            "section_break_14",
            "column_break_oufv",
            "payment_entries",
            "section_break_18",
            "allocated_amount",
            "column_break_17",
            "unallocated_amount",
            "party_section",
            "party_type",
            "party",
            "column_break_3czf",
            "bank_party_name",
            "bank_party_account_number",
            "bank_party_iban",
            "extended_bank_statement_section",
            "included_fee",
            "excluded_fee",
        ]
    );

    let fields = BankTransaction::fields();
    assert!(fields.contains(
        &FieldSpec::select("naming_series", "Series")
            .options("ACC-BTN-.YYYY.-")
            .default("ACC-BTN-.YYYY.-")
            .required()
            .no_copy()
            .print_hide()
    ));
    assert!(fields.contains(&FieldSpec::date("date", "Date")));
    assert!(fields.contains(
        &FieldSpec::select("status", "Status")
            .options("\nPending\nSettled\nUnreconciled\nReconciled\nCancelled")
            .default("Pending")
            .in_standard_filter()
    ));
    assert!(fields.contains(
        &FieldSpec::link("bank_account", "Bank Account")
            .options("Bank Account")
            .in_standard_filter()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("deposit", "Deposit")
            .options("currency")
            .oldfield("debit", "Currency")
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::table("payment_entries", "Payment Entries")
            .options("Bank Transaction Payments")
            .allow_on_submit()
    ));
    assert!(fields.contains(
        &FieldSpec::dynamic_link("party")
            .label("Party")
            .options("party_type")
            .allow_on_submit()
    ));

    assert_eq!(
        get_payment_doctypes(),
        [
            "Payment Entry",
            "Journal Entry",
            "Sales Invoice",
            "Purchase Invoice",
            "Bank Transaction",
        ]
    );
}

#[test]
fn bank_transaction_updates_allocated_and_unallocated_amount_like_erpnext() {
    let mut transaction = BankTransaction {
        withdrawal: 1200.0,
        deposit: 200.0,
        payment_entries: vec![
            BankTransactionPayment::new("Payment Entry", "PE-0001", 125.25),
            BankTransactionPayment::new("Journal Entry", "JE-0001", 74.75),
        ],
        ..Default::default()
    };

    transaction.update_allocated_amount();

    assert_eq!(transaction.allocated_amount, 200.0);
    assert_eq!(transaction.unallocated_amount, 800.0);
}

#[test]
fn bank_transaction_set_status_matches_docstatus_and_unallocated_amount() {
    let mut cancelled = BankTransaction {
        docstatus: 2,
        unallocated_amount: 999.0,
        ..Default::default()
    };
    cancelled.set_status();
    assert_eq!(cancelled.status, BankTransactionStatus::Cancelled);

    let mut unreconciled = BankTransaction {
        docstatus: 1,
        unallocated_amount: 0.01,
        ..Default::default()
    };
    unreconciled.set_status();
    assert_eq!(unreconciled.status, BankTransactionStatus::Unreconciled);

    let mut reconciled = BankTransaction {
        docstatus: 1,
        unallocated_amount: 0.0,
        ..Default::default()
    };
    reconciled.set_status();
    assert_eq!(reconciled.status, BankTransactionStatus::Reconciled);
}

#[test]
fn bank_transaction_duplicate_reference_validation_matches_erpnext() {
    let transaction = BankTransaction {
        payment_entries: vec![
            BankTransactionPayment::new("Payment Entry", "PE-0001", 100.0),
            BankTransactionPayment::new("Payment Entry", "PE-0001", 50.0),
        ],
        ..Default::default()
    };

    assert_eq!(
        transaction.validate_duplicate_references(),
        Err(BankTransactionError::DuplicateReference {
            payment_document: "Payment Entry".to_string(),
            payment_entry: "PE-0001".to_string(),
        })
    );
}

#[test]
fn bank_transaction_add_payment_entries_matches_erpnext_zero_allocation_append() {
    let mut transaction = BankTransaction {
        name: Some("BT-0001".to_string()),
        unallocated_amount: 500.0,
        ..Default::default()
    };

    transaction
        .add_payment_entries(&[("Payment Entry", "PE-0001"), ("Journal Entry", "JE-0001")])
        .unwrap();

    assert_eq!(
        transaction.payment_entries,
        vec![
            BankTransactionPayment::new("Payment Entry", "PE-0001", 0.0),
            BankTransactionPayment::new("Journal Entry", "JE-0001", 0.0),
        ]
    );

    transaction.unallocated_amount = 0.0;
    assert_eq!(
        transaction.add_payment_entries(&[("Payment Entry", "PE-0002")]),
        Err(BankTransactionError::AlreadyFullyReconciled {
            name: "BT-0001".to_string(),
        })
    );
}

#[test]
fn bank_transaction_fee_validation_and_excluded_fee_handling_match_erpnext() {
    let transaction = BankTransaction {
        withdrawal: 100.0,
        included_fee: 101.0,
        ..Default::default()
    };
    assert_eq!(
        transaction.validate_included_fee(),
        Err(BankTransactionError::IncludedFeeBiggerThanWithdrawal)
    );

    let mut deposit_fee = BankTransaction {
        deposit: 100.0,
        withdrawal: 0.0,
        included_fee: 2.0,
        excluded_fee: 5.0,
        ..Default::default()
    };
    deposit_fee.handle_excluded_fee().unwrap();
    assert_eq!(deposit_fee.deposit, 95.0);
    assert_eq!(deposit_fee.withdrawal, 0.0);
    assert_eq!(deposit_fee.included_fee, 7.0);
    assert_eq!(deposit_fee.excluded_fee, 0.0);

    let mut withdrawal_fee = BankTransaction {
        deposit: 0.0,
        withdrawal: 100.0,
        included_fee: 2.0,
        excluded_fee: 5.0,
        ..Default::default()
    };
    withdrawal_fee.handle_excluded_fee().unwrap();
    assert_eq!(withdrawal_fee.deposit, 0.0);
    assert_eq!(withdrawal_fee.withdrawal, 105.0);
    assert_eq!(withdrawal_fee.included_fee, 7.0);
    assert_eq!(withdrawal_fee.excluded_fee, 0.0);

    let mut zero_amount_fee = BankTransaction {
        excluded_fee: 5.0,
        ..Default::default()
    };
    zero_amount_fee.handle_excluded_fee().unwrap();
    assert_eq!(zero_amount_fee.withdrawal, 5.0);
    assert_eq!(zero_amount_fee.included_fee, 5.0);

    let mut invalid_fee = BankTransaction {
        deposit: 10.0,
        excluded_fee: 11.0,
        ..Default::default()
    };
    assert_eq!(
        invalid_fee.handle_excluded_fee(),
        Err(BankTransactionError::ExcludedFeeBiggerThanDeposit)
    );

    let mut bidirectional = BankTransaction {
        deposit: 10.0,
        withdrawal: 10.0,
        excluded_fee: 1.0,
        ..Default::default()
    };
    assert_eq!(
        bidirectional.handle_excluded_fee(),
        Err(BankTransactionError::DepositAndWithdrawalWithExcludedFee)
    );
}

#[test]
fn bank_transaction_controller_hooks_match_erpnext_lifecycle() {
    let doc = BankTransaction::default();

    assert_eq!(doc.doctype(), "Bank Transaction");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        [
            "before_validate",
            "validate",
            "before_submit",
            "before_update_after_submit",
            "on_cancel",
            "on_discard",
        ]
    );
}

#[test]
fn bank_transaction_clearance_details_for_linked_bank_transaction_match_erpnext() {
    let transaction = BankTransaction {
        date: Some("2026-05-10".to_string()),
        ..Default::default()
    };
    let payment_entry = BankTransactionPayment::new("Bank Transaction", "BT-REFUND", 0.0);

    assert_eq!(
        get_clearance_details(
            &transaction,
            &payment_entry,
            Default::default(),
            Default::default(),
            "Bank - TC",
            Some(LinkedBankTransaction {
                unallocated_amount: -250.0,
                gl_bank_account: "Bank - TC".to_string(),
            }),
        ),
        Ok(ClearanceDetails {
            allocable_amount: 250.0,
            should_clear: true,
            clearance_date: "2026-05-10".to_string(),
        })
    );

    assert_eq!(
        get_clearance_details(
            &transaction,
            &payment_entry,
            Default::default(),
            Default::default(),
            "Bank - TC",
            Some(LinkedBankTransaction {
                unallocated_amount: 250.0,
                gl_bank_account: "Other Bank - TC".to_string(),
            }),
        ),
        Err(BankTransactionError::LinkedBankAccountMismatch {
            linked_bank_account: "Other Bank - TC".to_string(),
            payment_entry: "BT-REFUND".to_string(),
            gl_bank_account: "Bank - TC".to_string(),
        })
    );
}

#[test]
fn bank_transaction_clearance_details_for_gl_entries_match_erpnext() {
    let transaction = BankTransaction {
        date: Some("2026-05-10".to_string()),
        ..Default::default()
    };
    let payment_entry = BankTransactionPayment::new("Payment Entry", "PE-0001", 0.0);

    let gl_entries = [
        ("Bank - TC".to_string(), 700.0),
        ("Charges - TC".to_string(), 50.0),
    ]
    .into_iter()
    .collect();
    let bt_allocations = [
        (
            "Bank - TC".to_string(),
            BankGlAllocation {
                total: 200.0,
                latest_date: Some("2026-05-12".to_string()),
            },
        ),
        (
            "Charges - TC".to_string(),
            BankGlAllocation {
                total: 50.0,
                latest_date: None,
            },
        ),
    ]
    .into_iter()
    .collect();

    assert_eq!(
        get_clearance_details(
            &transaction,
            &payment_entry,
            bt_allocations,
            gl_entries,
            "Bank - TC",
            None,
        ),
        Ok(ClearanceDetails {
            allocable_amount: 500.0,
            should_clear: true,
            clearance_date: "2026-05-12".to_string(),
        })
    );
}

#[test]
fn bank_transaction_clearance_details_preserve_erpnext_error_and_partial_clear_rules() {
    let transaction = BankTransaction {
        date: Some("2026-05-10".to_string()),
        ..Default::default()
    };
    let payment_entry = BankTransactionPayment::new("Payment Entry", "PE-0001", 0.0);

    assert_eq!(
        get_clearance_details(
            &transaction,
            &payment_entry,
            Default::default(),
            [("Other Bank - TC".to_string(), 100.0)]
                .into_iter()
                .collect(),
            "Bank - TC",
            None,
        ),
        Err(BankTransactionError::VoucherNotAffectingBankAccount {
            payment_document: "Payment Entry".to_string(),
            payment_entry: "PE-0001".to_string(),
            gl_bank_account: "Bank - TC".to_string(),
        })
    );

    assert_eq!(
        get_clearance_details(
            &transaction,
            &payment_entry,
            Default::default(),
            [("Bank - TC".to_string(), 0.0)].into_iter().collect(),
            "Bank - TC",
            None,
        ),
        Err(BankTransactionError::InvalidBankGlAmount {
            payment_document: "Payment Entry".to_string(),
            payment_entry: "PE-0001".to_string(),
            gl_bank_account: "Bank - TC".to_string(),
            amount: 0.0,
        })
    );

    assert_eq!(
        get_clearance_details(
            &transaction,
            &payment_entry,
            Default::default(),
            [
                ("Bank - TC".to_string(), 100.0),
                ("Charges - TC".to_string(), 10.0),
            ]
            .into_iter()
            .collect(),
            "Bank - TC",
            None,
        )
        .unwrap()
        .should_clear,
        false
    );
}

#[test]
fn bank_transaction_groups_related_bank_gl_entries_like_erpnext() {
    assert_eq!(group_related_bank_gl_entries(&[]), Default::default());

    let grouped = group_related_bank_gl_entries(&[
        RelatedBankGlEntryRow::new("Payment Entry", "PE-0001", "Bank - TC", 700.0),
        RelatedBankGlEntryRow::new("Payment Entry", "PE-0001", "Cash - TC", 50.0),
        RelatedBankGlEntryRow::new("Journal Entry", "JE-0001", "Bank - TC", 30.0),
    ]);

    assert_eq!(
        grouped[&("Payment Entry".to_string(), "PE-0001".to_string())]["Bank - TC"],
        700.0
    );
    assert_eq!(
        grouped[&("Payment Entry".to_string(), "PE-0001".to_string())]["Cash - TC"],
        50.0
    );
    assert_eq!(
        grouped[&("Journal Entry".to_string(), "JE-0001".to_string())]["Bank - TC"],
        30.0
    );
}

#[test]
fn bank_transaction_groups_total_allocated_amount_like_erpnext() {
    assert_eq!(group_total_allocated_amount(&[]), Default::default());

    let grouped = group_total_allocated_amount(&[
        TotalAllocatedAmountRow::new("Payment Entry", "PE-0001", "Bank - TC", 200.0, "2026-05-12"),
        TotalAllocatedAmountRow::new("Payment Entry", "PE-0001", "Cash - TC", 50.0, "2026-05-11"),
        TotalAllocatedAmountRow::new("Journal Entry", "JE-0001", "Bank - TC", 30.0, "2026-05-10"),
    ]);

    assert_eq!(
        grouped[&("Payment Entry".to_string(), "PE-0001".to_string())]["Bank - TC"],
        BankGlAllocation {
            total: 200.0,
            latest_date: Some("2026-05-12".to_string()),
        }
    );
    assert_eq!(
        grouped[&("Payment Entry".to_string(), "PE-0001".to_string())]["Cash - TC"],
        BankGlAllocation {
            total: 50.0,
            latest_date: Some("2026-05-11".to_string()),
        }
    );
    assert_eq!(
        grouped[&("Journal Entry".to_string(), "JE-0001".to_string())]["Bank - TC"],
        BankGlAllocation {
            total: 30.0,
            latest_date: Some("2026-05-10".to_string()),
        }
    );
}

#[test]
fn bank_transaction_remove_from_bank_transaction_plan_matches_erpnext_cancel_skip_and_save() {
    let submitted = BankTransaction {
        name: Some("BT-0001".to_string()),
        docstatus: 1,
        payment_entries: vec![
            BankTransactionPayment::new("Payment Entry", "PE-0001", 100.0),
            BankTransactionPayment::new("Journal Entry", "JE-0001", 50.0),
        ],
        ..Default::default()
    };
    let cancelled = BankTransaction {
        name: Some("BT-0002".to_string()),
        docstatus: 2,
        payment_entries: vec![BankTransactionPayment::new(
            "Payment Entry",
            "PE-0001",
            100.0,
        )],
        ..Default::default()
    };
    let untouched = BankTransaction {
        name: Some("BT-0003".to_string()),
        docstatus: 1,
        payment_entries: vec![BankTransactionPayment::new(
            "Sales Invoice",
            "SI-0001",
            100.0,
        )],
        ..Default::default()
    };

    assert_eq!(
        remove_from_bank_transaction_plan(
            "Payment Entry",
            "PE-0001",
            &[submitted, cancelled, untouched]
        ),
        vec![RemoveFromBankTransactionPlan {
            bank_transaction_name: "BT-0001".to_string(),
            removed_entries: vec![BankTransactionPayment::new(
                "Payment Entry",
                "PE-0001",
                100.0
            )],
            remaining_entries: vec![BankTransactionPayment::new(
                "Journal Entry",
                "JE-0001",
                50.0
            )],
            save: true,
        }]
    );
}

#[test]
fn bank_transaction_validate_currency_matches_erpnext_bank_account_guard() {
    let transaction = BankTransaction {
        currency: Some("USD".to_string()),
        bank_account: Some("Checking - TC".to_string()),
        ..Default::default()
    };

    assert_eq!(transaction.validate_currency(None, None), Ok(()));
    assert_eq!(
        transaction.validate_currency(Some("Bank - TC"), None),
        Ok(())
    );
    assert_eq!(
        transaction.validate_currency(Some("Bank - TC"), Some("USD")),
        Ok(())
    );
    assert_eq!(
        transaction.validate_currency(Some("Bank - TC"), Some("UZS")),
        Err(BankTransactionError::CurrencyMismatch {
            transaction_currency: "USD".to_string(),
            bank_account: "Checking - TC".to_string(),
            account_currency: "UZS".to_string(),
        })
    );

    let missing_currency = BankTransaction {
        bank_account: Some("Checking - TC".to_string()),
        ..Default::default()
    };
    assert_eq!(
        missing_currency.validate_currency(Some("Bank - TC"), Some("UZS")),
        Ok(())
    );

    let missing_bank_account = BankTransaction {
        currency: Some("USD".to_string()),
        ..Default::default()
    };
    assert_eq!(
        missing_bank_account.validate_currency(Some("Bank - TC"), Some("UZS")),
        Ok(())
    );
}

#[test]
fn bank_transaction_allocate_payment_entries_matches_erpnext_zero_allocation_flow() {
    let mut transaction = BankTransaction {
        name: Some("BT-0001".to_string()),
        date: Some("2026-05-20".to_string()),
        unallocated_amount: 150.0,
        withdrawal: 150.0,
        payment_entries: vec![
            BankTransactionPayment::new("Payment Entry", "PE-0001", 0.0),
            BankTransactionPayment::new("Payment Entry", "PE-0002", 25.0),
        ],
        ..Default::default()
    };
    let allocations = BTreeMap::new();
    let gl_entries = BTreeMap::from([(
        ("Payment Entry".to_string(), "PE-0001".to_string()),
        BTreeMap::from([("Bank - TC".to_string(), 100.0)]),
    )]);

    let actions = transaction
        .allocate_payment_entries(
            &allocations,
            &gl_entries,
            "Bank - TC",
            &BTreeMap::new(),
            false,
        )
        .unwrap();

    assert_eq!(
        transaction.payment_entries,
        vec![
            BankTransactionPayment::new("Payment Entry", "PE-0001", 100.0),
            BankTransactionPayment::new("Payment Entry", "PE-0002", 25.0),
        ]
    );
    assert_eq!(transaction.allocated_amount, 125.0);
    assert_eq!(transaction.unallocated_amount, 25.0);
    assert_eq!(
        actions,
        vec![BankTransactionAllocationAction::ClearLinkedPaymentEntry {
            payment_document: "Payment Entry".to_string(),
            payment_entry: "PE-0001".to_string(),
            clearance_date: Some("2026-05-20".to_string()),
        }]
    );
}

#[test]
fn bank_transaction_allocate_payment_entries_removes_cleared_and_excess_rows_like_erpnext() {
    let mut transaction = BankTransaction {
        name: Some("BT-0001".to_string()),
        date: Some("2026-05-20".to_string()),
        unallocated_amount: 10.0,
        withdrawal: 10.0,
        payment_entries: vec![
            BankTransactionPayment::new("Payment Entry", "PE-CLEARED", 0.0),
            BankTransactionPayment::new("Payment Entry", "PE-ALLOC", 0.0),
            BankTransactionPayment::new("Payment Entry", "PE-EXCESS", 0.0),
        ],
        ..Default::default()
    };
    let allocations = BTreeMap::from([(
        ("Payment Entry".to_string(), "PE-CLEARED".to_string()),
        BTreeMap::from([(
            "Bank - TC".to_string(),
            BankGlAllocation {
                total: 100.0,
                latest_date: Some("2026-05-21".to_string()),
            },
        )]),
    )]);
    let gl_entries = BTreeMap::from([
        (
            ("Payment Entry".to_string(), "PE-CLEARED".to_string()),
            BTreeMap::from([("Bank - TC".to_string(), 100.0)]),
        ),
        (
            ("Payment Entry".to_string(), "PE-ALLOC".to_string()),
            BTreeMap::from([("Bank - TC".to_string(), 10.0)]),
        ),
        (
            ("Payment Entry".to_string(), "PE-EXCESS".to_string()),
            BTreeMap::from([("Bank - TC".to_string(), 50.0)]),
        ),
    ]);

    let actions = transaction
        .allocate_payment_entries(
            &allocations,
            &gl_entries,
            "Bank - TC",
            &BTreeMap::new(),
            false,
        )
        .unwrap();

    assert_eq!(
        transaction.payment_entries,
        vec![BankTransactionPayment::new(
            "Payment Entry",
            "PE-ALLOC",
            10.0
        )]
    );
    assert_eq!(
        actions,
        vec![
            BankTransactionAllocationAction::ClearLinkedPaymentEntry {
                payment_document: "Payment Entry".to_string(),
                payment_entry: "PE-CLEARED".to_string(),
                clearance_date: Some("2026-05-21".to_string()),
            },
            BankTransactionAllocationAction::ClearLinkedPaymentEntry {
                payment_document: "Payment Entry".to_string(),
                payment_entry: "PE-ALLOC".to_string(),
                clearance_date: Some("2026-05-20".to_string()),
            },
        ]
    );
}

#[test]
fn bank_transaction_allocate_payment_entries_updates_linked_bank_transaction_like_erpnext() {
    let mut transaction = BankTransaction {
        name: Some("BT-0001".to_string()),
        date: Some("2026-05-20".to_string()),
        unallocated_amount: 80.0,
        payment_entries: vec![BankTransactionPayment::new(
            "Bank Transaction",
            "BT-REFUND",
            0.0,
        )],
        ..Default::default()
    };
    let linked_bank_transactions = BTreeMap::from([(
        "BT-REFUND".to_string(),
        LinkedBankTransaction {
            unallocated_amount: 120.0,
            gl_bank_account: "Bank - TC".to_string(),
        },
    )]);

    let actions = transaction
        .allocate_payment_entries(
            &BTreeMap::new(),
            &BTreeMap::new(),
            "Bank - TC",
            &linked_bank_transactions,
            false,
        )
        .unwrap();

    assert_eq!(
        transaction.payment_entries,
        vec![BankTransactionPayment::new(
            "Bank Transaction",
            "BT-REFUND",
            80.0
        )]
    );
    assert_eq!(
        actions,
        vec![
            BankTransactionAllocationAction::UpdateLinkedBankTransaction {
                bank_transaction_name: "BT-REFUND".to_string(),
                allocated_amount: Some(80.0),
            }
        ]
    );
}
