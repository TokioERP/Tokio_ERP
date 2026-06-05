use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Budget {
    pub name: Option<String>,
    pub amended_from: Option<String>,
    pub account: Option<String>,
    pub action_if_accumulated_monthly_budget_exceeded: Option<String>,
    pub action_if_accumulated_monthly_budget_exceeded_on_mr: Option<String>,
    pub action_if_accumulated_monthly_budget_exceeded_on_po: Option<String>,
    pub action_if_annual_budget_exceeded: Option<String>,
    pub action_if_annual_budget_exceeded_on_mr: Option<String>,
    pub action_if_annual_budget_exceeded_on_po: Option<String>,
    pub applicable_on_booking_actual_expenses: bool,
    pub applicable_on_material_request: bool,
    pub applicable_on_purchase_order: bool,
    pub budget_against: String,
    pub budget_amount: f64,
    pub budget_distribution: Vec<BudgetDistributionRow>,
    pub budget_distribution_total: f64,
    pub budget_end_date: Option<String>,
    pub budget_start_date: Option<String>,
    pub company: Option<String>,
    pub cost_center: Option<String>,
    pub distribute_equally: bool,
    pub distribution_frequency: String,
    pub from_fiscal_year: Option<String>,
    pub project: Option<String>,
    pub revision_of: Option<String>,
    pub to_fiscal_year: Option<String>,
    pub old_doc: Option<Box<Budget>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetDistributionRow {
    pub start_date: String,
    pub end_date: String,
    pub amount: f64,
    pub percent: f64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BudgetAccountDetails {
    pub is_group: bool,
    pub company: String,
    pub report_type: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExistingBudget {
    pub name: String,
    pub account: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetCheckParams {
    pub company: Option<String>,
    pub posting_date: Option<String>,
    pub fiscal_year: Option<String>,
    pub account: Option<String>,
    pub expense_account: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub item_code: Option<String>,
    pub item_group: Option<String>,
    pub doctype: Option<String>,
    pub budget_against_field: Option<String>,
    pub budget_against_doctype: Option<String>,
    pub from_fiscal_year: Option<String>,
    pub to_fiscal_year: Option<String>,
    pub budget_start_date: Option<String>,
    pub budget_end_date: Option<String>,
    pub month_end_date: Option<String>,
    pub is_tree: bool,
    pub lft: Option<i32>,
    pub rgt: Option<i32>,
    pub for_material_request: bool,
    pub for_purchase_order: bool,
    pub actual_expense: f64,
    pub requested_amount: f64,
    pub ordered_amount: f64,
    pub exception_approver_role: Option<String>,
    pub user_roles: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetRecord {
    pub name: String,
    pub budget_against: String,
    pub budget_amount: f64,
    pub from_fiscal_year: String,
    pub to_fiscal_year: String,
    pub budget_start_date: String,
    pub budget_end_date: String,
    pub for_material_request: bool,
    pub for_purchase_order: bool,
    pub for_actual_expenses: bool,
    pub action_if_annual_budget_exceeded: Option<String>,
    pub action_if_accumulated_monthly_budget_exceeded: Option<String>,
    pub action_if_annual_budget_exceeded_on_mr: Option<String>,
    pub action_if_accumulated_monthly_budget_exceeded_on_mr: Option<String>,
    pub action_if_annual_budget_exceeded_on_po: Option<String>,
    pub action_if_accumulated_monthly_budget_exceeded_on_po: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetCheckMessage {
    pub diff: f64,
    pub total_expense: f64,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BudgetCheckResult {
    Stop(BudgetCheckMessage),
    Warn(BudgetCheckMessage),
    WithinLimit { total_expense: f64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpenseQueryPlan {
    pub condition1: String,
    pub date_condition: String,
    pub condition2: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingAmountQueryPlan {
    pub source: String,
    pub item_code: Option<String>,
    pub condition: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FiscalYearDates {
    pub from_year_start_date: String,
    pub from_year_end_date: String,
    pub to_year_start_date: String,
    pub to_year_end_date: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemDefaultsContext {
    pub item_default: Option<(Option<String>, Option<String>)>,
    pub item_group_default: Option<(Option<String>, Option<String>)>,
    pub company_default: Option<(Option<String>, Option<String>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RevisionPlan {
    pub cancel_old_budget: bool,
    pub new_budget: Budget,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetExpenseValidationContext {
    pub budget_count: usize,
    pub budget_exists_for_fiscal_year: bool,
    pub posting_fiscal_year: Option<String>,
    pub fiscal_years: FiscalYearDates,
    pub exception_approver_role: Option<String>,
    pub account_root_type: Option<String>,
    pub item_defaults: ItemDefaultsContext,
    pub budget_records: Vec<BudgetRecord>,
    pub monthly_budgets: Vec<(String, f64)>,
    pub currency: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BudgetExpenseValidationResult {
    Skipped(String),
    Checked { results: Vec<BudgetCheckResult> },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BudgetContext {
    pub from_fiscal_year_companies: Vec<String>,
    pub to_fiscal_year_companies: Vec<String>,
    pub from_fiscal_year_start_date: Option<String>,
    pub to_fiscal_year_end_date: Option<String>,
    pub existing_budgets: Vec<ExistingBudget>,
    pub account_details: Option<BudgetAccountDetails>,
    pub budget_against_is_tree: bool,
    pub actual_spent: f64,
    pub is_new: bool,
    pub old_doc: Option<Budget>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DuplicateBudgetError(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BudgetError {
    BudgetLimitExceeded(String),
    Duplicate(DuplicateBudgetError),
    Validation(String),
}

impl Budget {
    pub const DOCTYPE: &'static str = "Budget";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const ALLOW_IMPORT: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const FIELD_ORDER: [&'static str; 40] = [
        "naming_series",
        "budget_against",
        "company",
        "cost_center",
        "project",
        "account",
        "column_break_3",
        "amended_from",
        "from_fiscal_year",
        "to_fiscal_year",
        "budget_start_date",
        "budget_end_date",
        "distribution_frequency",
        "budget_amount",
        "section_break_nwug",
        "distribute_equally",
        "section_break_fpdt",
        "budget_distribution",
        "section_break_wkqb",
        "column_break_paum",
        "column_break_nwor",
        "budget_distribution_total",
        "section_break_6",
        "applicable_on_material_request",
        "action_if_annual_budget_exceeded_on_mr",
        "action_if_accumulated_monthly_budget_exceeded_on_mr",
        "column_break_13",
        "applicable_on_purchase_order",
        "action_if_annual_budget_exceeded_on_po",
        "action_if_accumulated_monthly_budget_exceeded_on_po",
        "section_break_16",
        "applicable_on_booking_actual_expenses",
        "action_if_annual_budget_exceeded",
        "action_if_accumulated_monthly_budget_exceeded",
        "control_action_for_cumulative_expense_section",
        "applicable_on_cumulative_expense",
        "action_if_annual_exceeded_on_cumulative_expense",
        "action_if_accumulated_monthly_exceeded_on_cumulative_expense",
        "section_break_kkan",
        "revision_of",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("naming_series", "Series")
                .options("BUDGET-.########")
                .default("BUDGET-.########")
                .required()
                .no_copy()
                .print_hide()
                .set_only_once(),
            FieldSpec::select("budget_against", "Budget Against")
                .options("\nCost Center\nProject")
                .default("Cost Center")
                .required()
                .in_list_view()
                .in_standard_filter()
                .read_only_depends_on("eval: doc.revision_of"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view()
                .in_standard_filter()
                .read_only_depends_on("eval: doc.revision_of"),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .depends_on("eval:doc.budget_against == 'Cost Center'")
                .in_global_search()
                .in_standard_filter()
                .read_only_depends_on("eval: doc.revision_of"),
            FieldSpec::link("project", "Project")
                .options("Project")
                .depends_on("eval:doc.budget_against == 'Project'")
                .in_standard_filter()
                .read_only_depends_on("eval: doc.revision_of"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view()
                .read_only_depends_on("eval: doc.revision_of"),
            FieldSpec::table("budget_distribution", "Budget Distribution")
                .options("Budget Distribution"),
        ]
    }

    pub fn validate(&mut self, context: &BudgetContext) -> Result<(), BudgetError> {
        if !has_value(&self.budget_against_value()) {
            return Err(BudgetError::Validation(format!(
                "{} is mandatory",
                self.budget_against
            )));
        }
        self.validate_budget_amount()?;
        self.validate_fiscal_year(context)?;
        self.set_fiscal_year_dates(context)?;
        self.validate_duplicate(context)?;
        self.validate_account(context)?;
        self.set_null_value();
        self.validate_applicable_for()?;
        self.validate_existing_expenses(context)
    }

    pub fn validate_budget_amount(&self) -> Result<(), BudgetError> {
        if self.budget_amount <= 0.0 {
            return Err(BudgetError::Validation(format!(
                "Budget Amount can not be {}.",
                format_number(self.budget_amount)
            )));
        }
        Ok(())
    }

    pub fn validate_fiscal_year(&self, context: &BudgetContext) -> Result<(), BudgetError> {
        if let Some(from_fiscal_year) = self.from_fiscal_year.as_deref() {
            self.validate_fiscal_year_company(
                from_fiscal_year,
                &context.from_fiscal_year_companies,
            )?;
        }
        if let Some(to_fiscal_year) = self.to_fiscal_year.as_deref() {
            self.validate_fiscal_year_company(to_fiscal_year, &context.to_fiscal_year_companies)?;
        }
        Ok(())
    }

    pub fn validate_fiscal_year_company(
        &self,
        fiscal_year: &str,
        linked_companies: &[String],
    ) -> Result<(), BudgetError> {
        if !linked_companies.is_empty() && !linked_companies.iter().any(|c| c == self.company()) {
            return Err(BudgetError::Validation(format!(
                "Fiscal Year {fiscal_year} is not available for Company {}.",
                self.company()
            )));
        }
        Ok(())
    }

    pub fn set_fiscal_year_dates(&mut self, context: &BudgetContext) -> Result<(), BudgetError> {
        if self.from_fiscal_year.is_some() {
            self.budget_start_date = context.from_fiscal_year_start_date.clone();
        }
        if self.to_fiscal_year.is_some() {
            self.budget_end_date = context.to_fiscal_year_end_date.clone();
        }

        if self.budget_start_date.as_deref().unwrap_or_default()
            > self.budget_end_date.as_deref().unwrap_or_default()
        {
            return Err(BudgetError::Validation(
                "From Fiscal Year cannot be greater than To Fiscal Year".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_duplicate(&self, context: &BudgetContext) -> Result<(), BudgetError> {
        if !has_value(&self.account) || context.existing_budgets.is_empty() {
            return Ok(());
        }
        let existing = &context.existing_budgets[0];
        Err(BudgetError::Duplicate(DuplicateBudgetError(format!(
            "Another Budget record '{}' already exists against {} '{}' and account '{}' with overlapping fiscal years.",
            existing.name,
            self.budget_against,
            self.budget_against_value().unwrap_or_default(),
            existing.account
        ))))
    }

    pub fn validate_account(&self, context: &BudgetContext) -> Result<(), BudgetError> {
        if !has_value(&self.account) {
            return Err(BudgetError::Validation("Account is mandatory".to_string()));
        }
        let Some(account) = &context.account_details else {
            return Ok(());
        };
        if account.is_group {
            return Err(BudgetError::Validation(format!(
                "Budget cannot be assigned against Group Account {}",
                self.account()
            )));
        }
        if account.company != self.company() {
            return Err(BudgetError::Validation(format!(
                "Account {} does not belong to company {}",
                self.account(),
                self.company()
            )));
        }
        if account.report_type != "Profit and Loss" {
            return Err(BudgetError::Validation(format!(
                "Budget cannot be assigned against {}, as it's not an Income or Expense account",
                self.account()
            )));
        }
        Ok(())
    }

    pub fn set_null_value(&mut self) {
        if self.budget_against == "Cost Center" {
            self.project = None;
        } else {
            self.cost_center = None;
        }
    }

    pub fn validate_applicable_for(&mut self) -> Result<(), BudgetError> {
        if self.applicable_on_material_request
            && !(self.applicable_on_purchase_order && self.applicable_on_booking_actual_expenses)
        {
            return Err(BudgetError::Validation(
                "Please enable Applicable on Purchase Order and Applicable on Booking Actual Expenses"
                    .to_string(),
            ));
        }
        if self.applicable_on_purchase_order && !self.applicable_on_booking_actual_expenses {
            return Err(BudgetError::Validation(
                "Please enable Applicable on Booking Actual Expenses".to_string(),
            ));
        }
        if !(self.applicable_on_material_request
            || self.applicable_on_purchase_order
            || self.applicable_on_booking_actual_expenses)
        {
            self.applicable_on_booking_actual_expenses = true;
        }
        Ok(())
    }

    pub fn validate_existing_expenses(&self, context: &BudgetContext) -> Result<(), BudgetError> {
        if context.is_new && self.revision_of.is_some() {
            return Ok(());
        }
        if context.actual_spent > self.budget_amount {
            return Err(BudgetError::BudgetLimitExceeded(format!(
                "Spending for Account <b>{}</b> (<b>{}</b>) between <b>{}</b> and <b>{}</b> has already exceeded the new allocated budget. Spent: <b>{}</b>, Budget: <b>{}</b>",
                self.account(),
                self.company(),
                self.budget_start_date.as_deref().unwrap_or_default(),
                self.budget_end_date.as_deref().unwrap_or_default(),
                format_number(context.actual_spent),
                format_number(self.budget_amount)
            )));
        }
        Ok(())
    }

    pub fn before_save(&mut self) {
        self.allocate_budget();
        self.budget_distribution_total = self
            .budget_distribution
            .iter()
            .map(|row| row.amount)
            .sum::<f64>();
    }

    pub fn allocate_budget(&mut self) {
        if self.should_skip_allocation() {
            return;
        }
        if self.should_recalculate_manual_distribution() {
            self.recalculate_manual_distribution();
            return;
        }
        if !self.should_regenerate_budget_distribution() {
            return;
        }
        self.regenerate_distribution();
    }

    pub fn should_skip_allocation(&self) -> bool {
        self.revision_of.is_some() && !self.distribute_equally
    }

    pub fn should_recalculate_manual_distribution(&self) -> bool {
        if self.distribute_equally || self.budget_distribution.is_empty() {
            return false;
        }
        let Some(old) = &self.old_doc else {
            return false;
        };
        old.budget_amount != self.budget_amount
            && old.distribution_frequency == self.distribution_frequency
            && old.budget_start_date == self.budget_start_date
            && old.budget_end_date == self.budget_end_date
    }

    pub fn recalculate_manual_distribution(&mut self) {
        for row in &mut self.budget_distribution {
            row.amount = round_to_precision((row.percent / 100.0) * self.budget_amount, 3);
        }
    }

    pub fn should_regenerate_budget_distribution(&self) -> bool {
        let Some(old) = &self.old_doc else {
            return true;
        };
        if self.budget_distribution.is_empty() {
            return true;
        }
        if old.from_fiscal_year != self.from_fiscal_year
            || old.to_fiscal_year != self.to_fiscal_year
            || old.budget_amount != self.budget_amount
            || old.distribution_frequency != self.distribution_frequency
        {
            return true;
        }
        self.distribute_equally
    }

    pub fn regenerate_distribution(&mut self) {
        self.budget_distribution.clear();
        let periods = self.get_budget_periods();
        let total_periods = periods.len();
        let row_percent = if total_periods == 0 {
            0.0
        } else {
            100.0 / total_periods as f64
        };
        for (start_date, end_date) in periods {
            let mut row = BudgetDistributionRow {
                start_date,
                end_date,
                ..BudgetDistributionRow::default()
            };
            self.add_allocated_amount(&mut row, row_percent);
            self.budget_distribution.push(row);
        }
        self.budget_distribution_total = self.budget_amount;
    }

    pub fn get_budget_periods(&self) -> Vec<(String, String)> {
        let Ok(mut start_date) = parse_date(self.budget_start_date.as_deref().unwrap_or_default())
        else {
            return Vec::new();
        };
        let Ok(end_date) = parse_date(self.budget_end_date.as_deref().unwrap_or_default()) else {
            return Vec::new();
        };
        let mut periods = Vec::new();
        while start_date <= end_date {
            let period_start = first_day(start_date);
            let mut period_end = self.get_period_end(period_start, &self.distribution_frequency);
            if period_end > end_date {
                period_end = end_date;
            }
            periods.push((format_date(period_start), format_date(period_end)));
            start_date = add_months(
                period_start,
                self.get_month_increment(&self.distribution_frequency),
            );
        }
        periods
    }

    pub fn get_period_end(&self, start_date: DateValue, frequency: &str) -> DateValue {
        match frequency {
            "Monthly" => last_day(start_date),
            "Quarterly" => last_day(add_months(start_date, 2)),
            "Half-Yearly" => last_day(add_months(start_date, 5)),
            _ => last_day(add_months(start_date, 11)),
        }
    }

    pub fn get_month_increment(&self, frequency: &str) -> i32 {
        match frequency {
            "Monthly" => 1,
            "Quarterly" => 3,
            "Half-Yearly" => 6,
            "Yearly" => 12,
            _ => 1,
        }
    }

    pub fn add_allocated_amount(&self, row: &mut BudgetDistributionRow, row_percent: f64) {
        row.amount = round_to_precision(self.budget_amount * row_percent / 100.0, 3);
        row.percent = round_to_precision(row_percent, 3);
    }

    pub fn validate_distribution_totals(&self) -> Result<(), BudgetError> {
        if self.should_regenerate_budget_distribution() {
            return Ok(());
        }
        let total_amount = self
            .budget_distribution
            .iter()
            .map(|row| row.amount)
            .sum::<f64>();
        let total_percent = self
            .budget_distribution
            .iter()
            .map(|row| row.percent)
            .sum::<f64>();

        if round_to_precision((total_amount - self.budget_amount).abs(), 2) > 0.10 {
            return Err(BudgetError::Validation(format!(
                "Total distributed amount {} must be equal to Budget Amount {}",
                format_number(round_to_precision(total_amount, 2)),
                format_number(self.budget_amount)
            )));
        }
        if round_to_precision((total_percent - 100.0).abs(), 2) > 0.10 {
            return Err(BudgetError::Validation(format!(
                "Total distribution percent must equal 100 (currently {})",
                format_number(round_to_precision(total_percent, 2))
            )));
        }
        Ok(())
    }

    fn account(&self) -> &str {
        self.account.as_deref().unwrap_or_default()
    }

    fn budget_against_value(&self) -> Option<String> {
        match self.budget_against.as_str() {
            "Cost Center" => self.cost_center.clone(),
            "Project" => self.project.clone(),
            _ => None,
        }
    }

    fn company(&self) -> &str {
        self.company.as_deref().unwrap_or_default()
    }
}

impl BudgetDistributionRow {
    pub fn new(
        start_date: impl Into<String>,
        end_date: impl Into<String>,
        amount: f64,
        percent: f64,
    ) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
            amount,
            percent,
        }
    }
}

impl DocumentController for Budget {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "before_save", "on_update"]
    }
}

impl BudgetCheckParams {
    fn value_for_field(&self, fieldname: &str) -> Option<&str> {
        match fieldname {
            "cost_center" => self.cost_center.as_deref(),
            "project" => self.project.as_deref(),
            "account" => self.account.as_deref(),
            "expense_account" => self.expense_account.as_deref(),
            _ => None,
        }
    }
}

pub fn get_actions(
    params: &BudgetCheckParams,
    budget: &BudgetRecord,
) -> (Option<String>, Option<String>) {
    let mut yearly_action = budget.action_if_annual_budget_exceeded.clone();
    let mut monthly_action = budget.action_if_accumulated_monthly_budget_exceeded.clone();

    if params.doctype.as_deref() == Some("Material Request") && budget.for_material_request {
        yearly_action = budget.action_if_annual_budget_exceeded_on_mr.clone();
        monthly_action = budget
            .action_if_accumulated_monthly_budget_exceeded_on_mr
            .clone();
    } else if params.doctype.as_deref() == Some("Purchase Order") && budget.for_purchase_order {
        yearly_action = budget.action_if_annual_budget_exceeded_on_po.clone();
        monthly_action = budget
            .action_if_accumulated_monthly_budget_exceeded_on_po
            .clone();
    }

    (yearly_action, monthly_action)
}

pub fn compare_expense_with_budget(
    params: &mut BudgetCheckParams,
    budget_amount: f64,
    action_for: &str,
    action: &str,
    budget_against: &str,
    mut amount: f64,
    currency: &str,
) -> BudgetCheckResult {
    if amount == 0.0 {
        if params.doctype.as_deref() == Some("Material Request") && params.for_material_request {
            amount = params.requested_amount + params.ordered_amount;
        } else if params.doctype.as_deref() == Some("Purchase Order") && params.for_purchase_order {
            amount = params.ordered_amount;
        }
    } else {
        params.requested_amount = 0.0;
        params.ordered_amount = 0.0;
    }

    let total_expense = params.actual_expense + amount;
    if total_expense <= budget_amount {
        return BudgetCheckResult::WithinLimit { total_expense };
    }

    let (diff, msg_template) = if params.actual_expense > budget_amount {
        (
            params.actual_expense - budget_amount,
            "{action_for} Budget for Account <b>{account}</b> against {budget_against_field} <b>{budget_against}</b> is <b>{budget_amount}</b>. It is already exceeded by <b>{diff}</b>.",
        )
    } else {
        (
            total_expense - budget_amount,
            "{action_for} Budget for Account <b>{account}</b> against {budget_against_field} <b>{budget_against}</b> is <b>{budget_amount}</b>. It will be exceeded by <b>{diff}</b>.",
        )
    };

    let message = msg_template
        .replace("{action_for}", action_for)
        .replace("{account}", params.account.as_deref().unwrap_or_default())
        .replace(
            "{budget_against_field}",
            &unscrub(params.budget_against_field.as_deref().unwrap_or_default()),
        )
        .replace("{budget_against}", budget_against)
        .replace("{budget_amount}", &format_money(budget_amount, currency))
        .replace("{diff}", &format_money(diff, currency))
        + &get_expense_breakup(params, currency, budget_against);

    let result = BudgetCheckMessage {
        diff,
        total_expense,
        message,
    };
    if action == "Stop"
        && !params.exception_approver_role.as_ref().is_some_and(|role| {
            params
                .user_roles
                .iter()
                .any(|user_role| user_role.as_str() == role.as_str())
        })
    {
        BudgetCheckResult::Stop(result)
    } else {
        BudgetCheckResult::Warn(result)
    }
}

pub fn get_expense_breakup(
    params: &BudgetCheckParams,
    currency: &str,
    _budget_against: &str,
) -> String {
    format!(
        "<hr> Total Expenses booked through - <ul><li>Actual Expenses - <b>{}</b></li><li>Material Requests - <b>{}</b></li><li>Unbilled Orders - <b>{}</b></li></ul>",
        format_money(params.actual_expense, currency),
        format_money(params.requested_amount, currency),
        format_money(params.ordered_amount, currency),
    )
}

pub fn get_other_condition(
    params: &BudgetCheckParams,
    for_doc: &str,
    fiscal_years: &FiscalYearDates,
) -> String {
    let mut condition = format!(
        "expense_account = '{}'",
        params.expense_account.as_deref().unwrap_or_default()
    );
    if let Some(fieldname) = params.budget_against_field.as_deref() {
        if let Some(value) = params
            .value_for_field(fieldname)
            .filter(|value| !value.is_empty())
        {
            condition.push_str(&format!(" and child.{fieldname} = '{value}'"));
        }
    }

    let date_field = if for_doc == "Material Request" {
        "schedule_date"
    } else {
        "transaction_date"
    };
    condition.push_str(&format!(
        " and parent.{date_field} between '{}' and '{}'",
        fiscal_years.from_year_start_date, fiscal_years.to_year_end_date
    ));
    condition
}

pub fn get_requested_amount_query_plan(
    params: &BudgetCheckParams,
    fiscal_years: &FiscalYearDates,
) -> PendingAmountQueryPlan {
    PendingAmountQueryPlan {
        source: "Material Request".to_string(),
        item_code: params.item_code.clone(),
        condition: get_other_condition(params, "Material Request", fiscal_years),
    }
}

pub fn get_ordered_amount_query_plan(
    params: &BudgetCheckParams,
    fiscal_years: &FiscalYearDates,
) -> PendingAmountQueryPlan {
    PendingAmountQueryPlan {
        source: "Purchase Order".to_string(),
        item_code: params.item_code.clone(),
        condition: get_other_condition(params, "Purchase Order", fiscal_years),
    }
}

pub fn validate_budget_records(
    params: &mut BudgetCheckParams,
    budget_records: &[BudgetRecord],
    expense_amount: f64,
    monthly_budgets: &[(String, f64)],
    currency: &str,
) -> Vec<BudgetCheckResult> {
    let mut results = Vec::new();
    for budget in budget_records {
        if budget.budget_amount == 0.0 {
            continue;
        }

        let (yearly_action, monthly_action) = get_actions(params, budget);
        params.for_material_request = budget.for_material_request;
        params.for_purchase_order = budget.for_purchase_order;
        params.from_fiscal_year = Some(budget.from_fiscal_year.clone());
        params.to_fiscal_year = Some(budget.to_fiscal_year.clone());
        params.budget_start_date = Some(budget.budget_start_date.clone());
        params.budget_end_date = Some(budget.budget_end_date.clone());

        if yearly_action
            .as_deref()
            .is_some_and(|action| action == "Stop" || action == "Warn")
        {
            results.push(compare_expense_with_budget(
                params,
                budget.budget_amount,
                "Annual",
                yearly_action.as_deref().unwrap_or_default(),
                &budget.budget_against,
                expense_amount,
                currency,
            ));
        }

        if monthly_action
            .as_deref()
            .is_some_and(|action| action == "Stop" || action == "Warn")
        {
            let budget_amount = monthly_budgets
                .iter()
                .find(|(name, _)| name == &budget.name)
                .map(|(_, amount)| *amount)
                .unwrap_or(0.0);
            params.month_end_date = params
                .posting_date
                .as_deref()
                .and_then(|posting_date| parse_date(posting_date).ok())
                .map(last_day)
                .map(format_date);
            results.push(compare_expense_with_budget(
                params,
                budget_amount,
                "Accumulated Monthly",
                monthly_action.as_deref().unwrap_or_default(),
                &budget.budget_against,
                expense_amount,
                currency,
            ));
        }
    }
    results
}

pub fn validate_expense_against_budget(
    params: &mut BudgetCheckParams,
    context: &BudgetExpenseValidationContext,
    expense_amount: f64,
) -> BudgetExpenseValidationResult {
    if context.budget_count == 0 {
        return BudgetExpenseValidationResult::Skipped("No Budget records".to_string());
    }

    if params.fiscal_year.is_none() {
        params.fiscal_year = context.posting_fiscal_year.clone();
    }

    if !context.budget_exists_for_fiscal_year {
        return BudgetExpenseValidationResult::Skipped(
            "No submitted Budget for fiscal year".to_string(),
        );
    }

    params.exception_approver_role = context.exception_approver_role.clone();

    if params.account.is_none() {
        params.account = params.expense_account.clone();
    }
    if params.expense_account.is_none() {
        params.expense_account = params.account.clone();
    }

    if (params.account.is_none() || params.cost_center.is_none()) && params.item_code.is_some() {
        let (cost_center, account) = get_item_details(params, &context.item_defaults);
        if params.cost_center.is_none() {
            params.cost_center = cost_center;
        }
        if params.account.is_none() {
            params.account = account.clone();
        }
        if params.expense_account.is_none() {
            params.expense_account = account;
        }
    }

    if params.account.is_none() {
        return BudgetExpenseValidationResult::Skipped("No account".to_string());
    }
    if context.account_root_type.as_deref() != Some("Expense") {
        return BudgetExpenseValidationResult::Skipped("Account is not Expense".to_string());
    }

    if params
        .project
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        params.budget_against_field = Some("project".to_string());
        params.budget_against_doctype = Some("Project".to_string());
    } else if params
        .cost_center
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        params.budget_against_field = Some("cost_center".to_string());
        params.budget_against_doctype = Some("Cost Center".to_string());
    }

    let results = validate_budget_records(
        params,
        &context.budget_records,
        expense_amount,
        &context.monthly_budgets,
        &context.currency,
    );
    BudgetExpenseValidationResult::Checked { results }
}

pub fn get_actual_expense_query_plan(params: &BudgetCheckParams) -> ExpenseQueryPlan {
    let budget_against_field = params.budget_against_field.as_deref().unwrap_or_default();
    let budget_against_doctype = params
        .budget_against_doctype
        .clone()
        .unwrap_or_else(|| unscrub(budget_against_field));
    let condition1 = if params.month_end_date.is_some() {
        " and gle.posting_date <= %(month_end_date)s".to_string()
    } else {
        String::new()
    };
    let date_condition = format!(
        "and gle.posting_date between '{}' and '{}'",
        params.budget_start_date.as_deref().unwrap_or_default(),
        params.budget_end_date.as_deref().unwrap_or_default()
    );
    let condition2 = if params.is_tree {
        format!(
            "\n\t\t\t\tand exists(\n\t\t\t\t\tselect name from `tab{budget_against_doctype}`\n\t\t\t\t\twhere lft >= %(lft)s and rgt <= %(rgt)s\n\t\t\t\t\tand name = gle.{budget_against_field}\n\t\t\t\t)\n\t\t\t"
        )
    } else {
        format!("\n\t\t\t\tand gle.{budget_against_field} = %({budget_against_field})s\n\t\t\t")
    };
    ExpenseQueryPlan {
        condition1,
        date_condition,
        condition2,
    }
}

pub fn get_accumulated_monthly_budget(
    distribution: &[BudgetDistributionRow],
    posting_date: &str,
) -> f64 {
    let Ok(posting_date) = parse_date(posting_date) else {
        return 0.0;
    };
    distribution
        .iter()
        .filter(|row| {
            parse_date(&row.start_date)
                .map(|start_date| start_date <= posting_date)
                .unwrap_or(false)
        })
        .map(|row| row.amount)
        .sum()
}

pub fn get_item_details(
    params: &BudgetCheckParams,
    defaults: &ItemDefaultsContext,
) -> (Option<String>, Option<String>) {
    if params.company.as_deref().unwrap_or_default().is_empty() {
        return (None, None);
    }

    let (mut cost_center, mut expense_account) = if params.item_code.is_some() {
        defaults.item_default.clone().unwrap_or_default()
    } else {
        (None, None)
    };

    if !(cost_center.is_some() && expense_account.is_some()) {
        for data in [&defaults.item_group_default, &defaults.company_default]
            .into_iter()
            .flatten()
        {
            if cost_center.is_none() {
                cost_center = data.0.clone();
            }
            if expense_account.is_none() {
                expense_account = data.1.clone();
            }
            if cost_center.is_some() && expense_account.is_some() {
                return (cost_center, expense_account);
            }
        }
    }

    (cost_center, expense_account)
}

pub fn get_fiscal_year_date_range(fiscal_years: &FiscalYearDates) -> (String, String) {
    (
        fiscal_years.from_year_start_date.clone(),
        fiscal_years.to_year_end_date.clone(),
    )
}

pub fn revise_budget(old_budget: &Budget, old_docstatus: i32) -> RevisionPlan {
    let mut new_budget = old_budget.clone();
    new_budget.name = None;
    new_budget.revision_of = old_budget.name.clone();
    new_budget.old_doc = None;
    RevisionPlan {
        cancel_old_budget: old_docstatus == 1,
        new_budget,
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateValue {
    year: i32,
    month: u32,
    day: u32,
}

fn has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn format_money(value: f64, currency: &str) -> String {
    if currency.is_empty() {
        format_number(value)
    } else {
        format!("{} {}", currency, format_number(value))
    }
}

fn unscrub(value: &str) -> String {
    value
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_date(value: &str) -> Result<DateValue, String> {
    let mut parts = value.split('-');
    let year = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {value}"))?
        .parse()
        .map_err(|_| format!("Invalid date: {value}"))?;
    let month = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {value}"))?
        .parse()
        .map_err(|_| format!("Invalid date: {value}"))?;
    let day = parts
        .next()
        .ok_or_else(|| format!("Invalid date: {value}"))?
        .parse()
        .map_err(|_| format!("Invalid date: {value}"))?;
    if parts.next().is_some()
        || !(1..=12).contains(&month)
        || !(1..=days_in_month(year, month)).contains(&day)
    {
        return Err(format!("Invalid date: {value}"));
    }
    Ok(DateValue { year, month, day })
}

fn format_date(date: DateValue) -> String {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}

fn first_day(date: DateValue) -> DateValue {
    DateValue { day: 1, ..date }
}

fn last_day(date: DateValue) -> DateValue {
    DateValue {
        day: days_in_month(date.year, date.month),
        ..date
    }
}

fn add_months(date: DateValue, months: i32) -> DateValue {
    let zero_based = date.month as i32 - 1 + months;
    let year = date.year + zero_based.div_euclid(12);
    let month = zero_based.rem_euclid(12) as u32 + 1;
    let day = date.day.min(days_in_month(year, month));
    DateValue { year, month, day }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
