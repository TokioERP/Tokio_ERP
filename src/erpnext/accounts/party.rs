use std::collections::BTreeMap;

pub const PURCHASE_TRANSACTION_TYPES: &[&str] = &[
    "Supplier Quotation",
    "Purchase Order",
    "Purchase Receipt",
    "Purchase Invoice",
];
pub const SALES_TRANSACTION_TYPES: &[&str] = &[
    "Quotation",
    "Sales Order",
    "Delivery Note",
    "Sales Invoice",
    "POS Invoice",
];
pub const TRANSACTION_TYPES: &[&str] = &[
    "Supplier Quotation",
    "Purchase Order",
    "Purchase Receipt",
    "Purchase Invoice",
    "Quotation",
    "Sales Order",
    "Delivery Note",
    "Sales Invoice",
    "POS Invoice",
];

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanyDefaults {
    pub default_receivable_account: Option<String>,
    pub default_payable_account: Option<String>,
    pub default_advance_received_account: Option<String>,
    pub default_advance_paid_account: Option<String>,
    pub default_payment_terms: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyRecord {
    pub payment_terms: Option<String>,
    pub group: Option<String>,
    pub disabled: bool,
    pub frozen: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyAccountInputs {
    pub direct_accounts: BTreeMap<(String, String, String), String>,
    pub direct_advance_accounts: BTreeMap<(String, String, String), String>,
    pub group_accounts: BTreeMap<(String, String, String), String>,
    pub group_advance_accounts: BTreeMap<(String, String, String), String>,
    pub party_groups: BTreeMap<(String, String), String>,
    pub company_defaults: BTreeMap<String, CompanyDefaults>,
    pub party_type_account_types: BTreeMap<String, String>,
    pub fallback_default_accounts: BTreeMap<(String, String), String>,
    pub account_currencies: BTreeMap<String, String>,
    pub existing_gle_currency: Option<String>,
    pub existing_gle_account: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountDueDateResult {
    pub party: Option<String>,
    pub account_fieldname: Option<String>,
    pub account: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentTerm {
    pub due_date_based_on: String,
    pub credit_days: i32,
    pub credit_months: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PartyFrozenDisabledError {
    Disabled,
    Frozen,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressRecord {
    pub name: String,
    pub is_shipping_address: bool,
}

impl PaymentTerm {
    pub fn days_after_invoice(days: i32) -> Self {
        Self {
            due_date_based_on: "Day(s) after invoice date".to_string(),
            credit_days: days,
            credit_months: 0,
        }
    }

    pub fn days_after_invoice_month_end(days: i32) -> Self {
        Self {
            due_date_based_on: "Day(s) after the end of the invoice month".to_string(),
            credit_days: days,
            credit_months: 0,
        }
    }

    pub fn months_after_invoice_month_end(months: i32) -> Self {
        Self {
            due_date_based_on: String::new(),
            credit_days: 0,
            credit_months: months,
        }
    }
}

pub fn set_account_and_due_date(
    party: Option<&str>,
    account: Option<&str>,
    party_type: &str,
    company: Option<&str>,
    posting_date: Option<&str>,
    bill_date: Option<&str>,
    doctype: &str,
    inputs: &PartyAccountInputs,
    template_terms: &[PaymentTerm],
) -> AccountDueDateResult {
    if !matches!(
        doctype,
        "POS Invoice" | "Sales Invoice" | "Purchase Invoice"
    ) {
        return AccountDueDateResult {
            party: party.map(ToOwned::to_owned),
            ..Default::default()
        };
    }

    let party_account = party
        .and_then(|party| {
            company.map(|company| {
                choose_party_account(party_type, Some(party), company, false, inputs)
                    .into_iter()
                    .next()
            })
        })
        .flatten()
        .or_else(|| account.map(ToOwned::to_owned));

    AccountDueDateResult {
        party: party.map(ToOwned::to_owned),
        account_fieldname: Some(if party_type == "Customer" {
            "debit_to".to_string()
        } else {
            "credit_to".to_string()
        }),
        account: party_account,
        due_date: get_due_date(
            posting_date,
            party_type,
            party,
            company,
            bill_date,
            template_terms,
        ),
    }
}

pub fn choose_party_account(
    party_type: &str,
    party: Option<&str>,
    company: &str,
    include_advance: bool,
    inputs: &PartyAccountInputs,
) -> Vec<String> {
    let account = choose_base_party_account(party_type, party, company, inputs);
    if include_advance && matches!(party_type, "Customer" | "Supplier" | "Student") {
        let mut accounts = account.into_iter().collect::<Vec<_>>();
        if let Some(advance_account) =
            party.and_then(|party| choose_party_advance_account(party_type, party, company, inputs))
        {
            accounts.push(advance_account);
        }
        accounts
    } else {
        account.into_iter().collect()
    }
}

pub fn choose_party_advance_account(
    party_type: &str,
    party: &str,
    company: &str,
    inputs: &PartyAccountInputs,
) -> Option<String> {
    if let Some(account) = inputs
        .direct_advance_accounts
        .get(&key(party_type, party, company))
    {
        return Some(account.clone());
    }

    let group_doctype = party_group_doctype(party_type);
    if let Some(group_doctype) = group_doctype {
        if let Some(group) = inputs.party_groups.get(&party_key(party_type, party)) {
            if let Some(account) =
                inputs
                    .group_advance_accounts
                    .get(&key(group_doctype, group, company))
            {
                return Some(account.clone());
            }
        }
    }

    let defaults = inputs.company_defaults.get(company)?;
    if party_type == "Customer" {
        defaults.default_advance_received_account.clone()
    } else {
        defaults.default_advance_paid_account.clone()
    }
}

pub fn get_due_date(
    posting_date: Option<&str>,
    party_type: &str,
    party: Option<&str>,
    _company: Option<&str>,
    bill_date: Option<&str>,
    template_terms: &[PaymentTerm],
) -> Option<String> {
    party?;
    let base_date = bill_date.or(posting_date)?;
    let mut due_date = base_date.to_string();
    if !template_terms.is_empty() {
        due_date = get_due_date_from_template(
            posting_date.unwrap_or(base_date),
            bill_date,
            template_terms,
        );
    } else if party_type == "Supplier" {
        due_date = base_date.to_string();
    }
    if let Some(posting_date) = posting_date {
        if compare_dates(&due_date, posting_date).is_lt() {
            due_date = posting_date.to_string();
        }
    }
    Some(due_date)
}

pub fn get_due_date_from_template(
    posting_date: &str,
    bill_date: Option<&str>,
    terms: &[PaymentTerm],
) -> String {
    let mut due_date = parse_date(bill_date.unwrap_or(posting_date));
    for term in terms {
        let candidate = if term.due_date_based_on == "Day(s) after invoice date" {
            add_days(due_date, term.credit_days)
        } else if term.due_date_based_on == "Day(s) after the end of the invoice month" {
            add_days(
                last_day_of_month(due_date.year, due_date.month),
                term.credit_days,
            )
        } else {
            let shifted = add_months(due_date, term.credit_months);
            last_day_of_month(shifted.year, shifted.month)
        };
        if candidate > due_date {
            due_date = candidate;
        }
    }
    due_date.to_string()
}

pub fn get_payment_terms_template(
    party_type: &str,
    party: &PartyRecord,
    group_templates: &BTreeMap<String, String>,
    company_template: Option<&str>,
) -> Option<String> {
    if !matches!(party_type, "Customer" | "Supplier") {
        return None;
    }
    party
        .payment_terms
        .clone()
        .or_else(|| {
            party
                .group
                .as_ref()
                .and_then(|group| group_templates.get(group).cloned())
        })
        .or_else(|| company_template.map(ToOwned::to_owned))
}

pub fn choose_address_tax_category(
    tax_category: Option<&str>,
    billing_address_tax_category: Option<&str>,
    shipping_address_tax_category: Option<&str>,
    determine_from: &str,
) -> String {
    let selected = if determine_from == "Shipping Address" {
        shipping_address_tax_category.or(tax_category)
    } else {
        billing_address_tax_category.or(tax_category)
    };
    selected.unwrap_or_default().to_string()
}

pub fn build_tax_args(
    party: &str,
    party_type: &str,
    company: &str,
    customer_group: Option<&str>,
    supplier_group: Option<&str>,
    tax_category: Option<&str>,
    billing_address: Option<&str>,
    shipping_address: Option<&str>,
    use_for_shopping_cart: Option<bool>,
) -> BTreeMap<String, String> {
    let mut args = BTreeMap::from([
        (scrub(party_type), party.to_string()),
        ("company".to_string(), company.to_string()),
    ]);
    if let Some(tax_category) = tax_category {
        args.insert("tax_category".to_string(), tax_category.to_string());
    }
    if let Some(customer_group) = customer_group {
        args.insert("customer_group".to_string(), customer_group.to_string());
    }
    if let Some(supplier_group) = supplier_group {
        args.insert("supplier_group".to_string(), supplier_group.to_string());
    }
    if let Some(billing_address) = billing_address {
        args.insert("billing_address".to_string(), billing_address.to_string());
    }
    if let Some(shipping_address) = shipping_address {
        args.insert("shipping_address".to_string(), shipping_address.to_string());
    }

    if matches!(party_type, "Customer" | "Lead" | "Prospect" | "CRM Deal") {
        args.insert("tax_type".to_string(), "Sales".to_string());
        if matches!(party_type, "Lead" | "Prospect" | "CRM Deal") {
            args.insert("customer".to_string(), String::new());
            args.remove(&scrub(party_type));
        }
    } else {
        args.insert("tax_type".to_string(), "Purchase".to_string());
    }

    if use_for_shopping_cart.unwrap_or(false) {
        args.insert("use_for_shopping_cart".to_string(), "1".to_string());
    }
    args
}

pub fn validate_party_frozen_disabled(
    _company: &str,
    party_type: &str,
    _party_name: &str,
    party: &PartyRecord,
    role_allowed_for_frozen_entries: Option<&str>,
    user_roles: &[String],
    ignore_party_validation: bool,
) -> Result<(), PartyFrozenDisabledError> {
    if ignore_party_validation {
        return Ok(());
    }
    if matches!(party_type, "Customer" | "Supplier") {
        if party.disabled {
            return Err(PartyFrozenDisabledError::Disabled);
        }
        if party.frozen
            && !role_allowed_for_frozen_entries
                .map(|role| user_roles.iter().any(|user_role| user_role == role))
                .unwrap_or(false)
        {
            return Err(PartyFrozenDisabledError::Frozen);
        }
    }
    Ok(())
}

pub fn get_party_shipping_address(addresses: &[AddressRecord]) -> Option<String> {
    if addresses
        .first()
        .map(|address| address.is_shipping_address)
        .unwrap_or(false)
    {
        return addresses.first().map(|address| address.name.clone());
    }
    if addresses.len() == 1 {
        addresses.first().map(|address| address.name.clone())
    } else {
        None
    }
}

fn choose_base_party_account(
    party_type: &str,
    party: Option<&str>,
    company: &str,
    inputs: &PartyAccountInputs,
) -> Option<String> {
    if party.is_none() && matches!(party_type, "Customer" | "Supplier") {
        return company_default_party_account(party_type, company, inputs);
    }

    let party = party?;
    let mut account = inputs
        .direct_accounts
        .get(&key(party_type, party, company))
        .cloned();

    if account.is_none() {
        if let Some(group_doctype) = party_group_doctype(party_type) {
            if let Some(group) = inputs.party_groups.get(&party_key(party_type, party)) {
                account = inputs
                    .group_accounts
                    .get(&key(group_doctype, group, company))
                    .cloned();
            }
        }
    }

    if account.is_none() && matches!(party_type, "Customer" | "Supplier") {
        account = company_default_party_account(party_type, company, inputs);
    }

    if let Some(existing_gle_currency) = inputs.existing_gle_currency.as_deref() {
        let account_currency = account
            .as_ref()
            .and_then(|account| inputs.account_currencies.get(account))
            .map(String::as_str);
        if account
            .as_ref()
            .map(|_| account_currency != Some(existing_gle_currency))
            .unwrap_or(true)
        {
            account = inputs.existing_gle_account.clone();
        }
    }

    if account.is_none() {
        if let Some(account_type) = inputs.party_type_account_types.get(party_type) {
            account = inputs
                .fallback_default_accounts
                .get(&(company.to_string(), account_type.to_lowercase()))
                .cloned();
        }
    }
    account
}

fn company_default_party_account(
    party_type: &str,
    company: &str,
    inputs: &PartyAccountInputs,
) -> Option<String> {
    let defaults = inputs.company_defaults.get(company)?;
    if party_type == "Customer" {
        defaults.default_receivable_account.clone()
    } else {
        defaults.default_payable_account.clone()
    }
}

fn key(party_type: &str, party: &str, company: &str) -> (String, String, String) {
    (
        party_type.to_string(),
        party.to_string(),
        company.to_string(),
    )
}

fn party_key(party_type: &str, party: &str) -> (String, String) {
    (party_type.to_string(), party.to_string())
}

fn party_group_doctype(party_type: &str) -> Option<&'static str> {
    match party_type {
        "Customer" => Some("Customer Group"),
        "Supplier" => Some("Supplier Group"),
        _ => None,
    }
}

fn scrub(value: &str) -> String {
    value.trim().to_lowercase().replace(' ', "_")
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SimpleDate {
    year: i32,
    month: i32,
    day: i32,
}

impl ToString for SimpleDate {
    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn parse_date(value: &str) -> SimpleDate {
    let mut parts = value
        .split('-')
        .map(|part| part.parse::<i32>().unwrap_or(0));
    SimpleDate {
        year: parts.next().unwrap_or(0),
        month: parts.next().unwrap_or(1),
        day: parts.next().unwrap_or(1),
    }
}

fn compare_dates(left: &str, right: &str) -> std::cmp::Ordering {
    parse_date(left).cmp(&parse_date(right))
}

fn add_days(mut date: SimpleDate, days: i32) -> SimpleDate {
    for _ in 0..days {
        date.day += 1;
        let last = days_in_month(date.year, date.month);
        if date.day > last {
            date.day = 1;
            date.month += 1;
            if date.month > 12 {
                date.month = 1;
                date.year += 1;
            }
        }
    }
    date
}

fn add_months(mut date: SimpleDate, months: i32) -> SimpleDate {
    date.month += months;
    while date.month > 12 {
        date.month -= 12;
        date.year += 1;
    }
    while date.month < 1 {
        date.month += 12;
        date.year -= 1;
    }
    date.day = date.day.min(days_in_month(date.year, date.month));
    date
}

fn last_day_of_month(year: i32, month: i32) -> SimpleDate {
    SimpleDate {
        year,
        month,
        day: days_in_month(year, month),
    }
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
