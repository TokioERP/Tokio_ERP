use tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::{
    accounts_receivable_args, get_columns, get_currency_fields, AccountType,
    ReceivablePayableFilters, ReceivablePayableRuntime, ReceivablePayableSettings,
    ReceivablePayableState, ReportColumn,
};

fn filters() -> ReceivablePayableFilters {
    ReceivablePayableFilters {
        company: None,
        report_date: None,
        calculate_ageing_with: None,
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

#[test]
fn accounts_receivable_execute_args_match_erpnext_receivable_defaults() {
    let args = accounts_receivable_args();

    assert_eq!(args.account_type, AccountType::Receivable);
    assert_eq!(
        args.naming_by,
        ["Selling Settings".to_string(), "cust_master_name".to_string()]
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
            ReportColumn::link("Customer Contact", "customer_primary_contact", "Contact", 120),
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
