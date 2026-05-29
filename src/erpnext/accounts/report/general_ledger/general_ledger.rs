use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneralLedgerFilters {
    pub company: String,
    pub from_date: String,
    pub to_date: String,
    pub account: Vec<String>,
    pub party_type: Option<String>,
    pub party: Vec<String>,
    pub voucher_no: Option<String>,
    pub against_voucher_no: Option<String>,
    pub project: Vec<String>,
    pub cost_center: Vec<String>,
    pub finance_book: Option<String>,
    pub company_fb: Option<String>,
    pub include_default_book_entries: bool,
    pub categorize_by: Option<String>,
    pub group_by: Option<String>,
    pub show_cancelled_entries: bool,
    pub disable_opening_balance_calculation: bool,
    pub show_opening_entries: bool,
    pub include_dimensions: bool,
    pub show_remarks: bool,
    pub add_values_in_transaction_currency: bool,
    pub print_in_account_currency: bool,
    pub presentation_currency: Option<String>,
    pub account_currency: Option<String>,
    pub company_currency: Option<String>,
    pub show_amount_in_company_currency: bool,
    pub show_net_values_in_party_account: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountDetail {
    pub is_group: bool,
    pub account_currency: String,
    pub account_type: Option<String>,
    pub parent_account: Option<String>,
    pub lft: i32,
    pub rgt: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostCenterDetail {
    pub lft: i32,
    pub rgt: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeneralLedgerInput {
    pub company_currency: String,
    pub default_company: String,
    pub accounts: BTreeMap<String, AccountDetail>,
    pub valid_parties: BTreeMap<String, Vec<String>>,
    pub party_names: BTreeMap<String, BTreeMap<String, String>>,
    pub party_currencies: BTreeMap<String, BTreeMap<String, String>>,
    pub cost_centers: BTreeMap<String, CostCenterDetail>,
    pub supplier_invoice_details: BTreeMap<String, String>,
    pub gl_entries: Vec<GlEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub gl_entry: Option<String>,
    pub posting_date: Option<String>,
    pub account: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub party_name: Option<String>,
    pub voucher_type: Option<String>,
    pub voucher_subtype: Option<String>,
    pub voucher_no: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub against_voucher_type: Option<String>,
    pub against_voucher: Option<String>,
    pub against: Option<String>,
    pub is_opening: String,
    pub creation: Option<String>,
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub debit_in_transaction_currency: Option<f64>,
    pub credit_in_transaction_currency: Option<f64>,
    pub transaction_currency: Option<String>,
    pub account_currency: Option<String>,
    pub presentation_currency: Option<String>,
    pub remarks: Option<String>,
    pub bill_no: Option<String>,
    pub finance_book: Option<String>,
    pub balance: f64,
    pub is_cancelled: bool,
    pub is_blank: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: String,
    pub options: Option<String>,
    pub width: u16,
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeneralLedgerReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<GlEntry>,
}

impl GlEntry {
    pub fn new(
        gl_entry: &str,
        posting_date: &str,
        account: &str,
        debit: f64,
        credit: f64,
        is_opening: &str,
        voucher_type: &str,
        voucher_no: &str,
    ) -> Self {
        Self {
            gl_entry: Some(gl_entry.to_string()),
            posting_date: Some(posting_date.to_string()),
            account: Some(account.to_string()),
            party_type: None,
            party: None,
            party_name: None,
            voucher_type: Some(voucher_type.to_string()),
            voucher_subtype: None,
            voucher_no: Some(voucher_no.to_string()),
            cost_center: None,
            project: None,
            against_voucher_type: None,
            against_voucher: None,
            against: None,
            is_opening: is_opening.to_string(),
            creation: None,
            debit,
            credit,
            debit_in_account_currency: debit,
            credit_in_account_currency: credit,
            debit_in_transaction_currency: None,
            credit_in_transaction_currency: None,
            transaction_currency: None,
            account_currency: None,
            presentation_currency: None,
            remarks: None,
            bill_no: None,
            finance_book: None,
            balance: 0.0,
            is_cancelled: false,
            is_blank: false,
        }
    }

    pub fn empty_row() -> Self {
        Self {
            gl_entry: None,
            posting_date: None,
            account: None,
            party_type: None,
            party: None,
            party_name: None,
            voucher_type: None,
            voucher_subtype: None,
            voucher_no: None,
            cost_center: None,
            project: None,
            against_voucher_type: None,
            against_voucher: None,
            against: None,
            is_opening: "No".to_string(),
            creation: None,
            debit: 0.0,
            credit: 0.0,
            debit_in_account_currency: 0.0,
            credit_in_account_currency: 0.0,
            debit_in_transaction_currency: None,
            credit_in_transaction_currency: None,
            transaction_currency: None,
            account_currency: None,
            presentation_currency: None,
            remarks: None,
            bill_no: None,
            finance_book: None,
            balance: 0.0,
            is_cancelled: false,
            is_blank: true,
        }
    }

    pub fn with_party(mut self, party_type: &str, party: &str) -> Self {
        self.party_type = Some(party_type.to_string());
        self.party = Some(party.to_string());
        self
    }

    pub fn with_against(mut self, against_voucher_type: &str, against_voucher: &str) -> Self {
        self.against_voucher_type = Some(against_voucher_type.to_string());
        self.against_voucher = Some(against_voucher.to_string());
        self
    }

    pub fn with_transaction_currency(mut self, debit: f64, credit: f64, currency: &str) -> Self {
        self.debit_in_transaction_currency = Some(debit);
        self.credit_in_transaction_currency = Some(credit);
        self.transaction_currency = Some(currency.to_string());
        self
    }

    pub fn cancelled(mut self) -> Self {
        self.is_cancelled = true;
        self
    }

    fn total_row(label: &str) -> Self {
        let mut row = Self::empty_row();
        row.is_blank = false;
        row.account = Some(label.to_string());
        row
    }
}

impl ReportColumn {
    pub fn link(label: &str, fieldname: &str, options: &str, width: u16, hidden: bool) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Link".to_string(),
            options: Some(options.to_string()),
            width,
            hidden,
        }
    }

    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data".to_string(),
            options: None,
            width,
            hidden: false,
        }
    }

    pub fn currency(label: &str, fieldname: &str, options: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Currency".to_string(),
            options: Some(options.to_string()),
            width,
            hidden: false,
        }
    }
}

pub fn execute(
    mut filters: GeneralLedgerFilters,
    input: GeneralLedgerInput,
) -> Result<GeneralLedgerReport, String> {
    validate_filters(&mut filters, &input)?;
    validate_party(&filters, &input)?;
    filters = set_account_currency(filters, &input)?;
    let columns = get_columns(&filters, &input)?;
    let mut gl_entries = get_gl_entries(&filters, &input);
    apply_party_names(&mut gl_entries, &input);
    let rows = get_data_with_opening_closing(&filters, &gl_entries, &input);
    let mut rows = rows;
    get_result_as_list(&mut rows, &filters);
    Ok(GeneralLedgerReport { columns, rows })
}

pub fn validate_filters(
    filters: &mut GeneralLedgerFilters,
    input: &GeneralLedgerInput,
) -> Result<(), String> {
    if filters.print_in_account_currency && filters.account.is_empty() {
        return Err("Select an account to print in account currency".to_string());
    }
    if filters.company.is_empty() {
        return Err("Company is mandatory".to_string());
    }
    if filters.from_date.is_empty() && filters.to_date.is_empty() {
        return Err("From Date and To Date are mandatory".to_string());
    }

    for account in &filters.account {
        if !input.accounts.contains_key(account) {
            return Err(format!("Account {account} does not exists"));
        }
    }
    for cost_center in &filters.cost_center {
        if !input.cost_centers.contains_key(cost_center) {
            return Err(format!("Cost Center: {cost_center} does not exist"));
        }
    }

    if filters.categorize_by.is_none() {
        if let Some(group_by) = filters.group_by.as_ref() {
            filters.categorize_by = Some(group_by.replace("Group by", "Categorize by"));
        }
    }

    if !filters.account.is_empty()
        && filters.categorize_by.as_deref() == Some("Categorize by Account")
    {
        for account in &filters.account {
            if !input
                .accounts
                .get(account)
                .map(|detail| detail.is_group)
                .unwrap_or(false)
            {
                return Err(
                    "Can not filter based on Child Account, if grouped by Account".to_string(),
                );
            }
        }
    }

    if filters.voucher_no.is_some()
        && filters.categorize_by.as_deref() == Some("Categorize by Voucher")
    {
        return Err("Can not filter based on Voucher No, if grouped by Voucher".to_string());
    }

    if filters.from_date > filters.to_date {
        return Err("From Date must be before To Date".to_string());
    }

    if filters.include_default_book_entries {
        if let (Some(finance_book), Some(company_fb)) =
            (filters.finance_book.as_ref(), filters.company_fb.as_ref())
        {
            if !company_fb.is_empty() && finance_book != company_fb {
                return Err(
                    "To use a different finance book, please uncheck 'Include Default FB Entries'"
                        .to_string(),
                );
            }
        }
    }

    Ok(())
}

pub fn validate_party(
    filters: &GeneralLedgerFilters,
    input: &GeneralLedgerInput,
) -> Result<(), String> {
    if let Some(party_type) = filters.party_type.as_ref() {
        if !filters.party.is_empty() {
            let valid = input
                .valid_parties
                .get(party_type)
                .cloned()
                .unwrap_or_default();
            for party in &filters.party {
                if !valid.iter().any(|item| item == party) {
                    return Err(format!("Invalid {party_type}: {party}"));
                }
            }
        }
    }
    Ok(())
}

pub fn set_account_currency(
    mut filters: GeneralLedgerFilters,
    input: &GeneralLedgerInput,
) -> Result<GeneralLedgerFilters, String> {
    if !filters.account.is_empty() || (!filters.party.is_empty() && filters.party.len() == 1) {
        filters.company_currency = Some(input.company_currency.clone());
        let mut account_currency = None;

        if !filters.account.is_empty() {
            let first = input
                .accounts
                .get(&filters.account[0])
                .map(|detail| detail.account_currency.clone())
                .unwrap_or_else(|| input.company_currency.clone());
            if filters.account.iter().all(|account| {
                input
                    .accounts
                    .get(account)
                    .map(|detail| detail.account_currency == first)
                    .unwrap_or(false)
            }) {
                account_currency = Some(first);
            }
        } else if let (Some(party_type), Some(party)) =
            (filters.party_type.as_ref(), filters.party.first())
        {
            account_currency = input
                .gl_entries
                .iter()
                .find(|entry| {
                    entry.party_type.as_ref() == Some(party_type)
                        && entry.party.as_ref() == Some(party)
                })
                .and_then(|entry| entry.account_currency.clone())
                .or_else(|| {
                    if matches!(party_type.as_str(), "Employee" | "Shareholder" | "Member") {
                        None
                    } else {
                        input
                            .party_currencies
                            .get(party_type)
                            .and_then(|parties| parties.get(party))
                            .cloned()
                    }
                });
        }

        let account_currency = account_currency.unwrap_or_else(|| input.company_currency.clone());
        filters.account_currency = Some(account_currency.clone());
        if account_currency != input.company_currency && filters.presentation_currency.is_none() {
            filters.presentation_currency = Some(account_currency);
        }
    }

    Ok(filters)
}

pub fn get_conditions(filters: &GeneralLedgerFilters) -> Vec<String> {
    let mut conditions = Vec::new();

    if !filters.account.is_empty() {
        conditions.push("account in %(account)s".to_string());
    }
    if !filters.cost_center.is_empty() {
        conditions.push("cost_center in %(cost_center)s".to_string());
    }
    if filters.voucher_no.is_some() {
        conditions.push("voucher_no=%(voucher_no)s".to_string());
    }
    if filters.against_voucher_no.is_some() {
        conditions.push("against_voucher=%(against_voucher_no)s".to_string());
    }
    if filters.categorize_by.as_deref() == Some("Categorize by Party")
        && filters.party_type.is_none()
    {
        conditions.push("party_type in ('Customer', 'Supplier')".to_string());
    }
    if filters.party_type.is_some() {
        conditions.push("party_type=%(party_type)s".to_string());
    }
    if !filters.party.is_empty() {
        conditions.push("party in %(party)s".to_string());
    }

    if filters.disable_opening_balance_calculation
        || (filters.account.is_empty()
            && filters.party.is_empty()
            && !matches!(
                filters.categorize_by.as_deref(),
                Some("Categorize by Account") | Some("Categorize by Party")
            ))
    {
        conditions.push("(posting_date >=%(from_date)s or is_opening = 'Yes')".to_string());
    }

    conditions.push("(posting_date <=%(to_date)s or is_opening = 'Yes')".to_string());

    if !filters.project.is_empty() {
        conditions.push("project in %(project)s".to_string());
    }
    if filters.include_default_book_entries {
        if filters.finance_book.is_some() {
            conditions.push(
                "(finance_book in (%(finance_book)s, '') OR finance_book IS NULL)".to_string(),
            );
        } else {
            conditions
                .push("(finance_book in (%(company_fb)s, '') OR finance_book IS NULL)".to_string());
        }
    } else if filters.finance_book.is_some() {
        conditions
            .push("(finance_book in (%(finance_book)s, '') OR finance_book IS NULL)".to_string());
    } else {
        conditions.push("(finance_book in ('') OR finance_book IS NULL)".to_string());
    }
    if !filters.show_cancelled_entries {
        conditions.push("is_cancelled = 0".to_string());
    }

    conditions
}

pub fn get_gl_entries(filters: &GeneralLedgerFilters, input: &GeneralLedgerInput) -> Vec<GlEntry> {
    let account_filter = account_filter_with_children(&filters.account, input);
    let cost_center_filter = cost_center_filter_with_children(&filters.cost_center, input);
    let mut rows = input
        .gl_entries
        .iter()
        .filter(|entry| filters.show_cancelled_entries || !entry.is_cancelled)
        .filter(|entry| {
            account_filter.is_empty()
                || entry
                    .account
                    .as_ref()
                    .map(|account| account_filter.iter().any(|item| item == account))
                    .unwrap_or(false)
        })
        .filter(|entry| finance_book_matches(entry, filters))
        .filter(|entry| {
            filters
                .voucher_no
                .as_ref()
                .map(|voucher_no| entry.voucher_no.as_ref() == Some(voucher_no))
                .unwrap_or(true)
        })
        .filter(|entry| {
            filters
                .against_voucher_no
                .as_ref()
                .map(|against_voucher_no| {
                    entry.against_voucher.as_ref() == Some(against_voucher_no)
                })
                .unwrap_or(true)
        })
        .filter(|entry| {
            filters
                .party_type
                .as_ref()
                .map(|party_type| entry.party_type.as_ref() == Some(party_type))
                .unwrap_or(true)
        })
        .filter(|entry| {
            filters.party.is_empty()
                || entry
                    .party
                    .as_ref()
                    .map(|party| filters.party.iter().any(|item| item == party))
                    .unwrap_or(false)
        })
        .filter(|entry| {
            filters.project.is_empty()
                || entry
                    .project
                    .as_ref()
                    .map(|project| filters.project.iter().any(|item| item == project))
                    .unwrap_or(false)
        })
        .filter(|entry| {
            cost_center_filter.is_empty()
                || entry
                    .cost_center
                    .as_ref()
                    .map(|cost_center| cost_center_filter.iter().any(|item| item == cost_center))
                    .unwrap_or(false)
        })
        .cloned()
        .collect::<Vec<_>>();

    match filters.categorize_by.as_deref() {
        Some("Categorize by Account") => rows.sort_by(|left, right| {
            left.account
                .cmp(&right.account)
                .then(left.posting_date.cmp(&right.posting_date))
                .then(left.creation.cmp(&right.creation))
        }),
        Some("Categorize by Voucher") | Some("Categorize by Voucher (Consolidated)") => rows
            .sort_by(|left, right| {
                left.posting_date
                    .cmp(&right.posting_date)
                    .then(left.voucher_type.cmp(&right.voucher_type))
                    .then(left.voucher_no.cmp(&right.voucher_no))
            }),
        _ => rows.sort_by(|left, right| {
            left.posting_date
                .cmp(&right.posting_date)
                .then(left.account.cmp(&right.account))
                .then(left.creation.cmp(&right.creation))
        }),
    }

    rows
}

fn account_filter_with_children(accounts: &[String], input: &GeneralLedgerInput) -> Vec<String> {
    let mut expanded = Vec::new();
    for account in accounts {
        if !expanded.iter().any(|item| item == account) {
            expanded.push(account.clone());
        }
        let Some(parent) = input.accounts.get(account) else {
            continue;
        };
        if !parent.is_group {
            continue;
        }
        for (candidate_name, candidate) in &input.accounts {
            if candidate.lft >= parent.lft
                && candidate.rgt <= parent.rgt
                && !expanded.iter().any(|item| item == candidate_name)
            {
                expanded.push(candidate_name.clone());
            }
        }
    }
    expanded
}

fn cost_center_filter_with_children(
    cost_centers: &[String],
    input: &GeneralLedgerInput,
) -> Vec<String> {
    let mut expanded = Vec::new();
    for cost_center in cost_centers {
        if !expanded.iter().any(|item| item == cost_center) {
            expanded.push(cost_center.clone());
        }
        let Some(parent) = input.cost_centers.get(cost_center) else {
            continue;
        };
        for (candidate_name, candidate) in &input.cost_centers {
            if candidate.lft >= parent.lft
                && candidate.rgt <= parent.rgt
                && !expanded.iter().any(|item| item == candidate_name)
            {
                expanded.push(candidate_name.clone());
            }
        }
    }
    expanded
}

fn finance_book_matches(entry: &GlEntry, filters: &GeneralLedgerFilters) -> bool {
    let entry_book = entry.finance_book.as_deref().unwrap_or("");
    if filters.include_default_book_entries {
        if let Some(finance_book) = filters.finance_book.as_deref() {
            entry_book.is_empty() || entry_book == finance_book
        } else if let Some(company_fb) = filters.company_fb.as_deref() {
            entry_book.is_empty() || entry_book == company_fb
        } else {
            entry_book.is_empty()
        }
    } else if let Some(finance_book) = filters.finance_book.as_deref() {
        entry_book.is_empty() || entry_book == finance_book
    } else {
        entry_book.is_empty()
    }
}

pub fn get_data_with_opening_closing(
    filters: &GeneralLedgerFilters,
    gl_entries: &[GlEntry],
    input: &GeneralLedgerInput,
) -> Vec<GlEntry> {
    let mut gl_entries = gl_entries.to_vec();
    set_bill_no(&mut gl_entries, input);
    apply_party_names(&mut gl_entries, input);

    let mut gle_map = initialize_gle_map(&gl_entries, filters);
    let group_order = gle_group_order(&gl_entries, filters);
    let (totals, consolidated_entries) =
        get_accountwise_gle(filters, &gl_entries, &mut gle_map, input);
    let labels = labels();
    let mut data = Vec::new();

    push_total(&mut data, &totals, "opening", &labels);

    if filters.categorize_by.is_none() {
        let mut all_entries = Vec::new();
        for group_by_value in &group_order {
            if let Some(acc_dict) = gle_map.get(group_by_value) {
                all_entries.extend(acc_dict.entries.clone());
            }
        }
        data.extend(all_entries);
    } else if filters.categorize_by.as_deref() != Some("Categorize by Voucher (Consolidated)") {
        let set_opening_closing = filters.categorize_by.as_deref() != Some("Categorize by Voucher");
        let set_total = filters.categorize_by.is_some() || filters.voucher_no.is_none();

        for group_by_value in &group_order {
            let Some(acc_dict) = gle_map.get(group_by_value) else {
                continue;
            };
            if acc_dict.entries.is_empty() {
                continue;
            }
            data.push(GlEntry::empty_row());
            if set_opening_closing {
                push_total(&mut data, &acc_dict.totals, "opening", &labels);
            }
            data.extend(acc_dict.entries.clone());
            if set_total {
                push_total(&mut data, &acc_dict.totals, "total", &labels);
            }
            if set_opening_closing {
                push_total(&mut data, &acc_dict.totals, "closing", &labels);
            }
        }
        data.push(GlEntry::empty_row());
    } else {
        data.extend(consolidated_entries);
    }

    push_total(&mut data, &totals, "total", &labels);
    push_total(&mut data, &totals, "closing", &labels);
    data
}

pub fn get_group_by_field(group_by: Option<&str>) -> &'static str {
    match group_by {
        Some("Categorize by Party") => "party",
        Some("Categorize by Voucher (Consolidated)") | Some("Categorize by Account") => "account",
        _ => "voucher_no",
    }
}

pub fn get_result_as_list(data: &mut [GlEntry], filters: &GeneralLedgerFilters) {
    let mut balance = 0.0;
    for row in data {
        if row.posting_date.is_none() {
            balance = 0.0;
        }
        balance = get_balance(row, balance, "debit", "credit");
        row.balance = balance;
        row.account_currency = filters.account_currency.clone();
        row.presentation_currency = filters.presentation_currency.clone();
    }
}

pub fn get_balance(row: &GlEntry, balance: f64, debit_field: &str, credit_field: &str) -> f64 {
    balance + number_field(row, debit_field) - number_field(row, credit_field)
}

pub fn get_columns(
    filters: &GeneralLedgerFilters,
    input: &GeneralLedgerInput,
) -> Result<Vec<ReportColumn>, String> {
    let currency = filters
        .presentation_currency
        .as_deref()
        .unwrap_or(&input.company_currency);

    if filters.show_amount_in_company_currency && currency != input.company_currency {
        return Err("Presentation Currency cannot be used when Show Credit / Debit in Company Currency is enabled.".to_string());
    }

    let mut columns = vec![
        ReportColumn::link("GL Entry", "gl_entry", "GL Entry", 0, true),
        ReportColumn::data("Posting Date", "posting_date", 120),
        ReportColumn::link("Account", "account", "Account", 180, false),
        ReportColumn::currency(
            &format!("Debit ({currency})"),
            "debit",
            "presentation_currency",
            130,
        ),
        ReportColumn::currency(
            &format!("Credit ({currency})"),
            "credit",
            "presentation_currency",
            130,
        ),
        ReportColumn::currency(
            &format!("Balance ({currency})"),
            "balance",
            "presentation_currency",
            130,
        ),
    ];

    if filters.add_values_in_transaction_currency {
        columns.extend([
            ReportColumn::currency(
                "Debit (Transaction)",
                "debit_in_transaction_currency",
                "transaction_currency",
                130,
            ),
            ReportColumn::currency(
                "Credit (Transaction)",
                "credit_in_transaction_currency",
                "transaction_currency",
                130,
            ),
            ReportColumn::link(
                "Transaction Currency",
                "transaction_currency",
                "Currency",
                70,
                false,
            ),
        ]);
    }

    columns.extend([
        ReportColumn::data("Voucher Type", "voucher_type", 120),
        ReportColumn::data("Voucher Subtype", "voucher_subtype", 180),
        ReportColumn::link("Voucher No", "voucher_no", "voucher_type", 180, false),
        ReportColumn::data("Against Account", "against", 120),
        ReportColumn::data("Party Type", "party_type", 100),
        ReportColumn::data("Party", "party", 100),
        ReportColumn::data("Party Name", "party_name", 150),
        ReportColumn::data("Against Voucher Type", "against_voucher_type", 100),
        ReportColumn::link(
            "Against Voucher",
            "against_voucher",
            "against_voucher_type",
            100,
            false,
        ),
        ReportColumn::data("Supplier Invoice No", "bill_no", 100),
    ]);

    if filters.show_remarks {
        columns.push(ReportColumn::data("Remarks", "remarks", 400));
    }

    Ok(columns)
}

#[derive(Clone, Debug)]
struct GleBucket {
    totals: BTreeMap<String, GlEntry>,
    entries: Vec<GlEntry>,
}

fn labels() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("opening", "'Opening'"),
        ("total", "'Total'"),
        ("closing", "'Closing (Opening + Total)'"),
    ])
}

fn initialize_gle_map(
    gl_entries: &[GlEntry],
    filters: &GeneralLedgerFilters,
) -> BTreeMap<String, GleBucket> {
    let group_by = get_group_by_field(filters.categorize_by.as_deref());
    let mut gle_map = BTreeMap::new();
    for gle in gl_entries {
        let group_by_value = string_field(gle, group_by);
        gle_map.entry(group_by_value).or_insert_with(|| GleBucket {
            totals: get_totals_dict(),
            entries: Vec::new(),
        });
    }
    gle_map
}

fn gle_group_order(gl_entries: &[GlEntry], filters: &GeneralLedgerFilters) -> Vec<String> {
    let group_by = get_group_by_field(filters.categorize_by.as_deref());
    let mut order = Vec::new();
    for gle in gl_entries {
        let group_by_value = string_field(gle, group_by);
        if !order.iter().any(|item| item == &group_by_value) {
            order.push(group_by_value);
        }
    }
    order
}

fn get_accountwise_gle(
    filters: &GeneralLedgerFilters,
    gl_entries: &[GlEntry],
    gle_map: &mut BTreeMap<String, GleBucket>,
    input: &GeneralLedgerInput,
) -> (BTreeMap<String, GlEntry>, Vec<GlEntry>) {
    let mut totals = get_totals_dict();
    let mut consolidated_gle = BTreeMap::<String, GlEntry>::new();
    let mut entries = Vec::new();
    let group_by = get_group_by_field(filters.categorize_by.as_deref());
    let group_by_voucher_consolidated =
        filters.categorize_by.as_deref() == Some("Categorize by Voucher (Consolidated)");

    for gle in gl_entries {
        let group_by_value = string_field(gle, group_by);
        let opening = gle.posting_date.as_deref().unwrap_or("") < filters.from_date.as_str()
            || (gle.is_opening == "Yes"
                && !filters.show_opening_entries
                && !filters.disable_opening_balance_calculation);

        if opening {
            if !group_by_voucher_consolidated {
                if let Some(bucket) = gle_map.get_mut(&group_by_value) {
                    update_value_in_dict(&mut bucket.totals, "opening", gle, true, filters, input);
                    update_value_in_dict(&mut bucket.totals, "closing", gle, true, filters, input);
                }
            }
            update_value_in_dict(&mut totals, "opening", gle, true, filters, input);
            update_value_in_dict(&mut totals, "closing", gle, true, filters, input);
        } else if gle.posting_date.as_deref().unwrap_or("") <= filters.to_date.as_str()
            || (gle.is_opening == "Yes" && filters.show_opening_entries)
        {
            if !group_by_voucher_consolidated {
                if let Some(bucket) = gle_map.get_mut(&group_by_value) {
                    update_value_in_dict(&mut bucket.totals, "total", gle, false, filters, input);
                    update_value_in_dict(&mut bucket.totals, "closing", gle, false, filters, input);
                    update_value_in_dict(&mut totals, "total", gle, false, filters, input);
                    update_value_in_dict(&mut totals, "closing", gle, false, filters, input);
                    bucket.entries.push(gle.clone());
                }
            } else {
                let key = consolidated_key(gle);
                if let Some(existing) = consolidated_gle.get_mut(&key) {
                    accumulate_entry(existing, gle);
                    if filters.add_values_in_transaction_currency {
                        accumulate_transaction_currency(existing, gle);
                    }
                } else {
                    consolidated_gle.insert(key, gle.clone());
                }
            }
        }
    }

    for value in consolidated_gle.values() {
        update_value_in_dict(&mut totals, "total", value, false, filters, input);
        update_value_in_dict(&mut totals, "closing", value, false, filters, input);
        entries.push(value.clone());
    }

    (totals, entries)
}

fn get_totals_dict() -> BTreeMap<String, GlEntry> {
    BTreeMap::from([
        ("opening".to_string(), GlEntry::total_row("")),
        ("total".to_string(), GlEntry::total_row("")),
        ("closing".to_string(), GlEntry::total_row("")),
    ])
}

fn update_value_in_dict(
    data: &mut BTreeMap<String, GlEntry>,
    key: &str,
    gle: &GlEntry,
    show_net_values: bool,
    filters: &GeneralLedgerFilters,
    input: &GeneralLedgerInput,
) {
    let row = data.get_mut(key).expect("total bucket should exist");
    accumulate_entry(row, gle);
    if filters.add_values_in_transaction_currency && !matches!(key, "opening" | "closing" | "total")
    {
        accumulate_transaction_currency(row, gle);
    }

    if filters.show_net_values_in_party_account
        && row
            .account
            .as_ref()
            .and_then(|account| input.accounts.get(account))
            .and_then(|detail| detail.account_type.as_deref())
            .is_some_and(|account_type| matches!(account_type, "Receivable" | "Payable"))
        || show_net_values
    {
        let net_value = row.debit - row.credit;
        let net_value_account_currency =
            row.debit_in_account_currency - row.credit_in_account_currency;
        if net_value < 0.0 {
            row.credit = net_value.abs();
            row.debit = 0.0;
            row.credit_in_account_currency = net_value_account_currency.abs();
            row.debit_in_account_currency = 0.0;
        } else {
            row.debit = net_value.abs();
            row.credit = 0.0;
            row.debit_in_account_currency = net_value_account_currency.abs();
            row.credit_in_account_currency = 0.0;
        }
    }

    if let (Some(row_against), Some(gle_against)) =
        (row.against_voucher.as_mut(), gle.against_voucher.as_ref())
    {
        if !row_against.is_empty() {
            row_against.push_str(", ");
        }
        row_against.push_str(gle_against);
    }
}

fn accumulate_entry(target: &mut GlEntry, gle: &GlEntry) {
    target.debit += gle.debit;
    target.credit += gle.credit;
    target.debit_in_account_currency += gle.debit_in_account_currency;
    target.credit_in_account_currency += gle.credit_in_account_currency;
}

fn accumulate_transaction_currency(target: &mut GlEntry, gle: &GlEntry) {
    target.debit_in_transaction_currency = Some(
        target.debit_in_transaction_currency.unwrap_or_default()
            + gle.debit_in_transaction_currency.unwrap_or_default(),
    );
    target.credit_in_transaction_currency = Some(
        target.credit_in_transaction_currency.unwrap_or_default()
            + gle.credit_in_transaction_currency.unwrap_or_default(),
    );
}

fn push_total(
    data: &mut Vec<GlEntry>,
    totals: &BTreeMap<String, GlEntry>,
    key: &str,
    labels: &BTreeMap<&str, &str>,
) {
    let mut row = totals
        .get(key)
        .cloned()
        .unwrap_or_else(|| GlEntry::total_row(""));
    row.account = labels.get(key).map(|label| (*label).to_string());
    row.posting_date = None;
    row.is_blank = false;
    data.push(row);
}

fn set_bill_no(gl_entries: &mut [GlEntry], input: &GeneralLedgerInput) {
    for gl in gl_entries {
        if let Some(against_voucher) = gl.against_voucher.as_ref() {
            gl.bill_no = input.supplier_invoice_details.get(against_voucher).cloned();
        }
    }
}

fn apply_party_names(gl_entries: &mut [GlEntry], input: &GeneralLedgerInput) {
    for gl in gl_entries {
        if let (Some(party_type), Some(party)) = (gl.party_type.as_ref(), gl.party.as_ref()) {
            gl.party_name = input
                .party_names
                .get(party_type)
                .and_then(|parties| parties.get(party))
                .cloned();
        }
    }
}

fn consolidated_key(gle: &GlEntry) -> String {
    [
        gle.posting_date.as_deref().unwrap_or(""),
        gle.voucher_type.as_deref().unwrap_or(""),
        gle.voucher_no.as_deref().unwrap_or(""),
        gle.account.as_deref().unwrap_or(""),
        gle.party_type.as_deref().unwrap_or(""),
        gle.party.as_deref().unwrap_or(""),
    ]
    .join("|")
}

fn string_field(gle: &GlEntry, field: &str) -> String {
    match field {
        "account" => gle.account.clone(),
        "party" => gle.party.clone(),
        "voucher_no" => gle.voucher_no.clone(),
        _ => None,
    }
    .unwrap_or_default()
}

fn number_field(row: &GlEntry, field: &str) -> f64 {
    match field {
        "debit" => row.debit,
        "credit" => row.credit,
        "debit_in_account_currency" => row.debit_in_account_currency,
        "credit_in_account_currency" => row.credit_in_account_currency,
        _ => 0.0,
    }
}
