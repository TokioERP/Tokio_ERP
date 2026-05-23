use crate::erpnext::accounts::doctype::subscription_plan_detail::subscription_plan_detail::SubscriptionPlanDetail;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Blank,
    Trialing,
    Active,
    GracePeriod,
    Cancelled,
    Unpaid,
    Completed,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerateInvoiceAt {
    EndOfCurrentPeriod,
    BeginningOfCurrentPeriod,
    DaysBeforeCurrentPeriod,
}

impl Default for GenerateInvoiceAt {
    fn default() -> Self {
        Self::EndOfCurrentPeriod
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BillingCycle {
    pub billing_interval: String,
    pub billing_interval_count: i32,
}

impl BillingCycle {
    pub fn new(billing_interval: impl Into<String>, billing_interval_count: i32) -> Self {
        Self {
            billing_interval: billing_interval.into(),
            billing_interval_count,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BillingCycleDelta {
    pub days: Option<i32>,
    pub months: Option<i32>,
    pub years: Option<i32>,
}

impl BillingCycleDelta {
    pub fn days(days: i32) -> Self {
        Self {
            days: Some(days),
            ..Self::default()
        }
    }

    pub fn months_with_days(months: i32, days: i32) -> Self {
        Self {
            days: Some(days),
            months: Some(months),
            ..Self::default()
        }
    }

    pub fn years_with_days(years: i32, days: i32) -> Self {
        Self {
            days: Some(days),
            years: Some(years),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionError {
    MixedBillingCycles,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subscription {
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub company: Option<String>,
    pub status: SubscriptionStatus,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub cancelation_date: Option<String>,
    pub trial_period_start: Option<String>,
    pub trial_period_end: Option<String>,
    pub follow_calendar_months: bool,
    pub generate_new_invoices_past_due_date: bool,
    pub submit_invoice: bool,
    pub current_invoice_start: Option<String>,
    pub current_invoice_end: Option<String>,
    pub days_until_due: i32,
    pub generate_invoice_at: GenerateInvoiceAt,
    pub number_of_days: i32,
    pub cancel_at_period_end: bool,
    pub plans: Vec<SubscriptionPlanDetail>,
    pub sales_tax_template: Option<String>,
    pub purchase_tax_template: Option<String>,
    pub apply_additional_discount: Option<String>,
    pub additional_discount_percentage: f64,
    pub additional_discount_amount: f64,
    pub cost_center: Option<String>,
    pub billing_cycle: Option<BillingCycle>,
}

impl Default for Subscription {
    fn default() -> Self {
        Self {
            party_type: None,
            party: None,
            company: None,
            status: SubscriptionStatus::Blank,
            start_date: None,
            end_date: None,
            cancelation_date: None,
            trial_period_start: None,
            trial_period_end: None,
            follow_calendar_months: false,
            generate_new_invoices_past_due_date: false,
            submit_invoice: true,
            current_invoice_start: None,
            current_invoice_end: None,
            days_until_due: 0,
            generate_invoice_at: GenerateInvoiceAt::EndOfCurrentPeriod,
            number_of_days: 0,
            cancel_at_period_end: false,
            plans: Vec::new(),
            sales_tax_template: None,
            purchase_tax_template: None,
            apply_additional_discount: None,
            additional_discount_percentage: 0.0,
            additional_discount_amount: 0.0,
            cost_center: None,
            billing_cycle: None,
        }
    }
}

impl Subscription {
    pub const DOCTYPE: &'static str = "Subscription";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-SUB-.YYYY.-.#####";
    pub const FIELD_ORDER: [&'static str; 33] = [
        "party_type",
        "party",
        "cb_1",
        "company",
        "status",
        "subscription_period",
        "start_date",
        "end_date",
        "cancelation_date",
        "trial_period_start",
        "trial_period_end",
        "follow_calendar_months",
        "generate_new_invoices_past_due_date",
        "submit_invoice",
        "column_break_11",
        "current_invoice_start",
        "current_invoice_end",
        "days_until_due",
        "generate_invoice_at",
        "number_of_days",
        "cancel_at_period_end",
        "sb_4",
        "plans",
        "sb_1",
        "sales_tax_template",
        "purchase_tax_template",
        "sb_2",
        "apply_additional_discount",
        "cb_2",
        "additional_discount_percentage",
        "additional_discount_amount",
        "accounting_dimensions_section",
        "cost_center",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        party_type: impl Into<String>,
        party: impl Into<String>,
        start_date: impl Into<String>,
    ) -> Self {
        Self {
            party_type: Some(party_type.into()),
            party: Some(party.into()),
            start_date: Some(start_date.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::column_break("cb_1").allow_on_submit(),
            FieldSpec::select("status", "Status")
                .options("\nTrialing\nActive\nGrace Period\nCancelled\nUnpaid\nCompleted")
                .read_only()
                .no_copy(),
            FieldSpec::section_break("subscription_period").label("Subscription Period"),
            FieldSpec::date("cancelation_date", "Cancelation Date").read_only(),
            FieldSpec::date("trial_period_start", "Trial Period Start Date").allow_on_submit(),
            FieldSpec::date("trial_period_end", "Trial Period End Date")
                .depends_on("eval:doc.trial_period_start"),
            FieldSpec::column_break("column_break_11"),
            FieldSpec::date("current_invoice_start", "Current Invoice Start Date")
                .read_only()
                .no_copy(),
            FieldSpec::date("current_invoice_end", "Current Invoice End Date")
                .read_only()
                .no_copy(),
            FieldSpec::int("days_until_due", "Days Until Due")
                .default("0")
                .description("Number of days that the subscriber has to pay invoices generated by this subscription"),
            FieldSpec::check("cancel_at_period_end", "Cancel At End Of Period").default("0"),
            FieldSpec::section_break("sb_4")
                .label("Plans")
                .allow_on_submit(),
            FieldSpec::table("plans", "Plans")
                .options("Subscription Plan Detail")
                .required()
                .allow_on_submit(),
            FieldSpec::section_break("sb_1")
                .label("Taxes")
                .depends_on("eval:['Customer', 'Supplier'].includes(doc.party_type)"),
            FieldSpec::section_break("sb_2").label("Discounts"),
            FieldSpec::select("apply_additional_discount", "Apply Additional Discount On")
                .options("\nGrand Total\nNet Total"),
            FieldSpec::column_break("cb_2"),
            FieldSpec::percent(
                "additional_discount_percentage",
                "Additional Discount Percentage",
            ),
            FieldSpec::currency("additional_discount_amount", "Additional Discount Amount")
                .collapsible(),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions")
                .collapsible(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .required(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .required()
                .in_list_view(),
            FieldSpec::link("sales_tax_template", "Sales Taxes and Charges Template")
                .options("Sales Taxes and Charges Template")
                .depends_on("eval:doc.party_type === 'Customer'"),
            FieldSpec::link(
                "purchase_tax_template",
                "Purchase Taxes and Charges Template",
            )
            .options("Purchase Taxes and Charges Template")
            .depends_on("eval:doc.party_type === 'Supplier'"),
            FieldSpec::check("follow_calendar_months", "Follow Calendar Months")
                .default("0")
                .description("If this is checked subsequent new invoices will be created on calendar  month and quarter start dates irrespective of current invoice start date"),
            FieldSpec::check(
                "generate_new_invoices_past_due_date",
                "Generate New Invoices Past Due Date",
            )
            .default("0")
            .description("New invoices will be generated as per schedule even if current invoices are unpaid or past due date"),
            FieldSpec::date("end_date", "Subscription End Date"),
            FieldSpec::date("start_date", "Subscription Start Date"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::check("submit_invoice", "Submit Generated Invoices").default("1"),
            FieldSpec::select("generate_invoice_at", "Generate Invoice At")
                .options("End of the current subscription period\nBeginning of the current subscription period\nDays before the current subscription period")
                .default("End of the current subscription period")
                .required(),
            FieldSpec::int("number_of_days", "Number of Days")
                .depends_on("eval:doc.generate_invoice_at === \"Days before the current subscription period\"")
                .mandatory_depends_on("eval:doc.generate_invoice_at === \"Days before the current subscription period\""),
        ]
    }

    pub fn before_insert(&mut self, now_date: &str) {
        self.update_subscription_period(self.start_date.clone().as_deref(), now_date);
    }

    pub fn update_subscription_period(&mut self, date: Option<&str>, now_date: &str) {
        let start = self.get_current_invoice_start(date, now_date);
        let end = self.get_current_invoice_end(&start, now_date);
        self.current_invoice_start = Some(start);
        self.current_invoice_end = Some(end);
    }

    pub fn get_current_invoice_start(&self, date: Option<&str>, now_date: &str) -> String {
        if self
            .trial_period_end
            .as_deref()
            .zip(self.start_date.as_deref())
            .is_some_and(|(trial_end, start_date)| parse_date(trial_end) > parse_date(start_date))
        {
            return add_days(self.trial_period_end.as_deref().unwrap(), 1);
        }

        if let Some(date) = date {
            return date.to_string();
        }

        if self.trial_period_start.is_some() && self.is_trialling(now_date) {
            return self.trial_period_start.clone().unwrap();
        }

        now_date.to_string()
    }

    pub fn get_current_invoice_end(&self, date: &str, now_date: &str) -> String {
        if self.is_trialling(now_date)
            && self
                .trial_period_end
                .as_deref()
                .is_some_and(|trial_end| parse_date(date) < parse_date(trial_end))
        {
            return self.trial_period_end.clone().unwrap();
        }

        let mut current_invoice_end = if let Some(cycle) = self.billing_cycle.as_ref() {
            let cycle_delta = Self::billing_cycle_data_for(cycle.clone());
            let mut end = if self
                .start_date
                .as_deref()
                .is_some_and(|start_date| parse_date(start_date) < parse_date(date))
            {
                add_delta(self.start_date.as_deref().unwrap(), &cycle_delta)
            } else {
                add_delta(date, &cycle_delta)
            };

            let previous_invoice_end = self.current_invoice_end.as_deref().unwrap_or(now_date);
            if parse_date(previous_invoice_end) < parse_date(date) {
                end = add_delta(date, &cycle_delta);
            }

            end
        } else {
            get_last_day(date)
        };

        if self.follow_calendar_months {
            if let Some(cycle) = self.billing_cycle.as_ref() {
                let shifted = add_months(date, cycle.billing_interval_count - 1);
                current_invoice_end = get_last_day(&shifted);
            }
        }

        if self
            .end_date
            .as_deref()
            .is_some_and(|end_date| parse_date(&current_invoice_end) > parse_date(end_date))
        {
            current_invoice_end = self.end_date.clone().unwrap();
        }

        current_invoice_end
    }

    pub fn validate_plans_billing_cycle(
        billing_cycle_data: &[BillingCycle],
    ) -> Result<(), SubscriptionError> {
        if billing_cycle_data.len() > 1 {
            Err(SubscriptionError::MixedBillingCycles)
        } else {
            Ok(())
        }
    }

    pub fn billing_cycle_data_for(cycle: BillingCycle) -> BillingCycleDelta {
        match cycle.billing_interval.as_str() {
            "Day" => BillingCycleDelta::days(cycle.billing_interval_count - 1),
            "Week" => BillingCycleDelta::days(cycle.billing_interval_count * 7 - 1),
            "Month" => BillingCycleDelta::months_with_days(cycle.billing_interval_count, -1),
            "Year" => BillingCycleDelta::years_with_days(cycle.billing_interval_count, -1),
            _ => BillingCycleDelta::default(),
        }
    }

    pub fn is_trialling(&self, posting_date: &str) -> bool {
        !Self::period_has_passed(self.trial_period_end.as_deref(), posting_date)
    }

    pub fn period_has_passed(end_date: Option<&str>, posting_date: &str) -> bool {
        match end_date {
            Some(end_date) => parse_date(posting_date) > parse_date(end_date),
            None => true,
        }
    }

    pub fn can_generate_new_invoice(
        &self,
        posting_date: &str,
        has_outstanding_invoice: bool,
    ) -> bool {
        if self.cancelation_date.is_some() {
            return false;
        }

        if has_outstanding_invoice && !self.generate_new_invoices_past_due_date {
            return false;
        }

        let current_start = self.current_invoice_start.as_deref().unwrap_or("");
        let current_end = self.current_invoice_end.as_deref().unwrap_or("");

        match self.generate_invoice_at {
            GenerateInvoiceAt::BeginningOfCurrentPeriod => {
                parse_date(posting_date) == parse_date(current_start)
            }
            GenerateInvoiceAt::DaysBeforeCurrentPeriod => {
                parse_date(posting_date)
                    == parse_date(&add_days(current_start, -self.number_of_days))
            }
            GenerateInvoiceAt::EndOfCurrentPeriod => {
                parse_date(posting_date) == parse_date(current_end)
            }
        }
    }
}

impl DocumentController for Subscription {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "before_insert",
            "validate",
            "process",
            "cancel_subscription",
            "restart_subscription",
            "force_fetch_subscription_updates",
        ]
    }
}

pub fn get_prorata_factor_at(
    period_end: &str,
    period_start: &str,
    is_prepaid: Option<i32>,
    now_date: &str,
) -> f64 {
    if is_prepaid.unwrap_or(0) != 0 {
        return 1.0;
    }

    let diff = f64::from(date_diff(now_date, period_start) + 1);
    let plan_days = f64::from(date_diff(period_end, period_start) + 1);
    diff / plan_days
}

fn add_delta(date: &str, delta: &BillingCycleDelta) -> String {
    let mut result = date.to_string();
    if let Some(years) = delta.years {
        result = add_months(&result, years * 12);
    }
    if let Some(months) = delta.months {
        result = add_months(&result, months);
    }
    if let Some(days) = delta.days {
        result = add_days(&result, days);
    }
    result
}

fn add_days(date: &str, days: i32) -> String {
    SimpleDate::from_ordinal(parse_date(date).ordinal() + days).to_string()
}

fn add_months(date: &str, months: i32) -> String {
    let date = parse_date(date);
    let total_months = date.year * 12 + i32::from(date.month) - 1 + months;
    let year = total_months.div_euclid(12);
    let month = total_months.rem_euclid(12) as u8 + 1;
    let day = date.day.min(days_in_month(year, month));
    SimpleDate { year, month, day }.to_string()
}

fn get_last_day(date: &str) -> String {
    let date = parse_date(date);
    SimpleDate {
        day: days_in_month(date.year, date.month),
        ..date
    }
    .to_string()
}

fn date_diff(left: &str, right: &str) -> i32 {
    parse_date(left).ordinal() - parse_date(right).ordinal()
}

fn parse_date(value: &str) -> SimpleDate {
    SimpleDate::parse(value)
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => panic!("invalid month"),
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SimpleDate {
    year: i32,
    month: u8,
    day: u8,
}

impl SimpleDate {
    fn parse(value: &str) -> Self {
        let mut parts = value.split('-');
        let year = parts
            .next()
            .and_then(|part| part.parse::<i32>().ok())
            .expect("invalid date year");
        let month = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .expect("invalid date month");
        let day = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .expect("invalid date day");
        Self { year, month, day }
    }

    fn ordinal(self) -> i32 {
        days_from_civil(self.year, self.month, self.day)
    }

    fn from_ordinal(days: i32) -> Self {
        let z = days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = mp + if mp < 10 { 3 } else { -9 };
        let year = y + if month <= 2 { 1 } else { 0 };
        Self {
            year,
            month: month as u8,
            day: day as u8,
        }
    }
}

impl std::fmt::Display for SimpleDate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}

fn days_from_civil(year: i32, month: u8, day: u8) -> i32 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = i32::from(month);
    let day = i32::from(day);
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}
