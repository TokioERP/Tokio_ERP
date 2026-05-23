use tokio_erp::erpnext::accounts::doctype::sales_taxes_and_charges::sales_taxes_and_charges::SalesTaxesAndCharges;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_taxes_and_charges_matches_erpnext_core_metadata() {
    assert_eq!(SalesTaxesAndCharges::DOCTYPE, "Sales Taxes and Charges");
    assert_eq!(SalesTaxesAndCharges::MODULE, "Accounts");
    assert_eq!(SalesTaxesAndCharges::FIELD_ORDER.len(), 27);
    assert_eq!(SalesTaxesAndCharges::FIELD_ORDER[0], "charge_type");
    assert_eq!(SalesTaxesAndCharges::FIELD_ORDER[26], "dont_recompute_tax");
    assert!(SalesTaxesAndCharges::IS_TABLE);
    assert!(SalesTaxesAndCharges::EDITABLE_GRID);
    assert!(SalesTaxesAndCharges::INDEX_WEB_PAGES_FOR_SEARCH);

    let fields = SalesTaxesAndCharges::fields();
    assert_eq!(fields.len(), 27);
    assert!(fields.contains(
        &FieldSpec::select("charge_type", "Type")
            .options("\nActual\nOn Net Total\nOn Previous Row Amount\nOn Previous Row Total\nOn Item Quantity")
            .required()
            .in_list_view()
            .columns(2)
            .oldfield("charge_type", "Select")
    ));
    assert!(fields.contains(
        &FieldSpec::link("account_head", "Account Head")
            .options("Account")
            .required()
            .in_list_view()
            .columns(2)
            .search_index()
            .allow_on_submit()
            .oldfield("account_head", "Link")
    ));
    assert!(fields.contains(
        &FieldSpec::small_text("description", "Description")
            .required()
            .oldfield("description", "Small Text")
            .width("300px")
    ));
    assert!(fields.contains(
        &FieldSpec::currency("tax_amount", "Amount")
            .options("currency")
            .in_list_view()
            .columns(2)
    ));
}

#[test]
fn sales_taxes_and_charges_preserves_pass_controller_behavior() {
    let blank = SalesTaxesAndCharges::default();
    assert_eq!(blank.account_head, None);
    assert_eq!(blank.charge_type, None);
    assert_eq!(blank.rate, None);
    assert!(blank.custom_hooks().is_empty());

    let row = SalesTaxesAndCharges::new("On Net Total", "VAT - TC");
    assert_eq!(row.charge_type.as_deref(), Some("On Net Total"));
    assert_eq!(row.account_head.as_deref(), Some("VAT - TC"));
    assert_eq!(row.doctype(), "Sales Taxes and Charges");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
