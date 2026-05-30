use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::deferred_revenue::{
    book_revenue_via_journal_entry_plan, build_conditions, calculate_amount,
    calculate_monthly_amount, get_booking_dates, get_deferred_booking_accounts,
    make_gl_entries_plan, validate_service_stop_dates, AlreadyBookedAmounts, BookingDates,
    DeferredDoc, DeferredDocType, DeferredItem, DeferredProcessType, GlEntryPlan, JournalEntryPlan,
    ServiceStopDateError,
};

fn sales_doc() -> DeferredDoc {
    DeferredDoc {
        doctype: DeferredDocType::SalesInvoice,
        name: "SINV-0001".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        currency: "EUR".to_string(),
        party: "CUST-001".to_string(),
        project: Some("PROJ-SALES".to_string()),
        docstatus: 1,
    }
}

fn purchase_doc() -> DeferredDoc {
    DeferredDoc {
        doctype: DeferredDocType::PurchaseInvoice,
        name: "PINV-0001".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        currency: "USD".to_string(),
        party: "SUPP-001".to_string(),
        project: None,
        docstatus: 1,
    }
}

fn deferred_item() -> DeferredItem {
    DeferredItem {
        name: "ITEM-ROW-1".to_string(),
        idx: 1,
        service_start_date: "2026-01-10".to_string(),
        service_end_date: "2026-03-20".to_string(),
        service_stop_date: None,
        enable_deferred_revenue: true,
        enable_deferred_expense: true,
        deferred_revenue_account: Some("Deferred Income".to_string()),
        deferred_expense_account: Some("Deferred Expense".to_string()),
        income_account: Some("Sales".to_string()),
        expense_account: Some("Expense".to_string()),
        net_amount: 1200.0,
        base_net_amount: 2400.0,
        net_amount_precision: 2,
        base_net_amount_precision: 2,
        project: Some("PROJ-ITEM".to_string()),
        cost_center: Some("Main - TC".to_string()),
        accounting_dimensions: BTreeMap::from([("department".to_string(), "Sales".to_string())]),
    }
}

fn assert_pair_close(actual: (f64, f64), expected: (f64, f64)) {
    assert!((actual.0 - expected.0).abs() < 0.000_001, "{actual:?}");
    assert!((actual.1 - expected.1).abs() < 0.000_001, "{actual:?}");
}

#[test]
fn deferred_revenue_validate_service_stop_dates_matches_erpnext_guards() {
    let mut item = deferred_item();
    item.service_stop_date = Some("2026-01-09".to_string());
    assert_eq!(
        validate_service_stop_dates(
            DeferredDocType::SalesInvoice,
            "SINV-0001",
            &[item.clone()],
            &BTreeMap::new()
        ),
        Err(ServiceStopDateError::BeforeServiceStart { idx: 1 })
    );

    item.service_stop_date = Some("2026-03-21".to_string());
    assert_eq!(
        validate_service_stop_dates(
            DeferredDocType::SalesInvoice,
            "SINV-0001",
            &[item.clone()],
            &BTreeMap::new()
        ),
        Err(ServiceStopDateError::AfterServiceEnd { idx: 1 })
    );

    item.service_stop_date = Some("2026-02-01".to_string());
    assert_eq!(
        validate_service_stop_dates(
            DeferredDocType::SalesInvoice,
            "SINV-0001",
            &[item],
            &BTreeMap::from([("ITEM-ROW-1".to_string(), "2026-01-31".to_string())])
        ),
        Err(ServiceStopDateError::ChangedExistingStopDate { idx: 1 })
    );
}

#[test]
fn deferred_revenue_build_conditions_matches_income_expense_account_company_branching() {
    assert_eq!(
        build_conditions(
            DeferredProcessType::Income,
            Some("Deferred Income"),
            Some("_Test Company")
        ),
        "AND item.deferred_revenue_account='Deferred Income'"
    );
    assert_eq!(
        build_conditions(DeferredProcessType::Expense, None, Some("_Test Company")),
        "AND p.company = '_Test Company'"
    );
    assert_eq!(
        build_conditions(DeferredProcessType::Income, None, None),
        ""
    );
}

#[test]
fn deferred_revenue_get_booking_dates_matches_prev_gl_stop_and_posting_date_rules() {
    let mut item = deferred_item();
    item.service_stop_date = Some("2026-02-20".to_string());

    assert_eq!(
        get_booking_dates(
            &sales_doc(),
            &item,
            Some("2026-02-28"),
            Some("2026-01-31"),
            None,
        ),
        Some(BookingDates {
            start_date: "2026-02-01".to_string(),
            end_date: "2026-02-20".to_string(),
            last_gl_entry: true,
        })
    );

    assert_eq!(
        get_booking_dates(
            &sales_doc(),
            &item,
            Some("2026-01-15"),
            None,
            Some("2026-01-12"),
        ),
        Some(BookingDates {
            start_date: "2026-01-13".to_string(),
            end_date: "2026-01-15".to_string(),
            last_gl_entry: false,
        })
    );
}

#[test]
fn deferred_revenue_amount_formulas_match_daily_and_monthly_branches() {
    let doc = sales_doc();
    let item = deferred_item();

    assert_eq!(
        calculate_amount(
            &doc,
            &item,
            false,
            70,
            20,
            "EUR",
            AlreadyBookedAmounts::default(),
        ),
        (342.86, 685.71)
    );
    assert_eq!(
        calculate_amount(
            &doc,
            &item,
            true,
            70,
            20,
            "EUR",
            AlreadyBookedAmounts {
                base: 1800.0,
                account_currency: 900.0,
            },
        ),
        (300.0, 600.0)
    );

    assert_pair_close(
        calculate_monthly_amount(
            &doc,
            &item,
            false,
            "2026-01-10",
            "2026-01-31",
            "EUR",
            AlreadyBookedAmounts::default(),
        ),
        (365.218, 730.436),
    );
}

#[test]
fn deferred_revenue_gl_and_journal_entry_plans_match_erpnext_shape() {
    let item = deferred_item();
    let gl_entries = make_gl_entries_plan(
        &purchase_doc(),
        "Deferred Expense",
        "Expense",
        "SUPP-001",
        300.0,
        300.0,
        "2026-01-31",
        Some("PROJ-ITEM"),
        "USD",
        Some("Main - TC"),
        &item,
        Some("PDA-0001"),
    )
    .unwrap();

    assert_eq!(gl_entries.len(), 2);
    assert_eq!(gl_entries[0].account, "Deferred Expense");
    assert_eq!(gl_entries[0].credit, 300.0);
    assert_eq!(gl_entries[0].against_voucher.as_deref(), Some("PDA-0001"));
    assert_eq!(gl_entries[1].account, "Expense");
    assert_eq!(gl_entries[1].debit, 300.0);

    let journal = book_revenue_via_journal_entry_plan(
        &sales_doc(),
        "Sales",
        "Deferred Income",
        400.0,
        800.0,
        "2026-01-31",
        Some("PROJ-SALES"),
        "EUR",
        Some("Main - TC"),
        &item,
        Some("PDA-0002"),
        true,
    )
    .unwrap();

    assert_eq!(
        journal,
        JournalEntryPlan {
            posting_date: "2026-01-31".to_string(),
            company: "_Test Company".to_string(),
            voucher_type: "Deferred Revenue".to_string(),
            process_deferred_accounting: Some("PDA-0002".to_string()),
            submit: true,
            entries: vec![
                GlEntryPlan {
                    account: "Sales".to_string(),
                    against: None,
                    credit: 800.0,
                    credit_in_account_currency: 400.0,
                    debit: 0.0,
                    debit_in_account_currency: 0.0,
                    account_currency: "EUR".to_string(),
                    voucher_detail_no: Some("ITEM-ROW-1".to_string()),
                    posting_date: "2026-01-31".to_string(),
                    project: Some("PROJ-SALES".to_string()),
                    cost_center: Some("Main - TC".to_string()),
                    against_voucher_type: None,
                    against_voucher: None,
                    reference_name: Some("SINV-0001".to_string()),
                    reference_type: Some("Sales Invoice".to_string()),
                    reference_detail_no: Some("ITEM-ROW-1".to_string()),
                    accounting_dimensions: BTreeMap::from([(
                        "department".to_string(),
                        "Sales".to_string()
                    )]),
                },
                GlEntryPlan {
                    account: "Deferred Income".to_string(),
                    against: None,
                    credit: 0.0,
                    credit_in_account_currency: 0.0,
                    debit: 800.0,
                    debit_in_account_currency: 400.0,
                    account_currency: "EUR".to_string(),
                    voucher_detail_no: Some("ITEM-ROW-1".to_string()),
                    posting_date: "2026-01-31".to_string(),
                    project: Some("PROJ-SALES".to_string()),
                    cost_center: Some("Main - TC".to_string()),
                    against_voucher_type: None,
                    against_voucher: None,
                    reference_name: Some("SINV-0001".to_string()),
                    reference_type: Some("Sales Invoice".to_string()),
                    reference_detail_no: Some("ITEM-ROW-1".to_string()),
                    accounting_dimensions: BTreeMap::from([(
                        "department".to_string(),
                        "Sales".to_string()
                    )]),
                },
            ],
        }
    );
}

#[test]
fn deferred_revenue_booking_accounts_match_sales_purchase_and_dr_cr() {
    assert_eq!(
        get_deferred_booking_accounts(
            DeferredDocType::SalesInvoice,
            "Debit",
            "Sales",
            "Deferred Income"
        ),
        "Deferred Income"
    );
    assert_eq!(
        get_deferred_booking_accounts(
            DeferredDocType::PurchaseInvoice,
            "Credit",
            "Expense",
            "Deferred Expense"
        ),
        "Deferred Expense"
    );
}
