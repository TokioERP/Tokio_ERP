use tokio_erp::erpnext::accounts::report::bank_clearance_summary::bank_clearance_summary::{
    execute, get_columns, BankClearanceEntry, BankClearanceFilters, BankClearanceQueryPlan,
    BankClearanceSource, ReportColumn,
};

#[test]
fn bank_clearance_summary_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::data("Payment Document Type", "payment_document_type", 130),
            ReportColumn::dynamic_link(
                "Payment Entry",
                "payment_entry",
                "payment_document_type",
                140,
            ),
            ReportColumn::date("Posting Date", "posting_date", 120),
            ReportColumn::plain("Cheque/Reference No", "cheque_no", 120),
            ReportColumn::date("Clearance Date", "clearance_date", 120),
            ReportColumn::link("Against Account", "against", "Account", 200),
            ReportColumn::currency("Amount", "amount", 120),
        ]
    );
}

#[test]
fn bank_clearance_summary_sorts_hook_entries_by_posting_date() {
    let filters = BankClearanceFilters::new("Cash - _TC", "2026-05-01", "2026-05-31");
    let entries = vec![
        BankClearanceEntry::new(
            "Payment Entry",
            "PAY-0002",
            "2026-05-12",
            "CHK-2",
            None,
            "Customer A",
            250.0,
        ),
        BankClearanceEntry::new(
            "Journal Entry",
            "JV-0001",
            "2026-05-02",
            "CHK-1",
            Some("2026-05-03"),
            "Debtors - _TC",
            -100.0,
        ),
        BankClearanceEntry::new(
            "Purchase Invoice",
            "PINV-0001",
            "2026-05-05",
            "BILL-1",
            None,
            "Supplier A",
            -75.0,
        ),
    ];

    let report = execute(filters, vec![entries]);

    assert_eq!(report.columns, get_columns());
    assert_eq!(
        report
            .rows
            .iter()
            .map(|row| row.payment_entry.as_str())
            .collect::<Vec<_>>(),
        vec!["JV-0001", "PINV-0001", "PAY-0002"]
    );
}

#[test]
fn bank_clearance_summary_query_plan_matches_erpnext_sources_and_filters() {
    let filters = BankClearanceFilters::new("Cash - _TC", "2026-05-01", "2026-05-31");

    assert_eq!(
        BankClearanceQueryPlan::from_filters(&filters),
        vec![
            BankClearanceQueryPlan {
                source: BankClearanceSource::JournalEntry,
                account_filter: "Journal Entry Account.account = filters.account",
                date_filter: "Journal Entry.posting_date between filters.from_date and filters.to_date",
                docstatus_filter: "Journal Entry.docstatus = 1",
                extra_filter: "(Journal Entry.is_opening = 'No' or Journal Entry.is_opening is null)",
                ordering: vec![
                    "Journal Entry.posting_date desc",
                    "Journal Entry.name desc",
                ],
                amount_formula: "debit_in_account_currency - credit_in_account_currency",
            },
            BankClearanceQueryPlan {
                source: BankClearanceSource::PaymentEntry,
                account_filter: "Payment Entry.paid_from = filters.account or Payment Entry.paid_to = filters.account",
                date_filter: "Payment Entry.posting_date between filters.from_date and filters.to_date",
                docstatus_filter: "Payment Entry.docstatus = 1",
                extra_filter: "",
                ordering: vec![
                    "Payment Entry.posting_date desc",
                    "Payment Entry.name desc",
                ],
                amount_formula: "if paid_from == filters.account then (paid_amount * -1) - total_taxes_and_charges else received_amount",
            },
            BankClearanceQueryPlan {
                source: BankClearanceSource::PurchaseInvoice,
                account_filter: "Purchase Invoice.cash_bank_account = filters.account",
                date_filter: "Purchase Invoice.posting_date between filters.from_date and filters.to_date",
                docstatus_filter: "Purchase Invoice.docstatus = 1",
                extra_filter: "Purchase Invoice.is_paid = 1",
                ordering: vec![
                    "Purchase Invoice.posting_date desc",
                    "Purchase Invoice.name desc",
                ],
                amount_formula: "paid_amount * -1",
            },
        ]
    );
}

#[test]
fn bank_clearance_summary_amount_helpers_match_erpnext_cases() {
    assert_eq!(
        BankClearanceSource::JournalEntry.amount_from(200.0, 50.0, 0.0, 0.0, false),
        150.0
    );
    assert_eq!(
        BankClearanceSource::PaymentEntry.amount_from(0.0, 0.0, 300.0, 25.0, true),
        -325.0
    );
    assert_eq!(
        BankClearanceSource::PaymentEntry.amount_from(0.0, 0.0, 175.0, 25.0, false),
        175.0
    );
    assert_eq!(
        BankClearanceSource::PurchaseInvoice.amount_from(0.0, 0.0, 80.0, 0.0, false),
        -80.0
    );
}
