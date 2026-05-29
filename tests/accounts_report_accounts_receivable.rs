use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::{
    accounting_dimension_filter_conditions, accounts_receivable_args, add_common_filter_conditions,
    add_customer_filter_conditions, add_project_and_cost_center_conditions,
    add_supplier_filter_conditions, allocate_extra_payments_or_credits, allocate_future_payments,
    build_chart_data, build_delivery_note_map, build_exchange_rate_revaluations_plan,
    build_return_entries_plan, build_sales_person_records, build_voucher_dict, get_columns,
    get_currency_fields, get_party_group_with_children, group_future_payments,
    init_voucher_balance, is_invoice_type, payment_term_template_filter_conditions,
    prepare_conditions_plan, prepare_ple_query_plan, prepare_voucher_balance_rows, set_ageing,
    set_invoice_details, set_party_details, update_voucher_balance, AccountType,
    AccountingDimension, ChartInputRow, DeliveryNoteAgainstSalesInvoice, FuturePayment,
    FuturePaymentAllocationRow, InvoiceDetails, InvoiceDetailsRow, PartyDetails, PartyDetailsRow,
    PaymentLedgerEntry, PaymentTermAllocationRow, PaymentTermDetail, PaymentTermRow,
    ReceivablePayableAgeingRow, ReceivablePayableFilters, ReceivablePayableRuntime,
    ReceivablePayableSettings, ReceivablePayableState, ReportColumn, SalesInvoiceDeliveryNote,
    SalesPersonRecord, SubtotalDataRow, VoucherBalanceKey, VoucherBalanceRow,
};

fn filters() -> ReceivablePayableFilters {
    ReceivablePayableFilters {
        company: None,
        report_date: None,
        finance_book: None,
        party_type: None,
        customer_group: None,
        territory: None,
        supplier_group: None,
        payment_terms_template: None,
        cost_center: Vec::new(),
        project: Vec::new(),
        accounting_dimensions: BTreeMap::new(),
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
        sales_person: None,
        show_remarks: false,
        sales_partner: None,
        ignore_accounts: false,
        handle_employee_advances: false,
        for_revaluation_journals: false,
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

fn payment_term_detail(due_date: &str, base_payment_amount: f64) -> PaymentTermDetail {
    PaymentTermDetail {
        party_account_currency: "USD".to_string(),
        currency: "USD".to_string(),
        total_advance: 0.0,
        due_date: due_date.to_string(),
        payment_term: "Net 30".to_string(),
        payment_amount: base_payment_amount,
        base_payment_amount,
        description: None,
        paid_amount: 0.0,
        base_paid_amount: 0.0,
        discounted_amount: 0.0,
    }
}

#[test]
fn accounts_receivable_payment_terms_single_term_uses_base_currency_and_description_like_erpnext() {
    let mut row = PaymentTermAllocationRow {
        invoiced: 1000.0,
        paid: 200.0,
        credit_note: 0.0,
        payment_terms: Vec::new(),
    };
    let mut detail = payment_term_detail("2026-06-30", 600.0);
    detail.total_advance = 50.0;
    detail.description = Some("Milestone".to_string());
    detail.base_paid_amount = 80.0;
    detail.paid_amount = 90.0;
    detail.discounted_amount = 5.0;

    tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::allocate_outstanding_based_on_payment_terms(
        &mut row,
        &ReceivablePayableFilters {
            in_party_currency: true,
            ..filters()
        },
        &[detail],
        "USD",
    );

    assert_eq!(row.paid, 65.0);
    assert_eq!(
        row.payment_terms,
        vec![PaymentTermRow {
            due_date: "2026-06-30".to_string(),
            invoiced: 600.0,
            invoice_grand_total: 1000.0,
            payment_term: "Milestone".to_string(),
            paid: 85.0,
            credit_note: 0.0,
            outstanding: 515.0,
        }]
    );
}

#[test]
fn accounts_receivable_payment_terms_uses_party_currency_only_when_invoice_and_account_match() {
    let mut row = PaymentTermAllocationRow {
        invoiced: 1000.0,
        paid: 0.0,
        credit_note: 0.0,
        payment_terms: Vec::new(),
    };
    let mut detail = payment_term_detail("2026-06-30", 600.0);
    detail.currency = "EUR".to_string();
    detail.party_account_currency = "EUR".to_string();
    detail.payment_amount = 700.0;
    detail.base_paid_amount = 80.0;
    detail.paid_amount = 90.0;

    tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::allocate_outstanding_based_on_payment_terms(
        &mut row,
        &ReceivablePayableFilters {
            in_party_currency: true,
            ..filters()
        },
        &[detail],
        "USD",
    );

    assert_eq!(row.payment_terms[0].invoiced, 700.0);
    assert_eq!(row.payment_terms[0].paid, 90.0);
    assert_eq!(row.payment_terms[0].outstanding, 610.0);
}

#[test]
fn accounts_receivable_payment_terms_allocate_paid_and_credit_note_fifo_like_erpnext() {
    let mut row = PaymentTermAllocationRow {
        invoiced: 150.0,
        paid: 80.0,
        credit_note: 20.0,
        payment_terms: Vec::new(),
    };
    let mut first = payment_term_detail("2026-07-30", 50.0);
    first.payment_term = "Second".to_string();
    let mut second = payment_term_detail("2026-06-30", 100.0);
    second.payment_term = "First".to_string();

    tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::allocate_outstanding_based_on_payment_terms(
        &mut row,
        &filters(),
        &[first, second],
        "USD",
    );

    assert_eq!(row.paid, 0.0);
    assert_eq!(row.credit_note, 0.0);
    assert_eq!(
        row.payment_terms,
        vec![
            PaymentTermRow {
                due_date: "2026-06-30".to_string(),
                invoiced: 100.0,
                invoice_grand_total: 150.0,
                payment_term: "First".to_string(),
                paid: 30.0,
                credit_note: 20.0,
                outstanding: 50.0,
            },
            PaymentTermRow {
                due_date: "2026-07-30".to_string(),
                invoiced: 50.0,
                invoice_grand_total: 150.0,
                payment_term: "Second".to_string(),
                paid: 50.0,
                credit_note: 0.0,
                outstanding: 0.0,
            },
        ]
    );
}

#[test]
fn accounts_receivable_update_sub_total_row_adds_currency_fields_and_currency_like_erpnext() {
    let mut totals = BTreeMap::from([(
        "CUST-001".to_string(),
        SubtotalDataRow::with_values([("invoiced", 10.0), ("paid", 4.0)], "USD"),
    )]);
    let row = SubtotalDataRow::with_values(
        [
            ("invoiced", 90.0),
            ("paid", 6.0),
            ("credit_note", 3.0),
            ("range1", 20.0),
            ("future_amount", 7.0),
        ],
        "EUR",
    );

    tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::update_sub_total_row(
        &mut totals,
        &row,
        "CUST-001",
    );

    let total = &totals["CUST-001"];
    assert_eq!(total.currency_values["invoiced"], 100.0);
    assert_eq!(total.currency_values["paid"], 10.0);
    assert_eq!(total.currency_values["credit_note"], 3.0);
    assert_eq!(total.currency_values["range1"], 20.0);
    assert_eq!(total.currency_values["future_amount"], 7.0);
    assert_eq!(total.currency, "EUR");
}

#[test]
fn accounts_receivable_append_subtotal_row_appends_separator_and_updates_total_like_erpnext() {
    let mut totals = BTreeMap::from([
        (
            "CUST-001".to_string(),
            SubtotalDataRow::with_values([("outstanding", 60.0), ("range2", 12.0)], "USD"),
        ),
        (
            "Total".to_string(),
            SubtotalDataRow::with_values([("outstanding", 40.0)], "USD"),
        ),
    ]);
    let mut data = Vec::new();

    tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::append_subtotal_row(
        &mut data,
        &mut totals,
        "CUST-001",
    );

    assert_eq!(data.len(), 2);
    assert_eq!(
        data[0],
        SubtotalDataRow::with_values([("outstanding", 60.0), ("range2", 12.0)], "USD")
    );
    assert!(data[1].is_empty);
    assert_eq!(totals["Total"].currency_values["outstanding"], 100.0);
    assert_eq!(totals["Total"].currency_values["range2"], 12.0);
}

#[test]
fn accounts_receivable_prepare_voucher_balance_rows_rounds_and_filters_like_erpnext() {
    let base_ple = ple(
        "Debtors - TC",
        "Sales Invoice",
        "SINV-0001",
        "Sales Invoice",
        "SINV-0001",
        0.0,
    );
    let mut keep = build_voucher_dict(&base_ple);
    keep.invoiced = 100.004;
    keep.paid = 40.0;
    keep.credit_note = 10.0;
    keep.invoiced_in_account_currency = 100.0;
    keep.paid_in_account_currency = 25.0;
    keep.credit_note_in_account_currency = 5.0;
    let mut drop_without_account_outstanding = keep.clone();
    drop_without_account_outstanding.voucher_no = "SINV-0002".to_string();
    drop_without_account_outstanding.invoiced_in_account_currency = 10.0;
    drop_without_account_outstanding.paid_in_account_currency = 10.0;
    drop_without_account_outstanding.credit_note_in_account_currency = 0.0;
    let rows = BTreeMap::from([
        (
            VoucherBalanceKey::with_account(
                "Debtors - TC",
                "Sales Invoice",
                "SINV-0001",
                "CUST-001",
            ),
            keep,
        ),
        (
            VoucherBalanceKey::with_account(
                "Debtors - TC",
                "Sales Invoice",
                "SINV-0002",
                "CUST-001",
            ),
            drop_without_account_outstanding,
        ),
    ]);

    let prepared = prepare_voucher_balance_rows(&rows, &filters(), 2, &[]);

    assert_eq!(prepared.len(), 1);
    assert_eq!(prepared[0].voucher_no, "SINV-0001");
    assert_eq!(prepared[0].outstanding, 50.0);
    assert_eq!(prepared[0].outstanding_in_account_currency, 70.0);
    assert_eq!(prepared[0].invoice_grand_total, 100.004);
}

#[test]
fn accounts_receivable_prepare_voucher_balance_rows_keeps_err_journals_like_erpnext() {
    let base_ple = ple(
        "Debtors - TC",
        "Journal Entry",
        "JV-0001",
        "Journal Entry",
        "JV-0001",
        0.0,
    );
    let mut row = build_voucher_dict(&base_ple);
    row.invoiced = 100.0;
    row.paid = 0.0;
    row.credit_note = 0.0;
    row.invoiced_in_account_currency = 100.0;
    row.paid_in_account_currency = 100.0;
    row.credit_note_in_account_currency = 0.0;
    let rows = BTreeMap::from([(
        VoucherBalanceKey::with_account("Debtors - TC", "Journal Entry", "JV-0001", "CUST-001"),
        row,
    )]);

    let prepared = prepare_voucher_balance_rows(&rows, &filters(), 2, &["JV-0001".to_string()]);

    assert_eq!(prepared.len(), 1);
    assert_eq!(prepared[0].outstanding, 100.0);
    assert_eq!(prepared[0].outstanding_in_account_currency, 0.0);
}

#[test]
fn accounts_receivable_prepare_voucher_balance_rows_revaluation_uses_either_currency_balance() {
    let base_ple = ple(
        "Debtors - TC",
        "Journal Entry",
        "JV-0002",
        "Journal Entry",
        "JV-0002",
        0.0,
    );
    let mut row = build_voucher_dict(&base_ple);
    row.invoiced = 20.0;
    row.paid = 20.0;
    row.credit_note = 0.0;
    row.invoiced_in_account_currency = 30.0;
    row.paid_in_account_currency = 25.0;
    row.credit_note_in_account_currency = 0.0;
    let rows = BTreeMap::from([(
        VoucherBalanceKey::with_account("Debtors - TC", "Journal Entry", "JV-0002", "CUST-001"),
        row,
    )]);

    let prepared = prepare_voucher_balance_rows(
        &rows,
        &ReceivablePayableFilters {
            for_revaluation_journals: true,
            ..filters()
        },
        2,
        &[],
    );

    assert_eq!(prepared.len(), 1);
    assert_eq!(prepared[0].outstanding, 0.0);
    assert_eq!(prepared[0].outstanding_in_account_currency, 5.0);
}

#[test]
fn accounts_receivable_prepare_ple_query_plan_matches_standard_date_order_and_fields() {
    let plan = prepare_ple_query_plan(
        &ReceivablePayableFilters {
            report_date: Some("2026-05-29".to_string()),
            ..filters()
        },
        false,
        None,
    );

    assert_eq!(
        plan.selected_fields,
        vec![
            "name",
            "account",
            "voucher_type",
            "voucher_no",
            "against_voucher_type",
            "against_voucher_no",
            "party_type",
            "cost_center",
            "project",
            "party",
            "posting_date",
            "due_date",
            "account_currency",
            "amount",
            "amount_in_account_currency",
        ]
    );
    assert_eq!(
        plan.date_condition,
        "posting_date <= '2026-05-29'".to_string()
    );
    assert_eq!(plan.order_by, vec!["posting_date", "party"]);
    assert_eq!(plan.remarks_selection, None);
    assert!(plan.delinked_zero);
}

#[test]
fn accounts_receivable_prepare_ple_query_plan_matches_future_payment_date_condition_like_erpnext() {
    let plan = prepare_ple_query_plan(
        &ReceivablePayableFilters {
            report_date: Some("2026-05-29".to_string()),
            show_future_payments: true,
            group_by_party: true,
            show_remarks: true,
            ..filters()
        },
        true,
        Some(80),
    );

    assert_eq!(
        plan.date_condition,
        "posting_date <= '2026-05-29' OR (voucher_no = against_voucher_no AND DATE(creation) <= '2026-05-29')"
    );
    assert_eq!(plan.order_by, vec!["party", "posting_date"]);
    assert_eq!(
        plan.remarks_selection,
        Some("SUBSTRING(remarks, 1, 80)".to_string())
    );
}

#[test]
fn accounts_receivable_prepare_ple_query_plan_selects_full_remarks_when_length_missing() {
    let plan = prepare_ple_query_plan(
        &ReceivablePayableFilters {
            show_remarks: true,
            ..filters()
        },
        true,
        None,
    );

    assert_eq!(plan.remarks_selection, Some("remarks".to_string()));
}

#[test]
fn accounts_receivable_add_common_filter_conditions_uses_party_account_when_present_like_erpnext() {
    let conditions = add_common_filter_conditions(
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            finance_book: Some("IFRS".to_string()),
            party_type: Some("Customer".to_string()),
            party: vec!["CUST-001".to_string(), "CUST-002".to_string()],
            party_account: Some("Debtors - TC".to_string()),
            ..filters()
        },
        &["Debtors - TC".to_string(), "Other Debtors - TC".to_string()],
    );

    assert_eq!(
        conditions,
        vec![
            "company = '_Test Company'",
            "finance_book = 'IFRS'",
            "party_type = 'Customer'",
            "party IN ('CUST-001', 'CUST-002')",
            "account = 'Debtors - TC'",
        ]
    );
}

#[test]
fn accounts_receivable_add_common_filter_conditions_falls_back_to_account_type_accounts() {
    let conditions = add_common_filter_conditions(
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            ..filters()
        },
        &["Debtors - TC".to_string(), "Other Debtors - TC".to_string()],
    );

    assert_eq!(
        conditions,
        vec![
            "company = '_Test Company'",
            "account IN ('Debtors - TC', 'Other Debtors - TC')",
        ]
    );
}

#[test]
fn accounts_receivable_add_common_filter_conditions_skips_account_filter_when_no_accounts() {
    let conditions = add_common_filter_conditions(
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            ..filters()
        },
        &[],
    );

    assert_eq!(conditions, vec!["company = '_Test Company'"]);
}

#[test]
fn accounts_receivable_customer_filter_conditions_match_customer_specific_branches() {
    let conditions = add_customer_filter_conditions(
        &ReceivablePayableFilters {
            customer_group: Some("All Customer Groups".to_string()),
            territory: Some("All Territories".to_string()),
            payment_terms_template: Some("Net 30".to_string()),
            sales_partner: Some("Partner A".to_string()),
            ..filters()
        },
        &["All Customer Groups".to_string(), "Retail".to_string()],
        &["All Territories".to_string(), "Tashkent".to_string()],
        &["SINV-0001".to_string(), "SINV-0002".to_string()],
    );

    assert_eq!(
        conditions,
        vec![
            "party IN Customer WHERE customer_group IN ('All Customer Groups', 'Retail')",
            "party IN Customer WHERE territory IN ('All Territories', 'Tashkent')",
            "(party IN Customer WHERE payment_terms = 'Net 30' OR against_voucher_no IN ('SINV-0001', 'SINV-0002'))",
            "party IN Customer WHERE default_sales_partner = 'Partner A'",
            "party_type != 'Employee'",
        ]
    );
}

#[test]
fn accounts_receivable_supplier_filter_conditions_match_supplier_specific_branches() {
    let conditions = add_supplier_filter_conditions(
        &ReceivablePayableFilters {
            supplier_group: Some("All Supplier Groups".to_string()),
            payment_terms_template: Some("Net 45".to_string()),
            ..filters()
        },
        &["PINV-0001".to_string()],
    );

    assert_eq!(
        conditions,
        vec![
            "party IN Supplier WHERE supplier_group = 'All Supplier Groups'",
            "(party IN Supplier WHERE payment_terms = 'Net 45' OR against_voucher_no IN ('PINV-0001'))",
        ]
    );
}

#[test]
fn accounts_receivable_payment_term_template_filter_conditions_match_sales_and_purchase_variants() {
    let sales_conditions = payment_term_template_filter_conditions(
        "Sales Invoice",
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            payment_terms_template: Some("Net 30".to_string()),
            customer_group: Some("Retail".to_string()),
            party: vec!["CUST-001".to_string()],
            cost_center: vec!["Main - TC".to_string()],
            party_account: Some("Debtors - TC".to_string()),
            ..filters()
        },
        &["Retail".to_string(), "Online".to_string()],
        &["Main - TC".to_string(), "Sub - TC".to_string()],
    );
    let purchase_conditions = payment_term_template_filter_conditions(
        "Purchase Invoice",
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            payment_terms_template: Some("Net 45".to_string()),
            supplier_group: Some("Services".to_string()),
            party: vec!["SUPP-001".to_string()],
            cost_center: vec!["Main - TC".to_string()],
            party_account: Some("Creditors - TC".to_string()),
            ..filters()
        },
        &["Services".to_string()],
        &["Main - TC".to_string()],
    );

    assert_eq!(
        sales_conditions,
        vec![
            "payment_terms_template = 'Net 30'",
            "company = '_Test Company'",
            "customer_group IN ('Retail', 'Online')",
            "customer IN ('CUST-001')",
            "cost_center IN ('Main - TC', 'Sub - TC')",
            "debit_to = 'Debtors - TC'",
        ]
    );
    assert_eq!(
        purchase_conditions,
        vec![
            "payment_terms_template = 'Net 45'",
            "company = '_Test Company'",
            "supplier_group IN ('Services')",
            "supplier IN ('SUPP-001')",
            "cost_center IN ('Main - TC')",
            "credit_to = 'Creditors - TC'",
        ]
    );
}

#[test]
fn accounts_receivable_project_and_cost_center_conditions_match_prepare_conditions_branches() {
    let conditions = add_project_and_cost_center_conditions(
        &ReceivablePayableFilters {
            project: vec!["PROJ-001".to_string(), "PROJ-002".to_string()],
            cost_center: vec!["Main - TC".to_string()],
            ..filters()
        },
        &["Main - TC".to_string(), "Sub - TC".to_string()],
    );

    assert_eq!(
        conditions,
        vec![
            "cost_center IN ('Main - TC', 'Sub - TC')",
            "project IN ('PROJ-001', 'PROJ-002')",
        ]
    );
}

#[test]
fn accounts_receivable_project_and_cost_center_conditions_skip_empty_filters() {
    let conditions = add_project_and_cost_center_conditions(&filters(), &[]);

    assert!(conditions.is_empty());
}

#[test]
fn accounts_receivable_build_return_entries_plan_matches_receivable_party_or_filters() {
    let plan = build_return_entries_plan(
        AccountType::Receivable,
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            report_date: Some("2026-05-29".to_string()),
            party_type: Some("Customer".to_string()),
            party: vec!["CUST-001".to_string(), "CUST-002".to_string()],
            ..filters()
        },
    );

    assert_eq!(plan.doctype, "Sales Invoice");
    assert_eq!(
        plan.filters,
        vec![
            "posting_date <= '2026-05-29'",
            "is_return = 1",
            "docstatus = 1",
            "company = '_Test Company'",
            "update_outstanding_for_self = 0",
        ]
    );
    assert_eq!(
        plan.or_filters,
        vec!["customer IN ('CUST-001', 'CUST-002')"]
    );
    assert_eq!(plan.fields, vec!["name", "return_against"]);
}

#[test]
fn accounts_receivable_build_return_entries_plan_matches_payable_without_party_filter() {
    let plan = build_return_entries_plan(
        AccountType::Payable,
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            report_date: Some("2026-05-29".to_string()),
            ..filters()
        },
    );

    assert_eq!(plan.doctype, "Purchase Invoice");
    assert!(plan.or_filters.is_empty());
}

#[test]
fn accounts_receivable_build_delivery_note_map_merges_sales_invoice_and_delivery_note_links() {
    let notes = build_delivery_note_map(
        &["SINV-0001".to_string()],
        true,
        &[
            SalesInvoiceDeliveryNote {
                parent: "SINV-0001".to_string(),
                delivery_note: Some("DN-0002".to_string()),
            },
            SalesInvoiceDeliveryNote {
                parent: "SINV-0001".to_string(),
                delivery_note: None,
            },
        ],
        &[
            DeliveryNoteAgainstSalesInvoice {
                parent: "DN-0001".to_string(),
                against_sales_invoice: "SINV-0001".to_string(),
            },
            DeliveryNoteAgainstSalesInvoice {
                parent: "DN-0002".to_string(),
                against_sales_invoice: "SINV-0001".to_string(),
            },
        ],
    );

    assert_eq!(
        notes["SINV-0001"],
        vec!["DN-0001".to_string(), "DN-0002".to_string()]
    );
}

#[test]
fn accounts_receivable_build_delivery_note_map_returns_empty_when_hidden_or_no_invoices() {
    assert!(build_delivery_note_map(&[], true, &[], &[]).is_empty());
    assert!(build_delivery_note_map(&["SINV-0001".to_string()], false, &[], &[]).is_empty());
}

#[test]
fn accounts_receivable_build_sales_person_records_groups_parenttype_when_filter_present() {
    let records = build_sales_person_records(
        &ReceivablePayableFilters {
            sales_person: Some("Sales User".to_string()),
            ..filters()
        },
        &[
            SalesPersonRecord {
                parent: "CUST-001".to_string(),
                parenttype: "Customer".to_string(),
            },
            SalesPersonRecord {
                parent: "SINV-0001".to_string(),
                parenttype: "Sales Invoice".to_string(),
            },
            SalesPersonRecord {
                parent: "SINV-0002".to_string(),
                parenttype: "Sales Invoice".to_string(),
            },
        ],
    );

    assert_eq!(records["Customer"], vec!["CUST-001".to_string()]);
    assert_eq!(
        records["Sales Invoice"],
        vec!["SINV-0001".to_string(), "SINV-0002".to_string()]
    );
}

#[test]
fn accounts_receivable_build_sales_person_records_returns_empty_without_filter() {
    assert!(build_sales_person_records(&filters(), &[]).is_empty());
}

#[test]
fn accounts_receivable_accounting_dimension_conditions_match_tree_and_plain_dimensions() {
    let conditions = accounting_dimension_filter_conditions(
        &ReceivablePayableFilters {
            accounting_dimensions: BTreeMap::from([
                (
                    "branch".to_string(),
                    vec!["North".to_string(), "South".to_string()],
                ),
                ("department".to_string(), vec!["Sales".to_string()]),
            ]),
            ..filters()
        },
        &[
            AccountingDimension {
                fieldname: "branch".to_string(),
                document_type: "Branch".to_string(),
                is_tree: true,
            },
            AccountingDimension {
                fieldname: "department".to_string(),
                document_type: "Department".to_string(),
                is_tree: false,
            },
            AccountingDimension {
                fieldname: "region".to_string(),
                document_type: "Region".to_string(),
                is_tree: true,
            },
        ],
        &BTreeMap::from([(
            "branch".to_string(),
            vec![
                "North".to_string(),
                "North Child".to_string(),
                "South".to_string(),
            ],
        )]),
    );

    assert_eq!(
        conditions,
        vec![
            "branch IN ('North', 'North Child', 'South')",
            "department IN ('Sales')",
        ]
    );
}

#[test]
fn accounts_receivable_prepare_conditions_plan_matches_receivable_flow_order() {
    let conditions = prepare_conditions_plan(
        AccountType::Receivable,
        &["Customer"],
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            customer_group: Some("Retail".to_string()),
            project: vec!["PROJ-001".to_string()],
            accounting_dimensions: BTreeMap::from([(
                "branch".to_string(),
                vec!["North".to_string()],
            )]),
            ..filters()
        },
        &["Debtors - TC".to_string()],
        &["Retail".to_string()],
        &[],
        &[],
        &[],
        &["Main - TC".to_string()],
        &[AccountingDimension {
            fieldname: "branch".to_string(),
            document_type: "Branch".to_string(),
            is_tree: false,
        }],
        &BTreeMap::new(),
    );

    assert_eq!(
        conditions,
        vec![
            "company = '_Test Company'",
            "account IN ('Debtors - TC')",
            "party IN Customer WHERE customer_group IN ('Retail')",
            "party_type != 'Employee'",
            "project IN ('PROJ-001')",
            "branch IN ('North')",
        ]
    );
}

#[test]
fn accounts_receivable_prepare_conditions_plan_matches_payable_flow_order() {
    let conditions = prepare_conditions_plan(
        AccountType::Payable,
        &["Supplier"],
        &ReceivablePayableFilters {
            company: Some("_Test Company".to_string()),
            supplier_group: Some("Services".to_string()),
            cost_center: vec!["Main - TC".to_string()],
            ..filters()
        },
        &["Creditors - TC".to_string()],
        &[],
        &[],
        &[],
        &[],
        &["Main - TC".to_string(), "Sub - TC".to_string()],
        &[],
        &BTreeMap::new(),
    );

    assert_eq!(
        conditions,
        vec![
            "company = '_Test Company'",
            "account IN ('Creditors - TC')",
            "party IN Supplier WHERE supplier_group = 'Services'",
            "cost_center IN ('Main - TC', 'Sub - TC')",
        ]
    );
}

#[test]
fn accounts_receivable_is_invoice_type_matches_erpnext_sales_and_purchase_only() {
    assert!(is_invoice_type("Sales Invoice"));
    assert!(is_invoice_type("Purchase Invoice"));
    assert!(!is_invoice_type("Payment Entry"));
    assert!(!is_invoice_type("Journal Entry"));
}

#[test]
fn accounts_receivable_allocate_extra_payments_or_credits_builds_additional_row_like_erpnext() {
    let mut row = PaymentTermAllocationRow {
        invoiced: 100.0,
        paid: 15.0,
        credit_note: 25.0,
        payment_terms: Vec::new(),
    };

    let additional = allocate_extra_payments_or_credits(&mut row);

    assert_eq!(
        additional,
        Some(PaymentTermRow {
            due_date: String::new(),
            invoiced: 0.0,
            invoice_grand_total: 100.0,
            payment_term: String::new(),
            paid: 15.0,
            credit_note: 25.0,
            outstanding: -40.0,
        })
    );
}

#[test]
fn accounts_receivable_allocate_extra_payments_or_credits_returns_none_without_extra_amounts() {
    let mut row = PaymentTermAllocationRow {
        invoiced: 100.0,
        paid: 0.0,
        credit_note: 0.0,
        payment_terms: Vec::new(),
    };

    assert_eq!(allocate_extra_payments_or_credits(&mut row), None);
}

#[test]
fn accounts_receivable_build_chart_data_skips_bold_rows_and_rounds_ranges_like_erpnext() {
    let chart = build_chart_data(
        &[
            ChartInputRow {
                bold: false,
                range0: 1.234,
                ranges: vec![2.345, 3.456],
            },
            ChartInputRow {
                bold: true,
                range0: 9.0,
                ranges: vec![9.0, 9.0],
            },
            ChartInputRow {
                bold: false,
                range0: 0.0,
                ranges: vec![4.444],
            },
        ],
        &["<0".to_string(), "0-30".to_string(), "31-Above".to_string()],
        2,
        2,
    );

    assert_eq!(chart.chart_type, "percentage");
    assert_eq!(chart.labels, vec!["<0", "0-30", "31-Above"]);
    assert_eq!(
        chart.datasets,
        vec![vec![1.23, 2.35, 3.46], vec![0.0, 4.44, 0.0]]
    );
}

#[test]
fn accounts_receivable_exchange_rate_revaluations_plan_matches_erpnext_filters() {
    let plan = build_exchange_rate_revaluations_plan(&ReceivablePayableFilters {
        company: Some("_Test Company".to_string()),
        report_date: Some("2026-05-29".to_string()),
        ..filters()
    });

    assert_eq!(plan.doctype, "Journal Entry");
    assert_eq!(plan.selected_field, "name");
    assert_eq!(
        plan.conditions,
        vec![
            "company = '_Test Company'",
            "posting_date <= '2026-05-29'",
            "voucher_type IN ('Exchange Rate Revaluation', 'Exchange Gain Or Loss')",
        ]
    );
}

#[test]
fn accounts_receivable_get_party_group_with_children_matches_split_dedupe_and_party_guard() {
    let children_by_group = BTreeMap::from([
        (
            "Retail".to_string(),
            vec!["Retail".to_string(), "Online".to_string()],
        ),
        (
            "Wholesale".to_string(),
            vec!["Wholesale".to_string(), "Online".to_string()],
        ),
    ]);

    assert_eq!(
        get_party_group_with_children("Customer", " Retail, Wholesale ", &children_by_group)
            .unwrap(),
        vec![
            "Online".to_string(),
            "Retail".to_string(),
            "Wholesale".to_string()
        ]
    );
    assert!(
        get_party_group_with_children("Employee", "Retail", &children_by_group)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn accounts_receivable_get_party_group_with_children_errors_like_erpnext_for_missing_group() {
    let err = get_party_group_with_children("Supplier", "Missing", &BTreeMap::new()).unwrap_err();

    assert_eq!(err, "Supplier Group: Missing does not exist");
}
