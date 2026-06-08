use std::collections::BTreeMap;

use tokio_erp::erpnext::stock::{
    get_company_default_inventory_account, get_warehouse_account, get_warehouse_account_map,
    install_docs, AccountLookup, InstallDoc, StockAccountLookup, StockError, WarehouseAccount,
    WarehouseAccountContext, WarehouseAccountFlags, WarehouseAccountMapResult,
    WarehouseAccountQueryPlan, WarehouseRecord,
};

#[test]
fn stock_install_docs_match_erpnext_order_and_fields() {
    assert_eq!(
        install_docs(),
        vec![
            InstallDoc::role("Stock Manager"),
            InstallDoc::role("Item Manager"),
            InstallDoc::role("Stock User"),
            InstallDoc::role("Quality Manager"),
            InstallDoc::item_group("All Item Groups", None, 1),
            InstallDoc::item_group("Default", Some("All Item Groups"), 0),
        ]
    );
}

#[test]
fn warehouse_account_resolution_matches_direct_parent_and_rebuild_branches() {
    let context = WarehouseAccountContext::default()
        .with_company_default_inventory_account("_Test Company", "Inventory - TC")
        .with_stock_account("_Test Company", "Stock Fallback - TC");
    let direct = WarehouseRecord::new("Stores - TC", Some("_Test Company"))
        .with_account("Stores Account - TC")
        .with_parent("Group - TC");

    assert_eq!(
        get_warehouse_account(&direct, None, &context)
            .unwrap()
            .account,
        Some("Stores Account - TC".to_string())
    );

    let parent_map = BTreeMap::from([(
        "Group - TC".to_string(),
        WarehouseAccount::new("Group - TC", "Group Account - TC", Some("_Test Company")),
    )]);
    let child = WarehouseRecord::new("Child - TC", Some("_Test Company")).with_parent("Group - TC");
    assert_eq!(
        get_warehouse_account(&child, Some(&parent_map), &context)
            .unwrap()
            .account,
        Some("Group Account - TC".to_string())
    );

    let missing_parent = WarehouseRecord::new("Missing Parent Child - TC", Some("_Test Company"))
        .with_parent("Missing Group - TC");
    let resolution = get_warehouse_account(&missing_parent, Some(&parent_map), &context).unwrap();
    assert_eq!(
        resolution.rebuild_tree_doctype,
        Some("Warehouse".to_string())
    );
    assert_eq!(resolution.account, Some("Inventory - TC".to_string()));
}

#[test]
fn warehouse_account_resolution_matches_sql_default_stock_and_throw_order() {
    let context = WarehouseAccountContext::default()
        .with_ancestor_account("Child - TC", "Ancestor Account - TC")
        .with_stock_account("_Test Company", "Stock Account - TC");
    let child = WarehouseRecord::new("Child - TC", Some("_Test Company"))
        .with_parent("Group - TC")
        .with_bounds(3, 4);

    let resolution = get_warehouse_account(&child, None, &context).unwrap();
    assert_eq!(
        resolution.account,
        Some("Ancestor Account - TC".to_string())
    );
    assert_eq!(
        resolution.ancestor_query,
        Some(WarehouseAccountQueryPlan {
            sql: "select account from `tabWarehouse` where lft <= ? and rgt >= ? and company = ? and account is not null and ifnull(account, '') !='' order by lft desc limit 1",
            params: (3, 4, "_Test Company".to_string()),
            as_list: true,
        })
    );

    let fallback = WarehouseRecord::new("Fallback - TC", Some("_Test Company"));
    let fallback_resolution = get_warehouse_account(&fallback, None, &context).unwrap();
    assert_eq!(
        fallback_resolution.account,
        Some("Stock Account - TC".to_string())
    );
    assert_eq!(
        fallback_resolution.stock_account_lookup,
        Some(StockAccountLookup {
            doctype: "Account",
            filters: BTreeMap::from([
                ("account_type", "Stock".to_string()),
                ("is_group", "0".to_string()),
                ("company", "_Test Company".to_string()),
            ]),
            fieldname: "name",
        })
    );

    let group_without_account =
        WarehouseRecord::new("Group Without Account - TC", Some("_Test Company")).as_group();
    assert_eq!(
        get_warehouse_account(
            &group_without_account,
            None,
            &WarehouseAccountContext::default()
        )
        .unwrap()
        .account,
        None
    );

    let leaf_without_account =
        WarehouseRecord::new("Leaf Without Account - TC", Some("_Test Company"));
    let error = get_warehouse_account(
        &leaf_without_account,
        None,
        &WarehouseAccountContext::default(),
    )
    .unwrap_err();
    assert_eq!(
        error,
        StockError::MissingWarehouseAccount {
            warehouse: "Leaf Without Account - TC".to_string(),
            company: "_Test Company".to_string(),
        }
    );
    assert_eq!(
        error.to_string(),
        "Please set Account in Warehouse Leaf Without Account - TC or Default Inventory Account in Company _Test Company"
    );
}

#[test]
fn warehouse_account_map_rebuilds_company_cache_like_erpnext() {
    let flags = WarehouseAccountFlags::default();
    let warehouses = vec![
        WarehouseRecord::new("Group - TC", Some("_Test Company"))
            .with_account("Group Account - TC"),
        WarehouseRecord::new("Child - TC", Some("_Test Company")).with_parent("Group - TC"),
        WarehouseRecord::new("No Company - TC", None).with_account("Ignored - TC"),
    ];
    let context = WarehouseAccountContext::default()
        .with_account_currency("Group Account - TC", "USD")
        .with_account_currency("Ignored - TC", "EUR");

    let result =
        get_warehouse_account_map(Some("_Test Company"), flags, false, warehouses, &context)
            .unwrap();

    assert_eq!(
        result.query_plan,
        Some(AccountLookup {
            doctype: "Warehouse",
            fields: vec!["name", "account", "parent_warehouse", "company", "is_group"],
            filters: BTreeMap::from([("company".to_string(), "_Test Company".to_string())]),
            order_by: "lft, rgt",
        })
    );
    assert_eq!(
        result.map,
        BTreeMap::from([
            (
                "Group - TC".to_string(),
                WarehouseAccount::new("Group - TC", "Group Account - TC", Some("_Test Company"))
                    .with_account_currency("USD"),
            ),
            (
                "Child - TC".to_string(),
                WarehouseAccount::new("Child - TC", "Group Account - TC", Some("_Test Company"))
                    .with_parent("Group - TC")
                    .with_account_currency("USD"),
            ),
        ])
    );
    assert_eq!(
        result.flags.by_company.get("_Test Company"),
        Some(&result.map)
    );
    assert!(result.flags.global.is_empty());
}

#[test]
fn warehouse_account_map_uses_cache_unless_in_test_or_company_cache_missing() {
    let cached_map = BTreeMap::from([(
        "Cached - TC".to_string(),
        WarehouseAccount::new("Cached - TC", "Cached Account - TC", Some("_Test Company")),
    )]);
    let flags = WarehouseAccountFlags {
        global: BTreeMap::new(),
        by_company: BTreeMap::from([("_Test Company".to_string(), cached_map.clone())]),
    };

    assert_eq!(
        get_warehouse_account_map(
            Some("_Test Company"),
            flags.clone(),
            false,
            Vec::new(),
            &WarehouseAccountContext::default(),
        )
        .unwrap(),
        WarehouseAccountMapResult {
            map: cached_map,
            flags: flags.clone(),
            query_plan: None,
            rebuild_tree_doctypes: Vec::new(),
        }
    );

    assert!(get_warehouse_account_map(
        Some("_Other Company"),
        flags,
        false,
        Vec::new(),
        &WarehouseAccountContext::default(),
    )
    .unwrap()
    .query_plan
    .is_some());
}

#[test]
fn company_default_inventory_account_preserves_cached_value_contract() {
    let context = WarehouseAccountContext::default()
        .with_company_default_inventory_account("_Test Company", "Inventory - TC");

    assert_eq!(
        get_company_default_inventory_account("_Test Company", &context),
        Some("Inventory - TC".to_string())
    );
}
