use tokio_erp::erpnext::accounts::doctype::account_category::account_category::{
    account_categories_file, create_account_categories, import_account_categories_from_json,
    update_formula_rows_for_rename, AccountCategory, AccountCategoryData, AccountCategoryInsert,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn account_category_matches_erpnext_metadata() {
    assert_eq!(AccountCategory::DOCTYPE, "Account Category");
    assert_eq!(AccountCategory::MODULE, "Accounts");
    assert_eq!(AccountCategory::AUTONAME, "field:account_category_name");
    assert_eq!(
        AccountCategory::FIELD_ORDER,
        [
            "account_category_name",
            "root_type",
            "column_break_qluu",
            "description"
        ]
    );
    assert!(AccountCategory::ALLOW_RENAME);
    assert!(AccountCategory::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        AccountCategory::SEARCH_FIELDS,
        "account_category_name, root_type"
    );

    assert_eq!(
        AccountCategory::fields(),
        vec![
            FieldSpec::data("account_category_name", "Account Category Name")
                .required()
                .unique()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::select("root_type", "Root Type")
                .options("\nAsset\nLiability\nIncome\nExpense\nEquity")
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::column_break("column_break_qluu"),
            FieldSpec::small_text("description", "Description"),
        ]
    );
}

#[test]
fn account_category_controller_and_links_match_erpnext() {
    let doc = AccountCategory::new("Current Assets", "Asset");

    assert_eq!(doc.doctype(), "Account Category");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), ["after_rename"]);
    assert_eq!(AccountCategory::links(), [("Account", "account_category")]);
}

#[test]
fn create_account_categories_skips_empty_and_existing_names_like_erpnext() {
    let inserts = create_account_categories(
        &["Current Assets".to_string()],
        &[
            AccountCategoryData::new("Current Assets", "Asset", Some("Already exists")),
            AccountCategoryData::new("", "Income", None),
            AccountCategoryData::new("Sales", "Income", Some("Revenue categories")),
            AccountCategoryData::new("Sales", "Income", Some("Duplicate in source")),
            AccountCategoryData::new("Expenses", "Expense", None),
        ],
    );

    assert_eq!(
        inserts,
        vec![
            AccountCategoryInsert {
                doctype: "Account Category",
                name: "Sales".to_string(),
                data: AccountCategoryData::new("Sales", "Income", Some("Revenue categories")),
            },
            AccountCategoryInsert {
                doctype: "Account Category",
                name: "Expenses".to_string(),
                data: AccountCategoryData::new("Expenses", "Expense", None),
            },
        ]
    );
}

#[test]
fn import_account_categories_uses_template_account_categories_json() {
    assert_eq!(
        account_categories_file("/tmp/template"),
        "/tmp/template/account_categories.json"
    );

    let imported = import_account_categories_from_json(
        r#"[
            {"account_category_name":"Assets","root_type":"Asset","description":"Asset categories"},
            {"account_category_name":"Income","root_type":"Income"}
        ]"#,
        &["Assets".to_string()],
    )
    .expect("valid JSON category payload");

    assert_eq!(
        imported,
        vec![AccountCategoryInsert {
            doctype: "Account Category",
            name: "Income".to_string(),
            data: AccountCategoryData::new("Income", "Income", None),
        }]
    );
}

#[test]
fn after_rename_updates_matching_formula_rows_and_ignores_like_operators() {
    let rows = vec![
        (
            "ROW-1".to_string(),
            r#"["account_category","=","Old Category"]"#.to_string(),
        ),
        (
            "ROW-2".to_string(),
            r#"["account_category","in",["Old Category","Other"]]"#.to_string(),
        ),
        (
            "ROW-3".to_string(),
            r#"["account_category","like","%Old Category%"]"#.to_string(),
        ),
        ("ROW-4".to_string(), "not a formula".to_string()),
    ];

    let updated = update_formula_rows_for_rename("Old Category", "New Category", &rows);

    assert_eq!(updated.len(), 2);
    assert_eq!(
        updated.get("ROW-1").map(String::as_str),
        Some(r#"["account_category","=","New Category"]"#)
    );
    assert_eq!(
        updated.get("ROW-2").map(String::as_str),
        Some(r#"["account_category","in",["New Category","Other"]]"#)
    );
}

#[test]
fn after_rename_accepts_python_literal_formulas_like_erpnext_ast_literal_eval() {
    let rows = vec![
        (
            "ROW-1".to_string(),
            r#"['account_category', '=', 'Old Category']"#.to_string(),
        ),
        (
            "ROW-2".to_string(),
            r#"{'or': [['account_category', 'in', ['Old Category', 'Other']], ['account_category', 'not like', '%Old Category%']]}"#
                .to_string(),
        ),
    ];

    let updated = update_formula_rows_for_rename("Old Category", "New Category", &rows);

    assert_eq!(updated.len(), 2);
    assert_eq!(
        updated.get("ROW-1").map(String::as_str),
        Some(r#"["account_category","=","New Category"]"#)
    );
    assert_eq!(
        updated.get("ROW-2").map(String::as_str),
        Some(
            r#"{"or":[["account_category","in",["New Category","Other"]],["account_category","not like","%Old Category%"]]}"#
        )
    );
}

#[test]
fn after_rename_keeps_python_literal_keywords_inside_strings() {
    let rows = vec![(
        "ROW-1".to_string(),
        r#"['account_category', 'in', ['None Category', 'True Category', 'Old Category']]"#
            .to_string(),
    )];

    let updated = update_formula_rows_for_rename("Old Category", "New Category", &rows);

    assert_eq!(
        updated.get("ROW-1").map(String::as_str),
        Some(r#"["account_category","in",["None Category","True Category","New Category"]]"#)
    );
}
