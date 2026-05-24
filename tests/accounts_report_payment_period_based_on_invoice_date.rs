use tokio_erp::erpnext::accounts::report::payment_period_based_on_invoice_date::payment_period_based_on_invoice_date::{
    execute, get_columns, get_conditions, get_invoice_posting_date_map, PaymentInvoice,
    PaymentPeriodColumn, PaymentPeriodError, PaymentPeriodFilters, PaymentPeriodRow,
    PaymentPeriodTransaction,
};

#[test]
fn payment_period_based_on_invoice_date_validate_filters_matches_erpnext_guards() {
    let mut incoming_supplier = PaymentPeriodFilters::new("Incoming", "Test Company");
    incoming_supplier.party_type = Some("Supplier".to_string());
    assert_eq!(
        execute(incoming_supplier, vec![], vec![]).unwrap_err(),
        PaymentPeriodError::InvalidPartyTypeForPaymentType {
            payment_type: "Incoming".to_string(),
            party_type: "Supplier".to_string(),
        }
    );

    let mut outgoing_customer = PaymentPeriodFilters::new("Outgoing", "Test Company");
    outgoing_customer.party_type = Some("Customer".to_string());
    assert_eq!(
        execute(outgoing_customer, vec![], vec![]).unwrap_err(),
        PaymentPeriodError::InvalidPartyTypeForPaymentType {
            payment_type: "Outgoing".to_string(),
            party_type: "Customer".to_string(),
        }
    );
}

#[test]
fn payment_period_based_on_invoice_date_columns_match_erpnext_invoice_options() {
    let incoming = PaymentPeriodFilters::new("Incoming", "Test Company");
    assert_eq!(
        get_columns(&incoming)[5],
        PaymentPeriodColumn::link("Invoice", "invoice", "Sales Invoice", 160)
    );

    let outgoing = PaymentPeriodFilters::new("Outgoing", "Test Company");
    assert_eq!(
        get_columns(&outgoing)[5],
        PaymentPeriodColumn::link("Invoice", "invoice", "Purchase Invoice", 160)
    );
    assert_eq!(get_columns(&outgoing).len(), 16);
}

#[test]
fn payment_period_based_on_invoice_date_conditions_match_erpnext_branches() {
    let mut outgoing = PaymentPeriodFilters::new("Outgoing", "Test Company");
    outgoing.party = Some("SUP-0001".to_string());
    outgoing.from_date = Some("2026-05-01".to_string());
    outgoing.to_date = Some("2026-05-31".to_string());

    assert_eq!(
        get_conditions(&outgoing),
        vec![
            "delinked = 0",
            "party_type = Supplier",
            "against_voucher_type = Purchase Invoice",
            "party = filters.party",
            "posting_date >= filters.from_date",
            "posting_date <= filters.to_date",
            "company = filters.company",
        ]
    );

    let incoming = PaymentPeriodFilters::new("Incoming", "Test Company");
    assert_eq!(
        get_conditions(&incoming)[1..3],
        [
            "party_type = Customer",
            "against_voucher_type = Sales Invoice"
        ]
    );
}

#[test]
fn payment_period_based_on_invoice_date_invoice_map_uses_payment_type_and_company() {
    let invoices = vec![
        PaymentInvoice::new(
            "Sales Invoice",
            "SINV-0001",
            "Test Company",
            "2026-04-01",
            "2026-04-30",
            true,
        ),
        PaymentInvoice::new(
            "Purchase Invoice",
            "PINV-0001",
            "Test Company",
            "2026-04-02",
            "2026-05-02",
            true,
        ),
        PaymentInvoice::new(
            "Sales Invoice",
            "SINV-0002",
            "Other Company",
            "2026-04-03",
            "2026-04-30",
            true,
        ),
        PaymentInvoice::new(
            "Sales Invoice",
            "SINV-0003",
            "Test Company",
            "2026-04-04",
            "2026-04-30",
            false,
        ),
    ];

    let incoming = PaymentPeriodFilters::new("Incoming", "Test Company");
    let map = get_invoice_posting_date_map(&incoming, &invoices);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("SINV-0001").unwrap().posting_date, "2026-04-01");

    let outgoing = PaymentPeriodFilters::new("Outgoing", "Test Company");
    let map = get_invoice_posting_date_map(&outgoing, &invoices);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("PINV-0001").unwrap().due_date, "2026-05-02");
}

#[test]
fn payment_period_based_on_invoice_date_execute_builds_rows_with_abs_amount_ageing_and_delay() {
    let filters = PaymentPeriodFilters::new("Incoming", "Test Company")
        .with_report_date("2026-05-15")
        .with_range("30, 60, 90, 120");
    let entries = vec![PaymentPeriodTransaction::new(
        "Payment Entry",
        "PAY-0001",
        "Customer",
        "CUST-0001",
        "2026-05-10",
        -125.0,
        "Paid by bank",
        "SINV-0001",
    )];
    let invoices = vec![PaymentInvoice::new(
        "Sales Invoice",
        "SINV-0001",
        "Test Company",
        "2026-04-01",
        "2026-05-01",
        true,
    )];

    let report = execute(filters, entries, invoices).unwrap();

    assert_eq!(report.columns, get_columns(&report.filters));
    assert_eq!(
        report.rows,
        vec![PaymentPeriodRow {
            payment_document: "Payment Entry".to_string(),
            payment_entry: "PAY-0001".to_string(),
            party_type: "Customer".to_string(),
            party: "CUST-0001".to_string(),
            posting_date: "2026-05-10".to_string(),
            invoice: "SINV-0001".to_string(),
            invoice_posting_date: Some("2026-04-01".to_string()),
            due_date: Some("2026-05-01".to_string()),
            amount: 125.0,
            remarks: "Paid by bank".to_string(),
            age: Some(44),
            range1: 0.0,
            range2: 125.0,
            range3: 0.0,
            range4: 0.0,
            delay_in_payment: Some(9),
        }]
    );
}

#[test]
fn payment_period_based_on_invoice_date_execute_keeps_missing_invoice_shape() {
    let filters =
        PaymentPeriodFilters::new("Incoming", "Test Company").with_report_date("2026-05-15");
    let entries = vec![PaymentPeriodTransaction::new(
        "Payment Entry",
        "PAY-0002",
        "Customer",
        "CUST-0002",
        "2026-05-10",
        50.0,
        "",
        "",
    )];

    let report = execute(filters, entries, vec![]).unwrap();

    assert_eq!(report.rows[0].invoice_posting_date, None);
    assert_eq!(report.rows[0].due_date, None);
    assert_eq!(report.rows[0].age, None);
    assert_eq!(report.rows[0].delay_in_payment, None);
    assert_eq!(report.rows[0].range1, 0.0);
}
