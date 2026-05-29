use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountType {
    Receivable,
    Payable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountsReceivableArgs {
    pub account_type: AccountType,
    pub naming_by: [String; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableFilters {
    pub company: Option<String>,
    pub report_date: Option<String>,
    pub finance_book: Option<String>,
    pub party_type: Option<String>,
    pub customer_group: Option<String>,
    pub territory: Option<String>,
    pub supplier_group: Option<String>,
    pub payment_terms_template: Option<String>,
    pub cost_center: Vec<String>,
    pub project: Vec<String>,
    pub calculate_ageing_with: Option<String>,
    pub ageing_based_on: Option<String>,
    pub range: Option<String>,
    pub account_type: Option<AccountType>,
    pub group_by_party: bool,
    pub in_party_currency: bool,
    pub party: Vec<String>,
    pub party_account: Option<String>,
    pub based_on_payment_terms: bool,
    pub show_future_payments: bool,
    pub show_delivery_notes: bool,
    pub show_sales_person: bool,
    pub show_remarks: bool,
    pub sales_partner: Option<String>,
    pub ignore_accounts: bool,
    pub handle_employee_advances: bool,
    pub for_revaluation_journals: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableSettings {
    pub default_company: String,
    pub company_currency: String,
    pub currency_precision: u32,
    pub party_naming_by: String,
    pub ple_fetch_method: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableState {
    pub filters: ReceivablePayableFilters,
    pub default_company: String,
    pub age_as_on: String,
    pub ranges: Vec<String>,
    pub range_numbers: Vec<usize>,
    pub ple_fetch_method: String,
    pub company_currency: String,
    pub currency_precision: u32,
    pub dr_or_cr: &'static str,
    pub account_type: AccountType,
    pub party_type: Vec<&'static str>,
    pub skip_total_row: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceivablePayableRuntime {
    pub account_type: AccountType,
    pub party_naming_by: String,
    pub ranges: Vec<String>,
    pub range_numbers: Vec<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: &'static str,
    pub options: Option<&'static str>,
    pub width: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PleQueryPlan {
    pub selected_fields: Vec<&'static str>,
    pub delinked_zero: bool,
    pub date_condition: String,
    pub remarks_selection: Option<String>,
    pub order_by: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VoucherBalanceKey(Vec<String>);

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerEntry {
    pub account: String,
    pub voucher_type: String,
    pub voucher_no: String,
    pub against_voucher_type: String,
    pub against_voucher_no: String,
    pub party_type: String,
    pub party: String,
    pub posting_date: String,
    pub due_date: Option<String>,
    pub account_currency: String,
    pub remarks: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub amount: f64,
    pub amount_in_account_currency: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoucherBalanceRow {
    pub voucher_type: String,
    pub voucher_no: String,
    pub party: String,
    pub party_type: String,
    pub party_account: String,
    pub posting_date: String,
    pub due_date: Option<String>,
    pub account_currency: String,
    pub remarks: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub invoiced: f64,
    pub paid: f64,
    pub credit_note: f64,
    pub outstanding: f64,
    pub invoice_grand_total: f64,
    pub invoiced_in_account_currency: f64,
    pub paid_in_account_currency: f64,
    pub credit_note_in_account_currency: f64,
    pub outstanding_in_account_currency: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReceivablePayableAgeingRow {
    pub posting_date: String,
    pub due_date: Option<String>,
    pub bill_date: Option<String>,
    pub outstanding: f64,
    pub age: i64,
    pub range0: f64,
    pub ranges: Vec<f64>,
    pub total_due: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuturePayment {
    pub invoice_no: String,
    pub party: String,
    pub future_date: String,
    pub future_ref: String,
    pub future_amount: f64,
    pub future_amount_in_base_currency: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuturePaymentAllocationRow {
    pub voucher_no: String,
    pub party: String,
    pub outstanding: f64,
    pub remaining_balance: f64,
    pub future_amount: f64,
    pub future_ref: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceDetails {
    pub due_date: Option<String>,
    pub po_no: Option<String>,
    pub bill_no: Option<String>,
    pub bill_date: Option<String>,
    pub sales_team: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceDetailsRow {
    pub voucher_type: String,
    pub voucher_no: String,
    pub due_date: Option<String>,
    pub po_no: Option<String>,
    pub bill_no: Option<String>,
    pub bill_date: Option<String>,
    pub sales_team: Vec<String>,
    pub sales_person: Option<String>,
    pub delivery_notes: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyDetails {
    pub customer_name: Option<String>,
    pub territory: Option<String>,
    pub customer_group: Option<String>,
    pub customer_primary_contact: Option<String>,
    pub default_sales_partner: Option<String>,
    pub supplier_name: Option<String>,
    pub supplier_group: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyDetailsRow {
    pub party: String,
    pub account_currency: String,
    pub currency: Option<String>,
    pub customer_name: Option<String>,
    pub territory: Option<String>,
    pub customer_group: Option<String>,
    pub customer_primary_contact: Option<String>,
    pub default_sales_partner: Option<String>,
    pub supplier_name: Option<String>,
    pub supplier_group: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentTermDetail {
    pub party_account_currency: String,
    pub currency: String,
    pub total_advance: f64,
    pub due_date: String,
    pub payment_term: String,
    pub payment_amount: f64,
    pub base_payment_amount: f64,
    pub description: Option<String>,
    pub paid_amount: f64,
    pub base_paid_amount: f64,
    pub discounted_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentTermRow {
    pub due_date: String,
    pub invoiced: f64,
    pub invoice_grand_total: f64,
    pub payment_term: String,
    pub paid: f64,
    pub credit_note: f64,
    pub outstanding: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentTermAllocationRow {
    pub invoiced: f64,
    pub paid: f64,
    pub credit_note: f64,
    pub payment_terms: Vec<PaymentTermRow>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubtotalDataRow {
    pub currency_values: BTreeMap<String, f64>,
    pub currency: String,
    pub is_empty: bool,
}

impl SubtotalDataRow {
    pub fn with_values<const N: usize>(values: [(&str, f64); N], currency: &str) -> Self {
        Self {
            currency_values: values
                .into_iter()
                .map(|(field, value)| (field.to_string(), value))
                .collect(),
            currency: currency.to_string(),
            is_empty: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            currency_values: BTreeMap::new(),
            currency: String::new(),
            is_empty: true,
        }
    }
}

impl VoucherBalanceKey {
    pub fn with_account(account: &str, voucher_type: &str, voucher_no: &str, party: &str) -> Self {
        Self(vec![
            account.to_string(),
            voucher_type.to_string(),
            voucher_no.to_string(),
            party.to_string(),
        ])
    }

    pub fn without_account(voucher_type: &str, voucher_no: &str, party: &str) -> Self {
        Self(vec![
            voucher_type.to_string(),
            voucher_no.to_string(),
            party.to_string(),
        ])
    }
}

impl ReportColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data",
            options: None,
            width,
        }
    }

    pub fn text(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Text",
            options: None,
            width,
        }
    }

    pub fn date(label: &str, fieldname: &str) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Date",
            options: None,
            width: 90,
        }
    }

    pub fn int(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Int",
            options: None,
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

    pub fn dynamic_link(label: &str, fieldname: &str, options: &'static str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Dynamic Link",
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

pub fn accounts_receivable_args() -> AccountsReceivableArgs {
    AccountsReceivableArgs {
        account_type: AccountType::Receivable,
        naming_by: [
            "Selling Settings".to_string(),
            "cust_master_name".to_string(),
        ],
    }
}

impl ReceivablePayableState {
    pub fn new(
        filters: &ReceivablePayableFilters,
        settings: &ReceivablePayableSettings,
        today: &str,
    ) -> Self {
        let mut filters = filters.clone();
        if filters.report_date.is_none() {
            filters.report_date = Some(today.to_string());
        }
        if filters.range.is_none() {
            filters.range = Some("30, 60, 90, 120".to_string());
        }

        let report_date = filters
            .report_date
            .clone()
            .unwrap_or_else(|| today.to_string());
        let age_as_on = match filters.calculate_ageing_with.as_deref() {
            None | Some("Today Date") => today.to_string(),
            _ => report_date,
        };
        let ranges: Vec<String> = filters
            .range
            .as_deref()
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit()))
            .map(str::to_string)
            .collect();
        let range_numbers = (1..=ranges.len() + 1).collect();

        Self {
            filters,
            default_company: settings.default_company.clone(),
            age_as_on,
            ranges,
            range_numbers,
            ple_fetch_method: settings
                .ple_fetch_method
                .clone()
                .unwrap_or_else(|| "Buffered Cursor".to_string()),
            company_currency: settings.company_currency.clone(),
            currency_precision: settings.currency_precision,
            dr_or_cr: "",
            account_type: AccountType::Receivable,
            party_type: Vec::new(),
            skip_total_row: 0,
        }
    }

    pub fn with_defaults(mut self) -> Self {
        self.filters
            .company
            .get_or_insert_with(|| self.default_company.clone());
        self.account_type = self.filters.account_type.unwrap_or(AccountType::Receivable);
        self.dr_or_cr = match self.account_type {
            AccountType::Receivable => "debit",
            AccountType::Payable => "credit",
        };
        self.party_type = match self.account_type {
            AccountType::Receivable => vec!["Customer"],
            AccountType::Payable => vec!["Supplier"],
        };

        self.skip_total_row = 0;
        if self.filters.group_by_party {
            self.skip_total_row = 1;
        }
        if self.filters.in_party_currency {
            self.skip_total_row = if self.filters.party.len() == 1 { 0 } else { 1 };
        }

        self
    }
}

pub fn get_currency_fields() -> Vec<&'static str> {
    vec![
        "invoiced",
        "paid",
        "credit_note",
        "outstanding",
        "range1",
        "range2",
        "range3",
        "range4",
        "range5",
        "future_amount",
        "remaining_balance",
    ]
}

pub fn build_voucher_dict(ple: &PaymentLedgerEntry) -> VoucherBalanceRow {
    VoucherBalanceRow {
        voucher_type: ple.voucher_type.clone(),
        voucher_no: ple.voucher_no.clone(),
        party: ple.party.clone(),
        party_type: ple.party_type.clone(),
        party_account: ple.account.clone(),
        posting_date: ple.posting_date.clone(),
        due_date: ple.due_date.clone(),
        account_currency: ple.account_currency.clone(),
        remarks: ple.remarks.clone(),
        cost_center: None,
        project: None,
        invoiced: 0.0,
        paid: 0.0,
        credit_note: 0.0,
        outstanding: 0.0,
        invoice_grand_total: 0.0,
        invoiced_in_account_currency: 0.0,
        paid_in_account_currency: 0.0,
        credit_note_in_account_currency: 0.0,
        outstanding_in_account_currency: 0.0,
    }
}

pub fn init_voucher_balance(
    voucher_balance: &mut BTreeMap<VoucherBalanceKey, VoucherBalanceRow>,
    invoices: &mut Vec<String>,
    ple: &PaymentLedgerEntry,
    filters: &ReceivablePayableFilters,
    advance_payment_doctypes: &[String],
) {
    let key = own_voucher_key(ple, filters.ignore_accounts);
    voucher_balance
        .entry(key.clone())
        .or_insert_with(|| build_voucher_dict(ple));

    if is_self_voucher(ple)
        || ((ple.voucher_type == "Payment Entry" || ple.voucher_type == "Journal Entry")
            && advance_payment_doctypes
                .iter()
                .any(|doctype| doctype == &ple.against_voucher_type))
    {
        if let Some(row) = voucher_balance.get_mut(&key) {
            row.cost_center = ple.cost_center.clone();
            row.project = ple.project.clone();
        }
    }

    if is_invoice_type(&ple.voucher_type) && !invoices.contains(&ple.voucher_no) {
        invoices.push(ple.voucher_no.clone());
    }
}

pub fn update_voucher_balance(
    voucher_balance: &mut BTreeMap<VoucherBalanceKey, VoucherBalanceRow>,
    ple: &PaymentLedgerEntry,
    filters: &ReceivablePayableFilters,
    return_entries: &BTreeMap<String, String>,
    _advance_payment_doctypes: &[String],
) {
    let Some(key) = voucher_balance_key_for_update(voucher_balance, ple, filters, return_entries)
    else {
        return;
    };
    let Some(row) = voucher_balance.get_mut(&key) else {
        return;
    };

    row.party_type = ple.party_type.clone();
    let amount = if filters.in_party_currency || filters.party_account.is_some() {
        ple.amount_in_account_currency
    } else {
        ple.amount
    };
    let amount_in_account_currency = ple.amount_in_account_currency;

    if ple.amount > 0.0 {
        if (ple.voucher_type == "Journal Entry" || ple.voucher_type == "Payment Entry")
            && ple.voucher_no != ple.against_voucher_no
        {
            row.paid -= amount;
            row.paid_in_account_currency -= amount_in_account_currency;
        } else {
            row.invoiced += amount;
            row.invoiced_in_account_currency += amount_in_account_currency;
        }
    } else if is_invoice_type(&ple.voucher_type) {
        if row.voucher_no == ple.voucher_no && ple.voucher_no == ple.against_voucher_no {
            row.paid -= amount;
            row.paid_in_account_currency -= amount_in_account_currency;
        } else {
            row.credit_note -= amount;
            row.credit_note_in_account_currency -= amount_in_account_currency;
        }
    } else {
        row.paid -= amount;
        row.paid_in_account_currency -= amount_in_account_currency;
    }
}

pub fn prepare_voucher_balance_rows(
    voucher_balance: &BTreeMap<VoucherBalanceKey, VoucherBalanceRow>,
    filters: &ReceivablePayableFilters,
    currency_precision: u32,
    err_journals: &[String],
) -> Vec<VoucherBalanceRow> {
    let threshold = 1.0 / 10_f64.powi(currency_precision as i32);
    voucher_balance
        .values()
        .filter_map(|row| {
            let mut row = row.clone();
            row.outstanding = round_to_precision(
                row.invoiced - row.paid - row.credit_note,
                currency_precision,
            );
            row.outstanding_in_account_currency = round_to_precision(
                row.invoiced_in_account_currency
                    - row.paid_in_account_currency
                    - row.credit_note_in_account_currency,
                currency_precision,
            );
            row.invoice_grand_total = row.invoiced;

            let must_consider = if filters.for_revaluation_journals {
                row.outstanding.abs() >= threshold
                    || row.outstanding_in_account_currency.abs() >= threshold
            } else {
                row.outstanding.abs() >= threshold
                    && (row.outstanding_in_account_currency.abs() >= threshold
                        || err_journals.contains(&row.voucher_no))
            };

            must_consider.then_some(row)
        })
        .collect()
}

pub fn prepare_ple_query_plan(
    filters: &ReceivablePayableFilters,
    show_remarks: bool,
    remarks_length: Option<u32>,
) -> PleQueryPlan {
    let report_date = filters.report_date.as_deref().unwrap_or_default();
    let date_condition = if filters.show_future_payments {
        format!(
            "posting_date <= '{report_date}' OR (voucher_no = against_voucher_no AND DATE(creation) <= '{report_date}')"
        )
    } else {
        format!("posting_date <= '{report_date}'")
    };
    let remarks_selection = if show_remarks {
        Some(match remarks_length {
            Some(length) => format!("SUBSTRING(remarks, 1, {length})"),
            None => "remarks".to_string(),
        })
    } else {
        None
    };
    let order_by = if filters.group_by_party {
        vec!["party", "posting_date"]
    } else {
        vec!["posting_date", "party"]
    };

    PleQueryPlan {
        selected_fields: vec![
            "name",
            "account",
            "voucher_type",
            "voucher_no",
            "against_voucher_type",
            "against_voucher_no",
            "party_type",
            "cost_center",
            "project",
            "party",
            "posting_date",
            "due_date",
            "account_currency",
            "amount",
            "amount_in_account_currency",
        ],
        delinked_zero: true,
        date_condition,
        remarks_selection,
        order_by,
    }
}

pub fn add_common_filter_conditions(
    filters: &ReceivablePayableFilters,
    accounts: &[String],
) -> Vec<String> {
    let mut conditions = Vec::new();

    if let Some(company) = filters.company.as_deref() {
        conditions.push(format!("company = '{}'", company));
    }
    if let Some(finance_book) = filters.finance_book.as_deref() {
        conditions.push(format!("finance_book = '{}'", finance_book));
    }
    if let Some(party_type) = filters.party_type.as_deref() {
        conditions.push(format!("party_type = '{}'", party_type));
    }
    if !filters.party.is_empty() {
        conditions.push(format!("party IN ({})", quote_join(&filters.party)));
    }
    if let Some(party_account) = filters.party_account.as_deref() {
        conditions.push(format!("account = '{}'", party_account));
    } else if !accounts.is_empty() {
        conditions.push(format!("account IN ({})", quote_join(accounts)));
    }

    conditions
}

pub fn add_customer_filter_conditions(
    filters: &ReceivablePayableFilters,
    customer_group_children: &[String],
    territory_children: &[String],
    sales_invoice_payment_template_names: &[String],
) -> Vec<String> {
    let mut conditions = Vec::new();

    if filters.customer_group.is_some() {
        conditions.push(format!(
            "party IN Customer WHERE customer_group IN ({})",
            quote_join(customer_group_children)
        ));
    }
    if filters.territory.is_some() {
        conditions.push(format!(
            "party IN Customer WHERE territory IN ({})",
            quote_join(territory_children)
        ));
    }
    if let Some(template) = filters.payment_terms_template.as_deref() {
        conditions.push(format!(
            "(party IN Customer WHERE payment_terms = '{}' OR against_voucher_no IN ({}))",
            template,
            quote_join(sales_invoice_payment_template_names)
        ));
    }
    if let Some(sales_partner) = filters.sales_partner.as_deref() {
        conditions.push(format!(
            "party IN Customer WHERE default_sales_partner = '{}'",
            sales_partner
        ));
    }

    conditions.push("party_type != 'Employee'".to_string());
    conditions
}

pub fn add_supplier_filter_conditions(
    filters: &ReceivablePayableFilters,
    purchase_invoice_payment_template_names: &[String],
) -> Vec<String> {
    let mut conditions = Vec::new();

    if let Some(supplier_group) = filters.supplier_group.as_deref() {
        conditions.push(format!(
            "party IN Supplier WHERE supplier_group = '{}'",
            supplier_group
        ));
    }
    if let Some(template) = filters.payment_terms_template.as_deref() {
        conditions.push(format!(
            "(party IN Supplier WHERE payment_terms = '{}' OR against_voucher_no IN ({}))",
            template,
            quote_join(purchase_invoice_payment_template_names)
        ));
    }

    conditions
}

pub fn payment_term_template_filter_conditions(
    dtype: &str,
    filters: &ReceivablePayableFilters,
    party_group_children: &[String],
    cost_center_children: &[String],
) -> Vec<String> {
    let mut conditions = Vec::new();

    if let Some(template) = filters.payment_terms_template.as_deref() {
        conditions.push(format!("payment_terms_template = '{}'", template));
    }
    if let Some(company) = filters.company.as_deref() {
        conditions.push(format!("company = '{}'", company));
    }

    let (party_field, party_group_field, account_field, has_party_group) =
        if dtype == "Purchase Invoice" {
            (
                "supplier",
                "supplier_group",
                "credit_to",
                filters.supplier_group.is_some(),
            )
        } else {
            (
                "customer",
                "customer_group",
                "debit_to",
                filters.customer_group.is_some(),
            )
        };

    if has_party_group {
        conditions.push(format!(
            "{} IN ({})",
            party_group_field,
            quote_join(party_group_children)
        ));
    }
    if !filters.party.is_empty() {
        conditions.push(format!(
            "{} IN ({})",
            party_field,
            quote_join(&filters.party)
        ));
    }
    if !filters.cost_center.is_empty() {
        conditions.push(format!(
            "cost_center IN ({})",
            quote_join(cost_center_children)
        ));
    }
    if let Some(party_account) = filters.party_account.as_deref() {
        conditions.push(format!("{} = '{}'", account_field, party_account));
    }

    conditions
}

pub fn add_project_and_cost_center_conditions(
    filters: &ReceivablePayableFilters,
    cost_center_children: &[String],
) -> Vec<String> {
    let mut conditions = Vec::new();

    if !filters.cost_center.is_empty() {
        conditions.push(format!(
            "cost_center IN ({})",
            quote_join(cost_center_children)
        ));
    }
    if !filters.project.is_empty() {
        conditions.push(format!("project IN ({})", quote_join(&filters.project)));
    }

    conditions
}

pub fn set_ageing(
    row: &mut ReceivablePayableAgeingRow,
    filters: &ReceivablePayableFilters,
    runtime: &ReceivablePayableRuntime,
    age_as_on: &str,
) {
    let entry_date = match filters.ageing_based_on.as_deref() {
        Some("Due Date") => row
            .due_date
            .as_deref()
            .unwrap_or(&row.posting_date)
            .to_string(),
        Some("Supplier Invoice Date") => row.bill_date.clone().unwrap_or_default(),
        _ => row.posting_date.clone(),
    };

    row.range0 = 0.0;
    get_ageing_data(&entry_date, row, runtime, age_as_on);

    if !entry_date.is_empty()
        && !age_as_on.is_empty()
        && parse_date(&entry_date) > parse_date(age_as_on)
    {
        row.range0 = row.outstanding;
        row.ranges = vec![0.0; runtime.range_numbers.len()];
        row.total_due = 0.0;
        return;
    }

    row.total_due = row.ranges.iter().sum();
}

fn get_ageing_data(
    entry_date: &str,
    row: &mut ReceivablePayableAgeingRow,
    runtime: &ReceivablePayableRuntime,
    age_as_on: &str,
) {
    row.ranges = vec![0.0; runtime.range_numbers.len()];
    if age_as_on.is_empty() || entry_date.is_empty() {
        return;
    }

    row.age = parse_date(age_as_on).days_since_epoch() - parse_date(entry_date).days_since_epoch();
    let index = runtime
        .ranges
        .iter()
        .position(|days| row.age <= days.parse::<i64>().unwrap_or_default())
        .unwrap_or(runtime.ranges.len());

    if let Some(range) = row.ranges.get_mut(index) {
        *range = row.outstanding;
    }
}

pub fn allocate_future_payments(
    row: &mut FuturePaymentAllocationRow,
    filters: &ReceivablePayableFilters,
    future_payments: &mut BTreeMap<(String, String), Vec<FuturePayment>>,
) {
    if !filters.show_future_payments {
        return;
    }

    row.remaining_balance = row.outstanding;
    row.future_amount = 0.0;
    let mut future_refs = Vec::new();
    let key = (row.voucher_no.clone(), row.party.clone());
    let Some(payments) = future_payments.get_mut(&key) else {
        return;
    };

    for future in payments {
        let future_amount = if filters.in_party_currency {
            future.future_amount
        } else {
            future.future_amount_in_base_currency
        };

        if row.remaining_balance != 0.0 && future_amount != 0.0 {
            if future_amount > row.outstanding {
                row.future_amount = row.outstanding;
                if filters.in_party_currency {
                    future.future_amount = future_amount - row.outstanding;
                } else {
                    future.future_amount_in_base_currency = future_amount - row.outstanding;
                }
                row.remaining_balance = 0.0;
            } else {
                row.future_amount += future_amount;
                if filters.in_party_currency {
                    future.future_amount = 0.0;
                } else {
                    future.future_amount_in_base_currency = 0.0;
                }
                row.remaining_balance = row.outstanding - row.future_amount;
            }

            future_refs.push(format!("{}/{}", future.future_ref, future.future_date));
        }
    }

    if !future_refs.is_empty() {
        row.future_ref = Some(future_refs.join(", "));
    }
}

pub fn group_future_payments(
    filters: &ReceivablePayableFilters,
    payment_entry_payments: Vec<FuturePayment>,
    journal_entry_payments: Vec<FuturePayment>,
) -> BTreeMap<(String, String), Vec<FuturePayment>> {
    let mut future_payments = BTreeMap::new();
    if !filters.show_future_payments {
        return future_payments;
    }

    for payment in payment_entry_payments
        .into_iter()
        .chain(journal_entry_payments)
    {
        if payment.future_amount != 0.0 && !payment.invoice_no.is_empty() {
            future_payments
                .entry((payment.invoice_no.clone(), payment.party.clone()))
                .or_insert_with(Vec::new)
                .push(payment);
        }
    }

    future_payments
}

pub fn set_invoice_details(
    row: &mut InvoiceDetailsRow,
    filters: &ReceivablePayableFilters,
    invoice_details: &BTreeMap<String, InvoiceDetails>,
    delivery_notes: &BTreeMap<String, Vec<String>>,
) {
    if let Some(details) = invoice_details.get(&row.voucher_no) {
        if row.due_date.is_none() {
            row.due_date = details.due_date.clone();
        }
        row.po_no = details.po_no.clone();
        row.bill_no = details.bill_no.clone();
        row.bill_date = details.bill_date.clone();
        row.sales_team = details.sales_team.clone();
    }

    if row.voucher_type == "Sales Invoice" {
        if filters.show_delivery_notes {
            set_delivery_notes(row, delivery_notes);
        }

        if filters.show_sales_person && !row.sales_team.is_empty() {
            row.sales_person = Some(row.sales_team.join(", "));
            row.sales_team.clear();
        }
    }
}

fn set_delivery_notes(row: &mut InvoiceDetailsRow, delivery_notes: &BTreeMap<String, Vec<String>>) {
    if let Some(notes) = delivery_notes.get(&row.voucher_no) {
        if !notes.is_empty() {
            row.delivery_notes = Some(notes.join(", "));
        }
    }
}

pub fn set_party_details(
    row: &mut PartyDetailsRow,
    filters: &ReceivablePayableFilters,
    company_currency: &str,
    party_details: &BTreeMap<String, PartyDetails>,
) {
    if row.party.is_empty() {
        return;
    }

    if let Some(details) = party_details.get(&row.party) {
        row.customer_name = details.customer_name.clone();
        row.territory = details.territory.clone();
        row.customer_group = details.customer_group.clone();
        row.customer_primary_contact = details.customer_primary_contact.clone();
        row.default_sales_partner = details.default_sales_partner.clone();
        row.supplier_name = details.supplier_name.clone();
        row.supplier_group = details.supplier_group.clone();
    }

    row.currency = if filters.in_party_currency || filters.party_account.is_some() {
        Some(row.account_currency.clone())
    } else {
        Some(company_currency.to_string())
    };
}

pub fn allocate_outstanding_based_on_payment_terms(
    row: &mut PaymentTermAllocationRow,
    filters: &ReceivablePayableFilters,
    payment_terms_details: &[PaymentTermDetail],
    company_currency: &str,
) {
    get_payment_terms(row, filters, payment_terms_details, company_currency);

    for term in &mut row.payment_terms {
        if term.paid == 0.0 {
            allocate_closing_to_term(&mut row.paid, term, "paid");
        }

        if term.outstanding != 0.0 {
            allocate_closing_to_term(&mut row.credit_note, term, "credit_note");
        }
    }

    row.payment_terms
        .sort_by(|left, right| left.due_date.cmp(&right.due_date));
}

fn get_payment_terms(
    row: &mut PaymentTermAllocationRow,
    filters: &ReceivablePayableFilters,
    payment_terms_details: &[PaymentTermDetail],
    company_currency: &str,
) {
    row.payment_terms.clear();
    let Some(first) = payment_terms_details.first() else {
        return;
    };

    row.paid -= first.total_advance;

    for detail in payment_terms_details {
        append_payment_term(row, filters, detail, company_currency);
        if payment_terms_details.len() == 1 && !detail.payment_term.is_empty() {
            break;
        }
    }
}

fn append_payment_term(
    row: &mut PaymentTermAllocationRow,
    filters: &ReceivablePayableFilters,
    detail: &PaymentTermDetail,
    company_currency: &str,
) {
    let mut invoiced = detail.base_payment_amount;
    let mut paid_amount = detail.base_paid_amount;
    let mut in_party_currency = filters.in_party_currency;

    if company_currency == detail.currency && company_currency == detail.party_account_currency {
        in_party_currency = false;
    }
    if in_party_currency && detail.currency != detail.party_account_currency {
        in_party_currency = false;
    }

    if in_party_currency {
        invoiced = detail.payment_amount;
        paid_amount = detail.paid_amount;
    }

    row.payment_terms.push(PaymentTermRow {
        due_date: detail.due_date.clone(),
        invoiced,
        invoice_grand_total: row.invoiced,
        payment_term: detail
            .description
            .clone()
            .filter(|description| !description.is_empty())
            .unwrap_or_else(|| detail.payment_term.clone()),
        paid: paid_amount + detail.discounted_amount,
        credit_note: 0.0,
        outstanding: invoiced - paid_amount - detail.discounted_amount,
    });

    if paid_amount != 0.0 {
        row.paid -= paid_amount + detail.discounted_amount;
    }
}

fn allocate_closing_to_term(row_amount: &mut f64, term: &mut PaymentTermRow, key: &str) {
    let term_amount = match key {
        "paid" => &mut term.paid,
        "credit_note" => &mut term.credit_note,
        _ => return,
    };

    if *row_amount != 0.0 {
        if *row_amount > term.outstanding {
            *term_amount = term.outstanding;
            *row_amount -= term.outstanding;
        } else {
            *term_amount = *row_amount;
            *row_amount = 0.0;
        }
    }
    term.outstanding -= *term_amount;
}

pub fn update_sub_total_row(
    total_row_map: &mut BTreeMap<String, SubtotalDataRow>,
    row: &SubtotalDataRow,
    party: &str,
) {
    if let Some(total_row) = total_row_map.get_mut(party) {
        for field in get_currency_fields() {
            let value = row.currency_values.get(field).copied().unwrap_or_default();
            *total_row
                .currency_values
                .entry(field.to_string())
                .or_default() += value;
        }
        total_row.currency = row.currency.clone();
    }
}

pub fn append_subtotal_row(
    data: &mut Vec<SubtotalDataRow>,
    total_row_map: &mut BTreeMap<String, SubtotalDataRow>,
    party: &str,
) {
    if let Some(sub_total_row) = total_row_map.get(party).cloned() {
        data.push(sub_total_row.clone());
        data.push(SubtotalDataRow::empty());
        update_sub_total_row(total_row_map, &sub_total_row, "Total");
    }
}

pub fn get_columns(
    filters: &ReceivablePayableFilters,
    runtime: &ReceivablePayableRuntime,
) -> Vec<ReportColumn> {
    let mut columns = Vec::new();
    add_column(
        &mut columns,
        "Posting Date",
        Some("posting_date"),
        "Date",
        None,
        120,
    );
    add_column(
        &mut columns,
        "Party Type",
        Some("party_type"),
        "Data",
        None,
        100,
    );
    add_column(
        &mut columns,
        "Party",
        Some("party"),
        "Dynamic Link",
        Some("party_type"),
        180,
    );

    let party_account_label = match runtime.account_type {
        AccountType::Receivable => "Receivable Account",
        AccountType::Payable => "Payable Account",
    };
    add_column(
        &mut columns,
        party_account_label,
        Some("party_account"),
        "Link",
        Some("Account"),
        180,
    );

    if runtime.party_naming_by == "Naming Series" {
        match runtime.account_type {
            AccountType::Receivable => add_column(
                &mut columns,
                "Customer Name",
                Some("customer_name"),
                "Data",
                None,
                120,
            ),
            AccountType::Payable => add_column(
                &mut columns,
                "Supplier Name",
                Some("supplier_name"),
                "Data",
                None,
                120,
            ),
        }
    }

    if runtime.account_type == AccountType::Receivable {
        add_column(
            &mut columns,
            "Customer Contact",
            Some("customer_primary_contact"),
            "Link",
            Some("Contact"),
            120,
        );
    }

    add_column(
        &mut columns,
        "Cost Center",
        Some("cost_center"),
        "Data",
        None,
        120,
    );
    add_column(
        &mut columns,
        "Project",
        Some("project"),
        "Link",
        Some("Project"),
        120,
    );
    add_column(
        &mut columns,
        "Voucher Type",
        Some("voucher_type"),
        "Data",
        None,
        120,
    );
    add_column(
        &mut columns,
        "Voucher No",
        Some("voucher_no"),
        "Dynamic Link",
        Some("voucher_type"),
        180,
    );
    add_column(
        &mut columns,
        "Due Date",
        Some("due_date"),
        "Date",
        None,
        120,
    );

    if runtime.account_type == AccountType::Payable {
        add_column(&mut columns, "Bill No", Some("bill_no"), "Data", None, 120);
        add_column(
            &mut columns,
            "Bill Date",
            Some("bill_date"),
            "Date",
            None,
            120,
        );
    }

    if filters.based_on_payment_terms {
        add_column(
            &mut columns,
            "Payment Term",
            Some("payment_term"),
            "Data",
            None,
            120,
        );
        add_column(
            &mut columns,
            "Invoice Grand Total",
            Some("invoice_grand_total"),
            "Currency",
            None,
            120,
        );
    }

    add_column(
        &mut columns,
        "Invoiced Amount",
        Some("invoiced"),
        "Currency",
        None,
        120,
    );
    add_column(
        &mut columns,
        "Paid Amount",
        Some("paid"),
        "Currency",
        None,
        120,
    );
    let note_label = match runtime.account_type {
        AccountType::Receivable => "Credit Note",
        AccountType::Payable => "Debit Note",
    };
    add_column(
        &mut columns,
        note_label,
        Some("credit_note"),
        "Currency",
        None,
        120,
    );
    add_column(
        &mut columns,
        "Outstanding Amount",
        Some("outstanding"),
        "Currency",
        None,
        120,
    );
    add_column(&mut columns, "Age (Days)", Some("age"), "Int", None, 80);
    setup_ageing_columns(&mut columns, &runtime.ranges);
    add_column(
        &mut columns,
        "Currency",
        Some("currency"),
        "Link",
        Some("Currency"),
        80,
    );

    if filters.show_future_payments {
        add_column(
            &mut columns,
            "Future Payment Ref",
            Some("future_ref"),
            "Data",
            None,
            120,
        );
        add_column(
            &mut columns,
            "Future Payment Amount",
            Some("future_amount"),
            "Currency",
            None,
            120,
        );
        add_column(
            &mut columns,
            "Remaining Balance",
            Some("remaining_balance"),
            "Currency",
            None,
            120,
        );
    }

    if runtime.account_type == AccountType::Receivable {
        add_column(
            &mut columns,
            "Customer LPO",
            Some("po_no"),
            "Data",
            None,
            120,
        );
        if filters.show_delivery_notes {
            add_column(
                &mut columns,
                "Delivery Notes",
                Some("delivery_notes"),
                "Data",
                None,
                120,
            );
        }
        add_column(
            &mut columns,
            "Territory",
            Some("territory"),
            "Link",
            Some("Territory"),
            120,
        );
        add_column(
            &mut columns,
            "Customer Group",
            Some("customer_group"),
            "Link",
            Some("Customer Group"),
            120,
        );
        if filters.show_sales_person {
            add_column(
                &mut columns,
                "Sales Person",
                Some("sales_person"),
                "Data",
                None,
                120,
            );
        }
        if filters.sales_partner.is_some() {
            add_column(
                &mut columns,
                "Sales Partner",
                Some("default_sales_partner"),
                "Data",
                None,
                120,
            );
        }
    }

    if runtime.account_type == AccountType::Payable {
        add_column(
            &mut columns,
            "Supplier Group",
            Some("supplier_group"),
            "Link",
            Some("Supplier Group"),
            120,
        );
    }

    if filters.show_remarks {
        add_column(&mut columns, "Remarks", Some("remarks"), "Text", None, 200);
    }

    columns
}

fn setup_ageing_columns(columns: &mut Vec<ReportColumn>, ranges: &[String]) {
    let mut labels = ranges.to_vec();
    labels.push("Above".to_string());
    let mut previous = 0;

    add_column(columns, "<0", Some("range0"), "Currency", None, 120);
    for (index, current) in labels.iter().enumerate() {
        let label = format!("{previous}-{current}");
        add_column(
            columns,
            &label,
            Some(format!("range{}", index + 1).as_str()),
            "Currency",
            None,
            120,
        );
        if let Ok(value) = current.parse::<i32>() {
            previous = value + 1;
        }
    }
}

fn voucher_balance_key_for_update(
    voucher_balance: &mut BTreeMap<VoucherBalanceKey, VoucherBalanceRow>,
    ple: &PaymentLedgerEntry,
    filters: &ReceivablePayableFilters,
    return_entries: &BTreeMap<String, String>,
) -> Option<VoucherBalanceKey> {
    let mut against_voucher_no = ple.against_voucher_no.as_str();
    if is_invoice_type(&ple.against_voucher_type) {
        if let Some(return_against) = return_entries.get(&ple.against_voucher_no) {
            if !return_against.is_empty() {
                against_voucher_no = return_against;
            }
        }
    }

    let linked_key = if filters.ignore_accounts {
        VoucherBalanceKey::without_account(
            &ple.against_voucher_type,
            against_voucher_no,
            &ple.party,
        )
    } else {
        VoucherBalanceKey::with_account(
            &ple.account,
            &ple.against_voucher_type,
            against_voucher_no,
            &ple.party,
        )
    };
    if voucher_balance.contains_key(&linked_key) {
        return Some(linked_key);
    }

    if ple.against_voucher_type == "Employee Advance" && filters.handle_employee_advances {
        let key = if filters.ignore_accounts {
            VoucherBalanceKey::without_account(
                &ple.against_voucher_type,
                &ple.against_voucher_no,
                &ple.party,
            )
        } else {
            VoucherBalanceKey::with_account(
                &ple.account,
                &ple.against_voucher_type,
                &ple.against_voucher_no,
                &ple.party,
            )
        };
        voucher_balance.entry(key.clone()).or_insert_with(|| {
            let mut row = build_voucher_dict(ple);
            row.voucher_type = ple.against_voucher_type.clone();
            row.voucher_no = ple.against_voucher_no.clone();
            row
        });
        return Some(key);
    }

    let fallback_key = own_voucher_key(ple, filters.ignore_accounts);
    voucher_balance
        .contains_key(&fallback_key)
        .then_some(fallback_key)
}

fn own_voucher_key(ple: &PaymentLedgerEntry, ignore_accounts: bool) -> VoucherBalanceKey {
    if ignore_accounts {
        VoucherBalanceKey::without_account(&ple.voucher_type, &ple.voucher_no, &ple.party)
    } else {
        VoucherBalanceKey::with_account(
            &ple.account,
            &ple.voucher_type,
            &ple.voucher_no,
            &ple.party,
        )
    }
}

fn is_self_voucher(ple: &PaymentLedgerEntry) -> bool {
    ple.voucher_type == ple.against_voucher_type && ple.voucher_no == ple.against_voucher_no
}

fn is_invoice_type(voucher_type: &str) -> bool {
    voucher_type == "Sales Invoice" || voucher_type == "Purchase Invoice"
}

fn add_column(
    columns: &mut Vec<ReportColumn>,
    label: &str,
    fieldname: Option<&str>,
    fieldtype: &'static str,
    options: Option<&'static str>,
    width: u16,
) {
    let fieldname = fieldname
        .map(str::to_string)
        .unwrap_or_else(|| scrub(label));
    let (options, width) = match fieldtype {
        "Currency" => (Some("currency"), width),
        "Date" => (options, 90),
        _ => (options, width),
    };

    columns.push(ReportColumn {
        label: label.to_string(),
        fieldname,
        fieldtype,
        options,
        width,
    });
}

fn scrub(label: &str) -> String {
    label
        .trim()
        .to_ascii_lowercase()
        .replace(|ch: char| !ch.is_ascii_alphanumeric(), "_")
        .trim_matches('_')
        .to_string()
}

fn quote_join(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("'{}'", value))
        .collect::<Vec<_>>()
        .join(", ")
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ErpDate {
    year: i32,
    month: u32,
    day: u32,
}

fn parse_date(value: &str) -> ErpDate {
    let mut parts = value.split('-');
    ErpDate {
        year: parts.next().unwrap().parse().unwrap(),
        month: parts.next().unwrap().parse().unwrap(),
        day: parts.next().unwrap().parse().unwrap(),
    }
}

impl ErpDate {
    fn days_since_epoch(self) -> i64 {
        let year = self.year - i32::from(self.month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year - era * 400;
        let month = self.month as i32;
        let day_of_year =
            (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + self.day as i32 - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        i64::from(era * 146097 + day_of_era - 719468)
    }
}
