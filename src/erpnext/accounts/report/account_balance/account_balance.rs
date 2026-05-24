#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountBalanceFilters {
    pub company: Option<String>,
    pub account_type: Option<String>,
    pub root_type: Option<String>,
    pub report_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldtype: &'static str,
    pub fieldname: &'static str,
    pub options: &'static str,
    pub hidden: bool,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountBalanceAccount {
    pub name: String,
    pub account_currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountBalanceRow {
    pub account: String,
    pub currency: String,
    pub balance: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountBalanceReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<AccountBalanceRow>,
}

impl ReportColumn {
    pub const fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldtype: "Link",
            fieldname,
            options,
            hidden: false,
            width,
        }
    }

    pub const fn currency(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldtype: "Currency",
            fieldname,
            options,
            hidden: false,
            width,
        }
    }

    pub const fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }
}

impl AccountBalanceAccount {
    pub fn new(name: impl Into<String>, account_currency: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            account_currency: account_currency.into(),
        }
    }
}

impl AccountBalanceRow {
    pub fn new(account: impl Into<String>, currency: impl Into<String>, balance: f64) -> Self {
        Self {
            account: account.into(),
            currency: currency.into(),
            balance,
        }
    }
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Account", "account", "Account", 200),
        ReportColumn::link("Currency", "currency", "Currency", 100).hidden(),
        ReportColumn::currency("Balance", "balance", "currency", 100),
    ]
}

pub fn get_conditions(filters: &AccountBalanceFilters) -> Vec<(&'static str, &str)> {
    let mut conditions = Vec::new();
    if let Some(account_type) = filters.account_type.as_deref() {
        conditions.push(("account_type", account_type));
    }
    if let Some(company) = filters.company.as_deref() {
        conditions.push(("company", company));
    }
    if let Some(root_type) = filters.root_type.as_deref() {
        conditions.push(("root_type", root_type));
    }
    conditions
}

pub fn execute<F>(
    filters: AccountBalanceFilters,
    mut accounts: Vec<AccountBalanceAccount>,
    mut balance_on: F,
) -> AccountBalanceReport
where
    F: FnMut(&str, Option<&str>) -> f64,
{
    accounts.sort_by(|left, right| left.name.cmp(&right.name));
    let report_date = filters.report_date.as_deref();
    let rows = accounts
        .into_iter()
        .map(|account| {
            let balance = balance_on(&account.name, report_date);
            AccountBalanceRow::new(account.name, account.account_currency, balance)
        })
        .collect();

    AccountBalanceReport {
        columns: get_columns(),
        rows,
    }
}
