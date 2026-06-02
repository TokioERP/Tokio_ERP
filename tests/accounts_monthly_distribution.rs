use tokio_erp::erpnext::accounts::doctype::monthly_distribution::monthly_distribution::{
    get_percentage, get_periodwise_distribution_data, DistributionPeriod, MonthlyDistribution,
    MonthlyDistributionError,
};
use tokio_erp::erpnext::accounts::doctype::monthly_distribution_percentage::monthly_distribution_percentage::MonthlyDistributionPercentage;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn monthly_distribution_matches_erpnext_metadata() {
    assert_eq!(MonthlyDistribution::DOCTYPE, "Monthly Distribution");
    assert_eq!(MonthlyDistribution::MODULE, "Accounts");
    assert_eq!(
        MonthlyDistribution::FIELD_ORDER,
        ["distribution_id", "fiscal_year", "percentages"]
    );
    assert_eq!(MonthlyDistribution::AUTONAME, "field:distribution_id");
    assert_eq!(
        MonthlyDistribution::DESCRIPTION,
        "Helps you distribute the Budget/Target across months if you have seasonality in your business."
    );
    assert_eq!(MonthlyDistribution::ICON, "fa fa-bar-chart");
    assert_eq!(MonthlyDistribution::SORT_FIELD, "creation");
    assert_eq!(MonthlyDistribution::SORT_ORDER, "DESC");
    assert_eq!(
        MonthlyDistribution::fields(),
        vec![
            FieldSpec::data("distribution_id", "Distribution Name")
                .description("Name of the Monthly Distribution")
                .oldfield("distribution_id", "Data")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::link("fiscal_year", "Fiscal Year")
                .options("Fiscal Year")
                .oldfield("fiscal_year", "Select")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            FieldSpec::table("percentages", "Monthly Distribution Percentages")
                .options("Monthly Distribution Percentage")
                .oldfield("budget_distribution_details", "Table"),
        ]
    );

    let doc = MonthlyDistribution::default();
    assert_eq!(doc.doctype(), "Monthly Distribution");
    assert!(doc.custom_hooks().is_empty());
}

#[test]
fn get_months_populates_twelve_equal_rows_with_one_based_idx() {
    let mut doc = MonthlyDistribution::default();
    doc.get_months();

    assert_eq!(doc.percentages.len(), 12);
    assert_eq!(doc.percentages[0].idx, Some(1));
    assert_eq!(doc.percentages[0].month.as_deref(), Some("January"));
    assert_eq!(doc.percentages[11].idx, Some(12));
    assert_eq!(doc.percentages[11].month.as_deref(), Some("December"));
    assert_eq!(doc.percentages[0].percentage_allocation, Some(100.0 / 12.0));
}

#[test]
fn validate_requires_total_percentage_to_equal_one_hundred_at_two_decimals() {
    let mut doc = MonthlyDistribution::default();
    doc.get_months();
    assert_eq!(doc.validate(), Ok(()));

    doc.percentages[0].percentage_allocation = Some(1.0);
    assert_eq!(
        doc.validate(),
        Err(MonthlyDistributionError::PercentageAllocationMismatch { total: 92.67 })
    );
}

#[test]
fn periodwise_distribution_data_sums_months_by_periodicity() {
    let doc = MonthlyDistribution {
        percentages: vec![
            MonthlyDistributionPercentage::new("January", 10.0),
            MonthlyDistributionPercentage::new("February", 20.0),
            MonthlyDistributionPercentage::new("March", 30.0),
            MonthlyDistributionPercentage::new("April", 40.0),
        ],
        ..Default::default()
    };
    let periods = vec![
        DistributionPeriod::new("q1", "2026-01-15"),
        DistributionPeriod::new("m2", "2026-02-01"),
    ];

    let result = get_periodwise_distribution_data(&doc, &periods, "Quarterly").unwrap();
    assert_eq!(result["q1"], 60.0);
    assert_eq!(result["m2"], 90.0);
    assert_eq!(get_percentage(&doc, "2026-03-31", 1).unwrap(), 30.0);
}
