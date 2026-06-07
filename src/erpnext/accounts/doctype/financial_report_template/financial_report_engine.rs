use std::collections::BTreeMap;

use serde_json::Value;

use crate::erpnext::accounts::doctype::account::account::Account;
use crate::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use crate::erpnext::accounts::doctype::financial_report_template::financial_report_template::FinancialReportTemplate;

pub const DEFAULT_BULLET_PREFIX: &str = "• ";
pub const SEGMENT_PREFIX: &str = "seg_";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PeriodValue {
    pub period_key: String,
    pub opening: f64,
    pub closing: f64,
    pub movement: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountData {
    pub account: String,
    pub account_name: String,
    pub account_number: String,
    pub period_values: Vec<PeriodValue>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RowData {
    pub row: FinancialReportRow,
    pub values: Vec<f64>,
    pub account_details: Option<BTreeMap<String, AccountData>>,
    pub is_detail_row: bool,
    pub parent_reference: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SegmentData {
    pub rows: Vec<RowData>,
    pub label: String,
    pub index: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SectionData {
    pub segments: Vec<SegmentData>,
    pub label: String,
    pub index: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReportContext {
    pub template: FinancialReportTemplate,
    pub filters: BTreeMap<String, Value>,
    pub period_list: Vec<BTreeMap<String, Value>>,
    pub processed_rows: Vec<RowData>,
    pub column_segments: Vec<Vec<RowData>>,
    pub account_data: BTreeMap<String, AccountData>,
    pub raw_data: BTreeMap<String, Value>,
    pub show_detailed: bool,
    pub currency: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReportResult {
    pub columns: Value,
    pub data: Value,
    pub message: Option<Value>,
    pub chart: Value,
}

pub struct FormattingRule {
    condition: fn(&RowData) -> bool,
    format_properties: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyResolver {
    rows: Vec<FinancialReportRow>,
    pub dependencies: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaCalculator {
    row_data: BTreeMap<String, Vec<f64>>,
    period_keys: Vec<String>,
    precision: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EngineFilterValidation {
    pub missing_required: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct FinancialReportEngine;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FilterExpressionParser;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinancialReportPeriod {
    pub key: String,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountReportMeta {
    pub account_name: String,
    pub account_number: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlMovementRow {
    pub account: String,
    pub period_movements: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FinancialQueryBuilder {
    filters: Value,
    periods: Vec<FinancialReportPeriod>,
    account_meta: BTreeMap<String, AccountReportMeta>,
}

impl PeriodValue {
    pub fn get_value(&self, balance_type: &str) -> f64 {
        match balance_type {
            "Opening Balance" => self.opening,
            "Closing Balance" => self.closing,
            "Period Movement (Debits - Credits)" => self.movement,
            _ => 0.0,
        }
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }
}

impl AccountData {
    pub fn add_period(&mut self, period_value: PeriodValue) {
        if let Some(existing) = self
            .period_values
            .iter_mut()
            .find(|period| period.period_key == period_value.period_key)
        {
            *existing = period_value;
        } else {
            self.period_values.push(period_value);
        }
    }

    pub fn get_period(&self, period_key: &str) -> Option<&PeriodValue> {
        self.period_values
            .iter()
            .find(|period| period.period_key == period_key)
    }

    pub fn get_values_by_type(&self, balance_type: &str) -> Vec<f64> {
        self.period_values
            .iter()
            .map(|period_value| period_value.get_value(balance_type))
            .collect()
    }

    pub fn get_ordered_values(&self, period_keys: &[String], balance_type: &str) -> Vec<f64> {
        period_keys
            .iter()
            .map(|key| {
                self.period_values
                    .iter()
                    .find(|period| period.period_key == *key)
                    .map(|period_value| period_value.get_value(balance_type))
                    .unwrap_or(0.0)
            })
            .collect()
    }

    pub fn has_periods(&self) -> bool {
        !self.period_values.is_empty()
    }

    pub fn accumulate_values(&mut self) {
        for period_value in &mut self.period_values {
            period_value.movement += period_value.opening;
        }
    }

    pub fn unaccumulate_values(&mut self) {
        for period_value in &mut self.period_values {
            period_value.closing -= period_value.opening;
        }
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }

    pub fn reverse_values(&mut self) {
        for period_value in &mut self.period_values {
            period_value.opening = reverse_nonzero(period_value.opening);
            period_value.closing = reverse_nonzero(period_value.closing);
            period_value.movement = reverse_nonzero(period_value.movement);
        }
    }
}

impl SegmentData {
    pub fn id(&self) -> String {
        format!("{SEGMENT_PREFIX}{}", self.index)
    }
}

impl SectionData {
    pub fn id(&self) -> String {
        format!("section_{}", self.index)
    }
}

impl ReportContext {
    pub fn new(template: FinancialReportTemplate) -> Self {
        Self {
            template,
            ..Default::default()
        }
    }

    pub fn get_result(&self) -> ReportResult {
        ReportResult {
            columns: self
                .raw_data
                .get("columns")
                .cloned()
                .unwrap_or_else(|| Value::Array(Vec::new())),
            data: self
                .raw_data
                .get("formatted_data")
                .cloned()
                .unwrap_or_else(|| Value::Array(Vec::new())),
            message: None,
            chart: self
                .raw_data
                .get("chart")
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default())),
        }
    }
}

impl FormattingRule {
    pub fn static_properties(condition: fn(&RowData) -> bool, format_properties: Value) -> Self {
        Self {
            condition,
            format_properties,
        }
    }

    pub fn applies_to(&self, row_data: &RowData) -> bool {
        (self.condition)(row_data)
    }

    pub fn get_properties(&self, _row_data: &RowData) -> Value {
        self.format_properties.clone()
    }
}

impl FinancialReportEngine {
    pub fn validate_filters(filters: &Value) -> EngineFilterValidation {
        let required_filters_by_basis: BTreeMap<&str, &[&str]> = BTreeMap::from([
            ("Date Range", &["period_start_date", "period_end_date"][..]),
            ("Fiscal Year", &["from_fiscal_year", "to_fiscal_year"][..]),
        ]);

        let mut required_filters = vec!["report_template", "filter_based_on"];
        if let Some(filter_based_on) = get_string(filters, "filter_based_on") {
            if let Some(extra_filters) = required_filters_by_basis.get(filter_based_on) {
                required_filters.extend(extra_filters.iter().copied());
            }
        }

        let missing_required = required_filters
            .into_iter()
            .filter(|filter_key| {
                get_string(filters, filter_key)
                    .unwrap_or_default()
                    .is_empty()
            })
            .map(filter_label)
            .collect();

        let mut warnings = Vec::new();
        if get_string(filters, "presentation_currency")
            .map(|value| !value.is_empty())
            .unwrap_or(false)
        {
            warnings.push(
                "Currency filters are currently unsupported in Custom Financial Report."
                    .to_string(),
            );
        }

        if let Some(view) = get_string(filters, "selected_view") {
            if !view.is_empty() && !matches!(view, "Report" | "Growth") {
                warnings.push(format!(
                    "{view} view is currently unsupported in Custom Financial Report."
                ));
            }
        }

        EngineFilterValidation {
            missing_required,
            warnings,
        }
    }
}

impl FinancialReportPeriod {
    pub fn new(key: &str, from_date: &str, to_date: &str) -> Self {
        Self {
            key: key.to_string(),
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
        }
    }
}

impl GlMovementRow {
    pub fn new(account: &str, period_movements: BTreeMap<String, f64>) -> Self {
        Self {
            account: account.to_string(),
            period_movements,
        }
    }
}

impl FinancialQueryBuilder {
    pub fn new(
        filters: Value,
        periods: Vec<FinancialReportPeriod>,
        account_meta: BTreeMap<String, AccountReportMeta>,
    ) -> Self {
        Self {
            filters,
            periods,
            account_meta,
        }
    }

    pub fn calculate_running_balances(
        &self,
        balances_data: &mut BTreeMap<String, AccountData>,
        gl_data: &[GlMovementRow],
    ) {
        let gl_dict = gl_data
            .iter()
            .map(|row| (row.account.clone(), row))
            .collect::<BTreeMap<_, _>>();
        let mut accounts = balances_data.keys().cloned().collect::<Vec<_>>();
        for account in gl_dict.keys() {
            if !accounts.iter().any(|existing| existing == account) {
                accounts.push(account.clone());
            }
        }

        for account in accounts {
            if !balances_data.contains_key(&account) {
                let meta = self.get_account_meta(&account);
                balances_data.insert(
                    account.clone(),
                    AccountData {
                        account: account.clone(),
                        account_name: meta.account_name,
                        account_number: meta.account_number,
                        ..Default::default()
                    },
                );
            }

            let account_data = balances_data
                .get_mut(&account)
                .expect("account was inserted before balance calculation");
            let gl_movement = gl_dict.get(&account);
            let mut current_balance = if account_data.has_periods() {
                account_data
                    .get_period(&self.periods[0].key)
                    .map(|period| period.get_value("Opening Balance"))
                    .unwrap_or(0.0)
            } else {
                0.0
            };

            for period in &self.periods {
                let movement = gl_movement
                    .and_then(|row| row.period_movements.get(&period.key))
                    .copied()
                    .unwrap_or(0.0);
                let closing_balance = current_balance + movement;

                account_data.add_period(PeriodValue {
                    period_key: period.key.clone(),
                    opening: current_balance,
                    closing: closing_balance,
                    movement,
                });
                current_balance = closing_balance;
            }
        }
    }

    pub fn handle_balance_accumulation(&self, balances_data: &mut BTreeMap<String, AccountData>) {
        let accumulated_values = self
            .filters
            .get("accumulated_values")
            .and_then(Value::as_bool);

        match accumulated_values {
            None => {}
            Some(true) => {
                for account_data in balances_data.values_mut() {
                    account_data.accumulate_values();
                }
            }
            Some(false) => {
                for account_data in balances_data.values_mut() {
                    account_data.unaccumulate_values();
                }
            }
        }
    }

    fn get_account_meta(&self, account: &str) -> AccountReportMeta {
        self.account_meta.get(account).cloned().unwrap_or_default()
    }
}

impl FilterExpressionParser {
    pub const fn new() -> Self {
        Self
    }

    pub fn build_conditions(&self, report_rows: &[FinancialReportRow]) -> Option<String> {
        combine_conditions(
            "OR",
            report_rows
                .iter()
                .filter_map(|row| self.build_condition(row))
                .collect(),
        )
    }

    pub fn build_condition(&self, report_row: &FinancialReportRow) -> Option<String> {
        let filter_formula = report_row.calculation_formula.as_deref()?.trim();
        if filter_formula.is_empty() {
            return None;
        }

        let parsed = serde_json::from_str::<Value>(filter_formula).ok()?;
        self.validate_filter_structure(&parsed).ok()?;
        self.build_from_parsed(&parsed)
    }

    fn validate_filter_structure(&self, parsed: &Value) -> Result<(), ()> {
        match parsed {
            Value::Array(condition) => {
                if condition.len() != 3 {
                    return Err(());
                }
                let Some(field) = condition[0].as_str() else {
                    return Err(());
                };
                let Some(operator) = condition[1].as_str() else {
                    return Err(());
                };
                if !Account::FIELD_ORDER.contains(&field) {
                    return Err(());
                }
                if !is_valid_filter_operator(operator) {
                    return Err(());
                }
                if matches!(operator, "in" | "not in") && !condition[2].is_array() {
                    return Err(());
                }
                Ok(())
            }
            Value::Object(condition) => {
                if condition.len() != 1 {
                    return Err(());
                }
                let (operator, sub_conditions) = condition.iter().next().ok_or(())?;
                if !matches!(operator.as_str(), "and" | "or") {
                    return Err(());
                }
                let Some(sub_conditions) = sub_conditions.as_array() else {
                    return Err(());
                };
                if sub_conditions.is_empty() {
                    return Err(());
                }
                for sub_condition in sub_conditions {
                    self.validate_filter_structure(sub_condition)?;
                }
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn build_from_parsed(&self, parsed: &Value) -> Option<String> {
        match parsed {
            Value::Array(condition) => self.build_simple_condition(condition),
            Value::Object(condition) => self.build_logical_condition(condition),
            _ => None,
        }
    }

    fn build_simple_condition(&self, condition: &[Value]) -> Option<String> {
        let field_name = condition.first()?.as_str()?;
        let operator = condition.get(1)?.as_str()?;
        let value = condition.get(2)?;
        if value.is_null() {
            return None;
        }

        let operator_key = operator.to_ascii_lowercase();
        match operator_key.as_str() {
            "=" | "!=" | ">" | ">=" | "<" | "<=" => Some(format!(
                "{field_name} {operator_key} {}",
                render_value(value)?
            )),
            "like" | "not like" => {
                let mut value = value.as_str()?.to_string();
                if !value.contains('%') {
                    value = format!("%{value}%");
                }
                Some(format!(
                    "{field_name} {} {}",
                    operator_key.to_ascii_uppercase(),
                    quote_sql_string(&value)
                ))
            }
            "in" | "not in" => {
                let values = value.as_array()?;
                let rendered_values = values
                    .iter()
                    .map(render_value)
                    .collect::<Option<Vec<_>>>()?;
                Some(format!(
                    "{field_name} {} ({})",
                    operator_key.to_ascii_uppercase(),
                    rendered_values.join(", ")
                ))
            }
            _ => None,
        }
    }

    fn build_logical_condition(
        &self,
        condition: &serde_json::Map<String, Value>,
    ) -> Option<String> {
        if condition.len() != 1 {
            return None;
        }
        let (operator, sub_conditions) = condition.iter().next()?;
        if !matches!(operator.as_str(), "and" | "or") {
            return None;
        }
        let operator = operator.to_ascii_uppercase();
        let built_conditions = sub_conditions
            .as_array()?
            .iter()
            .filter_map(|sub_condition| self.build_from_parsed(sub_condition))
            .collect::<Vec<_>>();
        combine_conditions(&operator, built_conditions)
    }
}

impl DependencyResolver {
    pub fn new(template: &FinancialReportTemplate) -> Result<Self, String> {
        let rows = template.rows.clone();
        let dependencies = collect_dependencies(&rows);
        if has_cycle(&dependencies) {
            return Err("Circular dependency detected".to_string());
        }
        Ok(Self { rows, dependencies })
    }

    pub fn get_processing_order(&self) -> Vec<FinancialReportRow> {
        let mut api_rows = Vec::new();
        let mut account_rows = Vec::new();
        let mut formula_rows = Vec::new();
        let mut other_rows = Vec::new();

        for row in &self.rows {
            match row.data_source.as_deref() {
                Some("Custom API") => api_rows.push(row.clone()),
                Some("Account Data") => account_rows.push(row.clone()),
                Some("Calculated Amount") => formula_rows.push(row.clone()),
                _ => other_rows.push(row.clone()),
            }
        }

        let mut ordered_rows = api_rows;
        ordered_rows.extend(account_rows);
        ordered_rows.extend(self.topological_sort_formula_rows(&formula_rows));
        ordered_rows.extend(other_rows);
        ordered_rows
    }

    fn topological_sort_formula_rows(
        &self,
        formula_rows: &[FinancialReportRow],
    ) -> Vec<FinancialReportRow> {
        let formula_row_map = formula_rows
            .iter()
            .filter_map(|row| non_empty_reference(row).map(|code| (code.to_string(), row.clone())))
            .collect::<BTreeMap<_, _>>();
        let mut adj_list = formula_row_map
            .keys()
            .map(|code| (code.clone(), Vec::<String>::new()))
            .collect::<BTreeMap<_, _>>();
        let mut in_degree = formula_row_map
            .keys()
            .map(|code| (code.clone(), 0usize))
            .collect::<BTreeMap<_, _>>();

        for code in formula_row_map.keys() {
            for dep in self.dependencies.get(code).into_iter().flatten() {
                if formula_row_map.contains_key(dep) {
                    adj_list.entry(dep.clone()).or_default().push(code.clone());
                    *in_degree.entry(code.clone()).or_default() += 1;
                }
            }
        }

        let mut queue = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(code, _)| code.clone())
            .collect::<Vec<_>>();
        let mut result = Vec::new();

        while let Some(current) = queue.first().cloned() {
            queue.remove(0);
            if let Some(row) = formula_row_map.get(&current) {
                result.push(row.clone());
            }
            for neighbor in adj_list.get(&current).into_iter().flatten() {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(neighbor.clone());
                    }
                }
            }
        }

        for row in formula_rows {
            if !result.iter().any(|existing| existing == row) {
                result.push(row.clone());
            }
        }
        result
    }
}

impl FormulaCalculator {
    pub fn new(
        row_data: BTreeMap<String, Vec<f64>>,
        period_keys: Vec<String>,
        precision: u32,
    ) -> Self {
        Self {
            row_data,
            period_keys,
            precision,
        }
    }

    pub fn evaluate_formula(&self, report_row: &FinancialReportRow) -> Vec<f64> {
        let formula = report_row
            .calculation_formula
            .as_deref()
            .unwrap_or_default()
            .trim();
        if formula.is_empty() {
            return vec![0.0; self.period_keys.len()];
        }

        let negation_factor = if report_row.reverse_sign != 0 {
            -1.0
        } else {
            1.0
        };
        (0..self.period_keys.len())
            .map(|period_index| {
                let context = self.build_context(period_index);
                let mut parser = FormulaParser::new(formula, &context);
                parser
                    .parse()
                    .map(|value| round_to(value * negation_factor, self.precision))
                    .unwrap_or(0.0)
            })
            .collect()
    }

    pub fn build_context(&self, period_index: usize) -> BTreeMap<String, f64> {
        self.row_data
            .iter()
            .map(|(code, values)| {
                (
                    code.clone(),
                    values.get(period_index).copied().unwrap_or(0.0),
                )
            })
            .collect()
    }
}

fn is_valid_filter_operator(operator: &str) -> bool {
    matches!(
        operator.to_ascii_lowercase().as_str(),
        "=" | "!=" | ">" | ">=" | "<" | "<=" | "like" | "not like" | "in" | "not in"
    )
}

fn combine_conditions(operator: &str, conditions: Vec<String>) -> Option<String> {
    let mut iter = conditions.into_iter();
    let first = iter.next()?;
    Some(iter.fold(first, |left, right| format!("({left} {operator} {right})")))
}

fn render_value(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(quote_sql_string(value)),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

fn quote_sql_string(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn get_string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

fn filter_label(filter_key: &str) -> String {
    match filter_key {
        "report_template" => "Report Template",
        "filter_based_on" => "Filter Based On",
        "period_start_date" => "Start Date",
        "period_end_date" => "End Date",
        "from_fiscal_year" => "Start Year",
        "to_fiscal_year" => "End Year",
        _ => filter_key,
    }
    .to_string()
}

fn reverse_nonzero(value: f64) -> f64 {
    if value == 0.0 {
        0.0
    } else {
        -value
    }
}

fn collect_dependencies(rows: &[FinancialReportRow]) -> BTreeMap<String, Vec<String>> {
    let reference_codes = rows
        .iter()
        .filter_map(non_empty_reference)
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let mut dependencies = BTreeMap::new();

    for row in rows {
        let Some(code) = non_empty_reference(row) else {
            continue;
        };
        let formula = row.calculation_formula.as_deref().unwrap_or_default();
        let deps = reference_codes
            .iter()
            .filter(|reference| *reference != code)
            .filter(|reference| formula_contains_reference(formula, reference))
            .cloned()
            .collect::<Vec<_>>();
        if !deps.is_empty() {
            dependencies.insert(code.to_string(), deps);
        }
    }

    dependencies
}

fn non_empty_reference(row: &FinancialReportRow) -> Option<&str> {
    row.reference_code
        .as_deref()
        .map(str::trim)
        .filter(|code| !code.is_empty())
}

fn formula_contains_reference(formula: &str, reference: &str) -> bool {
    let tokens = formula
        .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
        .filter(|token| !token.is_empty());
    tokens.into_iter().any(|token| token == reference)
}

fn has_cycle(dependencies: &BTreeMap<String, Vec<String>>) -> bool {
    fn visit(
        code: &str,
        dependencies: &BTreeMap<String, Vec<String>>,
        visiting: &mut Vec<String>,
        visited: &mut Vec<String>,
    ) -> bool {
        if visiting.iter().any(|item| item == code) {
            return true;
        }
        if visited.iter().any(|item| item == code) {
            return false;
        }
        visiting.push(code.to_string());
        for dep in dependencies.get(code).into_iter().flatten() {
            if visit(dep, dependencies, visiting, visited) {
                return true;
            }
        }
        visiting.retain(|item| item != code);
        visited.push(code.to_string());
        false
    }

    let mut visiting = Vec::new();
    let mut visited = Vec::new();
    dependencies
        .keys()
        .any(|code| visit(code, dependencies, &mut visiting, &mut visited))
}

struct FormulaParser<'a> {
    chars: Vec<char>,
    pos: usize,
    context: &'a BTreeMap<String, f64>,
}

impl<'a> FormulaParser<'a> {
    fn new(formula: &str, context: &'a BTreeMap<String, f64>) -> Self {
        Self {
            chars: formula.chars().collect(),
            pos: 0,
            context,
        }
    }

    fn parse(&mut self) -> Result<f64, String> {
        let value = self.parse_expression()?;
        self.skip_whitespace();
        if self.pos == self.chars.len() {
            Ok(value)
        } else {
            Err("unexpected trailing input".to_string())
        }
    }

    fn parse_expression(&mut self) -> Result<f64, String> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_whitespace();
            if self.consume('+') {
                value += self.parse_term()?;
            } else if self.consume('-') {
                value -= self.parse_term()?;
            } else {
                return Ok(value);
            }
        }
    }

    fn parse_term(&mut self) -> Result<f64, String> {
        let mut value = self.parse_factor()?;
        loop {
            self.skip_whitespace();
            if self.consume('*') {
                value *= self.parse_factor()?;
            } else if self.consume('/') {
                let divisor = self.parse_factor()?;
                if divisor == 0.0 {
                    return Err("division by zero".to_string());
                }
                value /= divisor;
            } else {
                return Ok(value);
            }
        }
    }

    fn parse_factor(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        if self.consume('+') {
            return self.parse_factor();
        }
        if self.consume('-') {
            return Ok(-self.parse_factor()?);
        }
        if self.consume('(') {
            let value = self.parse_expression()?;
            self.skip_whitespace();
            if !self.consume(')') {
                return Err("missing closing parenthesis".to_string());
            }
            return Ok(value);
        }
        if self
            .peek()
            .is_some_and(|ch| ch.is_ascii_digit() || ch == '.')
        {
            return self.parse_number();
        }
        if self
            .peek()
            .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
        {
            return self.parse_identifier_or_function();
        }
        Err("unexpected token".to_string())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start = self.pos;
        while self
            .peek()
            .is_some_and(|ch| ch.is_ascii_digit() || ch == '.')
        {
            self.pos += 1;
        }
        self.chars[start..self.pos]
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .map_err(|_| "invalid number".to_string())
    }

    fn parse_identifier_or_function(&mut self) -> Result<f64, String> {
        let ident = self.parse_identifier();
        self.skip_whitespace();
        if self.consume('(') {
            let args = self.parse_arguments()?;
            return evaluate_function(&ident, &args);
        }
        self.context
            .get(&ident)
            .copied()
            .ok_or_else(|| "unknown identifier".to_string())
    }

    fn parse_identifier(&mut self) -> String {
        let start = self.pos;
        while self
            .peek()
            .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            self.pos += 1;
        }
        self.chars[start..self.pos].iter().collect()
    }

    fn parse_arguments(&mut self) -> Result<Vec<f64>, String> {
        let mut args = Vec::new();
        self.skip_whitespace();
        if self.consume(')') {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expression()?);
            self.skip_whitespace();
            if self.consume(')') {
                return Ok(args);
            }
            if !self.consume(',') {
                return Err("missing argument separator".to_string());
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.pos += 1;
        }
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
}

fn evaluate_function(name: &str, args: &[f64]) -> Result<f64, String> {
    match name {
        "abs" if args.len() == 1 => Ok(args[0].abs()),
        "round" if args.len() == 1 => Ok(args[0].round()),
        "round" if args.len() == 2 => Ok(round_to(args[0], args[1].max(0.0) as u32)),
        "min" if !args.is_empty() => Ok(args.iter().copied().fold(f64::INFINITY, f64::min)),
        "max" if !args.is_empty() => Ok(args.iter().copied().fold(f64::NEG_INFINITY, f64::max)),
        "sqrt" if args.len() == 1 && args[0] >= 0.0 => Ok(args[0].sqrt()),
        "pow" if args.len() == 2 => Ok(args[0].powf(args[1])),
        "ceil" if args.len() == 1 => Ok(args[0].ceil()),
        "floor" if args.len() == 1 => Ok(args[0].floor()),
        _ => Err("unknown function".to_string()),
    }
}

fn round_to(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}
