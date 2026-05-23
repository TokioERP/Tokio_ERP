use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SalesInvoiceTimesheet {
    pub activity_type: Option<String>,
    pub description: Option<String>,
    pub from_time: Option<String>,
    pub to_time: Option<String>,
    pub billing_hours: Option<String>,
    pub billing_amount: Option<String>,
    pub time_sheet: Option<String>,
    pub timesheet_detail: Option<String>,
    pub project_name: Option<String>,
}

impl SalesInvoiceTimesheet {
    pub const DOCTYPE: &'static str = "Sales Invoice Timesheet";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 15] = [
        "activity_type",
        "description",
        "section_break_3",
        "from_time",
        "column_break_5",
        "to_time",
        "section_break_7",
        "billing_hours",
        "column_break_9",
        "billing_amount",
        "section_break_11",
        "time_sheet",
        "timesheet_detail",
        "column_break_13",
        "project_name",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(time_sheet: impl Into<String>, timesheet_detail: impl Into<String>) -> Self {
        Self {
            time_sheet: Some(time_sheet.into()),
            timesheet_detail: Some(timesheet_detail.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("activity_type", "Activity Type")
                .options("Activity Type")
                .read_only()
                .in_list_view(),
            FieldSpec::small_text("description", "Description")
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_3").label("Time"),
            FieldSpec::datetime("from_time", "From Time"),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::datetime("to_time", "To Time"),
            FieldSpec::section_break("section_break_7").label("Totals"),
            FieldSpec::float("billing_hours", "Billing Hours")
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::currency("billing_amount", "Billing Amount")
                .options("currency")
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_11").label("Reference"),
            FieldSpec::link("time_sheet", "Time Sheet")
                .options("Timesheet")
                .read_only()
                .in_list_view(),
            FieldSpec::data("timesheet_detail", "Timesheet Detail")
                .read_only()
                .hidden()
                .print_hide()
                .allow_on_submit(),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::data("project_name", "Project Name").read_only(),
        ]
    }
}

impl DocumentController for SalesInvoiceTimesheet {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
