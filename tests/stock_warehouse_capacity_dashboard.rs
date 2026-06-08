use tokio_erp::erpnext::stock::dashboard::warehouse_capacity_dashboard::{
    get_data, get_filters, get_warehouse_capacity_data, get_warehouse_filter_based_on_permissions,
    DashboardFilter, DashboardFilterValue, PutawayRuleRow, StockBalance, WarehouseCapacityError,
    WarehousePermission,
};

#[test]
fn warehouse_capacity_filters_match_erpnext_order_and_parent_scope() {
    assert_eq!(
        get_filters(
            Some("ITEM-001"),
            Some("Stores - TC"),
            Some(vec![
                "Stores - TC".to_string(),
                "Finished Goods - TC".to_string()
            ]),
            Some("_Test Company")
        ),
        vec![
            DashboardFilter::eq("disable", DashboardFilterValue::Int(0)),
            DashboardFilter::eq(
                "item_code",
                DashboardFilterValue::Text("ITEM-001".to_string())
            ),
            DashboardFilter::eq(
                "warehouse",
                DashboardFilterValue::Text("Stores - TC".to_string())
            ),
            DashboardFilter::eq(
                "company",
                DashboardFilterValue::Text("_Test Company".to_string())
            ),
            DashboardFilter::in_list(
                "warehouse",
                vec!["Stores - TC".to_string(), "Finished Goods - TC".to_string()]
            ),
        ]
    );
}

#[test]
fn warehouse_capacity_permission_filter_matches_erpnext_branches() {
    let filters = vec![DashboardFilter::eq("disable", DashboardFilterValue::Int(0))];

    assert_eq!(
        get_warehouse_filter_based_on_permissions(
            filters.clone(),
            WarehousePermission::Unrestricted
        ),
        (false, filters.clone())
    );

    assert_eq!(
        get_warehouse_filter_based_on_permissions(
            filters.clone(),
            WarehousePermission::Restricted(vec!["Allowed - TC".to_string()])
        ),
        (
            false,
            vec![
                DashboardFilter::eq("disable", DashboardFilterValue::Int(0)),
                DashboardFilter::in_list("warehouse", vec!["Allowed - TC".to_string()]),
            ]
        )
    );

    assert_eq!(
        get_warehouse_filter_based_on_permissions(filters, WarehousePermission::Denied),
        (true, Vec::new())
    );
}

#[test]
fn warehouse_capacity_data_query_and_row_update_match_erpnext() {
    let filters = vec![DashboardFilter::eq("disable", DashboardFilterValue::Int(0))];
    let rows = vec![
        PutawayRuleRow::new("ITEM-LOW", "Bulk <A>", 40.0, "A & Co"),
        PutawayRuleRow::new("ITEM-HIGH", "Stores & Main", 80.0, "A & Co"),
    ];
    let balances = vec![
        StockBalance::new("ITEM-LOW", "Bulk <A>", Some(10.0)),
        StockBalance::new("ITEM-HIGH", "Stores & Main", Some(20.0)),
    ];

    let result = get_warehouse_capacity_data(filters.clone(), 5, rows, &balances).unwrap();

    assert_eq!(result.plan.doctype, "Putaway Rule");
    assert_eq!(
        result.plan.fields,
        ["item_code", "warehouse", "stock_capacity", "company"]
    );
    assert_eq!(result.plan.filters, filters);
    assert_eq!(result.plan.limit_start, 5);
    assert_eq!(result.plan.limit_page_length, "11");
    assert_eq!(result.rows[0].warehouse, "Bulk &lt;A&gt;");
    assert_eq!(result.rows[0].company, "A &amp; Co");
    assert_eq!(result.rows[0].actual_qty, 10.0);
    assert_eq!(result.rows[0].percent_occupied, 25.0);
}

#[test]
fn warehouse_capacity_get_data_sorts_like_erpnext_numeric_key() {
    let rows = vec![
        PutawayRuleRow::new("ITEM-A", "WH-A", 25.0, "TC"),
        PutawayRuleRow::new("ITEM-B", "WH-B", 100.0, "TC"),
        PutawayRuleRow::new("ITEM-C", "WH-C", 50.0, "TC"),
    ];
    let balances = vec![
        StockBalance::new("ITEM-A", "WH-A", Some(20.0)),
        StockBalance::new("ITEM-B", "WH-B", Some(10.0)),
        StockBalance::new("ITEM-C", "WH-C", Some(30.0)),
    ];

    let desc = get_data(
        None,
        None,
        None,
        None,
        0,
        "stock_capacity",
        "desc",
        WarehousePermission::Unrestricted,
        rows.clone(),
        &balances,
    )
    .unwrap();
    assert_eq!(
        desc.into_iter()
            .map(|row| row.item_code)
            .collect::<Vec<_>>(),
        vec!["ITEM-B", "ITEM-C", "ITEM-A"]
    );

    let asc_percent = get_data(
        None,
        None,
        None,
        None,
        0,
        "percent_occupied",
        "asc",
        WarehousePermission::Unrestricted,
        rows,
        &balances,
    )
    .unwrap();
    assert_eq!(
        asc_percent
            .into_iter()
            .map(|row| row.item_code)
            .collect::<Vec<_>>(),
        vec!["ITEM-B", "ITEM-C", "ITEM-A"]
    );
}

#[test]
fn warehouse_capacity_zero_capacity_preserves_erpnext_division_error() {
    let error = get_warehouse_capacity_data(
        Vec::new(),
        0,
        vec![PutawayRuleRow::new("ITEM-ZERO", "WH-ZERO", 0.0, "TC")],
        &[StockBalance::new("ITEM-ZERO", "WH-ZERO", Some(1.0))],
    )
    .unwrap_err();

    assert_eq!(
        error,
        WarehouseCapacityError::DivisionByZero {
            item_code: "ITEM-ZERO".to_string(),
            warehouse: "WH-ZERO".to_string()
        }
    );
}
