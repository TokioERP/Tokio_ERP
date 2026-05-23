use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessPeriodClosingVoucherDetail {
    pub closing_balance: Option<String>,
    pub processing_date: Option<String>,
    pub report_type: Option<String>,
    pub status: Option<String>,
}

impl Default for ProcessPeriodClosingVoucherDetail {
    fn default() -> Self {
        Self {
            closing_balance: None,
            processing_date: None,
            report_type: Some("Profit and Loss".to_string()),
            status: Some("Queued".to_string()),
        }
    }
}

impl ProcessPeriodClosingVoucherDetail {
    pub const DOCTYPE: &'static str = "Process Period Closing Voucher Detail";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] = [
        "processing_date",
        "report_type",
        "status",
        "closing_balance",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_TABLE: bool = true;
    pub const GRID_PAGE_LENGTH: usize = 50;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const ROWS_THRESHOLD_FOR_GRID_SEARCH: usize = 20;

    pub fn new(
        processing_date: impl Into<String>,
        report_type: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            processing_date: Some(processing_date.into()),
            report_type: Some(report_type.into()),
            status: Some(status.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("processing_date", "Processing Date").in_list_view(),
            FieldSpec::select("report_type", "Report Type")
                .options("Profit and Loss\nBalance Sheet")
                .default("Profit and Loss")
                .in_list_view(),
            FieldSpec::select("status", "Status")
                .options("Queued\nRunning\nPaused\nCompleted\nCancelled")
                .default("Queued")
                .in_list_view(),
            FieldSpec::json("closing_balance", "Closing Balance").in_list_view(),
        ]
    }
}

impl DocumentController for ProcessPeriodClosingVoucherDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
