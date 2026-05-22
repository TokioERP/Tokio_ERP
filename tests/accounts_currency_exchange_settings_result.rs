use tokio_erp::erpnext::accounts::doctype::currency_exchange_settings_result::currency_exchange_settings_result::CurrencyExchangeSettingsResult;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn currency_exchange_settings_result_matches_erpnext_metadata() {
    assert_eq!(
        CurrencyExchangeSettingsResult::DOCTYPE,
        "Currency Exchange Settings Result"
    );
    assert_eq!(CurrencyExchangeSettingsResult::MODULE, "Accounts");
    assert_eq!(CurrencyExchangeSettingsResult::FIELD_ORDER, ["key"]);
    assert!(CurrencyExchangeSettingsResult::IS_TABLE);
    assert!(CurrencyExchangeSettingsResult::EDITABLE_GRID);
    assert!(CurrencyExchangeSettingsResult::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(CurrencyExchangeSettingsResult::TRACK_CHANGES);

    assert_eq!(
        CurrencyExchangeSettingsResult::fields(),
        vec![FieldSpec::data("key", "Key").required().in_list_view()]
    );
}

#[test]
fn currency_exchange_settings_result_preserves_pass_controller_behavior() {
    let blank = CurrencyExchangeSettingsResult::default();
    assert_eq!(blank.key, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CurrencyExchangeSettingsResult::new("rates");
    assert_eq!(row.key.as_deref(), Some("rates"));
    assert_eq!(row.doctype(), "Currency Exchange Settings Result");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
