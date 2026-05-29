use tokio_erp::erpnext::accounts::report::gross_profit::gross_profit::{
    calculate_buying_amount_from_delivery_note, calculate_buying_amount_from_sle, calculate_row,
    get_average_buying_rate, get_bundle_item_row, get_buying_amount,
    get_buying_amount_from_product_bundle, get_buying_amount_from_so_dn_query_plan,
    get_column_names, get_columns, get_columns_for_grouped_by_invoice,
    get_data_when_grouped_by_invoice, get_data_when_not_grouped_by_invoice,
    get_delivery_notes_query_plan, get_group_wise_columns, get_grouped_by_invoice_total_row,
    get_invoice_row, get_last_purchase_rate_query_plan, get_product_bundle_query_plan,
    get_report_columns, get_returned_invoice_items_query_plan, get_stock_ledger_query_plan,
    group_delivery_notes, group_items_by_invoice, group_product_bundles,
    group_returned_invoice_items, group_rows, load_invoice_items_query_plans,
    load_non_stock_items_query_plan, prepare_delivered_by_supplier_purchase_query_plan,
    prepare_invoice_query_plan, prepare_return_invoice_query_plan, prepare_vouchers_to_ignore,
    process_gross_profit_rows, should_skip_row, update_return_invoices, AccountingDimensionFilter,
    DeliveryNoteLoadRow, DeliveryNoteSummary, GrossProfitBuyingAmountContext,
    GrossProfitBuyingAmountRow, GrossProfitFilters, GrossProfitInvoiceRow, GrossProfitProcessRow,
    GrossProfitSourceRow, IncomingRateCache, IncomingRateRequest, MasterNameSettings,
    PackedItemOverride, ProductBundleItem, ProductBundleLoadRow, ReportCell, ReportColumn,
    ReturnAdjustedRow, ReturnedInvoiceItem, StockLedgerCache, StockLedgerEntry,
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

fn invoice_item_row(invoice: &str, item_code: &str) -> GrossProfitInvoiceRow {
    GrossProfitInvoiceRow {
        parent_invoice: String::new(),
        parenttype: "Sales Invoice".to_string(),
        indent: 0.0,
        parent: Some(invoice.to_string()),
        invoice_or_item: item_code.to_string(),
        posting_date: "2026-05-15".to_string(),
        posting_time: "10:30:00".to_string(),
        project: "PROJ-001".to_string(),
        update_stock: false,
        customer: "CUST-001".to_string(),
        customer_group: "Retail".to_string(),
        customer_name: "Customer One".to_string(),
        item_code: Some(item_code.to_string()),
        item_name: Some(format!("{item_code} name")),
        description: Some("Item description".to_string()),
        warehouse: Some("Stores - TC".to_string()),
        item_group: Some("Products".to_string()),
        brand: Some("Brand A".to_string()),
        dn_detail: Some("DN-DETAIL-1".to_string()),
        delivery_note: Some("DN-0001".to_string()),
        qty: Some(2.0),
        item_row: Some(format!("{invoice}-{item_code}-ROW")),
        is_return: false,
        cost_center: "Main - TC".to_string(),
        base_net_amount: 200.0,
        invoice_base_net_total: 500.0,
        invoice: None,
        serial_and_batch_bundle: Some("SBB-001".to_string()),
    }
}

fn process_row(invoice: &str, item_code: Option<&str>, indent: f64) -> GrossProfitProcessRow {
    GrossProfitProcessRow {
        parent_invoice: if indent == 0.0 {
            String::new()
        } else {
            invoice.to_string()
        },
        parenttype: "Sales Invoice".to_string(),
        indent,
        parent: (indent != 0.0).then(|| invoice.to_string()),
        invoice_or_item: item_code.unwrap_or(invoice).to_string(),
        project: "PROJ-001".to_string(),
        customer: "CUST-001".to_string(),
        customer_group: "Retail".to_string(),
        customer_name: "Customer One".to_string(),
        posting_date: "2026-05-15".to_string(),
        monthly: String::new(),
        item_code: item_code.map(str::to_string),
        item_name: item_code.map(|code| format!("{code} name")),
        item_group: item_code.map(|_| "Products".to_string()),
        brand: item_code.map(|_| "Brand A".to_string()),
        description: item_code.map(|_| "Item description".to_string()),
        warehouse: item_code.map(|_| "Stores - TC".to_string()),
        qty: (indent != 0.0).then_some(2.0),
        item_row: item_code.map(|code| format!("{invoice}-{code}-ROW")),
        update_stock: false,
        dn_detail: None,
        delivery_note: None,
        delivered_by_supplier: false,
        base_net_amount: if indent == 0.0 { 0.0 } else { 200.0 },
        base_amount: 0.0,
        buying_amount: 0.0,
        buying_rate: None,
        base_rate: None,
        gross_profit: 0.0,
        gross_profit_percent: 0.0,
    }
}

fn buying_row(item_code: &str) -> GrossProfitBuyingAmountRow {
    GrossProfitBuyingAmountRow {
        item_code: item_code.to_string(),
        qty: 2.0,
        delivered_by_supplier: false,
        so_detail: None,
        sales_order: None,
        project: String::new(),
        cost_center: String::new(),
        update_stock: false,
        dn_detail: None,
        parenttype: "Sales Invoice".to_string(),
        parent: "SINV-0001".to_string(),
        invoice: None,
        delivery_note: None,
        item_row: Some("SINV-ROW-1".to_string()),
    }
}

fn buying_context() -> GrossProfitBuyingAmountContext {
    GrossProfitBuyingAmountContext {
        po_details: Vec::new(),
        delivered_purchase_amount: None,
        non_stock_items: Vec::new(),
        last_purchase_rate: None,
        stock_ledger_entries: Vec::new(),
        delivery_note: None,
        so_dn_incoming_rate: None,
        average_buying_rate: 50.0,
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
fn gross_profit_grouped_by_invoice_columns_match_erpnext_item_display_shape() {
    let columns = get_columns_for_grouped_by_invoice(
        &get_columns(&filters("Invoice"), &MasterNameSettings::default()),
        &MasterNameSettings::default(),
    );

    assert_eq!(
        columns[0],
        ReportColumn::link("Sales Invoice", "sales_invoice", "Item", 300)
    );
    assert_eq!(
        columns
            .iter()
            .map(|column| column.fieldname)
            .collect::<Vec<_>>(),
        vec![
            "sales_invoice",
            "customer",
            "customer_group",
            "customer_name",
            "posting_date",
            "item_group",
            "brand",
            "description",
            "warehouse",
            "qty",
            "avg._selling_rate",
            "valuation_rate",
            "selling_amount",
            "buying_amount",
            "gross_profit",
            "gross_profit_%",
            "project",
            "currency",
        ]
    );
}

#[test]
fn gross_profit_grouped_by_invoice_columns_delete_item_columns_after_customer_name_is_hidden() {
    let settings = MasterNameSettings {
        supplier_master_name: "Supplier Name".to_string(),
        customer_master_name: "Customer Name".to_string(),
    };
    let columns =
        get_columns_for_grouped_by_invoice(&get_columns(&filters("Invoice"), &settings), &settings);

    assert_eq!(
        columns
            .iter()
            .map(|column| column.fieldname)
            .collect::<Vec<_>>(),
        vec![
            "sales_invoice",
            "customer",
            "customer_group",
            "posting_date",
            "item_group",
            "brand",
            "description",
            "warehouse",
            "qty",
            "avg._selling_rate",
            "valuation_rate",
            "selling_amount",
            "buying_amount",
            "gross_profit",
            "gross_profit_%",
            "project",
            "currency",
        ]
    );
}

#[test]
fn gross_profit_report_columns_apply_invoice_branch_only_for_invoice_group_like_execute() {
    let invoice_columns = get_report_columns(&filters("Invoice"), &MasterNameSettings::default());
    assert_eq!(
        invoice_columns[0],
        ReportColumn::link("Sales Invoice", "sales_invoice", "Item", 300)
    );
    assert!(!invoice_columns
        .iter()
        .any(|column| column.fieldname == "item_code"));
    assert!(!invoice_columns
        .iter()
        .any(|column| column.fieldname == "item_name"));

    let item_columns = get_report_columns(&filters("Item Code"), &MasterNameSettings::default());
    assert_eq!(
        item_columns[0],
        ReportColumn::link("Item Code", "item_code", "Item", 100)
    );
    assert!(item_columns
        .iter()
        .any(|column| column.fieldname == "item_name"));
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
    assert_eq!(total[11], ReportCell::Empty);
}

#[test]
fn gross_profit_not_grouped_data_wrapper_matches_erpnext_data_path() {
    let source_rows = vec![
        source_row("SINV-0001", "ITEM-001", 2.0, 200.0, 120.0),
        source_row("SINV-0002", "ITEM-001", 3.0, 300.0, 150.0),
    ];
    let filters = filters("Item Code");
    let settings = MasterNameSettings::default();

    assert_eq!(
        get_data_when_not_grouped_by_invoice(&source_rows, &filters, &settings),
        group_rows(&source_rows, &filters, &settings)
    );
}

#[test]
fn gross_profit_group_rows_monthly_formats_posting_date_like_erpnext() {
    let mut may_one = source_row("SINV-0001", "ITEM-001", 2.0, 200.0, 120.0);
    may_one.posting_date = "2026-05-15".to_string();
    let mut may_two = source_row("SINV-0002", "ITEM-002", 3.0, 300.0, 150.0);
    may_two.posting_date = "2026-05-20".to_string();
    let mut june = source_row("SINV-0003", "ITEM-003", 1.0, 80.0, 40.0);
    june.posting_date = "2026-06-01".to_string();

    let rows = group_rows(
        &[may_one, may_two, june],
        &filters("Monthly"),
        &MasterNameSettings::default(),
    );

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0][0], ReportCell::Text("May 2026".to_string()));
    assert_eq!(rows[0][1], ReportCell::Number(5.0));
    assert_eq!(rows[0][2], ReportCell::Number(100.0));
    assert_eq!(rows[0][3], ReportCell::Number(54.0));
    assert_eq!(rows[0][4], ReportCell::Number(500.0));
    assert_eq!(rows[0][5], ReportCell::Number(270.0));
    assert_eq!(rows[0][6], ReportCell::Number(230.0));
    assert_eq!(rows[0][7], ReportCell::Number(46.0));

    assert_eq!(rows[1][0], ReportCell::Text("Jun 2026".to_string()));
    assert_eq!(rows[1][4], ReportCell::Number(80.0));
    assert_eq!(rows[1][5], ReportCell::Number(40.0));
    assert_eq!(rows[1][7], ReportCell::Number(50.0));

    assert_eq!(rows[2][0], ReportCell::Text("Total".to_string()));
    assert_eq!(rows[2][4], ReportCell::Number(580.0));
    assert_eq!(rows[2][5], ReportCell::Number(310.0));
    assert_eq!(rows[2][6], ReportCell::Number(270.0));
    assert_eq!(rows[2][7], ReportCell::Number(46.552));
    assert_eq!(rows[2][8], ReportCell::Empty);
}

#[test]
fn gross_profit_invoice_header_row_matches_erpnext_get_invoice_row_shape() {
    let row = get_invoice_row(&invoice_item_row("SINV-0001", "ITEM-001"));

    assert_eq!(row.parent_invoice, "");
    assert_eq!(row.indent, 0.0);
    assert_eq!(row.invoice_or_item, "SINV-0001");
    assert_eq!(row.parent, None);
    assert_eq!(row.item_code, None);
    assert_eq!(row.item_name, None);
    assert_eq!(row.description, None);
    assert_eq!(row.warehouse, None);
    assert_eq!(row.item_group, None);
    assert_eq!(row.brand, None);
    assert_eq!(row.dn_detail, None);
    assert_eq!(row.delivery_note, None);
    assert_eq!(row.qty, None);
    assert_eq!(row.item_row, None);
    assert_eq!(row.base_net_amount, 500.0);
}

#[test]
fn gross_profit_bundle_item_row_matches_erpnext_fallbacks_and_negative_qty() {
    let mut row = invoice_item_row("SINV-0001", "BUNDLE-001");
    row.indent = 1.0;
    let bundle = ProductBundleItem {
        item_code: "COMP-001".to_string(),
        item_name: "Component".to_string(),
        description: "Component desc".to_string(),
        warehouse: None,
        total_qty: -3.0,
        parent_detail_docname: "SINV-0001-BUNDLE-001-ROW".to_string(),
        serial_and_batch_bundle: Some("SBB-COMP".to_string()),
    };

    let bundle_row = get_bundle_item_row(&row, &bundle);

    assert_eq!(bundle_row.parent_invoice, "BUNDLE-001");
    assert_eq!(bundle_row.parenttype, "Sales Invoice");
    assert_eq!(bundle_row.indent, 2.0);
    assert_eq!(bundle_row.parent, None);
    assert_eq!(bundle_row.invoice_or_item, "COMP-001");
    assert_eq!(bundle_row.warehouse.as_deref(), Some("Stores - TC"));
    assert_eq!(bundle_row.item_group.as_deref(), Some(""));
    assert_eq!(bundle_row.brand.as_deref(), Some(""));
    assert_eq!(bundle_row.qty, Some(3.0));
    assert_eq!(
        bundle_row.item_row.as_deref(),
        Some("SINV-0001-BUNDLE-001-ROW")
    );
    assert_eq!(bundle_row.invoice.as_deref(), Some("SINV-0001"));
    assert_eq!(
        bundle_row.serial_and_batch_bundle.as_deref(),
        Some("SBB-001")
    );
}

#[test]
fn gross_profit_group_items_by_invoice_inserts_header_children_and_bundle_rows_like_erpnext() {
    let rows = vec![
        invoice_item_row("SINV-0001", "BUNDLE-001"),
        invoice_item_row("SINV-0001", "ITEM-002"),
        invoice_item_row("SINV-0002", "ITEM-003"),
    ];
    let bundle = ProductBundleItem {
        item_code: "COMP-001".to_string(),
        item_name: "Component".to_string(),
        description: "Component desc".to_string(),
        warehouse: Some("Components - TC".to_string()),
        total_qty: -2.0,
        parent_detail_docname: "SINV-0001-BUNDLE-001-ROW".to_string(),
        serial_and_batch_bundle: None,
    };

    let grouped = group_items_by_invoice(&rows, &[("SINV-0001", "BUNDLE-001", vec![bundle])]);

    assert_eq!(grouped.len(), 6);
    assert_eq!(grouped[0].invoice_or_item, "SINV-0001");
    assert_eq!(grouped[0].indent, 0.0);
    assert_eq!(grouped[1].invoice_or_item, "BUNDLE-001");
    assert_eq!(grouped[1].indent, 1.0);
    assert_eq!(grouped[1].parent_invoice, "SINV-0001");
    assert_eq!(grouped[2].invoice_or_item, "COMP-001");
    assert_eq!(grouped[2].parent_invoice, "BUNDLE-001");
    assert_eq!(grouped[3].invoice_or_item, "ITEM-002");
    assert_eq!(grouped[4].invoice_or_item, "SINV-0002");
    assert_eq!(grouped[5].invoice_or_item, "ITEM-003");
}

#[test]
fn gross_profit_skip_row_matches_erpnext_non_invoice_blank_group_behavior() {
    let mut row = calculate_row(
        &source_row("SINV-0001", "ITEM-001", 1.0, 100.0, 70.0),
        &filters("Item Code"),
    );
    row.brand.clear();

    assert!(should_skip_row(&row, "Brand"));
    assert!(!should_skip_row(&row, "Invoice"));
    assert!(!should_skip_row(&row, "Item Code"));
}

#[test]
fn gross_profit_payment_term_grouping_applies_invoice_portion_like_erpnext() {
    let mut row1 = source_row("SINV-0001", "ITEM-001", 1.0, 100.0, 60.0);
    row1.payment_term = "30 Days".to_string();
    row1.invoice_portion = 50.0;

    let mut row2 = source_row("SINV-0002", "ITEM-002", 2.0, 200.0, 100.0);
    row2.payment_term = "30 Days".to_string();
    row2.payment_amount = 50.0;

    let mut row3 = source_row("SINV-RET", "ITEM-003", -1.0, -80.0, -40.0);
    row3.payment_term = "Sales Return".to_string();
    row3.is_return = true;

    let rows = group_rows(
        &[row1, row2, row3],
        &filters("Payment Term"),
        &MasterNameSettings::default(),
    );

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0][0], ReportCell::Text("30 Days".to_string()));
    assert_eq!(rows[0][1], ReportCell::Number(100.0));
    assert_eq!(rows[0][2], ReportCell::Number(55.0));
    assert_eq!(rows[0][3], ReportCell::Number(45.0));
    assert_eq!(rows[0][4], ReportCell::Number(45.0));

    assert_eq!(rows[1][0], ReportCell::Text("Sales Return".to_string()));
    assert_eq!(rows[1][1], ReportCell::Number(-80.0));
    assert_eq!(rows[1][2], ReportCell::Number(-40.0));
    assert_eq!(rows[1][3], ReportCell::Number(-40.0));
    assert_eq!(rows[1][4], ReportCell::Number(-50.0));
}

#[test]
fn gross_profit_update_return_invoices_consumes_matching_return_rows_like_erpnext() {
    let mut row = ReturnAdjustedRow {
        parent: "SINV-0001".to_string(),
        item_code: "ITEM-001".to_string(),
        qty: 5.0,
        base_amount: 500.0,
        buying_rate: 60.0,
        buying_amount: 300.0,
        delivered_by_supplier: false,
    };
    let mut returned = vec![
        ReturnedInvoiceItem {
            return_against: "SINV-0001".to_string(),
            item_code: "ITEM-001".to_string(),
            qty: -2.0,
            base_amount: -180.0,
        },
        ReturnedInvoiceItem {
            return_against: "SINV-0002".to_string(),
            item_code: "ITEM-001".to_string(),
            qty: -1.0,
            base_amount: -90.0,
        },
    ];

    update_return_invoices(&mut row, &mut returned, 3);

    assert_eq!(row.qty, 3.0);
    assert_eq!(row.base_amount, 320.0);
    assert_eq!(row.buying_amount, 180.0);
    assert_eq!(returned[0].qty, 0.0);
    assert_eq!(returned[0].base_amount, 0.0);
    assert_eq!(returned[1].qty, -1.0);
}

#[test]
fn gross_profit_update_return_invoices_preserves_delivered_by_supplier_buying_amount() {
    let mut row = ReturnAdjustedRow {
        parent: "SINV-0001".to_string(),
        item_code: "ITEM-001".to_string(),
        qty: 1.0,
        base_amount: 100.0,
        buying_rate: 60.0,
        buying_amount: 999.0,
        delivered_by_supplier: true,
    };
    let mut returned = vec![ReturnedInvoiceItem {
        return_against: "SINV-0001".to_string(),
        item_code: "ITEM-001".to_string(),
        qty: -3.0,
        base_amount: -300.0,
    }];

    update_return_invoices(&mut row, &mut returned, 3);

    assert_eq!(row.qty, 0.0);
    assert_eq!(row.base_amount, 0.0);
    assert_eq!(row.buying_amount, 999.0);
    assert_eq!(returned[0].qty, -3.0);
    assert_eq!(returned[0].base_amount, -300.0);
}

#[test]
fn gross_profit_calculate_buying_amount_from_sle_matches_stock_value_delta_and_average_fallback() {
    let sles = vec![
        StockLedgerEntry {
            voucher_type: "Delivery Note".to_string(),
            voucher_no: "DN-0001".to_string(),
            voucher_detail_no: "DN-ROW-1".to_string(),
            stock_value: 900.0,
            qty: -3.0,
        },
        StockLedgerEntry {
            voucher_type: "Delivery Note".to_string(),
            voucher_no: "DN-0000".to_string(),
            voucher_detail_no: "OTHER".to_string(),
            stock_value: 600.0,
            qty: -2.0,
        },
    ];

    assert_eq!(
        calculate_buying_amount_from_sle(2.0, &sles, "Delivery Note", "DN-0001", "DN-ROW-1", 45.0),
        200.0
    );

    let no_previous = vec![StockLedgerEntry {
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: "SINV-0001".to_string(),
        voucher_detail_no: "SINV-ROW-1".to_string(),
        stock_value: 500.0,
        qty: -2.0,
    }];
    assert_eq!(
        calculate_buying_amount_from_sle(
            2.0,
            &no_previous,
            "Sales Invoice",
            "SINV-0001",
            "SINV-ROW-1",
            45.0
        ),
        90.0
    );
}

#[test]
fn gross_profit_delivery_note_buying_amount_matches_incoming_value_or_average_rate() {
    assert_eq!(
        calculate_buying_amount_from_delivery_note(
            3.0,
            &DeliveryNoteSummary {
                total_qty: 6.0,
                total_incoming_value: 240.0,
            },
            75.0
        ),
        120.0
    );
    assert_eq!(
        calculate_buying_amount_from_delivery_note(
            3.0,
            &DeliveryNoteSummary {
                total_qty: 0.0,
                total_incoming_value: 0.0,
            },
            75.0
        ),
        225.0
    );
}

#[test]
fn gross_profit_product_bundle_buying_amount_sums_matching_parent_detail_rows() {
    let bundle = vec![
        ProductBundleItem {
            item_code: "COMP-001".to_string(),
            item_name: "Component".to_string(),
            description: "Component desc".to_string(),
            warehouse: Some("Stores - TC".to_string()),
            total_qty: -2.0,
            parent_detail_docname: "ROW-1".to_string(),
            serial_and_batch_bundle: None,
        },
        ProductBundleItem {
            item_code: "COMP-002".to_string(),
            item_name: "Component 2".to_string(),
            description: "Component desc 2".to_string(),
            warehouse: Some("Stores - TC".to_string()),
            total_qty: -3.0,
            parent_detail_docname: "ROW-1".to_string(),
            serial_and_batch_bundle: None,
        },
        ProductBundleItem {
            item_code: "COMP-IGNORED".to_string(),
            item_name: "Ignored".to_string(),
            description: "Ignored".to_string(),
            warehouse: Some("Stores - TC".to_string()),
            total_qty: -5.0,
            parent_detail_docname: "OTHER-ROW".to_string(),
            serial_and_batch_bundle: None,
        },
    ];

    assert_eq!(
        get_buying_amount_from_product_bundle(
            "ROW-1",
            &bundle,
            &[
                ("COMP-001", 10.0),
                ("COMP-002", 20.0),
                ("COMP-IGNORED", 100.0)
            ],
            3
        ),
        80.0
    );
}

#[test]
fn gross_profit_buying_amount_query_plans_match_erpnext_delivered_supplier_and_so_dn() {
    let delivered = prepare_delivered_by_supplier_purchase_query_plan(&[
        "PO-ITEM-1".to_string(),
        "PO-ITEM-2".to_string(),
    ]);
    assert_eq!(delivered.source, "Purchase Invoice Item");
    assert_eq!(
        delivered.selects,
        vec!["sum(purchase_invoice_item.qty * purchase_invoice_item.base_net_rate)"]
    );
    assert_eq!(
        delivered.conditions,
        vec![
            "purchase_invoice_item.po_detail in [PO-ITEM-1, PO-ITEM-2]",
            "purchase_invoice_item.docstatus = 1",
        ]
    );

    let so_dn = get_buying_amount_from_so_dn_query_plan("SO-0001", "SO-ROW-1", "ITEM-001");
    assert_eq!(so_dn.source, "Delivery Note Item");
    assert_eq!(so_dn.selects, vec!["avg(delivery_note_item.incoming_rate)"]);
    assert_eq!(
        so_dn.conditions,
        vec![
            "delivery_note_item.docstatus = 1",
            "delivery_note_item.item_code = ITEM-001",
            "delivery_note_item.against_sales_order = SO-0001",
            "delivery_note_item.so_detail = SO-ROW-1",
        ]
    );
    assert_eq!(so_dn.group_by, vec!["delivery_note_item.item_code"]);
}

#[test]
fn gross_profit_get_buying_amount_uses_delivered_by_supplier_purchase_invoice_first() {
    let row = GrossProfitBuyingAmountRow {
        delivered_by_supplier: true,
        so_detail: Some("SO-ROW-1".to_string()),
        ..buying_row("ITEM-001")
    };
    let context = GrossProfitBuyingAmountContext {
        po_details: vec!["PO-ITEM-1".to_string()],
        delivered_purchase_amount: Some(425.0),
        average_buying_rate: 10.0,
        ..buying_context()
    };

    assert_eq!(get_buying_amount(&row, &context), 425.0);
}

#[test]
fn gross_profit_get_buying_amount_uses_last_purchase_rate_for_non_stock_project_or_cost_center() {
    let row = GrossProfitBuyingAmountRow {
        project: "PROJ-001".to_string(),
        qty: 3.0,
        ..buying_row("SERVICE-001")
    };
    let context = GrossProfitBuyingAmountContext {
        non_stock_items: vec!["SERVICE-001".to_string()],
        last_purchase_rate: Some(27.5),
        average_buying_rate: 99.0,
        ..buying_context()
    };

    assert_eq!(get_buying_amount(&row, &context), 82.5);
}

#[test]
fn gross_profit_get_buying_amount_prefers_sle_and_switches_to_delivery_note_parent_for_dn_detail() {
    let row = GrossProfitBuyingAmountRow {
        update_stock: false,
        dn_detail: Some("DN-ROW-1".to_string()),
        delivery_note: Some("DN-0001".to_string()),
        item_row: Some("DN-ROW-1".to_string()),
        qty: 2.0,
        ..buying_row("ITEM-001")
    };
    let context = GrossProfitBuyingAmountContext {
        stock_ledger_entries: vec![
            StockLedgerEntry {
                voucher_type: "Delivery Note".to_string(),
                voucher_no: "DN-0001".to_string(),
                voucher_detail_no: "DN-ROW-1".to_string(),
                stock_value: 900.0,
                qty: -3.0,
            },
            StockLedgerEntry {
                voucher_type: "Delivery Note".to_string(),
                voucher_no: "DN-0000".to_string(),
                voucher_detail_no: "OLD".to_string(),
                stock_value: 600.0,
                qty: -1.0,
            },
        ],
        average_buying_rate: 99.0,
        ..buying_context()
    };

    assert_eq!(get_buying_amount(&row, &context), 200.0);
}

#[test]
fn gross_profit_get_buying_amount_matches_delivery_note_so_dn_and_average_fallbacks() {
    let delivery_note = GrossProfitBuyingAmountContext {
        delivery_note: Some(DeliveryNoteSummary {
            total_qty: 6.0,
            total_incoming_value: 240.0,
        }),
        average_buying_rate: 99.0,
        ..buying_context()
    };
    assert_eq!(
        get_buying_amount(&buying_row("ITEM-001"), &delivery_note),
        80.0
    );

    let so_dn_row = GrossProfitBuyingAmountRow {
        sales_order: Some("SO-0001".to_string()),
        so_detail: Some("SO-ROW-1".to_string()),
        ..buying_row("ITEM-001")
    };
    let so_dn = GrossProfitBuyingAmountContext {
        so_dn_incoming_rate: Some(37.5),
        average_buying_rate: 99.0,
        ..buying_context()
    };
    assert_eq!(get_buying_amount(&so_dn_row, &so_dn), 75.0);

    assert_eq!(
        get_buying_amount(&buying_row("ITEM-001"), &buying_context()),
        100.0
    );
}

#[test]
fn gross_profit_average_buying_rate_caches_by_item_and_warehouse_like_erpnext() {
    let mut cache = IncomingRateCache::default();
    let request = IncomingRateRequest {
        item_code: "ITEM-001".to_string(),
        warehouse: "Stores - TC".to_string(),
        parenttype: "Sales Invoice".to_string(),
        parent: "SINV-0001".to_string(),
        company: "_Test Company".to_string(),
        serial_and_batch_bundle: Some("SBB-001".to_string()),
    };

    assert_eq!(
        get_average_buying_rate(&mut cache, &request, 42.5555, 3),
        42.556
    );
    assert_eq!(
        get_average_buying_rate(&mut cache, &request, 99.0, 3),
        42.556
    );
    assert_eq!(
        cache.rates[&("ITEM-001".to_string(), "Stores - TC".to_string())],
        42.556
    );
    assert_eq!(cache.requests, vec![request]);
}

#[test]
fn gross_profit_process_rows_sets_invoice_header_from_child_totals_like_erpnext() {
    let rows = vec![
        process_row("SINV-0001", None, 0.0),
        process_row("SINV-0001", Some("ITEM-001"), 1.0),
        GrossProfitProcessRow {
            base_net_amount: 150.0,
            qty: Some(3.0),
            ..process_row("SINV-0001", Some("ITEM-002"), 1.0)
        },
    ];

    let processed = process_gross_profit_rows(
        &rows,
        &filters("Invoice"),
        &[
            ("SINV-0001-ITEM-001-ROW", 120.0),
            ("SINV-0001-ITEM-002-ROW", 75.0),
        ],
        &[],
        &mut [],
    );

    assert_eq!(processed[0].base_amount, 350.0);
    assert_eq!(processed[0].buying_amount, 195.0);
    assert_eq!(processed[0].gross_profit, 155.0);
    assert_eq!(processed[0].gross_profit_percent, 44.286);
    assert_eq!(processed[0].base_rate, None);
    assert_eq!(processed[0].buying_rate, None);

    assert_eq!(processed[1].base_rate, Some(100.0));
    assert_eq!(processed[1].buying_rate, Some(60.0));
    assert_eq!(processed[2].base_rate, Some(50.0));
    assert_eq!(processed[2].buying_rate, Some(25.0));
}

#[test]
fn gross_profit_process_rows_applies_return_adjustment_before_invoice_header_total() {
    let rows = vec![
        process_row("SINV-0001", None, 0.0),
        GrossProfitProcessRow {
            base_net_amount: 500.0,
            qty: Some(5.0),
            ..process_row("SINV-0001", Some("ITEM-001"), 1.0)
        },
    ];
    let mut returns = vec![ReturnedInvoiceItem {
        return_against: "SINV-0001".to_string(),
        item_code: "ITEM-001".to_string(),
        qty: -2.0,
        base_amount: -180.0,
    }];

    let processed = process_gross_profit_rows(
        &rows,
        &filters("Invoice"),
        &[("SINV-0001-ITEM-001-ROW", 300.0)],
        &[],
        &mut returns,
    );

    assert_eq!(processed[1].qty, Some(3.0));
    assert_eq!(processed[1].base_amount, 320.0);
    assert_eq!(processed[1].buying_amount, 180.0);
    assert_eq!(processed[1].gross_profit, 140.0);
    assert_eq!(processed[0].base_amount, 320.0);
    assert_eq!(processed[0].buying_amount, 180.0);
    assert_eq!(returns[0].qty, 0.0);
}

#[test]
fn gross_profit_process_rows_applies_delivery_note_packed_item_override_like_erpnext() {
    let row = GrossProfitProcessRow {
        parent_invoice: "BUNDLE-001".to_string(),
        parent: None,
        invoice_or_item: "COMP-001".to_string(),
        item_code: Some("COMP-001".to_string()),
        item_row: Some("OLD-ROW".to_string()),
        dn_detail: Some("DN-ROW-1".to_string()),
        delivery_note: Some("DN-0001".to_string()),
        qty: Some(1.0),
        base_net_amount: 0.0,
        ..process_row("SINV-0001", Some("COMP-001"), 2.0)
    };

    let processed = process_gross_profit_rows(
        &[row],
        &filters("Invoice"),
        &[("DN-ROW-1", 45.0)],
        &[PackedItemOverride {
            parent_invoice: "BUNDLE-001".to_string(),
            item_code: "COMP-001".to_string(),
            parent_detail_docname: "DN-ROW-1".to_string(),
            warehouse: "Packed - TC".to_string(),
            base_amount: 75.5556,
        }],
        &mut [],
    );

    assert_eq!(processed[0].item_row.as_deref(), Some("DN-ROW-1"));
    assert_eq!(processed[0].warehouse.as_deref(), Some("Packed - TC"));
    assert_eq!(processed[0].base_amount, 75.556);
    assert_eq!(processed[0].buying_amount, 45.0);
    assert_eq!(processed[0].gross_profit, 30.556);
}

#[test]
fn gross_profit_grouped_by_invoice_total_row_sums_indent_one_rows_like_erpnext() {
    let rows = vec![
        GrossProfitProcessRow {
            base_amount: 999.0,
            buying_amount: 888.0,
            ..process_row("SINV-0001", None, 0.0)
        },
        GrossProfitProcessRow {
            base_amount: 200.0,
            buying_amount: 120.0,
            ..process_row("SINV-0001", Some("ITEM-001"), 1.0)
        },
        GrossProfitProcessRow {
            base_amount: 90.0,
            buying_amount: -30.0,
            ..process_row("SINV-0001", Some("COMP-001"), 2.0)
        },
        GrossProfitProcessRow {
            base_amount: 150.0,
            buying_amount: 75.0,
            ..process_row("SINV-0002", Some("ITEM-002"), 1.0)
        },
    ];

    assert_eq!(
        get_grouped_by_invoice_total_row(&rows, &filters("Invoice")),
        vec![
            ReportCell::Text("Total".to_string()),
            ReportCell::Empty,
            ReportCell::Empty,
            ReportCell::Number(350.0),
            ReportCell::Number(195.0),
            ReportCell::Number(155.0),
            ReportCell::Number(44.286),
        ]
    );
}

#[test]
fn gross_profit_grouped_by_invoice_data_rows_map_column_names_like_erpnext() {
    let processed = vec![
        GrossProfitProcessRow {
            base_amount: 350.0,
            buying_amount: 195.0,
            gross_profit: 155.0,
            gross_profit_percent: 44.286,
            ..process_row("SINV-0001", None, 0.0)
        },
        GrossProfitProcessRow {
            base_amount: 200.0,
            buying_amount: 120.0,
            buying_rate: Some(60.0),
            base_rate: Some(100.0),
            gross_profit: 80.0,
            gross_profit_percent: 40.0,
            ..process_row("SINV-0001", Some("ITEM-001"), 1.0)
        },
    ];

    let data = get_data_when_grouped_by_invoice(&processed, &filters("Invoice"));

    assert_eq!(data[0]["indent"], ReportCell::Number(0.0));
    assert_eq!(data[0]["parent_invoice"], ReportCell::Text(String::new()));
    assert_eq!(data[0]["currency"], ReportCell::Text("USD".to_string()));
    assert_eq!(
        data[0]["sales_invoice"],
        ReportCell::Text("SINV-0001".to_string())
    );
    assert_eq!(
        data[0]["customer"],
        ReportCell::Text("CUST-001".to_string())
    );
    assert_eq!(data[0]["project"], ReportCell::Text("PROJ-001".to_string()));
    assert_eq!(data[0]["selling_amount"], ReportCell::Number(350.0));

    assert_eq!(data[1]["indent"], ReportCell::Number(1.0));
    assert_eq!(
        data[1]["parent_invoice"],
        ReportCell::Text("SINV-0001".to_string())
    );
    assert_eq!(
        data[1]["item_code"],
        ReportCell::Text("ITEM-001".to_string())
    );
    assert_eq!(data[1]["avg._selling_rate"], ReportCell::Number(100.0));
    assert_eq!(data[1]["valuation_rate"], ReportCell::Number(60.0));

    let total = data.last().expect("total row");
    assert_eq!(
        total["sales_invoice"],
        ReportCell::Text("Total".to_string())
    );
    assert_eq!(total["selling_amount"], ReportCell::Number(200.0));
    assert_eq!(total["buying_amount"], ReportCell::Number(120.0));
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
fn gross_profit_load_invoice_items_query_plans_match_erpnext_normal_then_return_queries() {
    let rows = vec![
        invoice_item_row("SINV-0001", "ITEM-001"),
        invoice_item_row("SINV-0002", "ITEM-002"),
    ];

    let plans = load_invoice_items_query_plans(&filters("Invoice"), &rows);

    assert!(plans
        .normal
        .conditions
        .contains(&"sales_invoice.is_return = 0".to_string()));
    assert!(plans.returns.conditions.contains(
        &"(sales_invoice.is_return = 1 and sales_invoice.return_against is not null)".to_string()
    ));
    assert!(plans
        .returns
        .conditions
        .contains(&"sales_invoice.return_against not in [SINV-0001, SINV-0002]".to_string()));
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

#[test]
fn gross_profit_stock_ledger_entries_cache_and_blank_guard_match_erpnext() {
    let mut cache = StockLedgerCache::default();
    let entries = vec![StockLedgerEntry {
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: "SINV-0001".to_string(),
        voucher_detail_no: "SINV-ROW-1".to_string(),
        stock_value: 100.0,
        qty: -1.0,
    }];

    assert!(cache
        .get_entries("", "Stores - TC", "_Test Company", &entries)
        .is_empty());
    assert_eq!(
        cache.get_entries("ITEM-001", "Stores - TC", "_Test Company", &entries),
        entries
    );
    assert_eq!(
        cache.get_entries("ITEM-001", "Stores - TC", "_Test Company", &[]),
        entries
    );
    assert_eq!(cache.requests.len(), 1);
    assert_eq!(
        cache.requests[0].conditions[1],
        "stock_ledger_entry.item_code = ITEM-001"
    );
}

#[test]
fn gross_profit_load_non_stock_items_query_plan_matches_erpnext_sql_list() {
    let plan = load_non_stock_items_query_plan();

    assert_eq!(plan.source, "Item");
    assert_eq!(plan.selects, vec!["item.name"]);
    assert_eq!(plan.conditions, vec!["item.is_stock_item = 0"]);
}

#[test]
fn gross_profit_group_product_bundles_nests_parenttype_parent_and_parent_item_like_erpnext() {
    let rows = vec![
        ProductBundleLoadRow {
            parenttype: "Sales Invoice".to_string(),
            parent: "SINV-0001".to_string(),
            parent_item: "BUNDLE-001".to_string(),
            item: ProductBundleItem {
                item_code: "COMP-001".to_string(),
                item_name: "Component 1".to_string(),
                description: "Component 1 desc".to_string(),
                warehouse: Some("Stores - TC".to_string()),
                total_qty: 2.0,
                parent_detail_docname: "ROW-1".to_string(),
                serial_and_batch_bundle: Some("SBB-1".to_string()),
            },
        },
        ProductBundleLoadRow {
            parenttype: "Sales Invoice".to_string(),
            parent: "SINV-0001".to_string(),
            parent_item: "BUNDLE-001".to_string(),
            item: ProductBundleItem {
                item_code: "COMP-002".to_string(),
                item_name: "Component 2".to_string(),
                description: "Component 2 desc".to_string(),
                warehouse: Some("Stores - TC".to_string()),
                total_qty: 3.0,
                parent_detail_docname: "ROW-1".to_string(),
                serial_and_batch_bundle: None,
            },
        },
        ProductBundleLoadRow {
            parenttype: "Delivery Note".to_string(),
            parent: "DN-0001".to_string(),
            parent_item: "BUNDLE-002".to_string(),
            item: ProductBundleItem {
                item_code: "COMP-003".to_string(),
                item_name: "Component 3".to_string(),
                description: "Component 3 desc".to_string(),
                warehouse: Some("Transit - TC".to_string()),
                total_qty: 4.0,
                parent_detail_docname: "DN-ROW-1".to_string(),
                serial_and_batch_bundle: None,
            },
        },
    ];

    let grouped = group_product_bundles(&rows);

    assert_eq!(
        grouped["Sales Invoice"]["SINV-0001"]["BUNDLE-001"]
            .iter()
            .map(|item| item.item_code.as_str())
            .collect::<Vec<_>>(),
        vec!["COMP-001", "COMP-002"]
    );
    assert_eq!(
        grouped["Delivery Note"]["DN-0001"]["BUNDLE-002"][0]
            .warehouse
            .as_deref(),
        Some("Transit - TC")
    );
}

#[test]
fn gross_profit_group_delivery_notes_maps_si_detail_to_summary_like_erpnext() {
    let grouped = group_delivery_notes(&[
        DeliveryNoteLoadRow {
            si_detail: "SINV-ROW-1".to_string(),
            total_qty: 2.0,
            total_incoming_value: 120.0,
        },
        DeliveryNoteLoadRow {
            si_detail: "SINV-ROW-2".to_string(),
            total_qty: 5.0,
            total_incoming_value: 250.0,
        },
    ]);

    assert_eq!(
        grouped["SINV-ROW-1"],
        DeliveryNoteSummary {
            total_qty: 2.0,
            total_incoming_value: 120.0,
        }
    );
    assert_eq!(
        grouped["SINV-ROW-2"],
        DeliveryNoteSummary {
            total_qty: 5.0,
            total_incoming_value: 250.0,
        }
    );
}

#[test]
fn gross_profit_returned_invoice_items_query_plan_matches_erpnext_sql_shape() {
    let plan = get_returned_invoice_items_query_plan(&filters("Invoice"));

    assert_eq!(plan.source, "Sales Invoice");
    assert_eq!(plan.joins, vec!["Sales Invoice Item"]);
    assert_eq!(
        plan.selects,
        vec![
            "sales_invoice.name",
            "sales_invoice_item.item_code",
            "sales_invoice_item.stock_qty as qty",
            "sales_invoice_item.base_net_amount as base_amount",
            "sales_invoice.return_against",
        ]
    );
    assert_eq!(
        plan.conditions,
        vec![
            "sales_invoice.name = sales_invoice_item.parent",
            "sales_invoice.docstatus = 1",
            "sales_invoice.is_return = 1",
            "sales_invoice.posting_date between 2026-05-01 and 2026-05-31",
        ]
    );
}

#[test]
fn gross_profit_group_returned_invoice_items_nests_return_against_and_item_code_like_erpnext() {
    let returned = vec![
        ReturnedInvoiceItem {
            return_against: "SINV-0001".to_string(),
            item_code: "ITEM-001".to_string(),
            qty: -1.0,
            base_amount: -100.0,
        },
        ReturnedInvoiceItem {
            return_against: "SINV-0001".to_string(),
            item_code: "ITEM-001".to_string(),
            qty: -2.0,
            base_amount: -180.0,
        },
        ReturnedInvoiceItem {
            return_against: "SINV-0002".to_string(),
            item_code: "ITEM-002".to_string(),
            qty: -3.0,
            base_amount: -240.0,
        },
    ];

    let grouped = group_returned_invoice_items(&returned);

    assert_eq!(grouped["SINV-0001"]["ITEM-001"].len(), 2);
    assert_eq!(grouped["SINV-0001"]["ITEM-001"][0].qty, -1.0);
    assert_eq!(grouped["SINV-0001"]["ITEM-001"][1].base_amount, -180.0);
    assert_eq!(grouped["SINV-0002"]["ITEM-002"][0].qty, -3.0);
}

#[test]
fn gross_profit_prepare_vouchers_to_ignore_preserves_invoice_parent_order_like_erpnext() {
    let rows = vec![
        invoice_item_row("SINV-0001", "ITEM-001"),
        invoice_item_row("SINV-0002", "ITEM-002"),
        invoice_item_row("SINV-0001", "ITEM-003"),
    ];

    assert_eq!(
        prepare_vouchers_to_ignore(&rows),
        vec![
            "SINV-0001".to_string(),
            "SINV-0002".to_string(),
            "SINV-0001".to_string(),
        ]
    );
}
