use tokio_erp::erpnext::accounts::test::test_reports::{
    default_filters, optional_filters, report_filter_test_cases, ReportFilterValue,
};

#[test]
fn accounts_test_reports_default_and_optional_filters_match_erpnext() {
    assert_eq!(
        default_filters()
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<_>>(),
        vec![
            ("company", "_Test Company"),
            ("from_date", "2010-01-01"),
            ("to_date", "2030-01-01"),
            ("period_start_date", "2010-01-01"),
            ("period_end_date", "2030-01-01"),
        ]
    );
    assert!(optional_filters().is_empty());
}

#[test]
fn accounts_test_reports_filter_cases_match_erpnext_script_report_matrix() {
    let cases = report_filter_test_cases();
    assert_eq!(cases.len(), 17);

    assert_eq!(cases[0].report_name, "General Ledger");
    assert_eq!(
        cases[0].filters.get("categorize_by"),
        Some(&ReportFilterValue::Text(
            "Categorize by Voucher (Consolidated)".to_string()
        ))
    );
    assert_eq!(
        cases[1].filters.get("include_dimensions"),
        Some(&ReportFilterValue::Int(1))
    );
    assert_eq!(
        cases[2].filters.get("range"),
        Some(&ReportFilterValue::Text("30, 60, 90, 120".to_string()))
    );
    assert_eq!(cases[4].report_name, "Consolidated Financial Statement");
    assert_eq!(
        cases[6].filters.get("report"),
        Some(&ReportFilterValue::Text("Cash Flow".to_string()))
    );

    let report_names = cases
        .iter()
        .map(|case| case.report_name)
        .collect::<Vec<_>>();
    assert_eq!(
        report_names,
        vec![
            "General Ledger",
            "General Ledger",
            "Accounts Payable",
            "Accounts Receivable",
            "Consolidated Financial Statement",
            "Consolidated Financial Statement",
            "Consolidated Financial Statement",
            "Gross Profit",
            "Gross Profit",
            "Gross Profit",
            "Gross Profit",
            "Gross Profit",
            "Item-wise Sales Register",
            "Item-wise Purchase Register",
            "Sales Register",
            "Sales Register",
            "Purchase Register",
        ]
    );

    assert!(cases[12].filters.is_empty());
    assert_eq!(
        cases[15].filters.get("item_group"),
        Some(&ReportFilterValue::Text("All Item Groups".to_string()))
    );
}
