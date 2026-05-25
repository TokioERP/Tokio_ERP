#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentLedgerFilters {
    pub company: Option<String>,
    pub account: Vec<String>,
    pub period_start_date: Option<String>,
    pub period_end_date: Option<String>,
    pub voucher_no: Option<String>,
    pub against_voucher_no: Option<String>,
    pub party_type: Option<String>,
    pub party: Vec<String>,
    pub group_party: bool,
    pub include_account_currency: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerEntry {
    pub posting_date: String,
    pub account: String,
    pub party_type: String,
    pub party: String,
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub against_voucher_type: Option<String>,
    pub against_voucher_no: Option<String>,
    pub amount: f64,
    pub account_currency: String,
    pub amount_in_account_currency: Option<f64>,
    pub company: String,
    pub delinked: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerRow {
    pub posting_date: Option<String>,
    pub account: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub against_voucher_type: Option<String>,
    pub against_voucher_no: Option<String>,
    pub amount: Option<f64>,
    pub amount_in_account_currency: Option<f64>,
    pub currency: String,
    pub company: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: &'static str,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<PaymentLedgerRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentLedgerQueryPlan {
    pub source_doctype: &'static str,
    pub selected_fields: Vec<&'static str>,
    pub base_filters: Vec<String>,
    pub optional_filters: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GroupKey {
    left: Option<String>,
    right: Option<String>,
    party: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct VoucherGroup {
    key: GroupKey,
    increase: Vec<PaymentLedgerRow>,
    decrease: Vec<PaymentLedgerRow>,
}

impl Default for GroupKey {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            party: None,
        }
    }
}

impl PaymentLedgerEntry {
    pub fn new(
        posting_date: impl Into<String>,
        account: impl Into<String>,
        party_type: impl Into<String>,
        party: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            posting_date: posting_date.into(),
            account: account.into(),
            party_type: party_type.into(),
            party: party.into(),
            voucher_type: None,
            voucher_no: None,
            against_voucher_type: None,
            against_voucher_no: None,
            amount,
            account_currency: "INR".to_string(),
            amount_in_account_currency: None,
            company: "_Test Company".to_string(),
            delinked: false,
        }
    }
}

impl PaymentLedgerRow {
    pub fn from_entry(
        posting_date: impl Into<String>,
        account: impl Into<String>,
        party_type: impl Into<String>,
        party: impl Into<String>,
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
        against_voucher_type: impl Into<String>,
        against_voucher_no: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
        company: impl Into<String>,
        amount_in_account_currency: Option<f64>,
    ) -> Self {
        Self {
            posting_date: Some(posting_date.into()),
            account: Some(account.into()),
            party_type: Some(party_type.into()),
            party: Some(party.into()),
            voucher_type: Some(voucher_type.into()),
            voucher_no: Some(voucher_no.into()),
            against_voucher_type: Some(against_voucher_type.into()),
            against_voucher_no: Some(against_voucher_no.into()),
            amount: Some(amount),
            amount_in_account_currency,
            currency: currency.into(),
            company: company.into(),
        }
    }

    pub fn balance(
        against_voucher_no: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
        company: impl Into<String>,
        amount_in_account_currency: Option<f64>,
    ) -> Self {
        Self {
            posting_date: None,
            account: None,
            party_type: None,
            party: None,
            voucher_type: None,
            voucher_no: None,
            against_voucher_type: None,
            against_voucher_no: Some(against_voucher_no.into()),
            amount: Some(amount),
            amount_in_account_currency,
            currency: currency.into(),
            company: company.into(),
        }
    }

    pub fn empty(currency: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            posting_date: None,
            account: None,
            party_type: None,
            party: None,
            voucher_type: None,
            voucher_no: None,
            against_voucher_type: None,
            against_voucher_no: None,
            amount: None,
            amount_in_account_currency: None,
            currency: currency.into(),
            company: company.into(),
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
        hidden: bool,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype,
            options,
            width,
            hidden,
        }
    }
}

impl PaymentLedgerQueryPlan {
    pub fn from_filters(filters: &PaymentLedgerFilters) -> Self {
        let mut optional_filters = Vec::new();

        if let Some(company) = &filters.company {
            optional_filters.push(format!("company = {company}"));
        }
        if !filters.account.is_empty() {
            optional_filters.push(format!("account in {}", format_list(&filters.account)));
        }
        if let Some(period_start_date) = &filters.period_start_date {
            optional_filters.push(format!("posting_date >= {period_start_date}"));
        }
        if let Some(period_end_date) = &filters.period_end_date {
            optional_filters.push(format!("posting_date <= {period_end_date}"));
        }
        if let Some(voucher_no) = &filters.voucher_no {
            optional_filters.push(format!("voucher_no = {voucher_no}"));
        }
        if let Some(against_voucher_no) = &filters.against_voucher_no {
            optional_filters.push(format!("against_voucher_no = {against_voucher_no}"));
        }
        if let Some(party_type) = &filters.party_type {
            optional_filters.push(format!("party_type = {party_type}"));
        }
        if !filters.party.is_empty() {
            optional_filters.push(format!("party in {}", format_list(&filters.party)));
        }

        Self {
            source_doctype: "Payment Ledger Entry",
            selected_fields: vec!["*"],
            base_filters: vec!["delinked = 0".to_string()],
            optional_filters,
        }
    }
}

pub fn execute(
    entries: Vec<PaymentLedgerEntry>,
    filters: PaymentLedgerFilters,
) -> PaymentLedgerReport {
    let filtered_entries = apply_filters(entries, &filters);

    PaymentLedgerReport {
        columns: get_columns(&filters),
        rows: build_data(filtered_entries, &filters),
    }
}

pub fn get_columns(filters: &PaymentLedgerFilters) -> Vec<ReportColumn> {
    let mut columns = vec![
        ReportColumn::new("Posting Date", "posting_date", "Date", None, "100", false),
        ReportColumn::new("Account", "account", "data", None, "100", false),
        ReportColumn::new("Party Type", "party_type", "data", None, "100", false),
        ReportColumn::new("Party", "party", "data", None, "100", false),
        ReportColumn::new("Voucher Type", "voucher_type", "data", None, "100", false),
        ReportColumn::new(
            "Voucher No",
            "voucher_no",
            "Dynamic Link",
            Some("voucher_type"),
            "100",
            false,
        ),
        ReportColumn::new(
            "Against Voucher Type",
            "against_voucher_type",
            "data",
            None,
            "100",
            false,
        ),
        ReportColumn::new(
            "Against Voucher No",
            "against_voucher_no",
            "Dynamic Link",
            Some("against_voucher_type"),
            "100",
            false,
        ),
        ReportColumn::new(
            "Amount",
            "amount",
            "Currency",
            Some("Company:company:default_currency"),
            "100",
            false,
        ),
    ];

    if filters.include_account_currency {
        columns.push(ReportColumn::new(
            "Amount in Account Currency",
            "amount_in_account_currency",
            "Currency",
            Some("currency"),
            "100",
            false,
        ));
    }

    columns.push(ReportColumn::new(
        "Currency",
        "currency",
        "Link",
        Some("Currency"),
        "",
        true,
    ));

    columns
}

pub fn apply_filters(
    entries: Vec<PaymentLedgerEntry>,
    filters: &PaymentLedgerFilters,
) -> Vec<PaymentLedgerEntry> {
    entries
        .into_iter()
        .filter(|entry| !entry.delinked)
        .filter(|entry| {
            filters
                .company
                .as_ref()
                .is_none_or(|company| entry.company == *company)
        })
        .filter(|entry| filters.account.is_empty() || filters.account.contains(&entry.account))
        .filter(|entry| {
            filters
                .period_start_date
                .as_ref()
                .is_none_or(|period_start_date| entry.posting_date >= *period_start_date)
        })
        .filter(|entry| {
            filters
                .period_end_date
                .as_ref()
                .is_none_or(|period_end_date| entry.posting_date <= *period_end_date)
        })
        .filter(|entry| {
            filters
                .voucher_no
                .as_ref()
                .is_none_or(|voucher_no| entry.voucher_no.as_ref() == Some(voucher_no))
        })
        .filter(|entry| {
            filters
                .against_voucher_no
                .as_ref()
                .is_none_or(|against_voucher_no| {
                    entry.against_voucher_no.as_ref() == Some(against_voucher_no)
                })
        })
        .filter(|entry| {
            filters
                .party_type
                .as_ref()
                .is_none_or(|party_type| entry.party_type == *party_type)
        })
        .filter(|entry| filters.party.is_empty() || filters.party.contains(&entry.party))
        .collect()
}

fn build_data(
    entries: Vec<PaymentLedgerEntry>,
    filters: &PaymentLedgerFilters,
) -> Vec<PaymentLedgerRow> {
    let groups = init_voucher_dict(entries, filters);
    let mut rows = Vec::new();

    for group in groups {
        let mut voucher_data = group.increase;
        voucher_data.extend(group.decrease);

        if voucher_data.is_empty() {
            continue;
        }

        let amount = voucher_data
            .iter()
            .filter_map(|row| row.amount)
            .sum::<f64>();
        let amount_in_account_currency = if filters.include_account_currency {
            Some(
                voucher_data
                    .iter()
                    .filter_map(|row| row.amount_in_account_currency)
                    .sum::<f64>(),
            )
        } else {
            None
        };
        let currency = voucher_data[0].currency.clone();
        let company = voucher_data[0].company.clone();

        voucher_data.push(PaymentLedgerRow::balance(
            "Outstanding:",
            amount,
            currency.clone(),
            company.clone(),
            amount_in_account_currency,
        ));
        voucher_data.push(PaymentLedgerRow::empty(currency, company));

        rows.extend(voucher_data);
    }

    rows
}

fn init_voucher_dict(
    entries: Vec<PaymentLedgerEntry>,
    filters: &PaymentLedgerFilters,
) -> Vec<VoucherGroup> {
    let mut groups: Vec<VoucherGroup> = Vec::new();

    for entry in entries {
        let key = if filters.group_party {
            GroupKey {
                left: Some(entry.party_type.clone()),
                right: Some(entry.party.clone()),
                party: None,
            }
        } else {
            GroupKey {
                left: entry.against_voucher_type.clone(),
                right: entry.against_voucher_no.clone(),
                party: Some(entry.party.clone()),
            }
        };

        let row = row_from_entry(entry, filters.include_account_currency);
        let group = if let Some(existing) = groups.iter_mut().find(|group| group.key == key) {
            existing
        } else {
            groups.push(VoucherGroup {
                key,
                increase: Vec::new(),
                decrease: Vec::new(),
            });
            groups
                .last_mut()
                .expect("newly pushed voucher group exists")
        };

        if row.amount.unwrap_or_default() > 0.0 {
            group.increase.push(row);
        } else {
            group.decrease.push(row);
        }
    }

    groups
}

fn row_from_entry(entry: PaymentLedgerEntry, include_account_currency: bool) -> PaymentLedgerRow {
    PaymentLedgerRow {
        posting_date: Some(entry.posting_date),
        account: Some(entry.account),
        party_type: Some(entry.party_type),
        party: Some(entry.party),
        voucher_type: entry.voucher_type,
        voucher_no: entry.voucher_no,
        against_voucher_type: entry.against_voucher_type,
        against_voucher_no: entry.against_voucher_no,
        amount: Some(entry.amount),
        amount_in_account_currency: include_account_currency
            .then_some(entry.amount_in_account_currency)
            .flatten(),
        currency: entry.account_currency,
        company: entry.company,
    }
}

fn format_list(values: &[String]) -> String {
    let joined = values
        .iter()
        .map(|value| format!("'{value}'"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{joined}]")
}
