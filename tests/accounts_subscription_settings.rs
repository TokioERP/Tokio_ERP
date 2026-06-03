use tokio_erp::erpnext::accounts::doctype::subscription_settings::subscription_settings::SubscriptionSettings;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn subscription_settings_matches_erpnext_metadata() {
    assert_eq!(SubscriptionSettings::DOCTYPE, "Subscription Settings");
    assert_eq!(SubscriptionSettings::MODULE, "Accounts");
    assert_eq!(
        SubscriptionSettings::FIELD_ORDER,
        ["grace_period", "cancel_after_grace", "prorate"]
    );
    assert!(SubscriptionSettings::EDITABLE_GRID);
    assert_eq!(SubscriptionSettings::GRID_PAGE_LENGTH, 50);
    assert!(!SubscriptionSettings::HIDE_TOOLBAR);
    assert!(SubscriptionSettings::IS_SINGLE);
    assert!(SubscriptionSettings::QUICK_ENTRY);
    assert_eq!(SubscriptionSettings::ROW_FORMAT, "Dynamic");
    assert_eq!(SubscriptionSettings::SORT_FIELD, "creation");
    assert_eq!(SubscriptionSettings::SORT_ORDER, "DESC");
    assert!(SubscriptionSettings::TRACK_CHANGES);
    assert_eq!(
        SubscriptionSettings::fields(),
        vec![
            FieldSpec::int("grace_period", "Grace Period")
                .default("1")
                .description("Number of days after invoice date has elapsed before canceling subscription or marking subscription as unpaid"),
            FieldSpec::check(
                "cancel_after_grace",
                "Cancel Subscription After Grace Period",
            )
            .default("0"),
            FieldSpec::check("prorate", "Prorate").default("1"),
        ]
    );
}

#[test]
fn subscription_settings_preserves_pass_controller_behavior() {
    let settings = SubscriptionSettings::new(7, true, false);

    assert_eq!(settings.grace_period, 7);
    assert!(settings.cancel_after_grace);
    assert!(!settings.prorate);
    assert_eq!(settings.doctype(), "Subscription Settings");
    assert_eq!(settings.module(), "Accounts");
    assert!(settings.custom_hooks().is_empty());
}
