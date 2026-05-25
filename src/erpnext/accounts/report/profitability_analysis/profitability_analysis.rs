const VALUE_FIELDS: [&str; 3] = ["income", "expense", "gross_profit_loss"];

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityFilters {
    pub company: String,
    pub based_on: String,
    pub accounting_dimension: Option<String>,
    pub fiscal_year: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub with_period_closing_entry: bool,
    pub show_zero_values: bool,
    pub company_currency: String,
    pub zero_cutoff: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityInput {
    pub filters: ProfitabilityFilters,
    pub accounts: Vec<ProfitabilityAccount>,
    pub dimensions: Vec<ProfitabilityDimension>,
    pub gl_entries: Vec<ProfitabilityGlEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityReport {
    pub columns: Vec<ProfitabilityColumn>,
    pub rows: Vec<ProfitabilityRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityColumn {
    pub fieldname: &'static str,
    pub label: String,
    pub fieldtype: &'static str,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountDataPlan {
    pub doctype: String,
    pub fields: Vec<String>,
    pub filters: Vec<String>,
    pub order_by: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityAccount {
    pub name: String,
    pub parent_account: Option<String>,
    pub account_name: Option<String>,
    pub lft: i32,
    pub rgt: i32,
    pub indent: usize,
    pub income: f64,
    pub expense: f64,
    pub gross_profit_loss: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityDimension {
    pub document_type: String,
    pub fieldname: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityGlEntry {
    pub based_on: String,
    pub posting_date: String,
    pub root_type: String,
    pub debit: f64,
    pub credit: f64,
    pub is_opening: String,
    pub voucher_type: String,
    pub company: String,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfitabilityRow {
    pub account_name: Option<String>,
    pub account: Option<String>,
    pub parent_account: Option<String>,
    pub indent: Option<usize>,
    pub fiscal_year: Option<String>,
    pub currency: Option<String>,
    pub based_on: Option<String>,
    pub warn_if_negative: bool,
    pub income: Option<f64>,
    pub expense: Option<f64>,
    pub gross_profit_loss: Option<f64>,
    pub has_value: bool,
}

impl Default for ProfitabilityFilters {
    fn default() -> Self {
        Self {
            company: String::new(),
            based_on: "Cost Center".to_string(),
            accounting_dimension: None,
            fiscal_year: None,
            from_date: None,
            to_date: None,
            with_period_closing_entry: false,
            show_zero_values: false,
            company_currency: String::new(),
            zero_cutoff: 0.005,
        }
    }
}

impl Default for ProfitabilityInput {
    fn default() -> Self {
        Self {
            filters: ProfitabilityFilters::default(),
            accounts: Vec::new(),
            dimensions: Vec::new(),
            gl_entries: Vec::new(),
        }
    }
}

impl ProfitabilityColumn {
    pub fn new(
        fieldname: &'static str,
        label: impl Into<String>,
        fieldtype: &'static str,
        options: Option<&str>,
        width: u16,
        hidden: bool,
    ) -> Self {
        Self {
            fieldname,
            label: label.into(),
            fieldtype,
            options: options.map(str::to_string),
            width,
            hidden,
        }
    }
}

impl ProfitabilityAccount {
    pub fn new(
        name: impl Into<String>,
        parent_account: Option<&str>,
        account_name: Option<&str>,
        lft: i32,
        rgt: i32,
    ) -> Self {
        Self {
            name: name.into(),
            parent_account: parent_account.map(str::to_string),
            account_name: account_name.map(str::to_string),
            lft,
            rgt,
            indent: 0,
            income: 0.0,
            expense: 0.0,
            gross_profit_loss: 0.0,
        }
    }
}

impl ProfitabilityDimension {
    pub fn new(document_type: impl Into<String>, fieldname: impl Into<String>) -> Self {
        Self {
            document_type: document_type.into(),
            fieldname: fieldname.into(),
        }
    }
}

impl ProfitabilityGlEntry {
    pub fn new(
        based_on: impl Into<String>,
        posting_date: impl Into<String>,
        root_type: impl Into<String>,
        debit: f64,
        credit: f64,
    ) -> Self {
        Self {
            based_on: based_on.into(),
            posting_date: posting_date.into(),
            root_type: root_type.into(),
            debit,
            credit,
            is_opening: String::new(),
            voucher_type: String::new(),
            company: "_Test Company".to_string(),
            is_cancelled: false,
        }
    }
}

impl ProfitabilityRow {
    #[allow(clippy::too_many_arguments)]
    pub fn account(
        account_name: impl Into<String>,
        account: impl Into<String>,
        parent_account: Option<&str>,
        indent: usize,
        fiscal_year: impl Into<String>,
        currency: impl Into<String>,
        based_on: impl Into<String>,
        income: f64,
        expense: f64,
        gross_profit_loss: f64,
        has_value: bool,
    ) -> Self {
        Self {
            account_name: Some(account_name.into()),
            account: Some(account.into()),
            parent_account: parent_account.map(str::to_string),
            indent: Some(indent),
            fiscal_year: Some(fiscal_year.into()),
            currency: Some(currency.into()),
            based_on: Some(based_on.into()),
            warn_if_negative: false,
            income: Some(income),
            expense: Some(expense),
            gross_profit_loss: Some(gross_profit_loss),
            has_value,
        }
    }

    pub fn total(income: f64, expense: f64, gross_profit_loss: f64) -> Self {
        Self {
            account_name: Some("'Total'".to_string()),
            account: Some("'Total'".to_string()),
            parent_account: None,
            indent: Some(0),
            fiscal_year: None,
            currency: None,
            based_on: None,
            warn_if_negative: true,
            income: Some(income),
            expense: Some(expense),
            gross_profit_loss: Some(gross_profit_loss),
            has_value: true,
        }
    }

    pub fn blank() -> Self {
        Self {
            account_name: None,
            account: None,
            parent_account: None,
            indent: None,
            fiscal_year: None,
            currency: None,
            based_on: None,
            warn_if_negative: false,
            income: None,
            expense: None,
            gross_profit_loss: None,
            has_value: false,
        }
    }
}

pub fn execute(input: ProfitabilityInput) -> Result<ProfitabilityReport, String> {
    if input.filters.based_on == "Accounting Dimension"
        && input.filters.accounting_dimension.is_none()
    {
        return Err("Select Accounting Dimension.".to_string());
    }

    validate_filters(&input.filters)?;

    let based_on = resolved_based_on(&input.filters);
    let rows = get_data(
        input.accounts,
        input.gl_entries,
        &input.dimensions,
        &input.filters,
        &based_on,
    );

    Ok(ProfitabilityReport {
        columns: get_columns(&input.filters),
        rows,
    })
}

pub fn get_columns(filters: &ProfitabilityFilters) -> Vec<ProfitabilityColumn> {
    vec![
        ProfitabilityColumn::new(
            "account",
            filters.based_on.clone(),
            "Link",
            Some(&filters.based_on),
            300,
            false,
        ),
        ProfitabilityColumn::new("currency", "Currency", "Link", Some("Currency"), 0, true),
        ProfitabilityColumn::new("income", "Income", "Currency", Some("currency"), 305, false),
        ProfitabilityColumn::new(
            "expense",
            "Expense",
            "Currency",
            Some("currency"),
            305,
            false,
        ),
        ProfitabilityColumn::new(
            "gross_profit_loss",
            "Gross Profit / Loss",
            "Currency",
            Some("currency"),
            307,
            false,
        ),
    ]
}

pub fn get_accounts_data_plan(
    based_on: &str,
    company: &str,
    doctype_has_company: bool,
) -> AccountDataPlan {
    if based_on == "Cost Center" {
        AccountDataPlan {
            doctype: "Cost Center".to_string(),
            fields: vec![
                "name".to_string(),
                "parent_cost_center as parent_account".to_string(),
                "cost_center_name as account_name".to_string(),
                "lft".to_string(),
                "rgt".to_string(),
            ],
            filters: vec![format!("company = {company}")],
            order_by: "name".to_string(),
        }
    } else if based_on == "Project" {
        AccountDataPlan {
            doctype: "Project".to_string(),
            fields: vec!["name".to_string()],
            filters: vec![format!("company = {company}")],
            order_by: "name".to_string(),
        }
    } else {
        AccountDataPlan {
            doctype: based_on.to_string(),
            fields: vec!["name".to_string()],
            filters: doctype_has_company
                .then(|| format!("company = {company}"))
                .into_iter()
                .collect(),
            order_by: "name".to_string(),
        }
    }
}

pub fn get_gl_entries_query_plan(filters: &ProfitabilityFilters, fieldname: &str) -> Vec<String> {
    let mut conditions = vec![
        format!("company = {}", filters.company),
        format!("{fieldname} is not null"),
        "is_cancelled = 0".to_string(),
    ];

    match (&filters.from_date, &filters.to_date) {
        (Some(from_date), Some(to_date)) => {
            conditions.push(format!("posting_date between {from_date} and {to_date}"));
        }
        (Some(from_date), None) => {
            conditions.push(format!("posting_date >= {from_date}"));
        }
        (None, Some(to_date)) => {
            conditions.push(format!("posting_date <= {to_date}"));
        }
        (None, None) => {}
    }

    if !filters.with_period_closing_entry {
        conditions.push("voucher_type != Period Closing Voucher".to_string());
    }

    conditions
}

fn get_data(
    accounts: Vec<ProfitabilityAccount>,
    gl_entries: Vec<ProfitabilityGlEntry>,
    dimensions: &[ProfitabilityDimension],
    filters: &ProfitabilityFilters,
    based_on: &str,
) -> Vec<ProfitabilityRow> {
    if accounts.is_empty() {
        return Vec::new();
    }

    let fieldname = dimensions
        .iter()
        .find(|dimension| dimension.document_type == based_on)
        .map(|dimension| dimension.fieldname.as_str())
        .unwrap_or_default();

    let gl_entries_by_account = set_gl_entries_by_account(gl_entries, filters, fieldname);
    let mut filtered_accounts = filter_accounts(accounts);
    let total_row = calculate_values(&mut filtered_accounts, &gl_entries_by_account);
    accumulate_values_into_parents(&mut filtered_accounts);

    let rows = prepare_data(filtered_accounts, filters, total_row, based_on);
    filter_out_zero_value_rows(rows, filters.show_zero_values)
}

fn set_gl_entries_by_account(
    gl_entries: Vec<ProfitabilityGlEntry>,
    filters: &ProfitabilityFilters,
    _fieldname: &str,
) -> Vec<(String, Vec<ProfitabilityGlEntry>)> {
    let mut grouped: Vec<(String, Vec<ProfitabilityGlEntry>)> = Vec::new();
    let mut entries = gl_entries
        .into_iter()
        .filter(|entry| entry.company == filters.company)
        .filter(|entry| !entry.based_on.is_empty())
        .filter(|entry| !entry.is_cancelled)
        .filter(|entry| {
            filters
                .from_date
                .as_ref()
                .is_none_or(|from_date| entry.posting_date >= *from_date)
        })
        .filter(|entry| {
            filters
                .to_date
                .as_ref()
                .is_none_or(|to_date| entry.posting_date <= *to_date)
        })
        .filter(|entry| {
            filters.with_period_closing_entry || entry.voucher_type != "Period Closing Voucher"
        })
        .collect::<Vec<_>>();

    entries.sort_by(|left, right| {
        left.based_on
            .cmp(&right.based_on)
            .then(left.posting_date.cmp(&right.posting_date))
    });

    for entry in entries {
        if let Some((_, bucket)) = grouped
            .iter_mut()
            .find(|(account, _)| account == &entry.based_on)
        {
            bucket.push(entry);
        } else {
            grouped.push((entry.based_on.clone(), vec![entry]));
        }
    }

    grouped
}

fn filter_accounts(mut accounts: Vec<ProfitabilityAccount>) -> Vec<ProfitabilityAccount> {
    accounts.sort_by(|left, right| {
        left.lft
            .cmp(&right.lft)
            .then(left.name.cmp(&right.name))
            .then(left.rgt.cmp(&right.rgt))
    });

    let mut filtered = Vec::new();
    add_children(None, 0, &accounts, &mut filtered);
    filtered
}

fn add_children(
    parent: Option<&str>,
    indent: usize,
    accounts: &[ProfitabilityAccount],
    filtered: &mut Vec<ProfitabilityAccount>,
) {
    let mut children = accounts
        .iter()
        .filter(|account| account.parent_account.as_deref() == parent)
        .cloned()
        .collect::<Vec<_>>();
    children.sort_by(|left, right| left.name.cmp(&right.name));

    for mut child in children {
        child.indent = indent;
        let child_name = child.name.clone();
        filtered.push(child);
        add_children(Some(&child_name), indent + 1, accounts, filtered);
    }
}

fn calculate_values(
    accounts: &mut [ProfitabilityAccount],
    gl_entries_by_account: &[(String, Vec<ProfitabilityGlEntry>)],
) -> ProfitabilityRow {
    let mut total_income = 0.0;
    let mut total_expense = 0.0;

    for account in accounts.iter_mut() {
        account.income = 0.0;
        account.expense = 0.0;
        account.gross_profit_loss = 0.0;

        if let Some((_, entries)) = gl_entries_by_account
            .iter()
            .find(|(name, _)| name == &account.name)
        {
            for entry in entries {
                if entry.is_opening != "Yes" {
                    if entry.root_type == "Income" {
                        account.income += entry.credit - entry.debit;
                    }
                    if entry.root_type == "Expense" {
                        account.expense += entry.debit - entry.credit;
                    }
                    account.gross_profit_loss = account.income - account.expense;
                }
            }
        }

        total_income += account.income;
        total_expense += account.expense;
    }

    ProfitabilityRow::total(total_income, total_expense, total_income - total_expense)
}

fn accumulate_values_into_parents(accounts: &mut [ProfitabilityAccount]) {
    for index in (0..accounts.len()).rev() {
        if let Some(parent_account) = accounts[index].parent_account.clone() {
            if let Some(parent_index) = accounts
                .iter()
                .position(|account| account.name == parent_account)
            {
                accounts[parent_index].income += accounts[index].income;
                accounts[parent_index].expense += accounts[index].expense;
                accounts[parent_index].gross_profit_loss += accounts[index].gross_profit_loss;
            }
        }
    }
}

fn prepare_data(
    accounts: Vec<ProfitabilityAccount>,
    filters: &ProfitabilityFilters,
    total_row: ProfitabilityRow,
    based_on: &str,
) -> Vec<ProfitabilityRow> {
    let company_currency = filters.company_currency.clone();
    let mut rows = Vec::new();

    for account in accounts {
        let income = round3(account.income);
        let expense = round3(account.expense);
        let gross_profit_loss = round3(account.gross_profit_loss);
        let has_value = VALUE_FIELDS.iter().any(|field| {
            let value = match *field {
                "income" => income,
                "expense" => expense,
                "gross_profit_loss" => gross_profit_loss,
                _ => 0.0,
            };
            value.abs() >= filters.zero_cutoff
        });

        rows.push(ProfitabilityRow {
            account_name: Some(account.account_name.unwrap_or_else(|| account.name.clone())),
            account: Some(account.name),
            parent_account: account.parent_account,
            indent: Some(account.indent),
            fiscal_year: filters.fiscal_year.clone(),
            currency: Some(company_currency.clone()),
            based_on: Some(based_on.to_string()),
            warn_if_negative: false,
            income: Some(income),
            expense: Some(expense),
            gross_profit_loss: Some(gross_profit_loss),
            has_value,
        });
    }

    rows.push(ProfitabilityRow::blank());
    rows.push(total_row);
    rows
}

fn filter_out_zero_value_rows(
    rows: Vec<ProfitabilityRow>,
    show_zero_values: bool,
) -> Vec<ProfitabilityRow> {
    let mut accounts_to_show: Vec<Option<String>> = Vec::new();

    for row in &rows {
        if show_zero_values || row.has_value {
            push_unique(&mut accounts_to_show, row.account.clone());
            if let Some(parent) = row.parent_account.clone() {
                push_parent_accounts(&rows, &mut accounts_to_show, Some(parent));
            }
        }
    }

    rows.into_iter()
        .filter(|row| accounts_to_show.contains(&row.account))
        .collect()
}

fn push_parent_accounts(
    rows: &[ProfitabilityRow],
    accounts_to_show: &mut Vec<Option<String>>,
    account: Option<String>,
) {
    if let Some(account_name) = account {
        push_unique(accounts_to_show, Some(account_name.clone()));
        if let Some(parent) = rows
            .iter()
            .find(|row| row.account.as_deref() == Some(account_name.as_str()))
            .and_then(|row| row.parent_account.clone())
        {
            push_parent_accounts(rows, accounts_to_show, Some(parent));
        }
    }
}

fn push_unique(values: &mut Vec<Option<String>>, value: Option<String>) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn validate_filters(filters: &ProfitabilityFilters) -> Result<(), String> {
    if let (Some(from_date), Some(to_date)) = (&filters.from_date, &filters.to_date) {
        if from_date > to_date {
            return Err("From Date cannot be greater than To Date".to_string());
        }
    }
    Ok(())
}

fn resolved_based_on(filters: &ProfitabilityFilters) -> String {
    if filters.based_on == "Accounting Dimension" {
        filters.accounting_dimension.clone().unwrap_or_default()
    } else {
        filters.based_on.clone()
    }
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}
