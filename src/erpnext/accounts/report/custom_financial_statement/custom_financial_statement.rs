pub const XLSX_STYLES_HOOK: &str = "get_xlsx_styles";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomFinancialStatementFilters {
    pub report_template: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomFinancialStatementExecution {
    pub delegate: &'static str,
    pub method: &'static str,
    pub filters: CustomFinancialStatementFilters,
}

pub fn execute(
    filters: Option<CustomFinancialStatementFilters>,
) -> Option<CustomFinancialStatementExecution> {
    let filters = filters?;
    if !filters
        .report_template
        .as_deref()
        .is_some_and(|template| !template.is_empty())
    {
        return None;
    }

    Some(CustomFinancialStatementExecution {
        delegate: "FinancialReportEngine",
        method: "execute",
        filters,
    })
}
