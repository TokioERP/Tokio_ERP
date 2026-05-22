use tokio_erp::erpnext::accounts::doctype::cost_center_allocation_percentage::cost_center_allocation_percentage::CostCenterAllocationPercentage;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn cost_center_allocation_percentage_matches_erpnext_metadata() {
    assert_eq!(
        CostCenterAllocationPercentage::DOCTYPE,
        "Cost Center Allocation Percentage"
    );
    assert_eq!(CostCenterAllocationPercentage::MODULE, "Accounts");
    assert_eq!(
        CostCenterAllocationPercentage::FIELD_ORDER,
        ["cost_center", "percentage"]
    );
    assert!(CostCenterAllocationPercentage::IS_TABLE);
    assert!(CostCenterAllocationPercentage::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        CostCenterAllocationPercentage::fields(),
        vec![
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .required()
                .in_list_view(),
            FieldSpec::percent("percentage", "Percentage (%)")
                .required()
                .in_list_view(),
        ]
    );
}

#[test]
fn cost_center_allocation_percentage_preserves_pass_controller_behavior() {
    let blank = CostCenterAllocationPercentage::default();
    assert_eq!(blank.cost_center, None);
    assert_eq!(blank.percentage, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CostCenterAllocationPercentage::new("Main - TC", 37.5);
    assert_eq!(row.cost_center.as_deref(), Some("Main - TC"));
    assert_eq!(row.percentage, Some(37.5));
    assert_eq!(row.doctype(), "Cost Center Allocation Percentage");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
