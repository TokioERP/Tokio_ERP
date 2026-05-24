use serde_json::Value;

pub const AFFECTED_DOCTYPES: [&str; 9] = [
    "POS Invoice",
    "Purchase Invoice",
    "Sales Invoice",
    "Purchase Order",
    "Supplier Quotation",
    "Quotation",
    "Sales Order",
    "Delivery Note",
    "Purchase Receipt",
];
pub const LAST_MODIFIED_DATE_THRESHOLD: &str = "2025-05-30";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReportColumn {
    pub label: &'static str,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiscountTransaction {
    pub doctype: String,
    pub name: String,
    pub currency: String,
    pub additional_discount_percentage: f64,
    pub formatted_discount_amount: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionRecord {
    pub ref_doctype: String,
    pub docname: String,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiscountMismatchRow {
    pub doctype: String,
    pub docname: String,
    pub actual_discount_percentage: f64,
    pub actual_discount_amount: String,
    pub suspected_discount_amount: String,
}

impl ReportColumn {
    pub const fn link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Link",
            options,
            width,
        }
    }

    pub const fn dynamic_link(
        label: &'static str,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Dynamic Link",
            options,
            width,
        }
    }

    pub const fn percent(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Percent",
            options: "",
            width,
        }
    }

    pub const fn currency(label: &'static str, fieldname: &'static str, width: u16) -> Self {
        Self {
            label,
            fieldname,
            fieldtype: "Currency",
            options: "",
            width,
        }
    }
}

impl DiscountTransaction {
    pub fn new(
        doctype: impl Into<String>,
        name: impl Into<String>,
        currency: impl Into<String>,
        additional_discount_percentage: f64,
        formatted_discount_amount: impl Into<String>,
    ) -> Self {
        Self {
            doctype: doctype.into(),
            name: name.into(),
            currency: currency.into(),
            additional_discount_percentage,
            formatted_discount_amount: formatted_discount_amount.into(),
        }
    }
}

impl VersionRecord {
    pub fn new(
        ref_doctype: impl Into<String>,
        docname: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        Self {
            ref_doctype: ref_doctype.into(),
            docname: docname.into(),
            data: data.into(),
        }
    }
}

pub fn get_columns() -> Vec<ReportColumn> {
    vec![
        ReportColumn::link("Transaction Type", "doctype", "DocType", 120),
        ReportColumn::dynamic_link("Transaction Name", "docname", "doctype", 150),
        ReportColumn::percent(
            "Discount Percentage in Transaction",
            "actual_discount_percentage",
            180,
        ),
        ReportColumn::currency(
            "Discount Amount in Transaction",
            "actual_discount_amount",
            180,
        ),
        ReportColumn::currency(
            "Suspected Discount Amount",
            "suspected_discount_amount",
            180,
        ),
    ]
}

pub fn get_transactions_query_plan(
    doctype: &'static str,
) -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("docstatus", "<", "2"),
        ("additional_discount_percentage", ">", "0"),
        ("discount_amount", "!=", "0"),
        ("modified", ">", LAST_MODIFIED_DATE_THRESHOLD),
        ("doctype", "=", doctype),
    ]
}

pub fn parse_discount_change(version_data: &str) -> Option<(String, String)> {
    let parsed = serde_json::from_str::<Value>(version_data).ok()?;
    let changed = parsed.get("changed")?.as_array()?;
    changed.iter().find_map(|row| {
        let row = row.as_array()?;
        if row.first()?.as_str()? != "discount_amount" {
            return None;
        }
        Some((
            row.get(1)?.as_str()?.to_string(),
            row.get(2)?.as_str()?.to_string(),
        ))
    })
}

pub fn detect_mismatches(
    transactions: &[DiscountTransaction],
    versions: &[VersionRecord],
) -> Vec<DiscountMismatchRow> {
    let transactions_by_key = transactions
        .iter()
        .map(|transaction| {
            (
                (transaction.doctype.as_str(), transaction.name.as_str()),
                transaction,
            )
        })
        .collect::<Vec<_>>();

    let mut versions_by_key: Vec<((&str, &str), Vec<&str>)> = Vec::new();
    for version in versions {
        let key = (version.ref_doctype.as_str(), version.docname.as_str());
        if !transactions_by_key
            .iter()
            .any(|(transaction_key, _)| *transaction_key == key)
        {
            continue;
        }
        if let Some((_, version_data_list)) = versions_by_key
            .iter_mut()
            .find(|(version_key, _)| *version_key == key)
        {
            version_data_list.push(version.data.as_str());
        } else {
            versions_by_key.push((key, vec![version.data.as_str()]));
        }
    }

    let mut rows = Vec::new();
    for (key, version_data_list) in versions_by_key {
        for version_data in version_data_list {
            if version_data.contains("\"additional_discount_percentage\"") {
                break;
            }

            let Some((old, new)) = parse_discount_change(version_data) else {
                continue;
            };
            let Some((_, transaction)) = transactions_by_key
                .iter()
                .find(|(transaction_key, _)| *transaction_key == key)
            else {
                continue;
            };
            if new != transaction.formatted_discount_amount {
                break;
            }

            rows.push(DiscountMismatchRow {
                doctype: transaction.doctype.clone(),
                docname: transaction.name.clone(),
                actual_discount_percentage: transaction.additional_discount_percentage,
                actual_discount_amount: new,
                suspected_discount_amount: old,
            });
            break;
        }
    }
    rows
}
