use tokio_erp::erpnext::accounts::report::inactive_sales_items::inactive_sales_items::{
    execute, get_columns, get_items_query_plan, get_sales_details_query_plan,
    get_territories_query_plan, InactiveSalesItem, InactiveSalesItemsFilters, ReportColumn,
    SalesDetail, SalesDetailsQueryPlan, Territory,
};

fn filters() -> InactiveSalesItemsFilters {
    InactiveSalesItemsFilters {
        based_on: "Sales Invoice".to_string(),
        days: 30,
        territory: None,
        item_group: None,
        item: None,
    }
}

#[test]
fn inactive_sales_items_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::link("Territory", "territory", "Territory", 100),
            ReportColumn::link("Item Group", "item_group", "Item Group", 150),
            ReportColumn::link("Item", "item", "Item", 150),
            ReportColumn::data("Item Name", "item_name", 150),
            ReportColumn::link("Customer", "customer", "Customer", 100),
            ReportColumn::date("Last Order Date", "last_order_date", 100),
            ReportColumn::float("Quantity", "qty", 100),
            ReportColumn::int("Days Since Last Order", "days_since_last_order", 100),
        ]
    );
}

#[test]
fn inactive_sales_items_query_plans_match_erpnext_filters_and_date_field() {
    let report_filters = InactiveSalesItemsFilters {
        based_on: "Sales Order".to_string(),
        days: 30,
        territory: Some("West".to_string()),
        item_group: Some("Products".to_string()),
        item: Some("ITEM-001".to_string()),
    };

    assert_eq!(
        get_territories_query_plan(&report_filters),
        vec![("name", "=", "West".to_string())]
    );
    assert_eq!(
        get_items_query_plan(&report_filters),
        vec![
            ("disabled", "=", "0".to_string()),
            ("is_stock_item", "=", "1".to_string()),
            ("item_group", "=", "Products".to_string()),
            ("name", "=", "ITEM-001".to_string()),
            ("order_by", "=", "name".to_string()),
        ]
    );
    assert_eq!(
        get_sales_details_query_plan(&report_filters),
        SalesDetailsQueryPlan {
            doctype: "Sales Order",
            child_doctype: "Sales Order Item",
            date_field: "s.transaction_date",
            join_condition: "s.name = si.parent",
            docstatus_filter: "s.docstatus = 1",
            order_by: "days_since_last_order",
        }
    );
}

#[test]
fn inactive_sales_items_keeps_items_without_sales_and_oldest_setdefault_sales_details_only() {
    let report = execute(
        &filters(),
        &[
            Territory::new("North"),
            Territory::new("South"),
            Territory::new("West"),
        ],
        &[
            InactiveSalesItem::new("ITEM-001", "Products", "Item One"),
            InactiveSalesItem::new("ITEM-002", "Products", "Item Two"),
        ],
        &[
            SalesDetail::new("North", "CUST-RECENT", "ITEM-001", 2.0, "2026-05-20", 4),
            SalesDetail::new("North", "CUST-OLD", "ITEM-001", 6.0, "2026-04-01", 53),
            SalesDetail::new("North", "CUST-OLD", "ITEM-002", 3.0, "2026-04-10", 44),
        ],
    );

    assert_eq!(report.columns, get_columns());
    assert_eq!(report.rows.len(), 5);

    assert_eq!(report.rows[0].territory, "North");
    assert_eq!(report.rows[0].item, "ITEM-002");
    assert_eq!(report.rows[0].customer.as_deref(), Some("CUST-OLD"));
    assert_eq!(report.rows[0].days_since_last_order, Some(44));

    assert_eq!(report.rows[1].territory, "South");
    assert_eq!(report.rows[1].item, "ITEM-001");
    assert_eq!(report.rows[1].customer, None);

    assert_eq!(report.rows[4].territory, "West");
    assert_eq!(report.rows[4].item, "ITEM-002");
    assert_eq!(report.rows[4].customer, None);
}
