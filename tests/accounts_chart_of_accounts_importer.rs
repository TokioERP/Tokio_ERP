use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::chart_of_accounts_importer::chart_of_accounts_importer::{
    build_forest, generate_data_from_csv_rows, get_file_extension, get_mandatory_account_types,
    get_mandatory_group_accounts, get_report_type, get_root_types, get_template_rows,
    set_default_accounts_plan, unset_existing_data_plan, validate_accounts, validate_columns,
    validate_company, AccountTemplateRow, ChartAccountNode, ChartOfAccountsImporter,
    ChartOfAccountsImporterError, CompanyAccountDefaultsPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

#[test]
fn chart_of_accounts_importer_matches_erpnext_metadata() {
    assert_eq!(
        ChartOfAccountsImporter::DOCTYPE,
        "Chart of Accounts Importer"
    );
    assert_eq!(ChartOfAccountsImporter::MODULE, "Accounts");
    assert!(ChartOfAccountsImporter::IS_SINGLE);
    assert!(ChartOfAccountsImporter::QUICK_ENTRY);
    assert!(ChartOfAccountsImporter::READ_ONLY);
    assert_eq!(
        ChartOfAccountsImporter::FIELD_ORDER,
        [
            "company",
            "download_template",
            "import_file",
            "chart_preview",
            "chart_tree",
        ]
    );

    let fields = ChartOfAccountsImporter::fields();
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::button("download_template", "Download Template").depends_on("company")
    ));
    assert!(fields.contains(
        &FieldSpec::attach("import_file", "Attach custom Chart of Accounts file")
            .depends_on("company")
    ));
    assert!(fields.contains(
        &FieldSpec::section_break("chart_preview")
            .label("Chart Preview")
            .collapsible()
    ));
    assert!(fields.contains(&FieldSpec::html("chart_tree", "Chart Tree")));

    let controller = ChartOfAccountsImporter::default();
    assert_eq!(controller.doctype(), "Chart of Accounts Importer");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(controller.custom_hooks(), &["validate"]);
}

#[test]
fn chart_of_accounts_importer_validates_file_company_and_columns() {
    assert_eq!(get_file_extension("/tmp/chart.csv"), Ok("csv".to_string()));
    assert_eq!(
        get_file_extension("/tmp/chart.XLSX"),
        Ok("xlsx".to_string())
    );
    assert_eq!(
        get_file_extension("/tmp/chart.pdf"),
        Err(ChartOfAccountsImporterError::InvalidFileFormat)
    );

    assert_eq!(
        validate_company(Some("Parent Co"), false, false, "Child Co"),
        Err(ChartOfAccountsImporterError::WrongCompany {
            company: "Child Co".to_string(),
        })
    );
    assert_eq!(validate_company(None, false, true, "Main Co"), Ok(false));
    assert_eq!(validate_company(None, false, false, "Main Co"), Ok(true));

    assert_eq!(
        validate_columns(&[]),
        Err(ChartOfAccountsImporterError::NoData)
    );
    assert_eq!(
        validate_columns(&[vec!["only".to_string()]]),
        Err(ChartOfAccountsImporterError::WrongTemplate)
    );
    assert_eq!(validate_columns(&[vec![String::new(); 8]]), Ok(()));
}

#[test]
fn chart_of_accounts_importer_generates_rows_and_dicts_like_csv_reader() {
    let rows = vec![
        vec![
            "Account Name",
            "Parent Account",
            "Account Number",
            "Parent Account Number",
            "Is Group",
            "Account Type",
            "Root Type",
            "Account Currency",
        ],
        vec!["Cash", "", "1000", "", "1", "Cash", "Asset", "USD"],
    ];

    let data = generate_data_from_csv_rows(&rows, false);
    assert_eq!(
        data.rows,
        vec![vec![
            "Cash", "Cash", "1000", "1000", "1", "Cash", "Asset", "USD"
        ]]
    );

    let dicts = generate_data_from_csv_rows(&rows, true);
    assert_eq!(dicts.dicts[0]["account_name"], "Cash");
    assert_eq!(dicts.dicts[0]["parent_account"], "");
}

#[test]
fn chart_of_accounts_importer_builds_nested_forest_and_reports_template_errors() {
    let forest = build_forest(&[
        vec!["Assets", "Assets", "1000", "1000", "1", "", "Asset", "USD"],
        vec!["Cash", "Assets", "1010", "1000", "0", "Cash", "", "USD"],
    ])
    .expect("valid forest");

    let mut cash = ChartAccountNode::new("Cash");
    cash.account_number = Some("1010".to_string());
    cash.account_type = Some("Cash".to_string());
    cash.account_currency = Some("USD".to_string());
    let mut assets = ChartAccountNode::new("Assets");
    assets.account_number = Some("1000".to_string());
    assets.is_group = Some("1".to_string());
    assets.root_type = Some("Asset".to_string());
    assets.account_currency = Some("USD".to_string());
    assets.children.insert("1010 - Cash".to_string(), cash);

    assert_eq!(
        forest,
        BTreeMap::from([("1000 - Assets".to_string(), assets)])
    );

    assert_eq!(
        build_forest(&[vec!["Cash", "Assets", "", "", "0", "Cash", "", "USD"]]),
        Err(ChartOfAccountsImporterError::MissingParentAccount {
            parent_account: "Assets".to_string(),
        })
    );
    assert_eq!(
        build_forest(&[vec!["", "", "", "", "0", "", "Asset", "USD"]]),
        Err(ChartOfAccountsImporterError::MissingAccountName { row: 2 })
    );
}

#[test]
fn chart_of_accounts_importer_validates_accounts_roots_and_templates() {
    let accounts = vec![
        AccountTemplateRow {
            account_name: "Assets".to_string(),
            parent_account: String::new(),
            root_type: Some("Asset".to_string()),
            ..Default::default()
        },
        AccountTemplateRow {
            account_name: "Cash".to_string(),
            parent_account: "Assets".to_string(),
            account_type: Some("Cash".to_string()),
            ..Default::default()
        },
        AccountTemplateRow {
            account_name: "Liabilities".to_string(),
            parent_account: String::new(),
            root_type: Some("Liability".to_string()),
            ..Default::default()
        },
        AccountTemplateRow {
            account_name: "Expenses".to_string(),
            parent_account: String::new(),
            root_type: Some("Expense".to_string()),
            ..Default::default()
        },
        AccountTemplateRow {
            account_name: "Income".to_string(),
            parent_account: String::new(),
            root_type: Some("Income".to_string()),
            ..Default::default()
        },
        AccountTemplateRow {
            account_name: "Equity".to_string(),
            parent_account: String::new(),
            root_type: Some("Equity".to_string()),
            ..Default::default()
        },
    ];

    assert_eq!(validate_accounts(&accounts), Ok((true, 6)));
    assert_eq!(
        get_root_types(),
        ["Asset", "Liability", "Expense", "Income", "Equity"]
    );
    assert_eq!(get_report_type("Asset"), "Balance Sheet");
    assert_eq!(get_report_type("Income"), "Profit and Loss");
    assert_eq!(get_mandatory_group_accounts(), ["Bank", "Cash", "Stock"]);
    assert_eq!(get_mandatory_account_types().len(), 9);

    let missing_roots = vec![AccountTemplateRow {
        account_name: "Assets".to_string(),
        root_type: Some("Asset".to_string()),
        ..Default::default()
    }];
    assert_eq!(
        validate_accounts(&missing_roots),
        Err(ChartOfAccountsImporterError::MissingRootAccounts {
            root_types: vec![
                "Liability".to_string(),
                "Expense".to_string(),
                "Income".to_string(),
                "Equity".to_string(),
            ],
        })
    );

    let blank = get_template_rows("Blank Template", "USD", &[]);
    assert_eq!(
        blank[0],
        vec![
            "Account Name",
            "Parent Account",
            "Account Number",
            "Parent Account Number",
            "Is Group",
            "Account Type",
            "Root Type",
            "Account Currency",
        ]
    );
    assert!(blank.contains(&strings(["", "", "", "", "1", "", "Asset"])));
    assert!(blank.contains(&strings(["", "", "", "", "1", "Bank", "Asset"])));
    assert!(blank.contains(&strings(["", "", "", "", "0", "Receivable", "Asset"])));
}

#[test]
fn chart_of_accounts_importer_builds_unset_and_default_account_plans() {
    assert_eq!(
        unset_existing_data_plan("Wind Power LLC", &["default_receivable_account"]),
        (
            BTreeMap::from([("default_receivable_account".to_string(), String::new())]),
            vec![
                "Account",
                "Party Account",
                "Mode of Payment Account",
                "Tax Withholding Account",
                "Sales Taxes and Charges Template",
                "Purchase Taxes and Charges Template",
            ]
        )
    );

    let plan = set_default_accounts_plan(
        "Wind Power LLC",
        "Uzbekistan",
        &[
            ("Receivable", "Debtors - WP"),
            ("Payable", "Creditors - WP"),
        ],
    );
    assert_eq!(
        plan,
        CompanyAccountDefaultsPlan {
            company: "Wind Power LLC".to_string(),
            country: "Uzbekistan".to_string(),
            default_receivable_account: Some("Debtors - WP".to_string()),
            default_payable_account: Some("Creditors - WP".to_string()),
            default_provisional_account: None,
            install_country_fixtures: true,
            create_default_tax_template: true,
        }
    );
}
