use tokio_erp::erpnext::accounts::doctype::pos_opening_entry_detail::pos_opening_entry_detail::PosOpeningEntryDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_opening_entry_detail_matches_erpnext_metadata() {
    assert_eq!(PosOpeningEntryDetail::DOCTYPE, "POS Opening Entry Detail");
    assert_eq!(PosOpeningEntryDetail::MODULE, "Accounts");
    assert_eq!(
        PosOpeningEntryDetail::FIELD_ORDER,
        ["mode_of_payment", "opening_amount"]
    );
    assert!(PosOpeningEntryDetail::IS_TABLE);
    assert!(PosOpeningEntryDetail::EDITABLE_GRID);

    assert_eq!(
        PosOpeningEntryDetail::fields(),
        vec![
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
            FieldSpec::currency("opening_amount", "Opening Amount")
                .options("company:company_currency")
                .required()
                .in_list_view()
                .default("0"),
        ]
    );
}

#[test]
fn pos_opening_entry_detail_preserves_pass_controller_behavior() {
    let blank = PosOpeningEntryDetail::default();
    assert_eq!(blank.mode_of_payment, None);
    assert_eq!(blank.opening_amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosOpeningEntryDetail::new("Cash", "100");
    assert_eq!(row.mode_of_payment.as_deref(), Some("Cash"));
    assert_eq!(row.opening_amount.as_deref(), Some("100"));
    assert_eq!(row.doctype(), "POS Opening Entry Detail");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
