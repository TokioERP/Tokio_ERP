use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessPaymentReconciliation {
    pub name: Option<String>,
    pub amended_from: Option<String>,
    pub bank_cash_account: Option<String>,
    pub company: Option<String>,
    pub cost_center: Option<String>,
    pub default_advance_account: Option<String>,
    pub error_log: Option<String>,
    pub from_invoice_date: Option<String>,
    pub from_payment_date: Option<String>,
    pub party: Option<String>,
    pub party_type: Option<String>,
    pub receivable_payable_account: Option<String>,
    pub status: Option<String>,
    pub to_invoice_date: Option<String>,
    pub to_payment_date: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessPaymentReconciliationCancellation {
    pub log_name: Option<String>,
    pub log_status: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardData {
    pub fieldname: &'static str,
    pub transactions: [DashboardTransaction; 1],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardTransaction {
    pub label: &'static str,
    pub items: [&'static str; 1],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListIndicator {
    pub status: String,
    pub color: &'static str,
    pub filter: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentReconciliationSeed {
    pub company: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub receivable_payable_account: Option<String>,
    pub default_advance_account: Option<String>,
    pub from_invoice_date: Option<String>,
    pub to_invoice_date: Option<String>,
    pub from_payment_date: Option<String>,
    pub to_payment_date: Option<String>,
    pub invoice_limit: usize,
    pub payment_limit: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Allocation {
    pub idx: usize,
    pub reference_type: String,
    pub reference_name: String,
    pub reconciled: bool,
}

impl Allocation {
    pub fn new(
        idx: usize,
        reference_type: impl Into<String>,
        reference_name: impl Into<String>,
        reconciled: bool,
    ) -> Self {
        Self {
            idx,
            reference_type: reference_type.into(),
            reference_name: reference_name.into(),
            reconciled,
        }
    }
}

impl ProcessPaymentReconciliation {
    pub const DOCTYPE: &'static str = "Process Payment Reconciliation";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "format:ACC-PPR-{#####}";
    pub const TITLE_FIELD: &'static str = "company";
    pub const FIELD_ORDER: [&'static str; 20] = [
        "company",
        "party_type",
        "column_break_io6c",
        "party",
        "receivable_payable_account",
        "default_advance_account",
        "filter_section",
        "from_invoice_date",
        "to_invoice_date",
        "column_break_kegk",
        "from_payment_date",
        "to_payment_date",
        "column_break_uj04",
        "cost_center",
        "bank_cash_account",
        "section_break_2n02",
        "status",
        "error_log",
        "section_break_a8yx",
        "amended_from",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;

    pub fn new(
        name: impl Into<String>,
        company: impl Into<String>,
        party_type: impl Into<String>,
        party: impl Into<String>,
        receivable_payable_account: impl Into<String>,
        default_advance_account: impl Into<String>,
    ) -> Self {
        Self {
            name: Some(name.into()),
            company: Some(company.into()),
            party_type: Some(party_type.into()),
            party: Some(party.into()),
            receivable_payable_account: Some(receivable_payable_account.into()),
            default_advance_account: Some(default_advance_account.into()),
            ..Self::default()
        }
    }

    pub fn with_bank_cash_account(mut self, bank_cash_account: impl Into<String>) -> Self {
        self.bank_cash_account = Some(bank_cash_account.into());
        self
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_io6c"),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .required()
                .in_list_view(),
            FieldSpec::link("receivable_payable_account", "Receivable/Payable Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("default_advance_account", "Default Advance Account")
                .options("Account")
                .required()
                .depends_on("eval:doc.party")
                .mandatory_depends_on("doc.party_type")
                .description(
                    "Only 'Payment Entries' made against this advance account are supported.",
                )
                .documentation_url(
                    "https://docs.erpnext.com/docs/user/manual/en/advance-in-separate-party-account",
                ),
            FieldSpec::section_break("filter_section").label("Filters"),
            FieldSpec::date("from_invoice_date", "From Invoice Date"),
            FieldSpec::date("to_invoice_date", "To Invoice Date"),
            FieldSpec::column_break("column_break_kegk"),
            FieldSpec::date("from_payment_date", "From Payment Date"),
            FieldSpec::date("to_payment_date", "To Payment Date"),
            FieldSpec::column_break("column_break_uj04"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::link("bank_cash_account", "Bank/Cash Account").options("Account"),
            FieldSpec::section_break("section_break_2n02").label("Status"),
            FieldSpec::select("status", "Status")
                .options(
                    "\nQueued\nRunning\nPaused\nCompleted\nPartially Reconciled\nFailed\nCancelled",
                )
                .read_only()
                .allow_on_submit(),
            FieldSpec::long_text("error_log", "Error Log").depends_on("eval:doc.error_log"),
            FieldSpec::section_break("section_break_a8yx"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Payment Reconciliation")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn on_discard(&mut self) {
        self.status = Some("Cancelled".to_string());
    }

    pub fn validate(
        &self,
        receivable_payable_account_company: Option<&str>,
        bank_cash_account_company: Option<&str>,
    ) -> Result<(), String> {
        self.validate_receivable_payable_account(receivable_payable_account_company)?;
        self.validate_bank_cash_account(bank_cash_account_company)
    }

    pub fn validate_receivable_payable_account(
        &self,
        account_company: Option<&str>,
    ) -> Result<(), String> {
        if let Some(account) = &self.receivable_payable_account {
            if self.company.as_deref() != account_company {
                return Err(format!(
                    "Receivable/Payable Account: {account} doesn't belong to company {}",
                    self.company.as_deref().unwrap_or_default()
                ));
            }
        }

        Ok(())
    }

    pub fn validate_bank_cash_account(&self, account_company: Option<&str>) -> Result<(), String> {
        if let Some(account) = &self.bank_cash_account {
            if self.company.as_deref() != account_company {
                return Err(format!(
                    "Bank/Cash Account {account} doesn't belong to company {}",
                    self.company.as_deref().unwrap_or_default()
                ));
            }
        }

        Ok(())
    }

    pub fn before_save(&mut self) {
        self.status = Some(String::new());
        self.error_log = Some(String::new());
    }

    pub fn on_submit(&mut self) {
        self.status = Some("Queued".to_string());
        self.error_log = None;
    }

    pub fn on_cancel(&mut self, log: Option<&str>) -> ProcessPaymentReconciliationCancellation {
        self.status = Some("Cancelled".to_string());
        ProcessPaymentReconciliationCancellation {
            log_name: log.map(ToString::to_string),
            log_status: log.map(|_| "Cancelled".to_string()),
        }
    }

    pub fn dashboard_data() -> DashboardData {
        DashboardData {
            fieldname: "process_pr",
            transactions: [DashboardTransaction {
                label: "Reconciliation Logs",
                items: ["Process Payment Reconciliation Log"],
            }],
        }
    }

    pub fn indicator_for_status(status: &str) -> Option<ListIndicator> {
        let color = match status {
            "Queued" | "Paused" | "Partially Reconciled" => "orange",
            "Completed" => "green",
            "Running" => "blue",
            "Failed" => "red",
            _ => return None,
        };

        Some(ListIndicator {
            status: status.to_string(),
            color,
            filter: format!("status,=,{status}"),
        })
    }

    pub fn reconciled_count(log_counts: Option<(u64, u64)>) -> Option<(u64, u64)> {
        log_counts
    }

    pub fn payment_reconciliation_seed(&self) -> PaymentReconciliationSeed {
        PaymentReconciliationSeed {
            company: self.company.clone(),
            party_type: self.party_type.clone(),
            party: self.party.clone(),
            receivable_payable_account: self.receivable_payable_account.clone(),
            default_advance_account: self.default_advance_account.clone(),
            from_invoice_date: self.from_invoice_date.clone(),
            to_invoice_date: self.to_invoice_date.clone(),
            from_payment_date: self.from_payment_date.clone(),
            to_payment_date: self.to_payment_date.clone(),
            invoice_limit: 1000,
            payment_limit: 1000,
        }
    }
}

pub fn get_next_allocation(allocations: &[Allocation]) -> Vec<Allocation> {
    let mut pending: Vec<_> = allocations
        .iter()
        .filter(|allocation| !allocation.reconciled)
        .cloned()
        .collect();
    pending.sort_by_key(|allocation| allocation.idx);

    let Some(first) = pending.first() else {
        return Vec::new();
    };

    let reference_type = first.reference_type.clone();
    let reference_name = first.reference_name.clone();
    pending
        .into_iter()
        .filter(|allocation| {
            allocation.reference_type == reference_type
                && allocation.reference_name == reference_name
        })
        .collect()
}

pub fn reconcile_job_name(doc: &str, allocations: &[Allocation]) -> String {
    match (allocations.first(), allocations.last()) {
        (Some(first), Some(last)) => {
            format!(
                "process_{doc}_reconcile_allocation_{}_{}",
                first.idx, last.idx
            )
        }
        _ => format!("process_{doc}_reconcile"),
    }
}

impl DocumentController for ProcessPaymentReconciliation {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "on_discard",
            "validate",
            "before_save",
            "on_submit",
            "on_cancel",
        ]
    }
}
