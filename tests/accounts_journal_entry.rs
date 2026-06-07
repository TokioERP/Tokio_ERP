use std::collections::{BTreeMap, HashMap};

use tokio_erp::erpnext::accounts::doctype::journal_entry::journal_entry::{
    check_customer_credit_limits, get_account_details_and_party_type, get_outstanding,
    make_inter_company_journal_entry, make_payment_entry_against_invoice,
    make_payment_entry_against_order, make_reverse_journal_entry,
    validate_stock_account_transaction, AccountDetails, AccountMeta, BankCashAccount, InvoiceRef,
    JournalEntry, JournalEntryAccountRow, JournalEntryError, OrderRef, OutstandingArgs,
    PartyTypeMeta, ReferenceDoc, ReverseEntryError,
};
use tokio_erp::erpnext::DocumentController;

fn row(idx: usize, account: &str) -> JournalEntryAccountRow {
    JournalEntryAccountRow {
        idx,
        account: account.to_string(),
        account_currency: Some("USD".to_string()),
        exchange_rate: 1.0,
        ..Default::default()
    }
}

#[test]
fn journal_entry_metadata_matches_erpnext_json() {
    assert_eq!(JournalEntry::DOCTYPE, "Journal Entry");
    assert_eq!(JournalEntry::MODULE, "Accounts");
    assert_eq!(JournalEntry::AUTONAME, "naming_series:");
    assert_eq!(JournalEntry::TITLE_FIELD, "title");
    assert_eq!(JournalEntry::SORT_FIELD, "creation");
    assert_eq!(JournalEntry::SORT_ORDER, "DESC");
    assert!(JournalEntry::IS_SUBMITTABLE);
    assert!(JournalEntry::TRACK_CHANGES);
    assert_eq!(JournalEntry::FIELD_ORDER.len(), 74);
    assert_eq!(
        &JournalEntry::FIELD_ORDER[..10],
        [
            "entry_type_and_date",
            "title",
            "voucher_type",
            "naming_series",
            "column_break1",
            "posting_date",
            "company",
            "finance_book",
            "2_add_edit_gl_entries",
            "accounts",
        ]
    );

    let doc = JournalEntry::default();
    assert_eq!(doc.doctype(), "Journal Entry");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        [
            "validate",
            "before_submit",
            "on_submit",
            "on_update_after_submit",
            "before_cancel",
            "on_cancel",
        ]
    );
}

#[test]
fn journal_entry_validate_sets_amounts_totals_against_and_remarks_like_erpnext() {
    let mut je = JournalEntry {
        name: Some("ACC-JV-0001".to_string()),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        voucher_type: "Opening Entry".to_string(),
        posting_date: "2026-06-06".to_string(),
        cheque_no: Some("CHK-1".to_string()),
        cheque_date: Some("2026-06-05".to_string()),
        accounts: vec![
            JournalEntryAccountRow {
                debit_in_account_currency: 125.0,
                party_type: Some("Customer".to_string()),
                party: Some("_Test Customer".to_string()),
                reference_type: Some("Sales Invoice".to_string()),
                reference_name: Some("SINV-0001".to_string()),
                ..row(1, "Debtors - TC")
            },
            JournalEntryAccountRow {
                credit_in_account_currency: 125.0,
                user_remark: Some("Bank line".to_string()),
                ..row(2, "Cash - TC")
            },
        ],
        ..Default::default()
    };
    let accounts = HashMap::from([
        (
            "Debtors - TC".to_string(),
            AccountMeta {
                account_type: Some("Receivable".to_string()),
                account_currency: Some("USD".to_string()),
                root_type: Some("Asset".to_string()),
                company: Some("_Test Company".to_string()),
            },
        ),
        (
            "Cash - TC".to_string(),
            AccountMeta {
                account_type: Some("Cash".to_string()),
                account_currency: Some("USD".to_string()),
                root_type: Some("Asset".to_string()),
                company: Some("_Test Company".to_string()),
            },
        ),
    ]);
    let party_types = HashMap::from([(
        "Customer".to_string(),
        PartyTypeMeta {
            account_type: "Receivable".to_string(),
        },
    )]);
    let refs = HashMap::from([(
        ("Sales Invoice".to_string(), "SINV-0001".to_string()),
        ReferenceDoc {
            party: "_Test Customer".to_string(),
            account: "Debtors - TC".to_string(),
            docstatus: 1,
            outstanding_amount: 200.0,
            grand_total: 200.0,
            advance_paid: 0.0,
            per_billed: 0.0,
            status: None,
            company_currency: "USD".to_string(),
            conversion_rate: 1.0,
            due_date: Some("2026-06-30".to_string()),
        },
    )]);

    je.validate(&accounts, &party_types, &refs).unwrap();

    assert_eq!(je.is_opening, "Yes");
    assert_eq!(je.clearance_date, None);
    assert_eq!(je.total_debit, 125.0);
    assert_eq!(je.total_credit, 125.0);
    assert_eq!(je.difference, 0.0);
    assert_eq!(je.accounts[0].debit, 125.0);
    assert_eq!(je.accounts[1].credit, 125.0);
    assert_eq!(je.accounts[0].against_account.as_deref(), Some("Cash - TC"));
    assert_eq!(
        je.accounts[1].against_account.as_deref(),
        Some("_Test Customer")
    );
    assert!(je
        .remark
        .as_deref()
        .unwrap()
        .contains("Reference #CHK-1 dated 2026-06-05"));
    assert_eq!(je.title.as_deref(), Some("_Test Customer"));
}

#[test]
fn journal_entry_validation_errors_match_erpnext_core_guards() {
    let receivable_accounts = HashMap::from([(
        "Debtors - TC".to_string(),
        AccountMeta {
            account_type: Some("Receivable".to_string()),
            account_currency: Some("USD".to_string()),
            root_type: Some("Asset".to_string()),
            company: Some("_Test Company".to_string()),
        },
    )]);
    let supplier_party = HashMap::from([(
        "Supplier".to_string(),
        PartyTypeMeta {
            account_type: "Payable".to_string(),
        },
    )]);

    let mut missing_party = JournalEntry {
        company_currency: "USD".to_string(),
        accounts: vec![JournalEntryAccountRow {
            credit_in_account_currency: 10.0,
            ..row(1, "Debtors - TC")
        }],
        ..Default::default()
    };
    assert_eq!(
        missing_party.validate(&receivable_accounts, &HashMap::new(), &HashMap::new()),
        Err(JournalEntryError::PartyRequired {
            row: 1,
            account: "Debtors - TC".to_string(),
        })
    );

    let mut wrong_party_type = JournalEntry {
        company_currency: "USD".to_string(),
        accounts: vec![JournalEntryAccountRow {
            credit_in_account_currency: 10.0,
            party_type: Some("Supplier".to_string()),
            party: Some("_Test Supplier".to_string()),
            ..row(1, "Debtors - TC")
        }],
        ..Default::default()
    };
    assert_eq!(
        wrong_party_type.validate(&receivable_accounts, &supplier_party, &HashMap::new()),
        Err(JournalEntryError::PartyAccountTypeMismatch {
            row: 1,
            account: "Debtors - TC".to_string(),
            party_type: "Supplier".to_string(),
        })
    );

    let mut both_sides = JournalEntry {
        accounts: vec![JournalEntryAccountRow {
            debit: 10.0,
            credit: 2.0,
            debit_in_account_currency: 10.0,
            credit_in_account_currency: 2.0,
            ..row(1, "Cash - TC")
        }],
        ..Default::default()
    };
    assert_eq!(
        both_sides.set_total_debit_credit(),
        Err(JournalEntryError::DebitAndCreditSameRow)
    );

    let mut foreign = JournalEntry {
        company_currency: "USD".to_string(),
        multi_currency: false,
        accounts: vec![JournalEntryAccountRow {
            account_currency: Some("EUR".to_string()),
            credit_in_account_currency: 10.0,
            exchange_rate: 1.2,
            ..row(1, "Foreign Bank")
        }],
        ..Default::default()
    };
    assert_eq!(
        foreign.validate_multi_currency(&HashMap::new()),
        Err(JournalEntryError::MultiCurrencyRequired)
    );
}

#[test]
fn journal_entry_get_balance_and_gl_map_match_erpnext_amount_branches() {
    let mut je = JournalEntry {
        name: Some("ACC-JV-0002".to_string()),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        due_date: Some("2026-06-30".to_string()),
        remark: Some("Header remark".to_string()),
        accounts: vec![
            JournalEntryAccountRow {
                debit: 200.0,
                debit_in_account_currency: 200.0,
                account_currency: Some("USD".to_string()),
                exchange_rate: 1.0,
                ..row(1, "Debtors - TC")
            },
            JournalEntryAccountRow {
                account_currency: Some("EUR".to_string()),
                exchange_rate: 1.25,
                ..row(2, "Bank EUR - TC")
            },
        ],
        ..Default::default()
    };

    je.set_total_debit_credit().unwrap();
    assert_eq!(je.difference, 200.0);
    je.get_balance(Some("Exchange Gain/Loss - TC"), Some("Main - TC"))
        .unwrap();
    assert_eq!(je.difference, 0.0);
    assert_eq!(je.accounts[1].credit_in_account_currency, 200.0);
    assert_eq!(je.accounts[1].credit, 200.0);

    je.multi_currency = true;
    je.accounts[1].credit = 250.0;
    je.accounts[1].credit_in_account_currency = 200.0;
    let gl_map = je.build_gl_map();
    assert_eq!(gl_map.len(), 2);
    assert_eq!(gl_map[0].transaction_currency, "EUR");
    assert_eq!(gl_map[0].transaction_exchange_rate, 1.0);
    assert_eq!(gl_map[0].debit_in_transaction_currency, 200.0);
    assert_eq!(gl_map[1].credit_in_transaction_currency, 200.0);
    assert_eq!(gl_map[0].remarks.as_deref(), Some("Header remark"));
}

#[test]
fn journal_entry_reference_outstanding_and_helper_plans_match_erpnext() {
    let order = OrderRef {
        doctype: "Sales Order".to_string(),
        name: "SO-0001".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        party: "_Test Customer".to_string(),
        grand_total: 500.0,
        base_grand_total: 500.0,
        advance_paid: 125.0,
        per_billed: 0.0,
        transaction_date: "2026-06-01".to_string(),
    };
    let party_accounts = HashMap::from([(
        ("Customer".to_string(), "_Test Customer".to_string()),
        ("Debtors - TC".to_string(), "USD".to_string()),
    )]);
    let bank = BankCashAccount {
        account: "Bank - TC".to_string(),
        account_currency: "USD".to_string(),
        account_type: "Bank".to_string(),
        balance: Some(1000.0),
    };

    let advance =
        make_payment_entry_against_order(&order, None, &party_accounts, Some(bank.clone()))
            .unwrap();
    assert_eq!(advance.voucher_type, "Bank Entry");
    assert_eq!(advance.accounts.len(), 2);
    assert_eq!(advance.accounts[0].party_type.as_deref(), Some("Customer"));
    assert_eq!(advance.accounts[0].credit_in_account_currency, 375.0);
    assert_eq!(advance.accounts[1].debit_in_account_currency, 375.0);
    assert_eq!(advance.accounts[0].is_advance, "Yes");

    let invoice = InvoiceRef {
        doctype: "Purchase Invoice".to_string(),
        name: "PINV-0001".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        party: "_Test Supplier".to_string(),
        party_account: "Creditors - TC".to_string(),
        party_account_currency: "USD".to_string(),
        outstanding_amount: 90.0,
        conversion_rate: 1.0,
        remarks: "PI remarks".to_string(),
        posting_date: "2026-06-01".to_string(),
    };
    let payment = make_payment_entry_against_invoice(&invoice, Some(50.0), Some(bank)).unwrap();
    assert_eq!(payment.accounts[0].debit_in_account_currency, 50.0);
    assert_eq!(payment.accounts[1].credit_in_account_currency, 50.0);
    assert_eq!(payment.accounts[0].is_advance, "No");

    let mut jv_amounts = BTreeMap::new();
    jv_amounts.insert("ACC-JV-0001".to_string(), 75.0);
    assert_eq!(
        get_outstanding(
            &OutstandingArgs {
                doctype: "Journal Entry".to_string(),
                docname: "ACC-JV-0001".to_string(),
                account: "Debtors - TC".to_string(),
                party: Some("_Test Customer".to_string()),
                account_currency: "USD".to_string(),
                company: "_Test Company".to_string(),
            },
            &jv_amounts,
            &HashMap::new()
        ),
        Some(HashMap::from([(
            "credit_in_account_currency".to_string(),
            "75".to_string()
        )]))
    );

    let detail = get_account_details_and_party_type(
        "Debtors - TC",
        "2026-06-06",
        "_Test Company",
        &HashMap::from([(
            "Debtors - TC".to_string(),
            AccountDetails {
                account_type: "Receivable".to_string(),
                account_currency: Some("USD".to_string()),
                bank_account: None,
            },
        )]),
    )
    .unwrap();
    assert_eq!(detail.party_type.as_deref(), Some("Customer"));
    assert_eq!(detail.exchange_rate, 1.0);
}

#[test]
fn journal_entry_reverse_and_inter_company_mappers_match_erpnext() {
    let source = JournalEntry {
        name: Some("ACC-JV-0003".to_string()),
        company: "_Test Company".to_string(),
        posting_date: "2026-06-06".to_string(),
        accounts: vec![JournalEntryAccountRow {
            debit_in_account_currency: 30.0,
            debit: 30.0,
            ..row(1, "Cash - TC")
        }],
        ..Default::default()
    };
    assert_eq!(
        make_inter_company_journal_entry("ACC-JV-0003", "Inter Company Journal Entry", "Other Co")
            .inter_company_journal_entry_reference
            .as_deref(),
        Some("ACC-JV-0003")
    );

    let reversed = make_reverse_journal_entry(&source, false).unwrap();
    assert_eq!(reversed.reversal_of.as_deref(), Some("ACC-JV-0003"));
    assert_eq!(reversed.accounts[0].credit_in_account_currency, 30.0);
    assert_eq!(reversed.accounts[0].credit, 30.0);
    assert_eq!(
        make_reverse_journal_entry(&source, true),
        Err(ReverseEntryError::ReverseAlreadyExists)
    );
}

#[test]
fn journal_entry_python_stock_account_guard_regression_matches_erpnext() {
    assert_eq!(
        validate_stock_account_transaction(
            true,
            "Journal Entry",
            "Stock In Hand - TCP1",
            100.0,
            100.0,
        ),
        Err(JournalEntryError::StockAccountInvalidTransaction {
            account: "Stock In Hand - TCP1".to_string(),
        })
    );
    assert_eq!(
        validate_stock_account_transaction(
            true,
            "Periodic Accounting Entry",
            "Stock In Hand - TCP1",
            100.0,
            100.0,
        ),
        Ok(())
    );
    assert_eq!(
        validate_stock_account_transaction(
            false,
            "Journal Entry",
            "Stock In Hand - TCP1",
            100.0,
            100.0,
        ),
        Ok(())
    );
    assert_eq!(
        validate_stock_account_transaction(
            true,
            "Journal Entry",
            "Stock In Hand - TCP1",
            90.0,
            100.0,
        ),
        Ok(())
    );
}

#[test]
fn journal_entry_python_customer_credit_limit_regression_matches_erpnext() {
    let je = JournalEntry {
        company: "_Test Company".to_string(),
        accounts: vec![
            JournalEntryAccountRow {
                debit: 100.0,
                debit_in_account_currency: 100.0,
                party_type: Some("Customer".to_string()),
                party: Some("_Test New Customer".to_string()),
                ..row(1, "Debtors - TC")
            },
            JournalEntryAccountRow {
                credit: 100.0,
                credit_in_account_currency: 100.0,
                ..row(2, "_Test Cash - TC")
            },
        ],
        ..Default::default()
    };

    assert_eq!(
        check_customer_credit_limits(
            &je,
            &HashMap::from([("_Test New Customer".to_string(), 50.0)]),
            &HashMap::from([("_Test New Customer".to_string(), 100.0)]),
            &HashMap::new(),
        ),
        Err(JournalEntryError::CreditLimitCrossed {
            customer: "_Test New Customer".to_string(),
            outstanding: "100".to_string(),
            credit_limit: "50".to_string(),
        })
    );

    assert_eq!(
        check_customer_credit_limits(
            &je,
            &HashMap::from([("_Test New Customer".to_string(), 50.0)]),
            &HashMap::from([("_Test New Customer".to_string(), 100.0)]),
            &HashMap::from([("_Test New Customer".to_string(), true)]),
        ),
        Ok(())
    );
}
