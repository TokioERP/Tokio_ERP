use tokio_erp::erpnext::accounts::doctype::payment_entry_reference::payment_entry_reference::PaymentEntryReference;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_entry_reference_matches_erpnext_metadata() {
    assert_eq!(PaymentEntryReference::DOCTYPE, "Payment Entry Reference");
    assert_eq!(PaymentEntryReference::MODULE, "Accounts");
    assert!(PaymentEntryReference::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(PaymentEntryReference::IS_TABLE);
    assert!(PaymentEntryReference::QUICK_ENTRY);
    assert_eq!(PaymentEntryReference::ROW_FORMAT, "Dynamic");
    assert_eq!(PaymentEntryReference::SORT_FIELD, "creation");
    assert_eq!(PaymentEntryReference::SORT_ORDER, "DESC");
    assert!(PaymentEntryReference::TRACK_CHANGES);
    assert_eq!(
        PaymentEntryReference::FIELD_ORDER,
        [
            "reference_doctype",
            "reference_name",
            "due_date",
            "bill_no",
            "payment_term",
            "payment_term_outstanding",
            "account_type",
            "payment_type",
            "reconcile_effect_on",
            "column_break_4",
            "total_amount",
            "outstanding_amount",
            "allocated_amount",
            "exchange_rate",
            "exchange_gain_loss",
            "account",
            "payment_request",
            "payment_request_outstanding",
            "advance_voucher_type",
            "advance_voucher_no",
        ]
    );

    assert_eq!(
        PaymentEntryReference::fields(),
        vec![
            FieldSpec::link("reference_doctype", "Type")
                .options("DocType")
                .columns(2)
                .required()
                .in_list_view()
                .search_index(),
            FieldSpec::dynamic_link("reference_name")
                .label("Name")
                .options("reference_doctype")
                .columns(4)
                .required()
                .in_global_search()
                .in_list_view()
                .search_index(),
            FieldSpec::date("due_date", "Due Date")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::data("bill_no", "Supplier Invoice No")
                .read_only()
                .no_copy(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::currency("total_amount", "Grand Total")
                .columns(2)
                .read_only()
                .print_hide()
                .in_list_view(),
            FieldSpec::currency("outstanding_amount", "Outstanding")
                .columns(2)
                .read_only()
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated")
                .columns(2)
                .in_list_view(),
            FieldSpec::float("exchange_rate", "Exchange Rate")
                .depends_on("eval:(doc.reference_doctype=='Purchase Invoice')")
                .read_only()
                .print_hide(),
            FieldSpec::link("payment_term", "Payment Term").options("Payment Term"),
            FieldSpec::currency("exchange_gain_loss", "Exchange Gain/Loss")
                .options("Company:company:default_currency")
                .depends_on("exchange_gain_loss")
                .read_only(),
            FieldSpec::link("account", "Account").options("Account"),
            FieldSpec::data("account_type", "Account Type"),
            FieldSpec::data("payment_type", "Payment Type"),
            FieldSpec::link("payment_request", "Payment Request").options("Payment Request"),
            FieldSpec::float("payment_term_outstanding", "Payment Term Outstanding")
                .depends_on("eval: doc.payment_term")
                .read_only(),
            FieldSpec::float("payment_request_outstanding", "Payment Request Outstanding")
                .depends_on("eval: doc.payment_request && doc.payment_request_outstanding")
                .read_only()
                .is_virtual(),
            FieldSpec::date("reconcile_effect_on", "Reconcile Effect On").read_only(),
            FieldSpec::link("advance_voucher_type", "Advance Voucher Type")
                .options("DocType")
                .columns(2)
                .read_only(),
            FieldSpec::dynamic_link("advance_voucher_no")
                .label("Advance Voucher No")
                .options("advance_voucher_type")
                .columns(2)
                .read_only(),
        ]
    );
}

#[test]
fn payment_entry_reference_preserves_payment_request_outstanding_property() {
    let blank = PaymentEntryReference::default();
    assert_eq!(
        blank.payment_request_outstanding_with(|_| Some(100.0)),
        None
    );

    let row = PaymentEntryReference {
        payment_request: Some("PREQ-0001".to_string()),
    };
    assert_eq!(
        row.payment_request_outstanding_with(|name| {
            assert_eq!(name, "PREQ-0001");
            Some(42.5)
        }),
        Some(42.5)
    );
    assert_eq!(row.doctype(), "Payment Entry Reference");
    assert!(row.custom_hooks().is_empty());
}
