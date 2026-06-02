use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Account {
    pub name: String,
    pub account_name: String,
    pub account_number: Option<String>,
    pub account_category: Option<String>,
    pub account_currency: Option<String>,
    pub account_type: Option<String>,
    pub balance_must_be: String,
    pub company: String,
    pub disabled: bool,
    pub freeze_account: String,
    pub include_in_gross: bool,
    pub is_group: bool,
    pub lft: i32,
    pub old_parent: Option<String>,
    pub parent_account: Option<String>,
    pub report_type: Option<String>,
    pub rgt: i32,
    pub root_type: Option<String>,
    pub tax_rate: f64,
    pub currency_explicitly_specified: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ParentAccountInfo {
    pub name: String,
    pub is_group: bool,
    pub company: String,
    pub account_type: Option<String>,
    pub report_type: Option<String>,
    pub root_type: Option<String>,
    pub account_name: String,
    pub account_number: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CompanyInfo {
    pub name: String,
    pub abbr: String,
    pub default_currency: String,
    pub role_allowed_for_frozen_entries: Option<String>,
    pub allow_account_creation_against_child_company: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountContext {
    pub allow_unverified_charts: bool,
    pub ignore_root_company_validation: bool,
    pub parent: Option<ParentAccountInfo>,
    pub company: Option<CompanyInfo>,
    pub old_doc: Option<Account>,
    pub user_roles: BTreeSet<String>,
    pub gle_exists: bool,
    pub child_exists: bool,
    pub default_accounts: BTreeMap<String, String>,
    pub default_account_labels: BTreeMap<String, String>,
    pub balance: f64,
    pub gl_currency: Option<String>,
    pub any_gl_entry_exists: bool,
    pub same_number_account: Option<String>,
    pub root_company: Option<String>,
    pub root_company_has_account: bool,
    pub allow_account_creation_against_child_company: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountRecord {
    pub name: String,
    pub account_name: String,
    pub account_number: Option<String>,
    pub company: String,
    pub is_group: bool,
    pub docstatus: i32,
    pub root_type: Option<String>,
    pub account_currency: Option<String>,
    pub parent_account: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConversionPlan {
    pub is_group: bool,
    pub save: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChildCompanyUpdate {
    pub company: String,
    pub field_updates: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChildCompanyAccountPlan {
    Create {
        company: String,
        account_currency: String,
        parent_account: String,
    },
    Update {
        company: String,
        account: String,
        field_updates: BTreeMap<String, String>,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RenamePlan {
    pub company: String,
    pub old_account_name: String,
    pub account_name: String,
    pub account_number: Option<String>,
    pub old_account_number: Option<String>,
    pub from_descendant: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UpdateAccountNumberInput {
    pub name: String,
    pub account_name: String,
    pub account_number: Option<String>,
    pub old_account_name: String,
    pub old_account_number: Option<String>,
    pub company: CompanyInfo,
    pub descendants: Vec<String>,
    pub from_descendant: bool,
    pub parent_company_accounts: BTreeMap<String, String>,
    pub allow_independent_account_creation: bool,
    pub same_number_account: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UpdateAccountNumberPlan {
    pub account_number_value: Option<String>,
    pub account_name_value: String,
    pub new_name: Option<String>,
    pub child_sync: Vec<RenamePlan>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MergeAccountInput {
    pub old: AccountRecord,
    pub new: AccountRecord,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MergeAccountPlan {
    pub rename_old: String,
    pub merge_into: String,
    pub update_new_parent_account: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountError {
    ParentMissing {
        account: String,
        parent: String,
    },
    ParentSelf {
        account: String,
    },
    ParentIsLedger {
        account: String,
        parent: String,
    },
    ParentWrongCompany {
        account: String,
        parent: String,
        company: String,
    },
    ParentSameType {
        account_type: String,
    },
    RootNotEditable,
    RootMustBeGroup {
        account: String,
    },
    RootTypeMandatory,
    ReportTypeMandatory,
    CannotDisableDefaultAccount {
        account: String,
        field_label: String,
        company: String,
    },
    CannotConvertDefaultAccountToGroup {
        account: String,
        field_label: String,
        company: String,
    },
    CannotConvertTransactionAccountToLedger,
    CannotConvertTypedAccountToGroup,
    CannotConvertAccountWithChildrenToLedger,
    UnauthorizedFrozenAccountModifier,
    DebitBalanceCannotBeCredit,
    CreditBalanceCannotBeDebit,
    CurrencyChangeAfterEntries,
    DuplicateAccountNumber {
        account_number: String,
        account: String,
    },
    RootCompanyAccountMissing {
        root_company: String,
    },
    ChildCompanyParentMissing {
        company: String,
        parent: String,
    },
    ChildCompanyParentIsLedger {
        company: String,
        parent: String,
    },
    AccountWithExistingTransactionCannotBeDeleted,
    RenameNotAllowed {
        account: String,
        parent_company: String,
    },
    NewAccountMissing {
        account: String,
    },
    InvalidAccountMerge,
    SystemInUse,
}

impl Account {
    pub const DOCTYPE: &'static str = "Account";
    pub const MODULE: &'static str = "Accounts";
    pub const NSM_PARENT_FIELD: &'static str = "parent_account";
    pub const FIELD_ORDER: [&'static str; 21] = [
        "properties",
        "column_break0",
        "disabled",
        "account_name",
        "account_number",
        "is_group",
        "company",
        "root_type",
        "report_type",
        "account_currency",
        "column_break1",
        "parent_account",
        "account_category",
        "account_type",
        "tax_rate",
        "freeze_account",
        "balance_must_be",
        "lft",
        "rgt",
        "old_parent",
        "include_in_gross",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("properties"),
            FieldSpec::column_break("column_break0").width("50%"),
            FieldSpec::check("disabled", "Disable").default("0"),
            FieldSpec::data("account_name", "Account Name")
                .in_list_view()
                .no_copy()
                .oldfield("account_name", "Data")
                .required(),
            FieldSpec::data("account_number", "Account Number")
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::check("is_group", "Is Group").default("0"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .fetch_from("parent_account.company")
                .fetch_if_empty()
                .in_standard_filter()
                .oldfield("company", "Link")
                .required(),
            FieldSpec::select("root_type", "Root Type")
                .options("\nAsset\nLiability\nIncome\nExpense\nEquity")
                .in_standard_filter()
                .read_only(),
            FieldSpec::select("report_type", "Report Type")
                .options("\nBalance Sheet\nProfit and Loss")
                .in_standard_filter()
                .read_only(),
            FieldSpec::link("account_currency", "Currency")
                .options("Currency")
                .depends_on("eval:doc.is_group==0"),
            FieldSpec::column_break("column_break1").width("50%"),
            FieldSpec::link("parent_account", "Parent Account")
                .options("Account")
                .ignore_user_permissions()
                .oldfield("parent_account", "Link")
                .required()
                .search_index(),
            FieldSpec::link("account_category", "Account Category")
                .options("Account Category")
                .description("Used with Financial Report Template"),
            FieldSpec::select("account_type", "Account Type")
                .options(ACCOUNT_TYPE_OPTIONS)
                .description(
                    "Setting Account Type helps in selecting this Account in transactions.",
                )
                .oldfield("account_type", "Select")
                .in_standard_filter()
                .search_index(),
            FieldSpec::float("tax_rate", "Tax Rate")
                .description("Rate at which this tax is applied")
                .oldfield("tax_rate", "Currency"),
            FieldSpec::select("freeze_account", "Frozen")
                .options("No\nYes")
                .description("If the account is frozen, entries are allowed to restricted users.")
                .oldfield("freeze_account", "Select"),
            FieldSpec::select("balance_must_be", "Balance must be").options("\nDebit\nCredit"),
            FieldSpec::int("lft", "Lft")
                .hidden()
                .print_hide()
                .read_only()
                .search_index(),
            FieldSpec::int("rgt", "Rgt")
                .hidden()
                .print_hide()
                .read_only()
                .search_index(),
            FieldSpec::data("old_parent", "Old Parent")
                .hidden()
                .print_hide()
                .read_only(),
            FieldSpec::check("include_in_gross", "Include in gross")
                .default("0")
                .depends_on("eval:(doc.report_type == 'Profit and Loss' && !doc.is_group)"),
        ]
    }

    pub fn validate(&mut self, context: &mut AccountContext) -> Result<(), AccountError> {
        if context.allow_unverified_charts {
            return Ok(());
        }
        self.validate_parent(context)?;
        self.validate_parent_child_account_type(context)?;
        self.validate_root_details(context)?;
        self.validate_account_number(context)?;
        self.validate_disabled(context)?;
        self.validate_group_or_ledger(context)?;
        self.set_root_and_report_type(context);
        self.validate_mandatory()?;
        self.validate_frozen_accounts_modifier(context)?;
        self.validate_balance_must_be_debit_or_credit(context)?;
        self.validate_account_currency(context)?;
        self.validate_root_company_and_sync_account_to_children(context)?;
        self.validate_receivable_payable_account_type(context);
        Ok(())
    }

    pub fn onload_can_freeze_account(
        &self,
        company: &CompanyInfo,
        roles: &BTreeSet<String>,
    ) -> bool {
        company
            .role_allowed_for_frozen_entries
            .as_deref()
            .map(|role| roles.contains(role))
            .unwrap_or(true)
    }

    pub fn autoname(&mut self, company: &CompanyInfo) -> Result<(), AccountError> {
        self.name =
            get_account_autoname(self.account_number.as_deref(), &self.account_name, company)?;
        Ok(())
    }

    pub fn validate_parent(&self, context: &AccountContext) -> Result<(), AccountError> {
        let Some(parent_account) = self.parent_account.as_deref() else {
            return Ok(());
        };
        let Some(parent) = context.parent.as_ref() else {
            return Err(AccountError::ParentMissing {
                account: self.name.clone(),
                parent: parent_account.to_string(),
            });
        };
        if parent.name == self.name {
            return Err(AccountError::ParentSelf {
                account: self.name.clone(),
            });
        }
        if !parent.is_group {
            return Err(AccountError::ParentIsLedger {
                account: self.name.clone(),
                parent: parent_account.to_string(),
            });
        }
        if parent.company != self.company {
            return Err(AccountError::ParentWrongCompany {
                account: self.name.clone(),
                parent: parent_account.to_string(),
                company: self.company.clone(),
            });
        }
        Ok(())
    }

    pub fn validate_parent_child_account_type(
        &self,
        context: &AccountContext,
    ) -> Result<(), AccountError> {
        if self.parent_account.is_some()
            && PARENT_ONLY_ACCOUNT_TYPES.contains(&self.account_type.as_deref().unwrap_or_default())
            && context
                .parent
                .as_ref()
                .and_then(|parent| parent.account_type.as_deref())
                == self.account_type.as_deref()
        {
            return Err(AccountError::ParentSameType {
                account_type: self.account_type.clone().unwrap_or_default(),
            });
        }
        Ok(())
    }

    pub fn validate_root_details(&self, context: &AccountContext) -> Result<(), AccountError> {
        if context
            .old_doc
            .as_ref()
            .is_some_and(|old| old.parent_account.is_none())
        {
            return Err(AccountError::RootNotEditable);
        }
        if self.parent_account.is_none() && !self.is_group {
            return Err(AccountError::RootMustBeGroup {
                account: self.name.clone(),
            });
        }
        Ok(())
    }

    pub fn set_root_and_report_type(&mut self, context: &AccountContext) {
        if self.parent_account.is_some() {
            if let Some(parent) = context.parent.as_ref() {
                if parent.report_type.is_some() {
                    self.report_type = parent.report_type.clone();
                }
                if parent.root_type.is_some() {
                    self.root_type = parent.root_type.clone();
                }
            }
        }
        if self.root_type.is_some() && self.report_type.is_none() {
            self.report_type = Some(
                if matches!(
                    self.root_type.as_deref(),
                    Some("Asset" | "Liability" | "Equity")
                ) {
                    "Balance Sheet"
                } else {
                    "Profit and Loss"
                }
                .to_string(),
            );
        }
    }

    pub fn validate_receivable_payable_account_type(&self, context: &mut AccountContext) {
        let receivable_payable_types = ["Receivable", "Payable"];
        if context.old_doc.as_ref().is_some_and(|old| {
            receivable_payable_types.contains(&old.account_type.as_deref().unwrap_or_default())
                && old.account_type != self.account_type
                && context.gle_exists
        }) {
            context.default_account_labels.insert(
                "__warning__".to_string(),
                "Account Type change has ledger entries".to_string(),
            );
        }
    }

    pub fn validate_root_company_and_sync_account_to_children(
        &self,
        context: &AccountContext,
    ) -> Result<(), AccountError> {
        if context.ignore_root_company_validation {
            return Ok(());
        }
        if let Some(root_company) = context.root_company.as_deref() {
            if context.allow_account_creation_against_child_company
                || context
                    .company
                    .as_ref()
                    .is_some_and(|company| company.allow_account_creation_against_child_company)
            {
                return Ok(());
            }
            if !context.root_company_has_account {
                return Err(AccountError::RootCompanyAccountMissing {
                    root_company: root_company.to_string(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_disabled(&self, context: &AccountContext) -> Result<(), AccountError> {
        if context
            .old_doc
            .as_ref()
            .is_some_and(|old| old.disabled != self.disabled)
            && self.disabled
        {
            self.validate_default_accounts_in_company(context, true)?;
        }
        Ok(())
    }

    pub fn validate_group_or_ledger(&self, context: &AccountContext) -> Result<(), AccountError> {
        let Some(old) = context.old_doc.as_ref() else {
            return Ok(());
        };
        if old.is_group == self.is_group {
            return Ok(());
        }
        if context.gle_exists {
            return Err(AccountError::CannotConvertTransactionAccountToLedger);
        }
        if self.is_group {
            if self.account_type.is_some() {
                return Err(AccountError::CannotConvertTypedAccountToGroup);
            }
            self.validate_default_accounts_in_company(context, false)?;
        } else if context.child_exists {
            return Err(AccountError::CannotConvertAccountWithChildrenToLedger);
        }
        Ok(())
    }

    pub fn validate_default_accounts_in_company(
        &self,
        context: &AccountContext,
        disabling: bool,
    ) -> Result<(), AccountError> {
        let labels = if context.default_account_labels.is_empty() {
            get_company_default_account_fields()
        } else {
            context.default_account_labels.clone()
        };
        for (field, value) in &context.default_accounts {
            if value == &self.name {
                let field_label = labels.get(field).cloned().unwrap_or_else(|| field.clone());
                if disabling {
                    return Err(AccountError::CannotDisableDefaultAccount {
                        account: self.name.clone(),
                        field_label,
                        company: self.company.clone(),
                    });
                }
                return Err(AccountError::CannotConvertDefaultAccountToGroup {
                    account: self.name.clone(),
                    field_label,
                    company: self.company.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_frozen_accounts_modifier(
        &self,
        context: &AccountContext,
    ) -> Result<(), AccountError> {
        if !context
            .old_doc
            .as_ref()
            .is_some_and(|old| old.freeze_account != self.freeze_account)
        {
            return Ok(());
        }
        let allowed_role = context
            .company
            .as_ref()
            .and_then(|company| company.role_allowed_for_frozen_entries.as_deref());
        if allowed_role
            .map(|role| context.user_roles.contains(role))
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(AccountError::UnauthorizedFrozenAccountModifier)
        }
    }

    pub fn validate_balance_must_be_debit_or_credit(
        &self,
        context: &AccountContext,
    ) -> Result<(), AccountError> {
        if self.name.is_empty() || self.balance_must_be.is_empty() {
            return Ok(());
        }
        if context.balance > 0.0 && self.balance_must_be == "Credit" {
            Err(AccountError::DebitBalanceCannotBeCredit)
        } else if context.balance < 0.0 && self.balance_must_be == "Debit" {
            Err(AccountError::CreditBalanceCannotBeDebit)
        } else {
            Ok(())
        }
    }

    pub fn validate_account_currency(
        &mut self,
        context: &AccountContext,
    ) -> Result<(), AccountError> {
        self.currency_explicitly_specified = true;
        if self.account_currency.is_none() {
            self.account_currency = context
                .company
                .as_ref()
                .map(|company| company.default_currency.clone());
            self.currency_explicitly_specified = false;
        }
        if context
            .gl_currency
            .as_ref()
            .is_some_and(|currency| Some(currency) != self.account_currency.as_ref())
            && context.any_gl_entry_exists
        {
            return Err(AccountError::CurrencyChangeAfterEntries);
        }
        Ok(())
    }

    pub fn validate_account_number(&self, context: &AccountContext) -> Result<(), AccountError> {
        if let (Some(account_number), Some(account)) = (
            self.account_number.as_ref(),
            context.same_number_account.as_ref(),
        ) {
            return Err(AccountError::DuplicateAccountNumber {
                account_number: account_number.clone(),
                account: account.clone(),
            });
        }
        Ok(())
    }

    pub fn validate_mandatory(&self) -> Result<(), AccountError> {
        if self.root_type.as_deref().unwrap_or_default().is_empty() {
            return Err(AccountError::RootTypeMandatory);
        }
        if self.report_type.as_deref().unwrap_or_default().is_empty() {
            return Err(AccountError::ReportTypeMandatory);
        }
        Ok(())
    }

    pub fn convert_group_to_ledger_plan(
        &self,
        child_exists: bool,
        gle_exists: bool,
    ) -> Result<ConversionPlan, AccountError> {
        if child_exists {
            Err(AccountError::CannotConvertAccountWithChildrenToLedger)
        } else if gle_exists {
            Err(AccountError::CannotConvertTransactionAccountToLedger)
        } else {
            Ok(ConversionPlan {
                is_group: false,
                save: true,
            })
        }
    }

    pub fn convert_ledger_to_group_plan(
        &self,
        gle_exists: bool,
        exclude_account_type_check: bool,
    ) -> Result<ConversionPlan, AccountError> {
        if gle_exists {
            Err(AccountError::CannotConvertTransactionAccountToLedger)
        } else if self.account_type.is_some() && !exclude_account_type_check {
            Err(AccountError::CannotConvertTypedAccountToGroup)
        } else {
            Ok(ConversionPlan {
                is_group: true,
                save: true,
            })
        }
    }

    pub fn on_trash_plan(&self, gle_exists: bool) -> Result<bool, AccountError> {
        if gle_exists {
            Err(AccountError::AccountWithExistingTransactionCannotBeDeleted)
        } else {
            Ok(true)
        }
    }

    pub fn create_account_for_child_company_plan(
        &self,
        parent_acc_name_map: &BTreeMap<String, String>,
        descendants: &[String],
        parent_acc_name: &str,
        child_parent_is_group: &BTreeMap<String, bool>,
        existing_child_accounts: &BTreeMap<String, String>,
        child_company_currency: &BTreeMap<String, String>,
    ) -> Result<Vec<ChildCompanyAccountPlan>, AccountError> {
        let mut plans = Vec::new();
        for company in descendants {
            let Some(parent_account) = parent_acc_name_map.get(company) else {
                return Err(AccountError::ChildCompanyParentMissing {
                    company: company.clone(),
                    parent: parent_acc_name.to_string(),
                });
            };
            if !child_parent_is_group
                .get(parent_account)
                .copied()
                .unwrap_or(true)
            {
                return Err(AccountError::ChildCompanyParentIsLedger {
                    company: company.clone(),
                    parent: parent_acc_name.to_string(),
                });
            }
            if let Some(child_account) = existing_child_accounts.get(company) {
                let mut updates = BTreeMap::new();
                if let Some(account_type) = self.account_type.as_ref() {
                    updates.insert("account_type".to_string(), account_type.clone());
                }
                updates.insert("freeze_account".to_string(), self.freeze_account.clone());
                updates.insert("balance_must_be".to_string(), self.balance_must_be.clone());
                plans.push(ChildCompanyAccountPlan::Update {
                    company: company.clone(),
                    account: child_account.clone(),
                    field_updates: updates,
                });
            } else {
                plans.push(ChildCompanyAccountPlan::Create {
                    company: company.clone(),
                    account_currency: if self.currency_explicitly_specified {
                        self.account_currency.clone().unwrap_or_default()
                    } else {
                        child_company_currency
                            .get(company)
                            .cloned()
                            .unwrap_or_default()
                    },
                    parent_account: parent_account.clone(),
                });
            }
        }
        Ok(plans)
    }
}

impl DocumentController for Account {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["on_update", "onload", "autoname", "validate", "on_trash"]
    }
}

pub fn get_parent_account(
    accounts: &[AccountRecord],
    company: &str,
    txt: &str,
    start: usize,
    page_len: usize,
) -> Vec<String> {
    let txt = txt.to_ascii_lowercase();
    accounts
        .iter()
        .filter(|account| account.is_group && account.docstatus != 2 && account.company == company)
        .filter(|account| account.name.to_ascii_lowercase().contains(&txt))
        .map(|account| account.name.clone())
        .skip(start)
        .take(page_len)
        .collect()
}

pub fn get_account_currency(
    account: Option<&str>,
    account_currency: Option<&str>,
    company_default_currency: &str,
) -> Option<String> {
    account?;
    Some(
        account_currency
            .filter(|currency| !currency.is_empty())
            .unwrap_or(company_default_currency)
            .to_string(),
    )
}

pub fn get_account_autoname(
    account_number: Option<&str>,
    account_name: &str,
    company: &CompanyInfo,
) -> Result<String, AccountError> {
    if company.name.is_empty() {
        return Err(AccountError::NewAccountMissing {
            account: company.name.clone(),
        });
    }
    let mut parts = vec![account_name.trim().to_string(), company.abbr.clone()];
    if let Some(account_number) = account_number
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        parts.insert(0, account_number.to_string());
    }
    Ok(parts.join(" - "))
}

pub fn update_account_number_plan(
    input: UpdateAccountNumberInput,
) -> Result<UpdateAccountNumberPlan, AccountError> {
    if let Some(existing) = input.same_number_account.as_ref() {
        return Err(AccountError::DuplicateAccountNumber {
            account_number: input.account_number.clone().unwrap_or_default(),
            account: existing.clone(),
        });
    }
    if !input.parent_company_accounts.is_empty()
        && !input.allow_independent_account_creation
        && !input.from_descendant
    {
        if let Some((ancestor, account)) = input.parent_company_accounts.iter().next() {
            return Err(AccountError::RenameNotAllowed {
                account: account.clone(),
                parent_company: ancestor.clone(),
            });
        }
    }
    let new_name = get_account_autoname(
        input.account_number.as_deref(),
        &input.account_name,
        &input.company,
    )?;
    let child_sync = if input.from_descendant {
        Vec::new()
    } else {
        sync_update_account_number_in_child_plan(
            &input.descendants,
            &input.old_account_name,
            &input.account_name,
            input.account_number.as_deref(),
            input.old_account_number.as_deref(),
        )
    };
    Ok(UpdateAccountNumberPlan {
        account_number_value: input.account_number.map(|value| value.trim().to_string()),
        account_name_value: input.account_name.trim().to_string(),
        new_name: (input.name != new_name).then_some(new_name),
        child_sync,
    })
}

pub fn sync_update_account_number_in_child_plan(
    descendants: &[String],
    old_acc_name: &str,
    account_name: &str,
    account_number: Option<&str>,
    old_acc_number: Option<&str>,
) -> Vec<RenamePlan> {
    descendants
        .iter()
        .map(|company| RenamePlan {
            company: company.clone(),
            old_account_name: old_acc_name.to_string(),
            account_name: account_name.to_string(),
            account_number: account_number.map(str::to_string),
            old_account_number: old_acc_number.map(str::to_string),
            from_descendant: true,
        })
        .collect()
}

pub fn merge_account_plan(input: MergeAccountInput) -> Result<MergeAccountPlan, AccountError> {
    if input.new.name.is_empty() {
        return Err(AccountError::NewAccountMissing {
            account: input.new.name,
        });
    }
    if (
        input.new.is_group,
        input.new.root_type.clone(),
        input.new.company.clone(),
        input.new.account_currency.clone().unwrap_or_default(),
    ) != (
        input.old.is_group,
        input.old.root_type.clone(),
        input.old.company.clone(),
        input.old.account_currency.clone().unwrap_or_default(),
    ) {
        return Err(AccountError::InvalidAccountMerge);
    }
    Ok(MergeAccountPlan {
        update_new_parent_account: (input.old.is_group
            && input.new.parent_account.as_deref() == Some(input.old.name.as_str()))
        .then(|| input.old.parent_account.clone())
        .flatten(),
        rename_old: input.old.name,
        merge_into: input.new.name,
    })
}

pub fn ensure_idle_system(
    in_test: bool,
    last_gl_update_age_seconds: Option<i64>,
    lock_busy: bool,
) -> Result<(), AccountError> {
    if in_test {
        return Ok(());
    }
    let effective_age = if lock_busy {
        Some(1)
    } else {
        last_gl_update_age_seconds
    };
    if effective_age.is_some_and(|age| age < 300) {
        Err(AccountError::SystemInUse)
    } else {
        Ok(())
    }
}

pub fn get_root_company(ancestors: &[String]) -> Vec<String> {
    ancestors.first().cloned().into_iter().collect()
}

pub fn get_company_default_account_fields() -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "default_bank_account".to_string(),
            "Default Bank Account".to_string(),
        ),
        (
            "default_cash_account".to_string(),
            "Default Cash Account".to_string(),
        ),
        (
            "default_receivable_account".to_string(),
            "Default Receivable Account".to_string(),
        ),
        (
            "default_payable_account".to_string(),
            "Default Payable Account".to_string(),
        ),
        (
            "default_expense_account".to_string(),
            "Default Expense Account".to_string(),
        ),
        (
            "default_income_account".to_string(),
            "Default Income Account".to_string(),
        ),
        (
            "stock_received_but_not_billed".to_string(),
            "Stock Received But Not Billed Account".to_string(),
        ),
        (
            "stock_adjustment_account".to_string(),
            "Stock Adjustment Account".to_string(),
        ),
        (
            "write_off_account".to_string(),
            "Write Off Account".to_string(),
        ),
        (
            "default_discount_account".to_string(),
            "Default Payment Discount Account".to_string(),
        ),
        (
            "unrealized_profit_loss_account".to_string(),
            "Unrealized Profit / Loss Account".to_string(),
        ),
        (
            "exchange_gain_loss_account".to_string(),
            "Exchange Gain / Loss Account".to_string(),
        ),
        (
            "unrealized_exchange_gain_loss_account".to_string(),
            "Unrealized Exchange Gain / Loss Account".to_string(),
        ),
        (
            "round_off_account".to_string(),
            "Round Off Account".to_string(),
        ),
        (
            "default_deferred_revenue_account".to_string(),
            "Default Deferred Revenue Account".to_string(),
        ),
        (
            "default_deferred_expense_account".to_string(),
            "Default Deferred Expense Account".to_string(),
        ),
        (
            "accumulated_depreciation_account".to_string(),
            "Accumulated Depreciation Account".to_string(),
        ),
        (
            "depreciation_expense_account".to_string(),
            "Depreciation Expense Account".to_string(),
        ),
        (
            "disposal_account".to_string(),
            "Gain/Loss Account on Asset Disposal".to_string(),
        ),
    ])
}

const PARENT_ONLY_ACCOUNT_TYPES: &[&str] = &[
    "Direct Income",
    "Indirect Income",
    "Current Asset",
    "Current Liability",
    "Direct Expense",
    "Indirect Expense",
];

const ACCOUNT_TYPE_OPTIONS: &str = "\nAccumulated Depreciation\nAsset Received But Not Billed\nBank\nCash\nChargeable\nCapital Work in Progress\nCost of Goods Sold\nCurrent Asset\nCurrent Liability\nDepreciation\nDirect Expense\nDirect Income\nEquity\nExpense Account\nExpenses Included In Asset Valuation\nExpenses Included In Valuation\nFixed Asset\nIncome Account\nIndirect Expense\nIndirect Income\nLiability\nPayable\nReceivable\nRound Off\nRound Off for Opening\nStock\nStock Adjustment\nStock Received But Not Billed\nService Received But Not Billed\nTax\nTemporary";
