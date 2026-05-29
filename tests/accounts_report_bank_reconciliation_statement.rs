use tokio_erp::erpnext::accounts::report::bank_reconciliation_statement::bank_reconciliation_statement::{
    execute, execute_with_uncleared_entries, get_amounts_not_reflected_in_system_from_entries,
    get_balance_row, get_columns, BankReconciliationEntry, BankReconciliationFilters,
    BankReconciliationQueryPlan, BankReconciliationRow, BankReconciliationSource,
    IncorrectlyClearedJournalEntry, IncorrectlyClearedPaymentEntry,
    IncorrectlyClearedPurchaseInvoice, ReportColumn,
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

#[test]
fn bank_reconciliation_statement_source_helpers_match_erpnext_select_formulas() {
    let inbound_payment = BankReconciliationEntry::payment_entry(
        "PAY-IN",
        "2026-01-10",
        "Bank - TC",
        "Cash - TC",
        Some("_Test Customer"),
        250.0,
        999.0,
        "USD",
        "INR",
        Some("REF-IN"),
        Some("2026-01-09"),
        None,
        "Cash - TC",
    );
    assert_eq!(inbound_payment.debit, 250.0);
    assert_eq!(inbound_payment.credit, 0.0);
    assert_eq!(inbound_payment.against_account, "_Test Customer");
    assert_eq!(inbound_payment.account_currency, "USD");

    let outbound_payment = BankReconciliationEntry::payment_entry(
        "PAY-OUT",
        "2026-01-11",
        "Cash - TC",
        "Creditors - TC",
        None,
        999.0,
        175.0,
        "INR",
        "USD",
        None,
        None,
        None,
        "Cash - TC",
    );
    assert_eq!(outbound_payment.debit, 0.0);
    assert_eq!(outbound_payment.credit, 175.0);
    assert_eq!(outbound_payment.against_account, "Creditors - TC");
    assert_eq!(outbound_payment.account_currency, "USD");

    let purchase_invoice_credit = BankReconciliationEntry::purchase_invoice(
        "PINV-CREDIT",
        Some("BILL-1"),
        "2026-01-12",
        80.0,
        "_Test Supplier",
        None,
        "USD",
    );
    assert_eq!(purchase_invoice_credit.debit, 0.0);
    assert_eq!(purchase_invoice_credit.credit, 80.0);
    assert_eq!(
        purchase_invoice_credit.ref_date.as_deref(),
        Some("2026-01-12")
    );

    let purchase_invoice_debit = BankReconciliationEntry::purchase_invoice(
        "PINV-DEBIT",
        None,
        "2026-01-13",
        -45.0,
        "_Test Supplier",
        None,
        "USD",
    );
    assert_eq!(purchase_invoice_debit.debit, 45.0);
    assert_eq!(purchase_invoice_debit.credit, 0.0);
    assert_eq!(purchase_invoice_debit.reference_no, None);

    let pos_invoice = BankReconciliationEntry::pos_sales_invoice(
        "SINV-POS",
        "2026-01-14",
        60.0,
        "Debtors - TC",
        Some("2026-02-01"),
        "USD",
    );
    assert_eq!(pos_invoice.debit, 60.0);
    assert_eq!(pos_invoice.credit, 0.0);
    assert_eq!(pos_invoice.source, BankReconciliationSource::SalesInvoice);
}

#[test]
fn bank_reconciliation_statement_computes_incorrectly_cleared_amount_from_erpnext_sources() {
    let amount = get_amounts_not_reflected_in_system_from_entries(
        &[
            IncorrectlyClearedJournalEntry::new(120.0, 20.0),
            IncorrectlyClearedJournalEntry::new(0.0, 10.0),
        ],
        &[
            IncorrectlyClearedPaymentEntry::new(true, 200.0, 999.0),
            IncorrectlyClearedPaymentEntry::new(false, 999.0, 75.0),
        ],
        &[
            IncorrectlyClearedPurchaseInvoice::new(-30.0),
            IncorrectlyClearedPurchaseInvoice::new(50.0),
        ],
    );

    assert_eq!(amount, 385.0);
}

#[test]
fn bank_reconciliation_statement_execute_can_derive_incorrectly_cleared_summary_amount() {
    let report = execute_with_uncleared_entries(
        &filters(),
        vec![BankReconciliationEntry::journal_entry(
            "JV-0001",
            "2026-01-10",
            100.0,
            0.0,
            "Debtors - TC",
            Some("CHK-1"),
            Some("2026-01-09"),
            None,
            "USD",
        )],
        &[IncorrectlyClearedJournalEntry::new(75.0, 5.0)],
        &[IncorrectlyClearedPaymentEntry::new(false, 999.0, 30.0)],
        &[IncorrectlyClearedPurchaseInvoice::new(20.0)],
    );

    assert_eq!(
        report.rows[3],
        BankReconciliationRow::outstanding(100.0, 0.0, "USD")
    );
    assert_eq!(
        report.rows[4],
        BankReconciliationRow::balance("Cheques and Deposits incorrectly cleared", 120.0, "USD")
    );
    assert_eq!(
        report.rows[6],
        BankReconciliationRow::balance("Calculated Bank Statement balance", 1_020.0, "USD")
    );
}
