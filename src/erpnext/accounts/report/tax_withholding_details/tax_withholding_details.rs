pub const PARTY_TYPES: [&str; 2] = ["Customer", "Supplier"];
pub const DOCUMENT_TYPES: [&str; 4] = [
    "Purchase Invoice",
    "Sales Invoice",
    "Payment Entry",
    "Journal Entry",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaxWithholdingError {
    MissingDateRange,
    FromDateAfterToDate,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaxWithholdingFilters {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub company: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaxWithholdingColumn {
    pub label: String,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaxWithholdingEntry {
    pub party_type: String,
    pub party: String,
    pub tax_id: String,
    pub tax_withholding_category: String,
    pub total_amount: f64,
    pub rate: f64,
    pub tax_amount: f64,
    pub transaction_type: String,
    pub ref_no: String,
    pub taxable_date: Option<String>,
    pub withholding_doctype: String,
    pub withholding_name: String,
    pub transaction_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyInfo {
    pub party_type: String,
    pub name: String,
    pub party_entity_type: String,
    pub party_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocInfo {
    pub doctype: String,
    pub name: String,
    pub grand_total: Option<f64>,
    pub base_total: Option<f64>,
    pub supplier_invoice_no: Option<String>,
    pub supplier_invoice_date: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaxWithholdingRow {
    pub party_type: String,
    pub party: String,
    pub tax_id: String,
    pub tax_withholding_category: String,
    pub total_amount: f64,
    pub rate: f64,
    pub tax_amount: f64,
    pub transaction_type: String,
    pub ref_no: String,
    pub taxable_date: Option<String>,
    pub withholding_doctype: String,
    pub withholding_name: String,
    pub transaction_date: Option<String>,
    pub party_entity_type: Option<String>,
    pub party_name: Option<String>,
    pub grand_total: Option<f64>,
    pub base_total: Option<f64>,
    pub supplier_invoice_no: Option<String>,
    pub supplier_invoice_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaxWithholdingEntriesQueryPlan {
    pub source_doctype: &'static str,
    pub selected_fields: Vec<&'static str>,
    pub filters: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartyQueryPlan {
    pub party_type: &'static str,
    pub names: Vec<String>,
    pub fields: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocInfoQueryPlan {
    pub doctype: &'static str,
    pub names: Vec<String>,
    pub fields: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaxWithholdingDetailsReport {
    pub filters: TaxWithholdingFilters,
    pub columns: Vec<TaxWithholdingColumn>,
    pub rows: Vec<TaxWithholdingRow>,
}

impl TaxWithholdingFilters {
    pub fn new(from_date: impl Into<String>, to_date: impl Into<String>) -> Self {
        Self {
            from_date: Some(from_date.into()),
            to_date: Some(to_date.into()),
            company: None,
            party_type: None,
            party: None,
        }
    }
}

impl TaxWithholdingColumn {
    pub fn data(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Data",
            options: "",
            width,
        }
    }

    pub fn date(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Date",
            options: "",
            width,
        }
    }

    pub fn link(
        label: impl Into<String>,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Link",
            options,
            width,
        }
    }

    pub fn dynamic_link(
        label: impl Into<String>,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Dynamic Link",
            options,
            width,
        }
    }

    pub fn percent(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Percent",
            options: "",
            width,
        }
    }

    pub fn currency(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Currency",
            options: "",
            width,
        }
    }
}

impl TaxWithholdingEntry {
    pub fn new(
        party_type: impl Into<String>,
        party: impl Into<String>,
        tax_withholding_category: impl Into<String>,
        ref_no: impl Into<String>,
    ) -> Self {
        Self {
            party_type: party_type.into(),
            party: party.into(),
            tax_id: String::new(),
            tax_withholding_category: tax_withholding_category.into(),
            total_amount: 0.0,
            rate: 0.0,
            tax_amount: 0.0,
            transaction_type: String::new(),
            ref_no: ref_no.into(),
            taxable_date: None,
            withholding_doctype: String::new(),
            withholding_name: String::new(),
            transaction_date: None,
        }
    }
}

impl PartyInfo {
    pub fn new(
        party_type: impl Into<String>,
        name: impl Into<String>,
        party_entity_type: impl Into<String>,
        party_name: impl Into<String>,
    ) -> Self {
        Self {
            party_type: party_type.into(),
            name: name.into(),
            party_entity_type: party_entity_type.into(),
            party_name: party_name.into(),
        }
    }
}

impl DocInfo {
    pub fn purchase_invoice(
        name: impl Into<String>,
        grand_total: f64,
        base_total: f64,
        supplier_invoice_no: impl Into<String>,
        supplier_invoice_date: impl Into<String>,
    ) -> Self {
        Self {
            doctype: "Purchase Invoice".to_string(),
            name: name.into(),
            grand_total: Some(grand_total),
            base_total: Some(base_total),
            supplier_invoice_no: Some(supplier_invoice_no.into()),
            supplier_invoice_date: Some(supplier_invoice_date.into()),
        }
    }

    pub fn sales_invoice(name: impl Into<String>, grand_total: f64, base_total: f64) -> Self {
        Self {
            doctype: "Sales Invoice".to_string(),
            name: name.into(),
            grand_total: Some(grand_total),
            base_total: Some(base_total),
            supplier_invoice_no: None,
            supplier_invoice_date: None,
        }
    }

    pub fn payment_entry(name: impl Into<String>, grand_total: f64, base_total: f64) -> Self {
        Self {
            doctype: "Payment Entry".to_string(),
            name: name.into(),
            grand_total: Some(grand_total),
            base_total: Some(base_total),
            supplier_invoice_no: None,
            supplier_invoice_date: None,
        }
    }

    pub fn journal_entry(name: impl Into<String>, total_debit: f64) -> Self {
        Self {
            doctype: "Journal Entry".to_string(),
            name: name.into(),
            grand_total: Some(total_debit),
            base_total: Some(total_debit),
            supplier_invoice_no: None,
            supplier_invoice_date: None,
        }
    }
}

pub fn execute(
    filters: TaxWithholdingFilters,
    entries: Vec<TaxWithholdingEntry>,
    docs: Vec<DocInfo>,
    parties: Vec<PartyInfo>,
) -> Result<TaxWithholdingDetailsReport, TaxWithholdingError> {
    validate_filters(&filters)?;
    let rows = get_data(entries, docs, parties);
    Ok(TaxWithholdingDetailsReport {
        columns: get_columns(&filters),
        filters,
        rows,
    })
}

pub fn validate_filters(filters: &TaxWithholdingFilters) -> Result<(), TaxWithholdingError> {
    let Some(from_date) = filters.from_date.as_deref() else {
        return Err(TaxWithholdingError::MissingDateRange);
    };
    let Some(to_date) = filters.to_date.as_deref() else {
        return Err(TaxWithholdingError::MissingDateRange);
    };
    if from_date > to_date {
        return Err(TaxWithholdingError::FromDateAfterToDate);
    }
    Ok(())
}

pub fn get_data(
    entries: Vec<TaxWithholdingEntry>,
    docs: Vec<DocInfo>,
    parties: Vec<PartyInfo>,
) -> Vec<TaxWithholdingRow> {
    if entries.is_empty() {
        return Vec::new();
    }
    let doc_info = fetch_additional_doc_info(&entries, &docs);
    let party_details = fetch_party_details(&entries, &parties);
    build_rows(entries, &doc_info, &party_details)
}

pub fn build_rows(
    entries: Vec<TaxWithholdingEntry>,
    doc_info: &[DocInfo],
    party_details: &[PartyInfo],
) -> Vec<TaxWithholdingRow> {
    let mut rows = entries
        .into_iter()
        .map(|entry| {
            let doc = if entry.ref_no.is_empty() {
                None
            } else {
                doc_info
                    .iter()
                    .find(|doc| doc.doctype == entry.transaction_type && doc.name == entry.ref_no)
            };
            let party = party_details
                .iter()
                .find(|party| party.party_type == entry.party_type && party.name == entry.party);

            TaxWithholdingRow {
                party_type: entry.party_type,
                party: entry.party,
                tax_id: entry.tax_id,
                tax_withholding_category: entry.tax_withholding_category,
                total_amount: entry.total_amount,
                rate: entry.rate,
                tax_amount: entry.tax_amount,
                transaction_type: entry.transaction_type,
                ref_no: entry.ref_no,
                taxable_date: entry.taxable_date,
                withholding_doctype: entry.withholding_doctype,
                withholding_name: entry.withholding_name,
                transaction_date: entry.transaction_date,
                party_entity_type: party.map(|party| party.party_entity_type.clone()),
                party_name: party.map(|party| party.party_name.clone()),
                grand_total: doc.and_then(|doc| doc.grand_total),
                base_total: doc.and_then(|doc| doc.base_total),
                supplier_invoice_no: doc.and_then(|doc| doc.supplier_invoice_no.clone()),
                supplier_invoice_date: doc.and_then(|doc| doc.supplier_invoice_date.clone()),
            }
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| sort_key(left).cmp(&sort_key(right)));
    rows
}

pub fn get_entries_query_plan(filters: &TaxWithholdingFilters) -> TaxWithholdingEntriesQueryPlan {
    let mut query_filters = vec![
        "docstatus = 1",
        "withholding_date >= filters.from_date",
        "withholding_date <= filters.to_date",
        "IfNull(withholding_name, '') != ''",
        "status != 'Duplicate'",
    ];
    if filters.company.is_some() {
        query_filters.push("company = filters.company");
    }
    if filters.party_type.is_some() {
        query_filters.push("party_type = filters.party_type");
    }
    if filters.party.is_some() {
        query_filters.push("party = filters.party");
    }

    TaxWithholdingEntriesQueryPlan {
        source_doctype: "Tax Withholding Entry",
        selected_fields: vec![
            "party_type",
            "party",
            "IfNull(tax_id, '') as tax_id",
            "tax_withholding_category",
            "taxable_amount as total_amount",
            "tax_rate as rate",
            "withholding_amount as tax_amount",
            "IfNull(taxable_doctype, '') as transaction_type",
            "IfNull(taxable_name, '') as ref_no",
            "taxable_date",
            "IfNull(withholding_doctype, '') as withholding_doctype",
            "IfNull(withholding_name, '') as withholding_name",
            "withholding_date as transaction_date",
        ],
        filters: query_filters,
    }
}

pub fn fetch_party_details(
    entries: &[TaxWithholdingEntry],
    parties: &[PartyInfo],
) -> Vec<PartyInfo> {
    let mut result = Vec::new();
    for party_type in PARTY_TYPES {
        let mut names = Vec::<&str>::new();
        for entry in entries {
            if entry.party_type == party_type
                && !entry.party.is_empty()
                && !names.iter().any(|name| *name == entry.party)
            {
                names.push(entry.party.as_str());
            }
        }
        for name in names {
            if let Some(party) = parties
                .iter()
                .find(|party| party.party_type == party_type && party.name == name)
            {
                result.push(party.clone());
            }
        }
    }
    result
}

pub fn get_party_query(party_type: &str, names: Vec<String>) -> Option<PartyQueryPlan> {
    match party_type {
        "Supplier" => Some(PartyQueryPlan {
            party_type: "Supplier",
            names,
            fields: vec![
                "name",
                "supplier_type as party_entity_type",
                "supplier_name as party_name",
            ],
        }),
        "Customer" => Some(PartyQueryPlan {
            party_type: "Customer",
            names,
            fields: vec![
                "name",
                "customer_type as party_entity_type",
                "customer_name as party_name",
            ],
        }),
        _ => None,
    }
}

pub fn fetch_additional_doc_info(
    entries: &[TaxWithholdingEntry],
    docs: &[DocInfo],
) -> Vec<DocInfo> {
    let mut result = Vec::new();
    for doctype in DOCUMENT_TYPES {
        let mut names = Vec::<&str>::new();
        for entry in entries {
            if entry.transaction_type == doctype
                && !entry.ref_no.is_empty()
                && !names.iter().any(|name| *name == entry.ref_no)
            {
                names.push(entry.ref_no.as_str());
            }
        }
        for name in names {
            if let Some(doc) = docs
                .iter()
                .find(|doc| doc.doctype == doctype && doc.name == name)
            {
                result.push(doc.clone());
            }
        }
    }
    result
}

pub fn get_doc_info_query(doctype: &str, names: Vec<String>) -> Option<DocInfoQueryPlan> {
    match doctype {
        "Purchase Invoice" => Some(DocInfoQueryPlan {
            doctype: "Purchase Invoice",
            names,
            fields: vec![
                "name",
                "grand_total",
                "base_total",
                "bill_no as supplier_invoice_no",
                "bill_date as supplier_invoice_date",
            ],
        }),
        "Sales Invoice" => Some(DocInfoQueryPlan {
            doctype: "Sales Invoice",
            names,
            fields: vec!["name", "grand_total", "base_total"],
        }),
        "Payment Entry" => Some(DocInfoQueryPlan {
            doctype: "Payment Entry",
            names,
            fields: vec![
                "name",
                "paid_amount_after_tax as grand_total",
                "base_paid_amount as base_total",
            ],
        }),
        "Journal Entry" => Some(DocInfoQueryPlan {
            doctype: "Journal Entry",
            names,
            fields: vec![
                "name",
                "total_debit as grand_total",
                "total_debit as base_total",
            ],
        }),
        _ => None,
    }
}

pub fn get_columns(filters: &TaxWithholdingFilters) -> Vec<TaxWithholdingColumn> {
    let party_type = filters.party_type.as_deref().unwrap_or("Party");
    vec![
        TaxWithholdingColumn::link(
            "Tax Withholding Category",
            "tax_withholding_category",
            "Tax Withholding Category",
            90,
        ),
        TaxWithholdingColumn::data("Tax Id", "tax_id", 60),
        TaxWithholdingColumn::data(format!("{party_type} Name"), "party_name", 180),
        TaxWithholdingColumn::dynamic_link(party_type, "party", "party_type", 180),
        TaxWithholdingColumn::data(format!("{party_type} Type"), "party_entity_type", 100),
        TaxWithholdingColumn::data("Supplier Invoice No", "supplier_invoice_no", 120),
        TaxWithholdingColumn::date("Supplier Invoice Date", "supplier_invoice_date", 120),
        TaxWithholdingColumn::percent("Tax Rate %", "rate", 60),
        TaxWithholdingColumn::currency("Taxable Amount", "total_amount", 120),
        TaxWithholdingColumn::currency("Tax Amount", "tax_amount", 120),
        TaxWithholdingColumn::currency("Grand Total (Company Currency)", "base_total", 150),
        TaxWithholdingColumn::currency("Grand Total (Transaction Currency)", "grand_total", 170),
        TaxWithholdingColumn::date("Reference Date", "taxable_date", 100),
        TaxWithholdingColumn::data("Transaction Type", "transaction_type", 130),
        TaxWithholdingColumn::dynamic_link("Reference No.", "ref_no", "transaction_type", 180),
        TaxWithholdingColumn::date("Date of Transaction", "transaction_date", 100),
        TaxWithholdingColumn::dynamic_link(
            "Withholding Document",
            "withholding_name",
            "withholding_doctype",
            150,
        ),
    ]
}

fn sort_key(row: &TaxWithholdingRow) -> (String, String, String) {
    (
        row.tax_withholding_category.clone(),
        row.transaction_date.clone().unwrap_or_default(),
        row.withholding_name.clone(),
    )
}
