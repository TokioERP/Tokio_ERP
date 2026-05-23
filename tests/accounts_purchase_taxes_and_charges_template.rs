use tokio_erp::erpnext::accounts::doctype::purchase_taxes_and_charges_template::purchase_taxes_and_charges_template::PurchaseTaxesAndChargesTemplate;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn purchase_taxes_and_charges_template_matches_erpnext_metadata() {
    assert_eq!(
        PurchaseTaxesAndChargesTemplate::DOCTYPE,
        "Purchase Taxes and Charges Template"
    );
    assert_eq!(PurchaseTaxesAndChargesTemplate::MODULE, "Accounts");
    assert_eq!(
        PurchaseTaxesAndChargesTemplate::FIELD_ORDER,
        [
            "title",
            "is_default",
            "disabled",
            "column_break4",
            "company",
            "tax_category",
            "section_break6",
            "taxes",
        ]
    );
    assert!(PurchaseTaxesAndChargesTemplate::ALLOW_RENAME);
    assert_eq!(
        PurchaseTaxesAndChargesTemplate::fields(),
        vec![
            FieldSpec::data("title", "Title").required().no_copy(),
            FieldSpec::check("is_default", "Default")
                .default("0")
                .in_list_view(),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .in_list_view(),
            FieldSpec::column_break("column_break4"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("tax_category", "Tax Category").options("Tax Category"),
            FieldSpec::section_break("section_break6"),
            FieldSpec::table("taxes", "Purchase Taxes and Charges")
                .options("Purchase Taxes and Charges"),
        ]
    );
}

#[test]
fn purchase_taxes_and_charges_template_autoname_and_hooks_match_erpnext() {
    let mut doc = PurchaseTaxesAndChargesTemplate::new("Input VAT", "_Test Company");
    doc.autoname(Some("TC"));
    assert_eq!(doc.name.as_deref(), Some("Input VAT - TC"));
    assert_eq!(
        doc.validate_delegate(),
        "valdiate_taxes_and_charges_template"
    );
    assert_eq!(doc.custom_hooks(), ["validate", "autoname"]);
    assert_eq!(doc.doctype(), "Purchase Taxes and Charges Template");
    assert_eq!(doc.module(), "Accounts");
}
