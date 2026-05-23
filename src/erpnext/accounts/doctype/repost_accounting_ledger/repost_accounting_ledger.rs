use std::collections::BTreeSet;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostAccountingLedger {
    pub name: String,
    pub company: Option<String>,
    pub delete_cancelled_entries: bool,
    pub vouchers: Vec<RepostVoucher>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostVoucher {
    pub voucher_type: String,
    pub voucher_no: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VoucherPostingDate {
    pub voucher_type: String,
    pub voucher_no: String,
    pub posting_date: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostValidationContext {
    pub allowed_types: Vec<String>,
    pub latest_period_closing_voucher: Option<String>,
    pub voucher_posting_dates: Vec<VoucherPostingDate>,
    pub deferred_sales_documents: Vec<String>,
    pub deferred_purchase_documents: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepostSubmitPlan {
    StartRepost {
        account_repost_doc: String,
    },
    Enqueue {
        method: String,
        job_name: String,
        account_repost_doc: String,
        enqueue_after_commit: bool,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostStartContext {
    pub repost_docstatus: i32,
    pub hook_allowed_doctypes: Vec<String>,
    pub doc_capabilities: Vec<RepostDocCapability>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepostDocCapability {
    pub doctype: String,
    pub has_make_gl_entries: bool,
    pub make_gl_entries_supports_cancel_arg: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepostAction {
    SetThroughRepostFlag,
    DeleteLedgerEntries {
        ledger_doctype: String,
        voucher_type: String,
        voucher_no: String,
    },
    SetDocStatus {
        voucher_type: String,
        voucher_no: String,
        docstatus: i32,
    },
    MakeGlEntriesOnCancel {
        voucher_type: String,
        voucher_no: String,
        from_repost: bool,
    },
    ForceSetAgainstIncomeAccount {
        voucher_no: String,
    },
    ForceSetAgainstExpenseAccount {
        voucher_no: String,
    },
    MakeGlEntries {
        voucher_type: String,
        voucher_no: String,
    },
    MakeGlEntriesFromRepost {
        voucher_type: String,
        voucher_no: String,
    },
    MakeGlEntriesCancelArg {
        voucher_type: String,
        voucher_no: String,
    },
    MakeReverseGlEntries {
        voucher_type: String,
        voucher_no: String,
    },
}

impl RepostVoucher {
    pub fn new(voucher_type: impl Into<String>, voucher_no: impl Into<String>) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
        }
    }
}

impl VoucherPostingDate {
    pub fn new(
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
        posting_date: impl Into<String>,
    ) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
            posting_date: posting_date.into(),
        }
    }
}

impl RepostAccountingLedger {
    pub const DOCTYPE: &'static str = "Repost Accounting Ledger";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "company",
        "column_break_vpup",
        "delete_cancelled_entries",
        "section_break_metl",
        "vouchers",
        "amended_from",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(name: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            company: Some(company.into()),
            delete_cancelled_entries: false,
            vouchers: Vec::new(),
            amended_from: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::column_break("column_break_vpup"),
            FieldSpec::check(
                "delete_cancelled_entries",
                "Delete Cancelled Ledger Entries",
            )
            .default("0"),
            FieldSpec::section_break("section_break_metl"),
            FieldSpec::table("vouchers", "Vouchers").options("Repost Accounting Ledger Items"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Repost Accounting Ledger")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(&self, context: &RepostValidationContext) -> Result<(), String> {
        self.validate_vouchers(&context.allowed_types)?;
        self.validate_for_closed_fiscal_year(context)?;
        self.validate_for_deferred_accounting(context)
    }

    pub fn validate_vouchers(&self, allowed_types: &[impl AsRef<str>]) -> Result<(), String> {
        if self.vouchers.is_empty() {
            return Ok(());
        }
        let voucher_types = self
            .vouchers
            .iter()
            .map(|voucher| voucher.voucher_type.as_str())
            .collect::<Vec<_>>();
        validate_docs_for_voucher_types(&voucher_types, allowed_types)
    }

    pub fn validate_for_deferred_accounting(
        &self,
        context: &RepostValidationContext,
    ) -> Result<(), String> {
        let sales_docs = self
            .vouchers
            .iter()
            .filter(|voucher| voucher.voucher_type == "Sales Invoice")
            .map(|voucher| voucher.voucher_no.as_str())
            .collect::<Vec<_>>();
        let purchase_docs = self
            .vouchers
            .iter()
            .filter(|voucher| voucher.voucher_type == "Purchase Invoice")
            .map(|voucher| voucher.voucher_no.as_str())
            .collect::<Vec<_>>();
        validate_docs_for_deferred_accounting(
            &sales_docs,
            &purchase_docs,
            &context.deferred_sales_documents,
            &context.deferred_purchase_documents,
        )
    }

    pub fn validate_for_closed_fiscal_year(
        &self,
        context: &RepostValidationContext,
    ) -> Result<(), String> {
        if self.vouchers.is_empty() {
            return Ok(());
        }
        let Some(latest_pcv) = context.latest_period_closing_voucher.as_deref() else {
            return Ok(());
        };

        for allowed_type in &context.allowed_types {
            let names = self
                .vouchers
                .iter()
                .filter(|voucher| voucher.voucher_type == *allowed_type)
                .map(|voucher| voucher.voucher_no.as_str())
                .collect::<Vec<_>>();
            if names.is_empty() {
                continue;
            }

            let latest_voucher = context
                .voucher_posting_dates
                .iter()
                .filter(|posting| posting.voucher_type == *allowed_type)
                .filter(|posting| names.contains(&posting.voucher_no.as_str()))
                .map(|posting| posting.posting_date.as_str())
                .max();

            if latest_voucher.is_some_and(|posting_date| latest_pcv >= posting_date) {
                return Err(
                    "Cannot Resubmit Ledger entries for vouchers in Closed fiscal year."
                        .to_string(),
                );
            }
        }

        Ok(())
    }

    pub fn generate_preview(&self) -> Result<String, String> {
        if self.vouchers.is_empty() {
            return Err("Add vouchers to generate preview.".to_string());
        }
        Ok(
            "erpnext/accounts/doctype/repost_accounting_ledger/repost_accounting_ledger.html"
                .to_string(),
        )
    }

    pub fn on_submit(&self) -> RepostSubmitPlan {
        if self.vouchers.len() > 5 {
            RepostSubmitPlan::Enqueue {
                method: "erpnext.accounts.doctype.repost_accounting_ledger.repost_accounting_ledger.start_repost".to_string(),
                job_name: format!("repost_accounting_ledger_{}", self.name),
                account_repost_doc: self.name.clone(),
                enqueue_after_commit: true,
            }
        } else {
            RepostSubmitPlan::StartRepost {
                account_repost_doc: self.name.clone(),
            }
        }
    }

    pub fn start_repost(&self, context: &RepostStartContext) -> Vec<RepostAction> {
        let mut actions = vec![RepostAction::SetThroughRepostFlag];

        if context.repost_docstatus != 1 {
            return actions;
        }

        for voucher in &self.vouchers {
            if self.delete_cancelled_entries {
                for ledger_doctype in [
                    "GL Entry",
                    "Payment Ledger Entry",
                    "Advance Payment Ledger Entry",
                ] {
                    actions.push(RepostAction::DeleteLedgerEntries {
                        ledger_doctype: ledger_doctype.to_string(),
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                    });
                }
            }

            match voucher.voucher_type.as_str() {
                "Sales Invoice" => {
                    if !self.delete_cancelled_entries {
                        actions.push(RepostAction::SetDocStatus {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                            docstatus: 2,
                        });
                        actions.push(RepostAction::MakeGlEntriesOnCancel {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                            from_repost: true,
                        });
                    }
                    actions.push(RepostAction::SetDocStatus {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                        docstatus: 1,
                    });
                    actions.push(RepostAction::ForceSetAgainstIncomeAccount {
                        voucher_no: voucher.voucher_no.clone(),
                    });
                    actions.push(RepostAction::MakeGlEntries {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                    });
                }
                "Purchase Invoice" => {
                    if !self.delete_cancelled_entries {
                        actions.push(RepostAction::SetDocStatus {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                            docstatus: 2,
                        });
                        actions.push(RepostAction::MakeGlEntriesOnCancel {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                            from_repost: true,
                        });
                    }
                    actions.push(RepostAction::SetDocStatus {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                        docstatus: 1,
                    });
                    actions.push(RepostAction::ForceSetAgainstExpenseAccount {
                        voucher_no: voucher.voucher_no.clone(),
                    });
                    actions.push(RepostAction::MakeGlEntries {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                    });
                }
                "Purchase Receipt" => {
                    if !self.delete_cancelled_entries {
                        actions.push(RepostAction::SetDocStatus {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                            docstatus: 2,
                        });
                        actions.push(RepostAction::MakeGlEntriesOnCancel {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                            from_repost: true,
                        });
                    }
                    actions.push(RepostAction::SetDocStatus {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                        docstatus: 1,
                    });
                    actions.push(RepostAction::MakeGlEntriesFromRepost {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                    });
                }
                "Payment Entry" | "Journal Entry" | "Expense Claim" => {
                    if !self.delete_cancelled_entries {
                        actions.push(RepostAction::MakeGlEntriesCancelArg {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                        });
                    }
                    actions.push(RepostAction::MakeGlEntries {
                        voucher_type: voucher.voucher_type.clone(),
                        voucher_no: voucher.voucher_no.clone(),
                    });
                }
                doctype
                    if context
                        .hook_allowed_doctypes
                        .iter()
                        .any(|hook| hook == doctype) =>
                {
                    if let Some(capability) = context.doc_capabilities.iter().find(|capability| {
                        capability.doctype == doctype && capability.has_make_gl_entries
                    }) {
                        if !self.delete_cancelled_entries {
                            if capability.make_gl_entries_supports_cancel_arg {
                                actions.push(RepostAction::MakeGlEntriesCancelArg {
                                    voucher_type: voucher.voucher_type.clone(),
                                    voucher_no: voucher.voucher_no.clone(),
                                });
                            } else {
                                actions.push(RepostAction::MakeReverseGlEntries {
                                    voucher_type: voucher.voucher_type.clone(),
                                    voucher_no: voucher.voucher_no.clone(),
                                });
                            }
                        }
                        actions.push(RepostAction::MakeGlEntries {
                            voucher_type: voucher.voucher_type.clone(),
                            voucher_no: voucher.voucher_no.clone(),
                        });
                    }
                }
                _ => {}
            }
        }

        actions
    }
}

impl DocumentController for RepostAccountingLedger {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_submit"]
    }
}

pub fn get_allowed_types_from_settings(
    repost_docs: &[String],
    child_doc: bool,
    child_tables: &[(String, Vec<String>)],
) -> Vec<String> {
    let mut result = repost_docs.to_vec();
    if child_doc {
        for repost_doc in repost_docs {
            if let Some((_, children)) = child_tables
                .iter()
                .find(|(doctype, _)| doctype == repost_doc)
            {
                result.extend(children.iter().cloned());
            }
        }
    }
    result
}

pub fn get_repost_allowed_types(allowed_types: &[String], txt: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    allowed_types
        .iter()
        .filter(|doctype| txt.is_empty() || doctype.contains(txt))
        .filter(|doctype| seen.insert((*doctype).clone()))
        .cloned()
        .collect()
}

pub fn validate_docs_for_voucher_types(
    doc_voucher_types: &[impl AsRef<str>],
    allowed_types: &[impl AsRef<str>],
) -> Result<(), String> {
    let allowed = allowed_types
        .iter()
        .map(|doctype| doctype.as_ref())
        .collect::<BTreeSet<_>>();
    let disallowed = doc_voucher_types
        .iter()
        .map(|doctype| doctype.as_ref())
        .filter(|doctype| !allowed.contains(doctype))
        .collect::<BTreeSet<_>>();

    if disallowed.is_empty() {
        return Ok(());
    }

    let documents = comma_and(disallowed.iter().copied().collect::<Vec<_>>().as_slice());
    let message = if disallowed.len() > 1 { "are" } else { "is" };
    Err(format!(
        "<b>{documents}</b> {message} not allowed to be reposted. You can enable it by adding it '<b>Allowed Doctype</b>' table in Accounts Settings."
    ))
}

pub fn validate_docs_for_deferred_accounting(
    sales_docs: &[impl AsRef<str>],
    purchase_docs: &[impl AsRef<str>],
    docs_with_deferred_revenue: &[impl AsRef<str>],
    docs_with_deferred_expense: &[impl AsRef<str>],
) -> Result<(), String> {
    let sales = sales_docs
        .iter()
        .map(|doc| doc.as_ref())
        .collect::<BTreeSet<_>>();
    let purchase = purchase_docs
        .iter()
        .map(|doc| doc.as_ref())
        .collect::<BTreeSet<_>>();

    let mut deferred = Vec::new();
    deferred.extend(
        docs_with_deferred_expense
            .iter()
            .map(|doc| doc.as_ref())
            .filter(|doc| purchase.contains(doc)),
    );
    deferred.extend(
        docs_with_deferred_revenue
            .iter()
            .map(|doc| doc.as_ref())
            .filter(|doc| sales.contains(doc)),
    );

    if deferred.is_empty() {
        return Ok(());
    }

    Err(format!(
        "Documents: <b>{}</b> have deferred revenue/expense enabled for them. Cannot repost.",
        comma_and(&deferred)
    ))
}

fn comma_and(items: &[&str]) -> String {
    match items {
        [] => String::new(),
        [one] => (*one).to_string(),
        [first, second] => format!("{first} and {second}"),
        many => {
            let (last, rest) = many.split_last().expect("non-empty slice");
            format!("{}, and {}", rest.join(", "), last)
        }
    }
}
