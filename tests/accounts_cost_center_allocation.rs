use std::collections::HashSet;

use tokio_erp::erpnext::accounts::doctype::cost_center_allocation::cost_center_allocation::{
    add_days, CostCenterAllocation, CostCenterAllocationError, CostCenterAllocationWarning,
    CostCenterAllocationValidationContext, FutureAllocation,
};
use tokio_erp::erpnext::accounts::doctype::cost_center_allocation_percentage::cost_center_allocation_percentage::CostCenterAllocationPercentage;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn cost_center_allocation_matches_erpnext_metadata() {
    assert_eq!(CostCenterAllocation::DOCTYPE, "Cost Center Allocation");
    assert_eq!(CostCenterAllocation::MODULE, "Accounts");
    assert_eq!(CostCenterAllocation::AUTONAME, "CC-ALLOC-.#####");
    assert!(CostCenterAllocation::ALLOW_RENAME);
    assert!(CostCenterAllocation::IS_SUBMITTABLE);
    assert!(CostCenterAllocation::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(CostCenterAllocation::SORT_FIELD, "creation");
    assert_eq!(CostCenterAllocation::SORT_ORDER, "DESC");
    assert_eq!(
        CostCenterAllocation::FIELD_ORDER,
        [
            "main_cost_center",
            "company",
            "column_break_2",
            "valid_from",
            "section_break_5",
            "allocation_percentages",
            "amended_from",
        ]
    );
    assert_eq!(
        CostCenterAllocation::fields(),
        vec![
            FieldSpec::link("main_cost_center", "Main Cost Center")
                .options("Cost Center")
                .in_list_view()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .fetch_from("main_cost_center.company")
                .required(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::date("valid_from", "Valid From")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table(
                "allocation_percentages",
                "Cost Center Allocation Percentages",
            )
            .options("Cost Center Allocation Percentage")
            .required(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Cost Center Allocation")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn cost_center_allocation_validates_like_erpnext() {
    let allocation = CostCenterAllocation {
        name: Some("CC-ALLOC-00001".to_string()),
        main_cost_center: "Main - AC".to_string(),
        company: "Acme".to_string(),
        valid_from: "2026-06-01".to_string(),
        allocation_percentages: vec![
            CostCenterAllocationPercentage::new("Branch A - AC", 60.0),
            CostCenterAllocationPercentage::new("Branch B - AC", 40.0),
        ],
        ..Default::default()
    };
    assert_eq!(allocation.custom_hooks(), ["validate", "clear_cache"]);
    assert_eq!(allocation.doctype(), "Cost Center Allocation");
    assert_eq!(allocation.module(), "Accounts");

    let ctx = CostCenterAllocationValidationContext {
        last_gle_date: Some("2026-05-31".to_string()),
        future_allocation: Some(FutureAllocation {
            name: "CC-ALLOC-00002".to_string(),
            valid_from: "2026-07-01".to_string(),
        }),
        main_cost_center_used_as_child_parent: None,
        active_main_cost_centers: HashSet::new(),
        skip_from_date_validation: false,
    };
    let warning = allocation.validate(&ctx).expect("valid allocation");
    assert_eq!(
        warning,
        Some(CostCenterAllocationWarning {
            allocation_name: "CC-ALLOC-00002".to_string(),
            valid_from: "2026-07-01".to_string(),
            applicable_upto: "2026-06-30".to_string(),
        })
    );

    let wrong_total = CostCenterAllocation {
        allocation_percentages: vec![CostCenterAllocationPercentage::new("Branch A - AC", 99.0)],
        ..allocation.clone()
    };
    assert_eq!(
        wrong_total.validate(&ctx),
        Err(CostCenterAllocationError::WrongPercentageAllocation)
    );

    let bad_date = CostCenterAllocation {
        valid_from: "2026-05-31".to_string(),
        ..allocation.clone()
    };
    assert_eq!(
        bad_date.validate(&ctx),
        Err(CostCenterAllocationError::InvalidDate {
            last_gle_date: "2026-05-31".to_string(),
            main_cost_center: "Main - AC".to_string(),
        })
    );

    let main_as_child = CostCenterAllocation {
        allocation_percentages: vec![
            CostCenterAllocationPercentage::new("Main - AC", 50.0),
            CostCenterAllocationPercentage::new("Branch A - AC", 50.0),
        ],
        ..allocation.clone()
    };
    assert_eq!(
        main_as_child.validate(&CostCenterAllocationValidationContext {
            last_gle_date: None,
            future_allocation: None,
            ..ctx.clone()
        }),
        Err(CostCenterAllocationError::MainCostCenterCantBeChild {
            main_cost_center: "Main - AC".to_string(),
        })
    );

    let invalid_child = CostCenterAllocation {
        allocation_percentages: vec![CostCenterAllocationPercentage::new("Branch A - AC", 100.0)],
        ..allocation
    };
    assert_eq!(
        invalid_child.validate(&CostCenterAllocationValidationContext {
            active_main_cost_centers: HashSet::from(["Branch A - AC".to_string()]),
            last_gle_date: None,
            future_allocation: None,
            ..ctx
        }),
        Err(CostCenterAllocationError::InvalidChildCostCenter {
            cost_center: "Branch A - AC".to_string(),
        })
    );
}

#[test]
fn cost_center_allocation_date_helper_matches_erpnext_warning_upto_date() {
    assert_eq!(add_days("2026-07-01", -1), "2026-06-30");
    assert_eq!(add_days("2024-03-01", -1), "2024-02-29");
    assert_eq!(add_days("2026-12-31", 1), "2027-01-01");
}
