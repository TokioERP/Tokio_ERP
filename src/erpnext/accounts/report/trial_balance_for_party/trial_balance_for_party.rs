use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrialBalanceForPartyFilters {
    pub company: String,
    pub party_type: String,
    pub party: Option<String>,
    pub account_filter: Vec<String>,
    pub from_date: String,
    pub to_date: String,
    pub show_zero_values: bool,
    pub exclude_zero_balance_parties: bool,
    pub customer_naming_by: Option<String>,
    pub supplier_naming_by: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyRecord {
    pub name: String,
    pub party_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub company: String,
    pub account: String,
    pub party_type: String,
    pub party: String,
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
    pub is_opening: String,
    pub is_cancelled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub fieldname: &'static str,
    pub label: String,
    pub fieldtype: &'static str,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialBalancePartyRow {
    pub party: String,
    pub party_name: Option<String>,
    pub opening_debit: f64,
    pub opening_credit: f64,
    pub debit: f64,
    pub credit: f64,
    pub closing_debit: f64,
    pub closing_credit: f64,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialBalanceForPartyReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<TrialBalancePartyRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrialBalanceForPartyQueryPlan {
    pub party_doctype: String,
    pub party_fields: Vec<String>,
    pub party_filters: Vec<String>,
    pub opening_filters: Vec<String>,
    pub period_filters: Vec<String>,
    pub group_by: Vec<&'static str>,
}

impl PartyRecord {
    pub fn new(name: &str, party_name: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            party_name: party_name.map(str::to_string),
        }
    }
}

impl GlEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        company: &str,
        account: &str,
        party_type: &str,
        party: &str,
        posting_date: &str,
        debit: f64,
        credit: f64,
        is_opening: &str,
        is_cancelled: bool,
    ) -> Self {
        Self {
            company: company.to_string(),
            account: account.to_string(),
            party_type: party_type.to_string(),
            party: party.to_string(),
            posting_date: posting_date.to_string(),
            debit,
            credit,
            is_opening: is_opening.to_string(),
            is_cancelled,
        }
    }
}

impl ReportColumn {
    pub fn link(
        label: &str,
        fieldname: &'static str,
        options: &str,
        width: u16,
        hidden: bool,
    ) -> Self {
        Self {
            fieldname,
            label: label.to_string(),
            fieldtype: "Link",
            options: Some(options.to_string()),
            width,
            hidden,
        }
    }

    pub fn data(label: &str, fieldname: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label: label.to_string(),
            fieldtype: "Data",
            options: None,
            width,
            hidden: false,
        }
    }

    pub fn currency(label: &str, fieldname: &'static str, width: u16) -> Self {
        Self {
            fieldname,
            label: label.to_string(),
            fieldtype: "Currency",
            options: Some("currency".to_string()),
            width,
            hidden: false,
        }
    }
}

impl TrialBalanceForPartyQueryPlan {
    pub fn for_filters(filters: &TrialBalanceForPartyFilters) -> Self {
        let party_name_field = party_name_field(filters);
        let mut party_filters = Vec::new();
        if filters.party.is_some() {
            party_filters.push("name = filters.party".to_string());
        }

        let mut opening_filters = vec![
            "company = filters.company".to_string(),
            "is_cancelled = 0".to_string(),
            "party_type = filters.party_type".to_string(),
            "party != ''".to_string(),
            "posting_date < filters.from_date OR (is_opening = 'Yes' AND posting_date <= filters.to_date)".to_string(),
        ];
        let mut period_filters = vec![
            "company = filters.company".to_string(),
            "is_cancelled = 0".to_string(),
            "party_type = filters.party_type".to_string(),
            "party != ''".to_string(),
            "posting_date >= filters.from_date".to_string(),
            "posting_date <= filters.to_date".to_string(),
            "is_opening = 'No'".to_string(),
        ];

        if !filters.account_filter.is_empty() {
            opening_filters.push("account in account_filter".to_string());
            period_filters.push("account in account_filter".to_string());
        }

        Self {
            party_doctype: filters.party_type.clone(),
            party_fields: vec!["name".to_string(), party_name_field],
            party_filters,
            opening_filters,
            period_filters,
            group_by: vec!["party"],
        }
    }
}

pub fn execute(
    filters: &TrialBalanceForPartyFilters,
    parties: Vec<PartyRecord>,
    gl_entries: Vec<GlEntry>,
    company_currency: &str,
) -> TrialBalanceForPartyReport {
    let show_party_name = is_party_name_visible(filters);
    TrialBalanceForPartyReport {
        columns: get_columns(filters, show_party_name),
        rows: get_data(
            filters,
            show_party_name,
            parties,
            gl_entries,
            company_currency,
        ),
    }
}

pub fn get_data(
    filters: &TrialBalanceForPartyFilters,
    show_party_name: bool,
    parties: Vec<PartyRecord>,
    gl_entries: Vec<GlEntry>,
    company_currency: &str,
) -> Vec<TrialBalancePartyRow> {
    let selected_parties = parties
        .into_iter()
        .filter(|party| {
            filters
                .party
                .as_ref()
                .is_none_or(|selected| party.name == *selected)
        })
        .collect::<Vec<_>>();
    let opening_balances = get_opening_balances(filters, &gl_entries);
    let balances_within_period = get_balances_within_period(filters, &gl_entries);
    let mut rows = Vec::new();
    let mut total_row = TrialBalancePartyRow::totals(company_currency);

    for party in selected_parties {
        let (opening_debit, opening_credit) = opening_balances
            .get(&party.name)
            .copied()
            .unwrap_or((0.0, 0.0));
        let (debit, credit) = balances_within_period
            .get(&party.name)
            .copied()
            .unwrap_or((0.0, 0.0));
        let (closing_debit, closing_credit) =
            toggle_debit_credit(opening_debit + debit, opening_credit + credit);
        let has_value = opening_debit != 0.0
            || opening_credit != 0.0
            || debit != 0.0
            || credit != 0.0
            || closing_debit != 0.0
            || closing_credit != 0.0;

        if filters.exclude_zero_balance_parties && closing_debit == 0.0 && closing_credit == 0.0 {
            continue;
        }

        if filters.show_zero_values || has_value {
            let row = TrialBalancePartyRow {
                party: party.name,
                party_name: show_party_name.then_some(party.party_name).flatten(),
                opening_debit,
                opening_credit,
                debit,
                credit,
                closing_debit,
                closing_credit,
                currency: company_currency.to_string(),
            };
            total_row.add(&row);
            rows.push(row);
        }
    }

    rows.push(total_row);
    rows
}

pub fn get_opening_balances(
    filters: &TrialBalanceForPartyFilters,
    gl_entries: &[GlEntry],
) -> BTreeMap<String, (f64, f64)> {
    let mut grouped: BTreeMap<String, (f64, f64)> = BTreeMap::new();

    for entry in gl_entries {
        if !base_gl_match(filters, entry) {
            continue;
        }

        let is_opening_row = entry.posting_date < filters.from_date
            || (entry.is_opening == "Yes" && entry.posting_date <= filters.to_date);

        if is_opening_row {
            let balance = grouped.entry(entry.party.clone()).or_insert((0.0, 0.0));
            balance.0 += entry.debit;
            balance.1 += entry.credit;
        }
    }

    grouped
        .into_iter()
        .map(|(party, (debit, credit))| (party, toggle_debit_credit(debit, credit)))
        .collect()
}

pub fn get_balances_within_period(
    filters: &TrialBalanceForPartyFilters,
    gl_entries: &[GlEntry],
) -> BTreeMap<String, (f64, f64)> {
    let mut grouped: BTreeMap<String, (f64, f64)> = BTreeMap::new();

    for entry in gl_entries {
        if !base_gl_match(filters, entry) {
            continue;
        }

        let within_period = entry.posting_date >= filters.from_date
            && entry.posting_date <= filters.to_date
            && entry.is_opening == "No";

        if within_period {
            let balance = grouped.entry(entry.party.clone()).or_insert((0.0, 0.0));
            balance.0 += entry.debit;
            balance.1 += entry.credit;
        }
    }

    grouped
}

pub fn toggle_debit_credit(debit: f64, credit: f64) -> (f64, f64) {
    if flt(debit) > flt(credit) {
        (flt(debit) - flt(credit), 0.0)
    } else {
        (0.0, flt(credit) - flt(debit))
    }
}

pub fn get_columns(
    filters: &TrialBalanceForPartyFilters,
    show_party_name: bool,
) -> Vec<ReportColumn> {
    let mut columns = vec![
        ReportColumn::link(
            &filters.party_type,
            "party",
            &filters.party_type,
            200,
            false,
        ),
        ReportColumn::currency("Opening (Dr)", "opening_debit", 120),
        ReportColumn::currency("Opening (Cr)", "opening_credit", 120),
        ReportColumn::currency("Debit", "debit", 120),
        ReportColumn::currency("Credit", "credit", 120),
        ReportColumn::currency("Closing (Dr)", "closing_debit", 120),
        ReportColumn::currency("Closing (Cr)", "closing_credit", 120),
        ReportColumn::link("Currency", "currency", "Currency", 0, true),
    ];

    if show_party_name {
        columns.insert(
            1,
            ReportColumn::data(&format!("{} Name", filters.party_type), "party_name", 200),
        );
    }

    columns
}

pub fn is_party_name_visible(filters: &TrialBalanceForPartyFilters) -> bool {
    match filters.party_type.as_str() {
        "Customer" => filters.customer_naming_by.as_deref() == Some("Naming Series"),
        "Supplier" => filters.supplier_naming_by.as_deref() == Some("Naming Series"),
        _ => true,
    }
}

fn base_gl_match(filters: &TrialBalanceForPartyFilters, entry: &GlEntry) -> bool {
    !entry.is_cancelled
        && entry.company == filters.company
        && entry.party_type == filters.party_type
        && !entry.party.is_empty()
        && (filters.account_filter.is_empty() || filters.account_filter.contains(&entry.account))
}

fn party_name_field(filters: &TrialBalanceForPartyFilters) -> String {
    match filters.party_type.as_str() {
        "Customer" | "Supplier" | "Employee" | "Member" => {
            format!("{}_name", scrub(&filters.party_type))
        }
        "Shareholder" => "title".to_string(),
        _ => "name".to_string(),
    }
}

fn scrub(value: &str) -> String {
    let mut output = String::new();
    let mut last_was_underscore = false;

    for ch in value.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            output.push(ch);
            last_was_underscore = false;
        } else if !last_was_underscore {
            output.push('_');
            last_was_underscore = true;
        }
    }

    output.trim_matches('_').to_string()
}

fn flt(value: f64) -> f64 {
    value
}

impl TrialBalancePartyRow {
    fn totals(currency: &str) -> Self {
        Self {
            party: "'Totals'".to_string(),
            party_name: None,
            opening_debit: 0.0,
            opening_credit: 0.0,
            debit: 0.0,
            credit: 0.0,
            closing_debit: 0.0,
            closing_credit: 0.0,
            currency: currency.to_string(),
        }
    }

    fn add(&mut self, row: &TrialBalancePartyRow) {
        self.opening_debit += row.opening_debit;
        self.opening_credit += row.opening_credit;
        self.debit += row.debit;
        self.credit += row.credit;
        self.closing_debit += row.closing_debit;
        self.closing_credit += row.closing_credit;
    }
}
