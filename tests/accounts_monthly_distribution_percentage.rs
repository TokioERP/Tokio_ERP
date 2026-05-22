use tokio_erp::erpnext::accounts::doctype::monthly_distribution_percentage::monthly_distribution_percentage::MonthlyDistributionPercentage;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn monthly_distribution_percentage_matches_erpnext_metadata() {
    assert_eq!(
        MonthlyDistributionPercentage::DOCTYPE,
        "Monthly Distribution Percentage"
    );
    assert_eq!(MonthlyDistributionPercentage::MODULE, "Accounts");
    assert_eq!(
        MonthlyDistributionPercentage::FIELD_ORDER,
        ["month", "percentage_allocation"]
    );
    assert!(MonthlyDistributionPercentage::IS_TABLE);
    assert!(MonthlyDistributionPercentage::EDITABLE_GRID);
    assert_eq!(MonthlyDistributionPercentage::AUTONAME, Some("hash"));
    assert_eq!(MonthlyDistributionPercentage::IDX, Some(1));

    assert_eq!(
        MonthlyDistributionPercentage::fields(),
        vec![
            FieldSpec::data("month", "Month")
                .required()
                .read_only()
                .in_list_view()
                .oldfield("month", "Data"),
            FieldSpec::float("percentage_allocation", "Percentage Allocation")
                .in_list_view()
                .oldfield("percentage_allocation", "Currency"),
        ]
    );
}

#[test]
fn monthly_distribution_percentage_preserves_pass_controller_behavior() {
    let blank = MonthlyDistributionPercentage::default();
    assert_eq!(blank.month, None);
    assert_eq!(blank.percentage_allocation, None);
    assert!(blank.custom_hooks().is_empty());

    let row = MonthlyDistributionPercentage::new("January", 8.33);
    assert_eq!(row.month.as_deref(), Some("January"));
    assert_eq!(row.percentage_allocation, Some(8.33));
    assert_eq!(row.doctype(), "Monthly Distribution Percentage");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
