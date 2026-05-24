pub const GROUP_BY_FIELDS: [&str; 3] = ["party_type", "party", "tax_withholding_category"];
pub const CARRY_OVER_FIELDS: [&str; 7] = [
    "tax_id",
    "party",
    "party_type",
    "party_name",
    "tax_withholding_category",
    "party_entity_type",
    "rate",
];
pub const AGGREGATE_FIELDS: [&str; 2] = ["total_amount", "tax_amount"];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TdsReportError {
    FromDateAfterToDate,
    DifferentFiscalYear,
    FiscalYearNotFound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TdsFilters {
    pub from_date: String,
    pub to_date: String,
    pub party_type: Option<String>,
    pub fiscal_year: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYearSpan {
    pub name: String,
    pub year_start_date: String,
    pub year_end_date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TdsColumn {
    pub label: String,
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub options: &'static str,
    pub width: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TdsRow {
    pub tax_id: String,
    pub party: String,
    pub party_type: String,
    pub party_name: String,
    pub tax_withholding_category: String,
    pub party_entity_type: String,
    pub rate: Option<f64>,
    pub total_amount: Option<f64>,
    pub tax_amount: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TdsComputationSummaryReport {
    pub filters: TdsFilters,
    pub columns: Vec<TdsColumn>,
    pub rows: Vec<TdsRow>,
}

impl TdsFilters {
    pub fn new(from_date: impl Into<String>, to_date: impl Into<String>) -> Self {
        Self {
            from_date: from_date.into(),
            to_date: to_date.into(),
            party_type: None,
            fiscal_year: None,
        }
    }
}

impl FiscalYearSpan {
    pub fn new(
        name: impl Into<String>,
        year_start_date: impl Into<String>,
        year_end_date: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            year_start_date: year_start_date.into(),
            year_end_date: year_end_date.into(),
        }
    }

    fn contains(&self, date: &str) -> bool {
        self.year_start_date.as_str() <= date && date <= self.year_end_date.as_str()
    }
}

impl TdsColumn {
    pub fn data(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Data",
            options: "",
            width,
        }
    }

    pub fn dynamic_link(
        label: impl Into<String>,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Dynamic Link",
            options,
            width,
        }
    }

    pub fn link(
        label: impl Into<String>,
        fieldname: &'static str,
        options: &'static str,
        width: u16,
    ) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Link",
            options,
            width,
        }
    }

    pub fn percent(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Percent",
            options: "",
            width,
        }
    }

    pub fn float(label: impl Into<String>, fieldname: &'static str, width: u16) -> Self {
        Self {
            label: label.into(),
            fieldname,
            fieldtype: "Float",
            options: "",
            width,
        }
    }
}

impl TdsRow {
    pub fn new(
        tax_id: impl Into<String>,
        party_type: impl Into<String>,
        party: impl Into<String>,
        party_name: impl Into<String>,
        tax_withholding_category: impl Into<String>,
        party_entity_type: impl Into<String>,
        rate: Option<f64>,
    ) -> Self {
        Self {
            tax_id: tax_id.into(),
            party: party.into(),
            party_type: party_type.into(),
            party_name: party_name.into(),
            tax_withholding_category: tax_withholding_category.into(),
            party_entity_type: party_entity_type.into(),
            rate,
            total_amount: None,
            tax_amount: None,
        }
    }

    pub fn with_amounts(mut self, total_amount: Option<f64>, tax_amount: Option<f64>) -> Self {
        self.total_amount = total_amount;
        self.tax_amount = tax_amount;
        self
    }
}

pub fn execute(
    mut filters: TdsFilters,
    rows: Vec<TdsRow>,
    fiscal_years: &[FiscalYearSpan],
) -> Result<TdsComputationSummaryReport, TdsReportError> {
    validate_filters(&mut filters, fiscal_years)?;
    let columns = get_columns(&filters);
    Ok(TdsComputationSummaryReport {
        filters,
        columns,
        rows: group_rows(rows),
    })
}

pub fn validate_filters(
    filters: &mut TdsFilters,
    fiscal_years: &[FiscalYearSpan],
) -> Result<(), TdsReportError> {
    if filters.from_date > filters.to_date {
        return Err(TdsReportError::FromDateAfterToDate);
    }

    let from_year = fiscal_year_for(&filters.from_date, fiscal_years)?;
    let to_year = fiscal_year_for(&filters.to_date, fiscal_years)?;
    if from_year != to_year {
        return Err(TdsReportError::DifferentFiscalYear);
    }

    filters.fiscal_year = Some(from_year.to_string());
    Ok(())
}

pub fn group_rows(data: Vec<TdsRow>) -> Vec<TdsRow> {
    let mut grouped: Vec<TdsRow> = Vec::new();

    for row in data {
        let key = (
            row.party_type.as_str(),
            row.party.as_str(),
            row.tax_withholding_category.as_str(),
        );
        if let Some(bucket) = grouped.iter_mut().find(|bucket| {
            (
                bucket.party_type.as_str(),
                bucket.party.as_str(),
                bucket.tax_withholding_category.as_str(),
            ) == key
        }) {
            bucket.total_amount =
                Some(bucket.total_amount.unwrap_or(0.0) + row.total_amount.unwrap_or(0.0));
            bucket.tax_amount =
                Some(bucket.tax_amount.unwrap_or(0.0) + row.tax_amount.unwrap_or(0.0));
        } else {
            let total_amount = row_total_amount_or_zero(&row);
            let tax_amount = row_tax_amount_or_zero(&row);
            grouped.push(row.with_amounts(Some(total_amount), Some(tax_amount)));
        }
    }

    grouped
}

pub fn get_columns(filters: &TdsFilters) -> Vec<TdsColumn> {
    let party_type = filters.party_type.as_deref().unwrap_or("Party");
    vec![
        TdsColumn::data("Tax Id", "tax_id", 90),
        TdsColumn::dynamic_link(party_type, "party", "party_type", 180),
        TdsColumn::data(format!("{party_type} Name"), "party_name", 180),
        TdsColumn::link(
            "Tax Withholding Category",
            "tax_withholding_category",
            "Tax Withholding Category",
            180,
        ),
        TdsColumn::data(format!("{party_type} Type"), "party_entity_type", 180),
        TdsColumn::percent("Tax Rate %", "rate", 120),
        TdsColumn::float("Total Taxable Amount", "total_amount", 120),
        TdsColumn::float("Tax Amount", "tax_amount", 120),
    ]
}

fn fiscal_year_for<'a>(
    date: &str,
    fiscal_years: &'a [FiscalYearSpan],
) -> Result<&'a str, TdsReportError> {
    fiscal_years
        .iter()
        .find(|fiscal_year| fiscal_year.contains(date))
        .map(|fiscal_year| fiscal_year.name.as_str())
        .ok_or(TdsReportError::FiscalYearNotFound)
}

fn row_total_amount_or_zero(row: &TdsRow) -> f64 {
    row.total_amount.unwrap_or(0.0)
}

fn row_tax_amount_or_zero(row: &TdsRow) -> f64 {
    row.tax_amount.unwrap_or(0.0)
}
