use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PurchaseRegisterFilters {
    pub company: String,
    pub supplier: Option<String>,
    pub supplier_group: Option<String>,
    pub include_payments: bool,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub mode_of_payment: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: String,
    pub options: Option<String>,
    pub width: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdditionalColumn {
    pub label: String,
    pub fieldname: String,
    pub fieldtype: String,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseInvoice {
    pub doctype: String,
    pub name: String,
    pub posting_date: String,
    pub credit_to: String,
    pub supplier: String,
    pub supplier_name: String,
    pub tax_id: String,
    pub bill_no: Option<String>,
    pub bill_date: Option<String>,
    pub remarks: String,
    pub base_net_total: f64,
    pub base_grand_total: f64,
    pub base_rounded_total: f64,
    pub outstanding_amount: f64,
    pub mode_of_payment: Option<String>,
    pub is_internal_supplier: bool,
    pub unrealized_profit_loss_account: Option<String>,
    pub represents_company: Option<String>,
    pub company: String,
    pub extra: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseInvoiceItem {
    pub parent: String,
    pub parenttype: String,
    pub docstatus: i32,
    pub expense_account: String,
    pub base_net_amount: f64,
    pub purchase_order: Option<String>,
    pub purchase_receipt: Option<String>,
    pub po_detail: Option<String>,
    pub project: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseTaxCharge {
    pub parent: String,
    pub account_head: String,
    pub tax_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdvanceTaxCharge {
    pub parent: String,
    pub account_head: String,
    pub tax_amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentEntry {
    pub doctype: String,
    pub name: String,
    pub posting_date: String,
    pub credit_to: String,
    pub supplier: String,
    pub supplier_name: String,
    pub base_net_total: f64,
    pub base_grand_total: f64,
    pub mode_of_payment: Option<String>,
    pub project: Option<String>,
    pub remarks: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpeningRow {
    pub account: String,
    pub debit: f64,
    pub credit: f64,
    pub balance: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyDetail {
    pub name: String,
    pub supplier_group: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseRegisterInput {
    pub company_currency: String,
    pub invoices: Vec<PurchaseInvoice>,
    pub invoice_items: Vec<PurchaseInvoiceItem>,
    pub purchase_taxes: Vec<PurchaseTaxCharge>,
    pub advance_taxes: Vec<AdvanceTaxCharge>,
    pub payment_entries: Vec<PaymentEntry>,
    pub opening_row: Option<OpeningRow>,
    pub party_details: Vec<PartyDetail>,
    pub po_purchase_receipts: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountColumns {
    pub expense_columns: Vec<ReportColumn>,
    pub unrealized_profit_loss_account_columns: Vec<ReportColumn>,
    pub tax_columns: Vec<ReportColumn>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountLists {
    pub expense_accounts: Vec<String>,
    pub unrealized_profit_loss_accounts: Vec<String>,
    pub tax_accounts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseRegisterRow {
    pub voucher_no: Option<String>,
    pub posting_date: Option<String>,
    pub values: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PurchaseRegisterReport {
    pub columns: Vec<ReportColumn>,
    pub rows: Vec<PurchaseRegisterRow>,
    pub skip_total_row: bool,
}

impl PurchaseRegisterFilters {
    pub fn without_supplier(mut self) -> Self {
        self.supplier = None;
        self
    }
}

impl ReportColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data".to_string(),
            options: None,
            width,
        }
    }

    pub fn typed(label: &str, fieldname: &str, fieldtype: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: fieldtype.to_string(),
            options: None,
            width,
        }
    }

    pub fn link(label: &str, fieldname: &str, options: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Link".to_string(),
            options: Some(options.to_string()),
            width,
        }
    }

    pub fn currency(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Currency".to_string(),
            options: Some("currency".to_string()),
            width,
        }
    }
}

impl AdditionalColumn {
    pub fn data(label: &str, fieldname: &str, width: u16) -> Self {
        Self {
            label: label.to_string(),
            fieldname: fieldname.to_string(),
            fieldtype: "Data".to_string(),
            width,
        }
    }
}

impl From<&AdditionalColumn> for ReportColumn {
    fn from(column: &AdditionalColumn) -> Self {
        Self {
            label: column.label.clone(),
            fieldname: column.fieldname.clone(),
            fieldtype: column.fieldtype.clone(),
            options: None,
            width: column.width,
        }
    }
}

impl PurchaseTaxCharge {
    pub fn new(parent: &str, account_head: &str, tax_amount: f64) -> Self {
        Self {
            parent: parent.to_string(),
            account_head: account_head.to_string(),
            tax_amount,
        }
    }
}

impl AdvanceTaxCharge {
    pub fn new(parent: &str, account_head: &str, tax_amount: f64) -> Self {
        Self {
            parent: parent.to_string(),
            account_head: account_head.to_string(),
            tax_amount,
        }
    }
}

impl PaymentEntry {
    pub fn new(
        doctype: &str,
        name: &str,
        posting_date: &str,
        credit_to: &str,
        supplier: &str,
        supplier_name: &str,
        base_net_total: f64,
        base_grand_total: f64,
    ) -> Self {
        Self {
            doctype: doctype.to_string(),
            name: name.to_string(),
            posting_date: posting_date.to_string(),
            credit_to: credit_to.to_string(),
            supplier: supplier.to_string(),
            supplier_name: supplier_name.to_string(),
            base_net_total,
            base_grand_total,
            mode_of_payment: None,
            project: None,
            remarks: String::new(),
        }
    }
}

impl PartyDetail {
    pub fn supplier(name: &str, supplier_group: &str, tax_id: &str) -> Self {
        Self {
            name: name.to_string(),
            supplier_group: Some(supplier_group.to_string()),
            tax_id: Some(tax_id.to_string()),
        }
    }
}

pub fn execute(
    filters: PurchaseRegisterFilters,
    input: PurchaseRegisterInput,
    additional_table_columns: Vec<AdditionalColumn>,
) -> Result<PurchaseRegisterReport, String> {
    if filters.include_payments && filters.supplier.is_none() {
        return Err("Please select a supplier for fetching payments.".to_string());
    }

    let mut invoice_list = get_invoices(&filters, &input);
    let mut payment_rows = Vec::new();
    if filters.include_payments {
        payment_rows = get_payments(&filters, &input);
    }

    let (columns, expense_accounts, tax_accounts, unrealized_accounts) =
        get_columns(&input, &additional_table_columns, filters.include_payments);
    let invoice_expense_map = get_invoice_expense_map(&invoice_list, &input.invoice_items);
    let internal_invoice_map = get_internal_invoice_map(&invoice_list);
    let (invoice_expense_map, invoice_tax_map) = get_invoice_tax_map(
        &invoice_list,
        invoice_expense_map,
        &expense_accounts,
        &input,
        filters.include_payments,
    );
    let invoice_po_pr_map = get_invoice_po_pr_map(
        &invoice_list,
        &input.invoice_items,
        &input.po_purchase_receipts,
    );
    let supplier_details = input
        .party_details
        .iter()
        .map(|detail| (detail.name.clone(), detail.clone()))
        .collect::<BTreeMap<_, _>>();

    invoice_list.sort_by(|left, right| left.posting_date.cmp(&right.posting_date));
    let mut rows = Vec::new();

    if filters.include_payments {
        if let Some(opening) = &input.opening_row {
            rows.push(opening_report_row(opening));
        }
    }

    for inv in &invoice_list {
        rows.push(invoice_report_row(
            inv,
            &additional_table_columns,
            &expense_accounts,
            &unrealized_accounts,
            &tax_accounts,
            &invoice_expense_map,
            &invoice_tax_map,
            &internal_invoice_map,
            &invoice_po_pr_map,
            &supplier_details,
            &input.company_currency,
        ));
    }

    rows.extend(payment_rows.iter().map(payment_report_row));

    if filters.include_payments {
        let mut running_balance = input.opening_row.as_ref().map_or(0.0, |row| row.balance);
        for row in rows.iter_mut().skip(1) {
            let debit = number_value(&row.values, "debit");
            let credit = number_value(&row.values, "credit");
            running_balance += debit - credit;
            row.values
                .insert("balance".to_string(), json!(running_balance));
        }
    }

    Ok(PurchaseRegisterReport {
        columns,
        rows,
        skip_total_row: filters.include_payments,
    })
}

pub fn get_columns(
    input: &PurchaseRegisterInput,
    additional_table_columns: &[AdditionalColumn],
    include_payments: bool,
) -> (Vec<ReportColumn>, Vec<String>, Vec<String>, Vec<String>) {
    let mut columns = vec![
        ReportColumn::data("Voucher Type", "voucher_type", 120),
        ReportColumn {
            label: "Voucher".to_string(),
            fieldname: "voucher_no".to_string(),
            fieldtype: "Dynamic Link".to_string(),
            options: Some("voucher_type".to_string()),
            width: 120,
        },
        ReportColumn::typed("Posting Date", "posting_date", "Date", 80),
        ReportColumn::link("Supplier", "supplier_id", "Supplier", 120),
        ReportColumn::data("Supplier Name", "supplier_name", 120),
    ];

    if !include_payments {
        columns.extend(additional_table_columns.iter().map(ReportColumn::from));
        columns.extend(vec![
            ReportColumn::link("Supplier Group", "supplier_group", "Supplier Group", 120),
            ReportColumn::data("Tax Id", "tax_id", 80),
            ReportColumn::link("Payable Account", "payable_account", "Account", 100),
            ReportColumn::data("Mode Of Payment", "mode_of_payment", 120),
            ReportColumn::link("Project", "project", "Project", 80),
            ReportColumn::data("Bill No", "bill_no", 120),
            ReportColumn::typed("Bill Date", "bill_date", "Date", 80),
            ReportColumn::link("Purchase Order", "purchase_order", "Purchase Order", 100),
            ReportColumn::link(
                "Purchase Receipt",
                "purchase_receipt",
                "Purchase Receipt",
                100,
            ),
            ReportColumn::data("Currency", "currency", 80),
        ]);
    } else {
        columns.extend(vec![
            ReportColumn::link("Payable Account", "payable_account", "Account", 120),
            ReportColumn::typed("Debit", "debit", "Currency", 120),
            ReportColumn::typed("Credit", "credit", "Currency", 120),
            ReportColumn::typed("Balance", "balance", "Currency", 120),
        ]);
    }

    let (account_columns, accounts) = get_account_columns(input, include_payments);
    columns.extend(account_columns.expense_columns.clone());
    columns.extend(
        account_columns
            .unrealized_profit_loss_account_columns
            .clone(),
    );
    columns.push(ReportColumn::currency("Net Total", "net_total", 120));
    columns.extend(account_columns.tax_columns.clone());
    columns.push(ReportColumn::currency("Total Tax", "total_tax", 120));
    if !include_payments {
        columns.extend(vec![
            ReportColumn::currency("Grand Total", "grand_total", 120),
            ReportColumn::currency("Rounded Total", "rounded_total", 120),
            ReportColumn::currency("Outstanding Amount", "outstanding_amount", 120),
        ]);
    }
    columns.push(ReportColumn::data("Remarks", "remarks", 120));

    (
        columns,
        accounts.expense_accounts,
        accounts.tax_accounts,
        accounts.unrealized_profit_loss_accounts,
    )
}

pub fn get_account_columns(
    input: &PurchaseRegisterInput,
    include_payments: bool,
) -> (AccountColumns, AccountLists) {
    let invoice_names = invoice_names(&input.invoices);
    let expense_accounts = sorted_set(
        input
            .invoice_items
            .iter()
            .filter(|item| {
                item.docstatus == 1
                    && item.parenttype == "Purchase Invoice"
                    && !item.expense_account.is_empty()
                    && invoice_names.contains(&item.parent)
            })
            .map(|item| item.expense_account.clone()),
    );
    let mut tax_accounts = sorted_set(
        input
            .purchase_taxes
            .iter()
            .filter(|tax| invoice_names.contains(&tax.parent))
            .map(|tax| tax.account_head.clone()),
    );
    if include_payments {
        tax_accounts = sorted_set(
            tax_accounts.into_iter().chain(
                input
                    .advance_taxes
                    .iter()
                    .map(|tax| tax.account_head.clone()),
            ),
        );
    }
    let unrealized_profit_loss_accounts = sorted_set(
        input
            .invoices
            .iter()
            .filter_map(|inv| inv.unrealized_profit_loss_account.clone()),
    );

    let expense_columns = expense_accounts
        .iter()
        .map(|account| ReportColumn::currency(account, &scrub(account), 120))
        .collect::<Vec<_>>();
    let tax_columns = tax_accounts
        .iter()
        .filter(|account| !expense_accounts.contains(account))
        .map(|account| ReportColumn::currency(account, &scrub(account), 120))
        .collect::<Vec<_>>();
    let unrealized_profit_loss_account_columns = unrealized_profit_loss_accounts
        .iter()
        .map(|account| ReportColumn::currency(account, &scrub(account), 120))
        .collect::<Vec<_>>();

    (
        AccountColumns {
            expense_columns,
            unrealized_profit_loss_account_columns,
            tax_columns,
        },
        AccountLists {
            expense_accounts,
            unrealized_profit_loss_accounts,
            tax_accounts,
        },
    )
}

pub fn get_invoices(
    filters: &PurchaseRegisterFilters,
    input: &PurchaseRegisterInput,
) -> Vec<PurchaseInvoice> {
    input
        .invoices
        .iter()
        .filter(|inv| inv.company == filters.company)
        .filter(|inv| {
            filters
                .supplier
                .as_ref()
                .is_none_or(|supplier| inv.supplier == *supplier)
        })
        .filter(|inv| {
            filters
                .from_date
                .as_ref()
                .is_none_or(|from_date| inv.posting_date >= *from_date)
        })
        .filter(|inv| {
            filters
                .to_date
                .as_ref()
                .is_none_or(|to_date| inv.posting_date <= *to_date)
        })
        .filter(|inv| {
            filters
                .mode_of_payment
                .as_ref()
                .is_none_or(|mode| inv.mode_of_payment.as_ref() == Some(mode))
        })
        .cloned()
        .collect()
}

pub fn get_payments(
    filters: &PurchaseRegisterFilters,
    input: &PurchaseRegisterInput,
) -> Vec<PaymentEntry> {
    input
        .payment_entries
        .iter()
        .filter(|payment| {
            filters
                .supplier
                .as_ref()
                .is_none_or(|supplier| payment.supplier == *supplier)
        })
        .filter(|payment| {
            filters
                .from_date
                .as_ref()
                .is_none_or(|from_date| payment.posting_date >= *from_date)
        })
        .filter(|payment| {
            filters
                .to_date
                .as_ref()
                .is_none_or(|to_date| payment.posting_date <= *to_date)
        })
        .cloned()
        .collect()
}

pub fn get_invoice_expense_map(
    invoice_list: &[PurchaseInvoice],
    invoice_items: &[PurchaseInvoiceItem],
) -> BTreeMap<String, BTreeMap<String, f64>> {
    let invoice_names = invoice_names(invoice_list);
    let mut map = BTreeMap::<String, BTreeMap<String, f64>>::new();
    for item in invoice_items.iter().filter(|item| {
        item.parenttype == "Purchase Invoice" && invoice_names.contains(&item.parent)
    }) {
        *map.entry(item.parent.clone())
            .or_default()
            .entry(item.expense_account.clone())
            .or_default() += item.base_net_amount;
    }
    map
}

pub fn get_internal_invoice_map(
    invoice_list: &[PurchaseInvoice],
) -> BTreeMap<(String, String), f64> {
    let mut map = BTreeMap::new();
    for inv in invoice_list.iter().filter(|inv| {
        inv.is_internal_supplier && inv.represents_company.as_deref() == Some(inv.company.as_str())
    }) {
        if let Some(account) = &inv.unrealized_profit_loss_account {
            map.insert((inv.name.clone(), account.clone()), inv.base_net_total);
        }
    }
    map
}

pub fn get_invoice_tax_map(
    invoice_list: &[PurchaseInvoice],
    mut invoice_expense_map: BTreeMap<String, BTreeMap<String, f64>>,
    expense_accounts: &[String],
    input: &PurchaseRegisterInput,
    include_payments: bool,
) -> (
    BTreeMap<String, BTreeMap<String, f64>>,
    BTreeMap<String, BTreeMap<String, f64>>,
) {
    let invoice_names = invoice_names(invoice_list);
    let mut tax_map = BTreeMap::<String, BTreeMap<String, f64>>::new();
    for tax in input
        .purchase_taxes
        .iter()
        .filter(|tax| invoice_names.contains(&tax.parent))
    {
        if expense_accounts.contains(&tax.account_head) {
            *invoice_expense_map
                .entry(tax.parent.clone())
                .or_default()
                .entry(tax.account_head.clone())
                .or_default() += tax.tax_amount;
        } else {
            *tax_map
                .entry(tax.parent.clone())
                .or_default()
                .entry(tax.account_head.clone())
                .or_default() += tax.tax_amount;
        }
    }

    if include_payments {
        for tax in &input.advance_taxes {
            *tax_map
                .entry(tax.parent.clone())
                .or_default()
                .entry(tax.account_head.clone())
                .or_default() += tax.tax_amount;
        }
    }

    (invoice_expense_map, tax_map)
}

pub fn get_invoice_po_pr_map(
    invoice_list: &[PurchaseInvoice],
    invoice_items: &[PurchaseInvoiceItem],
    po_purchase_receipts: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, BTreeMap<String, Vec<String>>> {
    let invoice_names = invoice_names(invoice_list);
    let mut map = BTreeMap::<String, BTreeMap<String, Vec<String>>>::new();
    for item in invoice_items.iter().filter(|item| {
        item.parenttype == "Purchase Invoice" && invoice_names.contains(&item.parent)
    }) {
        if let Some(purchase_order) = &item.purchase_order {
            map.entry(item.parent.clone())
                .or_default()
                .entry("purchase_order".to_string())
                .or_default()
                .push(purchase_order.clone());
        }

        let receipt_list = if let Some(purchase_receipt) = &item.purchase_receipt {
            vec![purchase_receipt.clone()]
        } else if let Some(po_detail) = &item.po_detail {
            po_purchase_receipts
                .get(po_detail)
                .cloned()
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        if !receipt_list.is_empty() {
            map.entry(item.parent.clone())
                .or_default()
                .entry("purchase_receipt".to_string())
                .or_default()
                .extend(receipt_list);
        }

        if let Some(project) = &item.project {
            map.entry(item.parent.clone())
                .or_default()
                .entry("project".to_string())
                .or_default()
                .push(project.clone());
        }
    }
    dedup_nested(&mut map);
    map
}

fn invoice_report_row(
    inv: &PurchaseInvoice,
    additional_table_columns: &[AdditionalColumn],
    expense_accounts: &[String],
    unrealized_accounts: &[String],
    tax_accounts: &[String],
    expense_map: &BTreeMap<String, BTreeMap<String, f64>>,
    tax_map: &BTreeMap<String, BTreeMap<String, f64>>,
    internal_map: &BTreeMap<(String, String), f64>,
    po_pr_map: &BTreeMap<String, BTreeMap<String, Vec<String>>>,
    supplier_details: &BTreeMap<String, PartyDetail>,
    company_currency: &str,
) -> PurchaseRegisterRow {
    let mut values = BTreeMap::new();
    values.insert("voucher_type".to_string(), json!(inv.doctype));
    values.insert("voucher_no".to_string(), json!(inv.name));
    values.insert("posting_date".to_string(), json!(inv.posting_date));
    values.insert("supplier_id".to_string(), json!(inv.supplier));
    values.insert("supplier_name".to_string(), json!(inv.supplier_name));
    for column in additional_table_columns {
        values.insert(
            column.fieldname.clone(),
            inv.extra
                .get(&column.fieldname)
                .cloned()
                .unwrap_or(Value::Null),
        );
    }
    let party = supplier_details.get(&inv.supplier);
    values.insert(
        "supplier_group".to_string(),
        json!(party
            .and_then(|detail| detail.supplier_group.clone())
            .unwrap_or_default()),
    );
    values.insert(
        "tax_id".to_string(),
        json!(party
            .and_then(|detail| detail.tax_id.clone())
            .unwrap_or_else(|| inv.tax_id.clone())),
    );
    values.insert("payable_account".to_string(), json!(inv.credit_to));
    values.insert(
        "mode_of_payment".to_string(),
        json!(inv.mode_of_payment.clone().unwrap_or_default()),
    );
    values.insert(
        "bill_no".to_string(),
        json!(inv.bill_no.clone().unwrap_or_default()),
    );
    values.insert(
        "bill_date".to_string(),
        json!(inv.bill_date.clone().unwrap_or_default()),
    );
    values.insert("remarks".to_string(), json!(inv.remarks));
    values.insert("currency".to_string(), json!(company_currency));

    let links = po_pr_map.get(&inv.name);
    values.insert(
        "purchase_order".to_string(),
        json!(join_nested(links, "purchase_order")),
    );
    values.insert(
        "purchase_receipt".to_string(),
        json!(join_nested(links, "purchase_receipt")),
    );
    values.insert("project".to_string(), json!(join_nested(links, "project")));

    let mut base_net_total = 0.0;
    for account in expense_accounts {
        let mut amount = *expense_map
            .get(&inv.name)
            .and_then(|accounts| accounts.get(account))
            .unwrap_or(&0.0);
        if inv.is_internal_supplier
            && inv.company == inv.represents_company.as_deref().unwrap_or_default()
        {
            amount = 0.0;
        }
        base_net_total += amount;
        values.insert(scrub(account), json!(amount));
    }

    for account in unrealized_accounts {
        let amount = internal_map
            .get(&(inv.name.clone(), account.clone()))
            .copied()
            .unwrap_or(0.0);
        values.insert(scrub(account), json!(amount));
    }

    values.insert(
        "net_total".to_string(),
        json!(if base_net_total != 0.0 {
            base_net_total
        } else {
            inv.base_net_total
        }),
    );

    let mut total_tax = 0.0;
    for account in tax_accounts
        .iter()
        .filter(|account| !expense_accounts.contains(account))
    {
        let amount = *tax_map
            .get(&inv.name)
            .and_then(|accounts| accounts.get(account))
            .unwrap_or(&0.0);
        total_tax += amount;
        values.insert(scrub(account), json!(amount));
    }

    values.insert("total_tax".to_string(), json!(total_tax));
    values.insert("grand_total".to_string(), json!(inv.base_grand_total));
    values.insert("rounded_total".to_string(), json!(inv.base_rounded_total));
    values.insert(
        "outstanding_amount".to_string(),
        json!(inv.outstanding_amount),
    );
    values.insert("debit".to_string(), json!(inv.base_grand_total));
    values.insert("credit".to_string(), json!(0.0));

    PurchaseRegisterRow {
        voucher_no: Some(inv.name.clone()),
        posting_date: Some(inv.posting_date.clone()),
        values,
    }
}

fn payment_report_row(payment: &PaymentEntry) -> PurchaseRegisterRow {
    let mut values = BTreeMap::new();
    values.insert("voucher_type".to_string(), json!(payment.doctype));
    values.insert("voucher_no".to_string(), json!(payment.name));
    values.insert("posting_date".to_string(), json!(payment.posting_date));
    values.insert("supplier_id".to_string(), json!(payment.supplier));
    values.insert("supplier_name".to_string(), json!(payment.supplier_name));
    values.insert("payable_account".to_string(), json!(payment.credit_to));
    values.insert(
        "mode_of_payment".to_string(),
        json!(payment.mode_of_payment.clone().unwrap_or_default()),
    );
    values.insert(
        "project".to_string(),
        json!(payment.project.clone().unwrap_or_default()),
    );
    values.insert("remarks".to_string(), json!(payment.remarks));
    values.insert("net_total".to_string(), json!(payment.base_net_total));
    values.insert("debit".to_string(), json!(0.0));
    values.insert("credit".to_string(), json!(payment.base_grand_total));
    PurchaseRegisterRow {
        voucher_no: Some(payment.name.clone()),
        posting_date: Some(payment.posting_date.clone()),
        values,
    }
}

fn opening_report_row(opening: &OpeningRow) -> PurchaseRegisterRow {
    let mut values = BTreeMap::new();
    values.insert("payable_account".to_string(), json!(opening.account));
    values.insert("debit".to_string(), json!(opening.debit));
    values.insert("credit".to_string(), json!(opening.credit));
    values.insert("balance".to_string(), json!(opening.balance));
    PurchaseRegisterRow {
        voucher_no: None,
        posting_date: None,
        values,
    }
}

fn invoice_names(invoices: &[PurchaseInvoice]) -> BTreeSet<String> {
    invoices
        .iter()
        .map(|invoice| invoice.name.clone())
        .collect()
}

fn sorted_set(values: impl Iterator<Item = String>) -> Vec<String> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}

fn scrub(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn dedup_nested(map: &mut BTreeMap<String, BTreeMap<String, Vec<String>>>) {
    for fields in map.values_mut() {
        for values in fields.values_mut() {
            values.sort();
            values.dedup();
        }
    }
}

fn join_nested(map: Option<&BTreeMap<String, Vec<String>>>, fieldname: &str) -> String {
    map.and_then(|fields| fields.get(fieldname))
        .map(|values| values.join(", "))
        .unwrap_or_default()
}

fn number_value(values: &BTreeMap<String, Value>, fieldname: &str) -> f64 {
    values.get(fieldname).and_then(Value::as_f64).unwrap_or(0.0)
}
