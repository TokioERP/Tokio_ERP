use tokio_erp::erpnext::accounts::report::trial_balance_for_party::trial_balance_for_party::{
    execute, get_columns, is_party_name_visible, toggle_debit_credit, GlEntry, PartyRecord,
    ReportColumn, TrialBalanceForPartyFilters, TrialBalanceForPartyQueryPlan, TrialBalancePartyRow,
};

fn filters() -> TrialBalanceForPartyFilters {
    TrialBalanceForPartyFilters {
        company: "_Test Company".to_string(),
        party_type: "Customer".to_string(),
        party: None,
        account_filter: Vec::new(),
        from_date: "2026-01-01".to_string(),
        to_date: "2026-01-31".to_string(),
        show_zero_values: false,
        exclude_zero_balance_parties: false,
        customer_naming_by: Some("Naming Series".to_string()),
        supplier_naming_by: None,
    }
}

fn parties() -> Vec<PartyRecord> {
    vec![
        PartyRecord::new("CUST-0001", Some("_Test Customer")),
        PartyRecord::new("CUST-0002", Some("_Zero Customer")),
        PartyRecord::new("CUST-0003", Some("_Filtered Customer")),
    ]
}

#[test]
fn trial_balance_for_party_columns_match_erpnext_dynamic_party_name_shape() {
    assert_eq!(
        get_columns(&filters(), true),
        vec![
            ReportColumn::link("Customer", "party", "Customer", 200, false),
            ReportColumn::data("Customer Name", "party_name", 200),
            ReportColumn::currency("Opening (Dr)", "opening_debit", 120),
            ReportColumn::currency("Opening (Cr)", "opening_credit", 120),
            ReportColumn::currency("Debit", "debit", 120),
            ReportColumn::currency("Credit", "credit", 120),
            ReportColumn::currency("Closing (Dr)", "closing_debit", 120),
            ReportColumn::currency("Closing (Cr)", "closing_credit", 120),
            ReportColumn::link("Currency", "currency", "Currency", 0, true),
        ]
    );
}

#[test]
fn trial_balance_for_party_query_plan_matches_opening_and_period_filters() {
    let mut report_filters = filters();
    report_filters.account_filter = vec!["Debtors - TC".to_string(), "Receivable - TC".to_string()];

    assert_eq!(
        TrialBalanceForPartyQueryPlan::for_filters(&report_filters),
        TrialBalanceForPartyQueryPlan {
            party_doctype: "Customer".to_string(),
            party_fields: vec!["name".to_string(), "customer_name".to_string()],
            party_filters: Vec::new(),
            opening_filters: vec![
                "company = filters.company".to_string(),
                "is_cancelled = 0".to_string(),
                "party_type = filters.party_type".to_string(),
                "party != ''".to_string(),
                "posting_date < filters.from_date OR (is_opening = 'Yes' AND posting_date <= filters.to_date)".to_string(),
                "account in account_filter".to_string(),
            ],
            period_filters: vec![
                "company = filters.company".to_string(),
                "is_cancelled = 0".to_string(),
                "party_type = filters.party_type".to_string(),
                "party != ''".to_string(),
                "posting_date >= filters.from_date".to_string(),
                "posting_date <= filters.to_date".to_string(),
                "is_opening = 'No'".to_string(),
                "account in account_filter".to_string(),
            ],
            group_by: vec!["party"],
        }
    );
}

#[test]
fn trial_balance_for_party_visibility_matches_party_type_settings() {
    let mut customer = filters();
    customer.customer_naming_by = Some("Naming Series".to_string());
    assert!(is_party_name_visible(&customer));

    customer.customer_naming_by = Some("Customer Name".to_string());
    assert!(!is_party_name_visible(&customer));

    let mut supplier = filters();
    supplier.party_type = "Supplier".to_string();
    supplier.supplier_naming_by = Some("Naming Series".to_string());
    assert!(is_party_name_visible(&supplier));

    let mut employee = filters();
    employee.party_type = "Employee".to_string();
    assert!(is_party_name_visible(&employee));
}

#[test]
fn trial_balance_for_party_toggle_debit_credit_matches_erpnext() {
    assert_eq!(toggle_debit_credit(100.0, 30.0), (70.0, 0.0));
    assert_eq!(toggle_debit_credit(25.0, 75.0), (0.0, 50.0));
    assert_eq!(toggle_debit_credit(40.0, 40.0), (0.0, 0.0));
}

#[test]
fn trial_balance_for_party_calculates_opening_period_closing_and_totals() {
    let report = execute(
        &filters(),
        parties(),
        vec![
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0001",
                "2025-12-31",
                100.0,
                20.0,
                "No",
                false,
            ),
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0001",
                "2026-01-15",
                30.0,
                10.0,
                "No",
                false,
            ),
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0002",
                "2026-01-15",
                0.0,
                0.0,
                "No",
                false,
            ),
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0003",
                "2026-02-01",
                999.0,
                0.0,
                "No",
                false,
            ),
        ],
        "USD",
    );

    assert_eq!(report.rows.len(), 2);
    assert_eq!(
        report.rows[0],
        TrialBalancePartyRow {
            party: "CUST-0001".to_string(),
            party_name: Some("_Test Customer".to_string()),
            opening_debit: 80.0,
            opening_credit: 0.0,
            debit: 30.0,
            credit: 10.0,
            closing_debit: 100.0,
            closing_credit: 0.0,
            currency: "USD".to_string(),
        }
    );
    assert_eq!(report.rows[1].party, "'Totals'");
    assert_eq!(report.rows[1].opening_debit, 80.0);
    assert_eq!(report.rows[1].debit, 30.0);
    assert_eq!(report.rows[1].credit, 10.0);
    assert_eq!(report.rows[1].closing_debit, 100.0);
}

#[test]
fn trial_balance_for_party_filters_party_and_excludes_zero_closing_balance() {
    let mut report_filters = filters();
    report_filters.party = Some("CUST-0001".to_string());
    report_filters.exclude_zero_balance_parties = true;

    let report = execute(
        &report_filters,
        parties(),
        vec![
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0001",
                "2025-12-31",
                50.0,
                0.0,
                "No",
                false,
            ),
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0001",
                "2026-01-10",
                0.0,
                50.0,
                "No",
                false,
            ),
            GlEntry::new(
                "_Test Company",
                "Debtors - TC",
                "Customer",
                "CUST-0002",
                "2026-01-10",
                100.0,
                0.0,
                "No",
                false,
            ),
        ],
        "USD",
    );

    assert_eq!(report.rows.len(), 1);
    assert_eq!(report.rows[0].party, "'Totals'");
    assert_eq!(report.rows[0].debit, 0.0);
}
