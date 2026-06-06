use tokio_erp::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use tokio_erp::erpnext::accounts::doctype::financial_report_template::financial_report_engine::{
    AccountData, EngineFilterValidation, FinancialReportEngine, FormattingRule, PeriodValue,
    ReportContext, RowData, SectionData, SegmentData, DEFAULT_BULLET_PREFIX, SEGMENT_PREFIX,
};
use tokio_erp::erpnext::accounts::doctype::financial_report_template::financial_report_template::FinancialReportTemplate;

#[test]
fn financial_report_engine_data_models_match_python_dataclasses() {
    assert_eq!(DEFAULT_BULLET_PREFIX, "• ");
    assert_eq!(SEGMENT_PREFIX, "seg_");

    let period = PeriodValue {
        period_key: "2026".to_string(),
        opening: 10.0,
        closing: 25.0,
        movement: 15.0,
    };
    assert_eq!(period.get_value("Opening Balance"), 10.0);
    assert_eq!(period.get_value("Closing Balance"), 25.0);
    assert_eq!(period.get_value("Period Movement (Debits - Credits)"), 15.0);
    assert_eq!(period.get_value("Unknown"), 0.0);

    let mut account = AccountData {
        account: "Cash - TC".to_string(),
        account_name: "Cash".to_string(),
        account_number: "1001".to_string(),
        ..Default::default()
    };
    assert!(!account.has_periods());
    account.add_period(period);
    account.add_period(PeriodValue {
        period_key: "2027".to_string(),
        opening: 2.0,
        closing: 8.0,
        movement: 6.0,
    });
    assert!(account.has_periods());
    assert_eq!(
        account.get_ordered_values(
            &["2027".to_string(), "missing".to_string()],
            "Closing Balance"
        ),
        [8.0, 0.0]
    );
    assert_eq!(
        account.get_values_by_type("Period Movement (Debits - Credits)"),
        [15.0, 6.0]
    );

    let copied = account.copy();
    account.accumulate_values();
    assert_eq!(
        account.get_ordered_values(
            &["2026".to_string(), "2027".to_string()],
            "Period Movement (Debits - Credits)"
        ),
        [25.0, 8.0]
    );
    account.unaccumulate_values();
    assert_eq!(
        account.get_ordered_values(&["2026".to_string(), "2027".to_string()], "Closing Balance"),
        [15.0, 6.0]
    );
    account.reverse_values();
    assert_eq!(
        account.get_ordered_values(&["2026".to_string(), "2027".to_string()], "Opening Balance"),
        [-10.0, -2.0]
    );
    assert_eq!(copied.get_period("2026").unwrap().closing, 25.0);

    let mut inserted_out_of_sort_order = AccountData::default();
    inserted_out_of_sort_order.add_period(PeriodValue {
        period_key: "2027".to_string(),
        opening: 0.0,
        closing: 0.0,
        movement: 6.0,
    });
    inserted_out_of_sort_order.add_period(PeriodValue {
        period_key: "2026".to_string(),
        opening: 0.0,
        closing: 0.0,
        movement: 15.0,
    });
    assert_eq!(
        inserted_out_of_sort_order.get_values_by_type("Period Movement (Debits - Credits)"),
        [6.0, 15.0]
    );
}

#[test]
fn financial_report_context_segments_and_formatting_rules_match_python_helpers() {
    let template = FinancialReportTemplate {
        template_name: "P&L".to_string(),
        ..Default::default()
    };
    let mut context = ReportContext::new(template);
    context.raw_data.insert(
        "columns".to_string(),
        serde_json::json!([{"fieldname": "account"}]),
    );
    context.raw_data.insert(
        "formatted_data".to_string(),
        serde_json::json!([{"account": "Revenue"}]),
    );
    context
        .raw_data
        .insert("chart".to_string(), serde_json::json!({"data": []}));

    let result = context.get_result();
    assert_eq!(
        result.columns,
        serde_json::json!([{"fieldname": "account"}])
    );
    assert_eq!(result.data, serde_json::json!([{"account": "Revenue"}]));
    assert_eq!(result.message, None);
    assert_eq!(result.chart, serde_json::json!({"data": []}));

    let segment = SegmentData {
        index: 2,
        label: "Segment".to_string(),
        rows: vec![RowData {
            row: FinancialReportRow {
                data_source: Some("Account Data".to_string()),
                ..Default::default()
            },
            values: vec![100.0],
            ..Default::default()
        }],
    };
    assert_eq!(segment.id(), "seg_2");
    let section = SectionData {
        index: 3,
        label: "Section".to_string(),
        segments: vec![segment],
    };
    assert_eq!(section.id(), "section_3");

    let rule = FormattingRule::static_properties(
        |row| row.values.iter().sum::<f64>() > 0.0,
        serde_json::json!({"bold": true}),
    );
    assert!(rule.applies_to(&section.segments[0].rows[0]));
    assert_eq!(
        rule.get_properties(&section.segments[0].rows[0]),
        serde_json::json!({"bold": true})
    );
}

#[test]
fn financial_report_engine_filter_validation_matches_required_and_warning_rules() {
    let validation = FinancialReportEngine::validate_filters(&serde_json::json!({}));
    assert_eq!(
        validation,
        EngineFilterValidation {
            missing_required: vec!["Report Template".to_string(), "Filter Based On".to_string()],
            warnings: vec![],
        }
    );

    let date_range = FinancialReportEngine::validate_filters(&serde_json::json!({
        "report_template": "P&L",
        "filter_based_on": "Date Range",
        "period_start_date": "2026-01-01",
        "presentation_currency": "EUR",
        "selected_view": "Margin",
    }));
    assert_eq!(date_range.missing_required, ["End Date"]);
    assert_eq!(
        date_range.warnings,
        [
            "Currency filters are currently unsupported in Custom Financial Report.",
            "Margin view is currently unsupported in Custom Financial Report.",
        ]
    );

    let fiscal_year = FinancialReportEngine::validate_filters(&serde_json::json!({
        "report_template": "Balance Sheet",
        "filter_based_on": "Fiscal Year",
        "from_fiscal_year": "2026",
        "to_fiscal_year": "2027",
        "selected_view": "Growth",
    }));
    assert!(fiscal_year.missing_required.is_empty());
    assert!(fiscal_year.warnings.is_empty());
}
