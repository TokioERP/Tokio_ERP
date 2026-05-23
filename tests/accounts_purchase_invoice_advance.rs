use tokio_erp::erpnext::accounts::doctype::purchase_invoice_advance::purchase_invoice_advance::PurchaseInvoiceAdvance;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn purchase_invoice_advance_matches_erpnext_metadata() {
    assert_eq!(PurchaseInvoiceAdvance::DOCTYPE, "Purchase Invoice Advance");
    assert_eq!(PurchaseInvoiceAdvance::MODULE, "Accounts");
    assert_eq!(
        PurchaseInvoiceAdvance::FIELD_ORDER,
        [
            "reference_type",
            "reference_name",
            "remarks",
            "reference_row",
            "col_break1",
            "advance_amount",
            "allocated_amount",
            "exchange_gain_loss",
            "ref_exchange_rate",
            "difference_posting_date",
        ]
    );
    assert!(PurchaseInvoiceAdvance::IS_TABLE);
    assert!(PurchaseInvoiceAdvance::EDITABLE_GRID);
    assert!(PurchaseInvoiceAdvance::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        PurchaseInvoiceAdvance::fields(),
        vec![
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .read_only()
                .no_copy(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy(),
            FieldSpec::text("remarks", "Remarks")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy(),
            FieldSpec::data("reference_row", "Reference Row")
                .read_only()
                .hidden()
                .print_hide()
                .no_copy(),
            FieldSpec::column_break("col_break1"),
            FieldSpec::currency("advance_amount", "Advance Amount")
                .options("party_account_currency")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("party_account_currency")
                .in_list_view()
                .columns(2)
                .no_copy(),
            FieldSpec::currency("exchange_gain_loss", "Exchange Gain/Loss")
                .options("Company:company:default_currency")
                .read_only()
                .depends_on("exchange_gain_loss"),
            FieldSpec::float("ref_exchange_rate", "Reference Exchange Rate")
                .read_only()
                .depends_on("exchange_gain_loss"),
            FieldSpec::date("difference_posting_date", "Difference Posting Date")
                .in_list_view()
                .columns(2),
        ]
    );
}

#[test]
fn purchase_invoice_advance_preserves_pass_controller_behavior() {
    let blank = PurchaseInvoiceAdvance::default();
    assert_eq!(blank.reference_type, None);
    assert_eq!(blank.reference_name, None);
    assert_eq!(blank.allocated_amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PurchaseInvoiceAdvance::new("Payment Entry", "PE-0001");
    assert_eq!(row.reference_type.as_deref(), Some("Payment Entry"));
    assert_eq!(row.reference_name.as_deref(), Some("PE-0001"));
    assert_eq!(row.doctype(), "Purchase Invoice Advance");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
