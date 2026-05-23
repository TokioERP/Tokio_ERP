use tokio_erp::erpnext::accounts::doctype::process_subscription::process_subscription::{
    create_subscription_process, ProcessSubscription, SubscriptionJob,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_subscription_matches_erpnext_metadata() {
    assert_eq!(ProcessSubscription::DOCTYPE, "Process Subscription");
    assert_eq!(ProcessSubscription::MODULE, "Accounts");
    assert_eq!(
        ProcessSubscription::FIELD_ORDER,
        ["posting_date", "subscription", "amended_from"]
    );
    assert!(ProcessSubscription::ALLOW_RENAME);
    assert!(ProcessSubscription::EDITABLE_GRID);
    assert!(ProcessSubscription::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ProcessSubscription::IS_SUBMITTABLE);
    assert_eq!(
        ProcessSubscription::fields(),
        vec![
            FieldSpec::date("posting_date", "Posting Date")
                .required()
                .in_list_view(),
            FieldSpec::link("subscription", "Subscription").options("Subscription"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Subscription")
                .read_only()
                .print_hide(),
        ]
    );
}

#[test]
fn process_subscription_on_submit_batches_active_subscriptions() {
    let doc = ProcessSubscription::new("2026-05-23");
    let names = (1..=1001)
        .map(|idx| format!("SUB-{idx:04}"))
        .collect::<Vec<_>>();

    let jobs = doc.on_submit(&names);

    assert_eq!(jobs.len(), 3);
    assert_eq!(jobs[0].queue, "long");
    assert_eq!(
        jobs[0].method,
        "erpnext.accounts.doctype.subscription.subscription.process_all"
    );
    assert_eq!(jobs[0].posting_date, "2026-05-23");
    assert_eq!(jobs[0].subscriptions.len(), 500);
    assert_eq!(jobs[1].subscriptions.len(), 500);
    assert_eq!(jobs[2].subscriptions, vec!["SUB-1001"]);
}

#[test]
fn process_subscription_filters_single_subscription_before_batching() {
    let mut doc = ProcessSubscription::new("2026-05-23");
    doc.subscription = Some("SUB-0002".to_string());

    let jobs = doc.process_all_subscription(&[
        "SUB-0001".to_string(),
        "SUB-0002".to_string(),
        "SUB-0003".to_string(),
    ]);

    assert_eq!(
        jobs,
        vec![SubscriptionJob {
            method: "erpnext.accounts.doctype.subscription.subscription.process_all".to_string(),
            queue: "long".to_string(),
            subscriptions: vec!["SUB-0002".to_string()],
            posting_date: "2026-05-23".to_string(),
        }]
    );
    assert_eq!(doc.custom_hooks(), ["on_submit"]);
    assert_eq!(doc.doctype(), "Process Subscription");
    assert_eq!(doc.module(), "Accounts");
}

#[test]
fn create_subscription_process_matches_helper_defaults() {
    let doc = create_subscription_process(Some("SUB-0002"), Some("2026-05-23"));
    assert_eq!(doc.subscription.as_deref(), Some("SUB-0002"));
    assert_eq!(doc.posting_date, "2026-05-23");

    let today_doc = create_subscription_process(None, None);
    assert_eq!(today_doc.subscription, None);
    assert_eq!(today_doc.posting_date, "");
}
