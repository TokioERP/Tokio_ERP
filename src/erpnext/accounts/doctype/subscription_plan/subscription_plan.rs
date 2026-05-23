use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BillingInterval {
    Day,
    Week,
    Month,
    Year,
}

impl Default for BillingInterval {
    fn default() -> Self {
        Self::Day
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PriceDetermination {
    None,
    FixedRate,
    BasedOnPriceList,
    MonthlyRate,
}

impl Default for PriceDetermination {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubscriptionPlan {
    pub plan_name: Option<String>,
    pub currency: Option<String>,
    pub item: Option<String>,
    pub price_determination: PriceDetermination,
    pub cost: f64,
    pub price_list: Option<String>,
    pub billing_interval: BillingInterval,
    pub billing_interval_count: i32,
    pub product_price_id: Option<String>,
    pub payment_gateway: Option<String>,
    pub cost_center: Option<String>,
}

impl Default for SubscriptionPlan {
    fn default() -> Self {
        Self {
            plan_name: None,
            currency: None,
            item: None,
            price_determination: PriceDetermination::None,
            cost: 0.0,
            price_list: None,
            billing_interval: BillingInterval::Day,
            billing_interval_count: 1,
            product_price_id: None,
            payment_gateway: None,
            cost_center: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionPlanValidationError {
    BillingIntervalCountLessThanOne,
}

impl SubscriptionPlanValidationError {
    pub fn message(&self) -> &'static str {
        match self {
            Self::BillingIntervalCountLessThanOne => "Billing Interval Count cannot be less than 1",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubscriptionPlanRateInput {
    pub quantity: f64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub prorate_factor: f64,
    pub price_list_rate: Option<f64>,
    pub settings_prorate: bool,
}

impl Default for SubscriptionPlanRateInput {
    fn default() -> Self {
        Self {
            quantity: 1.0,
            start_date: None,
            end_date: None,
            prorate_factor: 1.0,
            price_list_rate: None,
            settings_prorate: false,
        }
    }
}

impl SubscriptionPlanRateInput {
    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    pub fn prorate_factor(mut self, prorate_factor: f64) -> Self {
        self.prorate_factor = prorate_factor;
        self
    }

    pub fn price_list_rate(mut self, price_list_rate: Option<f64>) -> Self {
        self.price_list_rate = price_list_rate;
        self
    }

    pub fn settings_prorate(mut self, settings_prorate: bool) -> Self {
        self.settings_prorate = settings_prorate;
        self
    }
}

impl SubscriptionPlan {
    pub const DOCTYPE: &'static str = "Subscription Plan";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: Option<&'static str> = Some("field:plan_name");
    pub const FIELD_ORDER: [&'static str; 20] = [
        "plan_name",
        "currency",
        "column_break_3",
        "item",
        "section_break_5",
        "price_determination",
        "column_break_7",
        "cost",
        "price_list",
        "section_break_11",
        "billing_interval",
        "column_break_13",
        "billing_interval_count",
        "payment_plan_section",
        "product_price_id",
        "column_break_16",
        "payment_gateway",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        plan_name: impl Into<String>,
        currency: impl Into<String>,
        item: impl Into<String>,
    ) -> Self {
        Self {
            plan_name: Some(plan_name.into()),
            currency: Some(currency.into()),
            item: Some(item.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("plan_name", "Plan Name")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .required(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("item", "Item")
                .options("Item")
                .required()
                .in_list_view(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::select("price_determination", "Subscription Price Based On")
                .options("\nFixed Rate\nBased On Price List\nMonthly Rate")
                .required(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::currency("cost", "Cost")
                .options("currency")
                .depends_on("eval:['Fixed Rate', 'Monthly Rate'].includes(doc.price_determination)")
                .in_list_view(),
            FieldSpec::link("price_list", "Price List")
                .options("Price List")
                .depends_on("eval:doc.price_determination==\"Based On Price List\""),
            FieldSpec::section_break("section_break_11"),
            FieldSpec::select("billing_interval", "Billing Interval")
                .options("Day\nWeek\nMonth\nYear")
                .default("Day")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::int("billing_interval_count", "Billing Interval Count")
                .default("1")
                .description("Number of intervals for the interval field e.g if Interval is 'Days' and Billing Interval Count is 3, invoices will be generated every 3 days")
                .required(),
            FieldSpec::section_break("payment_plan_section").label("Payment Plan"),
            FieldSpec::data("product_price_id", "Product Price ID"),
            FieldSpec::column_break("column_break_16"),
            FieldSpec::link("payment_gateway", "Payment Gateway")
                .options("Payment Gateway Account"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions")
                .collapsible(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::column_break("dimension_col_break"),
        ]
    }

    pub fn validate(&self) -> Result<(), SubscriptionPlanValidationError> {
        self.validate_interval_count()
    }

    pub fn validate_interval_count(&self) -> Result<(), SubscriptionPlanValidationError> {
        if self.billing_interval_count < 1 {
            Err(SubscriptionPlanValidationError::BillingIntervalCountLessThanOne)
        } else {
            Ok(())
        }
    }
}

impl DocumentController for SubscriptionPlan {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

pub fn get_plan_rate(plan: &SubscriptionPlan, input: SubscriptionPlanRateInput) -> f64 {
    match plan.price_determination {
        PriceDetermination::FixedRate => plan.cost * input.prorate_factor,
        PriceDetermination::BasedOnPriceList => {
            input.price_list_rate.unwrap_or(0.0) * input.prorate_factor
        }
        PriceDetermination::MonthlyRate => {
            let start_date = SimpleDate::parse(
                input
                    .start_date
                    .as_deref()
                    .expect("start_date is required for Monthly Rate"),
            );
            let end_date = SimpleDate::parse(
                input
                    .end_date
                    .as_deref()
                    .expect("end_date is required for Monthly Rate"),
            );
            let no_of_months = relative_delta_month_component(start_date, end_date) + 1;
            let mut cost = plan.cost * f64::from(no_of_months);

            if input.settings_prorate {
                cost -= plan.cost * get_prorate_factor_for_dates(start_date, end_date);
            }

            cost
        }
        PriceDetermination::None => 0.0,
    }
}

pub fn get_prorate_factor(start_date: &str, end_date: &str) -> f64 {
    get_prorate_factor_for_dates(SimpleDate::parse(start_date), SimpleDate::parse(end_date))
}

fn get_prorate_factor_for_dates(start_date: SimpleDate, end_date: SimpleDate) -> f64 {
    let start_month_first_day = SimpleDate {
        day: 1,
        ..start_date
    };
    let total_days_to_skip = date_diff(start_date, start_month_first_day);
    let total_days_in_month = days_in_month(start_date.year, start_date.month);
    let mut prorate_factor = total_days_to_skip as f64 / f64::from(total_days_in_month);

    let end_month_last_day = SimpleDate {
        day: days_in_month(end_date.year, end_date.month),
        ..end_date
    };
    let total_days_to_skip = date_diff(end_month_last_day, end_date);
    let total_days_in_month = days_in_month(end_date.year, end_date.month);
    prorate_factor += total_days_to_skip as f64 / f64::from(total_days_in_month);

    prorate_factor
}

fn relative_delta_month_component(start_date: SimpleDate, end_date: SimpleDate) -> i32 {
    let mut months = (end_date.year - start_date.year) * 12 + i32::from(end_date.month)
        - i32::from(start_date.month);
    if end_date.day < start_date.day {
        months -= 1;
    }
    months.rem_euclid(12)
}

fn date_diff(left: SimpleDate, right: SimpleDate) -> i32 {
    days_from_civil(left.year, left.month, left.day)
        - days_from_civil(right.year, right.month, right.day)
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

fn days_from_civil(year: i32, month: u8, day: u8) -> i32 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = i32::from(month);
    let day = i32::from(day);
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
}
