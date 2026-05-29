use tokio_erp::erpnext::accounts::report::deferred_revenue_and_expense::deferred_revenue_and_expense::{
    execute, BookingBasis, DeferredEntry, DeferredFilters, DeferredInvoice, DeferredItem,
    DeferredType, Period, ReportColumn,
};

fn filters(book_by: BookingBasis, with_upcoming_postings: bool) -> DeferredFilters {
    DeferredFilters {
        company: "Acme".to_string(),
        filter_based_on: "Date Range".to_string(),
        period_start_date: "2026-01-01".to_string(),
        period_end_date: "2026-03-31".to_string(),
        from_fiscal_year: "2026".to_string(),
        to_fiscal_year: "2026".to_string(),
        periodicity: "Monthly".to_string(),
        deferred_type: DeferredType::Revenue,
        with_upcoming_postings,
        book_deferred_entries_based_on: book_by,
    }
}

fn periods() -> Vec<Period> {
    vec![
        Period::new("jan_2026", "Jan 2026", "2026-01-01", "2026-01-31"),
        Period::new("feb_2026", "Feb 2026", "2026-02-01", "2026-02-28"),
        Period::new("mar_2026", "Mar 2026", "2026-03-01", "2026-03-31"),
    ]
}

fn revenue_entry(
    item: &str,
    gle_date: &str,
    debit: f64,
    credit: f64,
    posted: bool,
) -> DeferredEntry {
    DeferredEntry {
        doc: "SINV-1".to_string(),
        company: "Acme".to_string(),
        docstatus: 1,
        is_cancelled: false,
        enable_deferred_revenue: true,
        enable_deferred_expense: false,
        posting_date: "2026-01-01".to_string(),
        item: item.to_string(),
        item_name: item.to_string(),
        service_start_date: "2026-01-01".to_string(),
        service_end_date: "2026-03-31".to_string(),
        base_net_amount: 300.0,
        deferred_revenue_account: Some("Deferred Income".to_string()),
        deferred_expense_account: None,
        gle_posting_date: gle_date.to_string(),
        debit,
        credit,
        posted,
    }
}

fn expense_entry(
    item: &str,
    gle_date: &str,
    debit: f64,
    credit: f64,
    posted: bool,
) -> DeferredEntry {
    DeferredEntry {
        doc: "PINV-1".to_string(),
        company: "Acme".to_string(),
        docstatus: 1,
        is_cancelled: false,
        enable_deferred_revenue: false,
        enable_deferred_expense: true,
        posting_date: "2026-01-01".to_string(),
        item: item.to_string(),
        item_name: item.to_string(),
        service_start_date: "2026-01-01".to_string(),
        service_end_date: "2026-03-31".to_string(),
        base_net_amount: 300.0,
        deferred_revenue_account: None,
        deferred_expense_account: Some("Deferred Expense".to_string()),
        gle_posting_date: gle_date.to_string(),
        debit,
        credit,
        posted,
    }
}

#[test]
fn deferred_item_amount_sign_matches_sale_and_purchase_rules() {
    let sale = DeferredItem::new(
        "SERV",
        "SINV-1",
        vec![revenue_entry("SERV", "2026-01-31", 100.0, 0.0, true)],
        filters(BookingBasis::Days, false),
        periods(),
    )
    .expect("sale item");
    let purchase = DeferredItem::new(
        "SERV",
        "PINV-1",
        vec![expense_entry("SERV", "2026-01-31", 0.0, 80.0, true)],
        filters(BookingBasis::Days, false),
        periods(),
    )
    .expect("purchase item");

    assert_eq!(sale.get_amount(&sale.gle_entries[0]), 100.0);
    assert_eq!(purchase.get_amount(&purchase.gle_entries[0]), -80.0);
}

#[test]
fn deferred_item_calculates_inclusive_days_and_caps_against_invoice_amount() {
    let item = DeferredItem::new(
        "SERV",
        "SINV-1",
        vec![revenue_entry("SERV", "2026-01-31", 250.0, 0.0, true)],
        filters(BookingBasis::Days, false),
        periods(),
    )
    .expect("item");

    assert_eq!(
        item.calculate_amount("2026-02-01", "2026-02-28")
            .expect("amount"),
        50.0
    );
}

#[test]
fn deferred_item_calculates_monthly_amount_with_partial_month_proration() {
    let mut entry = revenue_entry("SERV", "2026-01-31", 0.0, 0.0, true);
    entry.service_start_date = "2026-01-16".to_string();
    entry.service_end_date = "2026-03-15".to_string();
    entry.base_net_amount = 240.0;
    let item = DeferredItem::new(
        "SERV",
        "SINV-1",
        vec![entry],
        filters(BookingBasis::Months, false),
        periods(),
    )
    .expect("item");

    assert_eq!(
        item.calculate_amount("2026-01-16", "2026-01-31")
            .expect("jan amount"),
        60.0
    );
    assert_eq!(
        item.calculate_amount("2026-02-01", "2026-02-28")
            .expect("feb amount"),
        120.0
    );
}

#[test]
fn deferred_item_simulates_future_monthly_postings_from_last_real_posting() {
    let mut item = DeferredItem::new(
        "SERV",
        "SINV-1",
        vec![revenue_entry("SERV", "2026-01-31", 100.0, 0.0, true)],
        filters(BookingBasis::Days, true),
        periods(),
    )
    .expect("item");

    item.simulate_future_posting().expect("simulate");

    assert_eq!(item.gle_entries.len(), 3);
    assert_eq!(item.gle_entries[1].gle_posting_date, "2026-02-28");
    assert_eq!(item.gle_entries[1].debit, 93.33333333333333);
    assert_eq!(item.gle_entries[1].posted, false);
    assert_eq!(item.gle_entries[2].gle_posting_date, "2026-03-31");
}

#[test]
fn deferred_invoice_rolls_item_totals_into_invoice_period_totals() {
    let mut invoice = DeferredInvoice::new(
        "SINV-1",
        vec![
            revenue_entry("SERV-A", "2026-01-31", 100.0, 0.0, true),
            revenue_entry("SERV-B", "2026-02-28", 80.0, 0.0, true),
        ],
        filters(BookingBasis::Days, false),
        periods(),
    )
    .expect("invoice");

    invoice
        .calculate_invoice_revenue_expense_for_period()
        .expect("period totals");
    let rows = invoice.report_data();

    assert_eq!(invoice.period_total[0].total, 100.0);
    assert_eq!(invoice.period_total[1].total, 80.0);
    assert_eq!(rows[0].name, "SINV-1");
    assert_eq!(rows[0].amount, 600.0);
    assert_eq!(rows[1].indent, Some(1));
}

#[test]
fn deferred_report_execute_returns_columns_rows_total_and_chart() {
    let report = execute(
        filters(BookingBasis::Days, false),
        periods(),
        vec![
            revenue_entry("SERV-A", "2026-01-31", 100.0, 0.0, true),
            revenue_entry("SERV-A", "2026-02-28", 90.0, 0.0, true),
        ],
    )
    .expect("report");

    assert_eq!(
        report.columns[..4],
        [
            ReportColumn::new("Name", "name", "Data"),
            ReportColumn::new("Service Start Date", "service_start_date", "Date"),
            ReportColumn::new("Service End Date", "service_end_date", "Date"),
            ReportColumn::new("Amount", "amount", "Currency"),
        ]
    );
    assert_eq!(report.columns[4].fieldname, "jan_2026");
    assert_eq!(report.data[0].name, "SINV-1");
    assert_eq!(report.data.last().unwrap().name, "Total Deferred Income");
    assert_eq!(
        report.chart.data.labels,
        vec!["Jan 2026", "Feb 2026", "Mar 2026"]
    );
    assert_eq!(report.chart.data.datasets[0].name, "Actual Posting");
    assert_eq!(report.chart.data.datasets[0].values, vec![100.0, 90.0, 0.0]);
    assert_eq!(report.chart.data.datasets.len(), 1);
}

#[test]
fn deferred_report_filters_company_type_status_cancelled_and_service_window_like_erpnext_query() {
    let mut wrong_company = revenue_entry("WRONG-COMPANY", "2026-01-31", 999.0, 0.0, true);
    wrong_company.company = "Other Co".to_string();
    let mut draft = revenue_entry("DRAFT", "2026-01-31", 999.0, 0.0, true);
    draft.docstatus = 0;
    let mut cancelled = revenue_entry("CANCELLED", "2026-01-31", 999.0, 0.0, true);
    cancelled.is_cancelled = true;
    let mut not_deferred = revenue_entry("NOT-DEFERRED", "2026-01-31", 999.0, 0.0, true);
    not_deferred.enable_deferred_revenue = false;
    let wrong_type = expense_entry("EXPENSE", "2026-01-31", 0.0, 999.0, true);
    let mut outside_period = revenue_entry("OUTSIDE", "2025-12-31", 999.0, 0.0, true);
    outside_period.service_start_date = "2025-10-01".to_string();
    outside_period.service_end_date = "2025-12-31".to_string();
    let valid = revenue_entry("VALID", "2026-01-31", 100.0, 0.0, true);

    let report = execute(
        filters(BookingBasis::Days, false),
        periods(),
        vec![
            wrong_company,
            draft,
            cancelled,
            not_deferred,
            wrong_type,
            outside_period,
            valid,
        ],
    )
    .expect("report");

    assert_eq!(report.data[0].name, "SINV-1");
    assert_eq!(report.data[1].name, "VALID");
    assert_eq!(report.data[1].periods["jan_2026"], 100.0);
    assert_eq!(report.chart.data.datasets[0].values, vec![100.0, 0.0, 0.0]);
}

#[test]
fn deferred_report_expense_filter_keeps_only_deferred_purchase_entries() {
    let mut expense_filters = filters(BookingBasis::Days, true);
    expense_filters.deferred_type = DeferredType::Expense;
    let report = execute(
        expense_filters,
        periods(),
        vec![
            revenue_entry("REVENUE", "2026-01-31", 100.0, 0.0, true),
            expense_entry("EXPENSE", "2026-01-31", 0.0, 80.0, true),
        ],
    )
    .expect("report");

    assert_eq!(report.data[0].name, "PINV-1");
    assert_eq!(report.data[1].name, "EXPENSE");
    assert_eq!(report.data.last().unwrap().name, "Total Deferred Expense");
    assert_eq!(report.chart.data.datasets[0].values, vec![-80.0, 0.0, 0.0]);
    assert_eq!(report.chart.data.datasets[1].name, "Expected");
}
