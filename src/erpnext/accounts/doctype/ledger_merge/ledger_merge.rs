use crate::erpnext::accounts::doctype::ledger_merge_accounts::ledger_merge_accounts::LedgerMergeAccounts;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerMerge {
    pub name: Option<String>,
    pub root_type: Option<String>,
    pub account: Option<String>,
    pub account_name: Option<String>,
    pub company: Option<String>,
    pub status: String,
    pub is_group: bool,
    pub merge_accounts: Vec<LedgerMergeAccounts>,
}

impl Default for LedgerMerge {
    fn default() -> Self {
        Self {
            name: None,
            root_type: None,
            account: None,
            account_name: None,
            company: None,
            status: "Pending".to_string(),
            is_group: false,
            merge_accounts: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LedgerMergeStartContext {
    pub scheduler_inactive: bool,
    pub in_test: bool,
    pub developer_mode: bool,
    pub job_enqueued: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerMergeEnqueuePlan {
    pub queue: &'static str,
    pub timeout: u32,
    pub event: &'static str,
    pub job_id: String,
    pub docname: String,
    pub now: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerMergeError {
    SchedulerInactive,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerMergeRunResult {
    pub status: Option<String>,
    pub successful_merges: usize,
    pub total: usize,
    pub progress_events: Vec<(usize, usize)>,
    pub refresh_published: bool,
    pub errors_logged: usize,
}

impl LedgerMerge {
    pub const DOCTYPE: &'static str = "Ledger Merge";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 10] = [
        "section_break_1",
        "root_type",
        "account",
        "account_name",
        "column_break_3",
        "company",
        "status",
        "is_group",
        "section_break_5",
        "merge_accounts",
    ];
    pub const AUTONAME: &'static str = "format:{account_name} merger on {creation}";
    pub const NAMING_RULE: &'static str = "Expression";
    pub const HIDE_TOOLBAR: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .depends_on("root_type")
                .required()
                .set_only_once(),
            FieldSpec::section_break("section_break_1"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::table("merge_accounts", "Accounts to Merge")
                .options("Ledger Merge Accounts")
                .required(),
            FieldSpec::section_break("section_break_5").depends_on("account"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .set_only_once(),
            FieldSpec::select("status", "Status")
                .options("Pending\nSuccess\nPartial Success\nError")
                .in_list_view()
                .read_only(),
            FieldSpec::select("root_type", "Root Type")
                .options("\nAsset\nLiability\nIncome\nExpense\nEquity")
                .required()
                .set_only_once(),
            FieldSpec::data("account_name", "Account Name")
                .depends_on("account")
                .fetch_from("account.account_name")
                .fetch_if_empty()
                .read_only()
                .required(),
            FieldSpec::check("is_group", "Is Group")
                .default("0")
                .depends_on("account")
                .fetch_from("account.is_group")
                .read_only(),
        ]
    }

    pub fn start_merge_plan(
        &self,
        ctx: LedgerMergeStartContext,
    ) -> Result<Option<LedgerMergeEnqueuePlan>, LedgerMergeError> {
        if ctx.scheduler_inactive && !ctx.in_test {
            return Err(LedgerMergeError::SchedulerInactive);
        }

        let docname = self.name.clone().unwrap_or_default();
        let job_id = format!("ledger_merge::{docname}");
        if ctx.job_enqueued {
            return Ok(None);
        }

        Ok(Some(LedgerMergeEnqueuePlan {
            queue: "default",
            timeout: 6000,
            event: "ledger_merge",
            job_id,
            docname,
            now: ctx.developer_mode || ctx.in_test,
        }))
    }
}

impl DocumentController for LedgerMerge {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn run_ledger_merge(
    ledger_merge: &mut LedgerMerge,
    merge_results: &[Result<(), ()>],
) -> LedgerMergeRunResult {
    let mut successful_merges = 0;
    let total = ledger_merge.merge_accounts.len();
    let mut progress_events = Vec::new();
    let mut errors_logged = 0;
    let mut status = None;
    let mut result_idx = 0;

    for row in &mut ledger_merge.merge_accounts {
        if row.merged {
            continue;
        }

        let merge_result = merge_results.get(result_idx).copied().unwrap_or(Ok(()));
        result_idx += 1;
        if merge_result.is_ok() {
            row.merged = true;
            successful_merges += 1;
            progress_events.push((successful_merges, total));
        } else {
            errors_logged += 1;
        }

        let next_status = if successful_merges == total {
            "Success"
        } else if successful_merges > 0 {
            "Partial Success"
        } else {
            "Error"
        };
        ledger_merge.status = next_status.to_string();
        status = Some(ledger_merge.status.clone());
    }

    LedgerMergeRunResult {
        status,
        successful_merges,
        total,
        progress_events,
        refresh_published: true,
        errors_logged,
    }
}
