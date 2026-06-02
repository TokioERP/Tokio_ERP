use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::doctype::accounting_dimension::accounting_dimension::{
    add_dimension_to_budget_doctype, create_accounting_dimensions_for_doctype_plan,
    delete_accounting_dimension_plan, get_dimension_with_children, get_dimensions,
    make_dimension_in_accounting_doctypes_plan, toggle_disabling_plan, AccountingDimension,
    AccountingDimensionDefault, AccountingDimensionError, AccountingDimensionRecord,
    CustomFieldPlan, DimensionNode, DimensionTogglePlan, PropertySetterPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn accounting_dimension_matches_erpnext_metadata() {
    assert_eq!(AccountingDimension::DOCTYPE, "Accounting Dimension");
    assert_eq!(AccountingDimension::MODULE, "Accounts");
    assert_eq!(AccountingDimension::AUTONAME, "field:label");
    assert_eq!(
        AccountingDimension::FIELD_ORDER,
        [
            "document_type",
            "label",
            "fieldname",
            "dimension_defaults",
            "disabled",
        ]
    );

    let fields = AccountingDimension::fields();
    assert!(fields.contains(
        &FieldSpec::link("document_type", "Reference Document Type")
            .options("DocType")
            .required()
            .search_index()
            .read_only_depends_on("eval:!doc.__islocal")
    ));
    assert!(fields.contains(
        &FieldSpec::data("label", "Dimension Name")
            .unique()
            .in_list_view()
    ));
    assert!(fields.contains(&FieldSpec::data("fieldname", "Fieldname").hidden()));
    assert!(fields.contains(
        &FieldSpec::table("dimension_defaults", "Dimension Defaults")
            .options("Accounting Dimension Detail")
    ));
    assert!(fields.contains(
        &FieldSpec::check("disabled", "Disable")
            .default("0")
            .read_only()
            .hidden()
    ));

    let controller = AccountingDimension::default();
    assert_eq!(controller.doctype(), "Accounting Dimension");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(
        controller.custom_hooks(),
        &["before_insert", "validate", "on_update", "on_trash"]
    );
}

#[test]
fn accounting_dimension_sets_fieldname_and_validates_same_rules_as_erpnext() {
    let mut doc = AccountingDimension {
        name: Some("Sales Region".to_string()),
        document_type: "Sales Region".to_string(),
        label: None,
        fieldname: None,
        dimension_defaults: vec![
            AccountingDimensionDefault {
                parent: None,
                company: "Wind Power LLC".to_string(),
                default_dimension: Some("North".to_string()),
                mandatory_for_pl: true,
                mandatory_for_bs: false,
            },
            AccountingDimensionDefault {
                parent: None,
                company: "Solar LLC".to_string(),
                default_dimension: Some("South".to_string()),
                mandatory_for_pl: false,
                mandatory_for_bs: true,
            },
        ],
        ..Default::default()
    };

    doc.before_insert();
    assert_eq!(doc.label.as_deref(), Some("Sales Region"));
    assert_eq!(doc.fieldname.as_deref(), Some("sales_region"));
    assert_eq!(
        doc.validate(&BTreeSet::new(), false, None, &[]),
        Ok(Vec::<String>::new())
    );

    let duplicate_company = AccountingDimension {
        document_type: "Sales Region".to_string(),
        fieldname: Some("sales_region".to_string()),
        dimension_defaults: vec![
            AccountingDimensionDefault {
                company: "Wind Power LLC".to_string(),
                ..Default::default()
            },
            AccountingDimensionDefault {
                company: "Wind Power LLC".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    assert_eq!(
        duplicate_company.validate(&BTreeSet::new(), false, None, &[]),
        Err(AccountingDimensionError::DuplicateDefaultCompany {
            company: "Wind Power LLC".to_string(),
        })
    );

    let prohibited = AccountingDimension {
        document_type: "Company".to_string(),
        fieldname: Some("company".to_string()),
        ..Default::default()
    };
    assert_eq!(
        prohibited.validate(&BTreeSet::new(), false, None, &[]),
        Err(AccountingDimensionError::NotAllowedDocumentType {
            document_type: "Company".to_string(),
        })
    );

    let changed = AccountingDimension {
        document_type: "Region".to_string(),
        fieldname: Some("region".to_string()),
        is_new: false,
        ..Default::default()
    };
    assert_eq!(
        changed.validate(&BTreeSet::new(), false, Some("Territory"), &[]),
        Err(AccountingDimensionError::DocumentTypeChanged)
    );
}

#[test]
fn accounting_dimension_reports_fieldname_conflicts_but_allows_save() {
    let doc = AccountingDimension {
        document_type: "Sales Region".to_string(),
        fieldname: Some("cost_center".to_string()),
        ..Default::default()
    };

    let warnings = doc
        .validate(
            &BTreeSet::new(),
            false,
            None,
            &[
                ("Sales Invoice".to_string(), vec!["cost_center".to_string()]),
                ("Purchase Invoice".to_string(), vec!["project".to_string()]),
            ],
        )
        .expect("conflicts are warnings");

    assert_eq!(warnings, vec!["Sales Invoice".to_string()]);
}

#[test]
fn accounting_dimension_builds_custom_field_plans_for_accounting_doctypes() {
    let doc = AccountingDimension {
        document_type: "Sales Region".to_string(),
        label: Some("Sales Region".to_string()),
        fieldname: Some("sales_region".to_string()),
        ..Default::default()
    };
    let plans = make_dimension_in_accounting_doctypes_plan(
        &doc,
        &["Sales Invoice", "Budget", "Payment Entry"],
        1,
        &["Sales Invoice"],
        &BTreeMap::from([(
            "Payment Entry".to_string(),
            vec!["sales_region".to_string()],
        )]),
        None,
    );

    assert_eq!(
        plans.custom_fields,
        vec![
            CustomFieldPlan {
                doctype: "Sales Invoice".to_string(),
                fieldname: "sales_region".to_string(),
                label: "Sales Region".to_string(),
                fieldtype: "Link".to_string(),
                options: "Sales Region".to_string(),
                insert_after: "dimension_col_break".to_string(),
                owner: Some("Administrator".to_string()),
                allow_on_submit: true,
                depends_on: None,
                read_only: None,
            },
            CustomFieldPlan {
                doctype: "Budget".to_string(),
                fieldname: "sales_region".to_string(),
                label: "Sales Region".to_string(),
                fieldtype: "Link".to_string(),
                options: "Sales Region".to_string(),
                insert_after: "cost_center".to_string(),
                owner: Some("Administrator".to_string()),
                allow_on_submit: false,
                depends_on: Some("eval:doc.budget_against == 'Sales Region'".to_string()),
                read_only: None,
            },
        ]
    );
    assert_eq!(
        plans.property_setters,
        vec![PropertySetterPlan {
            name: "Budget-budget_against-options".to_string(),
            doctype_or_field: "DocField".to_string(),
            doc_type: "Budget".to_string(),
            field_name: "budget_against".to_string(),
            property: "options".to_string(),
            property_type: "Text".to_string(),
            value: "\nCost Center\nProject\nSales Region".to_string(),
        }]
    );
    assert_eq!(
        plans.clear_cache_doctypes,
        vec!["Sales Invoice", "Budget", "Payment Entry"]
    );
}

#[test]
fn accounting_dimension_budget_plan_appends_existing_property_setter_options() {
    let doc = AccountingDimension {
        document_type: "Branch".to_string(),
        label: Some("Branch".to_string()),
        fieldname: Some("branch".to_string()),
        ..Default::default()
    };
    let (custom_field, property_setter) = add_dimension_to_budget_doctype(
        CustomFieldPlan {
            doctype: "Budget".to_string(),
            fieldname: "branch".to_string(),
            label: "Branch".to_string(),
            fieldtype: "Link".to_string(),
            options: "Branch".to_string(),
            insert_after: "accounting_dimensions_section".to_string(),
            owner: Some("Administrator".to_string()),
            allow_on_submit: false,
            depends_on: None,
            read_only: None,
        },
        &doc,
        Some("\nCost Center\nProject\nRegion"),
    );

    assert_eq!(custom_field.insert_after, "cost_center");
    assert_eq!(
        custom_field.depends_on.as_deref(),
        Some("eval:doc.budget_against == 'Branch'")
    );
    assert_eq!(
        property_setter.value,
        "\nCost Center\nProject\nRegion\nBranch"
    );
}

#[test]
fn accounting_dimension_delete_and_toggle_plans_match_erpnext_side_effects() {
    let doc = AccountingDimension {
        document_type: "Branch".to_string(),
        fieldname: Some("branch".to_string()),
        ..Default::default()
    };

    let delete_plan = delete_accounting_dimension_plan(
        &doc,
        &["Sales Invoice", "Budget"],
        "\nCost Center\nProject\nRegion\nBranch",
    );
    assert_eq!(delete_plan.fieldname, "branch");
    assert_eq!(delete_plan.doctypes, vec!["Sales Invoice", "Budget"]);
    assert_eq!(
        delete_plan.budget_against_options,
        "\nCost Center\nProject\nRegion"
    );

    let toggle_plan = toggle_disabling_plan(
        "branch",
        true,
        &["Sales Invoice", "Budget"],
        &BTreeSet::from(["Sales Invoice:branch".to_string()]),
    );
    assert_eq!(
        toggle_plan,
        DimensionTogglePlan {
            updates: vec![("Sales Invoice".to_string(), "branch".to_string(), true)],
            clear_cache_doctypes: vec!["Sales Invoice".to_string(), "Budget".to_string()],
        }
    );
}

#[test]
fn accounting_dimension_delete_keeps_budget_options_trailing_newline_like_erpnext() {
    let doc = AccountingDimension {
        document_type: "Branch".to_string(),
        fieldname: Some("branch".to_string()),
        ..Default::default()
    };

    let delete_plan =
        delete_accounting_dimension_plan(&doc, &["Budget"], "\nCost Center\nProject\nBranch");

    assert_eq!(
        delete_plan.budget_against_options,
        "\nCost Center\nProject\n"
    );
}

#[test]
fn accounting_dimension_helpers_return_dimensions_defaults_children_and_create_plan() {
    let records = vec![
        AccountingDimensionRecord {
            label: "Region".to_string(),
            fieldname: "region".to_string(),
            disabled: false,
            document_type: "Territory".to_string(),
        },
        AccountingDimensionRecord {
            label: "Disabled".to_string(),
            fieldname: "disabled_dim".to_string(),
            disabled: true,
            document_type: "Project".to_string(),
        },
    ];
    let defaults = vec![AccountingDimensionDefault {
        parent: Some("Region".to_string()),
        company: "Wind Power LLC".to_string(),
        default_dimension: Some("North".to_string()),
        ..Default::default()
    }];

    let (dimensions, default_map) = get_dimensions(&records, &defaults, "true");
    assert_eq!(
        dimensions,
        vec![
            AccountingDimensionRecord {
                label: "Region".to_string(),
                fieldname: "region".to_string(),
                disabled: false,
                document_type: "Territory".to_string(),
            },
            AccountingDimensionRecord {
                label: String::new(),
                fieldname: "cost_center".to_string(),
                disabled: false,
                document_type: "Cost Center".to_string(),
            },
            AccountingDimensionRecord {
                label: String::new(),
                fieldname: "project".to_string(),
                disabled: false,
                document_type: "Project".to_string(),
            },
        ]
    );
    assert_eq!(
        default_map["Wind Power LLC"]["region"],
        Some("North".to_string())
    );

    let children = get_dimension_with_children(
        &[
            DimensionNode {
                name: "All".to_string(),
                lft: 1,
                rgt: 6,
            },
            DimensionNode {
                name: "North".to_string(),
                lft: 2,
                rgt: 3,
            },
            DimensionNode {
                name: "South".to_string(),
                lft: 4,
                rgt: 5,
            },
        ],
        &["All".to_string()],
    );
    assert_eq!(children, vec!["All", "North", "South"]);

    let create_plan =
        create_accounting_dimensions_for_doctype_plan("Sales Invoice", &records, &BTreeSet::new());
    assert_eq!(create_plan.custom_fields.len(), 2);
    assert_eq!(
        create_plan.custom_fields[0].insert_after,
        "accounting_dimensions_section"
    );
    assert_eq!(create_plan.clear_cache_doctypes, vec!["Sales Invoice"]);
}
