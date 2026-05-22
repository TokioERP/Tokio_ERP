use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessPaymentReconciliationLog {
    pub allocated: bool,
    pub allocations: Vec<String>,
    pub error_log: Option<String>,
    pub process_pr: Option<String>,
    pub reconciled: bool,
    pub reconciled_entries: i64,
    pub status: Option<String>,
    pub total_allocations: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListIndicator {
    pub status: String,
    pub color: &'static str,
    pub filter: String,
}

impl ProcessPaymentReconciliationLog {
    pub const DOCTYPE: &'static str = "Process Payment Reconciliation Log";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "format:PPR-LOG-{##}";
    pub const SEARCH_FIELDS: &'static str =
        "allocated, reconciled, total_allocations, reconciled_entries";
    pub const FIELD_ORDER: [&'static str; 13] = [
        "process_pr",
        "section_break_fvdw",
        "status",
        "tasks_section",
        "allocated",
        "reconciled",
        "column_break_yhin",
        "total_allocations",
        "reconciled_entries",
        "section_break_4ywv",
        "error_log",
        "allocations_section",
        "allocations",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const IN_CREATE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;

    pub fn new(process_pr: impl Into<String>) -> Self {
        Self {
            process_pr: Some(process_pr.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("process_pr", "Parent Document")
                .options("Process Payment Reconciliation")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_fvdw").label("Status"),
            FieldSpec::select("status", "Status")
                .options("Running\nPaused\nReconciled\nPartially Reconciled\nFailed\nCancelled")
                .read_only(),
            FieldSpec::section_break("tasks_section").label("Tasks"),
            FieldSpec::check("allocated", "Allocated")
                .default("0")
                .description("Invoices and Payments have been Fetched and Allocated")
                .read_only(),
            FieldSpec::check("reconciled", "Reconciled")
                .default("0")
                .description("All allocations have been successfully reconciled")
                .read_only(),
            FieldSpec::column_break("column_break_yhin"),
            FieldSpec::int("total_allocations", "Total Allocations")
                .read_only()
                .in_list_view(),
            FieldSpec::int("reconciled_entries", "Reconciled Entries")
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_4ywv"),
            FieldSpec::long_text("error_log", "Reconciliation Error Log")
                .depends_on("eval:doc.error_log")
                .read_only(),
            FieldSpec::section_break("allocations_section").label("Allocations"),
            FieldSpec::table("allocations", "Allocations")
                .options("Process Payment Reconciliation Log Allocations")
                .read_only(),
        ]
    }

    pub fn progress(status: &str, reconciled_entries: i64, total_allocations: i64) -> Option<f64> {
        if !matches!(
            status,
            "Completed" | "Running" | "Paused" | "Partially Reconciled"
        ) {
            return None;
        }

        if reconciled_entries != 0 {
            Some((reconciled_entries as f64 / total_allocations as f64) * 100.0)
        } else if total_allocations == 0 && status == "Completed" {
            Some(100.0)
        } else {
            Some(0.0)
        }
    }

    pub fn indicator_for_status(status: &str) -> Option<ListIndicator> {
        let color = match status {
            "Partially Reconciled" | "Paused" => "orange",
            "Reconciled" => "green",
            "Failed" | "Cancelled" => "red",
            "Running" => "blue",
            _ => return None,
        };

        Some(ListIndicator {
            status: status.to_string(),
            color,
            filter: format!("status,=,{status}"),
        })
    }
}

impl DocumentController for ProcessPaymentReconciliationLog {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
