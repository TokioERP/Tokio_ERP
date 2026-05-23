use tokio_erp::erpnext::accounts::doctype::tax_category::tax_category::{
    tax_category_dashboard_groups, TaxCategory,
};
use tokio_erp::erpnext::accounts::doctype::tax_withholding_account::tax_withholding_account::TaxWithholdingAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn tax_category_matches_erpnext_metadata_and_dashboard() {
    assert_eq!(TaxCategory::DOCTYPE, "Tax Category");
    assert_eq!(TaxCategory::MODULE, "Accounts");
    assert_eq!(TaxCategory::AUTONAME, "field:title");
    assert_eq!(TaxCategory::FIELD_ORDER, ["title", "disabled"]);
    assert!(TaxCategory::ALLOW_RENAME);
    assert!(TaxCategory::EDITABLE_GRID);
    assert!(TaxCategory::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(TaxCategory::QUICK_ENTRY);
    assert!(TaxCategory::TRACK_CHANGES);
    assert_eq!(
        TaxCategory::fields(),
        vec![
            FieldSpec::data("title", "Title")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::check("disabled", "Disabled").default("0"),
        ]
    );
    assert_eq!(
        tax_category_dashboard_groups(),
        [
            ("Pre Sales", ["Quotation", "Supplier Quotation"].as_slice()),
            (
                "Sales",
                ["Sales Invoice", "Delivery Note", "Sales Order"].as_slice()
            ),
            (
                "Purchase",
                ["Purchase Invoice", "Purchase Receipt"].as_slice()
            ),
            ("Party", ["Customer", "Supplier"].as_slice()),
            ("Taxes", ["Item", "Tax Rule"].as_slice()),
        ]
    );
}

#[test]
fn tax_withholding_account_matches_erpnext_metadata() {
    assert_eq!(TaxWithholdingAccount::DOCTYPE, "Tax Withholding Account");
    assert_eq!(TaxWithholdingAccount::MODULE, "Accounts");
    assert_eq!(TaxWithholdingAccount::FIELD_ORDER, ["company", "account"]);
    assert!(TaxWithholdingAccount::IS_TABLE);
    assert!(TaxWithholdingAccount::EDITABLE_GRID);
    assert!(TaxWithholdingAccount::QUICK_ENTRY);
    assert!(TaxWithholdingAccount::TRACK_CHANGES);
    assert_eq!(
        TaxWithholdingAccount::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view()
                .ignore_user_permissions(),
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view()
                .ignore_user_permissions(),
        ]
    );
}

#[test]
fn tax_category_and_withholding_account_preserve_pass_controller_behavior() {
    let category = TaxCategory::new("VAT");
    assert_eq!(category.title.as_deref(), Some("VAT"));
    assert!(!category.disabled);
    assert_eq!(category.doctype(), "Tax Category");
    assert_eq!(category.module(), "Accounts");
    assert!(category.custom_hooks().is_empty());

    let row = TaxWithholdingAccount::new("_Test Company", "TDS - TC");
    assert_eq!(row.company.as_deref(), Some("_Test Company"));
    assert_eq!(row.account.as_deref(), Some("TDS - TC"));
    assert_eq!(row.doctype(), "Tax Withholding Account");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
