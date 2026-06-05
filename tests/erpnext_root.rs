use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::{
    check_app_permission, encode_company_abbr, get_company_currency, get_default_company,
    get_default_cost_center, get_default_currency, get_default_finance_book,
    get_party_account_type, get_region, is_perpetual_inventory_enabled, normalize_ctx_input,
    resolve_regional_override, set_perpetual_inventory, CtxInput, ErpnextLocalCache,
    PerpetualInventoryUpdate, ERPNEXT_VERSION,
};

#[test]
fn erpnext_root_company_cache_and_inventory_helpers_match_python() {
    let mut cache = ErpnextLocalCache::default();

    assert_eq!(ERPNEXT_VERSION, "16.19.1");
    assert_eq!(
        get_default_company(None, &["Alpha LLC".to_string()], Some("Global LLC")),
        Some("Alpha LLC".to_string())
    );
    assert_eq!(
        get_default_company(Some("user@example.com"), &[], Some("Global LLC")),
        Some("Global LLC".to_string())
    );
    assert_eq!(
        get_default_currency(Some("Alpha LLC"), Some("USD")),
        Some("USD".to_string())
    );
    assert_eq!(get_default_currency(None, Some("USD")), None);

    assert_eq!(
        get_default_cost_center(None, &mut cache, Some("Main - A")),
        None
    );
    assert_eq!(
        get_default_cost_center(Some("Alpha LLC"), &mut cache, Some("Main - A")),
        Some("Main - A".to_string())
    );
    assert_eq!(
        get_default_cost_center(Some("Alpha LLC"), &mut cache, Some("Changed - A")),
        Some("Main - A".to_string())
    );
    assert_eq!(
        get_company_currency("Alpha LLC", &mut cache, Some("USD")),
        Some("USD".to_string())
    );
    assert_eq!(
        get_company_currency("Alpha LLC", &mut cache, Some("EUR")),
        Some("USD".to_string())
    );
    assert_eq!(
        get_default_finance_book(Some("Alpha LLC"), None, &mut cache, Some("IFRS")),
        Some("IFRS".to_string())
    );
    assert_eq!(
        get_party_account_type("Customer", &mut cache, Some("Receivable")),
        "Receivable"
    );

    assert_eq!(
        set_perpetual_inventory(0, None, true, Some("Alpha LLC")),
        Some(PerpetualInventoryUpdate {
            company: "_Test Company".to_string(),
            enable_perpetual_inventory: 0,
        })
    );
    assert_eq!(
        is_perpetual_inventory_enabled(Some("Alpha LLC"), false, None, &mut cache, Some(1)),
        1
    );
    assert_eq!(
        is_perpetual_inventory_enabled(Some("Alpha LLC"), false, None, &mut cache, Some(0)),
        1
    );
}

#[test]
fn erpnext_root_region_permission_abbr_and_context_helpers_match_python() {
    assert_eq!(encode_company_abbr("Cash", Some("TC")), "Cash - TC");
    assert_eq!(encode_company_abbr("Cash - tc", Some("TC")), "Cash - tc");
    assert_eq!(encode_company_abbr("Cash", None), "Cash");

    assert_eq!(
        get_region(
            None,
            Some("Alpha LLC"),
            Some("United States"),
            Some("Uzbekistan"),
            Some("India"),
        ),
        Some("United States".to_string())
    );
    assert_eq!(
        get_region(None, None, None, Some("Uzbekistan"), Some("India")),
        Some("Uzbekistan".to_string())
    );

    let mut regional_hooks = BTreeMap::new();
    regional_hooks.insert(
        "Uzbekistan".to_string(),
        BTreeMap::from([(
            "erpnext.accounts.doctype.sales_invoice.sales_invoice.make_gl_entries".to_string(),
            vec!["first.override".to_string(), "last.override".to_string()],
        )]),
    );
    assert_eq!(
        resolve_regional_override(
            Some("Uzbekistan"),
            "erpnext.accounts.doctype.sales_invoice.sales_invoice.make_gl_entries",
            &regional_hooks,
        ),
        Some("last.override".to_string())
    );
    assert_eq!(
        resolve_regional_override(Some("India"), "missing.path", &regional_hooks),
        None
    );

    assert!(check_app_permission("Administrator", true));
    assert!(!check_app_permission("guest@example.com", true));
    assert!(check_app_permission("desk@example.com", false));

    assert_eq!(
        normalize_ctx_input(CtxInput::Dict(BTreeMap::from([(
            "company".to_string(),
            json!("Alpha LLC"),
        )])))
        .unwrap(),
        json!({"company": "Alpha LLC"})
    );
    assert_eq!(
        normalize_ctx_input(CtxInput::Json(r#"{"company":"Beta LLC"}"#.to_string())).unwrap(),
        json!({"company": "Beta LLC"})
    );
}
