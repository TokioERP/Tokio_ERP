use tokio_erp::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use tokio_erp::erpnext::accounts::doctype::financial_report_template::financial_report_engine::{
    AccountData, DependencyResolver, EngineFilterValidation, FinancialReportEngine, FormattingRule,
    FormulaCalculator, PeriodValue, ReportContext, RowData, SectionData, SegmentData,
    DEFAULT_BULLET_PREFIX, SEGMENT_PREFIX,
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

#[test]
fn dependency_resolver_matches_erpnext_ordering_and_reference_extraction() {
    let template = FinancialReportTemplate {
        template_name: "Dependency Test".to_string(),
        rows: vec![
            FinancialReportRow {
                reference_code: Some("CALC001".to_string()),
                display_name: Some("Calculated".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("API001 + ACC001".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: None,
                display_name: Some("Spacing".to_string()),
                data_source: Some("Blank Line".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("API001".to_string()),
                display_name: Some("API".to_string()),
                data_source: Some("Custom API".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("ACC001".to_string()),
                display_name: Some("Account".to_string()),
                data_source: Some("Account Data".to_string()),
                calculation_formula: Some("[\"account_type\", \"=\", \"Income\"]".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("FINAL001".to_string()),
                display_name: Some("Final".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("(CALC001 + API001) * 0.5".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let resolver = DependencyResolver::new(&template).unwrap();
    assert_eq!(
        resolver.dependencies.get("CALC001").cloned().unwrap(),
        vec!["API001".to_string(), "ACC001".to_string()]
    );
    assert_eq!(
        resolver.dependencies.get("FINAL001").cloned().unwrap(),
        vec!["CALC001".to_string(), "API001".to_string()]
    );

    let order = resolver
        .get_processing_order()
        .iter()
        .map(|row| {
            row.reference_code
                .as_deref()
                .unwrap_or(row.data_source.as_deref().unwrap_or(""))
                .to_string()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        order,
        vec!["API001", "ACC001", "CALC001", "FINAL001", "Blank Line"]
    );
}

#[test]
fn dependency_resolver_rejects_circular_formula_dependencies() {
    let template = FinancialReportTemplate {
        template_name: "Cycle Test".to_string(),
        rows: vec![
            FinancialReportRow {
                reference_code: Some("A001".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("C001 + 100".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("B001".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("A001 + 200".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("C001".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("B001 * 1.5".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    assert_eq!(
        DependencyResolver::new(&template).unwrap_err(),
        "Circular dependency detected".to_string()
    );
}

#[test]
fn formula_calculator_matches_erpnext_period_arithmetic_and_math_functions() {
    let calculator = FormulaCalculator::new(
        [
            ("INC001".to_string(), vec![1000.0, 1200.0, 1500.0]),
            ("EXP001".to_string(), vec![800.0, 900.0, 1100.0]),
            ("TAX001".to_string(), vec![50.0, 60.0, 75.0]),
            ("NEG_VAL".to_string(), vec![-100.0, -200.0, -150.0]),
            ("BASE".to_string(), vec![4.0, 9.0, 16.0]),
            ("DECIMAL".to_string(), vec![2.7, 3.2, 4.9]),
        ]
        .into_iter()
        .collect(),
        vec![
            "2023_q1".to_string(),
            "2023_q2".to_string(),
            "2023_q3".to_string(),
        ],
        2,
    );

    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("(INC001 - EXP001) * 0.8".to_string()),
            ..Default::default()
        }),
        vec![160.0, 240.0, 320.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("abs(NEG_VAL) + round(TAX001 / 3, 2)".to_string()),
            ..Default::default()
        }),
        vec![116.67, 220.0, 175.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("max(INC001, EXP001) + min(TAX001, 55)".to_string()),
            ..Default::default()
        }),
        vec![1050.0, 1255.0, 1555.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("sqrt(BASE) + floor(DECIMAL) + ceil(DECIMAL)".to_string()),
            ..Default::default()
        }),
        vec![7.0, 10.0, 13.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("pow(BASE, 2)".to_string()),
            ..Default::default()
        }),
        vec![16.0, 81.0, 256.0]
    );
}

#[test]
fn formula_calculator_matches_erpnext_missing_values_reverse_and_error_handling() {
    let calculator = FormulaCalculator::new(
        [
            ("SHORT".to_string(), vec![100.0]),
            ("NORMAL".to_string(), vec![10.0, 20.0, 30.0]),
            ("ZERO".to_string(), vec![0.0, 0.0, 0.0]),
        ]
        .into_iter()
        .collect(),
        vec!["p1".to_string(), "p2".to_string(), "p3".to_string()],
        2,
    );

    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("SHORT + NORMAL".to_string()),
            ..Default::default()
        }),
        vec![110.0, 20.0, 30.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("NORMAL / ZERO".to_string()),
            ..Default::default()
        }),
        vec![0.0, 0.0, 0.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("NORMAL + +".to_string()),
            ..Default::default()
        }),
        vec![0.0, 0.0, 0.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("__import__('os').system('ls')".to_string()),
            ..Default::default()
        }),
        vec![0.0, 0.0, 0.0]
    );
    assert_eq!(
        calculator.evaluate_formula(&FinancialReportRow {
            calculation_formula: Some("NORMAL + 100".to_string()),
            reverse_sign: 1,
            ..Default::default()
        }),
        vec![-110.0, -120.0, -130.0]
    );
}
