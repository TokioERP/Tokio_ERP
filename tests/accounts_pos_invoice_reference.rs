use tokio_erp::erpnext::accounts::doctype::pos_invoice_reference::pos_invoice_reference::PosInvoiceReference;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_invoice_reference_matches_erpnext_metadata() {
    assert_eq!(PosInvoiceReference::DOCTYPE, "POS Invoice Reference");
    assert_eq!(PosInvoiceReference::MODULE, "Accounts");
    assert_eq!(
        PosInvoiceReference::FIELD_ORDER,
        [
            "pos_invoice",
            "posting_date",
            "column_break_3",
            "customer",
            "grand_total",
            "is_return",
            "return_against",
        ]
    );
    assert!(PosInvoiceReference::IS_TABLE);
    assert!(PosInvoiceReference::EDITABLE_GRID);

    assert_eq!(
        PosInvoiceReference::fields(),
        vec![
            FieldSpec::link("pos_invoice", "POS Invoice")
                .options("POS Invoice")
                .required()
                .in_list_view(),
            FieldSpec::date("posting_date", "Date")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .required()
                .read_only(),
            FieldSpec::currency("grand_total", "Amount")
                .required()
                .in_list_view(),
            FieldSpec::check("is_return", "Is Return")
                .read_only()
                .default("0"),
            FieldSpec::link("return_against", "Return Against")
                .options("POS Invoice")
                .read_only(),
        ]
    );
}

#[test]
fn pos_invoice_reference_preserves_pass_controller_behavior() {
    let blank = PosInvoiceReference::default();
    assert_eq!(blank.pos_invoice, None);
    assert_eq!(blank.posting_date, None);
    assert_eq!(blank.customer, None);
    assert_eq!(blank.grand_total, None);
    assert!(!blank.is_return);
    assert_eq!(blank.return_against, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosInvoiceReference::new("POS-INV-0001", "2026-05-22", "Wikki", "100");
    assert_eq!(row.pos_invoice.as_deref(), Some("POS-INV-0001"));
    assert_eq!(row.posting_date.as_deref(), Some("2026-05-22"));
    assert_eq!(row.customer.as_deref(), Some("Wikki"));
    assert_eq!(row.grand_total.as_deref(), Some("100"));
    assert_eq!(row.doctype(), "POS Invoice Reference");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
