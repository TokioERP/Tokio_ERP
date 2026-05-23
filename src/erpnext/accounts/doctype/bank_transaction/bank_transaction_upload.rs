use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct BankEntryDraft {
    pub fields: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BankEntriesPlan {
    pub doctype: &'static str,
    pub bank_account: String,
    pub normalize_date_field: bool,
    pub skipped_empty_rows: usize,
    pub entries: Vec<BankEntryDraft>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankTransactionUploadError {
    InvalidColumns(String),
    InvalidData(String),
    MissingColumn { row: usize, col_index: usize },
}

#[derive(Clone, Debug, Deserialize)]
struct UploadedColumn {
    content: String,
    #[serde(rename = "colIndex")]
    col_index: usize,
}

impl fmt::Display for BankTransactionUploadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidColumns(message) => write!(f, "invalid columns JSON: {message}"),
            Self::InvalidData(message) => write!(f, "invalid data JSON: {message}"),
            Self::MissingColumn { row, col_index } => {
                write!(f, "row {row} has no column at 1-based index {col_index}")
            }
        }
    }
}

impl Error for BankTransactionUploadError {}

pub fn get_header_mapping(
    columns_json: &str,
    bank_mapping: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, usize>, BankTransactionUploadError> {
    let columns: Vec<UploadedColumn> = serde_json::from_str(columns_json)
        .map_err(|error| BankTransactionUploadError::InvalidColumns(error.to_string()))?;

    let mut header_map = BTreeMap::new();
    for column in columns {
        if let Some(bank_transaction_field) = bank_mapping.get(&column.content) {
            header_map.insert(bank_transaction_field.clone(), column.col_index);
        }
    }

    Ok(header_map)
}

pub fn create_bank_entries_plan(
    columns_json: &str,
    data_json: &str,
    bank_account: &str,
    bank_mapping: &BTreeMap<String, String>,
) -> Result<BankEntriesPlan, BankTransactionUploadError> {
    let header_map = get_header_mapping(columns_json, bank_mapping)?;
    let data: Vec<Vec<Value>> = serde_json::from_str(data_json)
        .map_err(|error| BankTransactionUploadError::InvalidData(error.to_string()))?;

    let mut skipped_empty_rows = 0;
    let mut entries = Vec::new();

    for (row_index, row) in data.into_iter().enumerate() {
        if row.iter().all(Value::is_null) {
            skipped_empty_rows += 1;
            continue;
        }

        let mut fields = BTreeMap::new();
        for (field, col_index) in &header_map {
            let zero_based = col_index.saturating_sub(1);
            let value =
                row.get(zero_based)
                    .cloned()
                    .ok_or(BankTransactionUploadError::MissingColumn {
                        row: row_index + 1,
                        col_index: *col_index,
                    })?;
            fields.insert(field.clone(), value);
        }

        entries.push(BankEntryDraft { fields });
    }

    Ok(BankEntriesPlan {
        doctype: "Bank Transaction",
        bank_account: bank_account.to_string(),
        normalize_date_field: true,
        skipped_empty_rows,
        entries,
    })
}
