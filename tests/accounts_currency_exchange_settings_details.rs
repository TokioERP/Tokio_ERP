use tokio_erp::erpnext::accounts::doctype::currency_exchange_settings_details::currency_exchange_settings_details::CurrencyExchangeSettingsDetails;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn currency_exchange_settings_details_matches_erpnext_metadata() {
    assert_eq!(
        CurrencyExchangeSettingsDetails::DOCTYPE,
        "Currency Exchange Settings Details"
    );
    assert_eq!(CurrencyExchangeSettingsDetails::MODULE, "Accounts");
    assert_eq!(
        CurrencyExchangeSettingsDetails::FIELD_ORDER,
        ["key", "value"]
    );
    assert!(CurrencyExchangeSettingsDetails::IS_TABLE);
    assert!(CurrencyExchangeSettingsDetails::EDITABLE_GRID);
    assert!(CurrencyExchangeSettingsDetails::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(CurrencyExchangeSettingsDetails::TRACK_CHANGES);

    assert_eq!(
        CurrencyExchangeSettingsDetails::fields(),
        vec![
            FieldSpec::data("key", "Key").required().in_list_view(),
            FieldSpec::data("value", "Value").required().in_list_view(),
        ]
    );
}

#[test]
fn currency_exchange_settings_details_preserves_pass_controller_behavior() {
    let blank = CurrencyExchangeSettingsDetails::default();
    assert_eq!(blank.key, None);
    assert_eq!(blank.value, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CurrencyExchangeSettingsDetails::new("access_key", "secret");
    assert_eq!(row.key.as_deref(), Some("access_key"));
    assert_eq!(row.value.as_deref(), Some("secret"));
    assert_eq!(row.doctype(), "Currency Exchange Settings Details");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
