use tokio_erp::erpnext::accounts::doctype::discounted_invoice::discounted_invoice::DiscountedInvoice;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn discounted_invoice_matches_erpnext_metadata() {
    assert_eq!(DiscountedInvoice::DOCTYPE, "Discounted Invoice");
    assert_eq!(DiscountedInvoice::MODULE, "Accounts");
    assert_eq!(
        DiscountedInvoice::FIELD_ORDER,
        [
            "sales_invoice",
            "customer",
            "column_break_3",
            "posting_date",
            "outstanding_amount",
            "debit_to",
        ]
    );
    assert!(DiscountedInvoice::IS_TABLE);
    assert!(DiscountedInvoice::EDITABLE_GRID);
    assert!(DiscountedInvoice::QUICK_ENTRY);
    assert!(DiscountedInvoice::TRACK_CHANGES);

    assert_eq!(
        DiscountedInvoice::fields(),
        vec![
            FieldSpec::link("sales_invoice", "Invoice")
                .options("Sales Invoice")
                .required()
                .in_list_view()
                .search_index(),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .fetch_from("sales_invoice.customer")
                .read_only()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::date("posting_date", "Date")
                .fetch_from("sales_invoice.posting_date")
                .read_only()
                .in_list_view(),
            FieldSpec::currency("outstanding_amount", "Outstanding Amount")
                .options("Company:company:default_currency")
                .fetch_from("sales_invoice.outstanding_amount")
                .fetch_if_empty()
                .in_list_view(),
            FieldSpec::link("debit_to", "Debit to")
                .options("Account")
                .fetch_from("sales_invoice.debit_to")
                .read_only(),
        ]
    );
}

#[test]
fn discounted_invoice_preserves_pass_controller_behavior() {
    let blank = DiscountedInvoice::default();
    assert_eq!(blank.sales_invoice, None);
    assert_eq!(blank.customer, None);
    assert_eq!(blank.posting_date, None);
    assert_eq!(blank.outstanding_amount, None);
    assert_eq!(blank.debit_to, None);
    assert!(blank.custom_hooks().is_empty());

    let row = DiscountedInvoice::new("SINV-0001");
    assert_eq!(row.sales_invoice.as_deref(), Some("SINV-0001"));
    assert_eq!(row.doctype(), "Discounted Invoice");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
