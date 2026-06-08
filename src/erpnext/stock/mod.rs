use std::collections::BTreeMap;

pub mod dashboard;
pub mod dashboard_chart_source;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstallDoc {
    Role {
        doctype: &'static str,
        role_name: &'static str,
        name: &'static str,
    },
    ItemGroup {
        doctype: &'static str,
        item_group_name: &'static str,
        parent_item_group: Option<&'static str>,
        is_group: i32,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WarehouseAccountFlags {
    pub global: BTreeMap<String, WarehouseAccount>,
    pub by_company: BTreeMap<String, BTreeMap<String, WarehouseAccount>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarehouseRecord {
    pub name: String,
    pub account: Option<String>,
    pub parent_warehouse: Option<String>,
    pub company: Option<String>,
    pub is_group: bool,
    pub lft: i32,
    pub rgt: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarehouseAccount {
    pub name: String,
    pub account: String,
    pub parent_warehouse: Option<String>,
    pub company: Option<String>,
    pub is_group: bool,
    pub account_currency: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WarehouseAccountContext {
    pub company_default_inventory_accounts: BTreeMap<String, String>,
    pub stock_accounts: BTreeMap<String, String>,
    pub ancestor_accounts: BTreeMap<String, String>,
    pub account_currencies: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarehouseAccountResolution {
    pub account: Option<String>,
    pub ancestor_query: Option<WarehouseAccountQueryPlan>,
    pub rebuild_tree_doctype: Option<String>,
    pub company_default_inventory_account_lookup: Option<String>,
    pub stock_account_lookup: Option<StockAccountLookup>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarehouseAccountQueryPlan {
    pub sql: &'static str,
    pub params: (i32, i32, String),
    pub as_list: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StockAccountLookup {
    pub doctype: &'static str,
    pub filters: BTreeMap<&'static str, String>,
    pub fieldname: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountLookup {
    pub doctype: &'static str,
    pub fields: Vec<&'static str>,
    pub filters: BTreeMap<String, String>,
    pub order_by: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarehouseAccountMapResult {
    pub map: BTreeMap<String, WarehouseAccount>,
    pub flags: WarehouseAccountFlags,
    pub query_plan: Option<AccountLookup>,
    pub rebuild_tree_doctypes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StockError {
    MissingWarehouseAccount { warehouse: String, company: String },
}

impl InstallDoc {
    pub fn role(name: &'static str) -> Self {
        Self::Role {
            doctype: "Role",
            role_name: name,
            name,
        }
    }

    pub fn item_group(
        item_group_name: &'static str,
        parent_item_group: Option<&'static str>,
        is_group: i32,
    ) -> Self {
        Self::ItemGroup {
            doctype: "Item Group",
            item_group_name,
            parent_item_group,
            is_group,
        }
    }
}

impl WarehouseRecord {
    pub fn new(name: impl Into<String>, company: Option<&str>) -> Self {
        Self {
            name: name.into(),
            account: None,
            parent_warehouse: None,
            company: company.map(ToOwned::to_owned),
            is_group: false,
            lft: 0,
            rgt: 0,
        }
    }

    pub fn with_account(mut self, account: impl Into<String>) -> Self {
        self.account = Some(account.into());
        self
    }

    pub fn with_parent(mut self, parent_warehouse: impl Into<String>) -> Self {
        self.parent_warehouse = Some(parent_warehouse.into());
        self
    }

    pub fn with_bounds(mut self, lft: i32, rgt: i32) -> Self {
        self.lft = lft;
        self.rgt = rgt;
        self
    }

    pub fn as_group(mut self) -> Self {
        self.is_group = true;
        self
    }
}

impl WarehouseAccount {
    pub fn new(name: impl Into<String>, account: impl Into<String>, company: Option<&str>) -> Self {
        Self {
            name: name.into(),
            account: account.into(),
            parent_warehouse: None,
            company: company.map(ToOwned::to_owned),
            is_group: false,
            account_currency: None,
        }
    }

    pub fn with_parent(mut self, parent_warehouse: impl Into<String>) -> Self {
        self.parent_warehouse = Some(parent_warehouse.into());
        self
    }

    pub fn with_account_currency(mut self, account_currency: impl Into<String>) -> Self {
        self.account_currency = Some(account_currency.into());
        self
    }
}

impl WarehouseAccountContext {
    pub fn with_company_default_inventory_account(
        mut self,
        company: impl Into<String>,
        account: impl Into<String>,
    ) -> Self {
        self.company_default_inventory_accounts
            .insert(company.into(), account.into());
        self
    }

    pub fn with_stock_account(
        mut self,
        company: impl Into<String>,
        account: impl Into<String>,
    ) -> Self {
        self.stock_accounts.insert(company.into(), account.into());
        self
    }

    pub fn with_ancestor_account(
        mut self,
        warehouse: impl Into<String>,
        account: impl Into<String>,
    ) -> Self {
        self.ancestor_accounts
            .insert(warehouse.into(), account.into());
        self
    }

    pub fn with_account_currency(
        mut self,
        account: impl Into<String>,
        currency: impl Into<String>,
    ) -> Self {
        self.account_currencies
            .insert(account.into(), currency.into());
        self
    }
}

pub fn install_docs() -> Vec<InstallDoc> {
    vec![
        InstallDoc::role("Stock Manager"),
        InstallDoc::role("Item Manager"),
        InstallDoc::role("Stock User"),
        InstallDoc::role("Quality Manager"),
        InstallDoc::item_group("All Item Groups", None, 1),
        InstallDoc::item_group("Default", Some("All Item Groups"), 0),
    ]
}

pub fn get_warehouse_account_map(
    company: Option<&str>,
    mut flags: WarehouseAccountFlags,
    in_test: bool,
    warehouses: Vec<WarehouseRecord>,
    context: &WarehouseAccountContext,
) -> Result<WarehouseAccountMapResult, StockError> {
    let warehouse_account_map_exists = !flags.global.is_empty() || !flags.by_company.is_empty();
    let company_warehouse_account_map = company.and_then(|company| flags.by_company.get(company));
    let company_warehouse_account_map_exists = company_warehouse_account_map
        .map(|map| !map.is_empty())
        .unwrap_or(false);

    if warehouse_account_map_exists && company_warehouse_account_map_exists && !in_test {
        let map = company_warehouse_account_map.cloned().unwrap_or_default();
        return Ok(WarehouseAccountMapResult {
            map,
            flags,
            query_plan: None,
            rebuild_tree_doctypes: Vec::new(),
        });
    }

    let filters = company
        .map(|company| BTreeMap::from([("company".to_string(), company.to_string())]))
        .unwrap_or_default();
    let query_plan = AccountLookup {
        doctype: "Warehouse",
        fields: vec!["name", "account", "parent_warehouse", "company", "is_group"],
        filters,
        order_by: "lft, rgt",
    };
    let mut warehouse_account = BTreeMap::new();
    let mut rebuild_tree_doctypes = Vec::new();

    for warehouse in warehouses
        .into_iter()
        .filter(|warehouse| company.is_none() || warehouse.company.as_deref() == company)
    {
        let mut account = warehouse.account.clone();
        if account.is_none() {
            let resolution = get_warehouse_account(&warehouse, Some(&warehouse_account), context)?;
            account = resolution.account;
            if let Some(doctype) = resolution.rebuild_tree_doctype {
                rebuild_tree_doctypes.push(doctype);
            }
        }

        if let Some(account) = account {
            warehouse_account
                .entry(warehouse.name.clone())
                .or_insert_with(|| WarehouseAccount {
                    name: warehouse.name,
                    account: account.clone(),
                    parent_warehouse: warehouse.parent_warehouse,
                    company: warehouse.company,
                    is_group: warehouse.is_group,
                    account_currency: context.account_currencies.get(&account).cloned(),
                });
        }
    }

    if let Some(company) = company {
        flags
            .by_company
            .insert(company.to_string(), warehouse_account.clone());
    } else {
        flags.global = warehouse_account.clone();
    }

    Ok(WarehouseAccountMapResult {
        map: warehouse_account,
        flags,
        query_plan: Some(query_plan),
        rebuild_tree_doctypes,
    })
}

pub fn get_warehouse_account(
    warehouse: &WarehouseRecord,
    warehouse_account: Option<&BTreeMap<String, WarehouseAccount>>,
    context: &WarehouseAccountContext,
) -> Result<WarehouseAccountResolution, StockError> {
    let mut account = warehouse.account.clone();
    let mut ancestor_query = None;
    let mut rebuild_tree_doctype = None;
    let mut company_default_inventory_account_lookup = None;
    let mut stock_account_lookup = None;

    if account.is_none() && warehouse.parent_warehouse.is_some() {
        if let Some(warehouse_account) = warehouse_account {
            if let Some(parent_account) = warehouse.parent_warehouse.as_ref().and_then(|parent| {
                warehouse_account
                    .get(parent)
                    .map(|warehouse| warehouse.account.clone())
            }) {
                account = Some(parent_account);
            } else {
                rebuild_tree_doctype = Some("Warehouse".to_string());
            }
        } else {
            let company = warehouse.company.clone().unwrap_or_default();
            ancestor_query = Some(WarehouseAccountQueryPlan {
                sql: "select account from `tabWarehouse` where lft <= ? and rgt >= ? and company = ? and account is not null and ifnull(account, '') !='' order by lft desc limit 1",
                params: (warehouse.lft, warehouse.rgt, company),
                as_list: true,
            });
            account = context.ancestor_accounts.get(&warehouse.name).cloned();
        }
    }

    if account.is_none() {
        if let Some(company) = &warehouse.company {
            company_default_inventory_account_lookup = Some(company.clone());
            account = get_company_default_inventory_account(company, context);
        }
    }

    if account.is_none() {
        if let Some(company) = &warehouse.company {
            stock_account_lookup = Some(stock_account_lookup_plan(company));
            account = context.stock_accounts.get(company).cloned();
        }
    }

    if account.is_none() {
        if let Some(company) = &warehouse.company {
            if !warehouse.is_group {
                return Err(StockError::MissingWarehouseAccount {
                    warehouse: warehouse.name.clone(),
                    company: company.clone(),
                });
            }
        }
    }

    Ok(WarehouseAccountResolution {
        account,
        ancestor_query,
        rebuild_tree_doctype,
        company_default_inventory_account_lookup,
        stock_account_lookup,
    })
}

pub fn get_company_default_inventory_account(
    company: &str,
    context: &WarehouseAccountContext,
) -> Option<String> {
    context
        .company_default_inventory_accounts
        .get(company)
        .cloned()
}

fn stock_account_lookup_plan(company: &str) -> StockAccountLookup {
    StockAccountLookup {
        doctype: "Account",
        filters: BTreeMap::from([
            ("account_type", "Stock".to_string()),
            ("is_group", "0".to_string()),
            ("company", company.to_string()),
        ]),
        fieldname: "name",
    }
}

impl std::fmt::Display for StockError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StockError::MissingWarehouseAccount { warehouse, company } => write!(
                formatter,
                "Please set Account in Warehouse {warehouse} or Default Inventory Account in Company {company}"
            ),
        }
    }
}
