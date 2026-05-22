use tokio_erp::erpnext::accounts::doctype::accounting_dimension_detail::accounting_dimension_detail::AccountingDimensionDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn accounting_dimension_detail_matches_erpnext_metadata() {
    assert_eq!(
        AccountingDimensionDetail::DOCTYPE,
        "Accounting Dimension Detail"
    );
    assert_eq!(AccountingDimensionDetail::MODULE, "Accounts");
    assert_eq!(
        AccountingDimensionDetail::FIELD_ORDER,
        [
            "company",
            "reference_document",
            "default_dimension",
            "mandatory_for_bs",
            "mandatory_for_pl",
            "column_break_lqns",
            "automatically_post_balancing_accounting_entry",
            "offsetting_account",
        ]
    );
    assert!(AccountingDimensionDetail::IS_TABLE);
    assert!(AccountingDimensionDetail::TRACK_CHANGES);

    assert_eq!(
        AccountingDimensionDetail::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .columns(2)
                .in_list_view(),
            FieldSpec::link("reference_document", "Reference Document")
                .options("DocType")
                .hidden()
                .read_only(),
            FieldSpec::dynamic_link("default_dimension")
                .label("Default Dimension")
                .options("reference_document")
                .columns(2)
                .in_list_view(),
            FieldSpec::check("mandatory_for_bs", "Mandatory For Balance Sheet")
                .default("0")
                .columns(3)
                .in_list_view(),
            FieldSpec::check("mandatory_for_pl", "Mandatory For Profit and Loss Account")
                .default("0")
                .columns(3)
                .in_list_view(),
            FieldSpec::check(
                "automatically_post_balancing_accounting_entry",
                "Automatically post balancing accounting entry",
            )
            .default("0"),
            FieldSpec::link("offsetting_account", "Offsetting Account")
                .options("Account")
                .mandatory_depends_on("eval: doc.automatically_post_balancing_accounting_entry"),
            FieldSpec::column_break("column_break_lqns"),
        ]
    );
}

#[test]
fn accounting_dimension_detail_preserves_pass_controller_behavior() {
    let blank = AccountingDimensionDetail::default();
    assert_eq!(blank.company, None);
    assert_eq!(blank.reference_document, None);
    assert_eq!(blank.default_dimension, None);
    assert!(!blank.mandatory_for_bs);
    assert!(!blank.mandatory_for_pl);
    assert!(!blank.automatically_post_balancing_accounting_entry);
    assert_eq!(blank.offsetting_account, None);
    assert!(blank.custom_hooks().is_empty());

    let row = AccountingDimensionDetail::new("UzKing LLC", "Cost Center", "Main - TC");
    assert_eq!(row.company.as_deref(), Some("UzKing LLC"));
    assert_eq!(row.reference_document.as_deref(), Some("Cost Center"));
    assert_eq!(row.default_dimension.as_deref(), Some("Main - TC"));
    assert_eq!(row.doctype(), "Accounting Dimension Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
