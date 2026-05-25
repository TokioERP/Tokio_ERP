use tokio_erp::erpnext::accounts::report::sales_payment_summary::sales_payment_summary::{
    execute, get_columns, get_conditions, get_pos_columns, JournalEntryAllocation,
    PaymentEntryAllocation, ReportCell, SalesInvoice, SalesInvoicePayment, SalesPaymentFilters,
    SalesPaymentInput, SalesPaymentRow,
};

fn filters() -> SalesPaymentFilters {
    SalesPaymentFilters {
        from_date: Some("2026-01-01".to_string()),
        to_date: Some("2026-01-31".to_string()),
        company: Some("_Test Company".to_string()),
        customer: Some("_Test Customer".to_string()),
        owner: Some("cashier@example.com".to_string()),
        is_pos: false,
        payment_detail: false,
    }
}

fn invoice(
    name: &str,
    owner: &str,
    posting_date: &str,
    net_total: f64,
    taxes: f64,
) -> SalesInvoice {
    SalesInvoice {
        name: name.to_string(),
        posting_date: posting_date.to_string(),
        owner: owner.to_string(),
        company: "_Test Company".to_string(),
        customer: "_Test Customer".to_string(),
        docstatus: 1,
        is_pos: false,
        net_total,
        total_taxes_and_charges: taxes,
        base_paid_amount: 0.0,
        outstanding_amount: 0.0,
        base_change_amount: 0.0,
        warehouse: None,
        cost_center: None,
        base_total: net_total,
    }
}

#[test]
fn sales_payment_summary_columns_match_erpnext_string_columns() {
    assert_eq!(
        get_columns(&filters()),
        vec![
            "Date:Date:80",
            "Owner:Data:200",
            "Payment Mode:Data:240",
            "Sales and Returns:Currency/currency:120",
            "Taxes:Currency/currency:120",
            "Payments:Currency/currency:120",
        ]
    );

    let mut pos_filters = filters();
    pos_filters.is_pos = true;
    assert_eq!(
        get_columns(&pos_filters),
        vec![
            "Date:Date:80",
            "Owner:Data:200",
            "Payment Mode:Data:240",
            "Sales and Returns:Currency/currency:120",
            "Taxes:Currency/currency:120",
            "Payments:Currency/currency:120",
            "Warehouse:Data:200",
            "Cost Center:Data:200",
        ]
    );
    assert_eq!(get_columns(&pos_filters), get_pos_columns());
}

#[test]
fn sales_payment_summary_conditions_match_erpnext_optional_filters() {
    assert_eq!(
        get_conditions(&filters()),
        "1=1 and a.posting_date >= %(from_date)s and a.posting_date <= %(to_date)s and a.company=%(company)s and a.customer = %(customer)s and a.owner = %(owner)s"
    );

    let mut pos_filters = filters();
    pos_filters.is_pos = true;
    assert!(get_conditions(&pos_filters).ends_with(" and a.is_pos = %(is_pos)s"));
}

#[test]
fn sales_payment_summary_non_pos_groups_invoices_and_adjusts_cash_change_amount() {
    let mut si_1 = invoice(
        "SINV-0001",
        "cashier@example.com",
        "2026-01-10",
        100.0,
        10.0,
    );
    si_1.base_change_amount = 15.0;
    let si_2 = invoice("SINV-0002", "cashier@example.com", "2026-01-10", 50.0, 5.0);
    let mut other_owner = invoice("SINV-0003", "other@example.com", "2026-01-10", 999.0, 99.0);
    other_owner.owner = "other@example.com".to_string();

    let report = execute(
        &filters(),
        &SalesPaymentInput {
            sales_invoices: vec![si_1, si_2, other_owner],
            sales_invoice_payments: vec![
                SalesInvoicePayment::new("SINV-0001", "Cash", "Cash", 100.0),
                SalesInvoicePayment::new("SINV-0002", "Card", "Bank", 40.0),
            ],
            payment_entries: vec![PaymentEntryAllocation::new("SINV-0001", "Bank", 25.0, 1)],
            journal_entries: vec![JournalEntryAllocation::new(
                "SINV-0002",
                "Journal Entry",
                "Sales Invoice",
                10.0,
                1,
            )],
        },
    );

    assert_eq!(report.columns, get_columns(&filters()));
    assert_eq!(
        report.rows,
        vec![SalesPaymentRow::new(vec![
            ReportCell::text("2026-01-10"),
            ReportCell::text("cashier@example.com"),
            ReportCell::text("Cash, Card, Bank, Journal Entry"),
            ReportCell::number(150.0),
            ReportCell::number(15.0),
            ReportCell::number(160.0),
        ])]
    );
}

#[test]
fn sales_payment_summary_payment_detail_preserves_erpnext_rows_and_width_oddity() {
    let mut report_filters = filters();
    report_filters.payment_detail = true;

    let report = execute(
        &report_filters,
        &SalesPaymentInput {
            sales_invoices: vec![invoice(
                "SINV-0001",
                "cashier@example.com",
                "2026-01-10",
                100.0,
                10.0,
            )],
            sales_invoice_payments: vec![SalesInvoicePayment::new(
                "SINV-0001",
                "Cash",
                "Cash",
                100.0,
            )],
            payment_entries: vec![],
            journal_entries: vec![],
        },
    );

    assert_eq!(
        report.rows,
        vec![
            SalesPaymentRow::new(vec![
                ReportCell::text("2026-01-10"),
                ReportCell::text("cashier@example.com"),
                ReportCell::text(" "),
                ReportCell::number(100.0),
                ReportCell::number(10.0),
                ReportCell::number(0.0),
            ]),
            SalesPaymentRow::new(vec![
                ReportCell::text("2026-01-10"),
                ReportCell::text("cashier@example.com"),
                ReportCell::text("Cash"),
                ReportCell::number(0.0),
                ReportCell::number(0.0),
                ReportCell::number(100.0),
                ReportCell::number(0.0),
            ]),
        ]
    );
}

#[test]
fn sales_payment_summary_pos_branch_maps_pos_invoice_fields() {
    let mut pos_invoice = invoice("POS-0001", "cashier@example.com", "2026-01-10", 200.0, 20.0);
    pos_invoice.is_pos = true;
    pos_invoice.base_paid_amount = 220.0;
    pos_invoice.warehouse = Some("Stores - TC".to_string());
    pos_invoice.cost_center = Some("Main - TC".to_string());

    let mut pos_filters = filters();
    pos_filters.is_pos = true;

    let report = execute(
        &pos_filters,
        &SalesPaymentInput {
            sales_invoices: vec![pos_invoice],
            sales_invoice_payments: vec![SalesInvoicePayment::new(
                "POS-0001", "Cash", "Cash", 220.0,
            )],
            payment_entries: vec![],
            journal_entries: vec![],
        },
    );

    assert_eq!(report.columns, get_columns(&pos_filters));
    assert_eq!(
        report.rows,
        vec![SalesPaymentRow::new(vec![
            ReportCell::text("2026-01-10"),
            ReportCell::text("cashier@example.com"),
            ReportCell::text("Cash"),
            ReportCell::number(200.0),
            ReportCell::number(20.0),
            ReportCell::number(220.0),
            ReportCell::text("Stores - TC"),
            ReportCell::text("Main - TC"),
        ])]
    );
}
