use tokio_erp::erpnext::accounts::report::gross_profit::gross_profit::{
    calculate_row, get_column_names, get_columns, get_group_wise_columns, group_rows,
    GrossProfitFilters, GrossProfitSourceRow, MasterNameSettings, ReportCell, ReportColumn,
};

fn filters(group_by: &str) -> GrossProfitFilters {
    GrossProfitFilters {
        company: "_Test Company".to_string(),
        from_date: "2026-05-01".to_string(),
        to_date: "2026-05-31".to_string(),
        group_by: group_by.to_string(),
        currency: "USD".to_string(),
        currency_precision: 3,
        float_precision: 2,
    }
}

fn source_row(
    invoice: &str,
    item_code: &str,
    qty: f64,
    base_net_amount: f64,
    buying_amount: f64,
) -> GrossProfitSourceRow {
    GrossProfitSourceRow {
        parent: invoice.to_string(),
        invoice_or_item: item_code.to_string(),
        customer: "CUST-001".to_string(),
        customer_group: "Retail".to_string(),
        customer_name: "Customer One".to_string(),
        posting_date: "2026-05-15".to_string(),
        item_code: item_code.to_string(),
        item_name: format!("{item_code} name"),
        item_group: "Products".to_string(),
        brand: "Brand A".to_string(),
        description: "Item description".to_string(),
        warehouse: "Stores - TC".to_string(),
        qty,
        base_net_amount,
        buying_amount,
        project: "PROJ-001".to_string(),
        cost_center: "Main - TC".to_string(),
        territory: "North".to_string(),
        sales_person: "Sales User".to_string(),
        allocated_amount: 40.0,
        payment_term: "30 Days".to_string(),
        is_return: false,
        invoice_portion: 0.0,
        payment_amount: 0.0,
    }
}

#[test]
fn gross_profit_group_wise_columns_match_erpnext_registry() {
    let columns = get_group_wise_columns();

    assert_eq!(
        columns["invoice"],
        vec![
            "invoice_or_item",
            "customer",
            "customer_group",
            "customer_name",
            "posting_date",
            "item_code",
            "item_name",
            "item_group",
            "brand",
            "description",
            "warehouse",
            "qty",
            "base_rate",
            "buying_rate",
            "base_amount",
            "buying_amount",
            "gross_profit",
            "gross_profit_percent",
            "project",
        ]
    );
    assert_eq!(
        columns["sales_person"],
        vec![
            "sales_person",
            "allocated_amount",
            "qty",
            "base_rate",
            "buying_rate",
            "base_amount",
            "buying_amount",
            "gross_profit",
            "gross_profit_percent",
        ]
    );
    assert_eq!(
        columns["payment_term"],
        vec![
            "payment_term",
            "base_amount",
            "buying_amount",
            "gross_profit",
            "gross_profit_percent",
        ]
    );
}

#[test]
fn gross_profit_column_names_match_erpnext_output_fieldnames() {
    let names = get_column_names();

    assert_eq!(names["invoice_or_item"], "sales_invoice");
    assert_eq!(names["base_rate"], "avg._selling_rate");
    assert_eq!(names["buying_rate"], "valuation_rate");
    assert_eq!(names["base_amount"], "selling_amount");
    assert_eq!(names["gross_profit_percent"], "gross_profit_%");
}

#[test]
fn gross_profit_columns_skip_customer_name_for_customer_name_master_settings_like_erpnext() {
    let columns = get_columns(
        &filters("Customer"),
        &MasterNameSettings {
            supplier_master_name: "Supplier Name".to_string(),
            customer_master_name: "Customer Name".to_string(),
        },
    );

    assert_eq!(
        columns,
        vec![
            ReportColumn::link("Customer", "customer", "Customer", 100),
            ReportColumn::link("Customer Group", "customer_group", "Customer Group", 100),
            ReportColumn::float("Qty", "qty", 80),
            ReportColumn::currency("Avg. Selling Rate", "avg._selling_rate", "currency", 100),
            ReportColumn::currency("Valuation Rate", "valuation_rate", "currency", 100),
            ReportColumn::currency("Selling Amount", "selling_amount", "currency", 100),
            ReportColumn::currency("Buying Amount", "buying_amount", "currency", 100),
            ReportColumn::currency("Gross Profit", "gross_profit", "currency", 100),
            ReportColumn::percent("Gross Profit Percent", "gross_profit_%", 100),
            ReportColumn::hidden_link("Currency", "currency", "Currency"),
        ]
    );
}

#[test]
fn gross_profit_columns_keep_customer_name_when_master_settings_do_not_hide_it() {
    let columns = get_columns(&filters("Customer"), &MasterNameSettings::default());

    assert_eq!(
        columns[0],
        ReportColumn::link("Customer", "customer", "Customer", 100)
    );
    assert_eq!(
        columns[2],
        ReportColumn::data("Customer Name", "customer_name", 150)
    );
    assert_eq!(
        columns.last(),
        Some(&ReportColumn::hidden_link(
            "Currency", "currency", "Currency"
        ))
    );
}

#[test]
fn gross_profit_calculate_row_matches_erpnext_amount_rate_and_percent_formulas() {
    let row = calculate_row(
        &source_row("SINV-0001", "ITEM-001", 4.0, 500.1254, 320.4444),
        &filters("Item Code"),
    );

    assert_eq!(row.base_amount, 500.125);
    assert_eq!(row.buying_amount, 320.444);
    assert_eq!(row.base_rate, 125.03);
    assert_eq!(row.buying_rate, 80.11);
    assert_eq!(row.gross_profit, 179.681);
    assert_eq!(row.gross_profit_percent, 35.927);
}

#[test]
fn gross_profit_calculate_row_adds_abs_negative_buying_amount_like_erpnext() {
    let row = calculate_row(
        &source_row("SINV-RETURN", "ITEM-001", -2.0, -200.0, -120.0),
        &filters("Item Code"),
    );

    assert_eq!(row.gross_profit, -80.0);
    assert_eq!(row.gross_profit_percent, -40.0);
}

#[test]
fn gross_profit_group_rows_aggregates_by_group_and_appends_total_like_erpnext() {
    let rows = group_rows(
        &[
            source_row("SINV-0001", "ITEM-001", 2.0, 200.0, 120.0),
            source_row("SINV-0002", "ITEM-001", 3.0, 300.0, 150.0),
            source_row("SINV-0003", "ITEM-002", 1.0, 80.0, 100.0),
        ],
        &filters("Item Code"),
        &MasterNameSettings::default(),
    );

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0][0], ReportCell::Text("ITEM-001".to_string()));
    assert_eq!(rows[0][4], ReportCell::Number(5.0));
    assert_eq!(rows[0][5], ReportCell::Number(100.0));
    assert_eq!(rows[0][6], ReportCell::Number(54.0));
    assert_eq!(rows[0][7], ReportCell::Number(500.0));
    assert_eq!(rows[0][8], ReportCell::Number(270.0));
    assert_eq!(rows[0][9], ReportCell::Number(230.0));
    assert_eq!(rows[0][10], ReportCell::Number(46.0));

    let total = rows.last().unwrap();
    assert_eq!(total[0], ReportCell::Text("Total".to_string()));
    assert_eq!(total[7], ReportCell::Number(580.0));
    assert_eq!(total[8], ReportCell::Number(370.0));
    assert_eq!(total[9], ReportCell::Number(210.0));
    assert_eq!(total[10], ReportCell::Number(36.207));
    assert_eq!(total[11], ReportCell::Text("USD".to_string()));
}
