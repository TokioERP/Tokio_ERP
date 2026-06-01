use crate::erpnext::accounts::doctype::closed_document::closed_document::ClosedDocument;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingPeriod {
    pub name: Option<String>,
    pub period_name: String,
    pub start_date: String,
    pub end_date: String,
    pub company: Option<String>,
    pub disabled: bool,
    pub exempted_role: Option<String>,
    pub closed_documents: Vec<ClosedDocument>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingPeriodOverlap {
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosedAccountingPeriodMatch {
    pub name: String,
    pub exempted_role: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingPeriodDocSaveContext {
    pub doctype: String,
    pub company: String,
    pub posting_date: Option<String>,
    pub asset_type: Option<String>,
    pub available_for_use_date: Option<String>,
    pub completion_date: Option<String>,
    pub period_end_date: Option<String>,
    pub accounting_period: Option<ClosedAccountingPeriodMatch>,
    pub user_roles: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountingPeriodError {
    Overlap {
        accounting_period: String,
    },
    ClosedAccountingPeriod {
        doctype: String,
        accounting_period: String,
    },
}

impl AccountingPeriod {
    pub const DOCTYPE: &'static str = "Accounting Period";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 9] = [
        "period_name",
        "start_date",
        "end_date",
        "column_break_4",
        "company",
        "disabled",
        "exempted_role",
        "section_break_7",
        "closed_documents",
    ];
    pub const AUTONAME: &'static str = "field:period_name";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("period_name", "Period Name")
                .required()
                .unique(),
            FieldSpec::date("start_date", "Start Date")
                .in_list_view()
                .required(),
            FieldSpec::date("end_date", "End Date")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .required(),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .in_list_view(),
            FieldSpec::link("exempted_role", "Exempted Role")
                .options("Role")
                .description("Role allowed to bypass period restrictions."),
            FieldSpec::section_break("section_break_7"),
            FieldSpec::table("closed_documents", "Closed Documents")
                .options("Closed Document")
                .required(),
        ]
    }

    pub fn autoname_with_company_abbr(&mut self, company_abbr: &str) -> String {
        let name = [self.period_name.as_str(), company_abbr].join(" - ");
        self.name = Some(name.clone());
        name
    }

    pub fn validate_overlap(
        &self,
        existing_accounting_periods: &[AccountingPeriodOverlap],
    ) -> Result<(), AccountingPeriodError> {
        if let Some(existing) = existing_accounting_periods.first() {
            return Err(AccountingPeriodError::Overlap {
                accounting_period: existing.name.clone(),
            });
        }
        Ok(())
    }

    pub fn get_doctypes_for_closing(doctypes: &[String]) -> Vec<ClosedDocument> {
        doctypes
            .iter()
            .map(|doctype| ClosedDocument::new(doctype, true))
            .collect()
    }

    pub fn bootstrap_doctypes_for_closing(&mut self, doctypes: &[String]) {
        if self.closed_documents.is_empty() {
            self.closed_documents = Self::get_doctypes_for_closing(doctypes);
        }
    }
}

impl DocumentController for AccountingPeriod {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "before_insert", "autoname"]
    }
}

pub fn validate_accounting_period_on_doc_save(
    ctx: &AccountingPeriodDocSaveContext,
) -> Result<(), AccountingPeriodError> {
    let _date = match ctx.doctype.as_str() {
        "Bank Clearance" => return Ok(()),
        "Asset" if ctx.asset_type.as_deref() == Some("Existing Asset") => return Ok(()),
        "Asset" => ctx.available_for_use_date.clone(),
        "Asset Repair" => ctx.completion_date.clone(),
        "Period Closing Voucher" => ctx.period_end_date.clone(),
        _ => ctx.posting_date.clone(),
    };

    if let Some(accounting_period) = &ctx.accounting_period {
        let exempted = accounting_period
            .exempted_role
            .as_ref()
            .is_some_and(|role| ctx.user_roles.contains(role));
        if !exempted {
            return Err(AccountingPeriodError::ClosedAccountingPeriod {
                doctype: ctx.doctype.clone(),
                accounting_period: accounting_period.name.clone(),
            });
        }
    }

    Ok(())
}
