use crate::erpnext::accounts::report::{DelegatedReportExecution, ReportArg, ReportFilters};

pub fn execute(filters: Option<ReportFilters>) -> DelegatedReportExecution {
    DelegatedReportExecution {
        delegate: "PartyLedgerSummaryReport",
        filters,
        args: vec![
            ("party_type", ReportArg::Text("Supplier")),
            (
                "naming_by",
                ReportArg::List(&["Buying Settings", "supp_master_name"]),
            ),
        ],
    }
}
