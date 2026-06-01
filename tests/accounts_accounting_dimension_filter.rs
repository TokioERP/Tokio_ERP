use std::collections::HashMap;

use tokio_erp::erpnext::accounts::doctype::accounting_dimension_filter::accounting_dimension_filter::{
    build_dimension_filter_map, AccountingDimensionFilter, AccountingDimensionFilterError,
    DimensionFilterInfo, DimensionFilterRow,
};
use tokio_erp::erpnext::accounts::doctype::allowed_dimension::allowed_dimension::AllowedDimension;
use tokio_erp::erpnext::accounts::doctype::applicable_on_account::applicable_on_account::ApplicableOnAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn accounting_dimension_filter_matches_erpnext_metadata() {
    assert_eq!(
        AccountingDimensionFilter::DOCTYPE,
        "Accounting Dimension Filter"
    );
    assert_eq!(AccountingDimensionFilter::MODULE, "Accounts");
    assert_eq!(
        AccountingDimensionFilter::FIELD_ORDER,
        [
            "accounting_dimension",
            "fieldname",
            "disabled",
            "column_break_2",
            "company",
            "apply_restriction_on_values",
            "allow_or_restrict",
            "section_break_4",
            "accounts",
            "column_break_6",
            "dimensions",
            "section_break_10",
            "dimension_filter_help",
        ]
    );
    assert_eq!(
        AccountingDimensionFilter::AUTONAME,
        "format:{accounting_dimension}-{#####}"
    );
    assert!(AccountingDimensionFilter::QUICK_ENTRY);
    assert_eq!(AccountingDimensionFilter::SORT_FIELD, "creation");
    assert_eq!(AccountingDimensionFilter::SORT_ORDER, "DESC");
    assert!(AccountingDimensionFilter::TRACK_CHANGES);

    assert_eq!(
        AccountingDimensionFilter::fields(),
        vec![
            FieldSpec::select("accounting_dimension", "Accounting Dimension")
                .in_list_view()
                .required(),
            FieldSpec::data("fieldname", "Fieldname").hidden(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::check(
                "apply_restriction_on_values",
                "Apply restriction on dimension values",
            )
            .default("1"),
            FieldSpec::select("allow_or_restrict", "Allow Or Restrict Dimension")
                .options("Allow\nRestrict")
                .depends_on("eval:doc.apply_restriction_on_values == 1;")
                .required(),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::table("accounts", "Applicable On Account")
                .options("Applicable On Account")
                .required(),
            FieldSpec::column_break("column_break_6"),
            FieldSpec::table("dimensions", "Applicable Dimension")
                .options("Allowed Dimension")
                .depends_on("eval:doc.accounting_dimension && doc.apply_restriction_on_values")
                .mandatory_depends_on("eval:doc.apply_restriction_on_values == 1;"),
            FieldSpec::section_break("section_break_10"),
            FieldSpec::html("dimension_filter_help", "Dimension Filter Help"),
        ]
    );
}

#[test]
fn accounting_dimension_filter_validate_and_before_save_match_erpnext() {
    let mut filter = AccountingDimensionFilter {
        name: Some("Cost Center-00001".to_string()),
        accounting_dimension: Some("Cost Center".to_string()),
        company: Some("Acme".to_string()),
        allow_or_restrict: "Allow".to_string(),
        apply_restriction_on_values: false,
        accounts: vec![ApplicableOnAccount::new("Sales - AC", true)],
        dimensions: vec![AllowedDimension::new("Cost Center", "Main - AC")],
        ..Default::default()
    };

    assert_eq!(filter.custom_hooks(), ["before_save", "validate"]);
    assert_eq!(filter.doctype(), "Accounting Dimension Filter");
    assert_eq!(filter.module(), "Accounts");

    filter.before_save();
    assert_eq!(filter.allow_or_restrict, "Restrict");
    assert!(filter.dimensions.is_empty());

    assert_eq!(filter.validate(None, &[]), Ok(()));
    assert_eq!(filter.fieldname.as_deref(), Some("cost_center"));

    let mut with_lookup = AccountingDimensionFilter {
        accounting_dimension: Some("Custom Dimension".to_string()),
        accounts: vec![ApplicableOnAccount::new("Sales - AC", false)],
        ..filter.clone()
    };
    assert_eq!(with_lookup.validate(Some("custom_dimension"), &[]), Ok(()));
    assert_eq!(with_lookup.fieldname.as_deref(), Some("custom_dimension"));

    assert_eq!(
        with_lookup.validate(None, &["Sales - AC".to_string()]),
        Err(AccountingDimensionFilterError::DuplicateApplicableAccount {
            row: 1,
            account: "Sales - AC".to_string(),
            accounting_dimension: "Custom Dimension".to_string(),
        })
    );
}

#[test]
fn accounting_dimension_filter_map_matches_erpnext_build_map() {
    let rows = vec![
        DimensionFilterRow {
            applicable_on_account: "Sales - AC".to_string(),
            dimension_value: Some("Main - AC".to_string()),
            accounting_dimension: "Cost Center".to_string(),
            allow_or_restrict: "Allow".to_string(),
            fieldname: "cost_center".to_string(),
            is_mandatory: true,
        },
        DimensionFilterRow {
            applicable_on_account: "Sales - AC".to_string(),
            dimension_value: Some("Branch - AC".to_string()),
            accounting_dimension: "Cost Center".to_string(),
            allow_or_restrict: "Allow".to_string(),
            fieldname: "cost_center".to_string(),
            is_mandatory: true,
        },
        DimensionFilterRow {
            applicable_on_account: "Bank - AC".to_string(),
            dimension_value: None,
            accounting_dimension: "Project".to_string(),
            allow_or_restrict: "Restrict".to_string(),
            fieldname: "project".to_string(),
            is_mandatory: false,
        },
    ];

    let mut expected = HashMap::new();
    expected.insert(
        ("cost_center".to_string(), "Sales - AC".to_string()),
        DimensionFilterInfo {
            allowed_dimensions: vec!["Main - AC".to_string(), "Branch - AC".to_string()],
            is_mandatory: true,
            allow_or_restrict: "Allow".to_string(),
        },
    );
    expected.insert(
        ("project".to_string(), "Bank - AC".to_string()),
        DimensionFilterInfo {
            allowed_dimensions: vec![],
            is_mandatory: false,
            allow_or_restrict: "Restrict".to_string(),
        },
    );

    assert_eq!(build_dimension_filter_map(&rows), expected);
}
