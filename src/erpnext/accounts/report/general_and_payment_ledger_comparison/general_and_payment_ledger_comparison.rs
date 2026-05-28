use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonFilters {
    pub company: String,
    pub account: Vec<String>,
    pub voucher_no: Option<String>,
    pub period_start_date: Option<String>,
    pub period_end_date: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountRecord {
    pub name: String,
    pub company: String,
    pub account_type: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeneralLedgerEntry {
    pub company: String,
    pub account: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub party_type: String,
    pub party: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerComparisonEntry {
    pub company: String,
    pub account: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub party_type: String,
    pub party: String,
    pub posting_date: String,
    pub amount: f64,
    pub delinked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LedgerComparisonRow {
    pub company: String,
    pub account: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub party_type: String,
    pub party: String,
    pub gl_balance: Option<f64>,
    pub pl_balance: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LedgerComparisonReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<LedgerComparisonRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonQueryPlan {
    pub account_doctype: &'static str,
    pub account_filters: Vec<String>,
    pub gl_doctype: &'static str,
    pub gl_base_filters: Vec<String>,
    pub ple_doctype: &'static str,
    pub ple_base_filters: Vec<String>,
    pub optional_filters: Vec<String>,
    pub group_by: Vec<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AccountSide {
    Receivable,
    Payable,
}

#[derive(Clone, Debug, PartialEq)]
struct BalanceTuple {
    company: String,
    account: String,
    voucher_type: String,
    voucher_no: String,
    party_type: String,
    party: String,
    outstanding: f64,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BalanceKey {
    company: String,
    account: String,
    voucher_type: String,
    voucher_no: String,
    party_type: String,
    party: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BalanceSetKey {
    key: BalanceKey,
    outstanding_bits: u64,
}

impl AccountRecord {
    pub fn new(name: &str, company: &str, account_type: &str) -> Self {
        Self {
            name: name.to_string(),
            company: company.to_string(),
            account_type: account_type.to_string(),
        }
    }
}

impl GeneralLedgerEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        company: &str,
        account: &str,
        voucher_type: &str,
        voucher_no: &str,
        party_type: &str,
        party: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
        is_cancelled: bool,
    ) -> Self {
        Self {
            company: company.to_string(),
            account: account.to_string(),
            voucher_type: voucher_type.to_string(),
            voucher_no: voucher_no.to_string(),
            party_type: party_type.to_string(),
            party: party.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
            is_cancelled,
        }
    }
}

impl PaymentLedgerComparisonEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        company: &str,
        account: &str,
        voucher_type: &str,
        voucher_no: &str,
        party_type: &str,
        party: &str,
        posting_date: &str,
        amount: f64,
        delinked: bool,
    ) -> Self {
        Self {
            company: company.to_string(),
            account: account.to_string(),
            voucher_type: voucher_type.to_string(),
            voucher_no: voucher_no.to_string(),
            party_type: party_type.to_string(),
            party: party.to_string(),
            posting_date: posting_date.to_string(),
            amount,
            delinked,
        }
    }
}

impl ReportColumn {
    pub fn new(
        label: &'static str,
        fieldname: &'static str,
        fieldtype: &'static str,
        options: Option<&'static str>,
        width: &'static str,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype,
            options,
            width,
        }
    }
}

impl ComparisonQueryPlan {
    pub fn for_filters(filters: &ComparisonFilters) -> Self {
        let mut account_filters = vec![
            "company = filters.company".to_string(),
            "account_type in ['Receivable', 'Payable']".to_string(),
        ];

        if !filters.account.is_empty() {
            account_filters.push("name in filters.account".to_string());
        }

        let mut optional_filters = Vec::new();
        if filters.voucher_no.is_some() {
            optional_filters.push("voucher_no = filters.voucher_no".to_string());
        }
        if filters.period_start_date.is_some() {
            optional_filters.push("posting_date >= filters.period_start_date".to_string());
        }
        if filters.period_end_date.is_some() {
            optional_filters.push("posting_date <= filters.period_end_date".to_string());
        }
        if filters.party_type.is_some() {
            optional_filters.push("party_type = filters.party_type".to_string());
        }
        if filters.party.is_some() {
            optional_filters.push("party = filters.party".to_string());
        }

        Self {
            account_doctype: "Account",
            account_filters,
            gl_doctype: "GL Entry",
            gl_base_filters: vec![
                "company = filters.company".to_string(),
                "is_cancelled = 0".to_string(),
                "account in account_type.accounts".to_string(),
            ],
            ple_doctype: "Payment Ledger Entry",
            ple_base_filters: vec![
                "company = filters.company".to_string(),
                "delinked = 0".to_string(),
                "account in account_type.accounts".to_string(),
            ],
            optional_filters,
            group_by: vec![
                "company",
                "account",
                "voucher_type",
                "voucher_no",
                "party_type",
                "party",
            ],
        }
    }
}

pub fn execute(
    filters: &ComparisonFilters,
    accounts: &[AccountRecord],
    gle: Vec<GeneralLedgerEntry>,
    ple: Vec<PaymentLedgerComparisonEntry>,
) -> LedgerComparisonReport {
    let account_types = get_accounts(filters, accounts);
    let receivable_gle = get_gle(
        filters,
        &gle,
        &account_types.receivable,
        AccountSide::Receivable,
    );
    let payable_gle = get_gle(filters, &gle, &account_types.payable, AccountSide::Payable);
    let receivable_ple = get_ple(filters, &ple, &account_types.receivable);
    let payable_ple = get_ple(filters, &ple, &account_types.payable);

    LedgerComparisonReport {
        columns: get_columns(),
        rows: compare(
            [receivable_gle, payable_gle].concat(),
            [receivable_ple, payable_ple].concat(),
        ),
    }
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::new("Company", "company", "Link", Some("Company"), "100"),
        ReportColumn::new("Account", "account", "Link", Some("Account"), "100"),
        ReportColumn::new("Voucher Type", "voucher_type", "Data", None, "100"),
        ReportColumn::new(
            "Voucher No",
            "voucher_no",
            "Dynamic Link",
            Some("voucher_type"),
            "100",
        ),
        ReportColumn::new("Party Type", "party_type", "Data", None, "100"),
        ReportColumn::new("Party", "party", "Dynamic Link", Some("party_type"), "100"),
        ReportColumn::new(
            "GL Balance",
            "gl_balance",
            "Currency",
            Some("Company:company:default_currency"),
            "100",
        ),
        ReportColumn::new(
            "Payment Ledger Balance",
            "pl_balance",
            "Currency",
            Some("Company:company:default_currency"),
            "100",
        ),
    ]
}

fn get_gle(
    filters: &ComparisonFilters,
    entries: &[GeneralLedgerEntry],
    accounts: &[String],
    side: AccountSide,
) -> Vec<BalanceTuple> {
    if accounts.is_empty() {
        return Vec::new();
    }

    let mut grouped: BTreeMap<BalanceKey, f64> = BTreeMap::new();

    for entry in entries {
        if entry.is_cancelled
            || !matches_common_filters(
                filters,
                &entry.company,
                &entry.account,
                &entry.voucher_no,
                &entry.posting_date,
                &entry.party_type,
                &entry.party,
                accounts,
            )
        {
            continue;
        }

        let key = BalanceKey::from_parts(
            &entry.company,
            &entry.account,
            &entry.voucher_type,
            &entry.voucher_no,
            &entry.party_type,
            &entry.party,
        );
        let outstanding = match side {
            AccountSide::Receivable => entry.debit - entry.credit,
            AccountSide::Payable => entry.credit - entry.debit,
        };
        *grouped.entry(key).or_insert(0.0) += outstanding;
    }

    grouped
        .into_iter()
        .map(|(key, outstanding)| BalanceTuple::from_key(key, outstanding))
        .collect()
}

fn get_ple(
    filters: &ComparisonFilters,
    entries: &[PaymentLedgerComparisonEntry],
    accounts: &[String],
) -> Vec<BalanceTuple> {
    if accounts.is_empty() {
        return Vec::new();
    }

    let mut grouped: BTreeMap<BalanceKey, f64> = BTreeMap::new();

    for entry in entries {
        if entry.delinked
            || !matches_common_filters(
                filters,
                &entry.company,
                &entry.account,
                &entry.voucher_no,
                &entry.posting_date,
                &entry.party_type,
                &entry.party,
                accounts,
            )
        {
            continue;
        }

        let key = BalanceKey::from_parts(
            &entry.company,
            &entry.account,
            &entry.voucher_type,
            &entry.voucher_no,
            &entry.party_type,
            &entry.party,
        );
        *grouped.entry(key).or_insert(0.0) += entry.amount;
    }

    grouped
        .into_iter()
        .map(|(key, outstanding)| BalanceTuple::from_key(key, outstanding))
        .collect()
}

fn compare(gle: Vec<BalanceTuple>, ple: Vec<BalanceTuple>) -> Vec<LedgerComparisonRow> {
    let gle_set: BTreeSet<BalanceSetKey> = gle.iter().map(BalanceSetKey::from_tuple).collect();
    let ple_set: BTreeSet<BalanceSetKey> = ple.iter().map(BalanceSetKey::from_tuple).collect();
    let gle_by_set_key: BTreeMap<BalanceSetKey, BalanceTuple> = gle
        .into_iter()
        .map(|tuple| (BalanceSetKey::from_tuple(&tuple), tuple))
        .collect();
    let ple_by_set_key: BTreeMap<BalanceSetKey, BalanceTuple> = ple
        .into_iter()
        .map(|tuple| (BalanceSetKey::from_tuple(&tuple), tuple))
        .collect();
    let mut diff: BTreeMap<BalanceKey, (Option<f64>, Option<f64>)> = BTreeMap::new();

    for set_key in gle_set.difference(&ple_set) {
        if let Some(tuple) = gle_by_set_key.get(set_key) {
            diff.insert(tuple.key(), (Some(tuple.outstanding), None));
        }
    }

    for set_key in ple_set.difference(&gle_set) {
        if let Some(tuple) = ple_by_set_key.get(set_key) {
            diff.entry(tuple.key())
                .and_modify(|value| value.1 = Some(tuple.outstanding))
                .or_insert((Some(0.0), Some(tuple.outstanding)));
        }
    }

    diff.into_iter()
        .map(|(key, (gl_balance, pl_balance))| LedgerComparisonRow {
            company: key.company,
            account: key.account,
            voucher_type: key.voucher_type,
            voucher_no: key.voucher_no,
            party_type: key.party_type,
            party: key.party,
            gl_balance,
            pl_balance,
        })
        .collect()
}

fn get_accounts(filters: &ComparisonFilters, accounts: &[AccountRecord]) -> AccountTypes {
    let mut account_types = AccountTypes::default();

    for account in accounts {
        if account.company != filters.company {
            continue;
        }

        if !filters.account.is_empty() && !filters.account.contains(&account.name) {
            continue;
        }

        match account.account_type.as_str() {
            "Receivable" => account_types.receivable.push(account.name.clone()),
            "Payable" => account_types.payable.push(account.name.clone()),
            _ => {}
        }
    }

    account_types
}

fn matches_common_filters(
    filters: &ComparisonFilters,
    company: &str,
    account: &str,
    voucher_no: &str,
    posting_date: &str,
    party_type: &str,
    party: &str,
    accounts: &[String],
) -> bool {
    company == filters.company
        && accounts.iter().any(|candidate| candidate == account)
        && filters
            .voucher_no
            .as_ref()
            .is_none_or(|expected| voucher_no == expected)
        && filters
            .period_start_date
            .as_ref()
            .is_none_or(|expected| posting_date >= expected)
        && filters
            .period_end_date
            .as_ref()
            .is_none_or(|expected| posting_date <= expected)
        && filters
            .party_type
            .as_ref()
            .is_none_or(|expected| party_type == expected)
        && filters
            .party
            .as_ref()
            .is_none_or(|expected| party == expected)
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct AccountTypes {
    receivable: Vec<String>,
    payable: Vec<String>,
}

impl BalanceKey {
    fn from_parts(
        company: &str,
        account: &str,
        voucher_type: &str,
        voucher_no: &str,
        party_type: &str,
        party: &str,
    ) -> Self {
        Self {
            company: company.to_string(),
            account: account.to_string(),
            voucher_type: voucher_type.to_string(),
            voucher_no: voucher_no.to_string(),
            party_type: party_type.to_string(),
            party: party.to_string(),
        }
    }
}

impl BalanceTuple {
    fn from_key(key: BalanceKey, outstanding: f64) -> Self {
        Self {
            company: key.company,
            account: key.account,
            voucher_type: key.voucher_type,
            voucher_no: key.voucher_no,
            party_type: key.party_type,
            party: key.party,
            outstanding,
        }
    }

    fn key(&self) -> BalanceKey {
        BalanceKey::from_parts(
            &self.company,
            &self.account,
            &self.voucher_type,
            &self.voucher_no,
            &self.party_type,
            &self.party,
        )
    }
}

impl BalanceSetKey {
    fn from_tuple(tuple: &BalanceTuple) -> Self {
        Self {
            key: tuple.key(),
            outstanding_bits: tuple.outstanding.to_bits(),
        }
    }
}
