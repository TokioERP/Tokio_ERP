#[derive(Clone, Debug, PartialEq)]
pub struct GlEntry {
    pub posting_date: String,
    pub debit: f64,
    pub credit: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountBalanceTimelineResult {
    pub date: String,
    pub balance: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlEntriesPlan {
    pub doctype: &'static str,
    pub fields: [String; 3],
    pub to_date_filter: (String, String, String),
    pub account_filter: (String, String, Vec<String>),
    pub voucher_type_filter: (String, String, String),
    pub order_by: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountBalanceTimelineDataset {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccountBalanceTimelineChart {
    pub labels: Vec<String>,
    pub datasets: Vec<AccountBalanceTimelineDataset>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountBalanceTimelineRequest {
    pub chart_name: Option<String>,
    pub account: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub timespan: String,
    pub time_interval: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ErpDate {
    year: i32,
    month: u32,
    day: u32,
}

impl GlEntry {
    pub fn new(posting_date: impl Into<String>, debit: f64, credit: f64) -> Self {
        Self {
            posting_date: posting_date.into(),
            debit,
            credit,
        }
    }
}

impl AccountBalanceTimelineResult {
    pub fn new(date: impl Into<String>, balance: f64) -> Self {
        Self {
            date: date.into(),
            balance,
        }
    }
}

pub fn get_dates_from_timegrain(from_date: &str, to_date: &str, timegrain: &str) -> Vec<String> {
    let mut dates = vec![period_ending(parse_date(from_date), timegrain)];
    let to_date = parse_date(to_date);

    while *dates.last().expect("dates are initialized") < to_date {
        let date = match timegrain {
            "Daily" => period_ending(dates.last().copied().unwrap().add_days(1), timegrain),
            "Weekly" => period_ending(dates.last().copied().unwrap().add_days(7), timegrain),
            "Monthly" => period_ending(dates.last().copied().unwrap().add_months(1), timegrain),
            "Quarterly" => period_ending(dates.last().copied().unwrap().add_months(3), timegrain),
            _ => panic!("unsupported timegrain: {timegrain}"),
        };
        dates.push(date);
    }

    dates.into_iter().map(|date| date.to_iso()).collect()
}

pub fn build_result(
    _account: &str,
    root_type: &str,
    dates: &[String],
    gl_entries: &[GlEntry],
) -> Vec<AccountBalanceTimelineResult> {
    let mut result: Vec<(ErpDate, f64)> =
        dates.iter().map(|date| (parse_date(date), 0.0)).collect();
    let mut date_index = 0;

    for entry in gl_entries {
        let posting_date = parse_date(&entry.posting_date);
        while posting_date > result[date_index].0 {
            date_index += 1;
        }

        result[date_index].1 += entry.debit - entry.credit;
    }

    if !matches!(root_type, "Asset" | "Expense") {
        for row in &mut result {
            row.1 *= -1.0;
        }
    }

    if matches!(root_type, "Asset" | "Liability" | "Equity") {
        for index in 1..result.len() {
            result[index].1 += result[index - 1].1;
        }
    }

    result
        .into_iter()
        .map(|(date, balance)| AccountBalanceTimelineResult::new(date.to_iso(), balance))
        .collect()
}

pub fn build_chart(
    account: &str,
    rows: &[AccountBalanceTimelineResult],
) -> AccountBalanceTimelineChart {
    AccountBalanceTimelineChart {
        labels: rows.iter().map(|row| row.date.clone()).collect(),
        datasets: vec![AccountBalanceTimelineDataset {
            name: account.to_string(),
            values: rows.iter().map(|row| row.balance).collect(),
        }],
    }
}

pub fn build_account_balance_timeline_chart(
    account: &str,
    root_type: &str,
    dates: &[String],
    gl_entries: &[GlEntry],
) -> AccountBalanceTimelineChart {
    let rows = build_result(account, root_type, dates, gl_entries);
    build_chart(account, &rows)
}

pub fn get_gl_entries_plan(account: &str, to_date: &str, descendants: &[String]) -> GlEntriesPlan {
    let mut child_accounts = descendants.to_vec();
    child_accounts.push(account.to_string());

    GlEntriesPlan {
        doctype: "GL Entry",
        fields: [
            "posting_date".to_string(),
            "debit".to_string(),
            "credit".to_string(),
        ],
        to_date_filter: (
            "posting_date".to_string(),
            "<".to_string(),
            to_date.to_string(),
        ),
        account_filter: ("account".to_string(), "in".to_string(), child_accounts),
        voucher_type_filter: (
            "voucher_type".to_string(),
            "!=".to_string(),
            "Period Closing Voucher".to_string(),
        ),
        order_by: "posting_date asc",
    }
}

impl AccountBalanceTimelineRequest {
    pub fn validate(&self, account_exists: bool) -> Result<(), String> {
        if self.account.is_none() {
            if let Some(chart_name) = &self.chart_name {
                return Err(format!(
                    "Account is not set for the dashboard chart {}",
                    dashboard_chart_link(chart_name)
                ));
            }
        }

        if !account_exists {
            if let (Some(account), Some(chart_name)) = (&self.account, &self.chart_name) {
                return Err(format!(
                    "Account {} does not exists in the dashboard chart {}",
                    account,
                    dashboard_chart_link(chart_name)
                ));
            }
        }

        Ok(())
    }
}

fn dashboard_chart_link(chart_name: &str) -> String {
    format!("Dashboard Chart/{chart_name}")
}

fn parse_date(value: &str) -> ErpDate {
    let mut parts = value.split('-');
    let year = parts.next().unwrap().parse().unwrap();
    let month = parts.next().unwrap().parse().unwrap();
    let day = parts.next().unwrap().parse().unwrap();

    ErpDate { year, month, day }
}

fn period_ending(date: ErpDate, timegrain: &str) -> ErpDate {
    match timegrain {
        "Daily" => date,
        "Weekly" => date.add_days(6 - date.sunday_based_weekday() as i32),
        "Monthly" => ErpDate {
            day: last_day_of_month(date.year, date.month),
            ..date
        },
        "Quarterly" => {
            let quarter_end_month = match date.month {
                1..=3 => 3,
                4..=6 => 6,
                7..=9 => 9,
                10..=12 => 12,
                _ => panic!("invalid month: {}", date.month),
            };
            ErpDate {
                year: date.year,
                month: quarter_end_month,
                day: last_day_of_month(date.year, quarter_end_month),
            }
        }
        _ => panic!("unsupported timegrain: {timegrain}"),
    }
}

impl ErpDate {
    fn to_iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    fn add_days(self, days: i32) -> Self {
        let mut date = self;
        if days >= 0 {
            for _ in 0..days {
                date = date.next_day();
            }
        } else {
            for _ in days..0 {
                date = date.previous_day();
            }
        }
        date
    }

    fn add_months(self, months: i32) -> Self {
        let zero_based = self.month as i32 - 1 + months;
        let year = self.year + zero_based.div_euclid(12);
        let month = zero_based.rem_euclid(12) as u32 + 1;
        let day = self.day.min(last_day_of_month(year, month));

        Self { year, month, day }
    }

    fn next_day(self) -> Self {
        let month_last_day = last_day_of_month(self.year, self.month);
        if self.day < month_last_day {
            Self {
                day: self.day + 1,
                ..self
            }
        } else if self.month < 12 {
            Self {
                year: self.year,
                month: self.month + 1,
                day: 1,
            }
        } else {
            Self {
                year: self.year + 1,
                month: 1,
                day: 1,
            }
        }
    }

    fn previous_day(self) -> Self {
        if self.day > 1 {
            Self {
                day: self.day - 1,
                ..self
            }
        } else if self.month > 1 {
            let month = self.month - 1;
            Self {
                year: self.year,
                month,
                day: last_day_of_month(self.year, month),
            }
        } else {
            Self {
                year: self.year - 1,
                month: 12,
                day: 31,
            }
        }
    }

    fn sunday_based_weekday(self) -> u32 {
        let (mut month, mut year) = (self.month as i32, self.year);
        if month < 3 {
            month += 12;
            year -= 1;
        }
        let day = self.day as i32;
        let k = year % 100;
        let j = year / 100;
        let h = (day + ((13 * (month + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;
        ((h + 6) % 7) as u32
    }
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => panic!("invalid month: {month}"),
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
