use tokio_erp::erpnext::accounts::doctype::dashboard::{DashboardData, DashboardSection};
use tokio_erp::erpnext::accounts::doctype::subscription_plan::subscription_plan::{
    get_plan_rate, get_prorate_factor, BillingInterval, PriceDetermination, SubscriptionPlan,
    SubscriptionPlanRateInput, SubscriptionPlanValidationError,
};
use tokio_erp::erpnext::accounts::doctype::subscription_plan::subscription_plan_dashboard::get_data;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn subscription_plan_matches_erpnext_metadata() {
    assert_eq!(SubscriptionPlan::DOCTYPE, "Subscription Plan");
    assert_eq!(SubscriptionPlan::MODULE, "Accounts");
    assert_eq!(SubscriptionPlan::AUTONAME, Some("field:plan_name"));
    assert_eq!(SubscriptionPlan::NAMING_RULE, "By fieldname");
    assert_eq!(
        SubscriptionPlan::FIELD_ORDER,
        [
            "plan_name",
            "currency",
            "column_break_3",
            "item",
            "section_break_5",
            "price_determination",
            "column_break_7",
            "cost",
            "price_list",
            "section_break_11",
            "billing_interval",
            "column_break_13",
            "billing_interval_count",
            "payment_plan_section",
            "product_price_id",
            "column_break_16",
            "payment_gateway",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
        ]
    );
    assert!(SubscriptionPlan::ALLOW_RENAME);
    assert!(SubscriptionPlan::EDITABLE_GRID);
    assert_eq!(SubscriptionPlan::SORT_FIELD, "creation");
    assert_eq!(SubscriptionPlan::SORT_ORDER, "DESC");
    assert!(SubscriptionPlan::TRACK_CHANGES);
    assert_eq!(
        SubscriptionPlan::fields(),
        vec![
            FieldSpec::data("plan_name", "Plan Name")
                .required()
                .unique()
                .in_list_view(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .required(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("item", "Item")
                .options("Item")
                .required()
                .in_list_view(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::select("price_determination", "Subscription Price Based On")
                .options("\nFixed Rate\nBased On Price List\nMonthly Rate")
                .required(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::currency("cost", "Cost")
                .options("currency")
                .depends_on("eval:['Fixed Rate', 'Monthly Rate'].includes(doc.price_determination)")
                .in_list_view(),
            FieldSpec::link("price_list", "Price List")
                .options("Price List")
                .depends_on("eval:doc.price_determination==\"Based On Price List\""),
            FieldSpec::section_break("section_break_11"),
            FieldSpec::select("billing_interval", "Billing Interval")
                .options("Day\nWeek\nMonth\nYear")
                .default("Day")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::int("billing_interval_count", "Billing Interval Count")
                .default("1")
                .description("Number of intervals for the interval field e.g if Interval is 'Days' and Billing Interval Count is 3, invoices will be generated every 3 days")
                .required(),
            FieldSpec::section_break("payment_plan_section").label("Payment Plan"),
            FieldSpec::data("product_price_id", "Product Price ID"),
            FieldSpec::column_break("column_break_16"),
            FieldSpec::link("payment_gateway", "Payment Gateway")
                .options("Payment Gateway Account"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions")
                .collapsible(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::column_break("dimension_col_break"),
        ]
    );
}

#[test]
fn subscription_plan_dashboard_matches_erpnext_get_data() {
    assert_eq!(
        get_data(),
        DashboardData::new("subscription_plan")
            .non_standard_fieldnames(vec![("Payment Request", "plan"), ("Subscription", "plan"),])
            .transactions(vec![DashboardSection::labeled(
                "References",
                vec!["Payment Request", "Subscription"],
            )])
    );
}

#[test]
fn subscription_plan_validate_rejects_interval_count_less_than_one() {
    let plan = SubscriptionPlan {
        billing_interval_count: 0,
        ..SubscriptionPlan::default()
    };

    assert_eq!(
        plan.validate(),
        Err(SubscriptionPlanValidationError::BillingIntervalCountLessThanOne)
    );
}

#[test]
fn subscription_plan_fixed_rate_and_price_list_rates_match_erpnext_branching() {
    let fixed = SubscriptionPlan {
        cost: 100.0,
        price_determination: PriceDetermination::FixedRate,
        ..SubscriptionPlan::default()
    };
    assert_eq!(
        get_plan_rate(
            &fixed,
            SubscriptionPlanRateInput::default().prorate_factor(0.25)
        ),
        25.0
    );

    let price_list = SubscriptionPlan {
        price_determination: PriceDetermination::BasedOnPriceList,
        ..SubscriptionPlan::default()
    };
    assert_eq!(
        get_plan_rate(
            &price_list,
            SubscriptionPlanRateInput::default()
                .quantity(3.0)
                .price_list_rate(Some(42.0))
                .prorate_factor(0.5),
        ),
        21.0
    );
    assert_eq!(
        get_plan_rate(&price_list, SubscriptionPlanRateInput::default()),
        0.0
    );
}

#[test]
fn subscription_plan_monthly_rate_and_prorate_factor_match_erpnext_math() {
    let plan = SubscriptionPlan {
        cost: 310.0,
        price_determination: PriceDetermination::MonthlyRate,
        ..SubscriptionPlan::default()
    };

    let prorate_factor = 15.0 / 31.0 + 9.0 / 29.0;
    assert_close(
        get_prorate_factor("2024-01-16", "2024-02-20"),
        prorate_factor,
    );
    assert_close(
        get_plan_rate(
            &plan,
            SubscriptionPlanRateInput::default()
                .start_date("2024-01-16")
                .end_date("2024-02-20")
                .settings_prorate(true),
        ),
        620.0 - 310.0 * prorate_factor,
    );
    assert_eq!(
        get_plan_rate(
            &plan,
            SubscriptionPlanRateInput::default()
                .start_date("2024-01-16")
                .end_date("2024-02-20")
                .settings_prorate(false),
        ),
        620.0
    );
}

#[test]
fn subscription_plan_preserves_controller_behavior() {
    let plan = SubscriptionPlan::new("Basic", "USD", "Hosting");

    assert_eq!(plan.plan_name.as_deref(), Some("Basic"));
    assert_eq!(plan.currency.as_deref(), Some("USD"));
    assert_eq!(plan.item.as_deref(), Some("Hosting"));
    assert_eq!(plan.billing_interval, BillingInterval::Day);
    assert_eq!(plan.billing_interval_count, 1);
    assert_eq!(plan.doctype(), "Subscription Plan");
    assert_eq!(plan.module(), "Accounts");
    assert_eq!(plan.custom_hooks(), vec!["validate"]);
}

fn assert_close(left: f64, right: f64) {
    assert!(
        (left - right).abs() < f64::EPSILON,
        "left {left} did not match right {right}"
    );
}
