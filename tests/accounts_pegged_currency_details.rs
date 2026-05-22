use tokio_erp::erpnext::accounts::doctype::pegged_currency_details::pegged_currency_details::PeggedCurrencyDetails;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pegged_currency_details_matches_erpnext_metadata() {
    assert_eq!(PeggedCurrencyDetails::DOCTYPE, "Pegged Currency Details");
    assert_eq!(PeggedCurrencyDetails::MODULE, "Accounts");
    assert_eq!(
        PeggedCurrencyDetails::FIELD_ORDER,
        ["source_currency", "pegged_against", "pegged_exchange_rate"]
    );
    assert!(PeggedCurrencyDetails::IS_TABLE);
    assert!(PeggedCurrencyDetails::EDITABLE_GRID);
    assert!(PeggedCurrencyDetails::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        PeggedCurrencyDetails::fields(),
        vec![
            FieldSpec::link("source_currency", "Currency")
                .options("Currency")
                .in_list_view(),
            FieldSpec::link("pegged_against", "Pegged Against")
                .options("Currency")
                .in_list_view(),
            FieldSpec::data("pegged_exchange_rate", "Exchange Rate").in_list_view(),
        ]
    );
}

#[test]
fn pegged_currency_details_preserves_pass_controller_behavior() {
    let blank = PeggedCurrencyDetails::default();
    assert_eq!(blank.source_currency, None);
    assert_eq!(blank.pegged_against, None);
    assert_eq!(blank.pegged_exchange_rate, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PeggedCurrencyDetails::new("USD", "AED", "3.6725");
    assert_eq!(row.source_currency.as_deref(), Some("USD"));
    assert_eq!(row.pegged_against.as_deref(), Some("AED"));
    assert_eq!(row.pegged_exchange_rate.as_deref(), Some("3.6725"));
    assert_eq!(row.doctype(), "Pegged Currency Details");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
