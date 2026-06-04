use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeriodClosingVoucher {
    pub amended_from: Option<String>,
    pub closing_account_head: Option<String>,
    pub company: Option<String>,
    pub docstatus: i32,
    pub error_message: Option<String>,
    pub fiscal_year: Option<String>,
    pub fy_end_date: Option<String>,
    pub fy_start_date: Option<String>,
    pub gle_processing_status: Option<String>,
    pub name: Option<String>,
    pub period_end_date: Option<String>,
    pub period_start_date: Option<String>,
    pub remarks: Option<String>,
    pub transaction_date: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeriodClosingVoucherContext {
    pub fy_start_date: String,
    pub fy_end_date: String,
    pub previous_closed_period_end_date: Option<String>,
    pub previous_fiscal_year: Option<(String, String)>,
    pub previous_fiscal_year_closed: bool,
    pub gle_exists_in_previous_year: bool,
    pub future_closing_voucher: Option<String>,
    pub closing_account_root_type: Option<String>,
    pub closing_account_currency: Option<String>,
    pub company_currency: Option<String>,
    pub use_legacy_controller_for_pcv: bool,
    pub gl_entry_estimated_count: usize,
    pub gle_count_against_current_pcv: usize,
    pub closing_account_currency_from_account: Option<String>,
    pub accounting_dimensions: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountBalance {
    pub debit: f64,
    pub credit: f64,
    pub debit_in_account_currency: f64,
    pub credit_in_account_currency: f64,
    pub account_currency: String,
    pub balance_in_account_currency: f64,
    pub balance_in_company_currency: f64,
}

pub type DimensionBalance = (Vec<String>, BTreeMap<String, AccountBalance>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PcvGlEntryRow {
    pub closing_date: Option<String>,
    pub company: String,
    pub posting_date: String,
    pub account: String,
    pub account_currency: String,
    pub debit_in_account_currency: f64,
    pub debit: f64,
    pub credit_in_account_currency: f64,
    pub credit: f64,
    pub is_period_closing_voucher_entry: bool,
    pub voucher_type: String,
    pub voucher_no: String,
    pub fiscal_year: String,
    pub remarks: String,
    pub is_opening: String,
    pub dimensions: BTreeMap<String, String>,
    pub period_closing_voucher: Option<String>,
}

pub type GlEntryRow = PcvGlEntryRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PeriodClosingVoucherError {
    Validation(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LifecycleAction {
    SetGleProcessingStatus(&'static str),
    ProcessGlAndClosingEntriesInline,
    EnqueueGlAndClosingEntries {
        timeout_seconds: usize,
    },
    CreateAndSubmitProcessPeriodClosingVoucher {
        parent_pcv: String,
    },
    CancelProcessPeriodClosingVoucherDocs,
    ProcessCancellationInline,
    EnqueueCancellation {
        queue: &'static str,
        enqueue_after_commit: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosingProcessPlan {
    pub voucher_name: String,
    pub make_gl_entries_merge_entries: bool,
    pub make_closing_entries: bool,
    pub success_status: &'static str,
    pub failure_status: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessCancellationPlan {
    pub voucher_type: &'static str,
    pub voucher_no: String,
    pub delete_closing_entries_doctype: &'static str,
    pub success_status: &'static str,
    pub failure_status: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessPcvDocumentPlan {
    pub doctype: &'static str,
    pub parent_pcv: String,
    pub docstatus: Vec<i32>,
    pub action: &'static str,
}

impl PeriodClosingVoucher {
    pub const DOCTYPE: &'static str = "Period Closing Voucher";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-PCV-.YYYY.-.#####";
    pub const FIELD_ORDER: [&'static str; 11] = [
        "transaction_date",
        "company",
        "fiscal_year",
        "period_start_date",
        "period_end_date",
        "amended_from",
        "column_break1",
        "closing_account_head",
        "gle_processing_status",
        "remarks",
        "error_message",
    ];
    pub const IS_SUBMITTABLE: bool = true;
    pub const SEARCH_FIELDS: &'static str = "fiscal_year, period_start_date, period_end_date";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TITLE_FIELD: &'static str = "closing_account_head";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("transaction_date", "Transaction Date").default("Today"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("fiscal_year", "Fiscal Year")
                .options("Fiscal Year")
                .required()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::date("period_start_date", "Period Start Date").required(),
            FieldSpec::date("period_end_date", "Period End Date").required(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Period Closing Voucher")
                .ignore_user_permissions()
                .no_copy()
                .read_only(),
            FieldSpec::column_break("column_break1"),
            FieldSpec::link("closing_account_head", "Closing Account Head")
                .options("Account")
                .description(
                    "The account head under Liability or Equity, in which Profit/Loss will be booked",
                )
                .required(),
            FieldSpec::select("gle_processing_status", "GL Entry Processing Status")
                .options("In Progress\nCompleted\nFailed")
                .depends_on("eval:doc.docstatus!=0")
                .read_only(),
            FieldSpec::small_text("remarks", "Remarks").required(),
            FieldSpec::text("error_message", "Error Message")
                .depends_on("eval:doc.gle_processing_status=='Failed'")
                .read_only(),
        ]
    }

    pub fn validate(
        &mut self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), PeriodClosingVoucherError> {
        self.validate_start_and_end_date(context)?;
        self.check_if_previous_year_closed(context)?;
        self.block_if_future_closing_voucher_exists(context)?;
        self.check_closing_account_type(context)?;
        self.check_closing_account_currency(context)
    }

    pub fn validate_start_and_end_date(
        &mut self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), PeriodClosingVoucherError> {
        self.fy_start_date = Some(context.fy_start_date.clone());
        self.fy_end_date = Some(context.fy_end_date.clone());

        let valid_start_date =
            if let Some(previous_end) = context.previous_closed_period_end_date.as_deref() {
                add_days(previous_end, 1).map_err(PeriodClosingVoucherError::Validation)?
            } else {
                context.fy_start_date.clone()
            };

        if self.period_start_date() != valid_start_date {
            return Err(PeriodClosingVoucherError::Validation(format!(
                "Period Start Date must be {valid_start_date}"
            )));
        }

        if self.period_start_date() > self.period_end_date() {
            return Err(PeriodClosingVoucherError::Validation(
                "Period Start Date cannot be greater than Period End Date".to_string(),
            ));
        }

        if self.period_end_date() > context.fy_end_date {
            return Err(PeriodClosingVoucherError::Validation(
                "Period End Date cannot be greater than Fiscal Year End Date".to_string(),
            ));
        }

        Ok(())
    }

    pub fn check_if_previous_year_closed(
        &self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), PeriodClosingVoucherError> {
        if context.previous_fiscal_year.is_none()
            || context.previous_fiscal_year_closed
            || !context.gle_exists_in_previous_year
        {
            return Ok(());
        }

        Err(PeriodClosingVoucherError::Validation(
            "Previous Year is not closed, please close it first".to_string(),
        ))
    }

    pub fn block_if_future_closing_voucher_exists(
        &self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), PeriodClosingVoucherError> {
        if let Some(future_closing_voucher) = &context.future_closing_voucher {
            let action = if self.docstatus == 2 {
                "cancel"
            } else {
                "create"
            };
            return Err(PeriodClosingVoucherError::Validation(format!(
                "You cannot {action} this document because another Period Closing Entry {future_closing_voucher} exists after {}",
                self.period_end_date()
            )));
        }

        Ok(())
    }

    pub fn check_closing_account_type(
        &self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), PeriodClosingVoucherError> {
        if matches!(
            context.closing_account_root_type.as_deref(),
            Some("Liability" | "Equity")
        ) {
            return Ok(());
        }

        Err(PeriodClosingVoucherError::Validation(format!(
            "Closing Account {} must be of type Liability / Equity",
            self.closing_account_head()
        )))
    }

    pub fn check_closing_account_currency(
        &self,
        context: &PeriodClosingVoucherContext,
    ) -> Result<(), PeriodClosingVoucherError> {
        let account_currency = context
            .closing_account_currency
            .as_deref()
            .unwrap_or_default();
        let company_currency = context.company_currency.as_deref().unwrap_or_default();
        if account_currency != company_currency {
            return Err(PeriodClosingVoucherError::Validation(format!(
                "Currency of the Closing Account must be {company_currency}"
            )));
        }

        Ok(())
    }

    pub fn on_submit(&self, context: &PeriodClosingVoucherContext) -> LifecycleAction {
        if context.use_legacy_controller_for_pcv {
            LifecycleAction::ProcessGlAndClosingEntriesInline
        } else {
            LifecycleAction::CreateAndSubmitProcessPeriodClosingVoucher {
                parent_pcv: self.name(),
            }
        }
    }

    pub fn on_cancel(&self, context: &PeriodClosingVoucherContext) -> Vec<LifecycleAction> {
        let mut actions = Vec::new();
        if !context.use_legacy_controller_for_pcv {
            actions.push(LifecycleAction::CancelProcessPeriodClosingVoucherDocs);
        }
        actions.push(LifecycleAction::SetGleProcessingStatus("In Progress"));
        actions.push(self.cancel_gl_entries(context));
        actions
    }

    pub fn make_gl_entries(&self, context: &PeriodClosingVoucherContext) -> LifecycleAction {
        if context.gl_entry_estimated_count > 100_000 {
            LifecycleAction::EnqueueGlAndClosingEntries {
                timeout_seconds: 1800,
            }
        } else {
            LifecycleAction::ProcessGlAndClosingEntriesInline
        }
    }

    pub fn cancel_gl_entries(&self, context: &PeriodClosingVoucherContext) -> LifecycleAction {
        if context.gle_count_against_current_pcv > 5000 {
            LifecycleAction::EnqueueCancellation {
                queue: "long",
                enqueue_after_commit: true,
            }
        } else {
            LifecycleAction::ProcessCancellationInline
        }
    }

    pub fn ignore_linked_doctypes() -> [&'static str; 5] {
        [
            "GL Entry",
            "Stock Ledger Entry",
            "Payment Ledger Entry",
            "Account Closing Balance",
            "Process Period Closing Voucher",
        ]
    }

    pub fn cancel_process_pcv_docs_plan(&self) -> ProcessPcvDocumentPlan {
        ProcessPcvDocumentPlan {
            doctype: "Process Period Closing Voucher",
            parent_pcv: self.name(),
            docstatus: vec![1],
            action: "cancel",
        }
    }

    pub fn on_trash_delete_process_pcv_docs_plan(&self) -> ProcessPcvDocumentPlan {
        ProcessPcvDocumentPlan {
            doctype: "Process Period Closing Voucher",
            parent_pcv: self.name(),
            docstatus: vec![1, 2],
            action: "delete",
        }
    }

    pub fn get_pcv_gl_entries(
        &self,
        pl_account_balances: &[DimensionBalance],
        closing_account_currency: &str,
    ) -> Vec<PcvGlEntryRow> {
        let mut rows = Vec::new();

        for (dimensions, account_balances) in pl_account_balances {
            for (account, balances) in account_balances {
                let balance_in_company_currency = balances.debit - balances.credit;
                if balance_in_company_currency != 0.0 && account != "balances" {
                    rows.push(self.get_gle_for_pl_account(account, balances, dimensions));
                }
            }

            if let Some(dimension_balance) = account_balances.get("balances") {
                rows.push(self.get_gle_for_closing_account(
                    dimension_balance,
                    dimensions,
                    closing_account_currency,
                ));
            }
        }

        rows
    }

    pub fn get_gle_for_pl_account(
        &self,
        account: &str,
        balances: &AccountBalance,
        dimensions: &[String],
    ) -> PcvGlEntryRow {
        let balance_in_account_currency =
            balances.debit_in_account_currency - balances.credit_in_account_currency;
        let balance_in_company_currency = balances.debit - balances.credit;
        let mut row = self.base_gl_row(account, &balances.account_currency);

        row.debit_in_account_currency = if balance_in_account_currency < 0.0 {
            balance_in_account_currency.abs()
        } else {
            0.0
        };
        row.debit = if balance_in_company_currency < 0.0 {
            balance_in_company_currency.abs()
        } else {
            0.0
        };
        row.credit_in_account_currency = if balance_in_account_currency > 0.0 {
            balance_in_account_currency.abs()
        } else {
            0.0
        };
        row.credit = if balance_in_company_currency > 0.0 {
            balance_in_company_currency.abs()
        } else {
            0.0
        };
        self.update_default_dimensions(&mut row, dimensions);
        row
    }

    pub fn get_gle_for_closing_account(
        &self,
        dimension_balance: &AccountBalance,
        dimensions: &[String],
        closing_account_currency: &str,
    ) -> PcvGlEntryRow {
        let balance_in_company_currency = dimension_balance.balance_in_company_currency;
        let debit = if balance_in_company_currency > 0.0 {
            balance_in_company_currency
        } else {
            0.0
        };
        let credit = if balance_in_company_currency < 0.0 {
            balance_in_company_currency.abs()
        } else {
            0.0
        };
        let mut row = self.base_gl_row(self.closing_account_head(), closing_account_currency);
        row.debit_in_account_currency = debit;
        row.debit = debit;
        row.credit_in_account_currency = credit;
        row.credit = credit;
        self.update_default_dimensions(&mut row, dimensions);
        row
    }

    pub fn update_default_dimensions(&self, row: &mut PcvGlEntryRow, dimensions: &[String]) {
        for (index, dimension) in ["cost_center", "finance_book", "project"]
            .iter()
            .enumerate()
        {
            if let Some(value) = dimensions.get(index) {
                row.dimensions
                    .insert((*dimension).to_string(), value.clone());
            }
        }
    }

    pub fn set_account_balance_dict(
        &self,
        gle: &GlEntryRow,
        balances_by_dimensions: &mut BTreeMap<Vec<String>, BTreeMap<String, AccountBalance>>,
    ) {
        let key = self.get_key(gle);
        let dimension_balances = balances_by_dimensions.entry(key).or_default();
        let account_balance = dimension_balances
            .entry(gle.account.clone())
            .or_insert_with(|| AccountBalance {
                account_currency: gle.account_currency.clone(),
                ..AccountBalance::default()
            });

        account_balance.debit_in_account_currency += gle.debit_in_account_currency;
        account_balance.credit_in_account_currency += gle.credit_in_account_currency;
        account_balance.debit += gle.debit;
        account_balance.credit += gle.credit;

        let balance = dimension_balances
            .entry("balances".to_string())
            .or_insert_with(AccountBalance::default);
        balance.balance_in_account_currency +=
            gle.debit_in_account_currency - gle.credit_in_account_currency;
        balance.balance_in_company_currency += gle.debit - gle.credit;
    }

    pub fn get_key(&self, gle: &GlEntryRow) -> Vec<String> {
        ["cost_center", "finance_book", "project"]
            .iter()
            .map(|dimension| gle.dimensions.get(*dimension).cloned().unwrap_or_default())
            .collect()
    }

    pub fn get_closing_entries_for_pl_accounts(
        &self,
        pl_accounts_reverse_gle: &[PcvGlEntryRow],
    ) -> Vec<PcvGlEntryRow> {
        let mut closing_entries = pl_accounts_reverse_gle.to_vec();
        for row in pl_accounts_reverse_gle {
            let mut row_copy = row.clone();
            row_copy.debit = row.credit;
            row_copy.credit = row.debit;
            row_copy.debit_in_account_currency = row.credit_in_account_currency;
            row_copy.credit_in_account_currency = row.debit_in_account_currency;
            row_copy.is_period_closing_voucher_entry = false;
            row_copy.period_closing_voucher = Some(self.name());
            closing_entries.push(row_copy);
        }

        closing_entries
    }

    pub fn get_closing_entries_for_balance_sheet_accounts(
        &self,
        balance_sheet_account_balances: &[DimensionBalance],
    ) -> Vec<PcvGlEntryRow> {
        let mut closing_entries = Vec::new();

        for (dimensions, account_balances) in balance_sheet_account_balances {
            for (account, balances) in account_balances {
                let balance_in_company_currency = balances.debit - balances.credit;
                if account != "balances" && balance_in_company_currency != 0.0 {
                    closing_entries.push(self.get_closing_entry(account, balances, dimensions));
                }
            }
        }

        closing_entries
    }

    pub fn get_closing_entry(
        &self,
        account: &str,
        balances: &AccountBalance,
        dimensions: &[String],
    ) -> PcvGlEntryRow {
        let mut row = PcvGlEntryRow {
            company: self.company(),
            closing_date: Some(self.period_end_date()),
            period_closing_voucher: Some(self.name()),
            account: account.to_string(),
            account_currency: balances.account_currency.clone(),
            debit_in_account_currency: balances.debit_in_account_currency,
            debit: balances.debit,
            credit_in_account_currency: balances.credit_in_account_currency,
            credit: balances.credit,
            is_period_closing_voucher_entry: false,
            ..PcvGlEntryRow::default()
        };
        self.update_default_dimensions(&mut row, dimensions);
        row
    }

    pub fn get_closing_entries_for_closing_account(
        &self,
        closing_account_gle: &[PcvGlEntryRow],
    ) -> Vec<PcvGlEntryRow> {
        let mut closing_entries = closing_account_gle.to_vec();
        for row in &mut closing_entries {
            row.period_closing_voucher = Some(self.name());
        }
        closing_entries
    }

    pub fn gl_entries_for_current_period_query(
        &self,
        report_type: &str,
        only_opening_entries: bool,
        as_iterator: bool,
        accounting_dimension_fields: &[String],
    ) -> String {
        let date_condition = if only_opening_entries {
            "is_opening = 'Yes'".to_string()
        } else {
            format!(
                "posting_date BETWEEN '{}' AND '{}' and is_opening = 'No'",
                self.period_start_date(),
                self.period_end_date()
            )
        };
        let iterator_suffix = if as_iterator { " AS ITERATOR" } else { "" };

        format!(
            "SELECT name, posting_date, account, account_currency, debit_in_account_currency, credit_in_account_currency, debit, credit, {} FROM `tabGL Entry` WHERE {} AND company = {} AND voucher_type != 'Period Closing Voucher' AND account report_type = {} AND is_cancelled = 0{}",
            accounting_dimension_fields.join(", "),
            date_condition,
            self.company(),
            report_type,
            iterator_suffix
        )
    }

    pub fn delete_closing_entries_filter(voucher_no: &str) -> (&'static str, &str) {
        ("period_closing_voucher", voucher_no)
    }

    pub fn get_future_closing_voucher_filter(&self) -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "period_end_date".to_string(),
                format!(">{}", self.period_end_date()),
            ),
            ("docstatus".to_string(), "1".to_string()),
            ("company".to_string(), self.company()),
        ])
    }

    pub fn is_first_period_closing_voucher(&self, first_pcv: Option<&str>) -> bool {
        first_pcv
            .map(|first_pcv| first_pcv == self.name())
            .unwrap_or(true)
    }

    pub fn accounting_dimension_fields(custom_dimensions: &[String]) -> Vec<String> {
        let mut fields = vec![
            "cost_center".to_string(),
            "finance_book".to_string(),
            "project".to_string(),
        ];
        fields.extend(custom_dimensions.iter().cloned());
        fields
    }

    fn base_gl_row(&self, account: &str, account_currency: &str) -> PcvGlEntryRow {
        PcvGlEntryRow {
            company: self.company(),
            posting_date: self.period_end_date(),
            account: account.to_string(),
            account_currency: account_currency.to_string(),
            is_period_closing_voucher_entry: true,
            voucher_type: "Period Closing Voucher".to_string(),
            voucher_no: self.name(),
            fiscal_year: self.fiscal_year(),
            remarks: self.remarks(),
            is_opening: "No".to_string(),
            ..PcvGlEntryRow::default()
        }
    }

    fn closing_account_head(&self) -> &str {
        self.closing_account_head.as_deref().unwrap_or_default()
    }

    fn company(&self) -> String {
        self.company.clone().unwrap_or_default()
    }

    fn fiscal_year(&self) -> String {
        self.fiscal_year.clone().unwrap_or_default()
    }

    fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    fn period_end_date(&self) -> String {
        self.period_end_date.clone().unwrap_or_default()
    }

    fn period_start_date(&self) -> String {
        self.period_start_date.clone().unwrap_or_default()
    }

    fn remarks(&self) -> String {
        self.remarks.clone().unwrap_or_default()
    }
}

impl DocumentController for PeriodClosingVoucher {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "validate",
            "on_submit",
            "on_cancel",
            "on_trash",
            "make_gl_entries",
        ]
    }
}

pub fn process_gl_and_closing_entries_plan(voucher_name: &str) -> ClosingProcessPlan {
    ClosingProcessPlan {
        voucher_name: voucher_name.to_string(),
        make_gl_entries_merge_entries: false,
        make_closing_entries: true,
        success_status: "Completed",
        failure_status: "Failed",
    }
}

pub fn process_cancellation_plan(voucher_no: &str) -> ProcessCancellationPlan {
    ProcessCancellationPlan {
        voucher_type: "Period Closing Voucher",
        voucher_no: voucher_no.to_string(),
        delete_closing_entries_doctype: "Account Closing Balance",
        success_status: "Completed",
        failure_status: "Failed",
    }
}

pub fn get_period_start_end_date(
    fy_start_date: &str,
    fy_end_date: &str,
    previous_closed_period_end_date: Option<&str>,
) -> (String, String) {
    let period_start_date = previous_closed_period_end_date
        .and_then(|date| add_days(date, 1).ok())
        .unwrap_or_else(|| fy_start_date.to_string());
    (period_start_date, fy_end_date.to_string())
}

fn add_days(date: &str, days: i64) -> Result<String, String> {
    parse_date(date).map(|value| format_date(value + days))
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
    let year = year - (month <= 2) as i64;
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * month + 2) / 5 + day - 1;
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
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    (year + (month <= 2) as i64, month, day)
}
