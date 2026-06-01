use std::collections::HashSet;

use crate::erpnext::accounts::doctype::pos_opening_entry_detail::pos_opening_entry_detail::PosOpeningEntryDetail;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosOpeningEntry {
    pub name: Option<String>,
    pub docstatus: i32,
    pub period_start_date: String,
    pub period_end_date: Option<String>,
    pub status: String,
    pub posting_date: String,
    pub set_posting_date: bool,
    pub company: String,
    pub pos_profile: String,
    pub pos_closing_entry: Option<String>,
    pub user: String,
    pub balance_details: Vec<PosOpeningEntryDetail>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosOpeningEntryValidationContext {
    pub pos_profile_exists: bool,
    pub pos_profile_company: Option<String>,
    pub pos_profile_disabled: bool,
    pub user_enabled: bool,
    pub open_pos_exists: bool,
    pub open_user_exists: bool,
    pub modes_with_default_account: HashSet<String>,
    pub unconsolidated_invoices_exist: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PosOpeningEntryError {
    PosProfileMissing {
        pos_profile: String,
    },
    PosProfileDisabled {
        pos_profile: String,
    },
    PosProfileWrongCompany {
        pos_profile: String,
        company: String,
    },
    UserDisabled {
        user: String,
    },
    OpenPosExists {
        pos_profile: String,
    },
    UserAlreadyAssigned,
    MissingPaymentMethodAccount {
        modes: Vec<String>,
    },
    UnconsolidatedInvoicesExist,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PosOpeningCancelEvent {
    pub event: String,
    pub operation: String,
    pub docname: String,
}

impl Default for PosOpeningEntry {
    fn default() -> Self {
        Self {
            name: None,
            docstatus: 0,
            period_start_date: String::new(),
            period_end_date: None,
            status: "Draft".to_string(),
            posting_date: String::new(),
            set_posting_date: false,
            company: String::new(),
            pos_profile: String::new(),
            pos_closing_entry: None,
            user: String::new(),
            balance_details: Vec::new(),
            amended_from: None,
        }
    }
}

impl PosOpeningEntry {
    pub const DOCTYPE: &'static str = "POS Opening Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "POS-OPE-.YYYY.-.#####";
    pub const FIELD_ORDER: [&'static str; 16] = [
        "period_start_date",
        "period_end_date",
        "status",
        "column_break_3",
        "posting_date",
        "set_posting_date",
        "section_break_5",
        "company",
        "pos_profile",
        "pos_closing_entry",
        "column_break_7",
        "user",
        "opening_balance_details_section",
        "balance_details",
        "section_break_9",
        "amended_from",
    ];
    pub const IS_SUBMITTABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::datetime("period_start_date", "Period Start Date")
                .in_list_view()
                .required(),
            FieldSpec::date("period_end_date", "Period End Date")
                .in_list_view()
                .read_only(),
            FieldSpec::select("status", "Status")
                .options("Draft\nOpen\nClosed\nCancelled")
                .default("Draft")
                .allow_on_submit()
                .hidden()
                .read_only(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::check("set_posting_date", "Set Posting Date").default("0"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("pos_profile", "POS Profile")
                .options("POS Profile")
                .in_list_view()
                .required(),
            FieldSpec::data("pos_closing_entry", "POS Closing Entry")
                .allow_on_submit()
                .read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("user", "Cashier")
                .options("User")
                .required(),
            FieldSpec::section_break("opening_balance_details_section"),
            FieldSpec::table("balance_details", "Opening Balance Details")
                .options("POS Opening Entry Detail")
                .required(),
            FieldSpec::section_break("section_break_9").read_only(),
            FieldSpec::link("amended_from", "Amended From")
                .options("POS Opening Entry")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn validate(
        &self,
        ctx: &PosOpeningEntryValidationContext,
    ) -> Result<(), PosOpeningEntryError> {
        self.validate_pos_profile_and_cashier(ctx)?;
        self.check_open_pos_exists(ctx)?;
        self.check_user_already_assigned(ctx)?;
        self.validate_payment_method_account(ctx)
    }

    pub fn validate_pos_profile_and_cashier(
        &self,
        ctx: &PosOpeningEntryValidationContext,
    ) -> Result<(), PosOpeningEntryError> {
        if !ctx.pos_profile_exists {
            return Err(PosOpeningEntryError::PosProfileMissing {
                pos_profile: self.pos_profile.clone(),
            });
        }
        if ctx.pos_profile_disabled {
            return Err(PosOpeningEntryError::PosProfileDisabled {
                pos_profile: self.pos_profile.clone(),
            });
        }
        if ctx.pos_profile_company.as_deref() != Some(self.company.as_str()) {
            return Err(PosOpeningEntryError::PosProfileWrongCompany {
                pos_profile: self.pos_profile.clone(),
                company: self.company.clone(),
            });
        }
        if !ctx.user_enabled {
            return Err(PosOpeningEntryError::UserDisabled {
                user: self.user.clone(),
            });
        }
        Ok(())
    }

    pub fn check_open_pos_exists(
        &self,
        ctx: &PosOpeningEntryValidationContext,
    ) -> Result<(), PosOpeningEntryError> {
        if ctx.open_pos_exists {
            return Err(PosOpeningEntryError::OpenPosExists {
                pos_profile: self.pos_profile.clone(),
            });
        }
        Ok(())
    }

    pub fn check_user_already_assigned(
        &self,
        ctx: &PosOpeningEntryValidationContext,
    ) -> Result<(), PosOpeningEntryError> {
        if ctx.open_user_exists {
            return Err(PosOpeningEntryError::UserAlreadyAssigned);
        }
        Ok(())
    }

    pub fn validate_payment_method_account(
        &self,
        ctx: &PosOpeningEntryValidationContext,
    ) -> Result<(), PosOpeningEntryError> {
        let mut invalid_modes = Vec::new();
        for row in &self.balance_details {
            if let Some(mode_of_payment) = row.mode_of_payment.as_deref() {
                if !ctx.modes_with_default_account.contains(mode_of_payment) {
                    invalid_modes.push(mode_of_payment.to_string());
                }
            }
        }
        if invalid_modes.is_empty() {
            Ok(())
        } else {
            Err(PosOpeningEntryError::MissingPaymentMethodAccount {
                modes: invalid_modes,
            })
        }
    }

    pub fn check_poe_is_cancellable(
        &self,
        ctx: &PosOpeningEntryValidationContext,
    ) -> Result<(), PosOpeningEntryError> {
        if ctx.unconsolidated_invoices_exist {
            Err(PosOpeningEntryError::UnconsolidatedInvoicesExist)
        } else {
            Ok(())
        }
    }

    pub fn set_status(&mut self) {
        self.status = if self.docstatus == 2 {
            "Cancelled"
        } else if self.docstatus == 1 && self.pos_closing_entry.is_some() {
            "Closed"
        } else if self.docstatus == 1 {
            "Open"
        } else {
            "Draft"
        }
        .to_string();
    }

    pub fn on_cancel_event(&self) -> PosOpeningCancelEvent {
        let name = self.name.clone().unwrap_or_default();
        PosOpeningCancelEvent {
            event: format!("poe_{name}"),
            operation: "Cancelled".to_string(),
            docname: format!("POS Opening Entry/{name}"),
        }
    }
}

impl DocumentController for PosOpeningEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_submit", "before_cancel", "on_cancel"]
    }
}
