use serde_json::json;

use tokio_erp::erpnext::accounts::doctype::currency_exchange_settings::currency_exchange_settings::{
    get_api_endpoint, CurrencyExchangeRequestPlan, CurrencyExchangeSettings,
    CurrencyExchangeSettingsError,
};
use tokio_erp::erpnext::accounts::doctype::currency_exchange_settings_details::currency_exchange_settings_details::CurrencyExchangeSettingsDetails;
use tokio_erp::erpnext::accounts::doctype::currency_exchange_settings_result::currency_exchange_settings_result::CurrencyExchangeSettingsResult;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn currency_exchange_settings_matches_erpnext_metadata() {
    assert_eq!(
        CurrencyExchangeSettings::DOCTYPE,
        "Currency Exchange Settings"
    );
    assert_eq!(CurrencyExchangeSettings::MODULE, "Accounts");
    assert_eq!(
        CurrencyExchangeSettings::FIELD_ORDER,
        [
            "api_details_section",
            "disabled",
            "service_provider",
            "api_endpoint",
            "use_http",
            "access_key",
            "url",
            "column_break_3",
            "help",
            "section_break_2",
            "req_params",
            "column_break_4",
            "result_key",
        ]
    );
    assert!(CurrencyExchangeSettings::EDITABLE_GRID);
    assert!(CurrencyExchangeSettings::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(CurrencyExchangeSettings::IS_SINGLE);
    assert_eq!(CurrencyExchangeSettings::ROW_FORMAT, "Dynamic");
    assert_eq!(CurrencyExchangeSettings::SORT_FIELD, "creation");
    assert_eq!(CurrencyExchangeSettings::SORT_ORDER, "DESC");
    assert!(CurrencyExchangeSettings::TRACK_CHANGES);

    let fields = CurrencyExchangeSettings::fields();
    assert_eq!(fields.len(), 13);
    assert_eq!(
        fields[3],
        FieldSpec::data("api_endpoint", "API Endpoint")
            .in_list_view()
            .read_only_depends_on("eval: doc.service_provider != \"Custom\"")
            .required()
    );
    assert_eq!(
        fields[10],
        FieldSpec::table("req_params", "Parameters")
            .options("Currency Exchange Settings Details")
            .read_only_depends_on("eval: doc.service_provider != \"Custom\"")
            .required()
    );
    assert_eq!(
        fields[12],
        FieldSpec::table("result_key", "Result Key")
            .options("Currency Exchange Settings Result")
            .read_only_depends_on("eval: doc.service_provider != \"Custom\"")
            .required()
    );
}

#[test]
fn currency_exchange_endpoint_and_provider_setup_match_erpnext() {
    assert_eq!(
        get_api_endpoint(Some("exchangerate.host"), false),
        Some("https://api.exchangerate.host/convert".to_string())
    );
    assert_eq!(
        get_api_endpoint(Some("frankfurter.app"), true),
        Some("http://api.frankfurter.app/{transaction_date}".to_string())
    );
    assert_eq!(
        get_api_endpoint(Some("frankfurter.dev"), false),
        Some("https://api.frankfurter.dev/v1/{transaction_date}".to_string())
    );
    assert_eq!(get_api_endpoint(Some("Custom"), false), None);

    let mut settings = CurrencyExchangeSettings {
        service_provider: Some("exchangerate.host".to_string()),
        access_key: Some("SECRET".to_string()),
        ..Default::default()
    };
    settings
        .set_parameters_and_result()
        .expect("provider setup");
    assert_eq!(
        settings.api_endpoint.as_deref(),
        Some("https://api.exchangerate.host/convert")
    );
    assert_eq!(
        settings.result_key,
        vec![CurrencyExchangeSettingsResult::new("result")]
    );
    assert_eq!(
        settings.req_params,
        vec![
            CurrencyExchangeSettingsDetails::new("access_key", "SECRET"),
            CurrencyExchangeSettingsDetails::new("amount", "1"),
            CurrencyExchangeSettingsDetails::new("date", "{transaction_date}"),
            CurrencyExchangeSettingsDetails::new("from", "{from_currency}"),
            CurrencyExchangeSettingsDetails::new("to", "{to_currency}"),
        ]
    );

    let mut missing_key = CurrencyExchangeSettings {
        service_provider: Some("exchangerate.host".to_string()),
        ..Default::default()
    };
    assert_eq!(
        missing_key.set_parameters_and_result(),
        Err(CurrencyExchangeSettingsError::AccessKeyRequired {
            service_provider: "exchangerate.host".to_string()
        })
    );

    let mut frankfurter = CurrencyExchangeSettings {
        service_provider: Some("frankfurter.dev".to_string()),
        use_http: true,
        ..Default::default()
    };
    frankfurter
        .set_parameters_and_result()
        .expect("provider setup");
    assert_eq!(
        frankfurter.api_endpoint.as_deref(),
        Some("http://api.frankfurter.dev/v1/{transaction_date}")
    );
    assert_eq!(
        frankfurter.result_key,
        vec![
            CurrencyExchangeSettingsResult::new("rates"),
            CurrencyExchangeSettingsResult::new("{to_currency}"),
        ]
    );
}

#[test]
fn currency_exchange_request_and_result_validation_match_erpnext() {
    let mut settings = CurrencyExchangeSettings {
        api_endpoint: Some("https://rates.example/{transaction_date}".to_string()),
        req_params: vec![
            CurrencyExchangeSettingsDetails::new("base", "{from_currency}"),
            CurrencyExchangeSettingsDetails::new("symbols", "{to_currency}"),
        ],
        result_key: vec![
            CurrencyExchangeSettingsResult::new("rates"),
            CurrencyExchangeSettingsResult::new("{to_currency}"),
        ],
        ..Default::default()
    };

    assert_eq!(
        settings.validate_parameters_plan("2026-05-30", "USD", "INR"),
        CurrencyExchangeRequestPlan {
            api_url: "https://rates.example/2026-05-30".to_string(),
            params: vec![
                ("base".to_string(), "USD".to_string()),
                ("symbols".to_string(), "INR".to_string()),
            ],
        }
    );

    assert_eq!(
        settings.validate_result(
            "https://rates.example/2026-05-30?base=USD&symbols=INR",
            "body",
            &json!({"rates": {"INR": 83.25}}),
            "2026-05-30",
            "USD",
            "INR",
        ),
        Ok(83.25)
    );
    assert_eq!(
        settings.url.as_deref(),
        Some("https://rates.example/2026-05-30?base=USD&symbols=INR")
    );

    assert_eq!(
        settings.validate_result(
            "url",
            "raw response",
            &json!({"rates": {}}),
            "2026-05-30",
            "USD",
            "INR"
        ),
        Err(CurrencyExchangeSettingsError::InvalidResultKey {
            response_text: "raw response".to_string()
        })
    );
    assert_eq!(
        settings.validate_result(
            "url",
            "raw response",
            &json!({"rates": {"INR": "83"}}),
            "2026-05-30",
            "USD",
            "INR"
        ),
        Err(CurrencyExchangeSettingsError::NonNumericExchangeRate)
    );
    assert_eq!(settings.custom_hooks(), ["validate"]);
    assert_eq!(settings.doctype(), "Currency Exchange Settings");
    assert_eq!(settings.module(), "Accounts");
}
