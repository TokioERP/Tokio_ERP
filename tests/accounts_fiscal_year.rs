use tokio_erp::erpnext::accounts::doctype::fiscal_year::fiscal_year::{
    auto_create_fiscal_year_plan, get_from_and_to_date, FiscalYear, FiscalYearError,
    FiscalYearOverlap,
};
use tokio_erp::erpnext::accounts::doctype::fiscal_year_company::fiscal_year_company::FiscalYearCompany;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn fiscal_year_matches_erpnext_metadata() {
    assert_eq!(FiscalYear::DOCTYPE, "Fiscal Year");
    assert_eq!(FiscalYear::MODULE, "Accounts");
    assert_eq!(
        FiscalYear::FIELD_ORDER,
        [
            "year",
            "disabled",
            "is_short_year",
            "year_start_date",
            "year_end_date",
            "companies",
            "auto_created",
        ]
    );
    assert_eq!(FiscalYear::SORT_FIELD, "name");
    assert_eq!(FiscalYear::SORT_ORDER, "DESC");

    assert_eq!(
        FiscalYear::fields(),
        vec![
            FieldSpec::data("year", "Year Name")
                .description("For e.g. 2012, 2012-13")
                .oldfield("year", "Data")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::check("is_short_year", "Is Short/Long Year")
                .default("0")
                .description("More/Less than 12 months.")
                .set_only_once(),
            FieldSpec::date("year_start_date", "Year Start Date")
                .oldfield("year_start_date", "Date")
                .in_list_view()
                .no_copy()
                .required()
                .set_only_once(),
            FieldSpec::date("year_end_date", "Year End Date")
                .in_list_view()
                .no_copy()
                .required()
                .set_only_once(),
            FieldSpec::table("companies", "Companies").options("Fiscal Year Company"),
            FieldSpec::check("auto_created", "Auto Created")
                .default("0")
                .hidden()
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn fiscal_year_validates_dates_and_overlaps_like_erpnext() {
    let fy = FiscalYear {
        year: "2026".to_string(),
        year_start_date: "2026-01-01".to_string(),
        year_end_date: "2026-12-31".to_string(),
        ..Default::default()
    };
    assert_eq!(fy.custom_hooks(), ["validate", "on_update", "on_trash"]);
    assert_eq!(fy.validate(&[]), Ok(()));
    assert_eq!(fy.doctype(), "Fiscal Year");
    assert_eq!(fy.module(), "Accounts");

    let wrong_end = FiscalYear {
        year: "2026".to_string(),
        year_start_date: "2026-01-01".to_string(),
        year_end_date: "2026-12-30".to_string(),
        ..Default::default()
    };
    assert_eq!(
        wrong_end.validate(&[]),
        Err(FiscalYearError::InvalidYearEndDate)
    );

    let extra_year = FiscalYear {
        year: "_Test Fiscal Year 2000".to_string(),
        year_start_date: "2000-04-01".to_string(),
        year_end_date: "2002-12-31".to_string(),
        ..Default::default()
    };
    assert_eq!(
        extra_year.validate(&[]),
        Err(FiscalYearError::InvalidYearEndDate)
    );

    let short_year = FiscalYear {
        is_short_year: true,
        ..wrong_end
    };
    assert_eq!(short_year.validate(&[]), Ok(()));

    let overlap = FiscalYearOverlap {
        name: "2025".to_string(),
        companies: vec![],
    };
    assert_eq!(
        fy.validate(&[overlap]),
        Err(FiscalYearError::OverlappingFiscalYear {
            fiscal_year: "2025".to_string()
        })
    );

    let company_specific = FiscalYear {
        companies: vec![FiscalYearCompany::new("Acme")],
        ..fy
    };
    assert_eq!(
        company_specific.validate(&[FiscalYearOverlap {
            name: "_Test Global FY 2001".to_string(),
            companies: vec![],
        }]),
        Ok(())
    );
    assert_eq!(
        company_specific.validate(&[FiscalYearOverlap {
            name: "2025".to_string(),
            companies: vec!["Other".to_string()],
        }]),
        Ok(())
    );
    assert_eq!(
        company_specific.validate(&[FiscalYearOverlap {
            name: "2025".to_string(),
            companies: vec!["Acme".to_string()],
        }]),
        Err(FiscalYearError::OverlappingFiscalYear {
            fiscal_year: "2025".to_string()
        })
    );
}

#[test]
fn fiscal_year_auto_create_and_date_lookup_plans_match_erpnext() {
    let current = FiscalYear {
        year: "2026".to_string(),
        year_start_date: "2026-01-01".to_string(),
        year_end_date: "2026-12-31".to_string(),
        disabled: true,
        companies: vec![FiscalYearCompany::new("Acme")],
        ..Default::default()
    };

    let next = auto_create_fiscal_year_plan(&current).expect("next fy");
    assert_eq!(next.year_start_date, "2027-01-01");
    assert_eq!(next.year_end_date, "2027-12-31");
    assert_eq!(next.year, "2027");
    assert!(next.disabled);
    assert!(next.auto_created);
    assert_eq!(next.companies, vec![FiscalYearCompany::new("Acme")]);

    let cross_year = FiscalYear {
        year_end_date: "2026-03-31".to_string(),
        ..current
    };
    let next = auto_create_fiscal_year_plan(&cross_year).expect("next fy");
    assert_eq!(next.year_start_date, "2026-04-01");
    assert_eq!(next.year_end_date, "2027-03-31");
    assert_eq!(next.year, "2026-2027");

    assert_eq!(
        get_from_and_to_date("2026-01-01", "2026-12-31"),
        ("2026-01-01".to_string(), "2026-12-31".to_string())
    );
}
