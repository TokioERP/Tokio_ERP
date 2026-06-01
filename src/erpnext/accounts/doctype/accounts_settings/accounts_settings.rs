use crate::erpnext::{DocumentController, FieldSpec};

pub const SELLING_DOCTYPES: [&str; 10] = [
    "Sales Invoice",
    "Sales Order",
    "Delivery Note",
    "Quotation",
    "Sales Invoice Item",
    "Sales Order Item",
    "Delivery Note Item",
    "Quotation Item",
    "POS Invoice",
    "POS Invoice Item",
];

pub const BUYING_DOCTYPES: [&str; 6] = [
    "Purchase Invoice",
    "Purchase Order",
    "Purchase Receipt",
    "Purchase Invoice Item",
    "Purchase Order Item",
    "Purchase Receipt Item",
];

#[derive(Clone, Debug, PartialEq)]
pub struct AccountsSettings {
    pub add_taxes_from_item_tax_template: bool,
    pub add_taxes_from_taxes_and_charges_template: bool,
    pub allow_stale: bool,
    pub stale_days: i32,
    pub enable_common_party_accounting: bool,
    pub show_payment_schedule_in_print: bool,
    pub enable_accounting_dimensions: bool,
    pub enable_discounts_and_margin: bool,
    pub enable_loyalty_point_program: bool,
    pub enable_subscription: bool,
    pub auto_reconciliation_job_trigger: i32,
    pub reconciliation_queue_size: i32,
    pub repost_allowed_types: Vec<String>,
}

impl Default for AccountsSettings {
    fn default() -> Self {
        Self {
            add_taxes_from_item_tax_template: true,
            add_taxes_from_taxes_and_charges_template: false,
            allow_stale: true,
            stale_days: 1,
            enable_common_party_accounting: false,
            show_payment_schedule_in_print: false,
            enable_accounting_dimensions: false,
            enable_discounts_and_margin: false,
            enable_loyalty_point_program: false,
            enable_subscription: true,
            auto_reconciliation_job_trigger: 15,
            reconciliation_queue_size: 5,
            repost_allowed_types: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountsSettingsError {
    AutoTaxSettingsConflict,
    StaleDaysShouldStartFromOne,
    CronIntervalOutOfRange,
    QueueSizeOutOfRange,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountsSettingsValidationPlan {
    pub set_defaults: Vec<(String, bool)>,
    pub property_setters: Vec<PropertySetterPlan>,
    pub clear_cache: bool,
    pub sync_auto_reconcile_config: Option<i32>,
    pub allow_on_submit_updates: Vec<(String, bool)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertySetterPlan {
    pub doctype: String,
    pub field_name: String,
    pub property: String,
    pub value: bool,
    pub property_type: String,
    pub validate_fields_for_doctype: bool,
}

impl PropertySetterPlan {
    pub fn new(doctype: &str, field_name: &str, property: &str, value: bool) -> Self {
        Self {
            doctype: doctype.to_string(),
            field_name: field_name.to_string(),
            property: property.to_string(),
            value,
            property_type: "Check".to_string(),
            validate_fields_for_doctype: false,
        }
    }
}

impl AccountsSettings {
    pub const DOCTYPE: &'static str = "Accounts Settings";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_SINGLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 102] = [
        "invoice_and_billing_tab",
        "enable_features_section",
        "unlink_payment_on_cancellation_of_invoice",
        "unlink_advance_payment_on_cancelation_of_order",
        "column_break_13",
        "delete_linked_ledger_entries",
        "enable_immutable_ledger",
        "invoicing_features_section",
        "check_supplier_invoice_uniqueness",
        "automatically_fetch_payment_terms",
        "enable_subscription",
        "column_break_17",
        "enable_common_party_accounting",
        "allow_multi_currency_invoices_against_single_party_account",
        "confirm_before_resetting_posting_date",
        "analytics_section",
        "enable_accounting_dimensions",
        "column_break_vtnr",
        "enable_discounts_and_margin",
        "journals_section",
        "merge_similar_account_heads",
        "deferred_accounting_settings_section",
        "book_deferred_entries_based_on",
        "column_break_18",
        "automatically_process_deferred_accounting_entry",
        "book_deferred_entries_via_journal_entry",
        "submit_journal_entries",
        "tax_settings_section",
        "determine_address_tax_category_from",
        "column_break_19",
        "add_taxes_from_item_tax_template",
        "add_taxes_from_taxes_and_charges_template",
        "book_tax_discount_loss",
        "round_row_wise_tax",
        "print_settings",
        "show_inclusive_tax_in_print",
        "show_taxes_as_table_in_print",
        "column_break_12",
        "show_payment_schedule_in_print",
        "item_price_settings_section",
        "maintain_same_internal_transaction_rate",
        "fetch_valuation_rate_for_internal_transaction",
        "column_break_feyo",
        "maintain_same_rate_action",
        "role_to_override_stop_action",
        "currency_exchange_section",
        "allow_stale",
        "allow_pegged_currencies_exchange_rates",
        "column_break_yuug",
        "stale_days",
        "payments_tab",
        "section_break_jpd0",
        "auto_reconcile_payments",
        "auto_reconciliation_job_trigger",
        "reconciliation_queue_size",
        "column_break_resa",
        "exchange_gain_loss_posting_date",
        "repost_section",
        "repost_allowed_types",
        "payment_options_section",
        "enable_loyalty_point_program",
        "column_break_ctam",
        "fetch_payment_schedule_in_payment_request",
        "invoicing_settings_tab",
        "accounts_transactions_settings_section",
        "over_billing_allowance",
        "column_break_11",
        "role_allowed_to_over_bill",
        "credit_controller",
        "make_payment_via_journal_entry",
        "assets_tab",
        "asset_settings_section",
        "calculate_depr_using_total_days",
        "column_break_gjcc",
        "book_asset_depreciation_entry_automatically",
        "role_to_notify_on_depreciation_failure",
        "closing_settings_tab",
        "period_closing_settings_section",
        "ignore_account_closing_balance",
        "use_legacy_controller_for_pcv",
        "column_break_25",
        "reports_tab",
        "remarks_section",
        "general_ledger_remarks_length",
        "column_break_lvjk",
        "receivable_payable_remarks_length",
        "accounts_receivable_payable_tuning_section",
        "receivable_payable_fetch_method",
        "default_ageing_range",
        "column_break_ntmi",
        "legacy_section",
        "ignore_is_opening_check_for_reporting",
        "tab_break_dpet",
        "chart_of_accounts_section",
        "show_balance_in_coa",
        "banking_section",
        "enable_party_matching",
        "enable_fuzzy_matching",
        "payment_request_section",
        "create_pr_in_draft_status",
        "budget_section",
        "use_legacy_budget_controller",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::tab_break("invoice_and_billing_tab", "Invoice and Billing"),
            FieldSpec::section_break("enable_features_section").label("Invoice Cancellation"),
            FieldSpec::check(
                "unlink_payment_on_cancellation_of_invoice",
                "Unlink Payment on Cancellation of Invoice",
            )
            .default("1"),
            FieldSpec::check(
                "unlink_advance_payment_on_cancelation_of_order",
                "Unlink Advance Payment on Cancellation of Order",
            )
            .default("1"),
            FieldSpec::section_break("tax_settings_section").label("Tax Settings"),
            FieldSpec::select(
                "determine_address_tax_category_from",
                "Determine Address Tax Category From",
            )
            .options("Billing Address\nShipping Address")
            .default("Billing Address")
            .description("Address used to determine Tax Category in transactions"),
            FieldSpec::check(
                "add_taxes_from_item_tax_template",
                "Automatically Add Taxes and Charges from Item Tax Template",
            )
            .default("1"),
            FieldSpec::check(
                "add_taxes_from_taxes_and_charges_template",
                "Automatically Add Taxes from Taxes and Charges Template",
            )
            .default("0")
            .description("If no taxes are set, and Taxes and Charges Template is selected, the system will automatically apply the taxes from the chosen template."),
            FieldSpec::section_break("print_settings").label("Print Settings"),
            FieldSpec::check("show_payment_schedule_in_print", "Show Payment Schedule in Print")
                .default("0"),
            FieldSpec::section_break("currency_exchange_section")
                .label("Currency Exchange Settings"),
            FieldSpec::check("allow_stale", "Allow Stale Exchange Rates")
                .default("1")
                .in_list_view(),
            FieldSpec::int("stale_days", "Stale Days")
                .default("1")
                .depends_on("eval:doc.allow_stale==0"),
            FieldSpec::section_break("section_break_jpd0")
                .label("Payment Reconciliation Settings"),
            FieldSpec::check("auto_reconcile_payments", "Auto Reconcile Payments").default("0"),
            FieldSpec::int("auto_reconciliation_job_trigger", "Auto Reconciliation Job Trigger")
                .default("15")
                .description("Interval should be between 1 to 59 MInutes"),
            FieldSpec::int("reconciliation_queue_size", "Reconciliation Queue Size")
                .default("5")
                .description("Documents Processed on each trigger. Queue Size should be between 5 and 100"),
            FieldSpec::table("repost_allowed_types", "Allowed Doctypes")
                .options("Repost Allowed Types"),
            FieldSpec::check("enable_accounting_dimensions", "Enable Accounting Dimensions")
                .default("0")
                .description("Enable cost center, projects and other custom accounting dimensions"),
            FieldSpec::check("enable_discounts_and_margin", "Enable Discounts and Margin")
                .default("0")
                .description("Apply discounts and margins on products"),
            FieldSpec::check("enable_loyalty_point_program", "Enable Loyalty Point Program")
                .default("0"),
            FieldSpec::check("enable_subscription", "Enable Subscription")
                .default("1")
                .description("Enable Subscription tracking in invoice"),
            FieldSpec::check("enable_common_party_accounting", "Enable Common Party Accounting")
                .default("0")
                .description("Learn about <a href=\"https://docs.frappe.io/erpnext/user/manual/en/common_party_accounting\" rel=\"noopener noreferrer\">Common Party</a>"),
        ]
    }

    #[allow(clippy::too_many_arguments)]
    pub fn validate(
        &self,
        old_doc: &Self,
        accounting_dimension_doctypes: &[&str],
        sales_discount_meta: &[(&str, bool, bool, bool)],
        subscription_doctypes: &[&str],
        child_docs: &[&str],
        accounting_dimensions: &[&str],
        dimension_fields: &[(&str, &str, bool)],
    ) -> Result<AccountsSettingsValidationPlan, AccountsSettingsError> {
        self.validate_auto_tax_settings()?;
        let mut plan = AccountsSettingsValidationPlan::default();

        if old_doc.add_taxes_from_item_tax_template != self.add_taxes_from_item_tax_template {
            plan.set_defaults.push((
                "add_taxes_from_item_tax_template".to_string(),
                self.add_taxes_from_item_tax_template,
            ));
            plan.clear_cache = true;
        }

        if old_doc.enable_common_party_accounting != self.enable_common_party_accounting {
            plan.set_defaults.push((
                "enable_common_party_accounting".to_string(),
                self.enable_common_party_accounting,
            ));
            plan.clear_cache = true;
        }

        self.validate_stale_days()?;

        if old_doc.show_payment_schedule_in_print != self.show_payment_schedule_in_print {
            plan.property_setters
                .extend(self.enable_payment_schedule_in_print());
        }

        if old_doc.enable_accounting_dimensions != self.enable_accounting_dimensions {
            plan.property_setters
                .extend(toggle_accounting_dimension_sections(
                    accounting_dimension_doctypes,
                    !self.enable_accounting_dimensions,
                ));
            plan.clear_cache = true;
        }

        if old_doc.enable_discounts_and_margin != self.enable_discounts_and_margin {
            plan.property_setters.extend(toggle_sales_discount_section(
                sales_discount_meta,
                !self.enable_discounts_and_margin,
            ));
            plan.clear_cache = true;
        }

        if old_doc.enable_loyalty_point_program != self.enable_loyalty_point_program {
            plan.property_setters
                .extend(toggle_loyalty_point_program_section(
                    sales_discount_meta,
                    !self.enable_loyalty_point_program,
                ));
            plan.clear_cache = true;
        }

        if old_doc.enable_subscription != self.enable_subscription {
            plan.property_setters.extend(toggle_subscription_sections(
                subscription_doctypes,
                !self.enable_subscription,
            ));
            plan.clear_cache = true;
        }

        plan.sync_auto_reconcile_config = self.validate_and_sync_auto_reconcile_config(old_doc)?;
        plan.allow_on_submit_updates = self.update_property_for_accounting_dimension(
            child_docs,
            accounting_dimensions,
            dimension_fields,
        );

        Ok(plan)
    }

    pub fn validate_stale_days(&self) -> Result<(), AccountsSettingsError> {
        if !self.allow_stale && self.stale_days <= 0 {
            Err(AccountsSettingsError::StaleDaysShouldStartFromOne)
        } else {
            Ok(())
        }
    }

    pub fn enable_payment_schedule_in_print(&self) -> Vec<PropertySetterPlan> {
        let show_in_print = self.show_payment_schedule_in_print;
        let mut plans = Vec::new();
        for doctype in [
            "Sales Order",
            "Sales Invoice",
            "Purchase Order",
            "Purchase Invoice",
        ] {
            plans.push(PropertySetterPlan::new(
                doctype,
                "due_date",
                "print_hide",
                show_in_print,
            ));
            plans.push(PropertySetterPlan::new(
                doctype,
                "payment_schedule",
                "print_hide",
                !show_in_print,
            ));
        }
        plans
    }

    pub fn validate_and_sync_auto_reconcile_config(
        &self,
        old_doc: &Self,
    ) -> Result<Option<i32>, AccountsSettingsError> {
        if old_doc.auto_reconciliation_job_trigger != self.auto_reconciliation_job_trigger {
            if self.auto_reconciliation_job_trigger > 0 && self.auto_reconciliation_job_trigger < 60
            {
                return Ok(Some(self.auto_reconciliation_job_trigger));
            }
            return Err(AccountsSettingsError::CronIntervalOutOfRange);
        }

        if old_doc.reconciliation_queue_size != self.reconciliation_queue_size
            && (self.reconciliation_queue_size < 5 || self.reconciliation_queue_size > 100)
        {
            return Err(AccountsSettingsError::QueueSizeOutOfRange);
        }

        Ok(None)
    }

    pub fn validate_auto_tax_settings(&self) -> Result<(), AccountsSettingsError> {
        if self.add_taxes_from_item_tax_template && self.add_taxes_from_taxes_and_charges_template {
            Err(AccountsSettingsError::AutoTaxSettingsConflict)
        } else {
            Ok(())
        }
    }

    pub fn update_property_for_accounting_dimension(
        &self,
        child_docs: &[&str],
        accounting_dimensions: &[&str],
        dimension_fields: &[(&str, &str, bool)],
    ) -> Vec<(String, bool)> {
        if self.repost_allowed_types.is_empty() {
            return Vec::new();
        }

        let mut doctypes = self
            .repost_allowed_types
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        doctypes.extend(child_docs.iter().copied());
        set_allow_on_submit_for_dimension_fields(&doctypes, accounting_dimensions, dimension_fields)
    }
}

impl DocumentController for AccountsSettings {
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

pub fn toggle_accounting_dimension_sections(
    accounting_dimension_doctypes: &[&str],
    hide: bool,
) -> Vec<PropertySetterPlan> {
    accounting_dimension_doctypes
        .iter()
        .map(|doctype| {
            create_property_setter_for_hiding_field(doctype, "accounting_dimensions_section", hide)
        })
        .collect()
}

pub fn toggle_sales_discount_section(
    meta: &[(&str, bool, bool, bool)],
    hide: bool,
) -> Vec<PropertySetterPlan> {
    let mut plans = Vec::new();
    for doctype in SELLING_DOCTYPES.into_iter().chain(BUYING_DOCTYPES) {
        if let Some((_, has_additional_discount, has_discount_and_margin, _)) = meta
            .iter()
            .find(|(meta_doctype, _, _, _)| *meta_doctype == doctype)
        {
            if *has_additional_discount {
                plans.push(create_property_setter_for_hiding_field(
                    doctype,
                    "additional_discount_section",
                    hide,
                ));
            }
            if *has_discount_and_margin {
                plans.push(create_property_setter_for_hiding_field(
                    doctype,
                    "discount_and_margin",
                    hide,
                ));
            }
        }
    }
    plans
}

pub fn toggle_loyalty_point_program_section(
    meta: &[(&str, bool, bool, bool)],
    hide: bool,
) -> Vec<PropertySetterPlan> {
    let mut plans = Vec::new();
    for doctype in SELLING_DOCTYPES {
        if meta
            .iter()
            .any(|(meta_doctype, _, _, has_loyalty)| *meta_doctype == doctype && *has_loyalty)
        {
            plans.push(create_property_setter_for_hiding_field(
                doctype,
                "loyalty_points_redemption",
                hide,
            ));
        }
    }
    plans
}

pub fn toggle_subscription_sections(
    subscription_doctypes: &[&str],
    hide: bool,
) -> Vec<PropertySetterPlan> {
    subscription_doctypes
        .iter()
        .map(|doctype| {
            create_property_setter_for_hiding_field(doctype, "subscription_section", hide)
        })
        .collect()
}

pub fn create_property_setter_for_hiding_field(
    doctype: &str,
    field_name: &str,
    hide: bool,
) -> PropertySetterPlan {
    PropertySetterPlan::new(doctype, field_name, "hidden", hide)
}

pub fn set_allow_on_submit_for_dimension_fields(
    doctypes: &[&str],
    accounting_dimensions: &[&str],
    dimension_fields: &[(&str, &str, bool)],
) -> Vec<(String, bool)> {
    let mut updates = Vec::new();
    for doctype in doctypes {
        for dimension in accounting_dimensions {
            if dimension_fields
                .iter()
                .any(|(field_doctype, fieldname, allow_on_submit)| {
                    field_doctype == doctype && fieldname == dimension && !allow_on_submit
                })
            {
                updates.push((format!("{doctype}-{dimension}"), true));
            }
        }
    }
    updates
}
