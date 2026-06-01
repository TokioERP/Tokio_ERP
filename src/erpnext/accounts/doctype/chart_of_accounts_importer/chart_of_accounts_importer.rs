use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChartOfAccountsImporter {
    pub company: Option<String>,
    pub import_file: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GeneratedCoaData {
    pub rows: Vec<Vec<String>>,
    pub dicts: Vec<BTreeMap<String, String>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChartAccountNode {
    pub account_name: String,
    pub account_number: Option<String>,
    pub is_group: Option<String>,
    pub account_type: Option<String>,
    pub root_type: Option<String>,
    pub account_currency: Option<String>,
    pub children: BTreeMap<String, ChartAccountNode>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountTemplateRow {
    pub account_name: String,
    pub parent_account: String,
    pub account_number: Option<String>,
    pub parent_account_number: Option<String>,
    pub is_group: Option<String>,
    pub account_type: Option<String>,
    pub root_type: Option<String>,
    pub account_currency: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanyAccountDefaultsPlan {
    pub company: String,
    pub country: String,
    pub default_receivable_account: Option<String>,
    pub default_payable_account: Option<String>,
    pub default_provisional_account: Option<String>,
    pub install_country_fixtures: bool,
    pub create_default_tax_template: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChartOfAccountsImporterError {
    NoData,
    WrongTemplate,
    WrongCompany { company: String },
    InvalidFileFormat,
    MissingParentAccount { parent_account: String },
    MissingAccountName { row: usize },
    ParentAccountColumnMissing,
    MissingRootType { accounts: Vec<String> },
    InvalidRootType { accounts: Vec<String> },
    MissingRootAccounts { root_types: Vec<String> },
}

impl ChartAccountNode {
    pub fn new(account_name: &str) -> Self {
        Self {
            account_name: account_name.to_string(),
            ..Default::default()
        }
    }
}

impl ChartOfAccountsImporter {
    pub const DOCTYPE: &'static str = "Chart of Accounts Importer";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_SINGLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const READ_ONLY: bool = true;
    pub const HIDE_TOOLBAR: bool = true;
    pub const IN_CREATE: bool = true;
    pub const FIELD_ORDER: [&'static str; 5] = [
        "company",
        "download_template",
        "import_file",
        "chart_preview",
        "chart_tree",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view(),
            FieldSpec::button("download_template", "Download Template").depends_on("company"),
            FieldSpec::attach("import_file", "Attach custom Chart of Accounts file")
                .depends_on("company"),
            FieldSpec::section_break("chart_preview")
                .label("Chart Preview")
                .collapsible(),
            FieldSpec::html("chart_tree", "Chart Tree"),
        ]
    }

    pub fn validate<F>(
        &self,
        get_coa: F,
    ) -> Result<Option<GeneratedCoaData>, ChartOfAccountsImporterError>
    where
        F: FnOnce(&str) -> Result<GeneratedCoaData, ChartOfAccountsImporterError>,
    {
        if let Some(import_file) = self.import_file.as_deref() {
            return get_coa(import_file).map(Some);
        }

        Ok(None)
    }
}

impl DocumentController for ChartOfAccountsImporter {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

pub fn validate_columns(data: &[Vec<String>]) -> Result<(), ChartOfAccountsImporterError> {
    if data.is_empty() {
        return Err(ChartOfAccountsImporterError::NoData);
    }

    let no_of_columns = data.iter().map(Vec::len).max().unwrap_or_default();
    if no_of_columns != 8 {
        return Err(ChartOfAccountsImporterError::WrongTemplate);
    }

    Ok(())
}

pub fn validate_company(
    parent_company: Option<&str>,
    allow_account_creation_against_child_company: bool,
    has_gl_entries: bool,
    company: &str,
) -> Result<bool, ChartOfAccountsImporterError> {
    if parent_company.is_some() && !allow_account_creation_against_child_company {
        return Err(ChartOfAccountsImporterError::WrongCompany {
            company: company.to_string(),
        });
    }

    Ok(!has_gl_entries)
}

pub fn get_file_extension(file_name: &str) -> Result<String, ChartOfAccountsImporterError> {
    let extension = file_name
        .rsplit_once('.')
        .map(|(_, extension)| extension.to_ascii_lowercase())
        .unwrap_or_default();
    if matches!(extension.as_str(), "csv" | "xlsx" | "xls") {
        Ok(extension)
    } else {
        Err(ChartOfAccountsImporterError::InvalidFileFormat)
    }
}

pub fn generate_data_from_csv_rows<S: AsRef<str>>(
    csv_rows: &[Vec<S>],
    as_dict: bool,
) -> GeneratedCoaData {
    generate_data_from_rows(csv_rows, as_dict)
}

pub fn generate_data_from_excel_rows<S: AsRef<str>>(
    rows: &[Vec<S>],
    as_dict: bool,
) -> GeneratedCoaData {
    generate_data_from_rows(rows, as_dict)
}

fn generate_data_from_rows<S: AsRef<str>>(rows: &[Vec<S>], as_dict: bool) -> GeneratedCoaData {
    let Some(headers) = rows.first() else {
        return GeneratedCoaData::default();
    };
    let headers = headers
        .iter()
        .map(|header| scrub(header.as_ref()))
        .collect::<Vec<_>>();
    let mut data = GeneratedCoaData::default();

    for row in rows.iter().skip(1) {
        if as_dict {
            data.dicts.push(
                headers
                    .iter()
                    .enumerate()
                    .map(|(index, header)| {
                        (
                            header.clone(),
                            row.get(index)
                                .map(|value| value.as_ref().to_string())
                                .unwrap_or_default(),
                        )
                    })
                    .collect(),
            );
        } else {
            let mut row = row
                .iter()
                .map(|value| value.as_ref().to_string())
                .collect::<Vec<_>>();
            if row.get(1).is_some_and(String::is_empty) && row.len() > 1 {
                row[1] = row.first().cloned().unwrap_or_default();
                if row.len() > 3 {
                    row[3] = row.get(2).cloned().unwrap_or_default();
                }
            }
            data.rows.push(row);
        }
    }

    data
}

pub fn build_forest<S: AsRef<str>>(
    data: &[Vec<S>],
) -> Result<BTreeMap<String, ChartAccountNode>, ChartOfAccountsImporterError> {
    let mut charts_map = BTreeMap::new();
    let mut paths = Vec::new();
    let mut errors = Vec::new();

    for (index, row) in data.iter().enumerate() {
        let line_no = index + 2;
        let parsed = parse_chart_row(row);
        if parsed.account_name.is_empty() {
            errors.push(ChartOfAccountsImporterError::MissingAccountName { row: line_no });
            continue;
        }

        let account_key = account_key(&parsed.account_name, parsed.account_number.as_deref());
        let mut node = ChartAccountNode::new(&parsed.account_name);
        node.account_number = parsed.account_number.clone();
        if parsed.is_group.as_deref() == Some("1") {
            node.is_group = parsed.is_group.clone();
        }
        node.account_type = parsed
            .account_type
            .clone()
            .filter(|value| !value.is_empty());
        node.root_type = parsed.root_type.clone().filter(|value| !value.is_empty());
        node.account_currency = parsed
            .account_currency
            .clone()
            .filter(|value| !value.is_empty());
        charts_map.insert(account_key.clone(), node);

        let path = return_parent(data, &account_key)?;
        paths.push(path.into_iter().rev().collect::<Vec<_>>());
    }

    if let Some(error) = errors.into_iter().next() {
        return Err(error);
    }

    let mut forest = BTreeMap::new();
    for path in paths {
        set_nested(&mut forest, &path, &charts_map);
    }

    Ok(forest)
}

pub fn validate_accounts(
    accounts: &[AccountTemplateRow],
) -> Result<(bool, usize), ChartOfAccountsImporterError> {
    let mut accounts_dict = BTreeMap::new();
    for account in accounts {
        accounts_dict.insert(account.account_name.clone(), account.clone());
        if account.parent_account.is_empty() && account.account_name.is_empty() {
            return Err(ChartOfAccountsImporterError::ParentAccountColumnMissing);
        }
        if !account.parent_account.is_empty() && accounts_dict.contains_key(&account.parent_account)
        {
            if let Some(parent) = accounts_dict.get_mut(&account.parent_account) {
                parent.is_group = Some("1".to_string());
            }
        }
    }

    validate_root(&accounts_dict)?;
    Ok((true, accounts.len()))
}

pub fn validate_root(
    accounts: &BTreeMap<String, AccountTemplateRow>,
) -> Result<(), ChartOfAccountsImporterError> {
    let roots = accounts
        .values()
        .filter(|account| account.parent_account.is_empty())
        .collect::<Vec<_>>();
    let mut missing_root_type = Vec::new();
    let mut invalid_root_type = Vec::new();

    for account in &roots {
        if account.account_name.is_empty() {
            continue;
        }
        match account.root_type.as_deref() {
            None | Some("") => missing_root_type.push(account.account_name.clone()),
            Some(root_type) if !get_root_types().contains(&root_type) => {
                invalid_root_type.push(account.account_name.clone())
            }
            _ => {}
        }
    }

    if !missing_root_type.is_empty() {
        return Err(ChartOfAccountsImporterError::MissingRootType {
            accounts: missing_root_type,
        });
    }
    if !invalid_root_type.is_empty() {
        return Err(ChartOfAccountsImporterError::InvalidRootType {
            accounts: invalid_root_type,
        });
    }

    validate_missing_roots(&roots)
}

pub fn validate_missing_roots(
    roots: &[&AccountTemplateRow],
) -> Result<(), ChartOfAccountsImporterError> {
    let root_types_added = roots
        .iter()
        .filter_map(|account| account.root_type.clone())
        .collect::<BTreeSet<_>>();
    let missing = get_root_types()
        .into_iter()
        .filter(|root_type| !root_types_added.contains(*root_type))
        .map(str::to_string)
        .collect::<Vec<_>>();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(ChartOfAccountsImporterError::MissingRootAccounts {
            root_types: missing,
        })
    }
}

pub fn get_root_types() -> [&'static str; 5] {
    ["Asset", "Liability", "Expense", "Income", "Equity"]
}

pub fn get_report_type(root_type: &str) -> &'static str {
    if matches!(root_type, "Asset" | "Liability" | "Equity") {
        "Balance Sheet"
    } else {
        "Profit and Loss"
    }
}

pub fn get_mandatory_group_accounts() -> [&'static str; 3] {
    ["Bank", "Cash", "Stock"]
}

pub fn get_mandatory_account_types() -> Vec<BTreeMap<&'static str, &'static str>> {
    [
        ("Cost of Goods Sold", "Expense"),
        ("Depreciation", "Expense"),
        ("Fixed Asset", "Asset"),
        ("Payable", "Liability"),
        ("Receivable", "Asset"),
        ("Stock Adjustment", "Expense"),
        ("Bank", "Asset"),
        ("Cash", "Asset"),
        ("Stock", "Asset"),
    ]
    .into_iter()
    .map(|(account_type, root_type)| {
        BTreeMap::from([("account_type", account_type), ("root_type", root_type)])
    })
    .collect()
}

pub fn get_template_rows(
    template_type: &str,
    company_currency: &str,
    sample_rows: &[Vec<&str>],
) -> Vec<Vec<String>> {
    let mut rows = vec![vec![
        "Account Name".to_string(),
        "Parent Account".to_string(),
        "Account Number".to_string(),
        "Parent Account Number".to_string(),
        "Is Group".to_string(),
        "Account Type".to_string(),
        "Root Type".to_string(),
        "Account Currency".to_string(),
    ]];

    if template_type == "Blank Template" {
        for root_type in get_root_types() {
            rows.push(strings(["", "", "", "", "1", "", root_type]));
        }
        for account in get_mandatory_group_accounts() {
            rows.push(strings(["", "", "", "", "1", account, "Asset"]));
        }
        for account_type in get_mandatory_account_types() {
            rows.push(strings([
                "",
                "",
                "",
                "",
                "0",
                account_type["account_type"],
                account_type["root_type"],
            ]));
        }
    } else {
        rows.extend(sample_rows.iter().map(|row| {
            let mut row = row
                .iter()
                .map(|value| (*value).to_string())
                .collect::<Vec<_>>();
            row.push(company_currency.to_string());
            row
        }));
    }

    rows
}

pub fn unset_existing_data_plan(
    _company: &str,
    linked_company_fieldnames: &[&str],
) -> (BTreeMap<String, String>, Vec<&'static str>) {
    (
        linked_company_fieldnames
            .iter()
            .map(|fieldname| ((*fieldname).to_string(), String::new()))
            .collect(),
        vec![
            "Account",
            "Party Account",
            "Mode of Payment Account",
            "Tax Withholding Account",
            "Sales Taxes and Charges Template",
            "Purchase Taxes and Charges Template",
        ],
    )
}

pub fn set_default_accounts_plan(
    company: &str,
    country: &str,
    account_by_type: &[(&str, &str)],
) -> CompanyAccountDefaultsPlan {
    let account_by_type = account_by_type.iter().copied().collect::<BTreeMap<_, _>>();

    CompanyAccountDefaultsPlan {
        company: company.to_string(),
        country: country.to_string(),
        default_receivable_account: account_by_type
            .get("Receivable")
            .map(|account| (*account).to_string()),
        default_payable_account: account_by_type
            .get("Payable")
            .map(|account| (*account).to_string()),
        default_provisional_account: account_by_type
            .get("Service Received But Not Billed")
            .map(|account| (*account).to_string()),
        install_country_fixtures: true,
        create_default_tax_template: true,
    }
}

fn parse_chart_row<S: AsRef<str>>(row: &[S]) -> AccountTemplateRow {
    AccountTemplateRow {
        account_name: cell(row, 0),
        parent_account: cell(row, 1),
        account_number: optional_cell(row, 2).map(|value| value.trim().to_string()),
        parent_account_number: optional_cell(row, 3).map(|value| value.trim().to_string()),
        is_group: optional_cell(row, 4),
        account_type: optional_cell(row, 5),
        root_type: optional_cell(row, 6),
        account_currency: optional_cell(row, 7),
    }
}

fn return_parent<S: AsRef<str>>(
    data: &[Vec<S>],
    child: &str,
) -> Result<Vec<String>, ChartOfAccountsImporterError> {
    for row in data {
        let parsed = parse_chart_row(row);
        let current_account_key =
            account_key(&parsed.account_name, parsed.account_number.as_deref());
        let parent_key = account_key(
            &parsed.parent_account,
            parsed.parent_account_number.as_deref(),
        );

        if parent_key == current_account_key && current_account_key == child {
            return Ok(vec![parent_key]);
        } else if current_account_key == child {
            let parent_account_list = return_parent(data, &parent_key)?;
            if parent_account_list.is_empty() && !parent_key.is_empty() {
                return Err(ChartOfAccountsImporterError::MissingParentAccount {
                    parent_account: parent_key,
                });
            }
            let mut path = vec![child.to_string()];
            path.extend(parent_account_list);
            return Ok(path);
        }
    }

    if child.is_empty() {
        Ok(Vec::new())
    } else {
        Err(ChartOfAccountsImporterError::MissingParentAccount {
            parent_account: child.to_string(),
        })
    }
}

fn set_nested(
    forest: &mut BTreeMap<String, ChartAccountNode>,
    path: &[String],
    charts_map: &BTreeMap<String, ChartAccountNode>,
) {
    let Some((head, tail)) = path.split_first() else {
        return;
    };

    let node = forest
        .entry(head.clone())
        .or_insert_with(|| charts_map.get(head).cloned().unwrap_or_default());
    if tail.is_empty() {
        *node = charts_map
            .get(head)
            .cloned()
            .unwrap_or_else(|| node.clone());
    } else {
        set_nested(&mut node.children, tail, charts_map);
    }
}

fn account_key(account_name: &str, account_number: Option<&str>) -> String {
    account_number
        .filter(|number| !number.trim().is_empty())
        .map(|number| format!("{} - {}", number.trim(), account_name))
        .unwrap_or_else(|| account_name.to_string())
}

fn cell<S: AsRef<str>>(row: &[S], index: usize) -> String {
    row.get(index)
        .map(|value| value.as_ref().to_string())
        .unwrap_or_default()
}

fn optional_cell<S: AsRef<str>>(row: &[S], index: usize) -> Option<String> {
    let value = cell(row, index);
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn scrub(value: &str) -> String {
    let mut scrubbed = String::new();
    let mut previous_underscore = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            scrubbed.push(ch.to_ascii_lowercase());
            previous_underscore = false;
        } else if !previous_underscore {
            scrubbed.push('_');
            previous_underscore = true;
        }
    }
    scrubbed.trim_matches('_').to_string()
}

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}
