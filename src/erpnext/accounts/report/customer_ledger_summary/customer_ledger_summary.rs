use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PartyType {
    Customer,
    Supplier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyLedgerFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyLedgerArgs {
    pub party_type: PartyType,
    pub party_naming_by: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyDetail {
    pub party: String,
    pub party_name: String,
    pub party_group: String,
    pub territory: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartyLedgerGlEntry {
    pub posting_date: String,
    pub party: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub against_voucher: Option<String>,
    pub debit: f64,
    pub credit: f64,
    pub is_opening: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartyLedgerSummaryContext {
    pub party_details: BTreeMap<String, PartyDetail>,
    pub gl_entries: Vec<PartyLedgerGlEntry>,
    pub return_invoices: BTreeSet<String>,
    pub party_adjustment_accounts: Vec<String>,
    pub party_adjustment_details: BTreeMap<String, BTreeMap<String, f64>>,
    pub company_currency: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyLedgerColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: &'static str,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
    pub is_adjustment: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartyLedgerRow {
    pub party: String,
    pub party_name: Option<String>,
    pub customer_group: Option<String>,
    pub supplier_group: Option<String>,
    pub territory: Option<String>,
    pub opening_balance: f64,
    pub invoiced_amount: f64,
    pub paid_amount: f64,
    pub return_amount: f64,
    pub adjustments: BTreeMap<String, f64>,
    pub closing_balance: f64,
    pub currency: String,
    pub dr_or_cr: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartyLedgerSummaryReport {
    pub columns: Vec<PartyLedgerColumn>,
    pub rows: Vec<PartyLedgerRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PartyLedgerError {
    MissingCompany,
    FromDateAfterToDate,
}

impl PartyType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Customer => "Customer",
            Self::Supplier => "Supplier",
        }
    }

    fn group_fieldname(self) -> &'static str {
        match self {
            Self::Customer => "customer_group",
            Self::Supplier => "supplier_group",
        }
    }

    fn group_label(self) -> &'static str {
        match self {
            Self::Customer => "Customer Group",
            Self::Supplier => "Supplier Group",
        }
    }
}

impl PartyLedgerColumn {
    pub fn link(label: &str, fieldname: &str, options: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Link",
            options: Some(options.to_string()),
            width,
            hidden: false,
            is_adjustment: false,
        }
    }

    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data",
            options: None,
            width,
            hidden: false,
            is_adjustment: false,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Currency",
            options: Some("currency".to_string()),
            width,
            hidden: false,
            is_adjustment: false,
        }
    }

    pub fn adjustment(label: &str, fieldname: &str) -> Self {
        Self {
            is_adjustment: true,
            ..Self::currency(label, fieldname, 120)
        }
    }

    pub fn hidden_currency() -> Self {
        Self {
            label: "Currency".to_string(),
            fieldname: "currency".to_string(),
            fieldtype: "Link",
            options: Some("Currency".to_string()),
            width: 50,
            hidden: true,
            is_adjustment: false,
        }
    }
}

pub fn execute(
    filters: &PartyLedgerFilters,
    args: &PartyLedgerArgs,
    context: &PartyLedgerSummaryContext,
) -> Result<PartyLedgerSummaryReport, PartyLedgerError> {
    validate_filters(filters)?;

    Ok(PartyLedgerSummaryReport {
        columns: get_columns(args, context),
        rows: get_data(filters, args, context),
    })
}

pub fn validate_filters(filters: &PartyLedgerFilters) -> Result<(), PartyLedgerError> {
    if filters.company.is_empty() {
        return Err(PartyLedgerError::MissingCompany);
    }

    if filters.from_date > filters.to_date {
        return Err(PartyLedgerError::FromDateAfterToDate);
    }

    Ok(())
}

pub fn get_columns(
    args: &PartyLedgerArgs,
    context: &PartyLedgerSummaryContext,
) -> Vec<PartyLedgerColumn> {
    let party_type = args.party_type;
    let mut columns = vec![PartyLedgerColumn::link(
        party_type.label(),
        "party",
        party_type.label(),
        200,
    )];

    if args.party_naming_by == "Naming Series" {
        columns.push(PartyLedgerColumn::data(
            &format!("{} Name", party_type.label()),
            "party_name",
            150,
        ));
    }

    match party_type {
        PartyType::Customer => {
            columns.push(PartyLedgerColumn::link(
                party_type.group_label(),
                party_type.group_fieldname(),
                party_type.group_label(),
                120,
            ));
            columns.push(PartyLedgerColumn::link(
                "Territory",
                "territory",
                "Territory",
                120,
            ));
        }
        PartyType::Supplier => {
            columns.push(PartyLedgerColumn::link(
                party_type.group_label(),
                party_type.group_fieldname(),
                party_type.group_label(),
                120,
            ));
        }
    }

    columns.extend([
        PartyLedgerColumn::currency("Opening Balance", "opening_balance", 120),
        PartyLedgerColumn::currency("Invoiced Amount", "invoiced_amount", 120),
        PartyLedgerColumn::currency("Paid Amount", "paid_amount", 120),
        PartyLedgerColumn::currency(
            match party_type {
                PartyType::Customer => "Credit Note",
                PartyType::Supplier => "Debit Note",
            },
            "return_amount",
            120,
        ),
    ]);

    columns.extend(
        context.party_adjustment_accounts.iter().map(|account| {
            PartyLedgerColumn::adjustment(account, &format!("adj_{}", scrub(account)))
        }),
    );

    columns.extend([
        PartyLedgerColumn::currency("Closing Balance", "closing_balance", 120),
        PartyLedgerColumn::hidden_currency(),
        PartyLedgerColumn::data("Dr/Cr", "dr_or_cr", 100),
    ]);

    columns
}

pub fn get_data(
    filters: &PartyLedgerFilters,
    args: &PartyLedgerArgs,
    context: &PartyLedgerSummaryContext,
) -> Vec<PartyLedgerRow> {
    let mut party_order = Vec::new();
    let mut party_data: BTreeMap<String, PartyLedgerRow> = BTreeMap::new();
    let company_currency = if context.company_currency.is_empty() {
        String::new()
    } else {
        context.company_currency.clone()
    };

    for gle in &context.gl_entries {
        let Some(party_details) = context.party_details.get(&gle.party) else {
            continue;
        };
        if !party_data.contains_key(&gle.party) {
            party_order.push(gle.party.clone());
            party_data.insert(
                gle.party.clone(),
                PartyLedgerRow {
                    party: gle.party.clone(),
                    party_name: Some(party_details.party_name.clone()),
                    customer_group: (args.party_type == PartyType::Customer)
                        .then(|| party_details.party_group.clone()),
                    supplier_group: (args.party_type == PartyType::Supplier)
                        .then(|| party_details.party_group.clone()),
                    territory: party_details.territory.clone(),
                    opening_balance: 0.0,
                    invoiced_amount: 0.0,
                    paid_amount: 0.0,
                    return_amount: 0.0,
                    adjustments: BTreeMap::new(),
                    closing_balance: 0.0,
                    currency: company_currency.clone(),
                    dr_or_cr: None,
                },
            );
        }

        let amount = signed_amount(args.party_type, gle.debit, gle.credit);
        let row = party_data.get_mut(&gle.party).expect("party row exists");
        row.closing_balance += amount;

        if gle.posting_date < filters.from_date || gle.is_opening == "Yes" {
            row.opening_balance += amount;
        } else if context.return_invoices.contains(&gle.voucher_no) {
            row.return_amount -= amount;
        } else if gle
            .against_voucher
            .as_ref()
            .is_some_and(|voucher| context.return_invoices.contains(voucher))
        {
            if amount > 0.0 {
                row.paid_amount -= amount;
            } else {
                row.invoiced_amount += amount;
            }
        } else if amount > 0.0 {
            row.invoiced_amount += amount;
        } else {
            row.paid_amount -= amount;
        }
    }

    let mut rows = Vec::new();
    for party in party_order {
        let Some(mut row) = party_data.remove(&party) else {
            continue;
        };

        if row.opening_balance == 0.0
            && row.invoiced_amount == 0.0
            && row.paid_amount == 0.0
            && row.return_amount == 0.0
            && row.closing_balance == 0.0
        {
            continue;
        }

        let adjustments = context
            .party_adjustment_details
            .get(&party)
            .cloned()
            .unwrap_or_default();
        let total_adjustment: f64 = adjustments.values().sum();
        row.paid_amount -= total_adjustment;

        for account in &context.party_adjustment_accounts {
            row.adjustments.insert(
                account.clone(),
                adjustments.get(account).copied().unwrap_or(0.0),
            );
        }

        row.dr_or_cr = match args.party_type {
            PartyType::Customer => {
                if row.closing_balance > 0.0 {
                    Some("Dr".to_string())
                } else if row.closing_balance < 0.0 {
                    Some("Cr".to_string())
                } else {
                    None
                }
            }
            PartyType::Supplier => {
                if row.closing_balance > 0.0 {
                    Some("Cr".to_string())
                } else if row.closing_balance < 0.0 {
                    Some("Dr".to_string())
                } else {
                    None
                }
            }
        };

        rows.push(row);
    }

    rows
}

pub fn scrub(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn signed_amount(party_type: PartyType, debit: f64, credit: f64) -> f64 {
    match party_type {
        PartyType::Customer => debit - credit,
        PartyType::Supplier => credit - debit,
    }
}
