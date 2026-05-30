use serde_json::Value;

use crate::erpnext::accounts::doctype::currency_exchange_settings_details::currency_exchange_settings_details::CurrencyExchangeSettingsDetails;
use crate::erpnext::accounts::doctype::currency_exchange_settings_result::currency_exchange_settings_result::CurrencyExchangeSettingsResult;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CurrencyExchangeSettings {
    pub disabled: bool,
    pub service_provider: Option<String>,
    pub api_endpoint: Option<String>,
    pub use_http: bool,
    pub access_key: Option<String>,
    pub url: Option<String>,
    pub req_params: Vec<CurrencyExchangeSettingsDetails>,
    pub result_key: Vec<CurrencyExchangeSettingsResult>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrencyExchangeRequestPlan {
    pub api_url: String,
    pub params: Vec<(String, String)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CurrencyExchangeSettingsError {
    AccessKeyRequired { service_provider: String },
    InvalidResultKey { response_text: String },
    NonNumericExchangeRate,
}

impl CurrencyExchangeSettings {
    pub const DOCTYPE: &'static str = "Currency Exchange Settings";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 13] = [
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
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const HELP_HTML: &'static str = "<h3>Currency Exchange Settings Help</h3>\n<p>There are 3 variables that could be used within the endpoint, result key and in values of the parameter.</p>\n<p>Exchange rate between {from_currency} and {to_currency} on {transaction_date} is fetched by the API.</p>\n<p>Example: If your endpoint is exchange.com/2021-08-01, then, you will have to input exchange.com/{transaction_date}</p>";
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SINGLE: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("api_details_section").label("API Details"),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::select("service_provider", "Service Provider")
                .options("frankfurter.dev\nexchangerate.host\nCustom")
                .required(),
            FieldSpec::data("api_endpoint", "API Endpoint")
                .in_list_view()
                .read_only_depends_on("eval: doc.service_provider != \"Custom\"")
                .required(),
            FieldSpec::check("use_http", "Use HTTP Protocol")
                .default("0")
                .depends_on("eval: doc.service_provider != \"Custom\""),
            FieldSpec::data("access_key", "Access Key")
                .depends_on("eval:doc.service_provider == 'exchangerate.host';"),
            FieldSpec::data("url", "Example URL").read_only(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::html("help", "Help").options(Self::HELP_HTML),
            FieldSpec::section_break("section_break_2").label("Request Parameters"),
            FieldSpec::table("req_params", "Parameters")
                .options("Currency Exchange Settings Details")
                .read_only_depends_on("eval: doc.service_provider != \"Custom\"")
                .required(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::table("result_key", "Result Key")
                .options("Currency Exchange Settings Result")
                .read_only_depends_on("eval: doc.service_provider != \"Custom\"")
                .required(),
        ]
    }

    pub fn set_parameters_and_result(&mut self) -> Result<(), CurrencyExchangeSettingsError> {
        match self.service_provider.as_deref() {
            Some("exchangerate.host") => {
                let access_key = self.access_key.clone().ok_or_else(|| {
                    CurrencyExchangeSettingsError::AccessKeyRequired {
                        service_provider: "exchangerate.host".to_string(),
                    }
                })?;
                self.result_key.clear();
                self.req_params.clear();
                self.api_endpoint =
                    get_api_endpoint(self.service_provider.as_deref(), self.use_http);
                self.result_key
                    .push(CurrencyExchangeSettingsResult::new("result"));
                self.req_params.push(CurrencyExchangeSettingsDetails::new(
                    "access_key",
                    access_key,
                ));
                self.req_params
                    .push(CurrencyExchangeSettingsDetails::new("amount", "1"));
                self.req_params.push(CurrencyExchangeSettingsDetails::new(
                    "date",
                    "{transaction_date}",
                ));
                self.req_params.push(CurrencyExchangeSettingsDetails::new(
                    "from",
                    "{from_currency}",
                ));
                self.req_params
                    .push(CurrencyExchangeSettingsDetails::new("to", "{to_currency}"));
            }
            Some("frankfurter.dev" | "frankfurter.app") => {
                self.result_key.clear();
                self.req_params.clear();
                self.api_endpoint =
                    get_api_endpoint(self.service_provider.as_deref(), self.use_http);
                self.result_key
                    .push(CurrencyExchangeSettingsResult::new("rates"));
                self.result_key
                    .push(CurrencyExchangeSettingsResult::new("{to_currency}"));
                self.req_params.push(CurrencyExchangeSettingsDetails::new(
                    "base",
                    "{from_currency}",
                ));
                self.req_params.push(CurrencyExchangeSettingsDetails::new(
                    "symbols",
                    "{to_currency}",
                ));
            }
            _ => {}
        }
        Ok(())
    }

    pub fn validate_parameters_plan(
        &self,
        transaction_date: &str,
        from_currency: &str,
        to_currency: &str,
    ) -> CurrencyExchangeRequestPlan {
        let api_url = replace_currency_placeholders(
            self.api_endpoint.as_deref().unwrap_or_default(),
            transaction_date,
            from_currency,
            to_currency,
        );
        let params = self
            .req_params
            .iter()
            .map(|row| {
                (
                    row.key.clone().unwrap_or_default(),
                    replace_currency_placeholders(
                        row.value.as_deref().unwrap_or_default(),
                        transaction_date,
                        from_currency,
                        to_currency,
                    ),
                )
            })
            .collect();
        CurrencyExchangeRequestPlan { api_url, params }
    }

    pub fn validate_result(
        &mut self,
        response_url: &str,
        response_text: &str,
        value: &Value,
        transaction_date: &str,
        from_currency: &str,
        to_currency: &str,
    ) -> Result<f64, CurrencyExchangeSettingsError> {
        let mut current = value;
        for key in &self.result_key {
            let key = replace_currency_placeholders(
                key.key.as_deref().unwrap_or_default(),
                transaction_date,
                from_currency,
                to_currency,
            );
            current = current.get(&key).ok_or_else(|| {
                CurrencyExchangeSettingsError::InvalidResultKey {
                    response_text: response_text.to_string(),
                }
            })?;
        }

        let Some(rate) = current.as_f64() else {
            return Err(CurrencyExchangeSettingsError::NonNumericExchangeRate);
        };
        self.url = Some(response_url.to_string());
        Ok(rate)
    }
}

impl DocumentController for CurrencyExchangeSettings {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

pub fn get_api_endpoint(service_provider: Option<&str>, use_http: bool) -> Option<String> {
    let api = match service_provider {
        Some("exchangerate.host") => "api.exchangerate.host/convert",
        Some("frankfurter.app") => "api.frankfurter.app/{transaction_date}",
        Some("frankfurter.dev") => "api.frankfurter.dev/v1/{transaction_date}",
        _ => return None,
    };
    let protocol = if use_http { "http://" } else { "https://" };
    Some(format!("{protocol}{api}"))
}

fn replace_currency_placeholders(
    value: &str,
    transaction_date: &str,
    from_currency: &str,
    to_currency: &str,
) -> String {
    value
        .replace("{transaction_date}", transaction_date)
        .replace("{from_currency}", from_currency)
        .replace("{to_currency}", to_currency)
}
