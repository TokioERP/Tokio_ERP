use crate::erpnext::accounts::report::{DelegatedReportExecution, ReportArg, ReportFilters};

pub fn execute(filters: Option<ReportFilters>) -> DelegatedReportExecution {
    DelegatedReportExecution {
        delegate: "ReceivablePayableReport",
        filters,
        args: vec![
            ("account_type", ReportArg::Text("Payable")),
            (
                "naming_by",
                ReportArg::List(&["Buying Settings", "supp_master_name"]),
            ),
        ],
    }
}
