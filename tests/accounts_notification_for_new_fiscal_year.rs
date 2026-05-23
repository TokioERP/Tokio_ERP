use tokio_erp::erpnext::accounts::notification::notification_for_new_fiscal_year::notification_for_new_fiscal_year::{
    get_context, FiscalYearNotificationContext, NotificationForNewFiscalYear,
};

#[test]
fn notification_for_new_fiscal_year_matches_erpnext_notification_metadata() {
    assert_eq!(
        NotificationForNewFiscalYear::NAME,
        "Notification for new fiscal year"
    );
    assert_eq!(NotificationForNewFiscalYear::MODULE, "Accounts");
    assert_eq!(
        NotificationForNewFiscalYear::SUBJECT,
        "New Fiscal Year {{ doc.name }} - Review Required"
    );
    assert_eq!(NotificationForNewFiscalYear::DOCUMENT_TYPE, "Fiscal Year");
    assert_eq!(NotificationForNewFiscalYear::EVENT, "New");
    assert_eq!(NotificationForNewFiscalYear::CHANNEL, "Email");
    assert!(NotificationForNewFiscalYear::ENABLED);
    assert_eq!(
        NotificationForNewFiscalYear::CONDITION,
        "doc.auto_created == 1"
    );
    assert!(!NotificationForNewFiscalYear::ATTACH_PRINT);
    assert_eq!(
        NotificationForNewFiscalYear::RECIPIENT_ROLES,
        ["Accounts User", "Accounts Manager"]
    );
}

#[test]
fn notification_template_preserves_company_and_disabled_branches() {
    let message = NotificationForNewFiscalYear::MESSAGE;

    assert!(message.contains("New Fiscal Year - {0}"));
    assert!(message.contains("doc.companies|length > 0"));
    assert!(message.contains("doc.companies|length < 2"));
    assert!(message.contains("doc.disabled"));
    assert!(message.contains("get_link_to_form(\"Fiscal Year\""));
}

#[test]
fn get_context_preserves_erpnext_noop_context_behavior() {
    let context = FiscalYearNotificationContext {
        name: "2026".to_string(),
        year_start_date: "2026-01-01".to_string(),
        year_end_date: "2026-12-31".to_string(),
        companies: vec!["Acme".to_string(), "Globex".to_string()],
        disabled: true,
        auto_created: true,
    };

    assert_eq!(get_context(context.clone()), context);
}
