use std::collections::BTreeSet;

use tokio_erp::erpnext::accounts::doctype::financial_report_template::financial_report_validation::{
    extract_reference_codes_from_formula, AccountFilterValidator, CalculationFormulaValidator,
    DependencyValidator, FinancialReportValidationRow, FinancialReportValidationTemplate,
    FormulaValidator, TemplateStructureValidator, TemplateValidator, ValidationIssue,
    ValidationResult,
};

#[test]
fn financial_report_validation_issue_and_result_match_erpnext_helpers() {
    let issue = ValidationIssue::new("Missing formula")
        .row_idx(4)
        .field("Formula");
    assert_eq!(issue.to_string(), "Row 4: [Formula] Missing formula");

    let mut result = ValidationResult::default();
    assert!(result.is_valid());
    assert!(!result.has_warnings());
    result.add_error(issue.clone());
    result.add_warning(ValidationIssue::new("Soft warning"));
    assert!(!result.is_valid());
    assert!(result.has_warnings());
    assert_eq!(result.error_count(), 1);
    assert_eq!(result.warning_count(), 1);

    let mut other = ValidationResult::default();
    other.add_error(ValidationIssue::new("Other"));
    result.merge(other);
    assert_eq!(result.error_count(), 2);
}

#[test]
fn financial_report_template_structure_validates_reference_codes_and_required_fields() {
    let template = FinancialReportValidationTemplate {
        rows: vec![
            FinancialReportValidationRow {
                idx: 1,
                reference_code: Some("1BAD".to_string()),
                data_source: Some("Account Data".to_string()),
                ..Default::default()
            },
            FinancialReportValidationRow {
                idx: 2,
                reference_code: Some("REV100".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                ..Default::default()
            },
            FinancialReportValidationRow {
                idx: 3,
                reference_code: Some("REV100".to_string()),
                data_source: Some("Custom API".to_string()),
                ..Default::default()
            },
        ],
    };

    let result = TemplateStructureValidator.validate(&template);
    let messages = result
        .issues
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert!(messages
        .iter()
        .any(|message| message.contains("Invalid line reference format: '1BAD'")));
    assert!(messages
        .iter()
        .any(|message| message.contains("Balance Type is required for Account Data")));
    assert!(messages
        .iter()
        .any(|message| message.contains("Formula is required for Calculated Amount")));
    assert!(messages
        .iter()
        .any(|message| message.contains("Duplicate line reference: 'REV100'")));
}

#[test]
fn financial_report_dependency_validator_detects_cycles_like_erpnext_available_code_scan() {
    let template = FinancialReportValidationTemplate {
        rows: vec![
            FinancialReportValidationRow {
                idx: 1,
                reference_code: Some("A".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("B + C".to_string()),
                ..Default::default()
            },
            FinancialReportValidationRow {
                idx: 2,
                reference_code: Some("B".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("A + MISSING".to_string()),
                ..Default::default()
            },
        ],
    };

    let validator = DependencyValidator::new(&template);
    assert_eq!(
        validator.dependencies(),
        &std::collections::BTreeMap::from([
            ("A".to_string(), vec!["B".to_string()]),
            ("B".to_string(), vec!["A".to_string()]),
        ])
    );

    let messages = validator
        .validate(&template)
        .issues
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    assert!(messages
        .iter()
        .any(|message| message.contains("Circular dependency detected")));
    assert!(!messages
        .iter()
        .any(|message| message.contains("Line References undefined")));
}

#[test]
fn financial_report_calculation_validator_matches_formula_rules() {
    let validator = CalculationFormulaValidator::new(BTreeSet::from([
        "REV100".to_string(),
        "EXP200".to_string(),
        "PROFIT".to_string(),
    ]));

    let mut row = FinancialReportValidationRow {
        idx: 7,
        reference_code: Some("PROFIT".to_string()),
        data_source: Some("Calculated Amount".to_string()),
        calculation_formula: Some(" PROFIT + REV100 ".to_string()),
        ..Default::default()
    };
    let messages = validator
        .validate(&mut row)
        .issues
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    assert_eq!(row.calculation_formula.as_deref(), Some("PROFIT + REV100"));
    assert!(messages
        .iter()
        .any(|message| message.contains("Formula references itself ('PROFIT')")));

    let mut unbalanced = FinancialReportValidationRow {
        idx: 8,
        data_source: Some("Calculated Amount".to_string()),
        calculation_formula: Some("(REV100 + EXP200".to_string()),
        ..Default::default()
    };
    assert_eq!(
        validator.validate(&mut unbalanced).issues[0].message,
        "Formula has unbalanced parentheses"
    );

    let mut bad_eval = FinancialReportValidationRow {
        idx: 9,
        data_source: Some("Calculated Amount".to_string()),
        calculation_formula: Some("REV100 / 0".to_string()),
        ..Default::default()
    };
    assert!(validator.validate(&mut bad_eval).issues[0]
        .message
        .contains("Formula evaluation error"));
}

#[test]
fn financial_report_account_filter_validator_matches_filter_structure_rules() {
    let validator = AccountFilterValidator::new(BTreeSet::from([
        "account_type".to_string(),
        "root_type".to_string(),
        "name".to_string(),
    ]));

    let mut row = FinancialReportValidationRow {
        idx: 1,
        data_source: Some("Account Data".to_string()),
        calculation_formula: Some(r#"["account_type", "in", "Receivable"]"#.to_string()),
        ..Default::default()
    };
    assert_eq!(
        validator.validate(&mut row).issues[0].message,
        "Operator 'in' requires a list value"
    );

    row.calculation_formula = Some(
        r#"{"and":[["account_type","=","Receivable"],["root_type","in",["Asset"]]]}"#.to_string(),
    );
    assert!(validator.validate(&mut row).is_valid());

    row.calculation_formula = Some(r#"["missing", "=", "x"]"#.to_string());
    assert_eq!(
        validator.validate(&mut row).issues[0].message,
        "Field 'missing' is not a valid Account field"
    );
}

#[test]
fn financial_report_formula_validator_and_template_validator_orchestrate_rows() {
    let template = FinancialReportValidationTemplate {
        rows: vec![
            FinancialReportValidationRow {
                idx: 1,
                reference_code: Some("REV".to_string()),
                data_source: Some("Account Data".to_string()),
                balance_type: Some("Closing Balance".to_string()),
                calculation_formula: Some(r#"["account_type", "=", "Receivable"]"#.to_string()),
                ..Default::default()
            },
            FinancialReportValidationRow {
                idx: 2,
                reference_code: Some("TOTAL".to_string()),
                data_source: Some("Calculated Amount".to_string()),
                calculation_formula: Some("REV + 1".to_string()),
                ..Default::default()
            },
            FinancialReportValidationRow {
                idx: 3,
                data_source: Some("Custom API".to_string()),
                calculation_formula: Some("badpath".to_string()),
                ..Default::default()
            },
        ],
    };

    let mut row = template.rows[2].clone();
    let formula_validator = FormulaValidator::new(&template);
    assert_eq!(
        formula_validator.validate(&mut row).issues[0].message,
        "Custom API path should be in format: app.module.method"
    );

    let result = TemplateValidator::new(&template).validate();
    assert_eq!(result.error_count(), 1);
    assert_eq!(
        result.issues[0].message,
        "Custom API path should be in format: app.module.method"
    );
}

#[test]
fn financial_report_extract_reference_codes_matches_complete_words_only() {
    assert_eq!(
        extract_reference_codes_from_formula(
            "REV100 + REV1000 + EXP_1 - EXP",
            &[
                "REV100".to_string(),
                "REV1000".to_string(),
                "EXP".to_string(),
                "EXP_1".to_string(),
            ],
        ),
        vec![
            "REV100".to_string(),
            "REV1000".to_string(),
            "EXP".to_string(),
            "EXP_1".to_string(),
        ]
    );
}
