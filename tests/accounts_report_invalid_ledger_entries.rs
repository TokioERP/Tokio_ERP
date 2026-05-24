use tokio_erp::erpnext::accounts::report::invalid_ledger_entries::invalid_ledger_entries::{
    build_query_filters, execute, identify_cancelled_vouchers, validate_filters,
    ActiveVoucherQueryPlan, InvalidLedgerEntryFilters, LedgerVoucher, ReportColumn,
};

#[test]
fn invalid_ledger_entries_columns_match_erpnext_report_shape() {
    assert_eq!(
        execute(
            InvalidLedgerEntryFilters::new("_Test Company", "2026-05-01", "2026-05-31"),
            vec![],
            vec![],
        )
        .unwrap()
        .columns,
        vec![
            ReportColumn::link("Voucher Type", "voucher_type", "DocType"),
            ReportColumn::dynamic_link("Voucher No", "voucher_no", "voucher_type"),
        ]
    );
}

#[test]
fn invalid_ledger_entries_validates_required_filters_like_erpnext() {
    assert_eq!(validate_filters(None).unwrap_err(), "Filters missing");

    let missing_company = InvalidLedgerEntryFilters {
        company: None,
        from_date: "2026-05-01".to_string(),
        to_date: "2026-05-31".to_string(),
        account: vec![],
        voucher_no: None,
    };
    assert_eq!(
        validate_filters(Some(&missing_company)).unwrap_err(),
        "Company is mandatory"
    );

    let reversed_dates =
        InvalidLedgerEntryFilters::new("_Test Company", "2026-06-01", "2026-05-31");
    assert_eq!(
        validate_filters(Some(&reversed_dates)).unwrap_err(),
        "Start Date should be lower than End Date"
    );
}

#[test]
fn invalid_ledger_entries_optional_query_filters_match_erpnext() {
    let mut filters = InvalidLedgerEntryFilters::new("_Test Company", "2026-05-01", "2026-05-31");
    filters.account = vec!["Cash - TC".to_string(), "Bank - TC".to_string()];
    filters.voucher_no = Some("SINV-0001".to_string());

    assert_eq!(
        build_query_filters(&filters),
        vec![
            "account in ['Cash - TC', 'Bank - TC']".to_string(),
            "voucher_no = SINV-0001".to_string(),
        ]
    );
}

#[test]
fn invalid_ledger_entries_active_voucher_query_plan_matches_gl_and_payment_ledgers() {
    let filters = InvalidLedgerEntryFilters::new("_Test Company", "2026-05-01", "2026-05-31");

    assert_eq!(
        ActiveVoucherQueryPlan::from_filters(&filters),
        vec![
            ActiveVoucherQueryPlan {
                source_doctype: "GL Entry",
                cancelled_field: "is_cancelled",
                company: "_Test Company".to_string(),
                from_date: "2026-05-01".to_string(),
                to_date: "2026-05-31".to_string(),
                extra_filters: vec![],
            },
            ActiveVoucherQueryPlan {
                source_doctype: "Payment Ledger Entry",
                cancelled_field: "delinked",
                company: "_Test Company".to_string(),
                from_date: "2026-05-01".to_string(),
                to_date: "2026-05-31".to_string(),
                extra_filters: vec![],
            },
        ]
    );
}

#[test]
fn invalid_ledger_entries_identifies_non_active_vouchers_by_type() {
    let active = vec![
        LedgerVoucher::new("Sales Invoice", "SINV-0001"),
        LedgerVoucher::new("Sales Invoice", "SINV-0002"),
        LedgerVoucher::new("Payment Entry", "PAY-0001"),
        LedgerVoucher::new("Sales Invoice", "SINV-0001"),
    ];
    let non_active = vec![
        LedgerVoucher::new("Sales Invoice", "SINV-0002"),
        LedgerVoucher::new("Purchase Invoice", "PINV-0001"),
        LedgerVoucher::new("Payment Entry", "PAY-0001"),
    ];

    assert_eq!(
        identify_cancelled_vouchers(&active, &non_active),
        vec![
            LedgerVoucher::new("Payment Entry", "PAY-0001"),
            LedgerVoucher::new("Sales Invoice", "SINV-0002"),
        ]
    );
}

#[test]
fn invalid_ledger_entries_execute_returns_cancelled_voucher_rows() {
    let filters = InvalidLedgerEntryFilters::new("_Test Company", "2026-05-01", "2026-05-31");
    let active = vec![
        LedgerVoucher::new("Journal Entry", "JV-0001"),
        LedgerVoucher::new("Sales Invoice", "SINV-0001"),
    ];
    let non_active = vec![
        LedgerVoucher::new("Sales Invoice", "SINV-0001"),
        LedgerVoucher::new("Purchase Invoice", "PINV-0001"),
    ];

    let report = execute(filters, active, non_active).unwrap();

    assert_eq!(
        report.rows,
        vec![LedgerVoucher::new("Sales Invoice", "SINV-0001")]
    );
}
