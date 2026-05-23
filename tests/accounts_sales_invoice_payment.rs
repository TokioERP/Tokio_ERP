use tokio_erp::erpnext::accounts::doctype::sales_invoice_payment::sales_invoice_payment::SalesInvoicePayment;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_invoice_payment_matches_erpnext_metadata() {
    assert_eq!(SalesInvoicePayment::DOCTYPE, "Sales Invoice Payment");
    assert_eq!(SalesInvoicePayment::MODULE, "Accounts");
    assert_eq!(
        SalesInvoicePayment::FIELD_ORDER,
        [
            "default",
            "mode_of_payment",
            "amount",
            "reference_no",
            "column_break_3",
            "account",
            "type",
            "base_amount",
            "clearance_date",
        ]
    );
    assert!(SalesInvoicePayment::IS_TABLE);
    assert!(SalesInvoicePayment::EDITABLE_GRID);
    assert!(SalesInvoicePayment::QUICK_ENTRY);
    assert_eq!(
        SalesInvoicePayment::fields(),
        vec![
            FieldSpec::check("default", "Default")
                .default("0")
                .hidden()
                .read_only(),
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .default("0")
                .required()
                .in_list_view()
                .depends_on("eval: [\"POS Invoice\", \"Sales Invoice\"].includes(parent.doctype)"),
            FieldSpec::data("reference_no", "Reference No"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("account", "Account")
                .options("Account")
                .read_only()
                .print_hide(),
            FieldSpec::read_only_field("type", "Type").fetch_from("mode_of_payment.type"),
            FieldSpec::currency("base_amount", "Base Amount (Company Currency)")
                .options("Company:company:default_currency")
                .read_only()
                .print_hide()
                .no_copy(),
            FieldSpec::date("clearance_date", "Clearance Date")
                .read_only()
                .print_hide()
                .no_copy(),
        ]
    );
}

#[test]
fn sales_invoice_payment_controller_is_pass_through() {
    let row = SalesInvoicePayment::new("Cash", "100");
    assert_eq!(row.mode_of_payment.as_deref(), Some("Cash"));
    assert_eq!(row.amount.as_deref(), Some("100"));
    assert_eq!(row.doctype(), "Sales Invoice Payment");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
