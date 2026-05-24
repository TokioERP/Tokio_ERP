use tokio_erp::erpnext::accounts::report::tds_computation_summary::tds_computation_summary::{
    execute, get_columns, group_rows, validate_filters, FiscalYearSpan, TdsColumn, TdsFilters,
    TdsReportError, TdsRow, AGGREGATE_FIELDS, CARRY_OVER_FIELDS, GROUP_BY_FIELDS,
};

#[test]
fn tds_computation_summary_constants_match_erpnext_class_fields() {
    assert_eq!(
        GROUP_BY_FIELDS,
        ["party_type", "party", "tax_withholding_category"]
    );
    assert_eq!(
        CARRY_OVER_FIELDS,
        [
            "tax_id",
            "party",
            "party_type",
            "party_name",
            "tax_withholding_category",
            "party_entity_type",
            "rate",
        ]
    );
    assert_eq!(AGGREGATE_FIELDS, ["total_amount", "tax_amount"]);
}

#[test]
fn tds_computation_summary_validate_filters_matches_erpnext_guards() {
    let fiscal_years = [FiscalYearSpan::new("2026", "2026-01-01", "2026-12-31")];

    let mut reversed = TdsFilters::new("2026-05-02", "2026-05-01");
    assert_eq!(
        validate_filters(&mut reversed, &fiscal_years),
        Err(TdsReportError::FromDateAfterToDate)
    );

    let mut valid = TdsFilters::new("2026-05-01", "2026-05-02");
    assert_eq!(validate_filters(&mut valid, &fiscal_years), Ok(()));
    assert_eq!(valid.fiscal_year.as_deref(), Some("2026"));
}

#[test]
fn tds_computation_summary_rejects_ranges_crossing_fiscal_years() {
    let fiscal_years = [
        FiscalYearSpan::new("2025", "2025-01-01", "2025-12-31"),
        FiscalYearSpan::new("2026", "2026-01-01", "2026-12-31"),
    ];
    let mut filters = TdsFilters::new("2025-12-31", "2026-01-01");

    assert_eq!(
        validate_filters(&mut filters, &fiscal_years),
        Err(TdsReportError::DifferentFiscalYear)
    );
}

#[test]
fn tds_computation_summary_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(&TdsFilters::new("2026-05-01", "2026-05-31")),
        vec![
            TdsColumn::data("Tax Id", "tax_id", 90),
            TdsColumn::dynamic_link("Party", "party", "party_type", 180),
            TdsColumn::data("Party Name", "party_name", 180),
            TdsColumn::link(
                "Tax Withholding Category",
                "tax_withholding_category",
                "Tax Withholding Category",
                180,
            ),
            TdsColumn::data("Party Type", "party_entity_type", 180),
            TdsColumn::percent("Tax Rate %", "rate", 120),
            TdsColumn::float("Total Taxable Amount", "total_amount", 120),
            TdsColumn::float("Tax Amount", "tax_amount", 120),
        ]
    );

    let mut supplier_filters = TdsFilters::new("2026-05-01", "2026-05-31");
    supplier_filters.party_type = Some("Supplier".to_string());
    assert_eq!(get_columns(&supplier_filters)[1].label, "Supplier");
    assert_eq!(get_columns(&supplier_filters)[2].label, "Supplier Name");
    assert_eq!(get_columns(&supplier_filters)[4].label, "Supplier Type");
}

#[test]
fn tds_computation_summary_group_rows_aggregates_and_preserves_first_bucket_fields() {
    let rows = vec![
        TdsRow::new(
            "TAX-1",
            "Supplier",
            "SUP-1",
            "Supplier One",
            "TDS-A",
            "Company",
            Some(10.0),
        )
        .with_amounts(Some(100.0), Some(10.0)),
        TdsRow::new(
            "TAX-2",
            "Supplier",
            "SUP-1",
            "Changed Name",
            "TDS-A",
            "Individual",
            Some(20.0),
        )
        .with_amounts(None, Some(5.0)),
        TdsRow::new(
            "TAX-3",
            "Customer",
            "CUST-1",
            "Customer One",
            "TDS-B",
            "Company",
            Some(7.5),
        )
        .with_amounts(Some(250.0), None),
    ];

    let grouped = group_rows(rows);

    assert_eq!(
        grouped,
        vec![
            TdsRow::new(
                "TAX-1",
                "Supplier",
                "SUP-1",
                "Supplier One",
                "TDS-A",
                "Company",
                Some(10.0),
            )
            .with_amounts(Some(100.0), Some(15.0)),
            TdsRow::new(
                "TAX-3",
                "Customer",
                "CUST-1",
                "Customer One",
                "TDS-B",
                "Company",
                Some(7.5),
            )
            .with_amounts(Some(250.0), Some(0.0)),
        ]
    );
}

#[test]
fn tds_computation_summary_execute_validates_then_groups_rows() {
    let fiscal_years = [FiscalYearSpan::new("2026", "2026-01-01", "2026-12-31")];
    let filters = TdsFilters::new("2026-05-01", "2026-05-31");
    let rows = vec![
        TdsRow::new(
            "TAX-1",
            "Supplier",
            "SUP-1",
            "Supplier One",
            "TDS-A",
            "Company",
            Some(10.0),
        )
        .with_amounts(Some(100.0), Some(10.0)),
        TdsRow::new(
            "TAX-1",
            "Supplier",
            "SUP-1",
            "Supplier One",
            "TDS-A",
            "Company",
            Some(10.0),
        )
        .with_amounts(Some(50.0), Some(5.0)),
    ];

    let report = execute(filters, rows, &fiscal_years).unwrap();

    assert_eq!(report.columns, get_columns(&report.filters));
    assert_eq!(report.filters.fiscal_year.as_deref(), Some("2026"));
    assert_eq!(report.rows[0].total_amount, Some(150.0));
    assert_eq!(report.rows[0].tax_amount, Some(15.0));
}
