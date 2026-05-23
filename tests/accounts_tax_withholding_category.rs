use tokio_erp::erpnext::accounts::doctype::tax_withholding_account::tax_withholding_account::TaxWithholdingAccount;
use tokio_erp::erpnext::accounts::doctype::tax_withholding_category::tax_withholding_category::{
    tax_withholding_category_dashboard_items, TaxWithholdingCategory, TaxWithholdingCategoryError,
};
use tokio_erp::erpnext::accounts::doctype::tax_withholding_rate::tax_withholding_rate::TaxWithholdingRate;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn tax_withholding_rate_matches_erpnext_metadata() {
    assert_eq!(TaxWithholdingRate::DOCTYPE, "Tax Withholding Rate");
    assert_eq!(TaxWithholdingRate::MODULE, "Accounts");
    assert_eq!(
        TaxWithholdingRate::FIELD_ORDER,
        [
            "from_date",
            "to_date",
            "tax_withholding_group",
            "column_break_3",
            "tax_withholding_rate",
            "cumulative_threshold",
            "single_threshold",
        ]
    );
    assert!(TaxWithholdingRate::IS_TABLE);
    assert!(TaxWithholdingRate::EDITABLE_GRID);
    assert!(TaxWithholdingRate::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(TaxWithholdingRate::QUICK_ENTRY);
    assert!(TaxWithholdingRate::TRACK_CHANGES);
    assert!(TaxWithholdingRate::fields().contains(
        &FieldSpec::float("tax_withholding_rate", "Tax Withholding Rate")
            .required()
            .in_list_view()
            .columns(1)
    ));
}

#[test]
fn tax_withholding_category_matches_erpnext_metadata_and_dashboard() {
    assert_eq!(TaxWithholdingCategory::DOCTYPE, "Tax Withholding Category");
    assert_eq!(TaxWithholdingCategory::MODULE, "Accounts");
    assert_eq!(TaxWithholdingCategory::AUTONAME, "Prompt");
    assert_eq!(TaxWithholdingCategory::FIELD_ORDER.len(), 12);
    assert!(TaxWithholdingCategory::ALLOW_IMPORT);
    assert!(TaxWithholdingCategory::ALLOW_RENAME);
    assert!(TaxWithholdingCategory::EDITABLE_GRID);
    assert!(TaxWithholdingCategory::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(TaxWithholdingCategory::TRACK_CHANGES);
    assert_eq!(tax_withholding_category_dashboard_items(), ["Supplier"]);

    let fields = TaxWithholdingCategory::fields();
    assert_eq!(fields.len(), 12);
    assert!(fields.contains(
        &FieldSpec::select("tax_deduction_basis", "Deduct Tax On Basis")
            .options("\nGross Total\nNet Total")
            .default("Net Total")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::table("rates", "Rates")
            .options("Tax Withholding Rate")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::table("accounts", "Accounts")
            .options("Tax Withholding Account")
            .required()
    ));
}

#[test]
fn tax_withholding_category_validation_matches_erpnext_date_company_and_threshold_rules() {
    let mut doc = TaxWithholdingCategory::new("TDS");
    doc.accounts = vec![
        TaxWithholdingAccount::new("_Test Company", "TDS - TC"),
        TaxWithholdingAccount::new("_Test Company", "TDS 2 - TC"),
    ];
    doc.rates = vec![TaxWithholdingRate::new(
        "2026-01-01",
        "2026-12-31",
        "",
        10.0,
    )];
    assert_eq!(
        doc.validate().unwrap_err(),
        TaxWithholdingCategoryError::DuplicateCompany(
            "Company _Test Company added multiple times".to_string()
        )
    );

    let mut overlap = TaxWithholdingCategory::new("TDS");
    overlap.rates = vec![
        TaxWithholdingRate::new("2026-01-01", "2026-06-30", "Group A", 10.0),
        TaxWithholdingRate::new("2026-06-01", "2026-12-31", "Group A", 10.0),
    ];
    assert_eq!(
        overlap.validate_dates().unwrap_err(),
        TaxWithholdingCategoryError::OverlappingDates(
            "Row #2: Dates overlapping with other row in group Group A".to_string()
        )
    );

    let mut thresholds = TaxWithholdingCategory::new("TDS");
    thresholds.rates = vec![TaxWithholdingRate {
        cumulative_threshold: 500.0,
        single_threshold: 1000.0,
        ..TaxWithholdingRate::new("2026-01-01", "2026-12-31", "", 10.0)
    }];
    assert_eq!(
        thresholds.validate_thresholds().unwrap_err(),
        TaxWithholdingCategoryError::InvalidThreshold(
            "Row #1: Cumulative threshold cannot be less than Single Transaction threshold"
                .to_string()
        )
    );
}

#[test]
fn tax_withholding_category_lookup_helpers_match_erpnext() {
    let mut doc = TaxWithholdingCategory::new("TDS");
    doc.accounts = vec![TaxWithholdingAccount::new("_Test Company", "TDS - TC")];
    doc.rates = vec![
        TaxWithholdingRate::new("2026-01-01", "2026-06-30", "", 10.0),
        TaxWithholdingRate::new("2026-07-01", "2026-12-31", "Group A", 5.0),
    ];

    assert_eq!(
        doc.get_applicable_tax_row("2026-08-15", "Group A")
            .unwrap()
            .tax_withholding_rate,
        5.0
    );
    assert_eq!(
        doc.get_company_account("_Test Company").unwrap(),
        "TDS - TC".to_string()
    );
    assert_eq!(
        doc.get_company_account("Missing Co").unwrap_err(),
        TaxWithholdingCategoryError::MissingAccount(
            "No Tax withholding account set for Company Missing Co in Tax Withholding Category TDS."
                .to_string()
        )
    );
    assert_eq!(doc.custom_hooks(), ["validate"]);
    assert_eq!(doc.doctype(), "Tax Withholding Category");
    assert_eq!(doc.module(), "Accounts");
}
