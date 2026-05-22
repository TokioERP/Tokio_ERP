use tokio_erp::erpnext::accounts::doctype::fiscal_year_company::fiscal_year_company::FiscalYearCompany;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn fiscal_year_company_matches_erpnext_metadata() {
    assert_eq!(FiscalYearCompany::DOCTYPE, "Fiscal Year Company");
    assert_eq!(FiscalYearCompany::MODULE, "Accounts");
    assert_eq!(FiscalYearCompany::FIELD_ORDER, ["company"]);
    assert!(FiscalYearCompany::IS_TABLE);
    assert!(FiscalYearCompany::EDITABLE_GRID);
    assert!(FiscalYearCompany::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(FiscalYearCompany::TRACK_CHANGES);
    assert_eq!(FiscalYearCompany::DOCUMENT_TYPE, Some("Setup"));
    assert_eq!(FiscalYearCompany::ROW_FORMAT, Some("Dynamic"));

    assert_eq!(
        FiscalYearCompany::fields(),
        vec![FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .ignore_user_permissions()
            .in_list_view(),]
    );
}

#[test]
fn fiscal_year_company_preserves_pass_controller_behavior() {
    let blank = FiscalYearCompany::default();
    assert_eq!(blank.company, None);
    assert!(blank.custom_hooks().is_empty());

    let row = FiscalYearCompany::new("Test Company");
    assert_eq!(row.company.as_deref(), Some("Test Company"));
    assert_eq!(row.doctype(), "Fiscal Year Company");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
