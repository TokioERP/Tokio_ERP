use serde_json::json;
use tokio_erp::erpnext::accounts::doctype::account::chart_of_accounts::chart_of_accounts::{
    add_suffix_if_duplicate, build_account_tree_from_rows, build_tree_from_json, create_chart_plan,
    get_chart_from_files, get_chart_metadata_fields, get_chart_template,
    get_charts_for_country_from_files, identify_is_group, validate_bank_account, AccountSourceRow,
    ChartFile, ChartTreeNode, CreateChartOptions, PlannedAccount,
};

#[test]
fn chart_metadata_fields_match_erpnext_source_order() {
    assert_eq!(
        get_chart_metadata_fields(),
        [
            "account_name",
            "account_number",
            "account_type",
            "account_category",
            "root_type",
            "is_group",
            "tax_rate",
            "account_currency",
        ]
    );
}

#[test]
fn duplicate_suffix_and_group_detection_match_python_helpers() {
    let accounts = vec![
        "1000 - cash".to_string(),
        "1000 - cash".to_string(),
        "debtors".to_string(),
    ];

    assert_eq!(
        add_suffix_if_duplicate(" Cash ", Some("1000"), &accounts),
        (" Cash  2".to_string(), "1000 - cash".to_string())
    );
    assert_eq!(
        add_suffix_if_duplicate("Debtors", None, &accounts),
        ("Debtors 1".to_string(), "debtors".to_string())
    );
    assert_eq!(
        add_suffix_if_duplicate("Café", None, &[]),
        ("Café".to_string(), "cafe".to_string())
    );

    assert_eq!(identify_is_group(&json!({"is_group": 1})), 1);
    assert_eq!(identify_is_group(&json!({"account_type": "Bank"})), 0);
    assert_eq!(
        identify_is_group(&json!({"Child": {}, "account_type": "Bank"})),
        1
    );
}

#[test]
fn build_tree_from_json_matches_erpnext_flattening() {
    let chart = json!({
        "Assets": {
            "Cash In Hand": {
                "Cash": {"account_number": "1110", "account_type": "Cash"},
                "account_number": "1100",
            },
            "root_type": "Asset",
            "account_number": "1000",
        },
        "Expenses": {
            "Office Rent": {"account_category": "Operating Expenses"},
            "root_type": "Expense",
        },
    });

    assert_eq!(
        build_tree_from_json(&chart, false),
        vec![
            ChartTreeNode::new(None, true, "1000 - Assets"),
            ChartTreeNode::new(Some("1000 - Assets"), true, "1100 - Cash In Hand"),
            ChartTreeNode::new(Some("1100 - Cash In Hand"), false, "1110 - Cash"),
            ChartTreeNode::new(None, true, "Expenses"),
            ChartTreeNode::new(Some("Expenses"), false, "Office Rent"),
        ]
    );
    assert!(validate_bank_account(&chart, "Cash"));
    assert!(!validate_bank_account(&chart, "Bank Accounts"));
}

#[test]
fn build_tree_from_json_uses_importer_account_name() {
    let chart = json!({
        "Root Alias": {
            "account_name": "Assets",
            "Child Alias": {
                "account_name": "Cash",
                "account_number": "1110",
            },
            "account_number": "1000",
            "root_type": "Asset",
        },
    });

    assert_eq!(
        build_tree_from_json(&chart, true),
        vec![
            ChartTreeNode::new(None, true, "1000 - Assets"),
            ChartTreeNode::new(Some("1000 - Assets"), false, "1110 - Cash"),
        ]
    );
}

#[test]
fn get_chart_template_returns_standard_templates() {
    assert_eq!(
        get_chart_template("Standard").expect("standard chart")["Application of Funds (Assets)"]
            ["root_type"],
        json!("Asset")
    );
    assert_eq!(
        get_chart_template("Standard with Numbers").expect("numbered chart")
            ["Application of Funds (Assets)"]["account_number"],
        json!("1000")
    );
    assert!(get_chart_template("Unknown").is_none());
}

#[test]
fn create_chart_plan_matches_python_account_import_fields() {
    let chart = json!({
        "Assets": {
            "Cash": {
                "account_number": "1000",
                "account_type": "Cash",
                "account_category": "Cash and Cash Equivalents",
            },
            "root_type": "Asset",
        },
        "Expenses": {
            "Office Rent": {"account_category": "Operating Expenses"},
            "root_type": "Expense",
        },
    });

    assert_eq!(
        create_chart_plan(
            "Acme",
            &chart,
            CreateChartOptions {
                default_currency: Some("USD".to_string()),
                ..Default::default()
            },
        ),
        vec![
            PlannedAccount {
                account_name: "Assets".to_string(),
                company: "Acme".to_string(),
                parent_account: None,
                is_group: 1,
                root_type: "Asset".to_string(),
                report_type: "Balance Sheet".to_string(),
                account_number: None,
                account_type: None,
                account_category: None,
                account_currency: Some("USD".to_string()),
                tax_rate: None,
                ignore_mandatory: true,
                ignore_permissions: true,
            },
            PlannedAccount {
                account_name: "Cash".to_string(),
                company: "Acme".to_string(),
                parent_account: Some("Assets".to_string()),
                is_group: 0,
                root_type: "Asset".to_string(),
                report_type: "Balance Sheet".to_string(),
                account_number: Some("1000".to_string()),
                account_type: Some("Cash".to_string()),
                account_category: Some("Cash and Cash Equivalents".to_string()),
                account_currency: Some("USD".to_string()),
                tax_rate: None,
                ignore_mandatory: false,
                ignore_permissions: true,
            },
            PlannedAccount {
                account_name: "Expenses".to_string(),
                company: "Acme".to_string(),
                parent_account: None,
                is_group: 1,
                root_type: "Expense".to_string(),
                report_type: "Profit and Loss".to_string(),
                account_number: None,
                account_type: None,
                account_category: None,
                account_currency: Some("USD".to_string()),
                tax_rate: None,
                ignore_mandatory: true,
                ignore_permissions: true,
            },
            PlannedAccount {
                account_name: "Office Rent".to_string(),
                company: "Acme".to_string(),
                parent_account: Some("Expenses".to_string()),
                is_group: 0,
                root_type: "Expense".to_string(),
                report_type: "Profit and Loss".to_string(),
                account_number: None,
                account_type: None,
                account_category: Some("Operating Expenses".to_string()),
                account_currency: Some("USD".to_string()),
                tax_rate: None,
                ignore_mandatory: false,
                ignore_permissions: true,
            },
        ]
    );
}

#[test]
fn chart_file_country_and_lookup_helpers_match_python_branches() {
    let files = vec![
        ChartFile::new(
            "verified",
            "uz.json",
            r#"{"name":"Uzbek Chart","disabled":"No","tree":{"Assets":{"root_type":"Asset"}}}"#,
        ),
        ChartFile::new(
            "verified",
            "uz_disabled.json",
            r#"{"name":"Disabled","disabled":"Yes"}"#,
        ),
        ChartFile::new(
            "unverified",
            "uz_alt.json",
            r#"{"name":"Alt Chart","disabled":"No"}"#,
        ),
    ];

    assert_eq!(
        get_chart_from_files("Uzbek Chart", false, &files).expect("chart exists"),
        json!({"Assets": {"root_type": "Asset"}})
    );
    assert!(get_chart_from_files("Alt Chart", false, &files).is_none());
    assert_eq!(
        get_charts_for_country_from_files("Uzbekistan", Some("uz"), false, false, &files),
        vec!["Uzbek Chart".to_string()]
    );
    assert_eq!(
        get_charts_for_country_from_files("Uzbekistan", Some("uz"), true, true, &files),
        vec![
            "Uzbek Chart".to_string(),
            "Disabled".to_string(),
            "Alt Chart".to_string(),
            "Standard".to_string(),
            "Standard with Numbers".to_string(),
        ]
    );
}

#[test]
fn existing_company_rows_build_account_tree_like_python() {
    let rows = vec![
        AccountSourceRow::new("Assets - AC", "Assets", None, true)
            .root_type("Asset")
            .account_number("1000"),
        AccountSourceRow::new("Cash - AC", "Cash", Some("Assets - AC"), false)
            .account_type("Cash")
            .account_currency("USD"),
        AccountSourceRow::new("Empty Group - AC", "Empty Group", Some("Assets - AC"), true)
            .account_number("1990"),
    ];

    assert_eq!(
        build_account_tree_from_rows(&rows),
        json!({
            "Assets": {
                "account_number": "1000",
                "root_type": "Asset",
                "Cash": {
                    "account_type": "Cash",
                    "account_currency": "USD",
                },
                "Empty Group": {
                    "is_group": 1,
                    "account_number": "1990",
                },
            },
        })
    );
}
