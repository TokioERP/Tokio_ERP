use crate::erpnext::accounts::doctype::cashier_closing_payments::cashier_closing_payments::CashierClosingPayments;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CashierClosingValidationError {
    FromTimeShouldBeLessThanToTime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashierClosingOutstandingQuery {
    pub query: &'static str,
    pub params: [String; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct CashierClosing {
    pub naming_series: Option<String>,
    pub user: Option<String>,
    pub date: Option<String>,
    pub from_time: Option<String>,
    pub time: Option<String>,
    pub expense: f64,
    pub custody: f64,
    pub returns: f64,
    pub outstanding_amount: f64,
    pub payments: Vec<CashierClosingPayments>,
    pub net_amount: f64,
    pub amended_from: Option<String>,
}

impl Default for CashierClosing {
    fn default() -> Self {
        Self {
            naming_series: Some("POS-CLO-".to_string()),
            user: None,
            date: None,
            from_time: None,
            time: None,
            expense: 0.0,
            custody: 0.0,
            returns: 0.0,
            outstanding_amount: 0.0,
            payments: Vec::new(),
            net_amount: 0.0,
            amended_from: None,
        }
    }
}

impl CashierClosing {
    pub const DOCTYPE: &'static str = "Cashier Closing";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 12] = [
        "naming_series",
        "user",
        "date",
        "from_time",
        "time",
        "expense",
        "custody",
        "returns",
        "outstanding_amount",
        "payments",
        "net_amount",
        "amended_from",
    ];
    pub const AUTONAME: &'static str = "naming_series:";
    pub const IS_SUBMITTABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const OUTSTANDING_QUERY: &'static str = "select sum(outstanding_amount) from `tabSales Invoice` where posting_date=%s and posting_time>=%s and posting_time<=%s and owner=%s";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("naming_series", "Series")
                .options("POS-CLO-")
                .default("POS-CLO-")
                .read_only()
                .in_filter()
                .in_global_search()
                .in_standard_filter(),
            FieldSpec::link("user", "User")
                .options("User")
                .read_only()
                .required()
                .in_filter()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::date("date", "Date")
                .default("Today")
                .read_only()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::time("from_time", "From Time")
                .required()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::time("time", "To Time")
                .required()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::float("expense", "Expense")
                .default("0.00")
                .in_filter(),
            FieldSpec::float("custody", "Custody")
                .default("0.00")
                .in_filter(),
            FieldSpec::float("returns", "Returns")
                .default("0.00")
                .precision("2")
                .in_filter(),
            FieldSpec::float("outstanding_amount", "Outstanding Amount")
                .default("0.00")
                .read_only(),
            FieldSpec::table("payments", "Payments")
                .options("Cashier Closing Payments")
                .in_filter(),
            FieldSpec::float("net_amount", "Net Amount")
                .read_only()
                .in_filter()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Cashier Closing")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn setup_default_user(&mut self, session_user: impl Into<String>) {
        if self.user.as_deref().unwrap_or_default().is_empty() {
            self.user = Some(session_user.into());
        }
    }

    pub fn validate(&self) -> Result<(), CashierClosingValidationError> {
        self.validate_time()
    }

    pub fn before_save(&mut self, outstanding_amount: Option<f64>) {
        self.get_outstanding(outstanding_amount);
        self.make_calculations();
    }

    pub fn outstanding_query(&self) -> CashierClosingOutstandingQuery {
        CashierClosingOutstandingQuery {
            query: Self::OUTSTANDING_QUERY,
            params: [
                self.date.clone().unwrap_or_default(),
                self.from_time.clone().unwrap_or_default(),
                self.time.clone().unwrap_or_default(),
                self.user.clone().unwrap_or_default(),
            ],
        }
    }

    pub fn get_outstanding(&mut self, outstanding_amount: Option<f64>) {
        self.outstanding_amount = flt(outstanding_amount.unwrap_or(0.0));
    }

    pub fn make_calculations(&mut self) {
        let payment_total = self
            .payments
            .iter()
            .map(|payment| flt_str(payment.amount.as_deref()))
            .sum::<f64>();
        self.net_amount = flt(payment_total + self.outstanding_amount + flt(self.expense)
            - flt(self.custody)
            + flt(self.returns));
    }

    pub fn validate_time(&self) -> Result<(), CashierClosingValidationError> {
        if self.from_time.as_deref().unwrap_or_default() >= self.time.as_deref().unwrap_or_default()
        {
            return Err(CashierClosingValidationError::FromTimeShouldBeLessThanToTime);
        }
        Ok(())
    }
}

impl DocumentController for CashierClosing {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "validate",
            "before_save",
            "get_outstanding",
            "make_calculations",
            "validate_time",
        ]
    }
}

fn flt(value: f64) -> f64 {
    value
}

fn flt_str(value: Option<&str>) -> f64 {
    value
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(0.0)
}
