use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::bank_reconciliation_tool::bank_reconciliation_tool::{
    account_balance, auto_reconcile_plan, create_journal_entry_bts_plan,
    create_payment_entry_bts_plan, get_auto_reconcile_message, get_bank_transactions_plan,
    get_matching_query_plan, reconcile_vouchers_plan, subtract_allocations,
    update_bank_transaction_plan, AllocatedAmount, AutoReconcilePlan,
    BankReconciliationClientConfig, BankReconciliationTool, BankTransactionInput,
    JournalEntryBtsArgs, JournalEntryBtsContext, JournalEntryPlan, MatchingQueryPlan,
    PaymentEntryBtsArgs, PaymentEntryBtsContext, PaymentEntryPlan, PaymentVoucher,
    ReconcileIndicator, ReconcileVouchersPlan, TransactionFilter, TransactionFilterOp,
    VoucherMatch,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_reconciliation_tool_matches_erpnext_metadata_fields_and_client_hooks() {
    assert_eq!(BankReconciliationTool::DOCTYPE, "Bank Reconciliation Tool");
    assert_eq!(BankReconciliationTool::MODULE, "Accounts");
    assert_eq!(
        BankReconciliationTool::FIELD_ORDER,
        [
            "company",
            "bank_account",
            "column_break_1",
            "bank_statement_from_date",
            "bank_statement_to_date",
            "from_reference_date",
            "to_reference_date",
            "filter_by_reference_date",
            "column_break_2",
            "account_currency",
            "account_opening_balance",
            "bank_statement_closing_balance",
            "section_break_1",
            "reconciliation_tool_cards",
            "reconciliation_tool_dt",
            "no_bank_transactions",
        ]
    );
    assert!(BankReconciliationTool::EDITABLE_GRID);
    assert!(BankReconciliationTool::HIDE_TOOLBAR);
    assert!(BankReconciliationTool::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(BankReconciliationTool::IS_SINGLE);
    assert!(BankReconciliationTool::QUICK_ENTRY);
    assert_eq!(BankReconciliationTool::SORT_FIELD, "creation");
    assert_eq!(BankReconciliationTool::SORT_ORDER, "DESC");

    let fields = BankReconciliationTool::fields();
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .ignore_user_permissions()
    ));
    assert!(
        fields.contains(&FieldSpec::link("bank_account", "Bank Account").options("Bank Account"))
    );
    assert!(fields.contains(
        &FieldSpec::date("bank_statement_from_date", "From Date")
            .depends_on("eval: doc.bank_account && !doc.filter_by_reference_date")
    ));
    assert!(fields.contains(
        &FieldSpec::currency("account_opening_balance", "Account Opening Balance")
            .options("account_currency")
            .depends_on("eval: doc.bank_statement_from_date")
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::check("filter_by_reference_date", "Filter by Reference Date").default("0")
    ));
    assert!(fields.contains(
        &FieldSpec::link("account_currency", "Account Currency")
            .options("Currency")
            .hidden()
    ));

    let client = BankReconciliationClientConfig::from_erpnext_js();
    assert_eq!(client.bank_account_query_company_filter, "company");
    assert!(client.bank_account_query_is_company_account);
    assert_eq!(
        client.default_company_source,
        "frappe.defaults.get_default(\"company\")"
    );
    assert_eq!(client.from_date_default, "today - 1 month");
    assert_eq!(client.to_date_default, "today");
    assert_eq!(client.upload_button, "Upload Bank Statement");
    assert_eq!(client.auto_reconcile_button, "Auto Reconcile");
    assert_eq!(client.get_unreconciled_button, "Get Unreconciled Entries");
}

#[test]
fn bank_reconciliation_transaction_filters_and_balance_formula_match_erpnext() {
    let plan = get_bank_transactions_plan("BA-0001", Some("2026-05-01"), Some("2026-05-31"));
    assert_eq!(
        plan.filters,
        vec![
            TransactionFilter::new("bank_account", TransactionFilterOp::Eq, "BA-0001"),
            TransactionFilter::new("docstatus", TransactionFilterOp::Eq, "1"),
            TransactionFilter::new("unallocated_amount", TransactionFilterOp::Gt, "0"),
            TransactionFilter::new("date", TransactionFilterOp::Le, "2026-05-31"),
            TransactionFilter::new("date", TransactionFilterOp::Ge, "2026-05-01"),
        ]
    );
    assert_eq!(plan.order_by, "date");
    assert_eq!(
        plan.fields,
        [
            "date",
            "deposit",
            "withdrawal",
            "currency",
            "description",
            "name",
            "bank_account",
            "company",
            "unallocated_amount",
            "reference_number",
            "party_type",
            "party",
        ]
    );

    assert_eq!(
        account_balance(1_000.0, &[(100.0, 25.0), (50.0, 10.0)], 15.0),
        900.0
    );
}

#[test]
fn bank_reconciliation_update_and_auto_reconcile_plans_match_erpnext() {
    let update =
        update_bank_transaction_plan("BT-0001", "REF-9", Some("Customer"), Some("_Test Customer"));
    assert_eq!(update.doctype, "Bank Transaction");
    assert_eq!(update.name, "BT-0001");
    assert_eq!(
        update.updates,
        BTreeMap::from([
            ("reference_number", Some("REF-9".to_string())),
            ("party_type", Some("Customer".to_string())),
            ("party", Some("_Test Customer".to_string())),
        ])
    );

    assert_eq!(
        auto_reconcile_plan(
            (1..=3).map(|idx| format!("BT-{idx:04}")).collect(),
            Some("2026-05-01"),
            Some("2026-05-31"),
            false,
            None,
            None,
        ),
        AutoReconcilePlan::StartNow {
            bank_transactions: vec![
                "BT-0001".to_string(),
                "BT-0002".to_string(),
                "BT-0003".to_string()
            ],
            from_date: Some("2026-05-01".to_string()),
            to_date: Some("2026-05-31".to_string()),
            filter_by_reference_date: false,
            from_reference_date: None,
            to_reference_date: None,
        }
    );

    let background = auto_reconcile_plan(
        (1..=11).map(|idx| format!("BT-{idx:04}")).collect(),
        None,
        None,
        true,
        Some("2026-05-01"),
        Some("2026-05-31"),
    );
    assert_eq!(
        background,
        AutoReconcilePlan::Enqueue {
            queue: "long",
            batches: vec![(1..=11).map(|idx| format!("BT-{idx:04}")).collect()],
            message: "Auto Reconciliation has started in the background",
        }
    );
}

#[test]
fn bank_reconciliation_create_journal_and_payment_entry_plans_match_erpnext() {
    let transaction = BankTransactionInput {
        name: "BT-0001".to_string(),
        bank_account: "BA-0001".to_string(),
        currency: Some("USD".to_string()),
        deposit: 100.0,
        withdrawal: 0.0,
        unallocated_amount: 100.0,
        reference_number: Some("REF-1".to_string()),
        party_type: Some("Customer".to_string()),
        party: Some("_Test Customer".to_string()),
    };

    let journal_plan = create_journal_entry_bts_plan(
        &transaction,
        JournalEntryBtsArgs {
            reference_number: Some("REF-1"),
            reference_date: Some("2026-05-10"),
            posting_date: Some("2026-05-11"),
            entry_type: Some("Bank Entry"),
            second_account: "Debtors - TC",
            mode_of_payment: Some("Wire Transfer"),
            party_type: Some("Customer"),
            party: Some("_Test Customer"),
            allow_edit: false,
        },
        JournalEntryBtsContext {
            company_account: "Bank - TC",
            second_account_type: "Receivable",
            company: "_Test Company",
            company_default_currency: "USD",
            company_account_currency: "USD",
            second_account_currency: "USD",
            default_cost_center: "Main - TC",
            transaction_to_company_exchange_rate: 1.0,
            second_account_exchange_rate: 1.0,
            company_account_exchange_rate: 1.0,
        },
    )
    .expect("party provided for receivable account");
    assert_eq!(
        journal_plan,
        JournalEntryPlan {
            voucher_type: Some("Bank Entry".to_string()),
            company: "_Test Company".to_string(),
            posting_date: Some("2026-05-11".to_string()),
            cheque_date: Some("2026-05-10".to_string()),
            cheque_no: Some("REF-1".to_string()),
            mode_of_payment: Some("Wire Transfer".to_string()),
            multi_currency: false,
            allow_edit: false,
            accounts_len: 2,
            reconcile_voucher: Some(PaymentVoucher::new("Journal Entry", "<new>", 100.0)),
        }
    );

    let payment_plan = create_payment_entry_bts_plan(
        &transaction,
        PaymentEntryBtsArgs {
            reference_number: Some("REF-1"),
            reference_date: Some("2026-05-10"),
            party_type: "Customer",
            party: "_Test Customer",
            posting_date: Some("2026-05-11"),
            mode_of_payment: Some("Wire Transfer"),
            project: Some("PROJ-1"),
            cost_center: Some("Main - TC"),
            allow_edit: false,
            company_bank_account: Some("Company BA"),
        },
        PaymentEntryBtsContext {
            bank_account: "Bank - TC",
            company: "_Test Company",
            party_account: "Debtors - TC",
            bank_currency: "USD",
            party_currency: "USD",
            exchange_rate: 1.0,
        },
    );
    assert_eq!(
        payment_plan,
        PaymentEntryPlan {
            payment_type: "Receive",
            company: "_Test Company".to_string(),
            paid_from: "Debtors - TC".to_string(),
            paid_to: "Bank - TC".to_string(),
            paid_from_account_currency: "USD".to_string(),
            paid_to_account_currency: "USD".to_string(),
            paid_amount: 100.0,
            received_amount: 100.0,
            bank_account: Some("Company BA".to_string()),
            allow_edit: false,
            reconcile_voucher: Some(PaymentVoucher::new("Payment Entry", "<new>", 100.0)),
        }
    );
}

#[test]
fn bank_reconciliation_messages_allocations_and_matching_query_plan_match_erpnext() {
    assert_eq!(
        get_auto_reconcile_message(&[], &[]),
        (
            "No matches occurred via auto reconciliation".to_string(),
            ReconcileIndicator::Blue
        )
    );
    assert_eq!(
        get_auto_reconcile_message(&["BT-2".to_string()], &["BT-1".to_string()]),
        (
            "1 Transaction(s) Reconciled<br>1 Transaction Partially Reconciled".to_string(),
            ReconcileIndicator::Green,
        )
    );

    let vouchers = vec![
        PaymentVoucher::new("Payment Entry", "PE-0001", 100.0),
        PaymentVoucher::new("Journal Entry", "JE-0001", 200.0),
    ];
    let allocated = vec![AllocatedAmount::new(
        "Payment Entry",
        "PE-0001",
        "Bank - TC",
        40.0,
    )];
    assert_eq!(
        subtract_allocations("Bank - TC", &vouchers, &allocated),
        vec![
            PaymentVoucher::new("Payment Entry", "PE-0001", 60.0),
            PaymentVoucher::new("Journal Entry", "JE-0001", 200.0),
        ]
    );

    let transaction = BankTransactionInput {
        name: "BT-0001".to_string(),
        bank_account: "BA-0001".to_string(),
        currency: Some("USD".to_string()),
        deposit: 100.0,
        withdrawal: 0.0,
        unallocated_amount: 100.0,
        reference_number: Some("REF-1".to_string()),
        party_type: Some("Customer".to_string()),
        party: Some("_Test Customer".to_string()),
    };
    assert_eq!(
        get_matching_query_plan(
            "Bank - TC",
            "_Test Company",
            &transaction,
            &[
                "payment_entry",
                "journal_entry",
                "sales_invoice",
                "purchase_invoice",
                "bank_transaction",
                "exact_match",
            ],
            Some("2026-05-01"),
            Some("2026-05-31"),
            false,
            None,
            None,
        ),
        MatchingQueryPlan {
            exact_match: true,
            account_from_to: "paid_to",
            payment_type: "Receive",
            common_amount: 100.0,
            bank_account: "Bank - TC".to_string(),
            company: "_Test Company".to_string(),
            sources: vec![
                VoucherMatch::PaymentEntry,
                VoucherMatch::JournalEntry,
                VoucherMatch::SalesInvoice,
                VoucherMatch::BankTransaction,
            ],
            date_field: "posting_date",
            from_date: Some("2026-05-01".to_string()),
            to_date: Some("2026-05-31".to_string()),
            from_reference_date: None,
            to_reference_date: None,
        }
    );
}

#[test]
fn bank_reconciliation_reconcile_vouchers_plan_matches_transaction_methods() {
    assert_eq!(
        reconcile_vouchers_plan(
            "BT-0001",
            vec![
                PaymentVoucher::new("Payment Entry", "PE-0001", 100.0),
                PaymentVoucher::new("Journal Entry", "JE-0001", 50.0),
            ],
        ),
        ReconcileVouchersPlan {
            bank_transaction_name: "BT-0001".to_string(),
            vouchers: vec![
                PaymentVoucher::new("Payment Entry", "PE-0001", 100.0),
                PaymentVoucher::new("Journal Entry", "JE-0001", 50.0),
            ],
            transaction_methods: vec![
                "add_payment_entries",
                "validate_duplicate_references",
                "allocate_payment_entries",
                "update_allocated_amount",
                "set_status",
                "save",
            ],
        }
    );
}

#[test]
fn bank_reconciliation_tool_controller_is_pass_through() {
    let doc = BankReconciliationTool::default();
    assert_eq!(doc.doctype(), "Bank Reconciliation Tool");
    assert_eq!(doc.module(), "Accounts");
    assert!(doc.custom_hooks().is_empty());
}
