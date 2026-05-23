use tokio_erp::erpnext::accounts::doctype::sales_taxes_and_charges_template::sales_taxes_and_charges_template::{
    sales_taxes_and_charges_template_dashboard, sales_taxes_and_charges_template_js_hooks,
    validate_disabled, validate_for_tax_category, SalesTaxRow, SalesTaxesAndChargesTemplate,
    TaxCategoryTemplate,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_taxes_and_charges_template_matches_erpnext_metadata() {
    assert_eq!(
        SalesTaxesAndChargesTemplate::DOCTYPE,
        "Sales Taxes and Charges Template"
    );
    assert_eq!(SalesTaxesAndChargesTemplate::MODULE, "Accounts");
    assert_eq!(
        SalesTaxesAndChargesTemplate::FIELD_ORDER,
        [
            "title",
            "is_default",
            "disabled",
            "column_break_3",
            "company",
            "tax_category",
            "section_break_5",
            "taxes",
        ]
    );
    assert!(SalesTaxesAndChargesTemplate::ALLOW_IMPORT);
    assert!(SalesTaxesAndChargesTemplate::ALLOW_RENAME);
    assert!(SalesTaxesAndChargesTemplate::SHOW_TITLE_FIELD_IN_LINK);
    assert_eq!(SalesTaxesAndChargesTemplate::TITLE_FIELD, "title");
    assert!(SalesTaxesAndChargesTemplate::TRACK_CHANGES);
    assert_eq!(
        SalesTaxesAndChargesTemplate::fields(),
        vec![
            FieldSpec::data("title", "Title")
                .required()
                .no_copy()
                .oldfield("title", "Data"),
            FieldSpec::check("is_default", "Default")
                .default("0")
                .in_list_view(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view()
                .in_standard_filter()
                .oldfield("company", "Link"),
            FieldSpec::link("tax_category", "Tax Category").options("Tax Category"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("taxes", "Sales Taxes and Charges")
                .options("Sales Taxes and Charges")
                .description("* Will be calculated in the transaction.")
                .oldfield("other_charges", "Table"),
        ]
    );
}

#[test]
fn sales_taxes_and_charges_template_validate_autoname_and_set_missing_values_match_erpnext() {
    let mut doc = SalesTaxesAndChargesTemplate::new("Output VAT", "_Test Company");
    doc.is_default = true;
    doc.disabled = true;
    assert_eq!(
        validate_disabled(&doc).unwrap_err(),
        "Disabled template must not be default template"
    );
    assert_eq!(
        doc.validate(&[]).unwrap_err(),
        "Disabled template must not be default template"
    );

    doc.disabled = false;
    doc.tax_category = Some("VAT".to_string());
    assert_eq!(
        validate_for_tax_category(
            &doc,
            &[TaxCategoryTemplate::new("Existing", "_Test Company", "VAT")]
        )
        .unwrap_err(),
        "A template with tax category <b>VAT</b> already exists. Only one template is allowed with each tax category"
    );

    doc.autoname(Some("TC"));
    assert_eq!(doc.name.as_deref(), Some("Output VAT - TC"));
    doc.taxes = vec![SalesTaxRow {
        charge_type: "On Net Total".to_string(),
        rate: Some("0".to_string()),
        account_head: "VAT - TC".to_string(),
    }];
    doc.set_missing_values(&[("VAT - TC".to_string(), "12.5".to_string())]);
    assert_eq!(doc.taxes[0].rate.as_deref(), Some("12.5"));
    assert_eq!(
        doc.validate_delegate(),
        "valdiate_taxes_and_charges_template"
    );
    assert_eq!(doc.custom_hooks(), ["validate", "autoname"]);
}

#[test]
fn sales_taxes_and_charges_template_dashboard_and_js_hooks_match_erpnext() {
    let dashboard = sales_taxes_and_charges_template_dashboard();
    assert_eq!(dashboard.fieldname, "taxes_and_charges");
    assert_eq!(
        dashboard.non_standard_fieldnames,
        vec![
            ("Tax Rule", "sales_tax_template"),
            ("Subscription", "sales_tax_template"),
            ("Restaurant", "default_tax_template"),
        ]
    );
    assert_eq!(
        dashboard.transactions,
        vec![
            (
                "Transactions",
                vec!["Sales Invoice", "Sales Order", "Delivery Note"]
            ),
            (
                "References",
                vec!["POS Profile", "Subscription", "Restaurant", "Tax Rule"],
            ),
        ]
    );

    let js_hooks = sales_taxes_and_charges_template_js_hooks();
    assert_eq!(js_hooks.tax_table, "Sales Taxes and Charges");
    assert_eq!(
        js_hooks.tax_validations_doctype,
        "Sales Taxes and Charges Template"
    );
    assert_eq!(js_hooks.tax_filters_doctype, "Sales Taxes and Charges");
    let doc = SalesTaxesAndChargesTemplate::new("Output VAT", "_Test Company");
    assert_eq!(doc.doctype(), "Sales Taxes and Charges Template");
    assert_eq!(doc.module(), "Accounts");
}
