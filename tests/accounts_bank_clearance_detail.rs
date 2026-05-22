use tokio_erp::erpnext::accounts::doctype::bank_clearance_detail::bank_clearance_detail::BankClearanceDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_clearance_detail_matches_erpnext_metadata() {
    assert_eq!(BankClearanceDetail::DOCTYPE, "Bank Clearance Detail");
    assert_eq!(BankClearanceDetail::MODULE, "Accounts");
    assert_eq!(
        BankClearanceDetail::FIELD_ORDER,
        [
            "payment_document",
            "payment_entry",
            "against_account",
            "amount",
            "column_break_5",
            "posting_date",
            "cheque_number",
            "cheque_date",
            "clearance_date",
        ]
    );
    assert!(BankClearanceDetail::IS_TABLE);
    assert!(BankClearanceDetail::QUICK_ENTRY);
    assert_eq!(BankClearanceDetail::GRID_PAGE_LENGTH, 50);
    assert_eq!(BankClearanceDetail::ROW_FORMAT, "Dynamic");

    assert_eq!(
        BankClearanceDetail::fields(),
        vec![
            FieldSpec::link("payment_document", "Payment Document").options("DocType"),
            FieldSpec::dynamic_link("payment_entry")
                .label("Payment Entry")
                .options("payment_document")
                .columns(2)
                .in_list_view()
                .oldfield("voucher_id", "Link"),
            FieldSpec::data("against_account", "Against Account")
                .columns(2)
                .in_list_view()
                .read_only()
                .oldfield("against_account", "Data")
                .width("15"),
            FieldSpec::data("amount", "Amount")
                .columns(2)
                .in_list_view()
                .read_only()
                .oldfield("debit", "Currency"),
            FieldSpec::column_break("column_break_5").width("50%"),
            FieldSpec::date("posting_date", "Posting Date")
                .columns(2)
                .read_only()
                .oldfield("posting_date", "Date"),
            FieldSpec::data("cheque_number", "Cheque Number")
                .columns(1)
                .in_list_view()
                .read_only()
                .oldfield("cheque_number", "Data"),
            FieldSpec::date("cheque_date", "Cheque Date")
                .columns(2)
                .in_list_view()
                .read_only()
                .oldfield("cheque_date", "Date"),
            FieldSpec::date("clearance_date", "Clearance Date")
                .columns(2)
                .in_list_view()
                .oldfield("clearance_date", "Date"),
        ]
    );
}

#[test]
fn bank_clearance_detail_preserves_pass_controller_behavior() {
    let blank = BankClearanceDetail::default();
    assert_eq!(blank.payment_document, None);
    assert_eq!(blank.payment_entry, None);
    assert_eq!(blank.clearance_date, None);
    assert!(blank.custom_hooks().is_empty());

    let row = BankClearanceDetail::new("Payment Entry", "PE-0001");
    assert_eq!(row.payment_document.as_deref(), Some("Payment Entry"));
    assert_eq!(row.payment_entry.as_deref(), Some("PE-0001"));
    assert_eq!(row.doctype(), "Bank Clearance Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
