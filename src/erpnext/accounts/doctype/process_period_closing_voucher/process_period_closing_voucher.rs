use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessPeriodClosingVoucher {
    pub amended_from: Option<String>,
    pub bs_closing_balance: Option<String>,
    pub name: Option<String>,
    pub normal_balances: Vec<ProcessPeriodClosingVoucherDetail>,
    pub p_l_closing_balance: Option<String>,
    pub parent_pcv: Option<String>,
    pub status: Option<String>,
    pub z_opening_balances: Vec<ProcessPeriodClosingVoucherDetail>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessPeriodClosingVoucherDetail {
    pub idx: usize,
    pub parentfield: String,
    pub processing_date: String,
    pub report_type: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodClosingVoucherContext {
    pub period_start_date: String,
    pub period_end_date: String,
    pub is_first_period_closing_voucher: bool,
    pub gl_min_posting_date: Option<String>,
    pub gl_max_posting_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProcessPeriodClosingVoucherHook {
    StartProcessing { docname: String },
    CancelProcessing { docname: String },
}

impl ProcessPeriodClosingVoucherDetail {
    pub fn new(
        idx: usize,
        parentfield: impl Into<String>,
        processing_date: impl Into<String>,
        report_type: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            idx,
            parentfield: parentfield.into(),
            processing_date: processing_date.into(),
            report_type: report_type.into(),
            status: status.into(),
        }
    }
}

impl ProcessPeriodClosingVoucher {
    pub const DOCTYPE: &'static str = "Process Period Closing Voucher";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "format:Process-PCV-{###}";
    pub const GRID_PAGE_LENGTH: usize = 50;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "parent_pcv",
        "status",
        "p_l_closing_balance",
        "normal_balances",
        "bs_closing_balance",
        "z_opening_balances",
        "amended_from",
    ];
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;

    pub fn new(parent_pcv: impl Into<String>) -> Self {
        Self {
            parent_pcv: Some(parent_pcv.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("parent_pcv", "PCV")
                .options("Period Closing Voucher")
                .required()
                .in_list_view(),
            FieldSpec::select("status", "Status")
                .options("Queued\nRunning\nPaused\nCompleted\nCancelled")
                .default("Queued")
                .no_copy(),
            FieldSpec::json("p_l_closing_balance", "P&L Closing Balance").no_copy(),
            FieldSpec::table("normal_balances", "Dates to Process")
                .options("Process Period Closing Voucher Detail")
                .no_copy(),
            FieldSpec::json("bs_closing_balance", "Balance Sheet Closing Balance"),
            FieldSpec::table("z_opening_balances", "Opening Balances")
                .options("Process Period Closing Voucher Detail")
                .no_copy(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Period Closing Voucher")
                .no_copy()
                .print_hide()
                .read_only()
                .search_index(),
        ]
    }

    pub fn on_discard(&mut self) {
        self.status = Some("Cancelled".to_string());
    }

    pub fn validate(&mut self, context: &PeriodClosingVoucherContext) -> Result<(), String> {
        self.status = Some("Queued".to_string());
        self.populate_processing_tables(context)
    }

    pub fn populate_processing_tables(
        &mut self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), String> {
        self.generate_pcv_dates(context)?;
        self.generate_opening_balances_dates(context)
    }

    pub fn generate_pcv_dates(
        &mut self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), String> {
        self.normal_balances.clear();

        for date in inclusive_dates(&context.period_start_date, &context.period_end_date)? {
            let next_idx = self.normal_balances.len() + 1;
            self.normal_balances
                .push(ProcessPeriodClosingVoucherDetail::new(
                    next_idx,
                    "normal_balances",
                    date.clone(),
                    "Profit and Loss",
                    "Queued",
                ));
            self.normal_balances
                .push(ProcessPeriodClosingVoucherDetail::new(
                    next_idx + 1,
                    "normal_balances",
                    date,
                    "Balance Sheet",
                    "Queued",
                ));
        }

        Ok(())
    }

    pub fn generate_opening_balances_dates(
        &mut self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), String> {
        self.z_opening_balances.clear();

        if context.is_first_period_closing_voucher {
            let Some(min_date) = &context.gl_min_posting_date else {
                return Ok(());
            };
            let Some(max_date) = &context.gl_max_posting_date else {
                return Ok(());
            };

            for date in inclusive_dates(min_date, max_date)? {
                self.z_opening_balances
                    .push(ProcessPeriodClosingVoucherDetail::new(
                        self.z_opening_balances.len() + 1,
                        "z_opening_balances",
                        date,
                        "Balance Sheet",
                        "Queued",
                    ));
            }
        }

        Ok(())
    }

    pub fn on_submit(&self) -> ProcessPeriodClosingVoucherHook {
        ProcessPeriodClosingVoucherHook::StartProcessing {
            docname: self.parent_pcv.clone().unwrap_or_default(),
        }
    }

    pub fn on_cancel(&self) -> ProcessPeriodClosingVoucherHook {
        ProcessPeriodClosingVoucherHook::CancelProcessing {
            docname: self.parent_pcv.clone().unwrap_or_default(),
        }
    }

    pub fn client_action(docstatus: i32, status: &str) -> Option<&'static str> {
        if docstatus != 1 {
            return None;
        }

        match status {
            "Queued" => Some("Start"),
            "Running" => Some("Pause"),
            "Paused" => Some("Resume"),
            _ => None,
        }
    }

    pub fn progress(
        normal_balances: &[ProcessPeriodClosingVoucherDetail],
        z_opening_balances: &[ProcessPeriodClosingVoucherDetail],
    ) -> Option<f64> {
        let total = normal_balances.len() + z_opening_balances.len();
        if total == 0 {
            return None;
        }

        let completed = normal_balances
            .iter()
            .chain(z_opening_balances.iter())
            .filter(|row| row.status == "Completed")
            .count();

        Some((completed as f64 / total as f64) * 100.0)
    }
}

impl DocumentController for ProcessPeriodClosingVoucher {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["on_discard", "validate", "on_submit", "on_cancel"]
    }
}

fn inclusive_dates(start: &str, end: &str) -> Result<Vec<String>, String> {
    let start_days = parse_date(start)?;
    let end_days = parse_date(end)?;
    if end_days < start_days {
        return Err("End date cannot be before start date".to_string());
    }

    Ok((start_days..=end_days).map(format_date).collect())
}

fn parse_date(date: &str) -> Result<i64, String> {
    let mut parts = date.split('-');
    let year = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {date}"))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid date: {date}"))?;
    let month = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {date}"))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid date: {date}"))?;
    let day = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {date}"))?
        .parse::<i64>()
        .map_err(|_| format!("Invalid date: {date}"))?;

    if parts.next().is_some()
        || !(1..=12).contains(&month)
        || !(1..=days_in_month(year, month)).contains(&day)
    {
        return Err(format!("Invalid date: {date}"));
    }

    Ok(days_from_civil(year, month, day))
}

fn format_date(days: i64) -> String {
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month_prime = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month_prime + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let month_prime = (5 * doy + 2) / 153;
    let day = doy - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };

    (year, month, day)
}
