use tokio_erp::erpnext::accounts::doctype::payment_gateway_account::payment_gateway_account::{
    PaymentGatewayAccount, PaymentGatewayDefaultUpdate,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_gateway_account_matches_erpnext_metadata() {
    assert_eq!(PaymentGatewayAccount::DOCTYPE, "Payment Gateway Account");
    assert_eq!(PaymentGatewayAccount::MODULE, "Accounts");
    assert_eq!(
        PaymentGatewayAccount::FIELD_ORDER,
        [
            "payment_gateway",
            "payment_channel",
            "company",
            "is_default",
            "column_break_4",
            "payment_account",
            "currency",
            "payment_request_message",
            "message",
            "message_examples",
        ]
    );
    assert!(PaymentGatewayAccount::EDITABLE_GRID);
    assert!(PaymentGatewayAccount::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(PaymentGatewayAccount::ROW_FORMAT, "Dynamic");
    assert_eq!(PaymentGatewayAccount::SORT_FIELD, "creation");
    assert_eq!(PaymentGatewayAccount::SORT_ORDER, "DESC");

    assert_eq!(
        PaymentGatewayAccount::fields(),
        vec![
            FieldSpec::link("payment_gateway", "Payment Gateway")
                .options("Payment Gateway")
                .in_list_view()
                .required(),
            FieldSpec::check("is_default", "Is Default").default("0"),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("payment_account", "Payment Account")
                .options("Account")
                .in_list_view()
                .required(),
            FieldSpec::read_only_field("currency", "Currency")
                .fetch_from("payment_account.account_currency"),
            FieldSpec::section_break("payment_request_message")
                .depends_on("eval: doc.payment_channel == 'Email' || (!doc.payment_channel)"),
            FieldSpec::small_text("message", "Default Payment Request Message")
                .default("Please click on the link below to make your payment"),
            FieldSpec::html("message_examples", "Message Examples")
                .options(PaymentGatewayAccount::MESSAGE_EXAMPLES),
            FieldSpec::select("payment_channel", "Payment Channel")
                .options("\nEmail\nPhone\nOther")
                .default("Email"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .print_hide()
                .required(),
        ]
    );
}

#[test]
fn payment_gateway_account_autoname_and_validation_match_erpnext() {
    let mut doc = PaymentGatewayAccount {
        name: Some("Stripe - USD - OLD".to_string()),
        payment_gateway: Some("Stripe".to_string()),
        company: Some("Acme LLC".to_string()),
        payment_account: Some("Stripe Receivable".to_string()),
        currency: Some("EUR".to_string()),
        is_default: true,
        ..Default::default()
    };

    assert_eq!(doc.custom_hooks(), ["autoname", "validate"]);
    assert_eq!(doc.autoname_with_company_abbr("AC"), "Stripe - EUR - AC");
    assert_eq!(doc.name.as_deref(), Some("Stripe - EUR - AC"));

    let plan = doc.validate_with_account_currency("USD", false);
    assert_eq!(doc.currency.as_deref(), Some("USD"));
    assert!(doc.is_default);
    assert_eq!(
        plan.unset_other_default,
        Some(PaymentGatewayDefaultUpdate {
            doctype: "Payment Gateway Account",
            company: Some("Acme LLC".to_string()),
            current_name: Some("Stripe - EUR - AC".to_string()),
            fieldname: "is_default",
            value: false,
        })
    );

    let mut non_default = PaymentGatewayAccount {
        name: Some("PayPal - USD - AC".to_string()),
        company: Some("Acme LLC".to_string()),
        is_default: false,
        ..Default::default()
    };
    let plan = non_default.validate_with_account_currency("USD", true);
    assert_eq!(non_default.currency.as_deref(), Some("USD"));
    assert!(!non_default.is_default);
    assert_eq!(plan.unset_other_default, None);

    non_default.set_as_default_if_not_set(false);
    assert!(non_default.is_default);
}
