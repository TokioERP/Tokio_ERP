use serde_json::{Map, Value};

use crate::erpnext::accounts::doctype::account::chart_of_accounts::verified::{
    standard_chart_of_accounts, standard_chart_of_accounts_with_account_number,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartTreeNode {
    pub parent_account: Option<String>,
    pub expandable: bool,
    pub value: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CreateChartOptions {
    pub custom_chart: bool,
    pub from_coa_importer: bool,
    pub allow_unverified_charts: bool,
    pub default_currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlannedAccount {
    pub account_name: String,
    pub company: String,
    pub parent_account: Option<String>,
    pub is_group: i32,
    pub root_type: String,
    pub report_type: String,
    pub account_number: Option<String>,
    pub account_type: Option<String>,
    pub account_category: Option<String>,
    pub account_currency: Option<String>,
    pub tax_rate: Option<f64>,
    pub ignore_mandatory: bool,
    pub ignore_permissions: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartFile {
    pub folder: String,
    pub file_name: String,
    pub content: String,
}

impl ChartFile {
    pub fn new(folder: &str, file_name: &str, content: &str) -> Self {
        Self {
            folder: folder.to_string(),
            file_name: file_name.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountSourceRow {
    pub name: String,
    pub account_name: String,
    pub parent_account: Option<String>,
    pub account_type: Option<String>,
    pub is_group: bool,
    pub root_type: Option<String>,
    pub tax_rate: Option<String>,
    pub account_number: Option<String>,
    pub account_currency: Option<String>,
}

impl AccountSourceRow {
    pub fn new(
        name: &str,
        account_name: &str,
        parent_account: Option<&str>,
        is_group: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            account_name: account_name.to_string(),
            parent_account: parent_account.map(str::to_string),
            account_type: None,
            is_group,
            root_type: None,
            tax_rate: None,
            account_number: None,
            account_currency: None,
        }
    }

    pub fn account_type(mut self, account_type: &str) -> Self {
        self.account_type = Some(account_type.to_string());
        self
    }

    pub fn root_type(mut self, root_type: &str) -> Self {
        self.root_type = Some(root_type.to_string());
        self
    }

    pub fn account_number(mut self, account_number: &str) -> Self {
        self.account_number = Some(account_number.to_string());
        self
    }

    pub fn account_currency(mut self, account_currency: &str) -> Self {
        self.account_currency = Some(account_currency.to_string());
        self
    }
}

impl ChartTreeNode {
    pub fn new(parent_account: Option<&str>, expandable: bool, value: &str) -> Self {
        Self {
            parent_account: parent_account.map(str::to_string),
            expandable,
            value: value.to_string(),
        }
    }
}

pub fn get_chart_metadata_fields() -> [&'static str; 8] {
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
}

pub fn add_suffix_if_duplicate(
    account_name: &str,
    account_number: Option<&str>,
    accounts: &[String],
) -> (String, String) {
    let account_name_in_db = if let Some(account_number) = account_number.map(str::trim) {
        if account_number.is_empty() {
            unidecode_like(&account_name.trim().to_lowercase())
        } else {
            unidecode_like(&format!(
                "{account_number} - {}",
                account_name.trim().to_lowercase()
            ))
        }
    } else {
        unidecode_like(&account_name.trim().to_lowercase())
    };

    if accounts
        .iter()
        .any(|account| account == &account_name_in_db)
    {
        let count = accounts
            .iter()
            .filter(|account| *account == &account_name_in_db)
            .count();
        (format!("{account_name} {count}"), account_name_in_db)
    } else {
        (account_name.to_string(), account_name_in_db)
    }
}

pub fn identify_is_group(child: &Value) -> i32 {
    if truthy(child.get("is_group")) {
        child
            .get("is_group")
            .and_then(Value::as_i64)
            .map(|value| value as i32)
            .unwrap_or(1)
    } else if child
        .as_object()
        .map(|object| {
            object
                .keys()
                .any(|key| !get_chart_metadata_fields().contains(&key.as_str()))
        })
        .unwrap_or(false)
    {
        1
    } else {
        0
    }
}

pub fn get_chart_template(chart_template: &str) -> Option<Value> {
    match chart_template {
        "Standard" => Some(standard_chart_of_accounts::get()),
        "Standard with Numbers" => Some(standard_chart_of_accounts_with_account_number::get()),
        _ => None,
    }
}

pub fn get_chart_from_files(
    chart_template: &str,
    allow_unverified_charts: bool,
    files: &[ChartFile],
) -> Option<Value> {
    let folders = if allow_unverified_charts {
        ["verified", "unverified"].as_slice()
    } else {
        ["verified"].as_slice()
    };

    for folder in folders {
        for file in files {
            if file.folder != *folder || !file.file_name.ends_with(".json") {
                continue;
            }
            let Ok(content) = serde_json::from_str::<Value>(&file.content) else {
                continue;
            };
            if content.get("name").and_then(Value::as_str) == Some(chart_template) {
                return content.get("tree").cloned();
            }
        }
    }
    None
}

pub fn get_charts_for_country_from_files(
    country: &str,
    country_code: Option<&str>,
    with_standard: bool,
    allow_unverified_charts: bool,
    files: &[ChartFile],
) -> Vec<String> {
    let mut charts = Vec::new();
    if let Some(country_code) = country_code {
        let folders = if allow_unverified_charts {
            ["verified", "unverified"].as_slice()
        } else {
            ["verified"].as_slice()
        };

        for folder in folders {
            for file in files {
                if file.folder != *folder || !file.file_name.ends_with(".json") {
                    continue;
                }
                if !(file.file_name.starts_with(country_code)
                    || file.file_name.starts_with(country))
                {
                    continue;
                }
                if let Ok(content) = serde_json::from_str::<Value>(&file.content) {
                    let disabled = content
                        .get("disabled")
                        .and_then(Value::as_str)
                        .unwrap_or("No");
                    if disabled == "No" || allow_unverified_charts {
                        if let Some(name) = content.get("name").and_then(Value::as_str) {
                            charts.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    if charts.len() != 1 || with_standard {
        charts.push("Standard".to_string());
        charts.push("Standard with Numbers".to_string());
    }
    charts
}

pub fn create_chart_plan(
    company: &str,
    chart: &Value,
    options: CreateChartOptions,
) -> Vec<PlannedAccount> {
    let mut plans = Vec::new();
    let mut accounts = Vec::new();
    import_account_plans(
        chart,
        None,
        None,
        true,
        company,
        &options,
        &mut accounts,
        &mut plans,
    );
    plans
}

pub fn build_account_tree_from_rows(rows: &[AccountSourceRow]) -> Value {
    let mut tree = Map::new();
    build_account_tree(&mut tree, None, rows);
    Value::Object(tree)
}

pub fn validate_bank_account(chart: &Value, bank_account: &str) -> bool {
    let mut accounts = Vec::new();
    collect_account_names(chart, &mut accounts);
    accounts.iter().any(|account| account == bank_account)
}

pub fn build_tree_from_json(chart: &Value, from_coa_importer: bool) -> Vec<ChartTreeNode> {
    let mut accounts = Vec::new();
    import_accounts(chart, None, from_coa_importer, &mut accounts);
    accounts
}

fn import_account_plans(
    children: &Value,
    parent: Option<String>,
    root_type: Option<String>,
    root_account: bool,
    company: &str,
    options: &CreateChartOptions,
    accounts: &mut Vec<String>,
    plans: &mut Vec<PlannedAccount>,
) {
    let Some(object) = children.as_object() else {
        return;
    };

    for (account_name, child) in object {
        let mut root_type = root_type.clone();
        if root_account {
            root_type = child
                .get("root_type")
                .and_then(Value::as_str)
                .map(str::to_string);
        }
        if get_chart_metadata_fields().contains(&account_name.as_str()) {
            continue;
        }

        let account_number = child
            .get("account_number")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        let (account_name, account_name_in_db) =
            add_suffix_if_duplicate(account_name, Some(&account_number), accounts);
        let is_group = identify_is_group(child);
        let root_type = root_type.unwrap_or_default();
        let report_type = if ["Asset", "Liability", "Equity"].contains(&root_type.as_str()) {
            "Balance Sheet"
        } else {
            "Profit and Loss"
        };
        let account_name_for_doc = if options.from_coa_importer {
            child
                .get("account_name")
                .and_then(Value::as_str)
                .unwrap_or(&account_name)
                .to_string()
        } else {
            account_name.clone()
        };

        plans.push(PlannedAccount {
            account_name: account_name_for_doc,
            company: company.to_string(),
            parent_account: parent.clone(),
            is_group,
            root_type: root_type.clone(),
            report_type: report_type.to_string(),
            account_number: string_field(&account_number),
            account_type: value_string(child, "account_type"),
            account_category: value_string(child, "account_category"),
            account_currency: if options.custom_chart {
                value_string(child, "account_currency")
            } else {
                options.default_currency.clone()
            },
            tax_rate: child.get("tax_rate").and_then(Value::as_f64),
            ignore_mandatory: root_account || options.allow_unverified_charts,
            ignore_permissions: true,
        });
        accounts.push(account_name_in_db);
        import_account_plans(
            child,
            Some(account_name),
            Some(root_type),
            false,
            company,
            options,
            accounts,
            plans,
        );
    }
}

fn import_accounts(
    children: &Value,
    parent: Option<String>,
    from_coa_importer: bool,
    accounts: &mut Vec<ChartTreeNode>,
) {
    let Some(object) = children.as_object() else {
        return;
    };

    for (account_name, child) in object {
        if get_chart_metadata_fields().contains(&account_name.as_str()) {
            continue;
        }

        let rendered_name = if from_coa_importer {
            child
                .get("account_name")
                .and_then(Value::as_str)
                .unwrap_or(account_name)
        } else {
            account_name
        };
        let value =
            if let Some(account_number) = child.get("account_number").and_then(Value::as_str) {
                format!("{} - {rendered_name}", account_number.trim())
            } else {
                rendered_name.to_string()
            };
        let expandable = identify_is_group(child) != 0;

        accounts.push(ChartTreeNode {
            parent_account: parent.clone(),
            expandable,
            value: value.clone(),
        });
        import_accounts(child, Some(value), from_coa_importer, accounts);
    }
}

fn build_account_tree(
    tree: &mut Map<String, Value>,
    parent: Option<&AccountSourceRow>,
    rows: &[AccountSourceRow],
) {
    let parent_account = parent.map(|account| account.name.as_str()).unwrap_or("");
    let children = rows
        .iter()
        .filter(|account| account.parent_account.as_deref().unwrap_or("") == parent_account)
        .collect::<Vec<_>>();

    if children.is_empty() && parent.is_some_and(|account| account.is_group) {
        tree.insert("is_group".to_string(), Value::from(1));
        if let Some(account_number) = parent.and_then(|account| account.account_number.clone()) {
            tree.insert("account_number".to_string(), Value::from(account_number));
        }
    }

    for child in children {
        let mut child_tree = Map::new();
        if let Some(account_number) = &child.account_number {
            child_tree.insert(
                "account_number".to_string(),
                Value::from(account_number.clone()),
            );
        }
        if let Some(account_type) = &child.account_type {
            child_tree.insert(
                "account_type".to_string(),
                Value::from(account_type.clone()),
            );
        }
        if let Some(tax_rate) = &child.tax_rate {
            child_tree.insert("tax_rate".to_string(), Value::from(tax_rate.clone()));
        }
        if let Some(account_currency) = &child.account_currency {
            child_tree.insert(
                "account_currency".to_string(),
                Value::from(account_currency.clone()),
            );
        }
        if parent.is_none() {
            if let Some(root_type) = &child.root_type {
                child_tree.insert("root_type".to_string(), Value::from(root_type.clone()));
            }
        }
        build_account_tree(&mut child_tree, Some(child), rows);
        tree.insert(child.account_name.clone(), Value::Object(child_tree));
    }
}

fn collect_account_names(chart: &Value, accounts: &mut Vec<String>) {
    let Some(object) = chart.as_object() else {
        return;
    };

    for (account_name, child) in object {
        if get_chart_metadata_fields().contains(&account_name.as_str()) {
            continue;
        }

        accounts.push(account_name.to_string());
        collect_account_names(child, accounts);
    }
}

fn value_string(child: &Value, key: &str) -> Option<String> {
    child.get(key).and_then(Value::as_str).map(str::to_string)
}

fn string_field(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn truthy(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Bool(value)) => *value,
        Some(Value::Number(value)) => value.as_i64().unwrap_or(0) != 0,
        Some(Value::String(value)) => !value.is_empty(),
        Some(Value::Array(value)) => !value.is_empty(),
        Some(Value::Object(value)) => !value.is_empty(),
        _ => false,
    }
}

fn unidecode_like(value: &str) -> String {
    deunicode::deunicode(value)
}
