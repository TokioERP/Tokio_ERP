use tokio_erp::erpnext::accounts::doctype::purchase_taxes_and_charges::purchase_taxes_and_charges::PurchaseTaxesAndCharges;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn purchase_taxes_and_charges_matches_erpnext_core_metadata() {
    assert_eq!(
        PurchaseTaxesAndCharges::DOCTYPE,
        "Purchase Taxes and Charges"
    );
    assert_eq!(PurchaseTaxesAndCharges::MODULE, "Accounts");
    assert_eq!(PurchaseTaxesAndCharges::FIELD_ORDER.len(), 29);
    assert_eq!(PurchaseTaxesAndCharges::FIELD_ORDER[0], "category");
    assert_eq!(
        PurchaseTaxesAndCharges::FIELD_ORDER[28],
        "dont_recompute_tax"
    );
    assert!(PurchaseTaxesAndCharges::IS_TABLE);
    assert!(PurchaseTaxesAndCharges::EDITABLE_GRID);

    let fields = PurchaseTaxesAndCharges::fields();
    assert_eq!(fields.len(), 29);
    assert!(fields.contains(
        &FieldSpec::select("category", "Consider Tax or Charge for")
            .options("Valuation and Total\nValuation\nTotal")
            .default("Total")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::select("charge_type", "Type")
            .options("\nActual\nOn Net Total\nOn Previous Row Amount\nOn Previous Row Total\nOn Item Quantity")
            .default("On Net Total")
            .required()
            .in_list_view()
            .columns(2)
    ));
    assert!(fields.contains(
        &FieldSpec::link("account_head", "Account Head")
            .options("Account")
            .required()
            .in_list_view()
            .columns(2)
    ));
    assert!(fields.contains(
        &FieldSpec::currency("tax_amount", "Amount")
            .options("currency")
            .in_list_view()
            .columns(2)
    ));
}

#[test]
fn purchase_taxes_and_charges_preserves_pass_controller_behavior() {
    let blank = PurchaseTaxesAndCharges::default();
    assert_eq!(blank.account_head, None);
    assert_eq!(blank.charge_type, None);
    assert_eq!(blank.rate, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PurchaseTaxesAndCharges::new("On Net Total", "VAT - TC");
    assert_eq!(row.charge_type.as_deref(), Some("On Net Total"));
    assert_eq!(row.account_head.as_deref(), Some("VAT - TC"));
    assert_eq!(row.doctype(), "Purchase Taxes and Charges");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
