use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::deferred_revenue::{
    book_deferred_income_or_expense_plan, book_revenue_via_journal_entry_plan, build_conditions,
    calculate_amount, calculate_monthly_amount, convert_deferred_expense_to_expense_plan,
    convert_deferred_revenue_to_income_plan, get_booking_dates, get_deferred_booking_accounts,
    make_gl_entries_plan, process_deferred_accounting_plan, send_mail_plan,
    validate_service_stop_dates, AlreadyBookedAmounts, BookingBasis, BookingDates,
    DeferredConversionPlan, DeferredDoc, DeferredDocType, DeferredErrorMailPlan, DeferredItem,
    DeferredPostingAction, DeferredPostingSettings, DeferredProcessAccountingDocPlan,
    DeferredProcessType, GlEntryPlan, JournalEntryPlan, ServiceStopDateError,
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
fn deferred_revenue_conversion_query_plans_match_income_and_expense_sql_defaults() {
    let income = convert_deferred_revenue_to_income_plan(
        "PDA-INCOME",
        None,
        None,
        "2026-05-30",
        "AND p.company = '_Test Company'",
        true,
    );

    assert_eq!(
        income,
        DeferredConversionPlan {
            deferred_process: "PDA-INCOME".to_string(),
            invoice_doctype: "Sales Invoice",
            item_table: "tabSales Invoice Item",
            parent_table: "tabSales Invoice",
            enable_field: "enable_deferred_revenue",
            date_params: ("2026-05-29".to_string(), "2026-04-30".to_string()),
            conditions: "AND p.company = '_Test Company'".to_string(),
            query: "select distinct item.parent from `tabSales Invoice Item` item, `tabSales Invoice` p where item.service_start_date<=%s and item.service_end_date>=%s and item.enable_deferred_revenue = 1 and item.parent=p.name and item.docstatus = 1 and ifnull(item.amount, 0) > 0 AND p.company = '_Test Company'".to_string(),
            mail_if_error: Some(send_mail_plan("PDA-INCOME")),
        }
    );

    let expense = convert_deferred_expense_to_expense_plan(
        "PDA-EXPENSE",
        Some("2026-01-01"),
        Some("2026-01-31"),
        "2026-05-30",
        "",
        false,
    );

    assert_eq!(
        expense,
        DeferredConversionPlan {
            deferred_process: "PDA-EXPENSE".to_string(),
            invoice_doctype: "Purchase Invoice",
            item_table: "tabPurchase Invoice Item",
            parent_table: "tabPurchase Invoice",
            enable_field: "enable_deferred_expense",
            date_params: ("2026-01-31".to_string(), "2026-01-01".to_string()),
            conditions: String::new(),
            query: "select distinct item.parent from `tabPurchase Invoice Item` item, `tabPurchase Invoice` p where item.service_start_date<=%s and item.service_end_date>=%s and item.enable_deferred_expense = 1 and item.parent=p.name and item.docstatus = 1 and ifnull(item.amount, 0) > 0".to_string(),
            mail_if_error: None,
        }
    );
}

#[test]
fn deferred_revenue_process_deferred_accounting_plan_matches_monthly_company_docs() {
    assert!(
        process_deferred_accounting_plan(None, "2026-05-30", false, &["_Test Company"]).is_empty()
    );

    assert_eq!(
        process_deferred_accounting_plan(
            Some("2026-05-15"),
            "2026-05-30",
            true,
            &["_Test Company", "Second Co"],
        ),
        vec![
            DeferredProcessAccountingDocPlan {
                company: "_Test Company".to_string(),
                posting_date: "2026-05-15".to_string(),
                start_date: "2026-04-30".to_string(),
                end_date: "2026-05-29".to_string(),
                process_type: DeferredProcessType::Income,
            },
            DeferredProcessAccountingDocPlan {
                company: "_Test Company".to_string(),
                posting_date: "2026-05-15".to_string(),
                start_date: "2026-04-30".to_string(),
                end_date: "2026-05-29".to_string(),
                process_type: DeferredProcessType::Expense,
            },
            DeferredProcessAccountingDocPlan {
                company: "Second Co".to_string(),
                posting_date: "2026-05-15".to_string(),
                start_date: "2026-04-30".to_string(),
                end_date: "2026-05-29".to_string(),
                process_type: DeferredProcessType::Income,
            },
            DeferredProcessAccountingDocPlan {
                company: "Second Co".to_string(),
                posting_date: "2026-05-15".to_string(),
                start_date: "2026-04-30".to_string(),
                end_date: "2026-05-29".to_string(),
                process_type: DeferredProcessType::Expense,
            },
        ]
    );
}

#[test]
fn deferred_revenue_send_mail_plan_matches_erpnext_message_shape() {
    assert_eq!(
        send_mail_plan("PDA-0001"),
        DeferredErrorMailPlan {
            title: "Error while processing deferred accounting for PDA-0001".to_string(),
            doctype: "Process Deferred Accounting".to_string(),
            docname: "PDA-0001".to_string(),
            content: "Deferred accounting failed for some invoices:\nPlease check Process Deferred Accounting Process Deferred Accounting/PDA-0001 and submit manually after resolving errors.".to_string(),
        }
    );
}

#[test]
fn deferred_revenue_book_deferred_income_or_expense_plan_handles_frozen_date_and_recursion() {
    let mut item = deferred_item();
    item.service_start_date = "2026-01-01".to_string();
    item.service_end_date = "2026-03-31".to_string();
    item.base_net_amount = 900.0;
    item.net_amount = 900.0;

    let actions = book_deferred_income_or_expense_plan(
        &sales_doc(),
        "PDA-0001",
        Some("2026-03-31"),
        Some("2026-01-31"),
        &DeferredPostingSettings {
            via_journal_entry: false,
            submit_journal_entry: false,
            booking_basis: BookingBasis::Days,
            account_currency: "USD".to_string(),
            already_booked: AlreadyBookedAmounts::default(),
            already_booked_by_item: BTreeMap::new(),
            latest_existing_posting_dates: BTreeMap::new(),
            deferred_accounting_error: false,
        },
        &[item],
    );

    assert_eq!(actions.len(), 3);
    assert!(matches!(
        &actions[0],
        DeferredPostingAction::GlEntries { posting_date, prev_posting_date, entries }
            if posting_date == "2026-02-28"
                && prev_posting_date.as_deref() == Some("2026-01-31")
                && entries[0].account == "Sales"
                && entries[0].credit == 310.0
    ));
    assert!(matches!(
        &actions[1],
        DeferredPostingAction::GlEntries { posting_date, prev_posting_date, entries }
            if posting_date == "2026-02-28"
                && prev_posting_date.is_none()
                && entries[0].credit == 280.0
    ));
    assert!(matches!(
        &actions[2],
        DeferredPostingAction::GlEntries { posting_date, prev_posting_date, entries }
            if posting_date == "2026-03-31"
                && prev_posting_date.is_none()
                && entries[0].credit == 310.0
    ));
}

#[test]
fn deferred_revenue_book_deferred_income_or_expense_plan_uses_journal_entry_and_stops_on_error() {
    let mut item = deferred_item();
    item.enable_deferred_revenue = false;
    let skipped = book_deferred_income_or_expense_plan(
        &sales_doc(),
        "PDA-0001",
        Some("2026-01-31"),
        None,
        &DeferredPostingSettings {
            via_journal_entry: true,
            submit_journal_entry: true,
            booking_basis: BookingBasis::Months,
            account_currency: "EUR".to_string(),
            already_booked: AlreadyBookedAmounts::default(),
            already_booked_by_item: BTreeMap::new(),
            latest_existing_posting_dates: BTreeMap::new(),
            deferred_accounting_error: false,
        },
        &[item.clone()],
    );
    assert!(skipped.is_empty());

    item.enable_deferred_revenue = true;
    let actions = book_deferred_income_or_expense_plan(
        &sales_doc(),
        "PDA-0001",
        Some("2026-02-28"),
        None,
        &DeferredPostingSettings {
            via_journal_entry: true,
            submit_journal_entry: true,
            booking_basis: BookingBasis::Months,
            account_currency: "EUR".to_string(),
            already_booked: AlreadyBookedAmounts::default(),
            already_booked_by_item: BTreeMap::new(),
            latest_existing_posting_dates: BTreeMap::new(),
            deferred_accounting_error: true,
        },
        &[item],
    );

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        &actions[0],
        DeferredPostingAction::JournalEntry { posting_date, journal_entry, prev_posting_date }
            if posting_date == "2026-01-31"
                && prev_posting_date.is_none()
                && journal_entry.submit
                && journal_entry.voucher_type == "Deferred Revenue"
    ));
}

#[test]
fn deferred_revenue_book_deferred_income_or_expense_plan_uses_item_existing_postings() {
    let mut first_item = deferred_item();
    first_item.service_start_date = "2026-01-01".to_string();
    first_item.service_end_date = "2026-03-31".to_string();
    first_item.base_net_amount = 900.0;
    first_item.net_amount = 900.0;

    let mut second_item = first_item.clone();
    second_item.name = "ITEM-ROW-2".to_string();

    let mut settings = DeferredPostingSettings {
        via_journal_entry: false,
        submit_journal_entry: false,
        booking_basis: BookingBasis::Days,
        account_currency: "USD".to_string(),
        already_booked: AlreadyBookedAmounts::default(),
        already_booked_by_item: BTreeMap::new(),
        latest_existing_posting_dates: BTreeMap::new(),
        deferred_accounting_error: false,
    };
    settings
        .latest_existing_posting_dates
        .insert("ITEM-ROW-1".to_string(), "2026-01-31".to_string());
    settings.already_booked_by_item.insert(
        "ITEM-ROW-1".to_string(),
        AlreadyBookedAmounts {
            base: 310.0,
            account_currency: 310.0,
        },
    );

    let actions = book_deferred_income_or_expense_plan(
        &sales_doc(),
        "PDA-0001",
        Some("2026-03-31"),
        None,
        &settings,
        &[first_item, second_item],
    );

    let credits = actions
        .iter()
        .map(|action| match action {
            DeferredPostingAction::GlEntries { entries, .. } => entries[0].credit,
            DeferredPostingAction::JournalEntry { .. } => unreachable!(),
        })
        .collect::<Vec<_>>();

    assert_eq!(credits, vec![280.0, 310.0, 310.0, 280.0, 310.0]);
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
