use tokio_erp::erpnext::accounts::doctype::subscription_plan_detail::subscription_plan_detail::SubscriptionPlanDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn subscription_plan_detail_matches_erpnext_metadata() {
    assert_eq!(SubscriptionPlanDetail::DOCTYPE, "Subscription Plan Detail");
    assert_eq!(SubscriptionPlanDetail::MODULE, "Accounts");
    assert_eq!(SubscriptionPlanDetail::FIELD_ORDER, ["plan", "qty"]);
    assert!(SubscriptionPlanDetail::IS_TABLE);
    assert!(SubscriptionPlanDetail::EDITABLE_GRID);
    assert!(SubscriptionPlanDetail::QUICK_ENTRY);
    assert!(SubscriptionPlanDetail::TRACK_CHANGES);
    assert_eq!(
        SubscriptionPlanDetail::fields(),
        vec![
            FieldSpec::link("plan", "Plan")
                .options("Subscription Plan")
                .required()
                .in_list_view(),
            FieldSpec::int("qty", "Quantity").required().in_list_view(),
        ]
    );
}

#[test]
fn subscription_plan_detail_preserves_pass_controller_behavior() {
    let detail = SubscriptionPlanDetail::new("Basic", 3);

    assert_eq!(detail.plan.as_deref(), Some("Basic"));
    assert_eq!(detail.qty, 3);
    assert_eq!(detail.doctype(), "Subscription Plan Detail");
    assert_eq!(detail.module(), "Accounts");
    assert!(detail.custom_hooks().is_empty());
}
