use tokio_erp::erpnext::accounts::doctype::cost_center::cost_center::{
    get_name_with_number, CostCenter, CostCenterError, CostCenterRenameUpdate,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn cost_center_matches_erpnext_metadata() {
    assert_eq!(CostCenter::DOCTYPE, "Cost Center");
    assert_eq!(CostCenter::MODULE, "Accounts");
    assert_eq!(CostCenter::DEFAULT_VIEW, "Tree");
    assert_eq!(
        CostCenter::DESCRIPTION,
        "Track separate Income and Expense for product verticals or divisions."
    );
    assert_eq!(CostCenter::DOCUMENT_TYPE, "Setup");
    assert_eq!(CostCenter::NSM_PARENT_FIELD, "parent_cost_center");
    assert!(CostCenter::ALLOW_COPY);
    assert!(CostCenter::ALLOW_IMPORT);
    assert!(CostCenter::IS_TREE);
    assert_eq!(CostCenter::SEARCH_FIELDS, "parent_cost_center, is_group");
    assert_eq!(CostCenter::SORT_FIELD, "creation");
    assert_eq!(CostCenter::SORT_ORDER, "ASC");
    assert_eq!(
        CostCenter::FIELD_ORDER,
        [
            "sb0",
            "cost_center_name",
            "cost_center_number",
            "parent_cost_center",
            "company",
            "cb0",
            "is_group",
            "disabled",
            "lft",
            "rgt",
            "old_parent",
        ]
    );

    assert_eq!(
        CostCenter::fields(),
        vec![
            FieldSpec::section_break("sb0"),
            FieldSpec::data("cost_center_name", "Cost Center Name")
                .oldfield("cost_center_name", "Data")
                .in_list_view()
                .no_copy()
                .required(),
            FieldSpec::data("cost_center_number", "Cost Center Number")
                .in_list_view()
                .in_standard_filter()
                .read_only_depends_on("eval:!doc.__islocal"),
            FieldSpec::link("parent_cost_center", "Parent Cost Center")
                .options("Cost Center")
                .oldfield("parent_cost_center", "Link")
                .ignore_user_permissions()
                .in_list_view()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .oldfield("company_name", "Link")
                .in_list_view()
                .in_standard_filter()
                .required(),
            FieldSpec::column_break("cb0").width("50%"),
            FieldSpec::check("is_group", "Is Group").default("0"),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::int("lft", "lft")
                .oldfield("lft", "Int")
                .hidden()
                .no_copy()
                .print_hide()
                .report_hide()
                .search_index(),
            FieldSpec::int("rgt", "rgt")
                .oldfield("rgt", "Int")
                .hidden()
                .no_copy()
                .print_hide()
                .report_hide()
                .search_index(),
            FieldSpec::link("old_parent", "old_parent")
                .options("Cost Center")
                .oldfield("old_parent", "Data")
                .hidden()
                .ignore_user_permissions()
                .no_copy()
                .print_hide()
                .report_hide(),
        ]
    );
}

#[test]
fn cost_center_validate_and_autoname_match_erpnext() {
    let mut cc = CostCenter {
        cost_center_name: "Marketing".to_string(),
        cost_center_number: Some("1200".to_string()),
        company: "Acme".to_string(),
        parent_cost_center: Some("Acme - AC".to_string()),
        ..Default::default()
    };
    assert_eq!(
        cc.custom_hooks(),
        ["autoname", "validate", "before_rename", "after_rename"]
    );
    assert_eq!(cc.doctype(), "Cost Center");
    assert_eq!(cc.module(), "Accounts");
    assert_eq!(cc.autoname_with_company_abbr("AC"), "1200 - Marketing - AC");
    assert_eq!(cc.name.as_deref(), Some("1200 - Marketing - AC"));
    assert_eq!(cc.validate(Some(true)), Ok(()));
    assert_eq!(
        get_name_with_number("Marketing - AC", Some("1200")),
        "1200 - Marketing - AC"
    );

    let missing_parent = CostCenter {
        cost_center_name: "Marketing".to_string(),
        company: "Acme".to_string(),
        parent_cost_center: None,
        ..Default::default()
    };
    assert_eq!(
        missing_parent.validate(None),
        Err(CostCenterError::ParentCostCenterRequired)
    );

    let root_with_parent = CostCenter {
        cost_center_name: "Acme".to_string(),
        company: "Acme".to_string(),
        parent_cost_center: Some("Parent".to_string()),
        ..Default::default()
    };
    assert_eq!(
        root_with_parent.validate(Some(true)),
        Err(CostCenterError::RootCannotHaveParent)
    );

    assert_eq!(
        cc.validate(Some(false)),
        Err(CostCenterError::ParentMustBeGroup {
            parent_cost_center: "Acme - AC".to_string(),
        })
    );
}

#[test]
fn cost_center_conversion_and_rename_helpers_match_erpnext() {
    let mut cc = CostCenter {
        name: Some("1200 - Marketing - AC".to_string()),
        cost_center_name: "Marketing".to_string(),
        cost_center_number: Some("1200".to_string()),
        company: "Acme".to_string(),
        is_group: true,
        ..Default::default()
    };
    assert_eq!(cc.convert_group_to_ledger(false, false), Ok(true));
    assert!(!cc.is_group);
    assert_eq!(cc.convert_ledger_to_group(false, false, false), Ok(true));
    assert!(cc.is_group);
    assert_eq!(
        cc.convert_group_to_ledger(true, false),
        Err(CostCenterError::CannotConvertGroupWithChildren)
    );
    assert_eq!(
        cc.convert_ledger_to_group(true, false, false),
        Err(CostCenterError::CannotConvertLedgerWithAllocationRecords)
    );

    assert_eq!(
        cc.before_rename("Marketing", "AC", false),
        "1200 - Marketing - AC"
    );
    assert_eq!(
        cc.after_rename_plan("1200 - Growth - AC", Some("Marketing"), Some("1200")),
        Some(CostCenterRenameUpdate {
            cost_center_number: None,
            cost_center_name: Some("Growth".to_string()),
        })
    );
}
