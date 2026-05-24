use crate::erpnext::accounts::report::{ReportFilters, TrendReportExecution};

pub fn execute(filters: Option<ReportFilters>) -> TrendReportExecution {
    TrendReportExecution::new("Sales Invoice", filters)
}
