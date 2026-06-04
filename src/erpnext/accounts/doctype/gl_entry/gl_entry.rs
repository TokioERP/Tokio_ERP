use std::collections::BTreeSet;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlEntry {
    pub name: Option<String>,
    pub account: Option<String>,
    pub account_currency: Option<String>,
    pub against: Option<String>,
    pub against_voucher: Option<String>,
    pub against_voucher_type: Option<String>,
    pub company: Option<String>,
    pub cost_center: Option<String>,
    pub credit: f64,
    pub credit_in_account_currency: f64,
    pub credit_in_reporting_currency: f64,
    pub debit: f64,
    pub debit_in_account_currency: f64,
    pub debit_in_reporting_currency: f64,
    pub due_date: Option<String>,
    pub finance_book: Option<String>,
    pub fiscal_year: Option<String>,
    pub is_advance: String,
    pub is_cancelled: bool,
    pub is_opening: String,
    pub party: Option<String>,
    pub party_type: Option<String>,
    pub posting_date: Option<String>,
    pub project: Option<String>,
    pub remarks: Option<String>,
    pub reporting_currency_exchange_rate: f64,
    pub to_rename: bool,
    pub transaction_currency: Option<String>,
    pub transaction_date: Option<String>,
    pub transaction_exchange_rate: f64,
    pub voucher_detail_no: Option<String>,
    pub voucher_no: Option<String>,
    pub voucher_subtype: Option<String>,
    pub voucher_type: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountDetails {
    pub is_group: bool,
    pub docstatus: i32,
    pub company: String,
    pub account_type: Option<String>,
    pub report_type: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostCenterDetails {
    pub is_group: bool,
    pub company: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlEntryContext {
    pub fiscal_year: Option<String>,
    pub company_default_currency: Option<String>,
    pub company_reporting_currency: Option<String>,
    pub reporting_currency_exchange_rate: Option<f64>,
    pub account_currency: Option<String>,
    pub account: Option<AccountDetails>,
    pub cost_center: Option<CostCenterDetails>,
    pub party_not_required: bool,
    pub from_repost: bool,
    pub journal_entry_voucher_type: Option<String>,
    pub dimensions: Vec<DimensionCheck>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DimensionCheck {
    pub company: String,
    pub fieldname: String,
    pub label: String,
    pub mandatory_for_pl: bool,
    pub mandatory_for_bs: bool,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutstandingInput {
    pub account: String,
    pub against_voucher_type: String,
    pub against_voucher: String,
    pub gl_balance: f64,
    pub journal_entry_unadjusted_amount: Option<f64>,
    pub on_cancel: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutstandingUpdate {
    pub doctype: String,
    pub name: String,
    pub outstanding_amount: f64,
    pub set_status_update: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlEntryLedgerRow {
    pub name: String,
    pub party: Option<String>,
    pub debit: String,
    pub credit: String,
    pub account: String,
    pub against: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgainstAccountUpdate {
    pub name: String,
    pub against: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenamePlan {
    pub doctype: &'static str,
    pub filters: Vec<(&'static str, &'static str)>,
    pub order_by: &'static str,
    pub limit: usize,
    pub batch_size: usize,
    pub autoname: &'static str,
    pub hooks: Vec<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlEntryError {
    InvalidAccountCurrency(String),
    ReportingCurrencyExchangeNotFound(String),
    Validation(String),
}

impl GlEntry {
    pub const DOCTYPE: &'static str = "GL Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "ACC-GLE-.YYYY.-.#####";
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const FIELD_ORDER: [&'static str; 47] = [
        "dates_section",
        "posting_date",
        "transaction_date",
        "column_break_avko",
        "fiscal_year",
        "due_date",
        "account_details_section",
        "account",
        "account_currency",
        "column_break_ifvf",
        "against",
        "party_type",
        "party",
        "transaction_details_section",
        "voucher_type",
        "voucher_no",
        "voucher_subtype",
        "transaction_currency",
        "column_break_dpsx",
        "against_voucher_type",
        "against_voucher",
        "voucher_detail_no",
        "transaction_exchange_rate",
        "reporting_currency_exchange_rate",
        "amounts_section",
        "debit_in_account_currency",
        "debit",
        "debit_in_transaction_currency",
        "debit_in_reporting_currency",
        "column_break_bm1w",
        "credit_in_account_currency",
        "credit",
        "credit_in_transaction_currency",
        "credit_in_reporting_currency",
        "dimensions_section",
        "cost_center",
        "column_break_lmnm",
        "project",
        "more_info_section",
        "finance_book",
        "company",
        "is_opening",
        "is_advance",
        "column_break_8abq",
        "to_rename",
        "is_cancelled",
        "remarks",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "posting_date" => FieldSpec::date("posting_date", "Posting Date")
                .in_filter()
                .in_list_view()
                .search_index(),
            "transaction_date" => {
                FieldSpec::date("transaction_date", "Transaction Date").in_list_view()
            }
            "fiscal_year" => FieldSpec::link("fiscal_year", "Fiscal Year")
                .options("Fiscal Year")
                .in_filter(),
            "due_date" => FieldSpec::date("due_date", "Due Date"),
            "account" => FieldSpec::link("account", "Account")
                .options("Account")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .search_index(),
            "account_currency" => {
                FieldSpec::link("account_currency", "Account Currency").options("Currency")
            }
            "against" => FieldSpec::text("against", "Against").in_filter(),
            "party_type" => FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .search_index(),
            "party" => FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .in_standard_filter()
                .search_index(),
            "voucher_type" => FieldSpec::link("voucher_type", "Voucher Type")
                .options("DocType")
                .in_filter(),
            "voucher_no" => FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type")
                .in_filter()
                .in_standard_filter()
                .search_index(),
            "voucher_subtype" => FieldSpec::small_text("voucher_subtype", "Voucher Subtype"),
            "transaction_currency" => {
                FieldSpec::link("transaction_currency", "Transaction Currency").options("Currency")
            }
            "against_voucher_type" => {
                FieldSpec::link("against_voucher_type", "Against Voucher Type").options("DocType")
            }
            "against_voucher" => FieldSpec::dynamic_link("against_voucher")
                .label("Against Voucher")
                .options("against_voucher_type")
                .in_filter()
                .search_index(),
            "voucher_detail_no" => FieldSpec::data("voucher_detail_no", "Voucher Detail No")
                .read_only()
                .search_index(),
            "transaction_exchange_rate" => {
                FieldSpec::float("transaction_exchange_rate", "Transaction Exchange Rate")
            }
            "reporting_currency_exchange_rate" => FieldSpec::float(
                "reporting_currency_exchange_rate",
                "Reporting Currency Exchange Rate",
            ),
            "debit_in_account_currency" => FieldSpec::currency(
                "debit_in_account_currency",
                "Debit Amount in Account Currency",
            )
            .options("account_currency"),
            "debit" => FieldSpec::currency("debit", "Debit Amount")
                .options("Company:company:default_currency"),
            "debit_in_transaction_currency" => FieldSpec::currency(
                "debit_in_transaction_currency",
                "Debit Amount in Transaction Currency",
            )
            .options("transaction_currency"),
            "debit_in_reporting_currency" => FieldSpec::currency(
                "debit_in_reporting_currency",
                "Debit Amount in Reporting Currency",
            ),
            "credit_in_account_currency" => FieldSpec::currency(
                "credit_in_account_currency",
                "Credit Amount in Account Currency",
            )
            .options("account_currency"),
            "credit" => FieldSpec::currency("credit", "Credit Amount")
                .options("Company:company:default_currency"),
            "credit_in_transaction_currency" => FieldSpec::currency(
                "credit_in_transaction_currency",
                "Credit Amount in Transaction Currency",
            )
            .options("transaction_currency"),
            "credit_in_reporting_currency" => FieldSpec::currency(
                "credit_in_reporting_currency",
                "Credit Amount in Reporting Currency",
            ),
            "cost_center" => FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .in_filter()
                .in_list_view()
                .search_index(),
            "project" => FieldSpec::link("project", "Project").options("Project"),
            "finance_book" => {
                FieldSpec::link("finance_book", "Finance Book").options("Finance Book")
            }
            "company" => FieldSpec::link("company", "Company")
                .options("Company")
                .in_filter(),
            "is_opening" => FieldSpec::select("is_opening", "Is Opening")
                .options("No\nYes")
                .default("No")
                .in_filter(),
            "is_advance" => FieldSpec::select("is_advance", "Is Advance")
                .options("No\nYes")
                .default("No"),
            "to_rename" => FieldSpec::check("to_rename", "To Rename")
                .default("1")
                .hidden(),
            "is_cancelled" => FieldSpec::check("is_cancelled", "Is Cancelled")
                .default("0")
                .in_filter(),
            "remarks" => FieldSpec::text("remarks", "Remarks").in_filter().no_copy(),
            "dates_section" => FieldSpec::section_break("dates_section").label("Dates"),
            "account_details_section" => {
                FieldSpec::section_break("account_details_section").label("Accounting Details")
            }
            "transaction_details_section" => {
                FieldSpec::section_break("transaction_details_section").label("Transaction Details")
            }
            "amounts_section" => FieldSpec::section_break("amounts_section").label("Amounts"),
            "dimensions_section" => {
                FieldSpec::section_break("dimensions_section").label("Dimensions")
            }
            "more_info_section" => FieldSpec::section_break("more_info_section").label("More Info"),
            "column_break_avko" | "column_break_ifvf" | "column_break_dpsx"
            | "column_break_bm1w" | "column_break_lmnm" | "column_break_8abq" => {
                FieldSpec::column_break(fieldname)
            }
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }

    pub fn autoname(&mut self, generated_hash: impl Into<String>, meta_autoname: &str) {
        self.name = Some(generated_hash.into());
        self.to_rename = meta_autoname != "hash";
    }

    pub fn validate_core(&mut self, context: &GlEntryContext) -> Result<(), GlEntryError> {
        self.validate_and_set_fiscal_year(context);
        self.pl_must_have_cost_center(context)?;

        if !context.from_repost && self.voucher_type.as_deref() != Some("Period Closing Voucher") {
            self.check_mandatory(context)?;
            self.validate_cost_center(context)?;
            self.check_pl_account(context)?;
            self.validate_currency(context)?;
        }

        self.set_amount_in_reporting_currency(context)
    }

    pub fn check_mandatory(&self, context: &GlEntryContext) -> Result<(), GlEntryError> {
        for (field, label) in [
            (&self.account, "Account"),
            (&self.voucher_type, "Voucher Type"),
            (&self.voucher_no, "Voucher No"),
            (&self.company, "Company"),
        ] {
            if !has_value(field) {
                return Err(GlEntryError::Validation(format!("{label} is required")));
            }
        }

        if !self.is_cancelled && !(has_value(&self.party_type) && has_value(&self.party)) {
            let account_type = context
                .account
                .as_ref()
                .and_then(|account| account.account_type.as_deref());

            if !context.party_not_required {
                if account_type == Some("Receivable") {
                    return Err(GlEntryError::Validation(format!(
                        "{} {}: Customer is required against Receivable account {}",
                        self.voucher_type(),
                        self.voucher_no(),
                        self.account()
                    )));
                }
                if account_type == Some("Payable") {
                    return Err(GlEntryError::Validation(format!(
                        "{} {}: Supplier is required against Payable account {}",
                        self.voucher_type(),
                        self.voucher_no(),
                        self.account()
                    )));
                }
            }
        }

        let exchange_gain_or_loss = self.voucher_type.as_deref() == Some("Journal Entry")
            && context.journal_entry_voucher_type.as_deref() == Some("Exchange Gain Or Loss");
        if round_to_precision(self.debit, 2) == 0.0
            && round_to_precision(self.credit, 2) == 0.0
            && !exchange_gain_or_loss
        {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Either debit or credit amount is required for {}",
                self.voucher_type(),
                self.voucher_no(),
                self.account()
            )));
        }

        Ok(())
    }

    pub fn pl_must_have_cost_center(&self, context: &GlEntryContext) -> Result<(), GlEntryError> {
        if has_value(&self.cost_center)
            || self.voucher_type.as_deref() == Some("Period Closing Voucher")
        {
            return Ok(());
        }

        if context
            .account
            .as_ref()
            .and_then(|account| account.report_type.as_deref())
            == Some("Profit and Loss")
        {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Cost Center is required for 'Profit and Loss' account {}. Please set the cost center field in {} or setup a default Cost Center for the Company.",
                self.voucher_type(),
                self.voucher_no(),
                self.account(),
                self.voucher_type()
            )));
        }

        Ok(())
    }

    pub fn validate_account_details(
        &self,
        context: &GlEntryContext,
        _adv_adj: bool,
    ) -> Result<(), GlEntryError> {
        let Some(account) = &context.account else {
            return Ok(());
        };

        if account.is_group {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Account {} is a Group Account and group accounts cannot be used in transactions",
                self.voucher_type(),
                self.voucher_no(),
                self.account()
            )));
        }

        if account.docstatus == 2 {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Account {} is inactive",
                self.voucher_type(),
                self.voucher_no(),
                self.account()
            )));
        }

        if account.company != self.company() {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Account {} does not belong to Company {}",
                self.voucher_type(),
                self.voucher_no(),
                self.account(),
                self.company()
            )));
        }

        Ok(())
    }

    pub fn validate_dimensions_for_pl_and_bs(
        &self,
        context: &GlEntryContext,
    ) -> Result<(), GlEntryError> {
        let account_type = context
            .account
            .as_ref()
            .and_then(|account| account.report_type.as_deref())
            .unwrap_or_default();

        for dimension in &context.dimensions {
            if account_type == "Profit and Loss"
                && self.company() == dimension.company
                && dimension.mandatory_for_pl
                && !self.is_cancelled
                && !dimension_has_value(dimension)
            {
                return Err(GlEntryError::Validation(format!(
                    "Accounting Dimension <b>{}</b> is required for 'Profit and Loss' account {}.",
                    dimension.label,
                    self.account()
                )));
            }

            if account_type == "Balance Sheet"
                && self.company() == dimension.company
                && dimension.mandatory_for_bs
                && !self.is_cancelled
                && !dimension_has_value(dimension)
            {
                return Err(GlEntryError::Validation(format!(
                    "Accounting Dimension <b>{}</b> is required for 'Balance Sheet' account {}.",
                    dimension.label,
                    self.account()
                )));
            }
        }

        Ok(())
    }

    pub fn validate_cost_center(&self, context: &GlEntryContext) -> Result<(), GlEntryError> {
        if !has_value(&self.cost_center) || self.is_cancelled {
            return Ok(());
        }

        let Some(cost_center) = &context.cost_center else {
            return Ok(());
        };

        if cost_center.company != self.company() {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Cost Center {} does not belong to Company {}",
                self.voucher_type(),
                self.voucher_no(),
                self.cost_center(),
                self.company()
            )));
        }

        if self.voucher_type.as_deref() != Some("Period Closing Voucher") && cost_center.is_group {
            return Err(GlEntryError::Validation(format!(
                "{} {}: Cost Center <b>{}</b> is a group cost center and group cost centers cannot be used in transactions",
                self.voucher_type(),
                self.voucher_no(),
                self.cost_center()
            )));
        }

        Ok(())
    }

    pub fn check_pl_account(&self, context: &GlEntryContext) -> Result<(), GlEntryError> {
        if self.is_opening == "Yes"
            && !self.is_cancelled
            && context
                .account
                .as_ref()
                .and_then(|account| account.report_type.as_deref())
                == Some("Profit and Loss")
        {
            return Err(GlEntryError::Validation(format!(
                "{} {}: 'Profit and Loss' type account {} not allowed in Opening Entry",
                self.voucher_type(),
                self.voucher_no(),
                self.account()
            )));
        }

        Ok(())
    }

    pub fn validate_currency(&mut self, context: &GlEntryContext) -> Result<(), GlEntryError> {
        if self.is_cancelled {
            return Ok(());
        }

        let company_currency = context
            .company_default_currency
            .as_deref()
            .unwrap_or_default();
        let account_currency = context.account_currency.as_deref();

        if !has_value(&self.account_currency) {
            self.account_currency = Some(account_currency.unwrap_or(company_currency).to_string());
        }

        if account_currency != self.account_currency.as_deref() {
            return Err(GlEntryError::InvalidAccountCurrency(format!(
                "{} {}: Accounting Entry for {} can only be made in currency: {}",
                self.voucher_type(),
                self.voucher_no(),
                self.account(),
                account_currency.unwrap_or(company_currency)
            )));
        }

        Ok(())
    }

    pub fn set_amount_in_reporting_currency(
        &mut self,
        context: &GlEntryContext,
    ) -> Result<(), GlEntryError> {
        let default_currency = context
            .company_default_currency
            .as_deref()
            .unwrap_or_default();
        let reporting_currency = context
            .company_reporting_currency
            .as_deref()
            .unwrap_or_default();
        let transaction_date = self
            .transaction_date
            .as_deref()
            .or(self.posting_date.as_deref())
            .unwrap_or_default();
        let Some(rate) = context.reporting_currency_exchange_rate else {
            return Err(GlEntryError::ReportingCurrencyExchangeNotFound(format!(
                "Unable to find exchange rate for {default_currency} to {reporting_currency} for key date {transaction_date}. Please create a Currency Exchange record manually."
            )));
        };

        self.reporting_currency_exchange_rate = rate;
        self.debit_in_reporting_currency = round_to_precision(self.debit * rate, 2);
        self.credit_in_reporting_currency = round_to_precision(self.credit * rate, 2);
        Ok(())
    }

    pub fn validate_and_set_fiscal_year(&mut self, context: &GlEntryContext) {
        if self.fiscal_year.is_none() {
            self.fiscal_year = context.fiscal_year.clone();
        }
    }

    pub fn on_cancel_error(&self) -> GlEntryError {
        GlEntryError::Validation(
            "Individual GL Entry cannot be cancelled.<br>Please cancel related transaction."
                .to_string(),
        )
    }

    pub fn doctype_update_indexes() -> Vec<Vec<&'static str>> {
        vec![
            vec!["voucher_type", "voucher_no"],
            vec!["posting_date", "company"],
            vec!["party_type", "party"],
        ]
    }

    pub fn rename_gle_sle_doctypes() -> [&'static str; 2] {
        ["GL Entry", "Stock Ledger Entry"]
    }

    pub fn rename_temporarily_named_docs_plan(
        doctype: &'static str,
        autoname: &'static str,
    ) -> RenamePlan {
        RenamePlan {
            doctype,
            filters: vec![("to_rename", "1")],
            order_by: "creation",
            limit: 50_000,
            batch_size: 100,
            autoname,
            hooks: vec!["on_gle_rename", "on_sle_rename"],
        }
    }

    fn account(&self) -> &str {
        self.account.as_deref().unwrap_or_default()
    }

    fn company(&self) -> &str {
        self.company.as_deref().unwrap_or_default()
    }

    fn cost_center(&self) -> &str {
        self.cost_center.as_deref().unwrap_or_default()
    }

    fn voucher_no(&self) -> &str {
        self.voucher_no.as_deref().unwrap_or_default()
    }

    fn voucher_type(&self) -> &str {
        self.voucher_type.as_deref().unwrap_or_default()
    }
}

impl DocumentController for GlEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

impl GlEntryLedgerRow {
    pub fn new(
        name: impl Into<String>,
        party: Option<&str>,
        debit: f64,
        credit: f64,
        account: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            party: party.map(str::to_string),
            debit: format_number(debit),
            credit: format_number(credit),
            account: account.into(),
            against: None,
        }
    }
}

impl AgainstAccountUpdate {
    pub fn new(name: impl Into<String>, against: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            against: against.into(),
        }
    }
}

pub fn validate_balance_type(
    account: &str,
    adv_adj: bool,
    balance_must_be: Option<&str>,
    balance: f64,
) -> Result<(), GlEntryError> {
    if adv_adj || account.is_empty() {
        return Ok(());
    }

    if (balance_must_be == Some("Debit") && round_to_precision(balance, 2) < 0.0)
        || (balance_must_be == Some("Credit") && round_to_precision(balance, 2) > 0.0)
    {
        return Err(GlEntryError::Validation(format!(
            "Balance for Account {account} must always be {}",
            balance_must_be.unwrap_or_default()
        )));
    }

    Ok(())
}

pub fn validate_frozen_account(
    account: &str,
    adv_adj: bool,
    freeze_account: Option<&str>,
    role_allowed_for_frozen_entries: Option<&str>,
    user_roles: &[String],
) -> Result<(), GlEntryError> {
    if freeze_account != Some("Yes") || adv_adj {
        return Ok(());
    }

    let Some(allowed_role) = role_allowed_for_frozen_entries else {
        return Err(GlEntryError::Validation(format!(
            "Account {account} is frozen"
        )));
    };

    if !user_roles.iter().any(|role| role == allowed_role) {
        return Err(GlEntryError::Validation(format!(
            "Not authorized to edit frozen Account {account}"
        )));
    }

    Ok(())
}

pub fn update_outstanding_amount(
    input: &OutstandingInput,
) -> Result<OutstandingUpdate, GlEntryError> {
    let mut balance = input.gl_balance;

    if input.against_voucher_type == "Purchase Invoice" {
        balance = -balance;
    } else if input.against_voucher_type == "Journal Entry" {
        let against_voucher_amount = input.journal_entry_unadjusted_amount.unwrap_or(0.0);
        if round_to_precision(against_voucher_amount, 2) == 0.0 {
            return Err(GlEntryError::Validation(format!(
                "Against Journal Entry {} is already adjusted against some other voucher",
                input.against_voucher
            )));
        }

        balance += against_voucher_amount;
        if against_voucher_amount < 0.0 {
            balance = -balance;
        }

        if balance < 0.0 && !input.on_cancel {
            return Err(GlEntryError::Validation(format!(
                "Outstanding for {} cannot be less than zero ({})",
                input.against_voucher,
                format_money(balance)
            )));
        }
    }

    Ok(OutstandingUpdate {
        doctype: input.against_voucher_type.clone(),
        name: input.against_voucher.clone(),
        outstanding_amount: round_to_precision(balance, 2),
        set_status_update: is_outstanding_doctype(&input.against_voucher_type),
    })
}

pub fn update_against_account(entries: &[GlEntryLedgerRow]) -> Vec<AgainstAccountUpdate> {
    if entries.is_empty() {
        return Vec::new();
    }

    let mut accounts_debited = BTreeSet::new();
    let mut accounts_credited = BTreeSet::new();

    for entry in entries {
        if parse_amount(&entry.debit) > 0.0 {
            accounts_debited.insert(entry.party.clone().unwrap_or_else(|| entry.account.clone()));
        }
        if parse_amount(&entry.credit) > 0.0 {
            accounts_credited.insert(entry.party.clone().unwrap_or_else(|| entry.account.clone()));
        }
    }

    let debit_against = join_set(&accounts_credited);
    let credit_against = join_set(&accounts_debited);

    entries
        .iter()
        .filter_map(|entry| {
            let new_against = if parse_amount(&entry.debit) > 0.0 {
                &debit_against
            } else if parse_amount(&entry.credit) > 0.0 {
                &credit_against
            } else {
                return None;
            };

            if entry.against.as_deref() == Some(new_against.as_str()) {
                None
            } else {
                Some(AgainstAccountUpdate::new(&entry.name, new_against))
            }
        })
        .collect()
}

fn has_value(value: &Option<String>) -> bool {
    value.as_deref().is_some_and(|value| !value.is_empty())
}

fn dimension_has_value(dimension: &DimensionCheck) -> bool {
    dimension
        .value
        .as_deref()
        .is_some_and(|value| !value.is_empty())
}

fn is_outstanding_doctype(doctype: &str) -> bool {
    matches!(
        doctype,
        "Sales Invoice" | "Purchase Invoice" | "Journal Entry" | "Fees"
    )
}

fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn parse_amount(value: &str) -> f64 {
    value.parse().unwrap_or(0.0)
}

fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

fn format_money(value: f64) -> String {
    format_number(value)
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
