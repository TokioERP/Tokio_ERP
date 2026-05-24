use tokio_erp::erpnext::accounts::report::tax_withholding_details::tax_withholding_details::{
    build_rows, execute, fetch_additional_doc_info, fetch_party_details, get_columns,
    get_doc_info_query, get_entries_query_plan, get_party_query, validate_filters, DocInfo,
    DocInfoQueryPlan, PartyInfo, PartyQueryPlan, TaxWithholdingColumn, TaxWithholdingEntry,
    TaxWithholdingError, TaxWithholdingFilters, TaxWithholdingRow, DOCUMENT_TYPES, PARTY_TYPES,
};

#[test]
fn tax_withholding_details_constants_match_erpnext_report() {
    assert_eq!(PARTY_TYPES, ["Customer", "Supplier"]);
    assert_eq!(
        DOCUMENT_TYPES,
        [
            "Purchase Invoice",
            "Sales Invoice",
            "Payment Entry",
            "Journal Entry"
        ]
    );
}

#[test]
fn tax_withholding_details_validate_filters_matches_erpnext_guards() {
    assert_eq!(
        validate_filters(&TaxWithholdingFilters::default()),
        Err(TaxWithholdingError::MissingDateRange)
    );
    assert_eq!(
        validate_filters(&TaxWithholdingFilters::new("2026-05-02", "2026-05-01")),
        Err(TaxWithholdingError::FromDateAfterToDate)
    );
    assert_eq!(
        validate_filters(&TaxWithholdingFilters::new("2026-05-01", "2026-05-02")),
        Ok(())
    );
}

#[test]
fn tax_withholding_details_entries_query_plan_matches_erpnext_filters() {
    let mut filters = TaxWithholdingFilters::new("2026-05-01", "2026-05-31");
    filters.company = Some("Test Company".to_string());
    filters.party_type = Some("Supplier".to_string());
    filters.party = Some("SUP-0001".to_string());

    let plan = get_entries_query_plan(&filters);

    assert_eq!(plan.source_doctype, "Tax Withholding Entry");
    assert_eq!(
        plan.selected_fields,
        vec![
            "party_type",
            "party",
            "IfNull(tax_id, '') as tax_id",
            "tax_withholding_category",
            "taxable_amount as total_amount",
            "tax_rate as rate",
            "withholding_amount as tax_amount",
            "IfNull(taxable_doctype, '') as transaction_type",
            "IfNull(taxable_name, '') as ref_no",
            "taxable_date",
            "IfNull(withholding_doctype, '') as withholding_doctype",
            "IfNull(withholding_name, '') as withholding_name",
            "withholding_date as transaction_date",
        ]
    );
    assert_eq!(
        plan.filters,
        vec![
            "docstatus = 1",
            "withholding_date >= filters.from_date",
            "withholding_date <= filters.to_date",
            "IfNull(withholding_name, '') != ''",
            "status != 'Duplicate'",
            "company = filters.company",
            "party_type = filters.party_type",
            "party = filters.party",
        ]
    );
}

#[test]
fn tax_withholding_details_party_queries_match_customer_supplier_fields() {
    assert_eq!(
        get_party_query("Supplier", vec!["SUP-0001".to_string()]),
        Some(PartyQueryPlan {
            party_type: "Supplier",
            names: vec!["SUP-0001".to_string()],
            fields: vec![
                "name",
                "supplier_type as party_entity_type",
                "supplier_name as party_name"
            ],
        })
    );
    assert_eq!(
        get_party_query("Customer", vec!["CUST-0001".to_string()]),
        Some(PartyQueryPlan {
            party_type: "Customer",
            names: vec!["CUST-0001".to_string()],
            fields: vec![
                "name",
                "customer_type as party_entity_type",
                "customer_name as party_name"
            ],
        })
    );
    assert_eq!(
        get_party_query("Employee", vec!["EMP-0001".to_string()]),
        None
    );
}

#[test]
fn tax_withholding_details_fetch_party_details_keeps_only_report_party_types() {
    let entries = vec![
        TaxWithholdingEntry::new("Supplier", "SUP-0001", "TDS-A", "PINV-0001"),
        TaxWithholdingEntry::new("Customer", "CUST-0001", "TDS-B", "SINV-0001"),
        TaxWithholdingEntry::new("Employee", "EMP-0001", "TDS-C", "JV-0001"),
    ];
    let parties = vec![
        PartyInfo::new("Supplier", "SUP-0001", "Company", "Supplier One"),
        PartyInfo::new("Customer", "CUST-0001", "Individual", "Customer One"),
        PartyInfo::new("Employee", "EMP-0001", "Individual", "Employee One"),
    ];

    assert_eq!(
        fetch_party_details(&entries, &parties),
        vec![
            PartyInfo::new("Customer", "CUST-0001", "Individual", "Customer One"),
            PartyInfo::new("Supplier", "SUP-0001", "Company", "Supplier One"),
        ]
    );
}

#[test]
fn tax_withholding_details_doc_info_queries_match_erpnext_dispatch_fields() {
    assert_eq!(
        get_doc_info_query("Purchase Invoice", vec!["PINV-0001".to_string()]),
        Some(DocInfoQueryPlan {
            doctype: "Purchase Invoice",
            names: vec!["PINV-0001".to_string()],
            fields: vec![
                "name",
                "grand_total",
                "base_total",
                "bill_no as supplier_invoice_no",
                "bill_date as supplier_invoice_date"
            ],
        })
    );
    assert_eq!(
        get_doc_info_query("Payment Entry", vec!["PAY-0001".to_string()])
            .unwrap()
            .fields,
        vec![
            "name",
            "paid_amount_after_tax as grand_total",
            "base_paid_amount as base_total"
        ]
    );
    assert_eq!(
        get_doc_info_query("Journal Entry", vec!["JV-0001".to_string()])
            .unwrap()
            .fields,
        vec![
            "name",
            "total_debit as grand_total",
            "total_debit as base_total"
        ]
    );
    assert_eq!(
        get_doc_info_query("Delivery Note", vec!["DN-0001".to_string()]),
        None
    );
}

#[test]
fn tax_withholding_details_fetch_additional_doc_info_keeps_supported_references_only() {
    let mut purchase = TaxWithholdingEntry::new("Supplier", "SUP-0001", "TDS-A", "PINV-0001");
    purchase.transaction_type = "Purchase Invoice".to_string();
    let mut unsupported = TaxWithholdingEntry::new("Supplier", "SUP-0002", "TDS-A", "DN-0001");
    unsupported.transaction_type = "Delivery Note".to_string();
    let docs = vec![
        DocInfo::purchase_invoice("PINV-0001", 120.0, 100.0, "BILL-1", "2026-05-01"),
        DocInfo::sales_invoice("SINV-0001", 220.0, 200.0),
    ];

    assert_eq!(
        fetch_additional_doc_info(&[purchase, unsupported], &docs),
        vec![DocInfo::purchase_invoice(
            "PINV-0001",
            120.0,
            100.0,
            "BILL-1",
            "2026-05-01"
        )]
    );
}

#[test]
fn tax_withholding_details_build_rows_merges_and_sorts_like_erpnext() {
    let mut later = TaxWithholdingEntry::new("Supplier", "SUP-0001", "TDS-B", "PINV-0001");
    later.transaction_type = "Purchase Invoice".to_string();
    later.transaction_date = Some("2026-05-02".to_string());
    later.withholding_name = "ACC-TDS-2".to_string();
    later.total_amount = 100.0;
    later.tax_amount = 10.0;

    let mut first = TaxWithholdingEntry::new("Customer", "CUST-0001", "TDS-A", "SINV-0001");
    first.transaction_type = "Sales Invoice".to_string();
    first.transaction_date = Some("2026-05-01".to_string());
    first.withholding_name = "ACC-TDS-1".to_string();
    first.total_amount = 200.0;
    first.tax_amount = 20.0;

    let rows = build_rows(
        vec![later, first],
        &[DocInfo::purchase_invoice(
            "PINV-0001",
            120.0,
            100.0,
            "BILL-1",
            "2026-05-01",
        )],
        &[
            PartyInfo::new("Supplier", "SUP-0001", "Company", "Supplier One"),
            PartyInfo::new("Customer", "CUST-0001", "Individual", "Customer One"),
        ],
    );

    assert_eq!(rows[0].tax_withholding_category, "TDS-A");
    assert_eq!(rows[0].party_name.as_deref(), Some("Customer One"));
    assert_eq!(rows[1].tax_withholding_category, "TDS-B");
    assert_eq!(rows[1].supplier_invoice_no.as_deref(), Some("BILL-1"));
}

#[test]
fn tax_withholding_details_columns_match_erpnext_report_shape() {
    let columns = get_columns(&TaxWithholdingFilters::new("2026-05-01", "2026-05-31"));

    assert_eq!(columns.len(), 17);
    assert_eq!(
        columns[0],
        TaxWithholdingColumn::link(
            "Tax Withholding Category",
            "tax_withholding_category",
            "Tax Withholding Category",
            90,
        )
    );
    assert_eq!(
        columns[3],
        TaxWithholdingColumn::dynamic_link("Party", "party", "party_type", 180)
    );
    assert_eq!(
        columns[16],
        TaxWithholdingColumn::dynamic_link(
            "Withholding Document",
            "withholding_name",
            "withholding_doctype",
            150
        )
    );

    let mut filters = TaxWithholdingFilters::new("2026-05-01", "2026-05-31");
    filters.party_type = Some("Supplier".to_string());
    assert_eq!(get_columns(&filters)[2].label, "Supplier Name");
    assert_eq!(get_columns(&filters)[3].label, "Supplier");
    assert_eq!(get_columns(&filters)[4].label, "Supplier Type");
}

#[test]
fn tax_withholding_details_execute_validates_and_returns_enriched_rows() {
    let mut entry = TaxWithholdingEntry::new("Supplier", "SUP-0001", "TDS-A", "PINV-0001");
    entry.transaction_type = "Purchase Invoice".to_string();
    entry.transaction_date = Some("2026-05-01".to_string());
    entry.withholding_name = "ACC-TDS-1".to_string();

    let report = execute(
        TaxWithholdingFilters::new("2026-05-01", "2026-05-31"),
        vec![entry],
        vec![DocInfo::purchase_invoice(
            "PINV-0001",
            120.0,
            100.0,
            "BILL-1",
            "2026-05-01",
        )],
        vec![PartyInfo::new(
            "Supplier",
            "SUP-0001",
            "Company",
            "Supplier One",
        )],
    )
    .unwrap();

    assert_eq!(report.columns, get_columns(&report.filters));
    assert_eq!(
        report.rows,
        vec![TaxWithholdingRow {
            party_type: "Supplier".to_string(),
            party: "SUP-0001".to_string(),
            tax_id: "".to_string(),
            tax_withholding_category: "TDS-A".to_string(),
            total_amount: 0.0,
            rate: 0.0,
            tax_amount: 0.0,
            transaction_type: "Purchase Invoice".to_string(),
            ref_no: "PINV-0001".to_string(),
            taxable_date: None,
            withholding_doctype: "".to_string(),
            withholding_name: "ACC-TDS-1".to_string(),
            transaction_date: Some("2026-05-01".to_string()),
            party_entity_type: Some("Company".to_string()),
            party_name: Some("Supplier One".to_string()),
            grand_total: Some(120.0),
            base_total: Some(100.0),
            supplier_invoice_no: Some("BILL-1".to_string()),
            supplier_invoice_date: Some("2026-05-01".to_string()),
        }]
    );
}
