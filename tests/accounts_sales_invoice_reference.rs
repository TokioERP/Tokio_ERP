use tokio_erp::erpnext::accounts::doctype::sales_invoice_reference::sales_invoice_reference::SalesInvoiceReference;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_invoice_reference_matches_erpnext_metadata() {
    assert_eq!(SalesInvoiceReference::DOCTYPE, "Sales Invoice Reference");
    assert_eq!(SalesInvoiceReference::MODULE, "Accounts");
    assert_eq!(
        SalesInvoiceReference::FIELD_ORDER,
        [
            "sales_invoice",
            "posting_date",
            "column_break_fear",
            "customer",
            "grand_total",
            "is_return",
            "return_against",
        ]
    );
    assert!(SalesInvoiceReference::ALLOW_RENAME);
    assert!(SalesInvoiceReference::IS_TABLE);
    assert!(SalesInvoiceReference::EDITABLE_GRID);
    assert!(SalesInvoiceReference::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        SalesInvoiceReference::fields(),
        vec![
            FieldSpec::link("sales_invoice", "Sales Invoice")
                .options("Sales Invoice")
                .required()
                .in_list_view(),
            FieldSpec::date("posting_date", "Date")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_fear"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .fetch_from("sales_invoice.customer")
                .read_only()
                .required(),
            FieldSpec::currency("grand_total", "Amount")
                .fetch_from("sales_invoice.grand_total")
                .required()
                .in_list_view(),
            FieldSpec::check("is_return", "Is Return")
                .default("0")
                .fetch_from("sales_invoice.is_return")
                .read_only(),
            FieldSpec::link("return_against", "Return Against")
                .options("Sales Invoice")
                .fetch_from("sales_invoice.return_against")
                .read_only(),
        ]
    );
}

#[test]
fn sales_invoice_reference_controller_is_pass_through() {
    let row = SalesInvoiceReference::new("SINV-0001", "2026-05-23");
    assert_eq!(row.sales_invoice.as_deref(), Some("SINV-0001"));
    assert_eq!(row.posting_date.as_deref(), Some("2026-05-23"));
    assert_eq!(row.doctype(), "Sales Invoice Reference");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
