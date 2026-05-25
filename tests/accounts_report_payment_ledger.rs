use tokio_erp::erpnext::accounts::report::payment_ledger::payment_ledger::{
    apply_filters, execute, get_columns, PaymentLedgerEntry, PaymentLedgerFilters,
    PaymentLedgerQueryPlan, PaymentLedgerRow, ReportColumn,
};

#[test]
fn payment_ledger_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(&PaymentLedgerFilters::default()),
        vec![
            ReportColumn::new("Posting Date", "posting_date", "Date", None, "100", false),
            ReportColumn::new("Account", "account", "data", None, "100", false),
            ReportColumn::new("Party Type", "party_type", "data", None, "100", false),
            ReportColumn::new("Party", "party", "data", None, "100", false),
            ReportColumn::new("Voucher Type", "voucher_type", "data", None, "100", false),
            ReportColumn::new(
                "Voucher No",
                "voucher_no",
                "Dynamic Link",
                Some("voucher_type"),
                "100",
                false,
            ),
            ReportColumn::new(
                "Against Voucher Type",
                "against_voucher_type",
                "data",
                None,
                "100",
                false,
            ),
            ReportColumn::new(
                "Against Voucher No",
                "against_voucher_no",
                "Dynamic Link",
                Some("against_voucher_type"),
                "100",
                false,
            ),
            ReportColumn::new(
                "Amount",
                "amount",
                "Currency",
                Some("Company:company:default_currency"),
                "100",
                false,
            ),
            ReportColumn::new("Currency", "currency", "Link", Some("Currency"), "", true,),
        ]
    );
}

#[test]
fn payment_ledger_account_currency_column_is_inserted_before_currency_like_erpnext() {
    let filters = PaymentLedgerFilters {
        include_account_currency: true,
        ..Default::default()
    };

    let columns = get_columns(&filters);

    assert_eq!(
        &columns[9..],
        &[
            ReportColumn::new(
                "Amount in Account Currency",
                "amount_in_account_currency",
                "Currency",
                Some("currency"),
                "100",
                false,
            ),
            ReportColumn::new("Currency", "currency", "Link", Some("Currency"), "", true,),
        ]
    );
}

#[test]
fn payment_ledger_query_plan_matches_erpnext_conditions() {
    let filters = PaymentLedgerFilters {
        company: Some("_Test Company".to_string()),
        account: vec!["Debtors - TC".to_string(), "Creditors - TC".to_string()],
        period_start_date: Some("2026-05-01".to_string()),
        period_end_date: Some("2026-05-31".to_string()),
        voucher_no: Some("PAY-0001".to_string()),
        against_voucher_no: Some("SINV-0001".to_string()),
        party_type: Some("Customer".to_string()),
        party: vec!["_Test Customer".to_string()],
        ..Default::default()
    };

    assert_eq!(
        PaymentLedgerQueryPlan::from_filters(&filters),
        PaymentLedgerQueryPlan {
            source_doctype: "Payment Ledger Entry",
            selected_fields: vec!["*"],
            base_filters: vec!["delinked = 0".to_string()],
            optional_filters: vec![
                "company = _Test Company".to_string(),
                "account in ['Debtors - TC', 'Creditors - TC']".to_string(),
                "posting_date >= 2026-05-01".to_string(),
                "posting_date <= 2026-05-31".to_string(),
                "voucher_no = PAY-0001".to_string(),
                "against_voucher_no = SINV-0001".to_string(),
                "party_type = Customer".to_string(),
                "party in ['_Test Customer']".to_string(),
            ],
        }
    );
}

#[test]
fn payment_ledger_apply_filters_keeps_only_matching_non_delinked_entries() {
    let entries = vec![
        PaymentLedgerEntry::new(
            "2026-05-03",
            "Debtors - TC",
            "Customer",
            "_Test Customer",
            50.0,
        ),
        PaymentLedgerEntry::new(
            "2026-05-04",
            "Bank - TC",
            "Customer",
            "_Test Customer",
            25.0,
        ),
        PaymentLedgerEntry::new(
            "2026-05-05",
            "Debtors - TC",
            "Supplier",
            "_Test Supplier",
            20.0,
        ),
        PaymentLedgerEntry::new(
            "2026-06-01",
            "Debtors - TC",
            "Customer",
            "_Test Customer",
            15.0,
        ),
        PaymentLedgerEntry {
            delinked: true,
            ..PaymentLedgerEntry::new(
                "2026-05-06",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                10.0,
            )
        },
    ];
    let filters = PaymentLedgerFilters {
        company: Some("_Test Company".to_string()),
        account: vec!["Debtors - TC".to_string()],
        period_start_date: Some("2026-05-01".to_string()),
        period_end_date: Some("2026-05-31".to_string()),
        party_type: Some("Customer".to_string()),
        party: vec!["_Test Customer".to_string()],
        ..Default::default()
    };

    let filtered = apply_filters(entries, &filters);

    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].posting_date, "2026-05-03");
}

#[test]
fn payment_ledger_groups_by_against_voucher_and_orders_increase_then_decrease() {
    let entries = vec![
        PaymentLedgerEntry {
            against_voucher_type: Some("Sales Invoice".to_string()),
            against_voucher_no: Some("SINV-0001".to_string()),
            voucher_type: Some("Sales Invoice".to_string()),
            voucher_no: Some("SINV-0001".to_string()),
            amount_in_account_currency: Some(100.0),
            ..PaymentLedgerEntry::new(
                "2026-05-01",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                100.0,
            )
        },
        PaymentLedgerEntry {
            against_voucher_type: Some("Sales Invoice".to_string()),
            against_voucher_no: Some("SINV-0001".to_string()),
            voucher_type: Some("Payment Entry".to_string()),
            voucher_no: Some("PAY-0001".to_string()),
            amount_in_account_currency: Some(-40.0),
            ..PaymentLedgerEntry::new(
                "2026-05-02",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                -40.0,
            )
        },
        PaymentLedgerEntry {
            against_voucher_type: Some("Sales Invoice".to_string()),
            against_voucher_no: Some("SINV-0001".to_string()),
            voucher_type: Some("Journal Entry".to_string()),
            voucher_no: Some("JV-0001".to_string()),
            amount_in_account_currency: Some(0.0),
            ..PaymentLedgerEntry::new(
                "2026-05-03",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                0.0,
            )
        },
    ];
    let filters = PaymentLedgerFilters {
        include_account_currency: true,
        ..Default::default()
    };

    let report = execute(entries, filters);

    assert_eq!(
        report.rows,
        vec![
            PaymentLedgerRow::from_entry(
                "2026-05-01",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                "Sales Invoice",
                "SINV-0001",
                "Sales Invoice",
                "SINV-0001",
                100.0,
                "INR",
                "_Test Company",
                Some(100.0),
            ),
            PaymentLedgerRow::from_entry(
                "2026-05-02",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                "Payment Entry",
                "PAY-0001",
                "Sales Invoice",
                "SINV-0001",
                -40.0,
                "INR",
                "_Test Company",
                Some(-40.0),
            ),
            PaymentLedgerRow::from_entry(
                "2026-05-03",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                "Journal Entry",
                "JV-0001",
                "Sales Invoice",
                "SINV-0001",
                0.0,
                "INR",
                "_Test Company",
                Some(0.0),
            ),
            PaymentLedgerRow::balance("Outstanding:", 60.0, "INR", "_Test Company", Some(60.0)),
            PaymentLedgerRow::empty("INR", "_Test Company"),
        ]
    );
}

#[test]
fn payment_ledger_group_party_uses_party_type_and_party_key() {
    let entries = vec![
        PaymentLedgerEntry {
            against_voucher_type: Some("Sales Invoice".to_string()),
            against_voucher_no: Some("SINV-0001".to_string()),
            amount: 100.0,
            ..PaymentLedgerEntry::new(
                "2026-05-01",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                100.0,
            )
        },
        PaymentLedgerEntry {
            against_voucher_type: Some("Sales Invoice".to_string()),
            against_voucher_no: Some("SINV-0002".to_string()),
            amount: -25.0,
            ..PaymentLedgerEntry::new(
                "2026-05-02",
                "Debtors - TC",
                "Customer",
                "_Test Customer",
                -25.0,
            )
        },
    ];
    let filters = PaymentLedgerFilters {
        group_party: true,
        ..Default::default()
    };

    let report = execute(entries, filters);

    assert_eq!(report.rows.len(), 4);
    assert_eq!(
        report.rows[2].against_voucher_no.as_deref(),
        Some("Outstanding:")
    );
    assert_eq!(report.rows[2].amount, Some(75.0));
}
