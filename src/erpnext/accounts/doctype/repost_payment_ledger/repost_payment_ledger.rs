use crate::erpnext::{DocumentController, FieldSpec};

pub const VOUCHER_TYPES: [&str; 4] = [
    "Sales Invoice",
    "Purchase Invoice",
    "Payment Entry",
    "Journal Entry",
];

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostPaymentLedger {
    pub name: String,
    pub company: String,
    pub posting_date: String,
    pub voucher_type: Option<String>,
    pub add_manually: bool,
    pub repost_status: Option<String>,
    pub repost_error_log: Option<String>,
    pub repost_vouchers: Vec<RepostPaymentVoucher>,
    pub amended_from: Option<String>,
    pub docstatus: i32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostPaymentVoucher {
    pub voucher_type: String,
    pub voucher_no: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VoucherSource {
    pub voucher_type: String,
    pub voucher_no: String,
    pub company: String,
    pub posting_date: String,
    pub docstatus: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentLedgerRepostJob {
    pub method: String,
    pub docname: String,
    pub job_name: String,
    pub is_async: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentLedgerRunContext {
    pub docstatus: i32,
    pub repost_status: String,
    pub fail_traceback: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentLedgerRepostAction {
    BuildGlMap {
        voucher_type: String,
        voucher_no: String,
    },
    GetGlEntries {
        voucher_type: String,
        voucher_no: String,
    },
    DeletePaymentLedgerEntries {
        voucher_type: String,
        voucher_no: String,
    },
    DeleteAdvancePaymentLedgerEntries {
        voucher_type: String,
        voucher_no: String,
    },
    CreatePaymentLedgerEntry {
        cancel: bool,
    },
    SetRepostErrorLog(String),
    SetRepostStatus(String),
    Rollback,
}

impl RepostPaymentVoucher {
    pub fn new(voucher_type: impl Into<String>, voucher_no: impl Into<String>) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
        }
    }
}

impl VoucherSource {
    pub fn new(
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
        company: impl Into<String>,
        posting_date: impl Into<String>,
        docstatus: i32,
    ) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
            company: company.into(),
            posting_date: posting_date.into(),
            docstatus,
        }
    }
}

impl PaymentLedgerRunContext {
    pub fn submitted(repost_status: impl Into<String>) -> Self {
        Self {
            docstatus: 1,
            repost_status: repost_status.into(),
            fail_traceback: None,
        }
    }
}

impl RepostPaymentLedger {
    pub const DOCTYPE: &'static str = "Repost Payment Ledger";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 12] = [
        "filters_section",
        "company",
        "posting_date",
        "column_break_4",
        "voucher_type",
        "add_manually",
        "status_section",
        "repost_status",
        "repost_error_log",
        "selected_vouchers_section",
        "repost_vouchers",
        "amended_from",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        name: impl Into<String>,
        company: impl Into<String>,
        posting_date: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            company: company.into(),
            posting_date: posting_date.into(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("filters_section").label("Filters"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .required(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("voucher_type", "Voucher Type").options("DocType"),
            FieldSpec::check("add_manually", "Add Manually")
                .default("0")
                .description("Ignore Voucher Type filter and Select Vouchers Manually"),
            FieldSpec::section_break("status_section").label("Status"),
            FieldSpec::select("repost_status", "Repost Status")
                .options("\nQueued\nFailed\nCompleted")
                .read_only(),
            FieldSpec::long_text("repost_error_log", "Repost Error Log")
                .depends_on("eval:doc.repost_error_log"),
            FieldSpec::section_break("selected_vouchers_section").label("Vouchers"),
            FieldSpec::table("repost_vouchers", "Selected Vouchers")
                .options("Repost Payment Ledger Items"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Repost Payment Ledger")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn before_validate(&mut self, sources: &[VoucherSource]) {
        self.load_vouchers_based_on_filters(sources);
        self.set_status();
    }

    pub fn load_vouchers_based_on_filters(&mut self, sources: &[VoucherSource]) {
        if !self.add_manually {
            self.repost_vouchers = self.get_vouchers(sources);
        }
    }

    pub fn get_vouchers(&self, sources: &[VoucherSource]) -> Vec<RepostPaymentVoucher> {
        let filter_on_voucher_types = self
            .voucher_type
            .as_ref()
            .map(|voucher_type| vec![voucher_type.as_str()])
            .unwrap_or_else(|| VOUCHER_TYPES.to_vec());

        let mut vouchers = Vec::new();
        for voucher_type in filter_on_voucher_types {
            vouchers.extend(
                sources
                    .iter()
                    .filter(|source| source.voucher_type == voucher_type)
                    .filter(|source| source.docstatus == 1)
                    .filter(|source| source.company == self.company)
                    .filter(|source| source.posting_date >= self.posting_date)
                    .map(|source| RepostPaymentVoucher {
                        voucher_type: source.voucher_type.clone(),
                        voucher_no: source.voucher_no.clone(),
                    }),
            );
        }
        vouchers
    }

    pub fn set_status(&mut self) {
        if self.docstatus == 0 {
            self.repost_status = Some("Queued".to_string());
        }
    }

    pub fn on_submit(&self) -> PaymentLedgerRepostJob {
        execute_repost_payment_ledger(&self.name)
    }
}

impl DocumentController for RepostPaymentLedger {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["before_validate", "on_submit"]
    }
}

pub fn repost_ple_for_voucher(
    voucher_type: &str,
    voucher_no: &str,
    has_gl_entries: bool,
) -> Vec<PaymentLedgerRepostAction> {
    if voucher_type.is_empty() || voucher_no.is_empty() || !has_gl_entries {
        return Vec::new();
    }

    vec![
        PaymentLedgerRepostAction::DeletePaymentLedgerEntries {
            voucher_type: voucher_type.to_string(),
            voucher_no: voucher_no.to_string(),
        },
        PaymentLedgerRepostAction::DeleteAdvancePaymentLedgerEntries {
            voucher_type: voucher_type.to_string(),
            voucher_no: voucher_no.to_string(),
        },
        PaymentLedgerRepostAction::CreatePaymentLedgerEntry { cancel: false },
    ]
}

pub fn execute_repost_payment_ledger(docname: &str) -> PaymentLedgerRepostJob {
    PaymentLedgerRepostJob {
        method: "erpnext.accounts.doctype.repost_payment_ledger.repost_payment_ledger.start_payment_ledger_repost".to_string(),
        docname: docname.to_string(),
        job_name: format!("payment_ledger_repost_{docname}"),
        is_async: true,
    }
}

pub fn start_payment_ledger_repost(
    doc: &RepostPaymentLedger,
    context: &PaymentLedgerRunContext,
) -> Vec<PaymentLedgerRepostAction> {
    if context.docstatus != 1 || !matches!(context.repost_status.as_str(), "Queued" | "Failed") {
        return Vec::new();
    }

    if let Some(traceback) = &context.fail_traceback {
        return vec![
            PaymentLedgerRepostAction::Rollback,
            PaymentLedgerRepostAction::SetRepostErrorLog(format!("Traceback: <br>{traceback}")),
            PaymentLedgerRepostAction::SetRepostStatus("Failed".to_string()),
        ];
    }

    let mut actions = Vec::new();
    for entry in &doc.repost_vouchers {
        if matches!(
            entry.voucher_type.as_str(),
            "Payment Entry" | "Journal Entry"
        ) {
            actions.push(PaymentLedgerRepostAction::BuildGlMap {
                voucher_type: entry.voucher_type.clone(),
                voucher_no: entry.voucher_no.clone(),
            });
        } else {
            actions.push(PaymentLedgerRepostAction::GetGlEntries {
                voucher_type: entry.voucher_type.clone(),
                voucher_no: entry.voucher_no.clone(),
            });
        }

        actions.extend(repost_ple_for_voucher(
            &entry.voucher_type,
            &entry.voucher_no,
            true,
        ));
    }
    actions.push(PaymentLedgerRepostAction::SetRepostErrorLog(String::new()));
    actions.push(PaymentLedgerRepostAction::SetRepostStatus(
        "Completed".to_string(),
    ));
    actions
}
