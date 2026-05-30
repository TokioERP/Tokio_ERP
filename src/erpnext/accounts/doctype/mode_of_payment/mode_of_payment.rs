use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::accounts::doctype::mode_of_payment_account::mode_of_payment_account::ModeOfPaymentAccount;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeOfPayment {
    pub name: Option<String>,
    pub mode_of_payment: String,
    pub enabled: bool,
    pub payment_type: Option<String>,
    pub accounts: Vec<ModeOfPaymentAccount>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeOfPaymentError {
    DuplicateCompany,
    AccountCompanyMismatch {
        default_account: String,
        company: String,
        mode_of_payment: String,
    },
    UsedInPosProfile {
        pos_profiles: Vec<String>,
        mode_of_payment: String,
    },
}

impl Default for ModeOfPayment {
    fn default() -> Self {
        Self {
            name: None,
            mode_of_payment: String::new(),
            enabled: true,
            payment_type: None,
            accounts: Vec::new(),
        }
    }
}

impl ModeOfPayment {
    pub const DOCTYPE: &'static str = "Mode of Payment";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] = ["mode_of_payment", "enabled", "type", "accounts"];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const AUTONAME: &'static str = "field:mode_of_payment";
    pub const DOCUMENT_TYPE: &'static str = "Setup";
    pub const ICON: &'static str = "fa fa-credit-card";
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SHOW_NAME_IN_GLOBAL_SEARCH: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "ASC";
    pub const TRANSLATED_DOCTYPE: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("mode_of_payment", "Mode of Payment")
                .oldfield("mode_of_payment", "Data")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::check("enabled", "Enabled").default("1"),
            FieldSpec::select("type", "Type")
                .options("Cash\nBank\nGeneral\nPhone")
                .in_standard_filter(),
            FieldSpec::table("accounts", "Accounts").options("Mode of Payment Account"),
        ]
    }

    pub fn validate(
        &self,
        account_companies: &BTreeMap<String, String>,
        pos_profiles_using_mode: &[String],
    ) -> Result<(), ModeOfPaymentError> {
        self.validate_accounts(account_companies)?;
        self.validate_repeating_companies()?;
        self.validate_pos_mode_of_payment(pos_profiles_using_mode)
    }

    pub fn validate_repeating_companies(&self) -> Result<(), ModeOfPaymentError> {
        let mut companies = BTreeSet::new();
        for entry in &self.accounts {
            if !companies.insert(entry.company.as_deref().unwrap_or_default()) {
                return Err(ModeOfPaymentError::DuplicateCompany);
            }
        }
        Ok(())
    }

    pub fn validate_accounts(
        &self,
        account_companies: &BTreeMap<String, String>,
    ) -> Result<(), ModeOfPaymentError> {
        for entry in &self.accounts {
            let default_account = entry.default_account.as_deref().unwrap_or_default();
            let company = entry.company.as_deref().unwrap_or_default();
            if account_companies.get(default_account).map(String::as_str) != Some(company) {
                return Err(ModeOfPaymentError::AccountCompanyMismatch {
                    default_account: default_account.to_string(),
                    company: company.to_string(),
                    mode_of_payment: self.mode_name(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_pos_mode_of_payment(
        &self,
        pos_profiles_using_mode: &[String],
    ) -> Result<(), ModeOfPaymentError> {
        if !self.enabled && !pos_profiles_using_mode.is_empty() {
            return Err(ModeOfPaymentError::UsedInPosProfile {
                pos_profiles: pos_profiles_using_mode.to_vec(),
                mode_of_payment: self.mode_name(),
            });
        }
        Ok(())
    }

    fn mode_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| self.mode_of_payment.clone())
    }
}

impl DocumentController for ModeOfPayment {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}
