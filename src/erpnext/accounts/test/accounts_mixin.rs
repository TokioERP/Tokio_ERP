use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountsMixinState {
    pub customer: Option<String>,
    pub supplier: Option<String>,
    pub item: Option<String>,
    pub company: Option<String>,
    pub company_abbr: Option<String>,
    pub cost_center: Option<String>,
    pub warehouse: Option<String>,
    pub finished_warehouse: Option<String>,
    pub income_account: Option<String>,
    pub expense_account: Option<String>,
    pub debit_to: Option<String>,
    pub cash: Option<String>,
    pub creditors: Option<String>,
    pub retained_earnings: Option<String>,
    pub deferred_revenue: Option<String>,
    pub deferred_expense: Option<String>,
    pub bank: Option<String>,
    pub advance_received: Option<String>,
    pub advance_paid: Option<String>,
    pub debtors_usd: Option<String>,
    pub creditors_usd: Option<String>,
    pub dynamic_attributes: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomerSeed {
    pub customer_name: String,
    pub currency: Option<String>,
    pub default_account: Option<String>,
    pub company: Option<String>,
    pub exists: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupplierSeed {
    pub supplier_name: String,
    pub currency: Option<String>,
    pub exists: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanySeed {
    pub company_name: String,
    pub abbr: String,
    pub exists: bool,
    pub cost_center: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WarehouseSeed {
    pub name: String,
    pub warehouse_name: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExistingAccount {
    pub name: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocumentPlan {
    pub doctype: &'static str,
    pub name: String,
    pub insert: bool,
    pub replace_child_table: bool,
    pub fields: BTreeMap<String, String>,
    pub child_rows: BTreeMap<String, Vec<BTreeMap<String, String>>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemPlan {
    pub item_name: String,
    pub is_stock_item: bool,
    pub warehouse: Option<String>,
    pub company: Option<String>,
    pub valuation_rate: f64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanyDefaults {
    pub warehouse: String,
    pub finished_warehouse: String,
    pub income_account: String,
    pub expense_account: String,
    pub debit_to: String,
    pub cash: String,
    pub creditors: String,
    pub retained_earnings: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountSeed {
    pub account_name: String,
    pub parent_account: String,
    pub account_type: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanyPlan {
    pub create_company: bool,
    pub company_fields: BTreeMap<String, String>,
    pub accounts_to_create: Vec<AccountSeed>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AdvanceLiabilityPlan {
    pub book_advance_payments_in_separate_party_account: bool,
    pub default_advance_received_account: Option<String>,
    pub default_advance_paid_account: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UsdAccountPlan {
    pub create: bool,
    pub name: String,
    pub account_name: String,
    pub parent_account: String,
    pub company: String,
    pub account_currency: String,
    pub account_type: String,
}

impl AccountSeed {
    pub fn new(
        account_name: impl Into<String>,
        parent_account: impl Into<String>,
        account_type: Option<&str>,
    ) -> Self {
        Self {
            account_name: account_name.into(),
            parent_account: parent_account.into(),
            account_type: account_type.map(str::to_string),
        }
    }

    pub fn full_name(&self, abbr: &str) -> String {
        format!("{} - {abbr}", self.account_name)
    }
}

impl AccountsMixinState {
    pub fn create_customer(
        &mut self,
        seed: CustomerSeed,
        _existing_customers: &mut BTreeSet<String>,
    ) -> DocumentPlan {
        let mut plan = DocumentPlan {
            doctype: "Customer",
            name: seed.customer_name.clone(),
            insert: !seed.exists,
            replace_child_table: seed.exists
                && seed.company.is_some()
                && seed.default_account.is_some(),
            ..Default::default()
        };

        if !seed.exists {
            plan.fields
                .insert("customer_name".to_string(), seed.customer_name.clone());
            plan.fields
                .insert("type".to_string(), "Individual".to_string());
            if let Some(currency) = seed.currency {
                plan.fields.insert("default_currency".to_string(), currency);
            }
        }

        if let (Some(company), Some(account)) = (seed.company, seed.default_account) {
            plan.child_rows.insert(
                "accounts".to_string(),
                vec![BTreeMap::from([
                    ("company".to_string(), company),
                    ("account".to_string(), account),
                ])],
            );
        }

        self.customer = Some(seed.customer_name);
        plan
    }

    pub fn create_supplier(&mut self, seed: SupplierSeed) -> DocumentPlan {
        let mut plan = DocumentPlan {
            doctype: "Supplier",
            name: seed.supplier_name.clone(),
            insert: !seed.exists,
            ..Default::default()
        };

        if !seed.exists {
            plan.fields
                .insert("supplier_name".to_string(), seed.supplier_name.clone());
            plan.fields
                .insert("supplier_type".to_string(), "Individual".to_string());
            plan.fields
                .insert("supplier_group".to_string(), "Local".to_string());
            if let Some(currency) = seed.currency {
                plan.fields.insert("default_currency".to_string(), currency);
            }
        }

        self.supplier = Some(seed.supplier_name);
        plan
    }

    pub fn create_item(
        &mut self,
        item_name: &str,
        is_stock: bool,
        warehouse: Option<&str>,
        company: Option<&str>,
        valuation_rate: f64,
    ) -> ItemPlan {
        self.item = Some(item_name.to_string());
        ItemPlan {
            item_name: item_name.to_string(),
            is_stock_item: is_stock,
            warehouse: warehouse.map(str::to_string),
            company: company.map(str::to_string),
            valuation_rate,
        }
    }

    pub fn create_company(
        &mut self,
        seed: CompanySeed,
        existing_accounts: &BTreeSet<String>,
        warehouses: &[WarehouseSeed],
    ) -> CompanyPlan {
        self.company_abbr = Some(seed.abbr.clone());
        self.company = Some(seed.company_name.clone());
        self.cost_center = Some(seed.cost_center.clone());

        let defaults = default_company_accounts(&seed.abbr);
        self.warehouse = Some(defaults.warehouse.clone());
        self.finished_warehouse = Some(defaults.finished_warehouse.clone());
        self.income_account = Some(defaults.income_account.clone());
        self.expense_account = Some(defaults.expense_account.clone());
        self.debit_to = Some(defaults.debit_to.clone());
        self.cash = Some(defaults.cash.clone());
        self.creditors = Some(defaults.creditors.clone());
        self.retained_earnings = Some(defaults.retained_earnings.clone());

        let mut plan = CompanyPlan {
            create_company: !seed.exists,
            ..Default::default()
        };
        if !seed.exists {
            plan.company_fields = BTreeMap::from([
                ("doctype".to_string(), "Company".to_string()),
                ("company_name".to_string(), seed.company_name.clone()),
                ("country".to_string(), "India".to_string()),
                ("default_currency".to_string(), "INR".to_string()),
                (
                    "create_chart_of_accounts_based_on".to_string(),
                    "Standard Template".to_string(),
                ),
                ("chart_of_accounts".to_string(), "Standard".to_string()),
            ]);
        }

        for account in other_account_seeds(&seed.abbr) {
            let account_name = account.full_name(&seed.abbr);
            set_account_attribute(self, &account, account_name.clone());
            if !existing_accounts.contains(&account_name) {
                plan.accounts_to_create.push(account);
            }
        }

        self.identify_default_warehouses(warehouses);
        plan
    }

    pub fn enable_advance_as_liability(&self) -> AdvanceLiabilityPlan {
        AdvanceLiabilityPlan {
            book_advance_payments_in_separate_party_account: true,
            default_advance_received_account: self.advance_received.clone(),
            default_advance_paid_account: self.advance_paid.clone(),
        }
    }

    pub fn disable_advance_as_liability(&self) -> AdvanceLiabilityPlan {
        AdvanceLiabilityPlan {
            book_advance_payments_in_separate_party_account: false,
            default_advance_received_account: None,
            default_advance_paid_account: None,
        }
    }

    pub fn identify_default_warehouses(&mut self, warehouses: &[WarehouseSeed]) {
        for warehouse in warehouses {
            self.dynamic_attributes.insert(
                warehouse_attribute_name(&warehouse.warehouse_name),
                warehouse.name.clone(),
            );
        }
    }

    pub fn create_usd_receivable_account(
        &mut self,
        existing: Option<ExistingAccount>,
    ) -> UsdAccountPlan {
        let plan =
            self.usd_account_plan("Debtors USD", "Accounts Receivable", "Receivable", existing);
        self.debtors_usd = Some(plan.name.clone());
        plan
    }

    pub fn create_usd_payable_account(
        &mut self,
        existing: Option<ExistingAccount>,
    ) -> UsdAccountPlan {
        let plan = self.usd_account_plan("Creditors USD", "Accounts Payable", "Payable", existing);
        self.creditors_usd = Some(plan.name.clone());
        plan
    }

    fn usd_account_plan(
        &self,
        account_name: &str,
        parent_prefix: &str,
        account_type: &str,
        existing: Option<ExistingAccount>,
    ) -> UsdAccountPlan {
        let abbr = self.company_abbr.as_deref().unwrap_or_default();
        let company = self.company.as_deref().unwrap_or_default();
        let name = existing
            .as_ref()
            .map(|account| account.name.clone())
            .unwrap_or_else(|| format!("{account_name} - {abbr}"));

        UsdAccountPlan {
            create: existing.is_none(),
            name,
            account_name: account_name.to_string(),
            parent_account: format!("{parent_prefix} - {abbr}"),
            company: company.to_string(),
            account_currency: "USD".to_string(),
            account_type: account_type.to_string(),
        }
    }
}

pub fn default_company_accounts(abbr: &str) -> CompanyDefaults {
    CompanyDefaults {
        warehouse: format!("Stores - {abbr}"),
        finished_warehouse: format!("Finished Goods - {abbr}"),
        income_account: format!("Sales - {abbr}"),
        expense_account: format!("Cost of Goods Sold - {abbr}"),
        debit_to: format!("Debtors - {abbr}"),
        cash: format!("Cash - {abbr}"),
        creditors: format!("Creditors - {abbr}"),
        retained_earnings: format!("Retained Earnings - {abbr}"),
    }
}

pub fn warehouse_attribute_name(warehouse_name: &str) -> String {
    format!(
        "warehouse_{}",
        warehouse_name.to_ascii_lowercase().trim().replace(' ', "_")
    )
}

pub fn account_attribute_name(account_name: &str) -> String {
    account_name.to_ascii_lowercase().trim().replace(' ', "_")
}

pub fn clear_old_entries_doctypes() -> Vec<&'static str> {
    vec![
        "GL Entry",
        "Payment Ledger Entry",
        "Sales Invoice",
        "Purchase Invoice",
        "Payment Entry",
        "Journal Entry",
        "Sales Order",
        "Exchange Rate Revaluation",
        "Bank Account",
        "Bank Transaction",
    ]
}

fn other_account_seeds(abbr: &str) -> Vec<AccountSeed> {
    vec![
        AccountSeed::new(
            "Deferred Revenue",
            format!("Current Liabilities - {abbr}"),
            None,
        ),
        AccountSeed::new("Deferred Expense", format!("Current Assets - {abbr}"), None),
        AccountSeed::new("HDFC", format!("Bank Accounts - {abbr}"), Some("Bank")),
        AccountSeed::new(
            "Advance Received",
            format!("Current Liabilities - {abbr}"),
            Some("Receivable"),
        ),
        AccountSeed::new(
            "Advance Paid",
            format!("Current Assets - {abbr}"),
            Some("Payable"),
        ),
    ]
}

fn set_account_attribute(state: &mut AccountsMixinState, account: &AccountSeed, value: String) {
    match account_attribute_name(&account.account_name).as_str() {
        "deferred_revenue" => state.deferred_revenue = Some(value),
        "deferred_expense" => state.deferred_expense = Some(value),
        "hdfc" => state.bank = Some(value),
        "advance_received" => state.advance_received = Some(value),
        "advance_paid" => state.advance_paid = Some(value),
        _ => {}
    }
}
