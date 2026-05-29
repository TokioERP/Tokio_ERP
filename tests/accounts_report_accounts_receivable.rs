use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::{
    accounts_receivable_args, allocate_future_payments, build_voucher_dict, get_columns,
    get_currency_fields, group_future_payments, init_voucher_balance, set_ageing,
    set_invoice_details, set_party_details, update_voucher_balance, AccountType, FuturePayment,
    FuturePaymentAllocationRow, InvoiceDetails, InvoiceDetailsRow, PartyDetails, PartyDetailsRow,
    PaymentLedgerEntry, ReceivablePayableAgeingRow, ReceivablePayableFilters,
    ReceivablePayableRuntime, ReceivablePayableSettings, ReceivablePayableState, ReportColumn,
    VoucherBalanceKey, VoucherBalanceRow,
};

fn filters() -> ReceivablePayableFilters {
    ReceivablePayableFilters {
        company: None,
        report_date: None,
        calculate_ageing_with: None,
        ageing_based_on: None,
        range: None,
        account_type: None,
        group_by_party: false,
        in_party_currency: false,
        party: Vec::new(),
        party_account: None,
        based_on_payment_terms: false,
        show_future_payments: false,
        show_delivery_notes: false,
        show_sales_person: false,
        show_remarks: false,
        sales_partner: None,
        ignore_accounts: false,
        handle_employee_advances: false,
    }
}

fn settings() -> ReceivablePayableSettings {
    ReceivablePayableSettings {
        default_company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        currency_precision: 3,
        party_naming_by: "Naming Series".to_string(),
        ple_fetch_method: None,
    }
}

fn ple(
    account: &str,
    voucher_type: &str,
    voucher_no: &str,
    against_voucher_type: &str,
    against_voucher_no: &str,
    amount: f64,
) -> PaymentLedgerEntry {
    PaymentLedgerEntry {
        account: account.to_string(),
        voucher_type: voucher_type.to_string(),
        voucher_no: voucher_no.to_string(),
        against_voucher_type: against_voucher_type.to_string(),
        against_voucher_no: against_voucher_no.to_string(),
        party_type: "Customer".to_string(),
        party: "CUST-001".to_string(),
        posting_date: "2026-05-10".to_string(),
        due_date: Some("2026-06-09".to_string()),
        account_currency: "USD".to_string(),
        remarks: Some("Remark".to_string()),
        cost_center: Some("Main - TC".to_string()),
        project: Some("PROJ-001".to_string()),
        amount,
        amount_in_account_currency: amount * 2.0,
    }
}

#[test]
fn accounts_receivable_execute_args_match_erpnext_receivable_defaults() {
    let args = accounts_receivable_args();

    assert_eq!(args.account_type, AccountType::Receivable);
    assert_eq!(
        args.naming_by,
        [
            "Selling Settings".to_string(),
            "cust_master_name".to_string()
        ]
    );
}

#[test]
fn accounts_receivable_state_initializes_dates_ranges_and_fetch_method_like_erpnext() {
    let state = ReceivablePayableState::new(&filters(), &settings(), "2026-05-29");

    assert_eq!(state.filters.report_date, Some("2026-05-29".to_string()));
    assert_eq!(state.age_as_on, "2026-05-29");
    assert_eq!(state.ranges, vec!["30", "60", "90", "120"]);
    assert_eq!(state.range_numbers, vec![1, 2, 3, 4, 5]);
    assert_eq!(state.ple_fetch_method, "Buffered Cursor");
}

#[test]
fn accounts_receivable_state_uses_report_date_for_ageing_when_filter_requests_it() {
    let state = ReceivablePayableState::new(
        &ReceivablePayableFilters {
            report_date: Some("2026-04-30".to_string()),
            calculate_ageing_with: Some("Posting Date".to_string()),
            range: Some("15, bad, 45".to_string()),
            ..filters()
        },
        &ReceivablePayableSettings {
            ple_fetch_method: Some("UnBuffered Cursor".to_string()),
            ..settings()
        },
        "2026-05-29",
    );

    assert_eq!(state.filters.report_date, Some("2026-04-30".to_string()));
    assert_eq!(state.age_as_on, "2026-04-30");
    assert_eq!(state.ranges, vec!["15", "45"]);
    assert_eq!(state.range_numbers, vec![1, 2, 3]);
    assert_eq!(state.ple_fetch_method, "UnBuffered Cursor");
}

#[test]
fn accounts_receivable_set_defaults_matches_receivable_party_and_skip_total_rules() {
    let state = ReceivablePayableState::new(
        &ReceivablePayableFilters {
            group_by_party: true,
            ..filters()
        },
        &settings(),
        "2026-05-29",
    )
    .with_defaults();

    assert_eq!(state.filters.company, Some("_Test Company".to_string()));
    assert_eq!(state.company_currency, "USD");
    assert_eq!(state.currency_precision, 3);
    assert_eq!(state.dr_or_cr, "debit");
    assert_eq!(state.account_type, AccountType::Receivable);
    assert_eq!(state.party_type, vec!["Customer"]);
    assert_eq!(state.skip_total_row, 1);
}

#[test]
fn accounts_receivable_in_party_currency_single_party_keeps_total_row_like_erpnext() {
    let state = ReceivablePayableState::new(
        &ReceivablePayableFilters {
            in_party_currency: true,
            party: vec!["CUST-001".to_string()],
            ..filters()
        },
        &settings(),
        "2026-05-29",
    )
    .with_defaults();

    assert_eq!(state.skip_total_row, 0);
}

#[test]
fn accounts_receivable_currency_fields_match_erpnext_subtotal_fields() {
    assert_eq!(
        get_currency_fields(),
        vec![
            "invoiced",
            "paid",
            "credit_note",
            "outstanding",
            "range1",
            "range2",
            "range3",
            "range4",
            "range5",
            "future_amount",
            "remaining_balance",
        ]
    );
}

#[test]
fn accounts_receivable_columns_match_initial_receivable_shape_with_optional_branches() {
    let runtime = ReceivablePayableRuntime {
        account_type: AccountType::Receivable,
        party_naming_by: "Naming Series".to_string(),
        ranges: vec!["30".to_string(), "60".to_string()],
        range_numbers: vec![1, 2, 3],
    };
    let report_filters = ReceivablePayableFilters {
        based_on_payment_terms: true,
        show_future_payments: true,
        show_delivery_notes: true,
        show_sales_person: true,
        show_remarks: true,
        sales_partner: Some("Partner A".to_string()),
        ..filters()
    };

    assert_eq!(
        get_columns(&report_filters, &runtime),
        vec![
            ReportColumn::date("Posting Date", "posting_date"),
            ReportColumn::data("Party Type", "party_type", 100),
            ReportColumn::dynamic_link("Party", "party", "party_type", 180),
            ReportColumn::link("Receivable Account", "party_account", "Account", 180),
            ReportColumn::data("Customer Name", "customer_name", 120),
            ReportColumn::link(
                "Customer Contact",
                "customer_primary_contact",
                "Contact",
                120
            ),
            ReportColumn::data("Cost Center", "cost_center", 120),
            ReportColumn::link("Project", "project", "Project", 120),
            ReportColumn::data("Voucher Type", "voucher_type", 120),
            ReportColumn::dynamic_link("Voucher No", "voucher_no", "voucher_type", 180),
            ReportColumn::date("Due Date", "due_date"),
            ReportColumn::data("Payment Term", "payment_term", 120),
            ReportColumn::currency("Invoice Grand Total", "invoice_grand_total", 120),
            ReportColumn::currency("Invoiced Amount", "invoiced", 120),
            ReportColumn::currency("Paid Amount", "paid", 120),
            ReportColumn::currency("Credit Note", "credit_note", 120),
            ReportColumn::currency("Outstanding Amount", "outstanding", 120),
            ReportColumn::int("Age (Days)", "age", 80),
            ReportColumn::currency("<0", "range0", 120),
            ReportColumn::currency("0-30", "range1", 120),
            ReportColumn::currency("31-60", "range2", 120),
            ReportColumn::currency("61-Above", "range3", 120),
            ReportColumn::link("Currency", "currency", "Currency", 80),
            ReportColumn::data("Future Payment Ref", "future_ref", 120),
            ReportColumn::currency("Future Payment Amount", "future_amount", 120),
            ReportColumn::currency("Remaining Balance", "remaining_balance", 120),
            ReportColumn::data("Customer LPO", "po_no", 120),
            ReportColumn::data("Delivery Notes", "delivery_notes", 120),
            ReportColumn::link("Territory", "territory", "Territory", 120),
            ReportColumn::link("Customer Group", "customer_group", "Customer Group", 120),
            ReportColumn::data("Sales Person", "sales_person", 120),
            ReportColumn::data("Sales Partner", "default_sales_partner", 120),
            ReportColumn::text("Remarks", "remarks", 200),
        ]
    );
}

#[test]
fn accounts_receivable_build_voucher_dict_matches_erpnext_zero_balance_shape() {
    let row = build_voucher_dict(&ple(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-0001",
        "Sales Invoice",
        "SINV-0001",
        100.0,
    ));

    assert_eq!(row.voucher_type, "Sales Invoice");
    assert_eq!(row.voucher_no, "SINV-0001");
    assert_eq!(row.party, "CUST-001");
    assert_eq!(row.party_account, "Debtors - TC");
    assert_eq!(row.posting_date, "2026-05-10");
    assert_eq!(row.account_currency, "USD");
    assert_eq!(row.remarks, Some("Remark".to_string()));
    assert_eq!(row.invoiced, 0.0);
    assert_eq!(row.paid, 0.0);
    assert_eq!(row.credit_note, 0.0);
    assert_eq!(row.outstanding, 0.0);
    assert_eq!(row.invoiced_in_account_currency, 0.0);
    assert_eq!(row.paid_in_account_currency, 0.0);
    assert_eq!(row.credit_note_in_account_currency, 0.0);
    assert_eq!(row.outstanding_in_account_currency, 0.0);
}

#[test]
fn accounts_receivable_init_voucher_balance_keys_and_invoice_tracking_match_erpnext() {
    let invoice = ple(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-0001",
        "Sales Invoice",
        "SINV-0001",
        100.0,
    );
    let mut balances = BTreeMap::new();
    let mut invoices = Vec::new();

    init_voucher_balance(
        &mut balances,
        &mut invoices,
        &invoice,
        &filters(),
        &["Sales Invoice".to_string(), "Purchase Invoice".to_string()],
    );

    let key =
        VoucherBalanceKey::with_account("Debtors - TC", "Sales Invoice", "SINV-0001", "CUST-001");
    assert!(balances.contains_key(&key));
    assert_eq!(balances[&key].cost_center, Some("Main - TC".to_string()));
    assert_eq!(balances[&key].project, Some("PROJ-001".to_string()));
    assert_eq!(invoices, vec!["SINV-0001".to_string()]);

    let mut ignored_account_balances = BTreeMap::new();
    init_voucher_balance(
        &mut ignored_account_balances,
        &mut Vec::new(),
        &invoice,
        &ReceivablePayableFilters {
            ignore_accounts: true,
            ..filters()
        },
        &[],
    );

    assert!(
        ignored_account_balances.contains_key(&VoucherBalanceKey::without_account(
            "Sales Invoice",
            "SINV-0001",
            "CUST-001"
        ))
    );
}

#[test]
fn accounts_receivable_update_voucher_balance_matches_erpnext_amount_branches() {
    let invoice = ple(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-0001",
        "Sales Invoice",
        "SINV-0001",
        100.0,
    );
    let payment = ple(
        "Debtors - TC",
        "Payment Entry",
        "PAY-0001",
        "Sales Invoice",
        "SINV-0001",
        40.0,
    );
    let credit_note = ple(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-RET-0001",
        "Sales Invoice",
        "SINV-0001",
        -15.0,
    );
    let mut balances = BTreeMap::from([(
        VoucherBalanceKey::with_account("Debtors - TC", "Sales Invoice", "SINV-0001", "CUST-001"),
        build_voucher_dict(&invoice),
    )]);

    update_voucher_balance(&mut balances, &invoice, &filters(), &BTreeMap::new(), &[]);
    update_voucher_balance(&mut balances, &payment, &filters(), &BTreeMap::new(), &[]);
    update_voucher_balance(
        &mut balances,
        &credit_note,
        &filters(),
        &BTreeMap::new(),
        &[],
    );

    let row = &balances[&VoucherBalanceKey::with_account(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-0001",
        "CUST-001",
    )];
    assert_eq!(row.invoiced, 100.0);
    assert_eq!(row.paid, -40.0);
    assert_eq!(row.credit_note, 15.0);
    assert_eq!(row.invoiced_in_account_currency, 200.0);
    assert_eq!(row.paid_in_account_currency, -80.0);
    assert_eq!(row.credit_note_in_account_currency, 30.0);
}

#[test]
fn accounts_receivable_update_voucher_balance_uses_party_currency_and_return_entry_remap() {
    let invoice = ple(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-0001",
        "Sales Invoice",
        "SINV-0001",
        100.0,
    );
    let mut payment_against_return = ple(
        "Debtors - TC",
        "Payment Entry",
        "PAY-0001",
        "Sales Invoice",
        "SINV-RET-0001",
        10.0,
    );
    payment_against_return.amount_in_account_currency = 25.0;
    let mut balances = BTreeMap::from([(
        VoucherBalanceKey::with_account("Debtors - TC", "Sales Invoice", "SINV-0001", "CUST-001"),
        build_voucher_dict(&invoice),
    )]);
    let return_entries = BTreeMap::from([("SINV-RET-0001".to_string(), "SINV-0001".to_string())]);

    update_voucher_balance(
        &mut balances,
        &payment_against_return,
        &ReceivablePayableFilters {
            in_party_currency: true,
            ..filters()
        },
        &return_entries,
        &[],
    );

    assert_eq!(
        balances[&VoucherBalanceKey::with_account(
            "Debtors - TC",
            "Sales Invoice",
            "SINV-0001",
            "CUST-001"
        )]
            .paid,
        -25.0
    );
}

#[test]
fn accounts_receivable_employee_advance_creates_separate_row_when_enabled_like_erpnext() {
    let advance_payment = ple(
        "Debtors - TC",
        "Payment Entry",
        "PAY-0001",
        "Employee Advance",
        "EMP-ADV-0001",
        75.0,
    );
    let mut balances: BTreeMap<VoucherBalanceKey, VoucherBalanceRow> = BTreeMap::new();

    update_voucher_balance(
        &mut balances,
        &advance_payment,
        &ReceivablePayableFilters {
            handle_employee_advances: true,
            ..filters()
        },
        &BTreeMap::new(),
        &[],
    );

    let key = VoucherBalanceKey::with_account(
        "Debtors - TC",
        "Employee Advance",
        "EMP-ADV-0001",
        "CUST-001",
    );
    assert!(balances.contains_key(&key));
    assert_eq!(balances[&key].voucher_type, "Employee Advance");
    assert_eq!(balances[&key].voucher_no, "EMP-ADV-0001");
    assert_eq!(balances[&key].paid, -75.0);
}

#[test]
fn accounts_receivable_ageing_uses_due_date_with_posting_fallback_like_erpnext() {
    let runtime = ReceivablePayableRuntime {
        account_type: AccountType::Receivable,
        party_naming_by: "Naming Series".to_string(),
        ranges: vec![
            "30".to_string(),
            "60".to_string(),
            "90".to_string(),
            "120".to_string(),
        ],
        range_numbers: vec![1, 2, 3, 4, 5],
    };
    let report_filters = ReceivablePayableFilters {
        ageing_based_on: Some("Due Date".to_string()),
        ..filters()
    };
    let mut due_date_row = ReceivablePayableAgeingRow {
        posting_date: "2026-04-01".to_string(),
        due_date: Some("2026-05-10".to_string()),
        bill_date: None,
        outstanding: 75.0,
        age: 0,
        range0: 0.0,
        ranges: Vec::new(),
        total_due: 0.0,
    };
    let mut fallback_row = ReceivablePayableAgeingRow {
        due_date: None,
        ..due_date_row.clone()
    };

    set_ageing(&mut due_date_row, &report_filters, &runtime, "2026-05-29");
    set_ageing(&mut fallback_row, &report_filters, &runtime, "2026-05-29");

    assert_eq!(due_date_row.age, 19);
    assert_eq!(due_date_row.range0, 0.0);
    assert_eq!(due_date_row.ranges, vec![75.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(due_date_row.total_due, 75.0);
    assert_eq!(fallback_row.age, 58);
    assert_eq!(fallback_row.ranges, vec![0.0, 75.0, 0.0, 0.0, 0.0]);
}

#[test]
fn accounts_receivable_ageing_supplier_invoice_date_uses_bill_date_like_erpnext() {
    let runtime = ReceivablePayableRuntime {
        account_type: AccountType::Payable,
        party_naming_by: "Naming Series".to_string(),
        ranges: vec![
            "30".to_string(),
            "60".to_string(),
            "90".to_string(),
            "120".to_string(),
        ],
        range_numbers: vec![1, 2, 3, 4, 5],
    };
    let mut row = ReceivablePayableAgeingRow {
        posting_date: "2026-05-20".to_string(),
        due_date: Some("2026-05-25".to_string()),
        bill_date: Some("2026-02-10".to_string()),
        outstanding: 88.0,
        age: 0,
        range0: 0.0,
        ranges: Vec::new(),
        total_due: 0.0,
    };

    set_ageing(
        &mut row,
        &ReceivablePayableFilters {
            ageing_based_on: Some("Supplier Invoice Date".to_string()),
            ..filters()
        },
        &runtime,
        "2026-05-29",
    );

    assert_eq!(row.age, 108);
    assert_eq!(row.ranges, vec![0.0, 0.0, 0.0, 88.0, 0.0]);
    assert_eq!(row.total_due, 88.0);
}

#[test]
fn accounts_receivable_ageing_future_entry_sets_range0_and_zero_total_like_erpnext() {
    let runtime = ReceivablePayableRuntime {
        account_type: AccountType::Receivable,
        party_naming_by: "Naming Series".to_string(),
        ranges: vec!["30".to_string(), "60".to_string()],
        range_numbers: vec![1, 2, 3],
    };
    let mut row = ReceivablePayableAgeingRow {
        posting_date: "2026-06-01".to_string(),
        due_date: None,
        bill_date: None,
        outstanding: 42.0,
        age: 0,
        range0: 0.0,
        ranges: Vec::new(),
        total_due: 0.0,
    };

    set_ageing(&mut row, &filters(), &runtime, "2026-05-29");

    assert_eq!(row.age, -3);
    assert_eq!(row.range0, 42.0);
    assert_eq!(row.ranges, vec![0.0, 0.0, 0.0]);
    assert_eq!(row.total_due, 0.0);
}

#[test]
fn accounts_receivable_allocate_future_payments_returns_early_when_hidden_like_erpnext() {
    let mut row = FuturePaymentAllocationRow {
        voucher_no: "SINV-0001".to_string(),
        party: "CUST-001".to_string(),
        outstanding: 100.0,
        remaining_balance: 12.0,
        future_amount: 34.0,
        future_ref: Some("OLD".to_string()),
    };
    let mut future_payments = BTreeMap::from([(
        ("SINV-0001".to_string(), "CUST-001".to_string()),
        vec![FuturePayment {
            invoice_no: "SINV-0001".to_string(),
            party: "CUST-001".to_string(),
            future_date: "2026-06-10".to_string(),
            future_ref: "CHK-001".to_string(),
            future_amount: 50.0,
            future_amount_in_base_currency: 60.0,
        }],
    )]);

    allocate_future_payments(&mut row, &filters(), &mut future_payments);

    assert_eq!(row.remaining_balance, 12.0);
    assert_eq!(row.future_amount, 34.0);
    assert_eq!(row.future_ref, Some("OLD".to_string()));
    assert_eq!(
        future_payments[&("SINV-0001".to_string(), "CUST-001".to_string())][0].future_amount,
        50.0
    );
}

#[test]
fn accounts_receivable_allocate_future_payments_caps_single_base_amount_like_erpnext() {
    let mut row = FuturePaymentAllocationRow {
        voucher_no: "SINV-0001".to_string(),
        party: "CUST-001".to_string(),
        outstanding: 100.0,
        remaining_balance: 0.0,
        future_amount: 0.0,
        future_ref: None,
    };
    let mut future_payments = BTreeMap::from([(
        ("SINV-0001".to_string(), "CUST-001".to_string()),
        vec![FuturePayment {
            invoice_no: "SINV-0001".to_string(),
            party: "CUST-001".to_string(),
            future_date: "2026-06-10".to_string(),
            future_ref: "CHK-001".to_string(),
            future_amount: 25.0,
            future_amount_in_base_currency: 150.0,
        }],
    )]);

    allocate_future_payments(
        &mut row,
        &ReceivablePayableFilters {
            show_future_payments: true,
            ..filters()
        },
        &mut future_payments,
    );

    assert_eq!(row.future_amount, 100.0);
    assert_eq!(row.remaining_balance, 0.0);
    assert_eq!(row.future_ref, Some("CHK-001/2026-06-10".to_string()));
    assert_eq!(
        future_payments[&("SINV-0001".to_string(), "CUST-001".to_string())][0].future_amount,
        25.0
    );
    assert_eq!(
        future_payments[&("SINV-0001".to_string(), "CUST-001".to_string())][0]
            .future_amount_in_base_currency,
        50.0
    );
}

#[test]
fn accounts_receivable_allocate_future_payments_uses_party_currency_and_joins_refs_like_erpnext() {
    let mut row = FuturePaymentAllocationRow {
        voucher_no: "SINV-0001".to_string(),
        party: "CUST-001".to_string(),
        outstanding: 100.0,
        remaining_balance: 0.0,
        future_amount: 0.0,
        future_ref: None,
    };
    let key = ("SINV-0001".to_string(), "CUST-001".to_string());
    let mut future_payments = BTreeMap::from([(
        key.clone(),
        vec![
            FuturePayment {
                invoice_no: "SINV-0001".to_string(),
                party: "CUST-001".to_string(),
                future_date: "2026-06-10".to_string(),
                future_ref: "CHK-001".to_string(),
                future_amount: 25.0,
                future_amount_in_base_currency: 250.0,
            },
            FuturePayment {
                invoice_no: "SINV-0001".to_string(),
                party: "CUST-001".to_string(),
                future_date: "2026-06-15".to_string(),
                future_ref: "CHK-002".to_string(),
                future_amount: 30.0,
                future_amount_in_base_currency: 300.0,
            },
        ],
    )]);

    allocate_future_payments(
        &mut row,
        &ReceivablePayableFilters {
            show_future_payments: true,
            in_party_currency: true,
            ..filters()
        },
        &mut future_payments,
    );

    assert_eq!(row.future_amount, 55.0);
    assert_eq!(row.remaining_balance, 45.0);
    assert_eq!(
        row.future_ref,
        Some("CHK-001/2026-06-10, CHK-002/2026-06-15".to_string())
    );
    assert_eq!(future_payments[&key][0].future_amount, 0.0);
    assert_eq!(
        future_payments[&key][0].future_amount_in_base_currency,
        250.0
    );
    assert_eq!(future_payments[&key][1].future_amount, 0.0);
    assert_eq!(
        future_payments[&key][1].future_amount_in_base_currency,
        300.0
    );
}

#[test]
fn accounts_receivable_group_future_payments_matches_erpnext_filter_and_merge_rules() {
    let payment_entry = FuturePayment {
        invoice_no: "SINV-0001".to_string(),
        party: "CUST-001".to_string(),
        future_date: "2026-06-10".to_string(),
        future_ref: "PAY-REF".to_string(),
        future_amount: 25.0,
        future_amount_in_base_currency: 250.0,
    };
    let journal_entry = FuturePayment {
        future_ref: "JE-REF".to_string(),
        future_date: "2026-06-15".to_string(),
        ..payment_entry.clone()
    };
    let zero_party_amount = FuturePayment {
        invoice_no: "SINV-0002".to_string(),
        party: "CUST-001".to_string(),
        future_amount: 0.0,
        future_amount_in_base_currency: 500.0,
        ..payment_entry.clone()
    };
    let missing_invoice = FuturePayment {
        invoice_no: String::new(),
        party: "CUST-001".to_string(),
        future_amount: 10.0,
        ..payment_entry.clone()
    };

    let grouped = group_future_payments(
        &ReceivablePayableFilters {
            show_future_payments: true,
            ..filters()
        },
        vec![payment_entry],
        vec![journal_entry, zero_party_amount, missing_invoice],
    );

    let key = ("SINV-0001".to_string(), "CUST-001".to_string());
    assert_eq!(grouped.len(), 1);
    assert_eq!(grouped[&key].len(), 2);
    assert_eq!(grouped[&key][0].future_ref, "PAY-REF");
    assert_eq!(grouped[&key][1].future_ref, "JE-REF");
}

#[test]
fn accounts_receivable_group_future_payments_returns_empty_when_hidden_like_erpnext() {
    let grouped = group_future_payments(
        &filters(),
        vec![FuturePayment {
            invoice_no: "SINV-0001".to_string(),
            party: "CUST-001".to_string(),
            future_date: "2026-06-10".to_string(),
            future_ref: "PAY-REF".to_string(),
            future_amount: 25.0,
            future_amount_in_base_currency: 250.0,
        }],
        Vec::new(),
    );

    assert!(grouped.is_empty());
}

#[test]
fn accounts_receivable_set_invoice_details_preserves_existing_due_date_and_sales_details() {
    let mut row = InvoiceDetailsRow {
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: "SINV-0001".to_string(),
        due_date: Some("2026-05-10".to_string()),
        po_no: None,
        bill_no: None,
        bill_date: None,
        sales_team: Vec::new(),
        sales_person: None,
        delivery_notes: None,
    };
    let invoice_details = BTreeMap::from([(
        "SINV-0001".to_string(),
        InvoiceDetails {
            due_date: Some("2026-06-09".to_string()),
            po_no: Some("PO-001".to_string()),
            bill_no: None,
            bill_date: None,
            sales_team: vec!["Ada".to_string(), "Grace".to_string()],
        },
    )]);
    let delivery_notes = BTreeMap::from([(
        "SINV-0001".to_string(),
        vec!["DN-0002".to_string(), "DN-0001".to_string()],
    )]);

    set_invoice_details(
        &mut row,
        &ReceivablePayableFilters {
            show_delivery_notes: true,
            show_sales_person: true,
            ..filters()
        },
        &invoice_details,
        &delivery_notes,
    );

    assert_eq!(row.due_date, Some("2026-05-10".to_string()));
    assert_eq!(row.po_no, Some("PO-001".to_string()));
    assert_eq!(row.delivery_notes, Some("DN-0002, DN-0001".to_string()));
    assert_eq!(row.sales_person, Some("Ada, Grace".to_string()));
    assert!(row.sales_team.is_empty());
}

#[test]
fn accounts_receivable_set_invoice_details_updates_purchase_invoice_bill_fields_like_erpnext() {
    let mut row = InvoiceDetailsRow {
        voucher_type: "Purchase Invoice".to_string(),
        voucher_no: "PINV-0001".to_string(),
        due_date: None,
        po_no: None,
        bill_no: None,
        bill_date: None,
        sales_team: Vec::new(),
        sales_person: None,
        delivery_notes: None,
    };
    let invoice_details = BTreeMap::from([(
        "PINV-0001".to_string(),
        InvoiceDetails {
            due_date: Some("2026-06-30".to_string()),
            po_no: None,
            bill_no: Some("BILL-777".to_string()),
            bill_date: Some("2026-05-20".to_string()),
            sales_team: Vec::new(),
        },
    )]);

    set_invoice_details(&mut row, &filters(), &invoice_details, &BTreeMap::new());

    assert_eq!(row.due_date, Some("2026-06-30".to_string()));
    assert_eq!(row.bill_no, Some("BILL-777".to_string()));
    assert_eq!(row.bill_date, Some("2026-05-20".to_string()));
}

#[test]
fn accounts_receivable_set_invoice_details_skips_sales_only_branches_when_disabled_like_erpnext() {
    let mut row = InvoiceDetailsRow {
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: "SINV-0001".to_string(),
        due_date: None,
        po_no: None,
        bill_no: None,
        bill_date: None,
        sales_team: Vec::new(),
        sales_person: None,
        delivery_notes: None,
    };
    let invoice_details = BTreeMap::from([(
        "SINV-0001".to_string(),
        InvoiceDetails {
            due_date: Some("2026-06-09".to_string()),
            po_no: Some("PO-001".to_string()),
            bill_no: None,
            bill_date: None,
            sales_team: vec!["Ada".to_string()],
        },
    )]);
    let delivery_notes = BTreeMap::from([("SINV-0001".to_string(), vec!["DN-0001".to_string()])]);

    set_invoice_details(&mut row, &filters(), &invoice_details, &delivery_notes);

    assert_eq!(row.due_date, Some("2026-06-09".to_string()));
    assert_eq!(row.po_no, Some("PO-001".to_string()));
    assert_eq!(row.sales_team, vec!["Ada".to_string()]);
    assert_eq!(row.sales_person, None);
    assert_eq!(row.delivery_notes, None);
}

#[test]
fn accounts_receivable_set_party_details_returns_early_without_party_like_erpnext() {
    let mut row = PartyDetailsRow {
        party: String::new(),
        account_currency: "EUR".to_string(),
        currency: None,
        customer_name: None,
        territory: None,
        customer_group: None,
        customer_primary_contact: None,
        default_sales_partner: None,
        supplier_name: None,
        supplier_group: None,
    };

    set_party_details(&mut row, &filters(), "USD", &BTreeMap::new());

    assert_eq!(row.currency, None);
    assert_eq!(row.customer_name, None);
}

#[test]
fn accounts_receivable_set_party_details_updates_customer_and_company_currency_like_erpnext() {
    let mut row = PartyDetailsRow {
        party: "CUST-001".to_string(),
        account_currency: "EUR".to_string(),
        currency: None,
        customer_name: None,
        territory: None,
        customer_group: None,
        customer_primary_contact: None,
        default_sales_partner: None,
        supplier_name: None,
        supplier_group: None,
    };
    let party_details = BTreeMap::from([(
        "CUST-001".to_string(),
        PartyDetails {
            customer_name: Some("Acme".to_string()),
            territory: Some("Uzbekistan".to_string()),
            customer_group: Some("Commercial".to_string()),
            customer_primary_contact: Some("CONT-001".to_string()),
            default_sales_partner: Some("Partner A".to_string()),
            supplier_name: None,
            supplier_group: None,
        },
    )]);

    set_party_details(&mut row, &filters(), "USD", &party_details);

    assert_eq!(row.customer_name, Some("Acme".to_string()));
    assert_eq!(row.territory, Some("Uzbekistan".to_string()));
    assert_eq!(row.customer_group, Some("Commercial".to_string()));
    assert_eq!(row.customer_primary_contact, Some("CONT-001".to_string()));
    assert_eq!(row.default_sales_partner, Some("Partner A".to_string()));
    assert_eq!(row.currency, Some("USD".to_string()));
}

#[test]
fn accounts_receivable_set_party_details_uses_account_currency_for_party_currency_filters() {
    let mut row = PartyDetailsRow {
        party: "SUPP-001".to_string(),
        account_currency: "EUR".to_string(),
        currency: None,
        customer_name: None,
        territory: None,
        customer_group: None,
        customer_primary_contact: None,
        default_sales_partner: None,
        supplier_name: None,
        supplier_group: None,
    };
    let party_details = BTreeMap::from([(
        "SUPP-001".to_string(),
        PartyDetails {
            customer_name: None,
            territory: None,
            customer_group: None,
            customer_primary_contact: None,
            default_sales_partner: None,
            supplier_name: Some("Supplier A".to_string()),
            supplier_group: Some("Services".to_string()),
        },
    )]);

    set_party_details(
        &mut row,
        &ReceivablePayableFilters {
            party_account: Some("Creditors - TC".to_string()),
            ..filters()
        },
        "USD",
        &party_details,
    );

    assert_eq!(row.supplier_name, Some("Supplier A".to_string()));
    assert_eq!(row.supplier_group, Some("Services".to_string()));
    assert_eq!(row.currency, Some("EUR".to_string()));
}
