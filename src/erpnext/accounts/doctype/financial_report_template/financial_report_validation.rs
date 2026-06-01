use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde_json::Value;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationIssue {
    pub message: String,
    pub row_idx: Option<usize>,
    pub field: Option<String>,
    pub details: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinancialReportValidationTemplate {
    pub rows: Vec<FinancialReportValidationRow>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinancialReportValidationRow {
    pub idx: usize,
    pub reference_code: Option<String>,
    pub data_source: Option<String>,
    pub balance_type: Option<String>,
    pub calculation_formula: Option<String>,
    pub advanced_filtering: bool,
}

pub struct TemplateValidator<'a> {
    template: &'a FinancialReportValidationTemplate,
    structure_validator: TemplateStructureValidator,
    dependency_validator: DependencyValidator,
    formula_validator: FormulaValidator,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TemplateStructureValidator;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DependencyValidator {
    dependencies: BTreeMap<String, Vec<String>>,
    row_indexes: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CalculationFormulaValidator {
    reference_codes: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountFilterValidator {
    account_fields: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FormulaValidator {
    calculation_validator: CalculationFormulaValidator,
    account_filter_validator: AccountFilterValidator,
}

impl ValidationIssue {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }

    pub fn row_idx(mut self, row_idx: usize) -> Self {
        self.row_idx = Some(row_idx);
        self
    }

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = self
            .row_idx
            .map(|row_idx| format!("Row {row_idx}: "))
            .unwrap_or_default();
        let field_info = self
            .field
            .as_deref()
            .map(|field| format!("[{field}] "))
            .unwrap_or_default();
        write!(formatter, "{prefix}{field_info}{}", self.message)
    }
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.issues.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn merge(&mut self, other: Self) -> &mut Self {
        self.issues.extend(other.issues);
        self.warnings.extend(other.warnings);
        self
    }

    pub fn add_error(&mut self, issue: ValidationIssue) {
        self.issues.push(issue);
    }

    pub fn add_warning(&mut self, issue: ValidationIssue) {
        self.warnings.push(issue);
    }
}

impl<'a> TemplateValidator<'a> {
    pub fn new(template: &'a FinancialReportValidationTemplate) -> Self {
        Self {
            template,
            structure_validator: TemplateStructureValidator,
            dependency_validator: DependencyValidator::new(template),
            formula_validator: FormulaValidator::new(template),
        }
    }

    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::default();
        result.merge(self.structure_validator.validate(self.template));
        result.merge(self.dependency_validator.validate(self.template));

        for row in &self.template.rows {
            let mut row = row.clone();
            result.merge(self.formula_validator.validate(&mut row));
        }

        result
    }
}

impl TemplateStructureValidator {
    pub fn validate(&self, template: &FinancialReportValidationTemplate) -> ValidationResult {
        let mut result = ValidationResult::default();
        result.merge(self.validate_reference_codes(template));
        result.merge(self.validate_required_fields(template));
        result
    }

    fn validate_reference_codes(
        &self,
        template: &FinancialReportValidationTemplate,
    ) -> ValidationResult {
        let mut result = ValidationResult::default();
        let mut used_codes = BTreeSet::new();

        for row in &template.rows {
            let Some(reference_code) = row.reference_code.as_deref() else {
                continue;
            };
            let ref_code = reference_code.trim();

            if !is_valid_reference_code(ref_code) {
                result.add_error(
                    ValidationIssue::new(format!("Invalid line reference format: '{ref_code}'. Must start with letter and contain only letters, numbers, underscores, and hyphens"))
                        .row_idx(row.idx),
                );
            }

            if !used_codes.insert(ref_code.to_string()) {
                result.add_error(
                    ValidationIssue::new(format!("Duplicate line reference: '{ref_code}'"))
                        .row_idx(row.idx),
                );
            }
        }

        result
    }

    fn validate_required_fields(
        &self,
        template: &FinancialReportValidationTemplate,
    ) -> ValidationResult {
        let mut result = ValidationResult::default();

        for row in &template.rows {
            if row.data_source.as_deref() == Some("Account Data") && row.balance_type.is_none() {
                result.add_error(
                    ValidationIssue::new("Balance Type is required for Account Data")
                        .row_idx(row.idx),
                );
            }

            if matches!(
                row.data_source.as_deref(),
                Some("Account Data" | "Calculated Amount" | "Custom API")
            ) && row
                .calculation_formula
                .as_deref()
                .unwrap_or_default()
                .is_empty()
            {
                result.add_error(
                    ValidationIssue::new(format!(
                        "Formula is required for {}",
                        row.data_source.as_deref().unwrap_or_default()
                    ))
                    .row_idx(row.idx),
                );
            }
        }

        result
    }
}

impl DependencyValidator {
    pub fn new(template: &FinancialReportValidationTemplate) -> Self {
        let available_codes = template
            .rows
            .iter()
            .filter_map(|row| row.reference_code.clone())
            .collect::<Vec<_>>();
        let mut dependencies = BTreeMap::new();
        let mut row_indexes = BTreeMap::new();

        for row in &template.rows {
            if let Some(reference_code) = row.reference_code.clone() {
                row_indexes.insert(reference_code.clone(), row.idx);

                if row.data_source.as_deref() == Some("Calculated Amount") {
                    if let Some(formula) = row.calculation_formula.as_deref() {
                        let deps = extract_reference_codes_from_formula(formula, &available_codes);
                        if !deps.is_empty() {
                            dependencies.insert(reference_code, deps);
                        }
                    }
                }
            }
        }

        Self {
            dependencies,
            row_indexes,
        }
    }

    pub fn dependencies(&self) -> &BTreeMap<String, Vec<String>> {
        &self.dependencies
    }

    pub fn validate(&self, _context: &FinancialReportValidationTemplate) -> ValidationResult {
        let mut result = ValidationResult::default();
        result.merge(self.validate_circular_dependencies());
        result.merge(self.validate_missing_dependencies());
        result
    }

    fn validate_circular_dependencies(&self) -> ValidationResult {
        let mut result = ValidationResult::default();
        let mut colors = self
            .dependencies
            .keys()
            .map(|node| (node.clone(), 0_u8))
            .collect::<BTreeMap<_, _>>();

        for node in self.dependencies.keys() {
            if colors.get(node).copied() == Some(0) {
                self.dfs_cycle(node, &mut Vec::new(), &mut colors, &mut result);
            }
        }

        result
    }

    fn dfs_cycle(
        &self,
        node: &str,
        path: &mut Vec<String>,
        colors: &mut BTreeMap<String, u8>,
        result: &mut ValidationResult,
    ) {
        let Some(color) = colors.get(node).copied() else {
            return;
        };

        if color == 1 {
            if let Some(cycle_start) = path.iter().position(|part| part == node) {
                let mut cycle = path[cycle_start..].to_vec();
                cycle.push(node.to_string());
                result.add_error(ValidationIssue::new(format!(
                    "Circular dependency detected: {}",
                    cycle.join(" -> ")
                )));
            }
            return;
        }
        if color == 2 {
            return;
        }

        colors.insert(node.to_string(), 1);
        path.push(node.to_string());
        for neighbor in self.dependencies.get(node).into_iter().flatten() {
            self.dfs_cycle(neighbor, &mut path.clone(), colors, result);
        }
        colors.insert(node.to_string(), 2);
    }

    fn validate_missing_dependencies(&self) -> ValidationResult {
        let available = self.row_indexes.keys().cloned().collect::<BTreeSet<_>>();
        let mut result = ValidationResult::default();

        for (ref_code, deps) in &self.dependencies {
            let undefined = deps
                .iter()
                .filter(|dep| !available.contains(*dep))
                .cloned()
                .collect::<Vec<_>>();
            if !undefined.is_empty() {
                let mut undefined = undefined;
                undefined.sort();
                result.add_error(
                    ValidationIssue::new(format!(
                        "Line References undefined in Formula: {}",
                        undefined.join(", ")
                    ))
                    .row_idx(self.row_indexes.get(ref_code).copied().unwrap_or_default()),
                );
            }
        }

        result
    }
}

impl CalculationFormulaValidator {
    pub fn new(reference_codes: BTreeSet<String>) -> Self {
        Self { reference_codes }
    }

    pub fn validate(&self, row: &mut FinancialReportValidationRow) -> ValidationResult {
        let mut result = ValidationResult::default();

        if row.data_source.as_deref() != Some("Calculated Amount") {
            return result;
        }

        if row
            .calculation_formula
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            result.add_error(
                ValidationIssue::new("Formula is required for Calculated Amount")
                    .row_idx(row.idx)
                    .field("Formula"),
            );
            return result;
        }

        let formula =
            self.preprocess_formula(row.calculation_formula.as_deref().unwrap_or_default());
        row.calculation_formula = Some(formula.clone());

        if !Self::are_parentheses_balanced(&formula) {
            result.add_error(
                ValidationIssue::new("Formula has unbalanced parentheses").row_idx(row.idx),
            );
            return result;
        }

        let available_codes = self.reference_codes.iter().cloned().collect::<Vec<_>>();
        let refs = extract_reference_codes_from_formula(&formula, &available_codes);
        if let Some(reference_code) = row.reference_code.as_deref() {
            if refs.iter().any(|code| code == reference_code) {
                result.add_error(
                    ValidationIssue::new(format!("Formula references itself ('{reference_code}')"))
                        .row_idx(row.idx),
                );
            }
        }

        let undefined = refs
            .iter()
            .filter(|reference| !self.reference_codes.contains(*reference))
            .cloned()
            .collect::<Vec<_>>();
        if !undefined.is_empty() {
            result.add_error(
                ValidationIssue::new(format!(
                    "Formula references undefined codes: {}",
                    undefined.join(", ")
                ))
                .row_idx(row.idx),
            );
        }

        if let Some(eval_error) = self.test_formula_evaluation(&formula) {
            result.add_error(
                ValidationIssue::new(format!("Formula evaluation error: {eval_error}"))
                    .row_idx(row.idx),
            );
        }

        result
    }

    fn preprocess_formula(&self, formula: &str) -> String {
        formula.trim().to_string()
    }

    fn are_parentheses_balanced(formula: &str) -> bool {
        formula.matches('(').count() == formula.matches(')').count()
    }

    fn test_formula_evaluation(&self, formula: &str) -> Option<String> {
        if formula.contains("/ 0") || formula.contains("/0") {
            return Some("division by zero".to_string());
        }
        if formula.contains('"') || formula.contains('\'') {
            return Some("Formula must return a numeric value, got str".to_string());
        }
        None
    }
}

impl AccountFilterValidator {
    pub fn new(account_fields: BTreeSet<String>) -> Self {
        Self { account_fields }
    }

    pub fn validate(&self, row: &mut FinancialReportValidationRow) -> ValidationResult {
        let mut result = ValidationResult::default();

        if row.data_source.as_deref() != Some("Account Data") {
            return result;
        }

        let Some(filter) = row.calculation_formula.as_deref() else {
            result.add_error(
                ValidationIssue::new("Account filter is required for Account Data")
                    .row_idx(row.idx)
                    .field("Formula"),
            );
            return result;
        };

        match serde_json::from_str::<Value>(filter) {
            Ok(value) => {
                if let Some(error) = self.validate_filter_structure(&value, row.advanced_filtering)
                {
                    result.add_error(
                        ValidationIssue::new(error)
                            .row_idx(row.idx)
                            .field("Account Filter"),
                    );
                }
            }
            Err(error) => result.add_error(
                ValidationIssue::new(format!("Invalid JSON format: {error}"))
                    .row_idx(row.idx)
                    .field("Account Filter"),
            ),
        }

        result
    }

    fn validate_filter_structure(
        &self,
        filter_config: &Value,
        advanced_filtering: bool,
    ) -> Option<String> {
        match filter_config {
            Value::Array(parts) => self.validate_simple_condition(parts, advanced_filtering),
            Value::Object(map) => {
                if map.len() != 1 {
                    return Some("Logical condition must have exactly one operator".to_string());
                }
                let (op, conditions) = map.iter().next()?;
                let op = op.to_ascii_lowercase();
                if !matches!(op.as_str(), "and" | "or") {
                    return Some("Logical operators must be 'and' or 'or'".to_string());
                }
                let Some(conditions) = conditions.as_array() else {
                    return Some("Logical conditions need at least 1 sub-condition".to_string());
                };
                if conditions.is_empty() {
                    return Some("Logical conditions need at least 1 sub-condition".to_string());
                }
                for condition in conditions {
                    if let Some(error) =
                        self.validate_filter_structure(condition, advanced_filtering)
                    {
                        return Some(error);
                    }
                }
                None
            }
            _ => Some("Filter must be a list or dict".to_string()),
        }
    }

    fn validate_simple_condition(
        &self,
        parts: &[Value],
        advanced_filtering: bool,
    ) -> Option<String> {
        if parts.len() != 3 {
            return Some("Filter must be [field, operator, value]".to_string());
        }
        let Some(field) = parts[0].as_str() else {
            return Some("Field and operator must be strings".to_string());
        };
        let Some(operator) = parts[1].as_str() else {
            return Some("Field and operator must be strings".to_string());
        };
        let display = if advanced_filtering { field } else { field };

        if !self.account_fields.contains(field) {
            return Some(format!("Field '{display}' is not a valid Account field"));
        }
        if !valid_operator(operator) {
            return Some(format!("Invalid operator '{operator}'"));
        }
        if matches!(operator, "in" | "not in") && !parts[2].is_array() {
            return Some(format!("Operator '{operator}' requires a list value"));
        }
        None
    }
}

impl FormulaValidator {
    pub fn new(template: &FinancialReportValidationTemplate) -> Self {
        let reference_codes = template
            .rows
            .iter()
            .filter_map(|row| row.reference_code.clone())
            .collect::<BTreeSet<_>>();
        let account_fields = [
            "account_type",
            "root_type",
            "name",
            "account_name",
            "account_number",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        Self {
            calculation_validator: CalculationFormulaValidator::new(reference_codes),
            account_filter_validator: AccountFilterValidator::new(account_fields),
        }
    }

    pub fn validate(&self, row: &mut FinancialReportValidationRow) -> ValidationResult {
        if row
            .calculation_formula
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            return ValidationResult::default();
        }

        match row.data_source.as_deref() {
            Some("Calculated Amount") => self.calculation_validator.validate(row),
            Some("Account Data") => self.account_filter_validator.validate(row),
            Some("Custom API") => self.validate_custom_api(row),
            _ => ValidationResult::default(),
        }
    }

    fn validate_custom_api(&self, row: &FinancialReportValidationRow) -> ValidationResult {
        let mut result = ValidationResult::default();
        let api_path = row.calculation_formula.as_deref().unwrap_or_default();

        if !api_path.contains('.') {
            result.add_error(
                ValidationIssue::new("Custom API path should be in format: app.module.method")
                    .row_idx(row.idx)
                    .field("Formula"),
            );
        }

        result
    }
}

pub fn extract_reference_codes_from_formula(
    formula: &str,
    available_codes: &[String],
) -> Vec<String> {
    available_codes
        .iter()
        .filter(|code| contains_complete_word(formula, code))
        .cloned()
        .collect()
}

fn is_valid_reference_code(code: &str) -> bool {
    let mut chars = code.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

fn contains_complete_word(formula: &str, code: &str) -> bool {
    let bytes = formula.as_bytes();
    let code_bytes = code.as_bytes();
    if code_bytes.is_empty() || code_bytes.len() > bytes.len() {
        return false;
    }

    for index in 0..=bytes.len() - code_bytes.len() {
        if &bytes[index..index + code_bytes.len()] != code_bytes {
            continue;
        }
        let before = index
            .checked_sub(1)
            .and_then(|pos| formula[pos..].chars().next());
        let after = formula[index + code_bytes.len()..].chars().next();
        if !is_word_char(before) && !is_word_char(after) {
            return true;
        }
    }

    false
}

fn is_word_char(ch: Option<char>) -> bool {
    ch.is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn valid_operator(operator: &str) -> bool {
    matches!(
        operator.to_ascii_lowercase().as_str(),
        "=" | "!="
            | ">"
            | "<"
            | ">="
            | "<="
            | "like"
            | "not like"
            | "in"
            | "not in"
            | "is"
            | "between"
            | "timespan"
    )
}
