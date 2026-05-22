use tokio_erp::erpnext::accounts::doctype::ledger_health_monitor_company::ledger_health_monitor_company::LedgerHealthMonitorCompany;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn ledger_health_monitor_company_matches_erpnext_metadata() {
    assert_eq!(
        LedgerHealthMonitorCompany::DOCTYPE,
        "Ledger Health Monitor Company"
    );
    assert_eq!(LedgerHealthMonitorCompany::MODULE, "Accounts");
    assert_eq!(LedgerHealthMonitorCompany::FIELD_ORDER, ["company"]);
    assert!(LedgerHealthMonitorCompany::IS_TABLE);
    assert!(LedgerHealthMonitorCompany::ALLOW_RENAME);
    assert!(LedgerHealthMonitorCompany::EDITABLE_GRID);
    assert!(LedgerHealthMonitorCompany::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        LedgerHealthMonitorCompany::fields(),
        vec![FieldSpec::link("company", "Company")
            .options("Company")
            .in_list_view()]
    );
}

#[test]
fn ledger_health_monitor_company_preserves_pass_controller_behavior() {
    let blank = LedgerHealthMonitorCompany::default();
    assert_eq!(blank.company, None);
    assert!(blank.custom_hooks().is_empty());

    let row = LedgerHealthMonitorCompany::new("Test Company");
    assert_eq!(row.company.as_deref(), Some("Test Company"));
    assert_eq!(row.doctype(), "Ledger Health Monitor Company");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
