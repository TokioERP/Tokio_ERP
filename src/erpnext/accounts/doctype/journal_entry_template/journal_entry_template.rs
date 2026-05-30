use std::collections::BTreeMap;

use crate::erpnext::accounts::doctype::journal_entry_template_account::journal_entry_template_account::JournalEntryTemplateAccount;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JournalEntryTemplate {
    pub template_title: String,
    pub voucher_type: Option<String>,
    pub naming_series: Option<String>,
    pub company: Option<String>,
    pub is_opening: Option<String>,
    pub multi_currency: bool,
    pub accounts: Vec<JournalEntryTemplateAccount>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JournalEntryTemplateError {
    PartyTypeOnlyAllowedForReceivableOrPayable { row: usize, account: String },
    PartyRequiresPartyType { row: usize, account: String },
}

impl Default for JournalEntryTemplate {
    fn default() -> Self {
        Self {
            template_title: String::new(),
            voucher_type: None,
            naming_series: None,
            company: None,
            is_opening: None,
            multi_currency: false,
            accounts: Vec::new(),
        }
    }
}

impl JournalEntryTemplate {
    pub const DOCTYPE: &'static str = "Journal Entry Template";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 10] = [
        "section_break_1",
        "template_title",
        "voucher_type",
        "naming_series",
        "column_break_3",
        "company",
        "is_opening",
        "multi_currency",
        "section_break_3",
        "accounts",
    ];
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;
    pub const VOUCHER_TYPE_OPTIONS: &'static str = "Journal Entry\nInter Company Journal Entry\nBank Entry\nCash Entry\nCredit Card Entry\nDebit Note\nCredit Note\nContra Entry\nExcise Entry\nWrite Off Entry\nOpening Entry\nDepreciation Entry\nExchange Rate Revaluation";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("section_break_1"),
            FieldSpec::data("template_title", "Template Title")
                .required()
                .unique(),
            FieldSpec::select("voucher_type", "Journal Entry Type")
                .options(Self::VOUCHER_TYPE_OPTIONS)
                .in_list_view()
                .required(),
            FieldSpec::select("naming_series", "Series")
                .no_copy()
                .print_hide()
                .required()
                .set_only_once(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .in_standard_filter()
                .required(),
            FieldSpec::select("is_opening", "Is Opening")
                .options("No\nYes")
                .default("No"),
            FieldSpec::check("multi_currency", "Multi Currency").default("0"),
            FieldSpec::section_break("section_break_3"),
            FieldSpec::table("accounts", "Accounting Entries")
                .options("Journal Entry Template Account"),
        ]
    }

    pub fn validate(
        &self,
        account_types: &BTreeMap<String, String>,
    ) -> Result<(), JournalEntryTemplateError> {
        self.validate_party(account_types)
    }

    pub fn validate_party(
        &self,
        account_types: &BTreeMap<String, String>,
    ) -> Result<(), JournalEntryTemplateError> {
        for (index, account) in self.accounts.iter().enumerate() {
            let row = index + 1;
            let account_name = account.account.as_deref().unwrap_or_default();
            if account.party_type.is_some() {
                let account_type = account_types.get(account_name).map(String::as_str);
                if !matches!(account_type, Some("Receivable" | "Payable")) {
                    return Err(
                        JournalEntryTemplateError::PartyTypeOnlyAllowedForReceivableOrPayable {
                            row,
                            account: account_name.to_string(),
                        },
                    );
                }
            }

            if account.party.is_some() && account.party_type.is_none() {
                return Err(JournalEntryTemplateError::PartyRequiresPartyType {
                    row,
                    account: account_name.to_string(),
                });
            }
        }
        Ok(())
    }
}

impl DocumentController for JournalEntryTemplate {
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

pub fn get_naming_series(journal_entry_naming_series_options: &str) -> &str {
    journal_entry_naming_series_options
}
