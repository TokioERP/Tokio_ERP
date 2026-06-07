use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use tokio_erp::erpnext::accounts::doctype::financial_report_template::financial_report_engine::{
    AccountData, AccountReportMeta, DependencyResolver, EngineFilterValidation,
    FilterExpressionParser, FinancialQueryBuilder, FinancialReportEngine, FinancialReportPeriod,
    FormattingRule, FormulaCalculator, FormulaFieldExtractor, FormulaFieldUpdater, GlMovementRow,
    PeriodValue, ReportContext, RowData, RowProcessor, SectionData, SegmentData,
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

#[test]
fn filter_expression_parser_matches_erpnext_simple_and_logical_conditions() {
    let parser = FilterExpressionParser::new();
    let row = |formula: &str| FinancialReportRow {
        reference_code: Some("TEST_ROW".to_string()),
        data_source: Some("Account Data".to_string()),
        calculation_formula: Some(formula.to_string()),
        ..Default::default()
    };

    assert_eq!(
        parser.build_condition(&row(r#"["account_type", "=", "Income"]"#)),
        Some("account_type = 'Income'".to_string())
    );
    assert_eq!(
        parser.build_condition(&row(r#"["account_name", "like", "Cash"]"#)),
        Some("account_name LIKE '%Cash%'".to_string())
    );
    assert_eq!(
        parser.build_condition(&row(r#"["account_number", "like", "%100%"]"#)),
        Some("account_number LIKE '%100%'".to_string())
    );
    assert_eq!(
        parser.build_condition(&row(r#"["account_type", "in", ["Income", "Expense"]]"#)),
        Some("account_type IN ('Income', 'Expense')".to_string())
    );
    assert_eq!(
        parser.build_condition(&row(
            r#"{"and": [["account_type", "=", "Income"], ["is_group", "=", 0], ["disabled", "=", 0]]}"#
        )),
        Some("((account_type = 'Income' AND is_group = 0) AND disabled = 0)".to_string())
    );
    assert_eq!(
        parser.build_condition(&row(
            r#"{"or": [["root_type", "=", "Asset"], ["root_type", "=", "Liability"]]}"#
        )),
        Some("(root_type = 'Asset' OR root_type = 'Liability')".to_string())
    );
    assert_eq!(
        parser.build_condition(&row(
            r#"{"and": [{"or": [["root_type", "=", "Income"], ["root_type", "=", "Expense"]]}, ["is_group", "=", 0]]}"#
        )),
        Some("((root_type = 'Income' OR root_type = 'Expense') AND is_group = 0)".to_string())
    );
}

#[test]
fn filter_expression_parser_rejects_erpnext_invalid_filter_shapes() {
    let parser = FilterExpressionParser::new();
    let row = |formula: &str| FinancialReportRow {
        reference_code: Some("TEST_ROW".to_string()),
        data_source: Some("Account Data".to_string()),
        calculation_formula: Some(formula.to_string()),
        ..Default::default()
    };

    for formula in [
        r#"["incomplete"]"#,
        r#"{"invalid": "structure"}"#,
        "not_a_list_or_dict",
        r#"["field", "=", "value", "extra"]"#,
        r#"["field"]"#,
        r#"["field", "="]"#,
        r#"{"AND": [["field", "=", "value"]]}"#,
        r#"{"and": [["field", "=", "value"]], "or": [["field2", "=", "value2"]]}"#,
        r#"{"xor": [["field", "=", "value"]]}"#,
        r#"{"and": "not_a_list"}"#,
        r#"{"and": []}"#,
        r#"{"and": [["account_type", "=", "Bank"], "string", 123]}"#,
        r#"["account_type", "in", "Income"]"#,
        r#"["missing_field", "=", "Income"]"#,
        r#"["account_type", "bad", "Income"]"#,
    ] {
        assert_eq!(
            parser.build_condition(&row(formula)),
            None,
            "{formula} should be invalid"
        );
    }

    assert_eq!(parser.build_condition(&FinancialReportRow::default()), None);
}

#[test]
fn filter_expression_parser_build_conditions_ors_valid_account_rows() {
    let parser = FilterExpressionParser::new();
    let rows = vec![
        FinancialReportRow {
            data_source: Some("Account Data".to_string()),
            calculation_formula: Some(r#"["root_type", "=", "Income"]"#.to_string()),
            ..Default::default()
        },
        FinancialReportRow {
            data_source: Some("Account Data".to_string()),
            calculation_formula: Some(r#"["root_type", "=", "Expense"]"#.to_string()),
            ..Default::default()
        },
        FinancialReportRow {
            data_source: Some("Blank Line".to_string()),
            ..Default::default()
        },
    ];

    assert_eq!(
        parser.build_conditions(&rows),
        Some("(root_type = 'Income' OR root_type = 'Expense')".to_string())
    );
    assert_eq!(parser.build_conditions(&[]), None);
}

#[test]
fn financial_query_builder_calculates_running_balances_from_opening_and_gl_movements() {
    let periods = vec![
        FinancialReportPeriod::new("2024_jan", "2024-01-01", "2024-01-31"),
        FinancialReportPeriod::new("2024_feb", "2024-02-01", "2024-02-29"),
        FinancialReportPeriod::new("2024_mar", "2024-03-01", "2024-03-31"),
    ];
    let builder = FinancialQueryBuilder::new(
        serde_json::json!({"company": "_Test Company"}),
        periods,
        [(
            "_Test Bank - _TC".to_string(),
            AccountReportMeta {
                account_name: "Bank".to_string(),
                account_number: "1002".to_string(),
            },
        )]
        .into_iter()
        .collect(),
    );
    let mut balances_data = [(
        "_Test Cash - _TC".to_string(),
        AccountData {
            account: "_Test Cash - _TC".to_string(),
            account_name: "Cash".to_string(),
            account_number: "1001".to_string(),
            period_values: vec![PeriodValue {
                period_key: "2024_jan".to_string(),
                opening: 5000.0,
                closing: 0.0,
                movement: 0.0,
            }],
        },
    )]
    .into_iter()
    .collect();
    let gl_data = vec![
        GlMovementRow::new(
            "_Test Cash - _TC",
            [
                ("2024_jan".to_string(), 100.0),
                ("2024_mar".to_string(), -50.0),
            ]
            .into_iter()
            .collect(),
        ),
        GlMovementRow::new(
            "_Test Bank - _TC",
            [
                ("2024_jan".to_string(), -100.0),
                ("2024_feb".to_string(), -25.0),
            ]
            .into_iter()
            .collect(),
        ),
    ];

    builder.calculate_running_balances(&mut balances_data, &gl_data);

    let cash = balances_data.get("_Test Cash - _TC").unwrap();
    assert_eq!(
        cash.get_ordered_values(
            &[
                "2024_jan".to_string(),
                "2024_feb".to_string(),
                "2024_mar".to_string()
            ],
            "Opening Balance"
        ),
        [5000.0, 5100.0, 5100.0]
    );
    assert_eq!(
        cash.get_ordered_values(
            &[
                "2024_jan".to_string(),
                "2024_feb".to_string(),
                "2024_mar".to_string()
            ],
            "Closing Balance"
        ),
        [5100.0, 5100.0, 5050.0]
    );
    assert_eq!(
        cash.get_ordered_values(
            &[
                "2024_jan".to_string(),
                "2024_feb".to_string(),
                "2024_mar".to_string()
            ],
            "Period Movement (Debits - Credits)"
        ),
        [100.0, 0.0, -50.0]
    );

    let bank = balances_data.get("_Test Bank - _TC").unwrap();
    assert_eq!(bank.account_name, "Bank");
    assert_eq!(bank.account_number, "1002");
    assert_eq!(
        bank.get_ordered_values(
            &[
                "2024_jan".to_string(),
                "2024_feb".to_string(),
                "2024_mar".to_string()
            ],
            "Closing Balance"
        ),
        [-100.0, -125.0, -125.0]
    );
}

#[test]
fn financial_query_builder_handles_accumulated_values_like_erpnext() {
    let base = AccountData {
        account: "Cash".to_string(),
        account_name: "Cash".to_string(),
        account_number: "1001".to_string(),
        period_values: vec![
            PeriodValue {
                period_key: "p1".to_string(),
                opening: 10.0,
                closing: 25.0,
                movement: 15.0,
            },
            PeriodValue {
                period_key: "p2".to_string(),
                opening: 2.0,
                closing: 8.0,
                movement: 6.0,
            },
        ],
    };

    let mut default_data = [("Cash".to_string(), base.clone())].into_iter().collect();
    FinancialQueryBuilder::new(serde_json::json!({}), Vec::new(), Default::default())
        .handle_balance_accumulation(&mut default_data);
    assert_eq!(default_data.get("Cash").unwrap(), &base);

    let mut accumulated = [("Cash".to_string(), base.clone())].into_iter().collect();
    FinancialQueryBuilder::new(
        serde_json::json!({"accumulated_values": true}),
        Vec::new(),
        Default::default(),
    )
    .handle_balance_accumulation(&mut accumulated);
    assert_eq!(
        accumulated
            .get("Cash")
            .unwrap()
            .get_values_by_type("Period Movement (Debits - Credits)"),
        [25.0, 8.0]
    );
    assert_eq!(
        accumulated
            .get("Cash")
            .unwrap()
            .get_values_by_type("Closing Balance"),
        [25.0, 8.0]
    );

    let mut unaccumulated = [("Cash".to_string(), base)].into_iter().collect();
    FinancialQueryBuilder::new(
        serde_json::json!({"accumulated_values": false}),
        Vec::new(),
        Default::default(),
    )
    .handle_balance_accumulation(&mut unaccumulated);
    assert_eq!(
        unaccumulated
            .get("Cash")
            .unwrap()
            .get_values_by_type("Closing Balance"),
        [15.0, 6.0]
    );
    assert_eq!(
        unaccumulated
            .get("Cash")
            .unwrap()
            .get_values_by_type("Period Movement (Debits - Credits)"),
        [15.0, 6.0]
    );
}

#[test]
fn formula_field_extractor_and_updater_match_nested_filter_formula_behavior() {
    let rows = vec![
        FinancialReportRow {
            calculation_formula: Some(
                r#"{"and": [["account_category", "=", "Revenue"], {"or": [["account_category", "in", ["Direct Income", "Indirect Income"]], ["account_category", "like", "Tax"]]}]}"#
                    .to_string(),
            ),
            ..Default::default()
        },
        FinancialReportRow {
            calculation_formula: Some("invalid formula".to_string()),
            ..Default::default()
        },
    ];

    let extractor = FormulaFieldExtractor::new("account_category", vec!["like".to_string()]);
    assert_eq!(
        extractor.extract_from_rows(&rows),
        ["Direct Income", "Indirect Income", "Revenue"]
            .into_iter()
            .map(str::to_string)
            .collect()
    );

    let updater = FormulaFieldUpdater::new(
        "account_category",
        [
            ("Revenue".to_string(), "Operating Revenue".to_string()),
            ("Direct Income".to_string(), "Primary Revenue".to_string()),
            ("Tax".to_string(), "Ignored Tax".to_string()),
        ]
        .into_iter()
        .collect(),
        vec!["like".to_string(), "not like".to_string()],
    );
    let updates = updater.update_in_rows(
        [
            (
                "ROW1".to_string(),
                rows[0].calculation_formula.clone().unwrap(),
            ),
            ("ROW2".to_string(), "invalid formula".to_string()),
        ]
        .into_iter()
        .collect(),
    );
    assert_eq!(
        updates
            .get("ROW1")
            .unwrap()
            .get("calculation_formula")
            .unwrap(),
        r#"{"and":[["account_category","=","Operating Revenue"],{"or":[["account_category","in",["Primary Revenue","Indirect Income"]],["account_category","like","Tax"]]}]}"#
    );
    assert!(!updates.contains_key("ROW2"));
}

#[test]
fn row_processor_matches_erpnext_processing_order_and_row_value_outputs() {
    let template = FinancialReportTemplate {
        rows: vec![
            FinancialReportRow {
                reference_code: Some("TOTAL".to_string()),
                display_name: Some("Total".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("ACC001 + API001".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("ACC001".to_string()),
                display_name: Some("Account".to_string()),
                data_source: Some("Account Data".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                reference_code: Some("API001".to_string()),
                display_name: Some("API".to_string()),
                data_source: Some("Custom API".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                display_name: Some("Spacer".to_string()),
                data_source: Some("Blank Line".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                display_name: Some("Break".to_string()),
                data_source: Some("Column Break".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                display_name: Some("Section".to_string()),
                data_source: Some("Section Break".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let mut context = ReportContext::new(template);
    context.period_list = vec![
        BTreeMap::from([("key".to_string(), serde_json::json!("p1"))]),
        BTreeMap::from([("key".to_string(), serde_json::json!("p2"))]),
    ];
    context.raw_data.insert(
        "summary".to_string(),
        serde_json::json!({"ACC001": [10.0, 20.0], "API001": [3.0, 4.0]}),
    );
    context.raw_data.insert(
        "api_summary".to_string(),
        serde_json::json!({"API001": [3.0, 4.0]}),
    );
    context.raw_data.insert(
        "account_details".to_string(),
        serde_json::json!({"ACC001": {"Cash": {
            "account": "Cash",
            "account_name": "Cash",
            "account_number": "1001",
            "period_values": [
                {"period_key": "p1", "opening": 0.0, "closing": 10.0, "movement": 10.0},
                {"period_key": "p2", "opening": 10.0, "closing": 20.0, "movement": 10.0}
            ]
        }}}),
    );

    let processor = RowProcessor::new(&context).unwrap();
    let processed = processor.process_all_rows();
    let rows = processed
        .iter()
        .map(|row| {
            (
                row.row
                    .reference_code
                    .as_deref()
                    .unwrap_or(row.row.data_source.as_deref().unwrap_or("")),
                row.values.clone(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        rows,
        vec![
            ("API001", vec![3.0, 4.0]),
            ("ACC001", vec![10.0, 20.0]),
            ("TOTAL", vec![13.0, 24.0]),
            ("Blank Line", vec![0.0, 0.0]),
            ("Column Break", vec![]),
            ("Section Break", vec![]),
        ]
    );
    assert!(processed[1]
        .account_details
        .as_ref()
        .unwrap()
        .contains_key("Cash"));
}
