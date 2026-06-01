use serde_json::{json, Value};
use tokio_erp::erpnext::accounts::doctype::account::chart_of_accounts::verified::standard_chart_of_accounts::get;

const METADATA_FIELDS: [&str; 8] = [
    "account_name",
    "account_number",
    "account_type",
    "account_category",
    "root_type",
    "is_group",
    "tax_rate",
    "account_currency",
];

#[test]
fn standard_chart_of_accounts_matches_erpnext_source_tree() {
    let chart = get();
    let root_names = chart
        .as_object()
        .expect("chart root is an object")
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert_eq!(
        root_names,
        [
            "Application of Funds (Assets)",
            "Expenses",
            "Income",
            "Source of Funds (Liabilities)",
            "Equity",
        ]
    );

    assert_eq!(
        chart["Application of Funds (Assets)"]["root_type"],
        json!("Asset")
    );
    assert_eq!(chart["Expenses"]["root_type"], json!("Expense"));
    assert_eq!(chart["Income"]["root_type"], json!("Income"));
    assert_eq!(
        chart["Source of Funds (Liabilities)"]["root_type"],
        json!("Liability")
    );
    assert_eq!(chart["Equity"]["root_type"], json!("Equity"));

    assert_eq!(
        chart["Application of Funds (Assets)"]["Current Assets"]["Accounts Receivable"]["Debtors"],
        json!({
            "account_type": "Receivable",
            "account_category": "Trade Receivables",
        })
    );
    assert_eq!(
        chart["Expenses"]["Indirect Expenses"]["Freight and Forwarding Charges"],
        json!({
            "account_type": "Chargeable",
            "account_category": "Operating Expenses",
        })
    );
    assert_eq!(
        chart["Source of Funds (Liabilities)"]["Current Liabilities"]["Duties and Taxes"],
        json!({
            "account_type": "Tax",
            "is_group": 1,
            "account_category": "Current Tax Liabilities",
        })
    );
    assert_eq!(
        chart["Equity"]["Retained Earnings"],
        json!({
            "account_type": "Equity",
            "account_category": "Reserves and Surplus",
        })
    );

    assert_eq!(count_account_nodes(&chart), 94);
    assert_eq!(count_metadata_fields(&chart), 119);
}

fn count_account_nodes(value: &Value) -> usize {
    value
        .as_object()
        .map(|object| {
            object
                .iter()
                .filter(|(key, _)| !METADATA_FIELDS.contains(&key.as_str()))
                .map(|(_, child)| 1 + count_account_nodes(child))
                .sum()
        })
        .unwrap_or(0)
}

fn count_metadata_fields(value: &Value) -> usize {
    value
        .as_object()
        .map(|object| {
            object
                .iter()
                .map(|(key, child)| {
                    usize::from(METADATA_FIELDS.contains(&key.as_str()))
                        + count_metadata_fields(child)
                })
                .sum()
        })
        .unwrap_or(0)
}
