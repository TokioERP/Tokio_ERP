use tokio_erp::erpnext::accounts::report::cheques_and_deposits_incorrectly_cleared::cheques_and_deposits_incorrectly_cleared::{
    build_data, build_journal_entry_dict, build_payment_entry_dict, execute, get_columns,
    ChequeClearanceFilters, ChequeClearanceQueryPlan, ChequeVoucher, ReportColumn,
};

#[test]
fn cheques_incorrectly_cleared_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::data("Payment Document Type", "payment_document", 220),
            ReportColumn::dynamic_link(
                "Payment Document",
                "payment_entry",
                "payment_document",
                220
            ),
            ReportColumn::currency("Debit", "debit", "account_currency", 120),
            ReportColumn::currency("Credit", "credit", "account_currency", 120),
            ReportColumn::date("Posting Date", "posting_date", 110),
            ReportColumn::date("Clearance Date", "clearance_date", 110),
        ]
    );
}

#[test]
fn cheques_incorrectly_cleared_payment_rows_match_erpnext_debit_credit_branching() {
    let receive = ChequeVoucher::payment_entry(
        "PAY-0001",
        150.0,
        "Receive",
        Some("Customer"),
        "2026-05-20",
        "2026-05-01",
    );
    assert_eq!(
        build_payment_entry_dict(&receive),
        ChequeVoucher::report_row(
            "Payment Entry",
            "PAY-0001",
            "2026-05-20",
            "2026-05-01",
            150.0,
            0.0
        )
    );

    let pay = ChequeVoucher::payment_entry(
        "PAY-0002",
        90.0,
        "Pay",
        Some("Supplier"),
        "2026-05-20",
        "2026-05-01",
    );
    assert_eq!(
        build_payment_entry_dict(&pay),
        ChequeVoucher::report_row(
            "Payment Entry",
            "PAY-0002",
            "2026-05-20",
            "2026-05-01",
            0.0,
            90.0
        )
    );

    let receive_employee = ChequeVoucher::payment_entry(
        "PAY-0003",
        80.0,
        "Receive",
        Some("Employee"),
        "2026-05-20",
        "2026-05-01",
    );
    assert_eq!(
        build_payment_entry_dict(&receive_employee).credit,
        Some(80.0)
    );
}

#[test]
fn cheques_incorrectly_cleared_journal_rows_preserve_debit_credit() {
    let journal = ChequeVoucher::journal_entry("JV-0001", 200.0, 50.0, "2026-05-20", "2026-05-01");

    assert_eq!(
        build_journal_entry_dict(&journal),
        ChequeVoucher::report_row(
            "Journal Entry",
            "JV-0001",
            "2026-05-20",
            "2026-05-01",
            200.0,
            50.0
        )
    );
}

#[test]
fn cheques_incorrectly_cleared_build_data_keeps_payment_and_journal_rows_only() {
    let vouchers = vec![
        ChequeVoucher::payment_entry(
            "PAY-0001",
            150.0,
            "Receive",
            Some("Customer"),
            "2026-05-20",
            "2026-05-01",
        ),
        ChequeVoucher::journal_entry("JV-0001", 200.0, 50.0, "2026-05-20", "2026-05-01"),
        ChequeVoucher::unknown("Sales Invoice", "SINV-0001"),
    ];

    let rows = build_data(vouchers);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].payment_document.as_deref(), Some("Payment Entry"));
    assert_eq!(rows[1].payment_document.as_deref(), Some("Journal Entry"));
}

#[test]
fn cheques_incorrectly_cleared_query_plan_matches_erpnext_filters() {
    let filters = ChequeClearanceFilters::new("Cash - TC", "2026-05-01");

    assert_eq!(
        ChequeClearanceQueryPlan::from_filters(&filters),
        vec![
            ChequeClearanceQueryPlan {
                source_doctype: "Journal Entry",
                child_doctype: Some("Journal Entry Account"),
                account_filter: "Journal Entry Account.account = filters.account",
                posting_date_filter: "Journal Entry.posting_date > filters.report_date",
                clearance_date_filter: "Journal Entry.clearance_date <= filters.report_date",
                docstatus_filter: "Journal Entry.docstatus = 1",
                extra_filter: "(Journal Entry.is_opening is null or Journal Entry.is_opening = 'No')",
            },
            ChequeClearanceQueryPlan {
                source_doctype: "Payment Entry",
                child_doctype: None,
                account_filter: "Payment Entry.paid_from = filters.account or Payment Entry.paid_to = filters.account",
                posting_date_filter: "Payment Entry.posting_date > filters.report_date",
                clearance_date_filter: "Payment Entry.clearance_date <= filters.report_date",
                docstatus_filter: "Payment Entry.docstatus = 1",
                extra_filter: "",
            },
        ]
    );
}

#[test]
fn cheques_incorrectly_cleared_execute_returns_columns_and_built_rows() {
    let report = execute(vec![
        ChequeVoucher::payment_entry(
            "PAY-0001",
            150.0,
            "Receive",
            Some("Customer"),
            "2026-05-20",
            "2026-05-01",
        ),
        ChequeVoucher::journal_entry("JV-0001", 200.0, 50.0, "2026-05-20", "2026-05-01"),
    ]);

    assert_eq!(report.columns, get_columns());
    assert_eq!(report.rows.len(), 2);
}
