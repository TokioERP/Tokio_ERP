use tokio_erp::erpnext::accounts::report::gross_profit::gross_profit::{
    calculate_row, get_column_names, get_columns, get_delivery_notes_query_plan,
    get_group_wise_columns, get_last_purchase_rate_query_plan, get_product_bundle_query_plan,
    get_stock_ledger_query_plan, group_rows, prepare_invoice_query_plan,
    prepare_return_invoice_query_plan, AccountingDimensionFilter, GrossProfitFilters,
    GrossProfitSourceRow, MasterNameSettings, ReportCell, ReportColumn,
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
        include_returned_invoices: false,
        item_group: None,
        sales_person: None,
        sales_invoice: None,
        item_code: None,
        cost_center: Vec::new(),
        project: Vec::new(),
        warehouse: None,
        accounting_dimensions: Vec::new(),
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

#[test]
fn gross_profit_invoice_query_plan_matches_erpnext_base_selects_and_filters() {
    let plan = prepare_invoice_query_plan(&filters("Invoice"));

    assert_eq!(plan.source, "Sales Invoice");
    assert_eq!(plan.joins, vec!["Sales Invoice Item", "Item"]);
    assert_eq!(
        plan.conditions,
        vec![
            "sales_invoice.docstatus = 1",
            "sales_invoice.is_opening != Yes",
            "sales_invoice.company = _Test Company",
            "sales_invoice.posting_date >= 2026-05-01",
            "sales_invoice.posting_date <= 2026-05-31",
            "sales_invoice.is_return = 0",
        ]
    );
    assert!(plan.selects.contains(&"sales_invoice_item.parenttype"));
    assert!(plan.selects.contains(&"sales_invoice_item.base_net_amount"));
    assert_eq!(
        plan.order_by,
        vec![
            "sales_invoice.posting_date desc",
            "sales_invoice.posting_time desc"
        ]
    );
}

#[test]
fn gross_profit_invoice_query_plan_matches_sales_person_and_payment_term_branches() {
    let sales_person = GrossProfitFilters {
        group_by: "Sales Person".to_string(),
        sales_person: Some("SP-001".to_string()),
        ..filters("Sales Person")
    };
    let plan = prepare_invoice_query_plan(&sales_person);

    assert_eq!(plan.left_joins, vec!["Sales Team"]);
    assert!(plan.selects.contains(&"sales_team.sales_person"));
    assert!(plan.selects.contains(&"sales_team.allocated_percentage * sales_invoice_item.base_net_amount / 100 as allocated_amount"));
    assert!(plan.conditions.contains(
        &"exists sales_team where parent = sales_invoice.name and sales_person = SP-001"
            .to_string()
    ));

    let payment_term = prepare_invoice_query_plan(&filters("Payment Term"));
    assert_eq!(payment_term.left_joins, vec!["Payment Schedule"]);
    assert!(payment_term
        .selects
        .contains(&"case when sales_invoice.is_return = 1 then Sales Return else coalesce(payment_schedule.payment_term, No Terms) end as payment_term"));
    assert!(payment_term
        .selects
        .contains(&"payment_schedule.invoice_portion"));
    assert!(payment_term
        .selects
        .contains(&"payment_schedule.payment_amount"));
}

#[test]
fn gross_profit_invoice_query_plan_matches_optional_common_filters() {
    let plan = prepare_invoice_query_plan(&GrossProfitFilters {
        item_group: Some("Products".to_string()),
        sales_invoice: Some("SINV-0001".to_string()),
        item_code: Some("ITEM-001".to_string()),
        cost_center: vec!["Main - TC".to_string(), "Child - TC".to_string()],
        project: vec!["PROJ-001".to_string()],
        warehouse: Some("Stores - TC".to_string()),
        accounting_dimensions: vec![AccountingDimensionFilter {
            fieldname: "department".to_string(),
            values: vec!["Sales".to_string(), "Retail".to_string()],
        }],
        ..filters("Item Code")
    });

    assert!(plan
        .conditions
        .contains(&"item group condition for Products".to_string()));
    assert!(plan
        .conditions
        .contains(&"sales_invoice.name = SINV-0001".to_string()));
    assert!(plan
        .conditions
        .contains(&"sales_invoice_item.item_code = ITEM-001".to_string()));
    assert!(plan
        .conditions
        .contains(&"sales_invoice_item.cost_center in [Main - TC, Child - TC]".to_string()));
    assert!(plan
        .conditions
        .contains(&"sales_invoice_item.project in [PROJ-001]".to_string()));
    assert!(plan
        .conditions
        .contains(&"sales_invoice_item.department in [Sales, Retail]".to_string()));
    assert!(plan.conditions.contains(
        &"sales_invoice_item.warehouse in warehouse descendants of Stores - TC".to_string()
    ));
}

#[test]
fn gross_profit_return_invoice_query_plan_matches_erpnext_include_returned_branching() {
    let excluded = prepare_return_invoice_query_plan(
        &GrossProfitFilters {
            include_returned_invoices: true,
            ..filters("Invoice")
        },
        &["SINV-0001".to_string(), "SINV-0002".to_string()],
    );

    assert!(excluded.conditions.contains(
        &"(sales_invoice.is_return = 1 and sales_invoice.return_against is not null)".to_string()
    ));
    assert!(excluded
        .conditions
        .contains(&"sales_invoice.return_against not in [SINV-0001, SINV-0002]".to_string()));

    let included = prepare_invoice_query_plan(&GrossProfitFilters {
        include_returned_invoices: true,
        ..filters("Invoice")
    });
    assert!(included.conditions.contains(
        &"(sales_invoice.is_return = 0 or (sales_invoice.is_return = 1 and sales_invoice.return_against is null))".to_string()
    ));
}

#[test]
fn gross_profit_auxiliary_query_plans_match_erpnext_shapes() {
    assert_eq!(
        get_delivery_notes_query_plan(&["SINV-0001".to_string(), "SINV-0002".to_string()])
            .conditions,
        vec![
            "delivery_note_item.docstatus = 1",
            "delivery_note_item.against_sales_invoice in [SINV-0001, SINV-0002]",
            "delivery_note_item.si_detail is not null",
            "delivery_note_item.si_detail != ",
        ]
    );

    assert_eq!(
        get_product_bundle_query_plan().selects,
        vec![
            "packed_item.parenttype",
            "packed_item.parent",
            "packed_item.parent_item",
            "packed_item.item_code",
            "packed_item.warehouse",
            "-1 * packed_item.qty as total_qty",
            "packed_item.rate",
            "packed_item.rate * packed_item.qty as base_amount",
            "packed_item.parent_detail_docname",
            "packed_item.serial_and_batch_bundle",
        ]
    );

    let stock = get_stock_ledger_query_plan("ITEM-001", "Stores - TC", "_Test Company");
    assert_eq!(stock.source, "Stock Ledger Entry");
    assert_eq!(
        stock.order_by,
        vec![
            "stock_ledger_entry.item_code",
            "stock_ledger_entry.warehouse desc",
            "stock_ledger_entry.posting_datetime desc",
            "stock_ledger_entry.creation desc",
        ]
    );

    let last_purchase = get_last_purchase_rate_query_plan(
        "ITEM-001",
        Some("PROJ-001"),
        Some("Main - TC"),
        "2026-05-31",
    );
    assert!(last_purchase
        .conditions
        .contains(&"purchase_invoice_item.project = PROJ-001".to_string()));
    assert!(last_purchase
        .conditions
        .contains(&"purchase_invoice_item.cost_center = Main - TC".to_string()));
    assert_eq!(last_purchase.limit, Some(1));
}
