use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::consolidated_financial_statement::consolidated_financial_statement::{
    add_total_row, calculate_values, filter_accounts, get_columns, get_companies,
    get_company_columns, prepare_companywise_opening_balance, prepare_data,
    update_parent_account_names, AccountRow, CompanyNode, ConsolidatedFilters,
    ConsolidatedGlEntry, FiscalYearData, ReportColumn,
};

fn filters() -> ConsolidatedFilters {
    ConsolidatedFilters {
        company: "Parent".to_string(),
        report: "Balance Sheet".to_string(),
        from_fiscal_year: "FY2026".to_string(),
        to_fiscal_year: "FY2026".to_string(),
        filter_based_on: "Fiscal Year".to_string(),
        period_start_date: None,
        period_end_date: None,
        presentation_currency: None,
        accumulated_in_group_company: true,
        show_zero_values: false,
    }
}

fn company(
    name: &str,
    parent_company: Option<&str>,
    lft: i32,
    rgt: i32,
    currency: &str,
) -> CompanyNode {
    CompanyNode {
        name: name.to_string(),
        parent_company: parent_company.map(str::to_string),
        lft,
        rgt,
        default_currency: currency.to_string(),
    }
}

fn account(
    name: &str,
    company: &str,
    parent_account: Option<&str>,
    account_name: &str,
    account_number: Option<&str>,
    root_type: &str,
    is_group: bool,
    lft: i32,
    rgt: i32,
) -> AccountRow {
    AccountRow {
        name: name.to_string(),
        company: company.to_string(),
        parent_account: parent_account.map(str::to_string),
        lft,
        rgt,
        root_type: root_type.to_string(),
        report_type: "Balance Sheet".to_string(),
        account_name: account_name.to_string(),
        account_number: account_number.map(str::to_string),
        is_group,
        account_key: String::new(),
        parent_account_name: None,
        indent: 0,
        company_wise_opening_bal: BTreeMap::new(),
        opening_balance: 0.0,
        values: BTreeMap::new(),
    }
}

fn companies() -> Vec<CompanyNode> {
    vec![
        company("Parent", None, 1, 6, "USD"),
        company("Child A", Some("Parent"), 2, 3, "USD"),
        company("Child B", Some("Parent"), 4, 5, "EUR"),
    ]
}

fn fiscal_years() -> BTreeMap<String, FiscalYearData> {
    BTreeMap::from([(
        "FY2026".to_string(),
        FiscalYearData {
            year_start_date: "2026-01-01".to_string(),
            year_end_date: "2026-12-31".to_string(),
        },
    )])
}

#[test]
fn consolidated_companies_and_columns_follow_company_tree_and_currency_rules() {
    let (company_list, company_map) = get_companies(&filters(), &companies()).unwrap();

    assert_eq!(company_list, vec!["Parent", "Child A", "Child B"]);
    assert_eq!(company_map["Parent"], vec!["Parent", "Child A", "Child B"]);
    assert_eq!(company_map["Child A"], vec!["Child A"]);

    let company_columns = get_company_columns(&company_list, &companies(), &filters()).unwrap();
    assert_eq!(company_columns[0].fieldname, "Parent");
    assert_eq!(company_columns[0].label, "Parent (USD)");
    assert!(company_columns[0].apply_currency_formatter);

    let columns = get_columns(&company_columns);
    assert_eq!(
        columns[0],
        ReportColumn::link("Account", "account", "Account", 300, false)
    );
    assert_eq!(columns[1].fieldname, "currency");
    assert!(columns[1].hidden);
    assert_eq!(columns[2].fieldname, "Parent");
}

#[test]
fn consolidated_account_keys_deduplicate_common_chart_accounts() {
    let mut accounts = vec![
        account(
            "Assets - P",
            "Parent",
            None,
            "Assets",
            None,
            "Asset",
            true,
            1,
            8,
        ),
        account(
            "Cash - P",
            "Parent",
            Some("Assets - P"),
            "Cash",
            Some("1010"),
            "Asset",
            false,
            2,
            3,
        ),
        account(
            "Assets - C",
            "Child A",
            None,
            "Assets",
            None,
            "Asset",
            true,
            1,
            8,
        ),
        account(
            "Cash - C",
            "Child A",
            Some("Assets - C"),
            "Cash",
            Some("1010"),
            "Asset",
            false,
            2,
            3,
        ),
    ];
    update_parent_account_names(&mut accounts);

    assert_eq!(accounts[1].account_key, "1010 - Cash");
    assert_eq!(accounts[1].parent_account_name.as_deref(), Some("Assets"));
    assert_eq!(accounts[3].account_key, "1010 - Cash");

    let (filtered, by_name, parent_children_map) = filter_accounts(accounts);
    assert_eq!(filtered.len(), 2);
    assert!(by_name.contains_key("Assets"));
    assert!(by_name.contains_key("1010 - Cash"));
    assert_eq!(
        parent_children_map[&Some("Assets".to_string())],
        vec!["1010 - Cash"]
    );
}

#[test]
fn consolidated_calculate_values_accumulates_group_company_and_opening_balances() {
    let mut accounts = vec![
        account(
            "Assets - P",
            "Parent",
            None,
            "Assets",
            None,
            "Asset",
            true,
            1,
            8,
        ),
        account(
            "Cash - P",
            "Parent",
            Some("Assets - P"),
            "Cash",
            Some("1010"),
            "Asset",
            false,
            2,
            3,
        ),
    ];
    update_parent_account_names(&mut accounts);
    let (_filtered, mut by_name, _parent_children_map) = filter_accounts(accounts);
    let company_map = BTreeMap::from([(
        "Parent".to_string(),
        vec!["Parent".to_string(), "Child A".to_string()],
    )]);
    let entries = BTreeMap::from([(
        "1010 - Cash".to_string(),
        vec![
            ConsolidatedGlEntry::new("1010 - Cash", "Parent", "2025-12-31", 100.0, 10.0),
            ConsolidatedGlEntry::new("1010 - Cash", "Child A", "2026-02-01", 50.0, 5.0),
        ],
    )]);

    calculate_values(
        &mut by_name,
        &entries,
        &company_map,
        &filters(),
        &fiscal_years()["FY2026"],
    );

    let cash = by_name.get("1010 - Cash").unwrap();
    assert_eq!(cash.values["Parent"], 135.0);
    assert_eq!(cash.company_wise_opening_bal["Parent"], 90.0);
    assert_eq!(cash.opening_balance, 90.0);
}

#[test]
fn consolidated_prepare_data_reverses_credit_rows_and_adds_total_blank() {
    let mut account = account(
        "Payables - P",
        "Parent",
        None,
        "Payables",
        None,
        "Liability",
        false,
        1,
        2,
    );
    account.account_key = "Payables".to_string();
    account.values.insert("Parent".to_string(), -45.0);
    account.opening_balance = -10.0;
    account
        .company_wise_opening_bal
        .insert("Parent".to_string(), -10.0);
    let companies = BTreeMap::from([("Parent".to_string(), vec!["Parent".to_string()])]);

    let mut data = prepare_data(
        &[account],
        Some("2026-01-01"),
        "2026-12-31",
        "Credit",
        &companies,
        "USD",
        &filters(),
    );

    assert_eq!(data[0].values["Parent"], 45.0);
    assert_eq!(data[0].opening_balance, 10.0);
    assert_eq!(data[0].total, 45.0);

    add_total_row(&mut data, "Liability", "Credit", &companies, "USD");
    assert_eq!(
        data[1].account.as_deref(),
        Some("'Total Liability (Credit)'")
    );
    assert_eq!(data[1].values["Parent"], 45.0);
    assert_eq!(data[1].total, 45.0);
    assert!(data[2].is_blank);
}

#[test]
fn consolidated_opening_balance_message_is_companywise() {
    let mut asset = account(
        "Assets - P",
        "Parent",
        None,
        "Assets",
        None,
        "Asset",
        true,
        1,
        2,
    );
    asset.account_key = "Assets".to_string();
    asset.account_name = "Assets".to_string();
    asset
        .company_wise_opening_bal
        .insert("Parent".to_string(), 100.0);
    let mut liability = account(
        "Liability - P",
        "Parent",
        None,
        "Liability",
        None,
        "Liability",
        true,
        3,
        4,
    );
    liability.account_key = "Liability".to_string();
    liability
        .company_wise_opening_bal
        .insert("Parent".to_string(), -40.0);

    let (message, opening) = prepare_companywise_opening_balance(
        &[asset],
        &[liability],
        &[],
        &BTreeMap::from([("Parent".to_string(), vec!["Parent".to_string()])]),
    );

    assert_eq!(
        message.as_deref(),
        Some("Previous Financial Year is not closed")
    );
    assert_eq!(opening["Parent"], 60.0);
}
