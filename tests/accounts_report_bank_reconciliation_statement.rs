use tokio_erp::erpnext::accounts::report::bank_reconciliation_statement::bank_reconciliation_statement::{
    execute, get_balance_row, get_columns, BankReconciliationEntry, BankReconciliationFilters,
    BankReconciliationQueryPlan, BankReconciliationRow, BankReconciliationSource, ReportColumn,
};

fn filters() -> BankReconciliationFilters {
    BankReconciliationFilters {
        company: "_Test Company".to_string(),
        account: Some("Cash - TC".to_string()),
        report_date: "2026-01-31".to_string(),
        account_currency: "USD".to_string(),
        balance_as_per_system: 1_000.0,
        include_pos_transactions: false,
    }
}

#[test]
fn bank_reconciliation_statement_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::date("Posting Date", "posting_date", 90),
            ReportColumn::data("Payment Document Type", "payment_document", 220),
            ReportColumn::dynamic_link(
                "Payment Document",
                "payment_entry",
                "payment_document",
                220,
            ),
            ReportColumn::currency("Debit", "debit", "account_currency", 120),
            ReportColumn::currency("Credit", "credit", "account_currency", 120),
            ReportColumn::link("Against Account", "against_account", "Account", 200),
            ReportColumn::data("Reference", "reference_no", 100),
            ReportColumn::date("Ref Date", "ref_date", 110),
            ReportColumn::date("Clearance Date", "clearance_date", 110),
            ReportColumn::link("Currency", "account_currency", "Currency", 100),
        ]
    );
}

#[test]
fn bank_reconciliation_statement_query_plan_matches_erpnext_sources_and_filters() {
    assert_eq!(
        BankReconciliationQueryPlan::for_filters(&filters()),
        vec![
            BankReconciliationQueryPlan {
                source: BankReconciliationSource::JournalEntry,
                base_filters: vec![
                    "docstatus = 1",
                    "account = filters.account",
                    "posting_date <= filters.report_date",
                    "clearance_date is null or clearance_date > filters.report_date",
                    "company = filters.company",
                    "is_opening is null or is_opening = 'No'",
                ],
                ordering: vec!["posting_date", "name desc"],
            },
            BankReconciliationQueryPlan {
                source: BankReconciliationSource::PaymentEntry,
                base_filters: vec![
                    "docstatus = 1",
                    "paid_from = filters.account or paid_to = filters.account",
                    "posting_date <= filters.report_date",
                    "clearance_date is null or clearance_date > filters.report_date",
                    "company = filters.company",
                ],
                ordering: vec!["posting_date", "name desc"],
            },
            BankReconciliationQueryPlan {
                source: BankReconciliationSource::PurchaseInvoice,
                base_filters: vec![
                    "docstatus = 1",
                    "is_paid = 1",
                    "cash_bank_account = filters.account",
                    "posting_date <= filters.report_date",
                    "clearance_date is null or clearance_date > filters.report_date",
                    "company = filters.company",
                ],
                ordering: vec!["posting_date", "name desc"],
            },
        ]
    );

    let mut pos_filters = filters();
    pos_filters.include_pos_transactions = true;
    assert_eq!(
        BankReconciliationQueryPlan::for_filters(&pos_filters)
            .last()
            .unwrap()
            .source,
        BankReconciliationSource::SalesInvoice
    );
}

#[test]
fn bank_reconciliation_statement_returns_columns_and_empty_data_without_account() {
    let mut report_filters = filters();
    report_filters.account = None;

    let report = execute(&report_filters, Vec::new(), 0.0);

    assert_eq!(report.columns, get_columns());
    assert!(report.rows.is_empty());
}

#[test]
fn bank_reconciliation_statement_sorts_uncleared_entries_and_appends_summary_rows() {
    let report = execute(
        &filters(),
        vec![
            BankReconciliationEntry::new(
                BankReconciliationSource::PaymentEntry,
                "PAY-0002",
                "2026-01-20",
                0.0,
                30.0,
                "Customer A",
                Some("CHK-2"),
                Some("2026-01-20"),
                None,
                "USD",
            ),
            BankReconciliationEntry::new(
                BankReconciliationSource::JournalEntry,
                "JV-0001",
                "2026-01-10",
                100.0,
                0.0,
                "Debtors - TC",
                Some("CHK-1"),
                Some("2026-01-09"),
                None,
                "USD",
            ),
        ],
        25.0,
    );

    assert_eq!(report.rows[0].payment_entry.as_deref(), Some("JV-0001"));
    assert_eq!(report.rows[1].payment_entry.as_deref(), Some("PAY-0002"));
    assert_eq!(
        report.rows[2],
        BankReconciliationRow::balance(
            "Bank Statement balance as per General Ledger",
            1_000.0,
            "USD"
        )
    );
    assert!(report.rows[3].is_blank());
    assert_eq!(
        report.rows[4],
        BankReconciliationRow::outstanding(100.0, 30.0, "USD")
    );
    assert_eq!(
        report.rows[5],
        BankReconciliationRow::balance("Cheques and Deposits incorrectly cleared", 25.0, "USD")
    );
    assert!(report.rows[6].is_blank());
    assert_eq!(
        report.rows[7],
        BankReconciliationRow::balance("Calculated Bank Statement balance", 955.0, "USD")
    );
}

#[test]
fn bank_reconciliation_statement_preserves_input_order_for_same_posting_date_like_python_sort() {
    let report = execute(
        &filters(),
        vec![
            BankReconciliationEntry::new(
                BankReconciliationSource::PaymentEntry,
                "PAY-0002",
                "2026-01-20",
                0.0,
                30.0,
                "Customer A",
                None,
                None,
                None,
                "USD",
            ),
            BankReconciliationEntry::new(
                BankReconciliationSource::PaymentEntry,
                "PAY-0001",
                "2026-01-20",
                0.0,
                20.0,
                "Customer B",
                None,
                None,
                None,
                "USD",
            ),
        ],
        0.0,
    );

    assert_eq!(report.rows[0].payment_entry.as_deref(), Some("PAY-0002"));
    assert_eq!(report.rows[1].payment_entry.as_deref(), Some("PAY-0001"));
}

#[test]
fn bank_reconciliation_statement_balance_row_places_negative_amount_in_credit() {
    assert_eq!(
        get_balance_row("Calculated Bank Statement balance", -42.0, "USD"),
        BankReconciliationRow::balance("Calculated Bank Statement balance", -42.0, "USD")
    );
    let row = get_balance_row("Calculated Bank Statement balance", -42.0, "USD");
    assert_eq!(row.debit, Some(0.0));
    assert_eq!(row.credit, Some(42.0));
}
