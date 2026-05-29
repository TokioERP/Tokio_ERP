use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountType {
    Receivable,
    Payable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsReceivableSummaryArgs {
    pub account_type: AccountType,
    pub party_naming_by: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsReceivableSummaryFilters {
    pub company: String,
    pub report_date: String,
    pub range: String,
    pub show_gl_balance: bool,
    pub show_future_payments: bool,
    pub show_sales_person: bool,
    pub sales_partner: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivableSummaryColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReceivableSummarySourceRow {
    pub party: String,
    pub party_type: String,
    pub currency: String,
    pub invoiced: f64,
    pub paid: f64,
    pub credit_note: f64,
    pub outstanding: f64,
    pub total_due: f64,
    pub future_amount: f64,
    pub range0: f64,
    pub ranges: Vec<f64>,
    pub territory: Option<String>,
    pub customer_group: Option<String>,
    pub supplier_group: Option<String>,
    pub sales_person: Option<String>,
    pub default_sales_partner: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReceivableSummaryContext {
    pub receivables: Vec<ReceivableSummarySourceRow>,
    pub advances: BTreeMap<String, f64>,
    pub gl_balances: BTreeMap<String, f64>,
    pub party_names: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReceivableSummaryRow {
    pub party_type: String,
    pub party: String,
    pub party_name: Option<String>,
    pub advance: f64,
    pub invoiced: f64,
    pub paid: f64,
    pub credit_note: f64,
    pub outstanding: f64,
    pub gl_balance: Option<f64>,
    pub diff: Option<f64>,
    pub range0: Option<f64>,
    pub ranges: Vec<f64>,
    pub total_due: f64,
    pub future_amount: f64,
    pub remaining_balance: Option<f64>,
    pub territory: Option<String>,
    pub customer_group: Option<String>,
    pub supplier_group: Option<String>,
    pub sales_person: Vec<String>,
    pub default_sales_partner: Option<String>,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountsReceivableSummaryReport {
    pub columns: Vec<ReceivableSummaryColumn>,
    pub rows: Vec<ReceivableSummaryRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub party: String,
    pub company: String,
    pub posting_date: String,
    pub is_cancelled: bool,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct PartyTotal {
    party_type: String,
    currency: String,
    invoiced: f64,
    paid: f64,
    credit_note: f64,
    outstanding: f64,
    total_due: f64,
    future_amount: f64,
    ranges: Vec<f64>,
    territory: Option<String>,
    customer_group: Option<String>,
    supplier_group: Option<String>,
    sales_person: Vec<String>,
    default_sales_partner: Option<String>,
}

impl ReceivableSummaryColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data",
            options: None,
            width,
        }
    }

    pub fn dynamic_link(label: &str, fieldname: &str, options: &'static str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Dynamic Link",
            options: Some(options),
            width,
        }
    }

    pub fn link(label: &str, fieldname: &str, options: &'static str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Link",
            options: Some(options),
            width,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Currency",
            options: Some("currency"),
            width,
        }
    }
}

impl AccountsReceivableSummaryArgs {
    pub fn receivable(party_naming_by: &str) -> Self {
        Self {
            account_type: AccountType::Receivable,
            party_naming_by: party_naming_by.to_string(),
        }
    }
}

impl GlEntry {
    pub fn new(
        party: &str,
        company: &str,
        posting_date: &str,
        is_cancelled: bool,
        debit: f64,
        credit: f64,
    ) -> Self {
        Self {
            party: party.to_string(),
            company: company.to_string(),
            posting_date: posting_date.to_string(),
            is_cancelled,
            debit,
            credit,
        }
    }
}

impl PartyTotal {
    fn from_source(row: &ReceivableSummarySourceRow, range_count: usize) -> Self {
        let mut total = Self {
            party_type: row.party_type.clone(),
            currency: row.currency.clone(),
            invoiced: 0.0,
            paid: 0.0,
            credit_note: 0.0,
            outstanding: 0.0,
            total_due: 0.0,
            future_amount: 0.0,
            ranges: vec![0.0; range_count],
            territory: None,
            customer_group: None,
            supplier_group: None,
            sales_person: Vec::new(),
            default_sales_partner: None,
        };
        total.add_source(row, false);
        total
    }

    fn add_source(&mut self, row: &ReceivableSummarySourceRow, include_amounts: bool) {
        self.currency = row.currency.clone();

        if include_amounts {
            self.invoiced += row.invoiced;
            self.paid += row.paid;
            self.credit_note += row.credit_note;
            self.outstanding += row.outstanding;
            self.total_due += row.total_due;
            self.future_amount += row.future_amount;
            for (idx, amount) in row.ranges.iter().enumerate() {
                if let Some(total) = self.ranges.get_mut(idx) {
                    *total += amount;
                }
            }
        } else {
            self.invoiced = row.invoiced;
            self.paid = row.paid;
            self.credit_note = row.credit_note;
            self.outstanding = row.outstanding;
            self.total_due = row.total_due;
            self.future_amount = row.future_amount;
            for (idx, amount) in row.ranges.iter().enumerate() {
                if let Some(total) = self.ranges.get_mut(idx) {
                    *total = *amount;
                }
            }
        }

        if row.territory.is_some() {
            self.territory = row.territory.clone();
        }
        if row.customer_group.is_some() {
            self.customer_group = row.customer_group.clone();
        }
        if row.supplier_group.is_some() {
            self.supplier_group = row.supplier_group.clone();
        }
        if let Some(sales_person) = row.sales_person.as_ref() {
            self.sales_person.push(sales_person.clone());
        }
        if row.default_sales_partner.is_some() {
            self.default_sales_partner = row.default_sales_partner.clone();
        }
    }
}

pub fn execute(
    filters: &AccountsReceivableSummaryFilters,
    args: &AccountsReceivableSummaryArgs,
    context: &ReceivableSummaryContext,
) -> AccountsReceivableSummaryReport {
    AccountsReceivableSummaryReport {
        columns: get_columns(filters, args),
        rows: get_data(filters, args, context),
    }
}

pub fn get_data(
    filters: &AccountsReceivableSummaryFilters,
    args: &AccountsReceivableSummaryArgs,
    context: &ReceivableSummaryContext,
) -> Vec<ReceivableSummaryRow> {
    let range_count = ageing_range_labels(&filters.range).len().saturating_sub(1);
    let mut party_order = Vec::new();
    let mut party_total: BTreeMap<String, PartyTotal> = BTreeMap::new();

    for row in &context.receivables {
        if !party_total.contains_key(&row.party) {
            party_order.push(row.party.clone());
            party_total.insert(row.party.clone(), PartyTotal::from_source(row, range_count));
        } else if let Some(total) = party_total.get_mut(&row.party) {
            total.add_source(row, true);
        }
    }

    let mut rows = Vec::new();
    for party in party_order {
        let Some(total) = party_total.get(&party) else {
            continue;
        };
        if round_to(total.outstanding, 2) == 0.0 {
            continue;
        }

        let advance = context.advances.get(&party).copied().unwrap_or(0.0);
        let gl_balance = filters
            .show_gl_balance
            .then(|| context.gl_balances.get(&party).copied())
            .flatten();
        let diff = filters
            .show_gl_balance
            .then(|| total.outstanding - gl_balance.unwrap_or(0.0));
        let remaining_balance = filters
            .show_future_payments
            .then(|| total.outstanding - total.future_amount);
        let party_name = (args.party_naming_by == "Naming Series")
            .then(|| context.party_names.get(&party).cloned())
            .flatten();

        rows.push(ReceivableSummaryRow {
            party_type: total.party_type.clone(),
            party: party.clone(),
            party_name,
            advance,
            invoiced: total.invoiced,
            paid: total.paid - advance,
            credit_note: total.credit_note,
            outstanding: total.outstanding,
            gl_balance,
            diff,
            range0: None,
            ranges: total.ranges.clone(),
            total_due: total.total_due,
            future_amount: total.future_amount,
            remaining_balance,
            territory: total.territory.clone(),
            customer_group: total.customer_group.clone(),
            supplier_group: total.supplier_group.clone(),
            sales_person: total.sales_person.clone(),
            default_sales_partner: filters
                .sales_partner
                .as_ref()
                .and(total.default_sales_partner.clone()),
            currency: total.currency.clone(),
        });
    }

    rows
}

pub fn get_columns(
    filters: &AccountsReceivableSummaryFilters,
    args: &AccountsReceivableSummaryArgs,
) -> Vec<ReceivableSummaryColumn> {
    let mut columns = vec![
        ReceivableSummaryColumn::data("Party Type", "party_type", 100),
        ReceivableSummaryColumn::dynamic_link("Party", "party", "party_type", 180),
    ];

    if args.party_naming_by == "Naming Series" {
        columns.push(ReceivableSummaryColumn::data(
            match args.account_type {
                AccountType::Payable => "Supplier Name",
                AccountType::Receivable => "Customer Name",
            },
            "party_name",
            120,
        ));
    }

    columns.extend([
        ReceivableSummaryColumn::currency("Advance Amount", "advance", 120),
        ReceivableSummaryColumn::currency("Invoiced Amount", "invoiced", 120),
        ReceivableSummaryColumn::currency("Paid Amount", "paid", 120),
        ReceivableSummaryColumn::currency(
            match args.account_type {
                AccountType::Receivable => "Credit Note",
                AccountType::Payable => "Debit Note",
            },
            "credit_note",
            120,
        ),
        ReceivableSummaryColumn::currency("Outstanding Amount", "outstanding", 120),
    ]);

    if filters.show_gl_balance {
        columns.push(ReceivableSummaryColumn::currency(
            "GL Balance",
            "gl_balance",
            120,
        ));
        columns.push(ReceivableSummaryColumn::currency("Difference", "diff", 120));
    }

    columns.extend(
        ageing_range_labels(&filters.range)
            .into_iter()
            .enumerate()
            .map(|(idx, label)| {
                ReceivableSummaryColumn::currency(&label, &range_fieldname(idx), 120)
            }),
    );
    columns.push(ReceivableSummaryColumn::currency(
        "Total Amount Due",
        "total_due",
        120,
    ));

    if filters.show_future_payments {
        columns.push(ReceivableSummaryColumn::currency(
            "Future Payment Amount",
            "future_amount",
            120,
        ));
        columns.push(ReceivableSummaryColumn::currency(
            "Remaining Balance",
            "remaining_balance",
            120,
        ));
    }

    match args.account_type {
        AccountType::Receivable => {
            columns.push(ReceivableSummaryColumn::link(
                "Territory",
                "territory",
                "Territory",
                120,
            ));
            columns.push(ReceivableSummaryColumn::link(
                "Customer Group",
                "customer_group",
                "Customer Group",
                120,
            ));
            if filters.show_sales_person {
                columns.push(ReceivableSummaryColumn::data(
                    "Sales Person",
                    "sales_person",
                    120,
                ));
            }
            if filters.sales_partner.is_some() {
                columns.push(ReceivableSummaryColumn::data(
                    "Sales Partner",
                    "default_sales_partner",
                    120,
                ));
            }
        }
        AccountType::Payable => {
            columns.push(ReceivableSummaryColumn::link(
                "Supplier Group",
                "supplier_group",
                "Supplier Group",
                120,
            ));
        }
    }

    columns.push(ReceivableSummaryColumn::link(
        "Currency", "currency", "Currency", 80,
    ));

    columns
}

pub fn get_gl_balance(
    report_date: &str,
    company: &str,
    account_type: AccountType,
    entries: &[GlEntry],
) -> BTreeMap<String, f64> {
    let mut balances = BTreeMap::new();

    for entry in entries {
        if entry.company != company
            || entry.is_cancelled
            || entry.posting_date.as_str() > report_date
        {
            continue;
        }

        let amount = match account_type {
            AccountType::Receivable => entry.debit - entry.credit,
            AccountType::Payable => entry.credit - entry.debit,
        };
        *balances.entry(entry.party.clone()).or_insert(0.0) += amount;
    }

    balances
}

fn ageing_range_labels(range: &str) -> Vec<String> {
    let ranges: Vec<&str> = range
        .split(',')
        .map(str::trim)
        .filter(|part| part.chars().all(|ch| ch.is_ascii_digit()) && !part.is_empty())
        .collect();
    let ranges = if ranges.is_empty() {
        vec!["30", "60", "90", "120"]
    } else {
        ranges
    };
    let mut labels = vec!["<0".to_string()];
    let mut previous = 0;

    for current in ranges {
        labels.push(format!("{previous}-{current}"));
        previous = current.parse::<i32>().unwrap_or_default() + 1;
    }

    labels.push(format!("{previous}-Above"));
    labels
}

fn range_fieldname(index: usize) -> String {
    format!("range{index}")
}

fn round_to(value: f64, precision: i32) -> f64 {
    let multiplier = 10_f64.powi(precision);
    (value * multiplier).round() / multiplier
}
