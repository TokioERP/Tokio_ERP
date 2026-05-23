use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::accounts::doctype::bank_transaction::bank_transaction_upload::{
    create_bank_entries_plan, get_header_mapping, BankEntriesPlan, BankEntryDraft,
};

#[test]
fn bank_transaction_upload_header_mapping_matches_erpnext_json_columns() {
    let columns = json!([
        {"content": "Posting Date", "colIndex": 1},
        {"content": "Withdrawal", "colIndex": 2},
        {"content": "Deposit", "colIndex": 3},
        {"content": "Ignored", "colIndex": 4}
    ])
    .to_string();
    let bank_mapping = BTreeMap::from([
        ("Posting Date".to_string(), "date".to_string()),
        ("Withdrawal".to_string(), "withdrawal".to_string()),
        ("Deposit".to_string(), "deposit".to_string()),
    ]);

    assert_eq!(
        get_header_mapping(&columns, &bank_mapping).unwrap(),
        BTreeMap::from([
            ("date".to_string(), 1),
            ("deposit".to_string(), 3),
            ("withdrawal".to_string(), 2),
        ])
    );
}

#[test]
fn bank_transaction_upload_create_bank_entries_plan_matches_erpnext_row_mapping() {
    let columns = json!([
        {"content": "Posting Date", "colIndex": 1},
        {"content": "Withdrawal", "colIndex": 2},
        {"content": "Reference", "colIndex": 3}
    ])
    .to_string();
    let data = json!([
        ["2026-05-01", 1200.0, "REF-1"],
        [null, null, null],
        ["2026-05-02", 0.0, "REF-2"]
    ])
    .to_string();
    let bank_mapping = BTreeMap::from([
        ("Posting Date".to_string(), "date".to_string()),
        ("Withdrawal".to_string(), "withdrawal".to_string()),
        ("Reference".to_string(), "reference_number".to_string()),
    ]);

    assert_eq!(
        create_bank_entries_plan(&columns, &data, "BA-0001", &bank_mapping).unwrap(),
        BankEntriesPlan {
            doctype: "Bank Transaction",
            bank_account: "BA-0001".to_string(),
            normalize_date_field: true,
            skipped_empty_rows: 1,
            entries: vec![
                BankEntryDraft {
                    fields: BTreeMap::from([
                        ("date".to_string(), json!("2026-05-01")),
                        ("reference_number".to_string(), json!("REF-1")),
                        ("withdrawal".to_string(), json!(1200.0)),
                    ])
                },
                BankEntryDraft {
                    fields: BTreeMap::from([
                        ("date".to_string(), json!("2026-05-02")),
                        ("reference_number".to_string(), json!("REF-2")),
                        ("withdrawal".to_string(), json!(0.0)),
                    ])
                },
            ],
        }
    );
}

#[test]
fn bank_transaction_upload_preserves_erpnext_missing_column_failure_shape() {
    let columns = json!([{"content": "Posting Date", "colIndex": 3}]).to_string();
    let data = json!([["2026-05-01"]]).to_string();
    let bank_mapping = BTreeMap::from([("Posting Date".to_string(), "date".to_string())]);

    let error = create_bank_entries_plan(&columns, &data, "BA-0001", &bank_mapping)
        .expect_err("ERPNext would fail while reading d[int(value) - 1]");

    assert_eq!(error.to_string(), "row 1 has no column at 1-based index 3");
}
