use tokio_erp::erpnext::accounts::report::calculated_discount_mismatch::calculated_discount_mismatch::{
    detect_mismatches, get_columns, get_transactions_query_plan, parse_discount_change,
    AFFECTED_DOCTYPES, LAST_MODIFIED_DATE_THRESHOLD, DiscountMismatchRow, DiscountTransaction,
    ReportColumn, VersionRecord,
};

#[test]
fn calculated_discount_mismatch_constants_match_erpnext() {
    assert_eq!(LAST_MODIFIED_DATE_THRESHOLD, "2025-05-30");
    assert_eq!(
        AFFECTED_DOCTYPES,
        [
            "POS Invoice",
            "Purchase Invoice",
            "Sales Invoice",
            "Purchase Order",
            "Supplier Quotation",
            "Quotation",
            "Sales Order",
            "Delivery Note",
            "Purchase Receipt",
        ]
    );
}

#[test]
fn calculated_discount_mismatch_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::link("Transaction Type", "doctype", "DocType", 120),
            ReportColumn::dynamic_link("Transaction Name", "docname", "doctype", 150),
            ReportColumn::percent(
                "Discount Percentage in Transaction",
                "actual_discount_percentage",
                180
            ),
            ReportColumn::currency(
                "Discount Amount in Transaction",
                "actual_discount_amount",
                180
            ),
            ReportColumn::currency(
                "Suspected Discount Amount",
                "suspected_discount_amount",
                180
            ),
        ]
    );
}

#[test]
fn calculated_discount_mismatch_transaction_query_plan_matches_erpnext_filters() {
    assert_eq!(
        get_transactions_query_plan("Sales Invoice"),
        vec![
            ("docstatus", "<", "2"),
            ("additional_discount_percentage", ">", "0"),
            ("discount_amount", "!=", "0"),
            ("modified", ">", LAST_MODIFIED_DATE_THRESHOLD),
            ("doctype", "=", "Sales Invoice"),
        ]
    );
}

#[test]
fn calculated_discount_mismatch_parses_discount_amount_change_from_version_json() {
    let version_data =
        r#"{"changed":[["discount_amount","$ 90.00","$ 100.00"],["status","Draft","Submitted"]]}"#;

    assert_eq!(
        parse_discount_change(version_data),
        Some(("$ 90.00".to_string(), "$ 100.00".to_string()))
    );
    assert_eq!(
        parse_discount_change(r#"{"changed":[["status","A","B"]]}"#),
        None
    );
    assert_eq!(parse_discount_change(r#"{"added":[]}"#), None);
}

#[test]
fn calculated_discount_mismatch_skips_when_newest_version_changes_percentage() {
    let transactions = vec![DiscountTransaction::new(
        "Sales Invoice",
        "SINV-0001",
        "USD",
        10.0,
        "$ 100.00",
    )];
    let versions = vec![
        VersionRecord::new(
            "Sales Invoice",
            "SINV-0001",
            r#"{"changed":[["additional_discount_percentage","5","10"]]}"#,
        ),
        VersionRecord::new(
            "Sales Invoice",
            "SINV-0001",
            r#"{"changed":[["discount_amount","$ 90.00","$ 100.00"]]}"#,
        ),
    ];

    assert!(detect_mismatches(&transactions, &versions).is_empty());
}

#[test]
fn calculated_discount_mismatch_keeps_only_versions_matching_current_discount_amount() {
    let transactions = vec![DiscountTransaction::new(
        "Sales Invoice",
        "SINV-0001",
        "USD",
        10.0,
        "$ 100.00",
    )];
    let versions = vec![VersionRecord::new(
        "Sales Invoice",
        "SINV-0001",
        r#"{"changed":[["discount_amount","$ 90.00","$ 95.00"]]}"#,
    )];

    assert!(detect_mismatches(&transactions, &versions).is_empty());
}

#[test]
fn calculated_discount_mismatch_reports_first_suspected_discount_amount() {
    let transactions = vec![
        DiscountTransaction::new("Sales Invoice", "SINV-0001", "USD", 10.0, "$ 100.00"),
        DiscountTransaction::new("Purchase Order", "PO-0001", "USD", 5.0, "$ 50.00"),
    ];
    let versions = vec![
        VersionRecord::new(
            "Sales Invoice",
            "SINV-0001",
            r#"{"changed":[["discount_amount","$ 90.00","$ 100.00"]]}"#,
        ),
        VersionRecord::new(
            "Sales Invoice",
            "SINV-0001",
            r#"{"changed":[["discount_amount","$ 80.00","$ 100.00"]]}"#,
        ),
        VersionRecord::new(
            "Purchase Order",
            "PO-0001",
            r#"{"changed":[["discount_amount","$ 45.00","$ 50.00"]]}"#,
        ),
    ];

    assert_eq!(
        detect_mismatches(&transactions, &versions),
        vec![
            DiscountMismatchRow {
                doctype: "Sales Invoice".to_string(),
                docname: "SINV-0001".to_string(),
                actual_discount_percentage: 10.0,
                actual_discount_amount: "$ 100.00".to_string(),
                suspected_discount_amount: "$ 90.00".to_string(),
            },
            DiscountMismatchRow {
                doctype: "Purchase Order".to_string(),
                docname: "PO-0001".to_string(),
                actual_discount_percentage: 5.0,
                actual_discount_amount: "$ 50.00".to_string(),
                suspected_discount_amount: "$ 45.00".to_string(),
            },
        ]
    );
}
