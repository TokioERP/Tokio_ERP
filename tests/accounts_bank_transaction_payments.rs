use tokio_erp::erpnext::accounts::doctype::bank_transaction_payments::bank_transaction_payments::BankTransactionPayments;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_transaction_payments_matches_erpnext_metadata() {
    assert_eq!(
        BankTransactionPayments::DOCTYPE,
        "Bank Transaction Payments"
    );
    assert_eq!(BankTransactionPayments::MODULE, "Accounts");
    assert_eq!(
        BankTransactionPayments::FIELD_ORDER,
        [
            "payment_document",
            "payment_entry",
            "allocated_amount",
            "clearance_date",
        ]
    );
    assert!(BankTransactionPayments::IS_TABLE);
    assert!(BankTransactionPayments::QUICK_ENTRY);
    assert!(BankTransactionPayments::TRACK_CHANGES);

    assert_eq!(
        BankTransactionPayments::fields(),
        vec![
            FieldSpec::link("payment_document", "Payment Document")
                .options("DocType")
                .required()
                .in_list_view(),
            FieldSpec::dynamic_link("payment_entry")
                .label("Payment Entry")
                .options("payment_document")
                .required()
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .required()
                .in_list_view(),
            FieldSpec::date("clearance_date", "Clearance Date")
                .depends_on("eval:doc.docstatus==1")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn bank_transaction_payments_preserves_pass_controller_behavior() {
    let blank = BankTransactionPayments::default();
    assert_eq!(blank.payment_document, None);
    assert_eq!(blank.payment_entry, None);
    assert_eq!(blank.allocated_amount, None);
    assert_eq!(blank.clearance_date, None);
    assert!(blank.custom_hooks().is_empty());

    let row = BankTransactionPayments::new("Payment Entry", "PE-0001", "100.00");
    assert_eq!(row.payment_document.as_deref(), Some("Payment Entry"));
    assert_eq!(row.payment_entry.as_deref(), Some("PE-0001"));
    assert_eq!(row.allocated_amount.as_deref(), Some("100.00"));
    assert_eq!(row.doctype(), "Bank Transaction Payments");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
