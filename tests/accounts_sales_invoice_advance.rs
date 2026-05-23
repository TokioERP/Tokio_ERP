use tokio_erp::erpnext::accounts::doctype::sales_invoice_advance::sales_invoice_advance::SalesInvoiceAdvance;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_invoice_advance_matches_erpnext_metadata() {
    assert_eq!(SalesInvoiceAdvance::DOCTYPE, "Sales Invoice Advance");
    assert_eq!(SalesInvoiceAdvance::MODULE, "Accounts");
    assert_eq!(
        SalesInvoiceAdvance::FIELD_ORDER,
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
    assert!(SalesInvoiceAdvance::IS_TABLE);
    assert!(SalesInvoiceAdvance::EDITABLE_GRID);
    assert!(SalesInvoiceAdvance::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(SalesInvoiceAdvance::QUICK_ENTRY);
    assert_eq!(
        SalesInvoiceAdvance::fields(),
        vec![
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .read_only()
                .no_copy()
                .oldfield("journal_voucher", "Link")
                .width("250px"),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .read_only()
                .in_list_view()
                .columns(2)
                .print_hide()
                .no_copy(),
            FieldSpec::text("remarks", "Remarks")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy()
                .oldfield("remarks", "Small Text")
                .width("150px"),
            FieldSpec::data("reference_row", "Reference Row")
                .read_only()
                .hidden()
                .print_hide()
                .no_copy()
                .oldfield("jv_detail_no", "Data")
                .width("120px"),
            FieldSpec::column_break("col_break1"),
            FieldSpec::currency("advance_amount", "Advance amount")
                .options("party_account_currency")
                .read_only()
                .in_list_view()
                .columns(2)
                .no_copy()
                .oldfield("advance_amount", "Currency")
                .width("120px"),
            FieldSpec::currency("allocated_amount", "Allocated amount")
                .options("party_account_currency")
                .in_list_view()
                .columns(2)
                .no_copy()
                .oldfield("allocated_amount", "Currency")
                .width("120px"),
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
fn sales_invoice_advance_controller_is_pass_through() {
    let row = SalesInvoiceAdvance::new("Payment Entry", "PAY-0001");
    assert_eq!(row.reference_type.as_deref(), Some("Payment Entry"));
    assert_eq!(row.reference_name.as_deref(), Some("PAY-0001"));
    assert_eq!(row.doctype(), "Sales Invoice Advance");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
