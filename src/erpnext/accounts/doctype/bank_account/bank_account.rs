use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankAccount {
    pub name: Option<String>,
    pub account: Option<String>,
    pub account_name: String,
    pub account_subtype: Option<String>,
    pub account_type: Option<String>,
    pub bank: String,
    pub bank_account_no: Option<String>,
    pub branch_code: Option<String>,
    pub company: Option<String>,
    pub disabled: bool,
    pub iban: Option<String>,
    pub integration_id: Option<String>,
    pub is_company_account: bool,
    pub is_default: bool,
    pub last_integration_date: Option<String>,
    pub mask: Option<String>,
    pub party: Option<String>,
    pub party_type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressAndContactAction {
    pub doctype: &'static str,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankAccountValidationError {
    CompanyMandatoryForCompanyAccount,
    CompanyAccountMandatory,
    AccountAlreadyUsed { account: String, links: Vec<String> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankAccountFilter {
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub is_company_account: bool,
    pub company: Option<String>,
    pub is_default: bool,
    pub disabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultBankAccountUpdate {
    pub doctype: &'static str,
    pub filters: BankAccountFilter,
    pub fieldname: &'static str,
    pub value: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankAccountLookupPlan {
    pub doctype: &'static str,
    pub filters: BTreeMap<String, String>,
    pub fieldname: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankAccountDetailsPlan {
    pub permission_doctype: &'static str,
    pub permission_doc: String,
    pub permission_type: &'static str,
    pub cached_doctype: &'static str,
    pub cached_name: String,
    pub fields: [&'static str; 3],
}

impl BankAccount {
    pub const DOCTYPE: &'static str = "Bank Account";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:account_name-bank";
    pub const FIELD_ORDER: [&'static str; 28] = [
        "account_name",
        "account",
        "bank",
        "account_type",
        "account_subtype",
        "column_break_7",
        "disabled",
        "is_default",
        "is_company_account",
        "company",
        "section_break_11",
        "party_type",
        "column_break_14",
        "party",
        "account_details_section",
        "iban",
        "column_break_12",
        "branch_code",
        "bank_account_no",
        "address_and_contact",
        "address_html",
        "column_break_13",
        "contact_html",
        "integration_details_section",
        "integration_id",
        "last_integration_date",
        "column_break_27",
        "mask",
    ];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const DOCUMENT_TYPE: &'static str = "Setup";
    pub const SEARCH_FIELDS: &'static str = "bank,account";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(account_name: impl Into<String>, bank: impl Into<String>) -> Self {
        Self {
            account_name: account_name.into(),
            bank: bank.into(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("account_name", "Account Name")
                .required()
                .in_global_search()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::link("account", "Company Account")
                .options("Account")
                .depends_on("is_company_account")
                .mandatory_depends_on("is_company_account")
                .in_list_view(),
            FieldSpec::link("bank", "Bank").options("Bank").required(),
            FieldSpec::link("account_type", "Account Type").options("Bank Account Type"),
            FieldSpec::link("account_subtype", "Account Subtype").options("Bank Account Subtype"),
            FieldSpec::column_break("column_break_7").search_index(),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::check("is_default", "Is Default Account").default("0"),
            FieldSpec::check("is_company_account", "Is Company Account")
                .default("0")
                .description(
                    "Setting the account as a Company Account is necessary for Bank Reconciliation",
                ),
            FieldSpec::link("company", "Company")
                .options("Company")
                .depends_on("is_company_account")
                .mandatory_depends_on("is_company_account")
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::section_break("section_break_11")
                .label("Party Details")
                .depends_on("eval:!doc.is_company_account"),
            FieldSpec::link("party_type", "Party Type").options("DocType"),
            FieldSpec::column_break("column_break_14"),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type"),
            FieldSpec::section_break("account_details_section").label("Account Details"),
            FieldSpec::data("iban", "IBAN")
                .options("IBAN")
                .length(34)
                .in_list_view(),
            FieldSpec::column_break("column_break_12"),
            FieldSpec::data("branch_code", "Branch Code").in_global_search(),
            FieldSpec::data("bank_account_no", "Bank Account No")
                .length(30)
                .in_list_view(),
            FieldSpec::section_break("address_and_contact")
                .label("Address and Contact")
                .options("fa fa-map-marker"),
            FieldSpec::html("address_html", "Address HTML"),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::html("contact_html", "Contact HTML"),
            FieldSpec::section_break("integration_details_section").label("Integration Details"),
            FieldSpec::data("integration_id", "Integration ID")
                .hidden()
                .no_copy()
                .read_only()
                .unique(),
            FieldSpec::date("last_integration_date", "Last Integration Date").description(
                "Change this date manually to setup the next synchronization start date",
            ),
            FieldSpec::column_break("column_break_27"),
            FieldSpec::data("mask", "Mask").read_only(),
        ]
    }

    pub fn onload(&self) -> AddressAndContactAction {
        AddressAndContactAction {
            doctype: Self::DOCTYPE,
            name: self.name.clone().unwrap_or_default(),
        }
    }

    pub fn autoname(&mut self) -> String {
        let name = format!("{} - {}", self.account_name, self.bank);
        self.name = Some(name.clone());
        name
    }

    pub fn on_trash(&self) -> AddressAndContactAction {
        AddressAndContactAction {
            doctype: Self::DOCTYPE,
            name: self.name.clone().unwrap_or_default(),
        }
    }

    pub fn validate(
        &self,
        bank_accounts_using_same_account: &[String],
    ) -> Result<Option<DefaultBankAccountUpdate>, BankAccountValidationError> {
        self.validate_is_company_account(bank_accounts_using_same_account)?;
        Ok(self.update_default_bank_account())
    }

    pub fn validate_is_company_account(
        &self,
        bank_accounts_using_same_account: &[String],
    ) -> Result<(), BankAccountValidationError> {
        if self.is_company_account {
            if self.company.is_none() {
                return Err(BankAccountValidationError::CompanyMandatoryForCompanyAccount);
            }

            if self.account.is_none() {
                return Err(BankAccountValidationError::CompanyAccountMandatory);
            }

            self.validate_account(bank_accounts_using_same_account)?;
        }

        Ok(())
    }

    pub fn validate_account(
        &self,
        bank_accounts_using_same_account: &[String],
    ) -> Result<(), BankAccountValidationError> {
        if !bank_accounts_using_same_account.is_empty() {
            return Err(BankAccountValidationError::AccountAlreadyUsed {
                account: self.account.clone().unwrap_or_default(),
                links: bank_accounts_using_same_account
                    .iter()
                    .map(|name| format!("{}/{}", Self::DOCTYPE, name))
                    .collect(),
            });
        }

        Ok(())
    }

    pub fn update_default_bank_account(&self) -> Option<DefaultBankAccountUpdate> {
        if self.is_default && !self.disabled {
            return Some(DefaultBankAccountUpdate {
                doctype: Self::DOCTYPE,
                filters: BankAccountFilter {
                    party_type: self.party_type.clone(),
                    party: self.party.clone(),
                    is_company_account: self.is_company_account,
                    company: self.company.clone(),
                    is_default: true,
                    disabled: false,
                },
                fieldname: "is_default",
                value: false,
            });
        }

        None
    }
}

impl DocumentController for BankAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["onload", "autoname", "on_trash", "validate"]
    }
}

pub fn get_party_bank_account_plan(party_type: &str, party: &str) -> BankAccountLookupPlan {
    BankAccountLookupPlan {
        doctype: "Bank Account",
        filters: BTreeMap::from([
            ("party_type".to_string(), party_type.to_string()),
            ("party".to_string(), party.to_string()),
            ("is_default".to_string(), "1".to_string()),
            ("disabled".to_string(), "0".to_string()),
        ]),
        fieldname: "name",
    }
}

pub fn get_default_company_bank_account(
    company: &str,
    party_default_bank_account: Option<&str>,
    party_default_bank_account_company: Option<&str>,
    fallback_company_bank_account: Option<&str>,
) -> Option<String> {
    if let Some(default_company_bank_account) = party_default_bank_account {
        if party_default_bank_account_company == Some(company) {
            return Some(default_company_bank_account.to_string());
        }
    }

    fallback_company_bank_account.map(str::to_string)
}

pub fn get_bank_account_details_plan(bank_account: &str) -> BankAccountDetailsPlan {
    BankAccountDetailsPlan {
        permission_doctype: "Bank Account",
        permission_doc: bank_account.to_string(),
        permission_type: "read",
        cached_doctype: "Bank Account",
        cached_name: bank_account.to_string(),
        fields: ["account", "bank", "bank_account_no"],
    }
}
