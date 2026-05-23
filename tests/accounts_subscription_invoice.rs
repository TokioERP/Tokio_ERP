use tokio_erp::erpnext::accounts::doctype::subscription_invoice::subscription_invoice::SubscriptionInvoice;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn subscription_invoice_matches_erpnext_metadata() {
    assert_eq!(SubscriptionInvoice::DOCTYPE, "Subscription Invoice");
    assert_eq!(SubscriptionInvoice::MODULE, "Accounts");
    assert_eq!(
        SubscriptionInvoice::FIELD_ORDER,
        ["document_type", "invoice"]
    );
    assert!(SubscriptionInvoice::EDITABLE_GRID);
    assert!(SubscriptionInvoice::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(SubscriptionInvoice::IS_TABLE);
    assert!(SubscriptionInvoice::QUICK_ENTRY);
    assert_eq!(SubscriptionInvoice::SORT_FIELD, "creation");
    assert_eq!(SubscriptionInvoice::SORT_ORDER, "DESC");
    assert!(SubscriptionInvoice::TRACK_CHANGES);
    assert_eq!(
        SubscriptionInvoice::fields(),
        vec![
            FieldSpec::link("document_type", "Document Type ")
                .options("DocType")
                .read_only()
                .no_copy(),
            FieldSpec::dynamic_link("invoice")
                .label("Invoice")
                .options("document_type")
                .read_only()
                .no_copy()
                .in_list_view(),
        ]
    );
}

#[test]
fn subscription_invoice_preserves_pass_controller_behavior() {
    let invoice = SubscriptionInvoice::new("Sales Invoice", "SINV-0001");

    assert_eq!(invoice.document_type.as_deref(), Some("Sales Invoice"));
    assert_eq!(invoice.invoice.as_deref(), Some("SINV-0001"));
    assert_eq!(invoice.doctype(), "Subscription Invoice");
    assert_eq!(invoice.module(), "Accounts");
    assert!(invoice.custom_hooks().is_empty());
}
