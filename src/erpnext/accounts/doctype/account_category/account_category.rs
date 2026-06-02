use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountCategory {
    pub account_category_name: Option<String>,
    pub description: Option<String>,
    pub root_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountCategoryData {
    pub account_category_name: String,
    pub root_type: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountCategoryInsert {
    pub doctype: &'static str,
    pub name: String,
    pub data: AccountCategoryData,
}

impl AccountCategory {
    pub const DOCTYPE: &'static str = "Account Category";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:account_category_name";
    pub const FIELD_ORDER: [&'static str; 4] = [
        "account_category_name",
        "root_type",
        "column_break_qluu",
        "description",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const SEARCH_FIELDS: &'static str = "account_category_name, root_type";

    pub fn new(account_category_name: impl Into<String>, root_type: impl Into<String>) -> Self {
        Self {
            account_category_name: Some(account_category_name.into()),
            root_type: Some(root_type.into()),
            description: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
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
    }

    pub fn links() -> [(&'static str, &'static str); 1] {
        [("Account", "account_category")]
    }

    pub fn after_rename_plan(old_name: &str, new_name: &str) -> AccountCategoryRenamePlan {
        AccountCategoryRenamePlan {
            query_doctype: "Financial Report Row",
            query_field: "calculation_formula",
            query_like: format!("%{old_name}%"),
            updater_field_name: "account_category",
            value_mapping: [(old_name.to_string(), new_name.to_string())].into(),
            exclude_operators: vec!["like".to_string(), "not like".to_string()],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountCategoryRenamePlan {
    pub query_doctype: &'static str,
    pub query_field: &'static str,
    pub query_like: String,
    pub updater_field_name: &'static str,
    pub value_mapping: BTreeMap<String, String>,
    pub exclude_operators: Vec<String>,
}

impl AccountCategoryData {
    pub fn new(
        account_category_name: impl Into<String>,
        root_type: impl Into<String>,
        description: Option<&str>,
    ) -> Self {
        Self {
            account_category_name: account_category_name.into(),
            root_type: root_type.into(),
            description: description.map(str::to_string),
        }
    }
}

impl DocumentController for AccountCategory {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["after_rename"]
    }
}

pub fn create_account_categories(
    existing_category_names: &[String],
    categories: &[AccountCategoryData],
) -> Vec<AccountCategoryInsert> {
    let mut existing_categories: BTreeSet<String> =
        existing_category_names.iter().cloned().collect();
    let mut new_categories = Vec::new();

    for category_data in categories {
        let category_name = category_data.account_category_name.clone();
        if category_name.is_empty() || existing_categories.contains(&category_name) {
            continue;
        }

        new_categories.push(AccountCategoryInsert {
            doctype: AccountCategory::DOCTYPE,
            name: category_name.clone(),
            data: category_data.clone(),
        });
        existing_categories.insert(category_name);
    }

    new_categories
}

pub fn account_categories_file(template_path: &str) -> String {
    format!(
        "{}/account_categories.json",
        template_path.trim_end_matches('/')
    )
}

pub fn import_account_categories_from_json(
    categories_json: &str,
    existing_category_names: &[String],
) -> Result<Vec<AccountCategoryInsert>, serde_json::Error> {
    let categories = serde_json::from_str::<Vec<AccountCategoryData>>(categories_json)?;
    Ok(create_account_categories(
        existing_category_names,
        &categories,
    ))
}

pub fn update_formula_rows_for_rename(
    old_name: &str,
    new_name: &str,
    rows: &[(String, String)],
) -> BTreeMap<String, String> {
    let mut updated_rows = BTreeMap::new();

    for (row_name, formula) in rows {
        let Some(parsed) = parse_formula_literal(formula) else {
            continue;
        };

        let updated = update_formula_value(parsed.clone(), old_name, new_name);
        if updated != parsed {
            updated_rows.insert(row_name.clone(), compact_json(&updated));
        }
    }

    updated_rows
}

fn parse_formula_literal(formula: &str) -> Option<Value> {
    serde_json::from_str::<Value>(formula)
        .or_else(|_| python_literal_to_json(formula).and_then(|json| serde_json::from_str(&json)))
        .ok()
}

fn python_literal_to_json(input: &str) -> Result<String, serde_json::Error> {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\'' || ch == '"' {
            let quote = ch;
            let mut literal = String::new();

            while let Some(next) = chars.next() {
                if next == '\\' {
                    if let Some(escaped) = chars.next() {
                        literal.push(match escaped {
                            '\'' if quote == '\'' => '\'',
                            '"' if quote == '"' => '"',
                            '\\' => '\\',
                            other => other,
                        });
                    }
                } else if next == quote {
                    break;
                } else {
                    literal.push(next);
                }
            }

            output.push_str(&serde_json::to_string(&literal)?);
        } else if ch.is_ascii_alphabetic() || ch == '_' {
            let mut token = String::from(ch);
            while let Some(next) = chars.peek().copied() {
                if next.is_ascii_alphanumeric() || next == '_' {
                    token.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            output.push_str(match token.as_str() {
                "None" => "null",
                "True" => "true",
                "False" => "false",
                _ => token.as_str(),
            });
        } else {
            output.push(ch);
        }
    }

    Ok(output)
}

fn update_formula_value(value: Value, old_name: &str, new_name: &str) -> Value {
    match value {
        Value::Array(values) if values.len() == 3 => {
            let field = values.first().and_then(Value::as_str).unwrap_or_default();
            let operator = values.get(1).and_then(Value::as_str).unwrap_or_default();

            if field == "account_category"
                && !matches!(operator.to_lowercase().as_str(), "like" | "not like")
            {
                let updated_value = update_formula_operand(values[2].clone(), old_name, new_name);
                Value::Array(vec![values[0].clone(), values[1].clone(), updated_value])
            } else {
                Value::Array(values)
            }
        }
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| {
                    let updated_value = match value {
                        Value::Array(children) => Value::Array(
                            children
                                .into_iter()
                                .map(|child| update_formula_value(child, old_name, new_name))
                                .collect(),
                        ),
                        other => other,
                    };
                    (key, updated_value)
                })
                .collect(),
        ),
        other => other,
    }
}

fn update_formula_operand(value: Value, old_name: &str, new_name: &str) -> Value {
    match value {
        Value::String(value) if value == old_name => Value::String(new_name.to_string()),
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| update_formula_operand(value, old_name, new_name))
                .collect(),
        ),
        other => other,
    }
}

fn compact_json(value: &Value) -> String {
    serde_json::to_string(value).expect("serde_json can serialize parsed values")
}
