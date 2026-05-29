use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyFilters {
    pub company: Option<String>,
    pub presentation_currency: Option<String>,
    pub to_date: Option<String>,
    pub period_end_date: Option<String>,
    pub to_fiscal_year: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyInfo {
    pub company: String,
    pub company_currency: String,
    pub presentation_currency: String,
    pub report_date: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurrencyRate {
    pub from_currency: String,
    pub to_currency: String,
    pub date: String,
    pub rate: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub account_currency: String,
    pub account: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryColumn {
    pub fieldname: String,
    pub doctype: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyRecord {
    pub name: String,
    pub tax_id: String,
    pub supplier_group: Option<String>,
    pub customer_group: Option<String>,
    pub territory: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentRow {
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySpec {
    pub doctype: String,
    pub selects: Vec<String>,
    pub conditions: Vec<String>,
    pub order_by: Vec<String>,
    pub group_by: Vec<String>,
    pub inner_joins: Vec<String>,
    pub distinct: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalPaymentArgs {
    pub account: String,
    pub party: String,
    pub party_name: String,
    pub account_fieldname: String,
    pub party_account: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommonConditionOptions {
    pub doctype: String,
    pub child_doctype: Option<String>,
    pub payments: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingDimension {
    pub fieldname: String,
    pub document_type: String,
    pub is_tree: bool,
}

impl CurrencyRate {
    pub fn new(from_currency: &str, to_currency: &str, date: &str, rate: f64) -> Self {
        Self {
            from_currency: from_currency.to_string(),
            to_currency: to_currency.to_string(),
            date: date.to_string(),
            rate,
        }
    }
}

impl QueryColumn {
    pub fn new(fieldname: &str) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            doctype: None,
        }
    }

    pub fn with_doctype(fieldname: &str, doctype: &str) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            doctype: Some(doctype.to_string()),
        }
    }
}

impl PartyRecord {
    pub fn supplier(name: &str, tax_id: &str, supplier_group: &str) -> Self {
        Self {
            name: name.to_string(),
            tax_id: tax_id.to_string(),
            supplier_group: Some(supplier_group.to_string()),
            customer_group: None,
            territory: None,
        }
    }

    pub fn customer(name: &str, tax_id: &str, customer_group: &str, territory: &str) -> Self {
        Self {
            name: name.to_string(),
            tax_id: tax_id.to_string(),
            supplier_group: None,
            customer_group: Some(customer_group.to_string()),
            territory: Some(territory.to_string()),
        }
    }
}

impl DocumentRow {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl QuerySpec {
    pub fn new(doctype: &str) -> Self {
        Self {
            doctype: doctype.to_string(),
            selects: Vec::new(),
            conditions: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            inner_joins: Vec::new(),
            distinct: false,
        }
    }
}

impl AccountingDimension {
    pub fn new_tree(fieldname: &str, document_type: &str) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            document_type: document_type.to_string(),
            is_tree: true,
        }
    }

    pub fn new(fieldname: &str, document_type: &str) -> Self {
        Self {
            fieldname: fieldname.to_string(),
            document_type: document_type.to_string(),
            is_tree: false,
        }
    }
}

pub fn get_currency(
    filters: &CurrencyFilters,
    default_company: Option<&str>,
    company_currency: impl Fn(&str) -> String,
    fiscal_year_to_date: impl Fn(&str) -> Option<String>,
) -> CurrencyInfo {
    let company = get_appropriate_company(filters.company.as_deref(), default_company);
    let company_currency = company_currency(&company);
    let presentation_currency = filters
        .presentation_currency
        .clone()
        .unwrap_or_else(|| company_currency.clone());
    let report_date = filters
        .to_date
        .clone()
        .or_else(|| filters.period_end_date.clone())
        .or_else(|| {
            filters
                .to_fiscal_year
                .as_deref()
                .and_then(fiscal_year_to_date)
        })
        .unwrap_or_default();

    CurrencyInfo {
        company,
        company_currency,
        presentation_currency,
        report_date,
    }
}

pub fn convert(
    value: f64,
    from_currency: &str,
    to_currency: &str,
    date: &str,
    rates: &[CurrencyRate],
) -> f64 {
    value / get_rate_as_at(date, from_currency, to_currency, rates)
}

pub fn get_rate_as_at(
    date: &str,
    from_currency: &str,
    to_currency: &str,
    rates: &[CurrencyRate],
) -> f64 {
    rates
        .iter()
        .find(|rate| {
            rate.date == date
                && rate.from_currency == from_currency
                && rate.to_currency == to_currency
        })
        .map_or(1.0, |rate| if rate.rate == 0.0 { 1.0 } else { rate.rate })
}

pub fn convert_to_presentation_currency(
    gl_entries: &mut [GlEntry],
    currency_info: &CurrencyInfo,
    account_filter: Option<&[String]>,
    rates: &[CurrencyRate],
) {
    let presentation_currency = &currency_info.presentation_currency;
    let company_currency = &currency_info.company_currency;
    let mut account_currencies = gl_entries
        .iter()
        .map(|entry| entry.account_currency.clone())
        .collect::<Vec<_>>();
    account_currencies.sort_unstable();
    account_currencies.dedup();

    let exchange_gain_or_loss = account_filter
        .is_some_and(|accounts| accounts.len() == 1 && accounts[0] == "Exchange Gain/Loss");

    for entry in gl_entries {
        if account_currencies.len() == 1
            && entry.account_currency == *presentation_currency
            && !exchange_gain_or_loss
        {
            entry.debit = entry.debit_in_account_currency;
            entry.credit = entry.credit_in_account_currency;
        } else {
            let converted_debit = convert(
                entry.debit,
                presentation_currency,
                company_currency,
                &currency_info.report_date,
                rates,
            );
            let converted_credit = convert(
                entry.credit,
                presentation_currency,
                company_currency,
                &currency_info.report_date,
                rates,
            );

            if entry.debit != 0.0 {
                entry.debit = converted_debit;
            }
            if entry.credit != 0.0 {
                entry.credit = converted_credit;
            }
        }
    }
}

pub fn get_appropriate_company(company: Option<&str>, default_company: Option<&str>) -> String {
    company
        .or(default_company)
        .map(str::to_string)
        .unwrap_or_default()
}

pub fn get_query_columns(report_columns: &[QueryColumn]) -> String {
    if report_columns.is_empty() {
        return String::new();
    }

    report_columns
        .iter()
        .map(|column| {
            if let Some(doctype) = &column.doctype {
                format!("`{}`.`{}`", get_table_name(doctype), column.fieldname)
            } else {
                column.fieldname.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn get_values_for_columns(
    report_columns: &[QueryColumn],
    report_row: &BTreeMap<String, Value>,
) -> BTreeMap<String, Value> {
    report_columns
        .iter()
        .map(|column| {
            (
                column.fieldname.clone(),
                report_row
                    .get(&column.fieldname)
                    .cloned()
                    .unwrap_or(Value::Null),
            )
        })
        .collect()
}

pub fn get_party_details(
    party_type: &str,
    party_list: &[String],
    records: &[PartyRecord],
) -> BTreeMap<String, PartyRecord> {
    records
        .iter()
        .filter(|record| party_list.contains(&record.name))
        .map(|record| {
            let mut projected = PartyRecord {
                name: record.name.clone(),
                tax_id: record.tax_id.clone(),
                supplier_group: None,
                customer_group: None,
                territory: None,
            };
            if party_type == "Supplier" {
                projected.supplier_group = record.supplier_group.clone();
            } else {
                projected.customer_group = record.customer_group.clone();
                projected.territory = record.territory.clone();
            }
            (record.name.clone(), projected)
        })
        .collect()
}

pub fn get_taxes_query_spec(
    invoice_list: &[DocumentRow],
    doctype: &str,
    parenttype: &str,
) -> QuerySpec {
    let mut spec = QuerySpec::new(doctype);
    spec.selects.push("account_head".to_string());
    spec.distinct = true;
    spec.conditions = vec![
        format!("parenttype = {parenttype}"),
        "docstatus = 1".to_string(),
        "account_head is not null".to_string(),
        format!("parent in [{}]", names(invoice_list).join(", ")),
    ];
    spec.order_by.push("account_head".to_string());

    if doctype == "Purchase Taxes and Charges" {
        spec.conditions
            .push("category in [Total, Valuation and Total]".to_string());
    } else if doctype != "Sales Taxes and Charges" {
        spec.conditions
            .push("charge_type in [On Paid Amount, Actual]".to_string());
    }

    spec
}

pub fn get_journal_entries_spec(args: &JournalPaymentArgs) -> QuerySpec {
    let mut spec = QuerySpec::new("Journal Entry");
    spec.inner_joins.push(
        "Journal Entry Account on Journal Entry.name = Journal Entry Account.parent".to_string(),
    );
    spec.selects = vec![
        "voucher_type as doctype".to_string(),
        "name".to_string(),
        "posting_date".to_string(),
        format!("Journal Entry Account.account as {}", args.account),
        format!("Journal Entry Account.party as {}", args.party),
        format!("Journal Entry Account.party as {}", args.party_name),
        "bill_no".to_string(),
        "bill_date".to_string(),
        "remark as remarks".to_string(),
        "total_amount as base_net_total".to_string(),
        "total_amount as base_grand_total".to_string(),
        "mode_of_payment".to_string(),
        "Journal Entry Account.project".to_string(),
    ];
    spec.conditions = vec![
        "voucher_type = Journal Entry".to_string(),
        "docstatus = 1".to_string(),
        format!("Journal Entry Account.party = filters.{}", args.party),
        format!(
            "Journal Entry Account.account in [{}]",
            args.party_account.join(", ")
        ),
    ];
    spec.order_by = vec!["posting_date desc".to_string(), "name desc".to_string()];
    spec
}

pub fn get_payment_entries_spec(args: &JournalPaymentArgs) -> QuerySpec {
    let mut spec = QuerySpec::new("Payment Entry");
    spec.selects = vec![
        "'Payment Entry' as doctype".to_string(),
        "name".to_string(),
        "posting_date".to_string(),
        format!("{} as {}", args.account_fieldname, args.account),
        format!("party as {}", args.party),
        format!("party_name as {}", args.party_name),
        "remarks".to_string(),
        "paid_amount as base_net_total".to_string(),
        "paid_amount_after_tax as base_grand_total".to_string(),
        "mode_of_payment".to_string(),
        "project".to_string(),
        "cost_center".to_string(),
    ];
    spec.conditions = vec![
        "docstatus = 1".to_string(),
        format!("party = filters.{}", args.party),
        format!(
            "{} in [{}]",
            args.account_fieldname,
            args.party_account.join(", ")
        ),
    ];
    spec.order_by = vec!["posting_date desc".to_string(), "name desc".to_string()];
    spec
}

pub fn apply_common_conditions(
    filters: &BTreeMap<String, Value>,
    query: &mut QuerySpec,
    options: CommonConditionOptions,
) {
    if let Some(company) = string_filter(filters, "company") {
        query
            .conditions
            .push(format!("{}.company = {company}", options.doctype));
    }
    if let Some(from_date) = string_filter(filters, "from_date") {
        query
            .conditions
            .push(format!("{}.posting_date >= {from_date}", options.doctype));
    }
    if let Some(to_date) = string_filter(filters, "to_date") {
        query
            .conditions
            .push(format!("{}.posting_date <= {to_date}", options.doctype));
    }

    if options.payments {
        if let Some(cost_center) = string_filter(filters, "cost_center") {
            if options.doctype == "Journal Entry" {
                if let Some(child_doctype) = options.child_doctype.as_deref() {
                    query
                        .conditions
                        .push(format!("{child_doctype}.cost_center = {cost_center}"));
                }
            } else {
                query
                    .conditions
                    .push(format!("{}.cost_center = {cost_center}", options.doctype));
            }
        }
    } else if let Some(child_doctype) = options.child_doctype.as_deref() {
        let mut join_required = false;
        for field in ["cost_center", "warehouse", "item_group", "brand"] {
            if let Some(value) = string_filter(filters, field) {
                query
                    .conditions
                    .push(format!("{child_doctype}.{field} = {value}"));
                join_required = true;
            }
        }

        if join_required {
            query.inner_joins.push(format!(
                "{child_doctype} on {}.name = {child_doctype}.parent",
                options.doctype
            ));
            query
                .conditions
                .push(format!("{child_doctype}.parenttype = {}", options.doctype));
            query.distinct = true;
        }
    }
}

pub fn get_advance_taxes_and_charges_spec(invoice_list: &[DocumentRow]) -> QuerySpec {
    let mut spec = QuerySpec::new("Advance Taxes and Charges");
    spec.selects = vec![
        "parent".to_string(),
        "account_head".to_string(),
        "case add_deduct_tax Add then sum(base_tax_amount) else sum(base_tax_amount) * -1 as tax_amount".to_string(),
    ];
    spec.conditions = vec![
        format!("parent in [{}]", names(invoice_list).join(", ")),
        "charge_type in [On Paid Amount, Actual]".to_string(),
        "base_tax_amount != 0".to_string(),
    ];
    spec.group_by = vec![
        "parent".to_string(),
        "account_head".to_string(),
        "add_deduct_tax".to_string(),
    ];
    spec
}

pub fn filter_invoices_based_on_dimensions(
    filters: &BTreeMap<String, Value>,
    query: &mut QuerySpec,
    accounting_dimensions: &[AccountingDimension],
    get_dimension_with_children: impl Fn(&str, &str) -> Vec<String>,
) {
    for dimension in accounting_dimensions {
        if let Some(value) = string_filter(filters, &dimension.fieldname) {
            let values = if dimension.is_tree {
                get_dimension_with_children(&dimension.document_type, value)
            } else {
                vec![value.to_string()]
            };
            query.conditions.push(format!(
                "{}.{} in [{}]",
                query.doctype,
                dimension.fieldname,
                values.join(", ")
            ));
        }
    }
}

pub fn get_opening_row_spec(
    party_type: &str,
    party: &str,
    from_date: &str,
    company: &str,
    get_party_account: impl Fn(&str, &str, &str, bool) -> Vec<String>,
) -> QuerySpec {
    let accounts = get_party_account(party_type, party, company, true);
    let mut spec = QuerySpec::new("GL Entry");
    spec.selects = vec![
        "'Opening' as account".to_string(),
        "sum(debit) as debit".to_string(),
        "sum(credit) as credit".to_string(),
        "sum(debit) - sum(credit) as balance".to_string(),
    ];
    spec.conditions = vec![
        format!("account in [{}]", accounts.join(", ")),
        format!("party = {party}"),
        format!("posting_date < {from_date}"),
        "is_cancelled = 0".to_string(),
    ];
    spec
}

fn string_filter<'a>(filters: &'a BTreeMap<String, Value>, fieldname: &str) -> Option<&'a str> {
    filters.get(fieldname).and_then(Value::as_str)
}

fn get_table_name(doctype: &str) -> String {
    format!("tab{doctype}")
}

fn names(rows: &[DocumentRow]) -> Vec<String> {
    rows.iter().map(|row| row.name.clone()).collect()
}
