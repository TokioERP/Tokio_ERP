use tokio_erp::erpnext::accounts::doctype::subscription::subscription::{
    get_prorata_factor_at, BillingCycle, BillingCycleDelta, GenerateInvoiceAt, Subscription,
    SubscriptionError, SubscriptionStatus,
};
use tokio_erp::erpnext::accounts::doctype::subscription_plan_detail::subscription_plan_detail::SubscriptionPlanDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn subscription_matches_erpnext_metadata() {
    assert_eq!(Subscription::DOCTYPE, "Subscription");
    assert_eq!(Subscription::MODULE, "Accounts");
    assert_eq!(Subscription::AUTONAME, "ACC-SUB-.YYYY.-.#####");
    assert_eq!(
        Subscription::FIELD_ORDER,
        [
            "party_type",
            "party",
            "cb_1",
            "company",
            "status",
            "subscription_period",
            "start_date",
            "end_date",
            "cancelation_date",
            "trial_period_start",
            "trial_period_end",
            "follow_calendar_months",
            "generate_new_invoices_past_due_date",
            "submit_invoice",
            "column_break_11",
            "current_invoice_start",
            "current_invoice_end",
            "days_until_due",
            "generate_invoice_at",
            "number_of_days",
            "cancel_at_period_end",
            "sb_4",
            "plans",
            "sb_1",
            "sales_tax_template",
            "purchase_tax_template",
            "sb_2",
            "apply_additional_discount",
            "cb_2",
            "additional_discount_percentage",
            "additional_discount_amount",
            "accounting_dimensions_section",
            "cost_center",
        ]
    );
    assert!(Subscription::EDITABLE_GRID);
    assert!(Subscription::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(Subscription::SORT_FIELD, "creation");
    assert_eq!(Subscription::SORT_ORDER, "DESC");
    assert!(Subscription::TRACK_CHANGES);

    let fields = Subscription::fields();
    assert_eq!(fields.len(), 33);
    assert!(fields.contains(
        &FieldSpec::link("party_type", "Party Type")
            .options("DocType")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::dynamic_link("party")
            .label("Party")
            .options("party_type")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::select("generate_invoice_at", "Generate Invoice At")
            .options("End of the current subscription period\nBeginning of the current subscription period\nDays before the current subscription period")
            .default("End of the current subscription period")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::int("number_of_days", "Number of Days")
            .depends_on(
                "eval:doc.generate_invoice_at === \"Days before the current subscription period\""
            )
            .mandatory_depends_on(
                "eval:doc.generate_invoice_at === \"Days before the current subscription period\""
            )
    ));
}

#[test]
fn subscription_billing_cycle_data_matches_erpnext_mapping() {
    assert_eq!(
        Subscription::billing_cycle_data_for(BillingCycle::new("Day", 3)),
        BillingCycleDelta::days(2)
    );
    assert_eq!(
        Subscription::billing_cycle_data_for(BillingCycle::new("Week", 2)),
        BillingCycleDelta::days(13)
    );
    assert_eq!(
        Subscription::billing_cycle_data_for(BillingCycle::new("Month", 1)),
        BillingCycleDelta::months_with_days(1, -1)
    );
    assert_eq!(
        Subscription::billing_cycle_data_for(BillingCycle::new("Year", 1)),
        BillingCycleDelta::years_with_days(1, -1)
    );
}

#[test]
fn subscription_validates_single_billing_cycle_like_erpnext() {
    assert_eq!(
        Subscription::validate_plans_billing_cycle(&[
            BillingCycle::new("Month", 1),
            BillingCycle::new("Year", 1),
        ]),
        Err(SubscriptionError::MixedBillingCycles)
    );
    assert!(Subscription::validate_plans_billing_cycle(&[BillingCycle::new("Month", 1)]).is_ok());
}

#[test]
fn subscription_period_helpers_match_trial_and_calendar_month_rules() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.trial_period_start = Some("2018-01-01".to_string());
    subscription.trial_period_end = Some("2018-01-31".to_string());
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));
    subscription.update_subscription_period(Some("2018-01-01"), "2018-01-05");
    assert_eq!(
        subscription.current_invoice_start.as_deref(),
        Some("2018-02-01")
    );
    assert_eq!(
        subscription.current_invoice_end.as_deref(),
        Some("2018-02-28")
    );

    let mut calendar = Subscription::new("Supplier", "_Test Supplier", "2018-01-15");
    calendar.follow_calendar_months = true;
    calendar.end_date = Some("2018-07-15".to_string());
    calendar.billing_cycle = Some(BillingCycle::new("Month", 3));
    calendar.update_subscription_period(Some("2018-01-15"), "2018-01-01");
    assert_eq!(calendar.current_invoice_end.as_deref(), Some("2018-03-31"));
}

#[test]
fn subscription_status_and_invoice_generation_gates_match_erpnext() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    assert!(subscription.can_generate_new_invoice("2018-01-31", false));
    assert!(!subscription.can_generate_new_invoice("2018-01-30", false));

    subscription.generate_invoice_at = GenerateInvoiceAt::BeginningOfCurrentPeriod;
    assert!(subscription.can_generate_new_invoice("2018-01-01", false));

    subscription.generate_invoice_at = GenerateInvoiceAt::DaysBeforeCurrentPeriod;
    subscription.number_of_days = 10;
    assert!(subscription.can_generate_new_invoice("2017-12-22", false));

    subscription.cancelation_date = Some("2018-01-20".to_string());
    assert!(!subscription.can_generate_new_invoice("2017-12-22", false));
}

#[test]
fn subscription_prorata_factor_matches_erpnext_formula() {
    assert_eq!(
        get_prorata_factor_at("2018-01-31", "2018-01-01", Some(1), "2018-01-15"),
        1.0
    );
    assert_close(
        get_prorata_factor_at("2018-01-31", "2018-01-01", Some(0), "2018-01-15"),
        15.0 / 31.0,
    );
}

#[test]
fn subscription_preserves_controller_hooks_and_defaults() {
    let subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");

    assert_eq!(subscription.party_type.as_deref(), Some("Customer"));
    assert_eq!(subscription.party.as_deref(), Some("_Test Customer"));
    assert_eq!(subscription.status, SubscriptionStatus::default());
    assert_eq!(
        subscription.generate_invoice_at,
        GenerateInvoiceAt::EndOfCurrentPeriod
    );
    assert!(subscription.submit_invoice);
    assert_eq!(subscription.plans, Vec::<SubscriptionPlanDetail>::new());
    assert_eq!(subscription.doctype(), "Subscription");
    assert_eq!(subscription.module(), "Accounts");
    assert_eq!(
        subscription.custom_hooks(),
        vec![
            "before_insert",
            "validate",
            "process",
            "cancel_subscription",
            "restart_subscription",
            "force_fetch_subscription_updates",
        ]
    );
}

fn assert_close(left: f64, right: f64) {
    assert!(
        (left - right).abs() < f64::EPSILON,
        "left {left} did not match right {right}"
    );
}
