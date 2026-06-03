use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::subscription::subscription::{
    get_prorata_factor_at, BillingCycle, BillingCycleDelta, GenerateInvoiceAt,
    GeneratedInvoiceState, InvoiceItemPlan, InvoicePaymentSchedulePlan, InvoicePlan,
    PlanRateSource, Subscription, SubscriptionError, SubscriptionPlanSnapshot, SubscriptionStatus,
};
use tokio_erp::erpnext::accounts::doctype::subscription_plan_detail::subscription_plan_detail::SubscriptionPlanDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn subscription_matches_erpnext_metadata() {
    assert_eq!(Subscription::DOCTYPE, "Subscription");
    assert_eq!(Subscription::MODULE, "Accounts");
    assert_eq!(Subscription::AUTONAME, "ACC-SUB-.YYYY.-.#####");
    assert_eq!(Subscription::NAMING_RULE, "Expression (old style)");
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
    assert_eq!(Subscription::ROW_FORMAT, "Dynamic");
    assert_eq!(Subscription::SORT_FIELD, "creation");
    assert_eq!(Subscription::SORT_ORDER, "DESC");
    assert!(Subscription::TRACK_CHANGES);

    let fields = Subscription::fields();
    assert_eq!(fields.len(), 33);
    assert!(fields.contains(
        &FieldSpec::link("party_type", "Party Type")
            .options("DocType")
            .required()
            .set_only_once()
    ));
    assert!(fields.contains(
        &FieldSpec::dynamic_link("party")
            .label("Party")
            .options("party_type")
            .required()
            .set_only_once()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::date("trial_period_start", "Trial Period Start Date")
            .allow_on_submit()
            .set_only_once()
    ));
    assert!(fields.contains(
        &FieldSpec::date("trial_period_end", "Trial Period End Date")
            .depends_on("eval:doc.trial_period_start")
            .set_only_once()
    ));
    assert!(fields.contains(
        &FieldSpec::check("follow_calendar_months", "Follow Calendar Months")
            .default("0")
            .description("If this is checked subsequent new invoices will be created on calendar  month and quarter start dates irrespective of current invoice start date")
            .set_only_once()
    ));
    assert!(fields.contains(
        &FieldSpec::date("end_date", "Subscription End Date").set_only_once()
    ));
    assert!(fields.contains(
        &FieldSpec::date("start_date", "Subscription Start Date").set_only_once()
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
fn subscription_current_invoice_generated_matches_posting_date_period_check() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));
    subscription.current_invoice = Some(GeneratedInvoiceState::with_posting_date(
        "2018-01-10",
        "Unpaid",
        "2018-01-15",
    ));

    assert!(subscription.is_current_invoice_generated(
        Some("2018-01-01"),
        Some("2018-01-31"),
        "2018-01-31"
    ));
    assert!(!subscription.is_current_invoice_generated(
        Some("2018-02-01"),
        Some("2018-02-28"),
        "2018-01-31"
    ));

    subscription.current_invoice = Some(GeneratedInvoiceState::with_posting_date(
        "2018-02-10",
        "Unpaid",
        "2018-02-01",
    ));
    assert!(subscription.is_current_invoice_generated(None, None, "2018-01-31"));
}

#[test]
fn subscription_validation_helpers_match_erpnext_errors() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.trial_period_start = Some("2018-01-10".to_string());
    subscription.trial_period_end = Some("2018-01-05".to_string());
    assert_eq!(
        subscription.validate_trial_period(),
        Err(SubscriptionError::TrialPeriodEndBeforeStart)
    );

    subscription.trial_period_end = None;
    assert_eq!(
        subscription.validate_trial_period(),
        Err(SubscriptionError::TrialPeriodIncomplete)
    );

    subscription.trial_period_start = Some("2018-01-02".to_string());
    subscription.trial_period_end = Some("2018-01-03".to_string());
    assert_eq!(
        subscription.validate_trial_period(),
        Err(SubscriptionError::TrialPeriodStartAfterSubscriptionStart)
    );

    let mut ending = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    ending.billing_cycle = Some(BillingCycle::new("Month", 1));
    ending.end_date = Some("2018-01-31".to_string());
    assert_eq!(
        ending.validate_end_date(),
        Err(SubscriptionError::EndDateNotAfterBillingCycle {
            minimum_end_date: "2018-01-31".to_string(),
        })
    );

    let mut calendar = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    calendar.follow_calendar_months = true;
    assert_eq!(
        calendar.validate_to_follow_calendar_months(),
        Err(SubscriptionError::CalendarMonthsRequireEndDate)
    );
    calendar.end_date = Some("2018-07-01".to_string());
    calendar.billing_cycle = Some(BillingCycle::new("Year", 1));
    assert_eq!(
        calendar.validate_to_follow_calendar_months(),
        Err(SubscriptionError::CalendarMonthsRequireMonthlyBilling)
    );
}

#[test]
fn subscription_validate_party_billing_currency_matches_erpnext_plan_currency_guard() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.company = Some("_Test Company".to_string());
    subscription.plans = vec![
        SubscriptionPlanDetail::new("USD-PLAN", 1),
        SubscriptionPlanDetail::new("INR-PLAN", 1),
    ];
    let snapshots = vec![
        SubscriptionPlanSnapshot {
            name: "USD-PLAN".to_string(),
            item: "Service A".to_string(),
            currency: "USD".to_string(),
            billing_interval: "Month".to_string(),
            billing_interval_count: 1,
            cost_center: None,
            rate_source: PlanRateSource::FixedRate { cost: 100.0 },
            enable_deferred_revenue: false,
            enable_deferred_expense: false,
            dimensions: BTreeMap::new(),
        },
        SubscriptionPlanSnapshot {
            name: "INR-PLAN".to_string(),
            item: "Service B".to_string(),
            currency: "INR".to_string(),
            billing_interval: "Month".to_string(),
            billing_interval_count: 1,
            cost_center: None,
            rate_source: PlanRateSource::FixedRate { cost: 100.0 },
            enable_deferred_revenue: false,
            enable_deferred_expense: false,
            dimensions: BTreeMap::new(),
        },
    ];

    assert_eq!(
        subscription.validate_party_billing_currency(&snapshots, Some("USD"), Some("EUR")),
        Err(SubscriptionError::UnsupportedPlanCurrencies {
            party_billing_currency: "USD".to_string(),
            plans: vec!["INR-PLAN".to_string()],
        })
    );

    assert!(subscription
        .validate_party_billing_currency(&snapshots[..1], None, Some("USD"))
        .is_ok());
}

#[test]
fn subscription_validate_runs_erpnext_sequence_defaults_status_and_cost_center() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.company = Some("_Test Company".to_string());
    subscription.plans = vec![SubscriptionPlanDetail::new("USD-PLAN", 1)];
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));
    let snapshots = vec![SubscriptionPlanSnapshot {
        name: "USD-PLAN".to_string(),
        item: "Service A".to_string(),
        currency: "USD".to_string(),
        billing_interval: "Month".to_string(),
        billing_interval_count: 1,
        cost_center: None,
        rate_source: PlanRateSource::FixedRate { cost: 100.0 },
        enable_deferred_revenue: false,
        enable_deferred_expense: false,
        dimensions: BTreeMap::new(),
    }];

    subscription
        .validate(
            &snapshots,
            "2018-01-01",
            Some("USD"),
            Some("EUR"),
            Some("Main - TC"),
            true,
        )
        .expect("valid subscription");

    assert_eq!(subscription.cost_center.as_deref(), Some("Main - TC"));
    assert_eq!(subscription.status, SubscriptionStatus::Active);
}

#[test]
fn subscription_invoice_due_and_status_rules_match_erpnext() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    assert!(!subscription.current_invoice_is_past_due("2018-01-10"));

    subscription.current_invoice = Some(GeneratedInvoiceState::new("2018-01-10", "Unpaid"));
    assert!(subscription.current_invoice_is_past_due("2018-01-10"));
    assert!(!subscription.is_past_grace_period("2018-01-12", 3));
    assert!(subscription.is_past_grace_period("2018-01-13", 3));
    assert_eq!(
        subscription.get_status_for_past_grace_period(false),
        SubscriptionStatus::Unpaid
    );
    assert_eq!(
        subscription.get_status_for_past_grace_period(true),
        SubscriptionStatus::Cancelled
    );

    subscription.current_invoice = Some(GeneratedInvoiceState::new("2018-01-10", "Paid"));
    assert!(!subscription.current_invoice_is_past_due("2018-01-10"));
}

#[test]
fn subscription_set_status_matches_erpnext_ordering() {
    let mut trial = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    trial.trial_period_end = Some("2018-01-31".to_string());
    trial.set_subscription_status("2018-01-10", false, 0, false);
    assert_eq!(trial.status, SubscriptionStatus::Trialing);

    let mut completed = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    completed.end_date = Some("2018-01-31".to_string());
    completed.set_subscription_status("2018-02-01", false, 0, false);
    assert_eq!(completed.status, SubscriptionStatus::Completed);

    let mut grace = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    grace.current_invoice = Some(GeneratedInvoiceState::new("2018-01-10", "Unpaid"));
    grace.set_subscription_status("2018-01-11", true, 10, false);
    assert_eq!(grace.status, SubscriptionStatus::GracePeriod);

    let mut unpaid = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    unpaid.current_invoice = Some(GeneratedInvoiceState::new("2018-01-10", "Unpaid"));
    unpaid.set_subscription_status("2018-01-10", true, 0, false);
    assert_eq!(unpaid.status, SubscriptionStatus::Unpaid);

    let mut cancelled = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    cancelled.current_invoice = Some(GeneratedInvoiceState::new("2018-01-10", "Unpaid"));
    cancelled.set_subscription_status("2018-01-10", true, 0, true);
    assert_eq!(cancelled.status, SubscriptionStatus::Cancelled);
    assert_eq!(cancelled.cancelation_date.as_deref(), Some("2018-01-10"));

    let mut active = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    active.set_subscription_status("2018-01-10", false, 0, false);
    assert_eq!(active.status, SubscriptionStatus::Active);
}

#[test]
fn subscription_cancel_and_restart_match_erpnext_lifecycle_rules() {
    let mut active = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    active.status = SubscriptionStatus::Active;
    active.current_invoice_start = Some("2018-01-01".to_string());
    active.current_invoice_end = Some("2018-01-31".to_string());
    active.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;

    let cancel_plan = active
        .cancel_subscription("2018-01-15")
        .expect("cancel active subscription");
    assert_eq!(active.status, SubscriptionStatus::Cancelled);
    assert_eq!(active.cancelation_date.as_deref(), Some("2018-01-15"));
    assert_eq!(
        cancel_plan,
        Some(("2018-01-01".to_string(), "2018-01-15".to_string()))
    );
    assert_eq!(
        active.cancel_subscription("2018-01-16"),
        Err(SubscriptionError::InvoiceCancelled)
    );

    active.billing_cycle = Some(BillingCycle::new("Month", 1));
    active
        .restart_subscription(Some("2018-02-01"), "2018-02-10")
        .expect("restart cancelled subscription");
    assert_eq!(active.status, SubscriptionStatus::Active);
    assert_eq!(active.cancelation_date, None);
    assert_eq!(active.current_invoice_start.as_deref(), Some("2018-02-01"));
    assert_eq!(active.current_invoice_end.as_deref(), Some("2018-02-28"));

    assert_eq!(
        active.restart_subscription(None, "2018-03-01"),
        Err(SubscriptionError::InvoiceNotCancelled)
    );

    let mut beginning = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    beginning.status = SubscriptionStatus::Active;
    beginning.current_invoice_start = Some("2018-01-01".to_string());
    beginning.generate_invoice_at = GenerateInvoiceAt::BeginningOfCurrentPeriod;
    assert_eq!(beginning.cancel_subscription("2018-01-15"), Ok(None));
}

#[test]
fn subscription_cancel_at_period_end_matches_erpnext_simple_cancel_marker() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.status = SubscriptionStatus::Active;

    subscription.cancel_subscription_at_period_end("2018-02-01");

    assert_eq!(subscription.status, SubscriptionStatus::Cancelled);
    assert_eq!(subscription.cancelation_date.as_deref(), Some("2018-02-01"));
}

#[test]
fn subscription_invoice_query_plans_match_erpnext_db_shapes() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.name = Some("SUB-0001".to_string());

    let current_invoice = subscription.current_invoice_query_plan();
    assert_eq!(current_invoice.doctype, "Sales Invoice");
    assert_eq!(
        current_invoice.filters,
        vec![
            (
                "subscription".to_string(),
                "=".to_string(),
                "SUB-0001".to_string()
            ),
            ("docstatus".to_string(), "<".to_string(), "2".to_string()),
        ]
    );
    assert_eq!(current_invoice.limit, Some(1));
    assert_eq!(current_invoice.order_by.as_deref(), Some("to_date desc"));
    assert_eq!(current_invoice.pluck.as_deref(), Some("name"));

    let invoices = subscription.invoices_query_plan();
    assert_eq!(invoices.doctype, "Sales Invoice");
    assert_eq!(
        invoices.filters,
        vec![(
            "subscription".to_string(),
            "=".to_string(),
            "SUB-0001".to_string()
        )]
    );
    assert_eq!(invoices.limit, None);
    assert_eq!(invoices.order_by.as_deref(), Some("from_date asc"));
    assert_eq!(invoices.pluck, None);

    let outstanding = subscription.outstanding_invoice_count_query_plan();
    assert_eq!(outstanding.doctype, "Sales Invoice");
    assert_eq!(
        outstanding.filters,
        vec![
            (
                "subscription".to_string(),
                "=".to_string(),
                "SUB-0001".to_string()
            ),
            ("docstatus".to_string(), "=".to_string(), "1".to_string()),
            ("status".to_string(), "!=".to_string(), "Paid".to_string()),
        ]
    );
}

#[test]
fn subscription_billing_cycle_query_and_data_match_erpnext_plan_distinct_logic() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.plans = vec![
        SubscriptionPlanDetail::new("MONTHLY-A", 1),
        SubscriptionPlanDetail::new("MONTHLY-B", 1),
        SubscriptionPlanDetail::new("YEARLY", 1),
    ];
    let snapshots = vec![
        SubscriptionPlanSnapshot {
            name: "MONTHLY-A".to_string(),
            item: "Service A".to_string(),
            currency: "USD".to_string(),
            billing_interval: "Month".to_string(),
            billing_interval_count: 1,
            cost_center: None,
            rate_source: PlanRateSource::FixedRate { cost: 100.0 },
            enable_deferred_revenue: false,
            enable_deferred_expense: false,
            dimensions: BTreeMap::new(),
        },
        SubscriptionPlanSnapshot {
            name: "MONTHLY-B".to_string(),
            item: "Service B".to_string(),
            currency: "USD".to_string(),
            billing_interval: "Month".to_string(),
            billing_interval_count: 1,
            cost_center: None,
            rate_source: PlanRateSource::FixedRate { cost: 200.0 },
            enable_deferred_revenue: false,
            enable_deferred_expense: false,
            dimensions: BTreeMap::new(),
        },
        SubscriptionPlanSnapshot {
            name: "YEARLY".to_string(),
            item: "Service C".to_string(),
            currency: "USD".to_string(),
            billing_interval: "Year".to_string(),
            billing_interval_count: 1,
            cost_center: None,
            rate_source: PlanRateSource::FixedRate { cost: 1200.0 },
            enable_deferred_revenue: false,
            enable_deferred_expense: false,
            dimensions: BTreeMap::new(),
        },
    ];

    assert_eq!(
        subscription.get_billing_cycle_and_interval(&snapshots),
        vec![BillingCycle::new("Month", 1), BillingCycle::new("Year", 1)]
    );
    assert_eq!(
        subscription.get_billing_cycle_data(&snapshots),
        Some(BillingCycleDelta::months_with_days(1, -1))
    );
}

#[test]
fn subscription_force_fetch_update_date_matches_erpnext_branches() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.current_invoice_start = Some("2018-01-10".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::BeginningOfCurrentPeriod;
    assert_eq!(
        subscription.force_fetch_subscription_updates("2018-01-09"),
        None
    );
    assert_eq!(
        subscription.force_fetch_subscription_updates("2018-01-10"),
        Some("2018-01-10".to_string())
    );

    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    assert_eq!(
        subscription.force_fetch_subscription_updates("2018-02-01"),
        Some("2018-01-31".to_string())
    );

    subscription.generate_invoice_at = GenerateInvoiceAt::DaysBeforeCurrentPeriod;
    subscription.number_of_days = 5;
    assert_eq!(
        subscription.force_fetch_subscription_updates("2018-02-01"),
        Some("2018-01-05".to_string())
    );
}

#[test]
fn subscription_process_generates_invoice_and_advances_period_like_erpnext() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.status = SubscriptionStatus::Active;
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));

    let plan = subscription.process_subscription("2018-01-31", false, 0, false);

    assert_eq!(
        plan.generated_invoice_posting_date.as_deref(),
        Some("2018-01-31")
    );
    assert_eq!(plan.updated_period_start.as_deref(), Some("2018-02-01"));
    assert_eq!(
        subscription.current_invoice_start.as_deref(),
        Some("2018-02-01")
    );
    assert_eq!(
        subscription.current_invoice_end.as_deref(),
        Some("2018-02-28")
    );
    assert_eq!(subscription.status, SubscriptionStatus::Active);
    assert!(!plan.cancelled);
    assert!(!plan.returned_early);
}

#[test]
fn subscription_process_cancels_at_period_end_without_advancing_past_end_date() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.status = SubscriptionStatus::Active;
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.end_date = Some("2018-01-31".to_string());
    subscription.cancel_at_period_end = true;
    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));

    let plan = subscription.process_subscription("2018-01-31", false, 0, false);

    assert_eq!(
        plan.generated_invoice_posting_date.as_deref(),
        Some("2018-01-31")
    );
    assert_eq!(plan.updated_period_start, None);
    assert!(plan.cancelled);
    assert!(plan.returned_early);
    assert_eq!(subscription.status, SubscriptionStatus::Cancelled);
    assert_eq!(subscription.cancelation_date.as_deref(), Some("2018-01-31"));
    assert_eq!(
        subscription.current_invoice_start.as_deref(),
        Some("2018-01-01")
    );
    assert_eq!(
        subscription.current_invoice_end.as_deref(),
        Some("2018-01-31")
    );
}

#[test]
fn subscription_process_returns_early_without_advancing_past_end_date_when_not_cancelled() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.status = SubscriptionStatus::Active;
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.end_date = Some("2018-01-31".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));

    let plan = subscription.process_subscription("2018-01-31", false, 0, false);

    assert_eq!(
        plan.generated_invoice_posting_date.as_deref(),
        Some("2018-01-31")
    );
    assert_eq!(plan.updated_period_start, None);
    assert!(!plan.cancelled);
    assert!(plan.returned_early);
    assert_eq!(subscription.status, SubscriptionStatus::Active);
    assert_eq!(subscription.cancelation_date, None);
    assert_eq!(
        subscription.current_invoice_start.as_deref(),
        Some("2018-01-01")
    );
    assert_eq!(
        subscription.current_invoice_end.as_deref(),
        Some("2018-01-31")
    );
}

#[test]
fn subscription_process_updates_period_when_posting_date_passes_current_end_without_invoice() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.status = SubscriptionStatus::Active;
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    subscription.billing_cycle = Some(BillingCycle::new("Month", 1));

    let plan = subscription.process_subscription("2018-02-05", false, 0, false);

    assert_eq!(plan.generated_invoice_posting_date, None);
    assert_eq!(plan.updated_period_start.as_deref(), Some("2018-02-05"));
    assert_eq!(
        subscription.current_invoice_start.as_deref(),
        Some("2018-02-05")
    );
    assert_eq!(
        subscription.current_invoice_end.as_deref(),
        Some("2018-03-04")
    );
    assert_eq!(subscription.status, SubscriptionStatus::Active);
    assert!(!plan.cancelled);
    assert!(!plan.returned_early);
}

#[test]
fn subscription_process_cancel_at_period_end_guard_runs_after_invoice_checks() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.status = SubscriptionStatus::Active;
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.end_date = Some("2018-02-28".to_string());
    subscription.cancel_at_period_end = true;
    subscription.generate_invoice_at = GenerateInvoiceAt::EndOfCurrentPeriod;
    subscription.current_invoice = Some(GeneratedInvoiceState::with_posting_date(
        "2018-02-10",
        "Unpaid",
        "2018-01-31",
    ));

    let plan = subscription.process_subscription("2018-01-31", true, 0, false);

    assert_eq!(plan.generated_invoice_posting_date, None);
    assert_eq!(plan.updated_period_start, None);
    assert!(plan.cancelled);
    assert!(!plan.returned_early);
    assert_eq!(subscription.status, SubscriptionStatus::Cancelled);
    assert_eq!(subscription.cancelation_date.as_deref(), Some("2018-01-31"));
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
fn subscription_get_items_from_plans_matches_erpnext_prorate_deferred_and_dimensions() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.party_type = Some("Customer".to_string());
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.plans = vec![SubscriptionPlanDetail::new("_Test Plan", 2)];

    let mut dimensions = BTreeMap::new();
    dimensions.insert("project".to_string(), "PROJ-001".to_string());

    let plan = SubscriptionPlanSnapshot {
        name: "_Test Plan".to_string(),
        item: "Service Item".to_string(),
        currency: "USD".to_string(),
        billing_interval: "Month".to_string(),
        billing_interval_count: 1,
        cost_center: Some("Main - TC".to_string()),
        rate_source: PlanRateSource::FixedRate { cost: 900.0 },
        enable_deferred_revenue: true,
        enable_deferred_expense: true,
        dimensions: dimensions.clone(),
    };

    assert_eq!(
        subscription.get_items_from_plans(&[plan], true, "2018-01-15"),
        vec![InvoiceItemPlan {
            item_code: "Service Item".to_string(),
            qty: 2,
            rate: 900.0 * (15.0 / 31.0),
            cost_center: Some("Main - TC".to_string()),
            enable_deferred_revenue: false,
            enable_deferred_expense: true,
            service_start_date: Some("2018-01-01".to_string()),
            service_end_date: Some("2018-01-31".to_string()),
            dimensions,
        }]
    );
}

#[test]
fn subscription_create_invoice_plan_matches_erpnext_invoice_fields() {
    let mut subscription = Subscription::new("Customer", "_Test Customer", "2018-01-01");
    subscription.name = Some("ACC-SUB-0001".to_string());
    subscription.company = Some("_Test Company".to_string());
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.days_until_due = 10;
    subscription.generate_invoice_at = GenerateInvoiceAt::BeginningOfCurrentPeriod;
    subscription.cost_center = Some("Main - TC".to_string());
    subscription.sales_tax_template = Some("_Test Sales Taxes".to_string());
    subscription.additional_discount_percentage = 10.0;
    subscription.plans = vec![SubscriptionPlanDetail::new("_Test Plan", 1)];

    let plan = SubscriptionPlanSnapshot {
        name: "_Test Plan".to_string(),
        item: "Service Item".to_string(),
        currency: "USD".to_string(),
        billing_interval: "Month".to_string(),
        billing_interval_count: 1,
        cost_center: Some("Main - TC".to_string()),
        rate_source: PlanRateSource::FixedRate { cost: 900.0 },
        enable_deferred_revenue: false,
        enable_deferred_expense: false,
        dimensions: BTreeMap::new(),
    };

    assert_eq!(
        subscription.create_invoice_plan(
            &[plan],
            false,
            "2018-01-15",
            Some("_Default Company"),
            false
        ),
        Ok(InvoicePlan {
            document_type: "Sales Invoice".to_string(),
            company: "_Test Company".to_string(),
            set_posting_time: true,
            posting_date: "2018-01-01".to_string(),
            cost_center: Some("Main - TC".to_string()),
            customer: Some("_Test Customer".to_string()),
            supplier: None,
            apply_tds: false,
            currency: "USD".to_string(),
            items: vec![InvoiceItemPlan {
                item_code: "Service Item".to_string(),
                qty: 1,
                rate: 900.0,
                cost_center: Some("Main - TC".to_string()),
                enable_deferred_revenue: false,
                enable_deferred_expense: false,
                service_start_date: None,
                service_end_date: None,
                dimensions: BTreeMap::new(),
            }],
            taxes_and_charges: Some("_Test Sales Taxes".to_string()),
            payment_schedule: vec![InvoicePaymentSchedulePlan {
                due_date: "2018-01-11".to_string(),
                invoice_portion: 100,
            }],
            additional_discount_percentage: 10.0,
            discount_amount: 0.0,
            apply_discount_on: Some("Grand Total".to_string()),
            subscription: Some("ACC-SUB-0001".to_string()),
            from_date: "2018-01-01".to_string(),
            to_date: "2018-01-31".to_string(),
            ignore_mandatory: true,
            submit: true,
        })
    );
}

#[test]
fn subscription_create_invoice_plan_matches_supplier_tds_and_trial_discount() {
    let mut subscription = Subscription::new("Supplier", "_Test Supplier", "2018-01-01");
    subscription.company = None;
    subscription.current_invoice_start = Some("2018-01-01".to_string());
    subscription.current_invoice_end = Some("2018-01-31".to_string());
    subscription.purchase_tax_template = Some("_Test Purchase Taxes".to_string());
    subscription.trial_period_end = Some("2018-01-31".to_string());
    subscription.plans = vec![SubscriptionPlanDetail::new("_Test Plan", 1)];

    let plan = SubscriptionPlanSnapshot {
        name: "_Test Plan".to_string(),
        item: "Service Item".to_string(),
        currency: "INR".to_string(),
        billing_interval: "Month".to_string(),
        billing_interval_count: 1,
        cost_center: None,
        rate_source: PlanRateSource::FixedRate { cost: 100.0 },
        enable_deferred_revenue: false,
        enable_deferred_expense: true,
        dimensions: BTreeMap::new(),
    };

    let invoice = subscription
        .create_invoice_plan(
            &[plan.clone()],
            false,
            "2018-01-10",
            Some("_Default Company"),
            true,
        )
        .expect("invoice plan");
    let invoice_without_tds = subscription
        .create_invoice_plan(&[plan], true, "2018-01-10", Some("_Default Company"), false)
        .expect("invoice plan without tds");

    assert_eq!(invoice.document_type, "Purchase Invoice");
    assert_eq!(invoice.company, "_Default Company");
    assert_eq!(invoice.supplier.as_deref(), Some("_Test Supplier"));
    assert_eq!(invoice.customer, None);
    assert!(invoice.apply_tds);
    assert!(!invoice_without_tds.apply_tds);
    assert_eq!(
        invoice.taxes_and_charges.as_deref(),
        Some("_Test Purchase Taxes")
    );
    assert_eq!(invoice.additional_discount_percentage, 100.0);
    assert_eq!(invoice.apply_discount_on, None);
    assert_eq!(invoice.items[0].enable_deferred_expense, true);
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
