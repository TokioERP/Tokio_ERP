use tokio_erp::erpnext::accounts::doctype::accounts_settings::accounts_settings::{
    toggle_accounting_dimension_sections, toggle_loyalty_point_program_section,
    toggle_sales_discount_section, toggle_subscription_sections, AccountsSettings,
    AccountsSettingsError, AccountsSettingsValidationPlan, PropertySetterPlan, BUYING_DOCTYPES,
    SELLING_DOCTYPES,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn accounts_settings_matches_erpnext_metadata_shape() {
    assert_eq!(AccountsSettings::DOCTYPE, "Accounts Settings");
    assert_eq!(AccountsSettings::MODULE, "Accounts");
    assert!(AccountsSettings::IS_SINGLE);
    assert_eq!(AccountsSettings::FIELD_ORDER.len(), 102);
    assert_eq!(
        &AccountsSettings::FIELD_ORDER[..10],
        [
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
        ]
    );
    assert_eq!(
        &AccountsSettings::FIELD_ORDER[AccountsSettings::FIELD_ORDER.len() - 6..],
        [
            "enable_party_matching",
            "enable_fuzzy_matching",
            "payment_request_section",
            "create_pr_in_draft_status",
            "budget_section",
            "use_legacy_budget_controller",
        ]
    );

    let fields = AccountsSettings::fields();
    assert!(fields.contains(
        &FieldSpec::select(
            "determine_address_tax_category_from",
            "Determine Address Tax Category From"
        )
        .options("Billing Address\nShipping Address")
        .default("Billing Address")
        .description("Address used to determine Tax Category in transactions")
    ));
    assert!(fields.contains(
        &FieldSpec::check(
            "add_taxes_from_item_tax_template",
            "Automatically Add Taxes and Charges from Item Tax Template",
        )
        .default("1")
    ));
    assert!(fields.contains(
        &FieldSpec::int(
            "auto_reconciliation_job_trigger",
            "Auto Reconciliation Job Trigger"
        )
        .default("15")
        .description("Interval should be between 1 to 59 MInutes")
    ));
    assert!(fields.contains(
        &FieldSpec::table("repost_allowed_types", "Allowed Doctypes")
            .options("Repost Allowed Types")
    ));
}

#[test]
fn accounts_settings_validate_rejects_conflicting_tax_and_stale_days() {
    let old = AccountsSettings::default();
    let settings = AccountsSettings {
        add_taxes_from_item_tax_template: true,
        add_taxes_from_taxes_and_charges_template: true,
        ..Default::default()
    };
    assert_eq!(
        settings.validate(&old, &[], &[], &[], &[], &[], &[]),
        Err(AccountsSettingsError::AutoTaxSettingsConflict)
    );

    let stale = AccountsSettings {
        allow_stale: false,
        stale_days: 0,
        ..Default::default()
    };
    assert_eq!(
        stale.validate(&old, &[], &[], &[], &[], &[], &[]),
        Err(AccountsSettingsError::StaleDaysShouldStartFromOne)
    );
}

#[test]
fn accounts_settings_validate_sync_ranges_match_erpnext() {
    let old = AccountsSettings {
        auto_reconciliation_job_trigger: 15,
        reconciliation_queue_size: 5,
        ..Default::default()
    };
    let invalid_cron = AccountsSettings {
        auto_reconciliation_job_trigger: 60,
        reconciliation_queue_size: 5,
        ..Default::default()
    };
    assert_eq!(
        invalid_cron.validate(&old, &[], &[], &[], &[], &[], &[]),
        Err(AccountsSettingsError::CronIntervalOutOfRange)
    );

    let invalid_queue = AccountsSettings {
        auto_reconciliation_job_trigger: 15,
        reconciliation_queue_size: 4,
        ..Default::default()
    };
    assert_eq!(
        invalid_queue.validate(&old, &[], &[], &[], &[], &[], &[]),
        Err(AccountsSettingsError::QueueSizeOutOfRange)
    );
}

#[test]
fn accounts_settings_validate_builds_side_effect_plan_like_erpnext() {
    let old = AccountsSettings {
        add_taxes_from_item_tax_template: false,
        enable_common_party_accounting: false,
        show_payment_schedule_in_print: false,
        enable_accounting_dimensions: false,
        enable_discounts_and_margin: false,
        enable_loyalty_point_program: false,
        enable_subscription: false,
        auto_reconciliation_job_trigger: 15,
        reconciliation_queue_size: 5,
        repost_allowed_types: vec!["Sales Invoice".to_string()],
        ..Default::default()
    };
    let settings = AccountsSettings {
        add_taxes_from_item_tax_template: true,
        enable_common_party_accounting: true,
        show_payment_schedule_in_print: true,
        enable_accounting_dimensions: true,
        enable_discounts_and_margin: true,
        enable_loyalty_point_program: true,
        enable_subscription: true,
        auto_reconciliation_job_trigger: 30,
        reconciliation_queue_size: 10,
        repost_allowed_types: vec!["Sales Invoice".to_string()],
        ..Default::default()
    };

    let plan = settings
        .validate(
            &old,
            &["Sales Invoice", "Purchase Invoice"],
            &[
                ("Sales Invoice", true, true, true),
                ("Purchase Invoice", true, true, false),
            ],
            &["Sales Invoice", "Subscription"],
            &["Sales Invoice Item"],
            &["cost_center", "project"],
            &[
                ("Sales Invoice", "cost_center", false),
                ("Sales Invoice Item", "project", false),
            ],
        )
        .expect("valid settings");

    assert!(plan.clear_cache);
    assert_eq!(
        plan.set_defaults,
        vec![
            ("add_taxes_from_item_tax_template".to_string(), true),
            ("enable_common_party_accounting".to_string(), true),
        ]
    );
    assert_eq!(plan.sync_auto_reconcile_config, Some(30));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Sales Order",
        "due_date",
        "print_hide",
        true,
    )));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Sales Invoice",
        "payment_schedule",
        "print_hide",
        false,
    )));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Sales Invoice",
        "accounting_dimensions_section",
        "hidden",
        false,
    )));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Sales Invoice",
        "additional_discount_section",
        "hidden",
        false,
    )));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Sales Invoice",
        "discount_and_margin",
        "hidden",
        false,
    )));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Sales Invoice",
        "loyalty_points_redemption",
        "hidden",
        false,
    )));
    assert!(plan.property_setters.contains(&PropertySetterPlan::new(
        "Subscription",
        "subscription_section",
        "hidden",
        false,
    )));
    assert_eq!(
        plan.allow_on_submit_updates,
        vec![
            ("Sales Invoice-cost_center".to_string(), true),
            ("Sales Invoice Item-project".to_string(), true),
        ]
    );
}

#[test]
fn accounts_settings_toggle_helpers_match_erpnext_doctype_sets() {
    assert_eq!(
        SELLING_DOCTYPES[..4],
        ["Sales Invoice", "Sales Order", "Delivery Note", "Quotation"]
    );
    assert_eq!(
        BUYING_DOCTYPES[..3],
        ["Purchase Invoice", "Purchase Order", "Purchase Receipt"]
    );
    assert_eq!(
        toggle_accounting_dimension_sections(&["Sales Invoice"], true),
        vec![PropertySetterPlan::new(
            "Sales Invoice",
            "accounting_dimensions_section",
            "hidden",
            true,
        )]
    );
    assert_eq!(
        toggle_sales_discount_section(&[("Sales Invoice", true, false, false)], true),
        vec![PropertySetterPlan::new(
            "Sales Invoice",
            "additional_discount_section",
            "hidden",
            true,
        )]
    );
    assert_eq!(
        toggle_loyalty_point_program_section(&[("Sales Invoice", false, false, true)], true),
        vec![PropertySetterPlan::new(
            "Sales Invoice",
            "loyalty_points_redemption",
            "hidden",
            true,
        )]
    );
    assert_eq!(
        toggle_subscription_sections(&["Sales Invoice"], true),
        vec![PropertySetterPlan::new(
            "Sales Invoice",
            "subscription_section",
            "hidden",
            true,
        )]
    );
}

#[test]
fn accounts_settings_preserves_controller_hook() {
    let doc = AccountsSettings::default();

    assert_eq!(doc.doctype(), "Accounts Settings");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), ["validate"]);
    assert_eq!(
        AccountsSettingsValidationPlan::default().property_setters,
        Vec::new()
    );
}
