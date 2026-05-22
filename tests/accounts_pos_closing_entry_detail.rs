use tokio_erp::erpnext::accounts::doctype::pos_closing_entry_detail::pos_closing_entry_detail::PosClosingEntryDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_closing_entry_detail_matches_erpnext_metadata() {
    assert_eq!(PosClosingEntryDetail::DOCTYPE, "POS Closing Entry Detail");
    assert_eq!(PosClosingEntryDetail::MODULE, "Accounts");
    assert_eq!(
        PosClosingEntryDetail::FIELD_ORDER,
        [
            "mode_of_payment",
            "opening_amount",
            "expected_amount",
            "closing_amount",
            "difference",
        ]
    );
    assert!(PosClosingEntryDetail::IS_TABLE);
    assert!(PosClosingEntryDetail::EDITABLE_GRID);
    assert!(PosClosingEntryDetail::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        PosClosingEntryDetail::fields(),
        vec![
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
            FieldSpec::currency("opening_amount", "Opening Amount")
                .options("company:company_currency")
                .required()
                .in_list_view()
                .read_only(),
            FieldSpec::currency("expected_amount", "Expected Amount")
                .options("company:company_currency")
                .in_list_view()
                .read_only(),
            FieldSpec::currency("closing_amount", "Closing Amount")
                .options("company:company_currency")
                .required()
                .in_list_view()
                .default("0"),
            FieldSpec::currency("difference", "Difference")
                .options("company:company_currency")
                .in_list_view()
                .read_only(),
        ]
    );
}

#[test]
fn pos_closing_entry_detail_preserves_pass_controller_behavior() {
    let blank = PosClosingEntryDetail::default();
    assert_eq!(blank.mode_of_payment, None);
    assert_eq!(blank.opening_amount, None);
    assert_eq!(blank.expected_amount, None);
    assert_eq!(blank.closing_amount, None);
    assert_eq!(blank.difference, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosClosingEntryDetail::new("Cash", "10", "15", "20", "5");
    assert_eq!(row.mode_of_payment.as_deref(), Some("Cash"));
    assert_eq!(row.opening_amount.as_deref(), Some("10"));
    assert_eq!(row.expected_amount.as_deref(), Some("15"));
    assert_eq!(row.closing_amount.as_deref(), Some("20"));
    assert_eq!(row.difference.as_deref(), Some("5"));
    assert_eq!(row.doctype(), "POS Closing Entry Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
