use tokio_erp::erpnext::stock::dashboard::item_dashboard::{
    get_bin_data, get_data, get_filters, BinRow, DashboardFilter, DashboardFilterValue, ItemInfo,
    ItemReservedStock, WarehousePermission,
};

#[test]
fn item_dashboard_filters_match_erpnext_order_and_item_group_scope() {
    assert_eq!(
        get_filters(
            Some("ITEM-001"),
            Some("Stores - TC"),
            Some(vec!["ITEM-001".to_string(), "ITEM-002".to_string()])
        ),
        vec![
            DashboardFilter::eq(
                "item_code",
                DashboardFilterValue::Text("ITEM-001".to_string())
            ),
            DashboardFilter::eq(
                "warehouse",
                DashboardFilterValue::Text("Stores - TC".to_string())
            ),
            DashboardFilter::in_list(
                "item_code",
                vec!["ITEM-001".to_string(), "ITEM-002".to_string()]
            ),
        ]
    );
}

#[test]
fn item_dashboard_bin_query_plan_matches_erpnext_get_all_shape() {
    let filters = vec![DashboardFilter::eq(
        "warehouse",
        DashboardFilterValue::Text("Stores - TC".to_string()),
    )];
    let result = get_bin_data(
        filters.clone(),
        7,
        "actual_qty",
        "desc",
        vec![BinRow::new("ITEM-001", "Stores - TC").actual_qty(10.0)],
        &[],
        &[],
        2,
    );

    assert_eq!(result.plan.doctype, "Bin");
    assert_eq!(
        result.plan.fields,
        [
            "item_code",
            "warehouse",
            "projected_qty",
            "reserved_qty",
            "reserved_qty_for_production",
            "reserved_qty_for_sub_contract",
            "actual_qty",
            "valuation_rate",
        ]
    );
    assert_eq!(result.plan.filters, filters);
    assert_eq!(
        result.plan.or_filters,
        vec![
            DashboardFilter::not_eq("projected_qty", DashboardFilterValue::Int(0)),
            DashboardFilter::not_eq("reserved_qty", DashboardFilterValue::Int(0)),
            DashboardFilter::not_eq("reserved_qty_for_production", DashboardFilterValue::Int(0)),
            DashboardFilter::not_eq(
                "reserved_qty_for_sub_contract",
                DashboardFilterValue::Int(0)
            ),
            DashboardFilter::not_eq("actual_qty", DashboardFilterValue::Int(0)),
        ]
    );
    assert_eq!(result.plan.order_by, "actual_qty desc");
    assert_eq!(result.plan.limit_start, 7);
    assert_eq!(result.plan.limit_page_length, 21);
}

#[test]
fn item_dashboard_permission_denied_returns_empty_like_erpnext() {
    let result = get_data(
        None,
        None,
        None,
        0,
        "actual_qty",
        "desc",
        WarehousePermission::Denied,
        vec![BinRow::new("ITEM-001", "Stores - TC").actual_qty(10.0)],
        &[],
        &[],
        2,
    );

    assert!(result.rows.is_empty());
}

#[test]
fn item_dashboard_permission_filter_and_reserved_request_match_erpnext() {
    let result = get_data(
        Some("ITEM-FILTER"),
        Some("WH-FILTER"),
        None,
        0,
        "projected_qty",
        "asc",
        WarehousePermission::Restricted(vec!["WH-ALLOWED".to_string()]),
        vec![BinRow::new("ITEM-ROW", "WH-ROW").projected_qty(3.0)],
        &[],
        &[],
        2,
    );

    assert_eq!(
        result.plan.filters,
        vec![
            DashboardFilter::eq(
                "item_code",
                DashboardFilterValue::Text("ITEM-FILTER".to_string())
            ),
            DashboardFilter::eq(
                "warehouse",
                DashboardFilterValue::Text("WH-FILTER".to_string())
            ),
            DashboardFilter::in_list("warehouse", vec!["WH-ALLOWED".to_string()]),
        ]
    );
    assert_eq!(result.plan.order_by, "projected_qty asc");
    assert_eq!(result.reserved_stock_item_codes, vec!["ITEM-FILTER"]);
    assert_eq!(result.reserved_stock_warehouses, vec!["WH-FILTER"]);
}

#[test]
fn item_dashboard_row_update_matches_erpnext_escape_rounding_and_quick_entry() {
    let result = get_data(
        None,
        None,
        None,
        0,
        "actual_qty",
        "desc",
        WarehousePermission::Unrestricted,
        vec![BinRow::new("ITEM-<A>", "Stores & Main")
            .projected_qty(1.234)
            .reserved_qty(2.345)
            .reserved_qty_for_production(3.456)
            .reserved_qty_for_sub_contract(4.567)
            .actual_qty(5.678)
            .valuation_rate(9.876)],
        &[ItemInfo {
            item_code: "ITEM-<A>".to_string(),
            item_name: Some("Bolt <Large>".to_string()),
            stock_uom: Some("Nos & Box".to_string()),
            has_batch_no: true,
            has_serial_no: false,
        }],
        &[ItemReservedStock::new(
            "ITEM-<A>",
            "Stores & Main",
            Some(1.5),
        )],
        2,
    );

    assert_eq!(result.reserved_stock_item_codes, vec!["ITEM-<A>"]);
    assert_eq!(result.reserved_stock_warehouses, vec!["Stores & Main"]);
    assert_eq!(result.rows.len(), 1);

    let row = &result.rows[0];
    assert_eq!(row.item_code, "ITEM-&lt;A&gt;");
    assert_eq!(row.item_name, "Bolt &lt;Large&gt;");
    assert_eq!(row.stock_uom, "Nos &amp; Box");
    assert_eq!(row.warehouse, "Stores &amp; Main");
    assert!(row.disable_quick_entry);
    assert_eq!(row.projected_qty, 1.23);
    assert_eq!(row.reserved_qty, 2.35);
    assert_eq!(row.reserved_qty_for_production, 3.46);
    assert_eq!(row.reserved_qty_for_sub_contract, 4.57);
    assert_eq!(row.actual_qty, 5.68);
    assert_eq!(row.valuation_rate, 9.876);
    assert_eq!(row.reserved_stock, 1.5);
}
