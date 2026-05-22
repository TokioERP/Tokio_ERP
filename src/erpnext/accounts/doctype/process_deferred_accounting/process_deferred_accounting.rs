use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessDeferredAccounting {
    pub name: Option<String>,
    pub account: Option<String>,
    pub amended_from: Option<String>,
    pub company: Option<String>,
    pub end_date: Option<String>,
    pub posting_date: Option<String>,
    pub start_date: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeferredAccountingConversion {
    RevenueToIncome {
        process_name: String,
        start_date: String,
        end_date: String,
        accounting_type: String,
        account: Option<String>,
        company: String,
    },
    ExpenseToExpense {
        process_name: String,
        start_date: String,
        end_date: String,
        accounting_type: String,
        account: Option<String>,
        company: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredAccountingCancellation {
    pub ignore_linked_doctypes: [&'static str; 1],
    pub against_voucher_type: &'static str,
    pub against_voucher: String,
    pub cancel_gl_entries: bool,
}

impl ProcessDeferredAccounting {
    pub const DOCTYPE: &'static str = "Process Deferred Accounting";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-PDA-.#####";
    pub const FIELD_ORDER: [&'static str; 8] = [
        "company",
        "type",
        "account",
        "column_break_3",
        "posting_date",
        "start_date",
        "end_date",
        "amended_from",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;

    pub fn new(
        name: impl Into<String>,
        posting_date: impl Into<String>,
        start_date: impl Into<String>,
        end_date: impl Into<String>,
        accounting_type: impl Into<String>,
        company: impl Into<String>,
    ) -> Self {
        Self {
            name: Some(name.into()),
            posting_date: Some(posting_date.into()),
            start_date: Some(start_date.into()),
            end_date: Some(end_date.into()),
            r#type: Some(accounting_type.into()),
            company: Some(company.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::select("type", "Type")
                .options("\nIncome\nExpense")
                .required()
                .in_list_view(),
            FieldSpec::link("account", "Account")
                .options("Account")
                .depends_on("eval: doc.type"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .required()
                .in_list_view(),
            FieldSpec::date("start_date", "Service Start Date")
                .required()
                .in_list_view(),
            FieldSpec::date("end_date", "Service End Date")
                .required()
                .in_list_view(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Deferred Accounting")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if let (Some(start_date), Some(end_date)) = (&self.start_date, &self.end_date) {
            if end_date < start_date {
                return Err("End date cannot be before start date");
            }
        }

        Ok(())
    }

    pub fn on_submit(&self) -> DeferredAccountingConversion {
        let process_name = self.name.clone().unwrap_or_default();
        let start_date = self.start_date.clone().unwrap_or_default();
        let end_date = self.end_date.clone().unwrap_or_default();
        let accounting_type = self.r#type.clone().unwrap_or_default();
        let company = self.company.clone().unwrap_or_default();
        let account = self.account.clone();

        if accounting_type == "Income" {
            DeferredAccountingConversion::RevenueToIncome {
                process_name,
                start_date,
                end_date,
                accounting_type,
                account,
                company,
            }
        } else {
            DeferredAccountingConversion::ExpenseToExpense {
                process_name,
                start_date,
                end_date,
                accounting_type,
                account,
                company,
            }
        }
    }

    pub fn on_cancel(&self) -> DeferredAccountingCancellation {
        DeferredAccountingCancellation {
            ignore_linked_doctypes: ["GL Entry"],
            against_voucher_type: Self::DOCTYPE,
            against_voucher: self.name.clone().unwrap_or_default(),
            cancel_gl_entries: true,
        }
    }
}

impl DocumentController for ProcessDeferredAccounting {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_submit", "on_cancel"]
    }
}
