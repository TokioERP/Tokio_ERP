use std::collections::BTreeMap;

use serde_json::Value;

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EngineFilterValidation {
    pub missing_required: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct FinancialReportEngine;

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
