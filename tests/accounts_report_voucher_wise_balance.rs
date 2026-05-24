use tokio_erp::erpnext::accounts::report::voucher_wise_balance::voucher_wise_balance::{
    apply_filters, execute, get_columns, get_data, GlEntry, VoucherBalanceFilters,
    VoucherBalanceQueryPlan, VoucherBalanceRow,
};

#[test]
fn voucher_wise_balance_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ("Voucher Type", "voucher_type", "", "", 300),
            (
                "Voucher No",
                "voucher_no",
                "Dynamic Link",
                "voucher_type",
                300
            ),
            ("Debit", "debit", "Currency", "currency", 300),
            ("Credit", "credit", "Currency", "currency", 300),
        ]
    );
}

#[test]
fn voucher_wise_balance_query_plan_matches_erpnext_grouping_and_filters() {
    assert_eq!(
        VoucherBalanceQueryPlan::default(),
        VoucherBalanceQueryPlan {
            source_doctype: "GL Entry",
            selected_fields: vec!["voucher_type", "voucher_no", "Sum(debit)", "Sum(credit)"],
            base_filter: "is_cancelled = 0",
            group_by: "voucher_no",
            optional_filters: vec![
                "company = filters.company",
                "voucher_type = filters.voucher_type",
                "posting_date >= filters.from_date",
                "posting_date <= filters.to_date",
            ],
        }
    );
}

#[test]
fn voucher_wise_balance_apply_filters_matches_erpnext_optional_filters() {
    let entries = vec![
        GlEntry::new(
            "Sales Invoice",
            "SINV-0001",
            "Test Company",
            "2026-05-01",
            100.0,
            0.0,
            false,
        ),
        GlEntry::new(
            "Purchase Invoice",
            "PINV-0001",
            "Test Company",
            "2026-05-03",
            40.0,
            0.0,
            false,
        ),
        GlEntry::new(
            "Sales Invoice",
            "SINV-0002",
            "Other Company",
            "2026-05-02",
            10.0,
            0.0,
            false,
        ),
        GlEntry::new(
            "Sales Invoice",
            "SINV-0003",
            "Test Company",
            "2026-04-30",
            10.0,
            0.0,
            false,
        ),
    ];

    let filters = VoucherBalanceFilters {
        company: Some("Test Company".to_string()),
        voucher_type: Some("Sales Invoice".to_string()),
        from_date: Some("2026-05-01".to_string()),
        to_date: Some("2026-05-02".to_string()),
    };

    let filtered = apply_filters(entries, &filters);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].voucher_no, "SINV-0001");
}

#[test]
fn voucher_wise_balance_groups_active_gl_entries_and_returns_only_unmatched() {
    let entries = vec![
        GlEntry::new(
            "Sales Invoice",
            "SINV-0001",
            "Test Company",
            "2026-05-01",
            100.0,
            0.0,
            false,
        ),
        GlEntry::new(
            "Sales Invoice",
            "SINV-0001",
            "Test Company",
            "2026-05-01",
            0.0,
            75.0,
            false,
        ),
        GlEntry::new(
            "Journal Entry",
            "JV-0001",
            "Test Company",
            "2026-05-01",
            40.0,
            0.0,
            false,
        ),
        GlEntry::new(
            "Journal Entry",
            "JV-0001",
            "Test Company",
            "2026-05-01",
            0.0,
            40.0,
            false,
        ),
        GlEntry::new(
            "Payment Entry",
            "PAY-0001",
            "Test Company",
            "2026-05-01",
            500.0,
            0.0,
            true,
        ),
    ];

    assert_eq!(
        get_data(entries, &VoucherBalanceFilters::default()),
        vec![VoucherBalanceRow {
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: "SINV-0001".to_string(),
            debit: 100.0,
            credit: 75.0,
        }]
    );
}

#[test]
fn voucher_wise_balance_execute_returns_columns_and_data() {
    let report = execute(
        vec![GlEntry::new(
            "Sales Invoice",
            "SINV-0001",
            "Test Company",
            "2026-05-01",
            100.0,
            0.0,
            false,
        )],
        VoucherBalanceFilters::default(),
    );

    assert_eq!(report.columns, get_columns());
    assert_eq!(report.rows.len(), 1);
}
