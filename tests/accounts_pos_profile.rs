use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::doctype::pos_profile::pos_profile::{
    get_child_nodes, get_item_groups, get_permitted_nodes, pos_profile_query,
    set_default_profile_plan, AccountingDimensionCheck, ChildNode, POSItemGroupRow,
    POSPaymentMethodRow, POSProfile, POSProfileError, POSProfileQueryRow, POSProfileUserRow,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_profile() -> POSProfile {
    POSProfile {
        name: Some("POS-1".to_string()),
        company: "Wind Power LLC".to_string(),
        warehouse: "Stores - WP".to_string(),
        currency: "USD".to_string(),
        write_off_account: "Write Off - WP".to_string(),
        write_off_cost_center: "Main - CC".to_string(),
        payments: vec![POSPaymentMethodRow {
            mode_of_payment: "Cash".to_string(),
            default: true,
        }],
        ..Default::default()
    }
}

#[test]
fn pos_profile_matches_erpnext_metadata() {
    assert_eq!(POSProfile::DOCTYPE, "POS Profile");
    assert_eq!(POSProfile::MODULE, "Accounts");
    assert_eq!(POSProfile::AUTONAME, "Prompt");
    assert_eq!(POSProfile::FIELD_ORDER.len(), 59);
    assert_eq!(
        &POSProfile::FIELD_ORDER[..8],
        [
            "company",
            "customer",
            "country",
            "disabled",
            "column_break_9",
            "warehouse",
            "company_address",
            "section_break_15",
        ]
    );

    let fields = POSProfile::fields();
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::table("payments", "Payment Methods")
            .options("POS Payment Method")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("warehouse", "Warehouse")
            .options("Warehouse")
            .required()
    ));
    assert!(fields.contains(&FieldSpec::currency("write_off_limit", "Write Off Limit").required()));
    assert!(fields.contains(
        &FieldSpec::select("action_on_new_invoice", "Action on New Invoice")
            .options("Always Ask\nSave Changes and Load New Invoice\nDiscard Changes and Load New Invoice")
            .default("Always Ask")
    ));

    let controller = POSProfile::default();
    assert_eq!(controller.doctype(), "POS Profile");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(
        controller.custom_hooks(),
        &["validate", "on_update", "on_trash"]
    );
}

#[test]
fn pos_profile_validate_catches_disabled_default_links_duplicates_payments_and_dimensions() {
    let mut profile = base_profile();
    profile.disabled = true;
    assert_eq!(
        profile.validate(
            Some(false),
            true,
            &BTreeMap::new(),
            &[],
            &BTreeMap::new(),
            &BTreeSet::new(),
            &[],
        ),
        Err(POSProfileError::CannotDisableWithOpenSessions {
            profile: "POS-1".to_string(),
        })
    );

    let mut default_conflict = base_profile();
    default_conflict.applicable_for_users = vec![POSProfileUserRow {
        idx: 1,
        user: "cashier@example.com".to_string(),
        default: true,
        ..Default::default()
    }];
    assert_eq!(
        default_conflict.validate(
            None,
            false,
            &BTreeMap::from([(
                (
                    "cashier@example.com".to_string(),
                    "Wind Power LLC".to_string()
                ),
                "POS-OLD".to_string(),
            )]),
            &[],
            &BTreeMap::from([("Cash".to_string(), Some("Cash - WP".to_string()))]),
            &BTreeSet::from([
                "Warehouse:Wind Power LLC:Stores - WP".to_string(),
                "Account:Wind Power LLC:Write Off - WP".to_string(),
                "Cost Center:Wind Power LLC:Main - CC".to_string(),
            ]),
            &[],
        ),
        Err(POSProfileError::DefaultAlreadySet {
            profile: "POS-OLD".to_string(),
            user: "cashier@example.com".to_string(),
        })
    );

    let mut duplicate = base_profile();
    duplicate.item_groups = vec![
        POSItemGroupRow {
            item_group: "Products".to_string(),
        },
        POSItemGroupRow {
            item_group: "Products".to_string(),
        },
    ];
    assert_eq!(
        duplicate.validate(
            None,
            false,
            &BTreeMap::new(),
            &[],
            &BTreeMap::from([("Cash".to_string(), Some("Cash - WP".to_string()))]),
            &BTreeSet::from([
                "Warehouse:Wind Power LLC:Stores - WP".to_string(),
                "Account:Wind Power LLC:Write Off - WP".to_string(),
                "Cost Center:Wind Power LLC:Main - CC".to_string(),
            ]),
            &[],
        ),
        Err(POSProfileError::DuplicateItemGroup)
    );

    let missing_dim = base_profile();
    assert_eq!(
        missing_dim.validate(
            None,
            false,
            &BTreeMap::new(),
            &[],
            &BTreeMap::from([("Cash".to_string(), Some("Cash - WP".to_string()))]),
            &BTreeSet::from([
                "Warehouse:Wind Power LLC:Stores - WP".to_string(),
                "Account:Wind Power LLC:Write Off - WP".to_string(),
                "Cost Center:Wind Power LLC:Main - CC".to_string(),
            ]),
            &[AccountingDimensionCheck {
                label: "Project".to_string(),
                fieldname: "project".to_string(),
                company: "Wind Power LLC".to_string(),
                mandatory_for_pl: true,
                mandatory_for_bs: false,
            }],
        ),
        Err(POSProfileError::MandatoryAccountingDimension {
            label: "Project".to_string(),
        })
    );

    assert_eq!(
        base_profile().validate(
            None,
            false,
            &BTreeMap::new(),
            &[],
            &BTreeMap::from([("Cash".to_string(), Some("Cash - WP".to_string()))]),
            &BTreeSet::from(["Warehouse:Wind Power LLC:Stores - WP".to_string()]),
            &[],
        ),
        Ok(Vec::new())
    );
}

#[test]
fn pos_profile_validate_payment_method_rules_match_erpnext() {
    let mut no_payment = base_profile();
    no_payment.payments.clear();
    assert_eq!(
        no_payment.validate_payment_methods(&BTreeMap::new()),
        Err(POSProfileError::MissingPaymentMethods)
    );

    let mut no_default = base_profile();
    no_default.payments[0].default = false;
    assert_eq!(
        no_default.validate_payment_methods(&BTreeMap::from([(
            "Cash".to_string(),
            Some("Cash - WP".to_string()),
        )])),
        Err(POSProfileError::MissingDefaultPaymentMethod)
    );

    let mut two_defaults = base_profile();
    two_defaults.payments.push(POSPaymentMethodRow {
        mode_of_payment: "Card".to_string(),
        default: true,
    });
    assert_eq!(
        two_defaults.validate_payment_methods(&BTreeMap::from([
            ("Cash".to_string(), Some("Cash - WP".to_string())),
            ("Card".to_string(), Some("Bank - WP".to_string())),
        ])),
        Err(POSProfileError::MultipleDefaultPaymentMethods)
    );

    assert_eq!(
        base_profile().validate_payment_methods(&BTreeMap::from([("Cash".to_string(), None)])),
        Err(POSProfileError::MissingModeOfPaymentAccount {
            modes: vec!["Cash".to_string()],
        })
    );
}

#[test]
fn pos_profile_defaults_item_groups_and_permissions_follow_erpnext_helpers() {
    let plan = base_profile().set_defaults_plan(
        true,
        &[
            POSProfileUserRow {
                user: "cashier@example.com".to_string(),
                default: true,
                ..Default::default()
            },
            POSProfileUserRow {
                user: String::new(),
                default: true,
                ..Default::default()
            },
        ],
    );
    assert!(plan.clear_is_pos);
    assert_eq!(
        plan.user_defaults,
        vec![("cashier@example.com".to_string(), true)]
    );
    assert_eq!(plan.global_default, Some(true));

    let nodes = vec![
        ChildNode {
            name: "All Item Groups".to_string(),
            lft: 1,
            rgt: 6,
            is_group: true,
        },
        ChildNode {
            name: "Products".to_string(),
            lft: 2,
            rgt: 3,
            is_group: false,
        },
        ChildNode {
            name: "Services".to_string(),
            lft: 4,
            rgt: 5,
            is_group: false,
        },
    ];
    assert_eq!(
        get_child_nodes(&nodes, "All Item Groups"),
        vec!["All Item Groups", "Products", "Services"]
    );
    assert_eq!(
        get_permitted_nodes(&nodes, &["All Item Groups".to_string()]),
        vec!["All Item Groups", "Products", "Services"]
    );
    assert_eq!(
        get_permitted_nodes(
            &nodes,
            &["All Item Groups".to_string(), "Products".to_string()]
        ),
        vec!["All Item Groups", "Products", "Services", "Products"]
    );

    let mut profile = base_profile();
    profile.item_groups = vec![POSItemGroupRow {
        item_group: "All Item Groups".to_string(),
    }];
    let groups = get_item_groups(&profile, &nodes, &BTreeSet::from(["Products".to_string()]));
    assert_eq!(groups, BTreeSet::from(["Products".to_string()]));
}

#[test]
fn pos_profile_query_and_set_default_profile_plan_match_sql_paths() {
    let rows = vec![
        POSProfileQueryRow {
            name: "POS-User".to_string(),
            company: "Wind Power LLC".to_string(),
            disabled: false,
            user: Some("cashier@example.com".to_string()),
        },
        POSProfileQueryRow {
            name: "POS-Open".to_string(),
            company: "Wind Power LLC".to_string(),
            disabled: false,
            user: None,
        },
        POSProfileQueryRow {
            name: "POS-Open-2".to_string(),
            company: "Wind Power LLC".to_string(),
            disabled: false,
            user: None,
        },
    ];

    assert_eq!(
        pos_profile_query(
            &rows,
            "cashier@example.com",
            "POS",
            0,
            20,
            Some("Wind Power LLC"),
        ),
        vec!["POS-User".to_string()]
    );
    assert_eq!(
        pos_profile_query(
            &rows,
            "other@example.com",
            "POS",
            1,
            1,
            Some("Wind Power LLC")
        ),
        vec!["POS-Open".to_string(), "POS-Open-2".to_string()]
    );

    let plan = set_default_profile_plan(
        "POS-User",
        "Wind Power LLC",
        "cashier@example.com",
        "2026-06-01",
    );
    assert_eq!(plan.clear_default_for_user, "cashier@example.com");
    assert_eq!(plan.set_default_profile.as_deref(), Some("POS-User"));
    assert_eq!(plan.modified_by, "cashier@example.com");

    let no_op = set_default_profile_plan("POS-User", "", "cashier@example.com", "2026-06-01");
    assert_eq!(no_op.clear_default_for_user, "");
    assert_eq!(no_op.set_default_profile, None);
    assert_eq!(no_op.modified, "");
    assert_eq!(no_op.modified_by, "");
}

#[test]
fn pos_profile_validate_preserves_non_blocking_default_warnings() {
    let mut profile = base_profile();
    profile.applicable_for_users = vec![POSProfileUserRow {
        idx: 9,
        user: "cashier@example.com".to_string(),
        default: false,
        ..Default::default()
    }];

    let warnings = profile
        .validate(
            None,
            false,
            &BTreeMap::new(),
            &[],
            &BTreeMap::from([("Cash".to_string(), Some("Cash - WP".to_string()))]),
            &BTreeSet::from(["Warehouse:Wind Power LLC:Stores - WP".to_string()]),
            &[],
        )
        .unwrap();

    assert_eq!(
        warnings,
        vec![
            "User cashier@example.com doesn't have any default POS Profile. Check Default at Row 9 for this User."
                .to_string()
        ]
    );
}
