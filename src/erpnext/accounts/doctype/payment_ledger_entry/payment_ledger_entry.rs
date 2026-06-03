use std::collections::HashMap;

use crate::erpnext::accounts::doctype::accounting_dimension_filter::accounting_dimension_filter::DimensionFilterInfo;
use crate::erpnext::{DocumentController, FieldSpec};

const OUTSTANDING_DOCTYPES: [&str; 3] = ["Sales Invoice", "Purchase Invoice", "Fees"];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PaymentLedgerEntry {
    pub posting_date: Option<String>,
    pub company: Option<String>,
    pub account_type: String,
    pub account: Option<String>,
    pub party_type: Option<String>,
    pub party: Option<String>,
    pub due_date: Option<String>,
    pub voucher_detail_no: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
    pub finance_book: Option<String>,
    pub voucher_type: Option<String>,
    pub voucher_no: Option<String>,
    pub against_voucher_type: Option<String>,
    pub against_voucher_no: Option<String>,
    pub amount: f64,
    pub account_currency: Option<String>,
    pub amount_in_account_currency: f64,
    pub delinked: bool,
    pub remarks: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountSnapshot {
    pub account_type: String,
    pub company: String,
    pub is_group: bool,
    pub docstatus: i32,
    pub report_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingDimensionCheck {
    pub fieldname: String,
    pub label: String,
    pub company: String,
    pub mandatory_for_pl: bool,
    pub mandatory_for_bs: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentLedgerEntryError {
    AccountWrongCompany {
        account: String,
        company: String,
    },
    AccountWrongType {
        account: String,
        account_type: String,
    },
    GroupAccount {
        voucher_type: String,
        voucher_no: String,
        account: String,
    },
    InactiveAccount {
        voucher_type: String,
        voucher_no: String,
        account: String,
    },
    AccountDoesNotBelongToCompany {
        voucher_type: String,
        voucher_no: String,
        account: String,
        company: String,
    },
    MandatoryDimension {
        dimension: String,
        account: String,
    },
    InvalidDimension {
        dimension: String,
        value: String,
        account: String,
    },
    MandatoryPlBsDimension {
        label: String,
        account: String,
        report_type: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentLedgerEntryFlags {
    pub adv_adj: bool,
    pub from_repost: bool,
    pub update_outstanding: String,
    pub is_reverse_depr_entry: bool,
}

impl Default for PaymentLedgerEntryFlags {
    fn default() -> Self {
        Self {
            adv_adj: false,
            from_repost: false,
            update_outstanding: "No".to_string(),
            is_reverse_depr_entry: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentLedgerOnUpdatePlan {
    pub validate_frozen_account: bool,
    pub validate_account_details: bool,
    pub validate_dimensions: bool,
    pub validate_balance_type: bool,
    pub update_outstanding: Option<(String, String, String, String, String)>,
    pub adv_adj: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PaymentLedgerExpectedRow {
    pub voucher_type: String,
    pub voucher_no: String,
    pub against_voucher_type: String,
    pub against_voucher_no: String,
    pub amount: f64,
    pub delinked: bool,
}

impl PaymentLedgerExpectedRow {
    pub fn new(
        voucher_type: impl Into<String>,
        voucher_no: impl Into<String>,
        against_voucher_type: impl Into<String>,
        against_voucher_no: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            voucher_type: voucher_type.into(),
            voucher_no: voucher_no.into(),
            against_voucher_type: against_voucher_type.into(),
            against_voucher_no: against_voucher_no.into(),
            amount,
            delinked: false,
        }
    }
}

impl PaymentLedgerEntry {
    pub const DOCTYPE: &'static str = "Payment Ledger Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 19] = [
        "posting_date",
        "company",
        "account_type",
        "account",
        "party_type",
        "party",
        "due_date",
        "voucher_detail_no",
        "cost_center",
        "finance_book",
        "voucher_type",
        "voucher_no",
        "against_voucher_type",
        "against_voucher_no",
        "amount",
        "account_currency",
        "amount_in_account_currency",
        "delinked",
        "remarks",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const IN_CREATE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const SEARCH_FIELDS: &'static str = "voucher_no, against_voucher_no";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("posting_date", "Posting Date").search_index(),
            FieldSpec::select("account_type", "Account Type").options("Receivable\nPayable"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .search_index(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .search_index(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .search_index(),
            FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .in_standard_filter()
                .search_index(),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            FieldSpec::link("against_voucher_type", "Against Voucher Type")
                .options("DocType")
                .in_standard_filter()
                .search_index(),
            FieldSpec::dynamic_link("against_voucher_no")
                .label("Against Voucher No")
                .options("against_voucher_type")
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            FieldSpec::currency("amount", "Amount")
                .options("Company:company:default_currency")
                .in_list_view(),
            FieldSpec::link("account_currency", "Currency").options("Currency"),
            FieldSpec::currency("amount_in_account_currency", "Amount in Account Currency")
                .options("account_currency"),
            FieldSpec::check("delinked", "DeLinked")
                .default("0")
                .in_list_view(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .search_index(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::link("project", "Project").options("Project"),
            FieldSpec::date("due_date", "Due Date"),
            FieldSpec::link("finance_book", "Finance Book").options("Finance Book"),
            FieldSpec::text("remarks", "Remarks"),
            FieldSpec::data("voucher_detail_no", "Voucher Detail No").search_index(),
        ]
    }

    pub fn validate_account(
        &self,
        account: &AccountSnapshot,
    ) -> Result<(), PaymentLedgerEntryError> {
        let account_name = self.account.clone().unwrap_or_default();
        if Some(account.company.as_str()) != self.company.as_deref() {
            return Err(PaymentLedgerEntryError::AccountWrongCompany {
                account: account_name,
                company: self.company.clone().unwrap_or_default(),
            });
        }
        if account.account_type != self.account_type {
            return Err(PaymentLedgerEntryError::AccountWrongType {
                account: account_name,
                account_type: self.account_type.clone(),
            });
        }
        Ok(())
    }

    pub fn validate_account_details(
        &self,
        account: &AccountSnapshot,
    ) -> Result<(), PaymentLedgerEntryError> {
        let voucher_type = self.voucher_type.clone().unwrap_or_default();
        let voucher_no = self.voucher_no.clone().unwrap_or_default();
        let account_name = self.account.clone().unwrap_or_default();
        if account.is_group {
            return Err(PaymentLedgerEntryError::GroupAccount {
                voucher_type,
                voucher_no,
                account: account_name,
            });
        }
        if account.docstatus == 2 {
            return Err(PaymentLedgerEntryError::InactiveAccount {
                voucher_type,
                voucher_no,
                account: account_name,
            });
        }
        if Some(account.company.as_str()) != self.company.as_deref() {
            return Err(PaymentLedgerEntryError::AccountDoesNotBelongToCompany {
                voucher_type,
                voucher_no,
                account: account_name,
                company: self.company.clone().unwrap_or_default(),
            });
        }
        Ok(())
    }

    pub fn validate_allowed_dimensions(
        &self,
        dimension_filter_map: &HashMap<(String, String), DimensionFilterInfo>,
    ) -> Result<(), PaymentLedgerEntryError> {
        let account = self.account.as_deref().unwrap_or("");
        for ((dimension, filter_account), value) in dimension_filter_map {
            if account != filter_account {
                continue;
            }
            let dimension_value = self.dimension_value(dimension);
            if value.is_mandatory && dimension_value.is_none() {
                return Err(PaymentLedgerEntryError::MandatoryDimension {
                    dimension: dimension.clone(),
                    account: account.to_string(),
                });
            }
            if value.allow_or_restrict == "Allow" {
                if let Some(dimension_value) = dimension_value {
                    if !value
                        .allowed_dimensions
                        .contains(&dimension_value.to_string())
                    {
                        return Err(PaymentLedgerEntryError::InvalidDimension {
                            dimension: dimension.clone(),
                            value: dimension_value.to_string(),
                            account: account.to_string(),
                        });
                    }
                }
            } else if let Some(dimension_value) = dimension_value {
                if value
                    .allowed_dimensions
                    .contains(&dimension_value.to_string())
                {
                    return Err(PaymentLedgerEntryError::InvalidDimension {
                        dimension: dimension.clone(),
                        value: dimension_value.to_string(),
                        account: account.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn validate_dimensions_for_pl_and_bs(
        &self,
        account_report_type: &str,
        dimensions: &[AccountingDimensionCheck],
    ) -> Result<(), PaymentLedgerEntryError> {
        for dimension in dimensions {
            let mandatory = (account_report_type == "Profit and Loss"
                && self.company.as_deref() == Some(dimension.company.as_str())
                && dimension.mandatory_for_pl)
                || (account_report_type == "Balance Sheet"
                    && self.company.as_deref() == Some(dimension.company.as_str())
                    && dimension.mandatory_for_bs);
            if mandatory && self.dimension_value(&dimension.fieldname).is_none() {
                return Err(PaymentLedgerEntryError::MandatoryPlBsDimension {
                    label: dimension.label.clone(),
                    account: self.account.clone().unwrap_or_default(),
                    report_type: account_report_type.to_string(),
                });
            }
        }
        Ok(())
    }

    pub fn on_update_plan(&self, flags: PaymentLedgerEntryFlags) -> PaymentLedgerOnUpdatePlan {
        let should_validate = !flags.from_repost;
        let update_outstanding = if self
            .against_voucher_type
            .as_deref()
            .is_some_and(|doctype| OUTSTANDING_DOCTYPES.contains(&doctype))
            && flags.update_outstanding == "Yes"
            && !flags.is_reverse_depr_entry
        {
            Some((
                self.against_voucher_type.clone().unwrap_or_default(),
                self.against_voucher_no.clone().unwrap_or_default(),
                self.account.clone().unwrap_or_default(),
                self.party_type.clone().unwrap_or_default(),
                self.party.clone().unwrap_or_default(),
            ))
        } else {
            None
        };

        PaymentLedgerOnUpdatePlan {
            validate_frozen_account: should_validate,
            validate_account_details: should_validate && !self.delinked,
            validate_dimensions: should_validate && !self.delinked,
            validate_balance_type: should_validate && !self.delinked,
            update_outstanding,
            adv_adj: flags.adv_adj,
        }
    }

    fn dimension_value(&self, dimension: &str) -> Option<&str> {
        match dimension {
            "cost_center" => self.cost_center.as_deref(),
            "project" => self.project.as_deref(),
            _ => None,
        }
        .filter(|value| !value.is_empty())
    }
}

pub fn expected_invoice_payment_rows(
    invoice_doctype: &str,
    invoice_name: &str,
    payment_doctype: &str,
    payment_name: &str,
    invoice_amount: f64,
    allocated_amount: f64,
) -> Vec<PaymentLedgerExpectedRow> {
    vec![
        PaymentLedgerExpectedRow::new(
            invoice_doctype,
            invoice_name,
            invoice_doctype,
            invoice_name,
            invoice_amount,
        ),
        PaymentLedgerExpectedRow::new(
            payment_doctype,
            payment_name,
            invoice_doctype,
            invoice_name,
            -allocated_amount,
        ),
    ]
}

pub fn payment_ledger_entry_doctype_update_indices() -> Vec<Vec<&'static str>> {
    vec![
        vec!["against_voucher_no", "against_voucher_type"],
        vec!["voucher_no", "voucher_type"],
    ]
}

impl DocumentController for PaymentLedgerEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_update"]
    }
}
