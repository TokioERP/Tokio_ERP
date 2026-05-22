use tokio_erp::erpnext::accounts::doctype::cashier_closing_payments::cashier_closing_payments::CashierClosingPayments;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn cashier_closing_payments_matches_erpnext_metadata() {
    assert_eq!(CashierClosingPayments::DOCTYPE, "Cashier Closing Payments");
    assert_eq!(CashierClosingPayments::MODULE, "Accounts");
    assert_eq!(
        CashierClosingPayments::FIELD_ORDER,
        ["mode_of_payment", "amount"]
    );
    assert!(CashierClosingPayments::IS_TABLE);
    assert!(CashierClosingPayments::QUICK_ENTRY);
    assert!(CashierClosingPayments::TRACK_CHANGES);

    assert_eq!(
        CashierClosingPayments::fields(),
        vec![
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view()
                .in_filter()
                .in_standard_filter(),
            FieldSpec::float("amount", "Amount")
                .default("0.00")
                .in_list_view(),
        ]
    );
}

#[test]
fn cashier_closing_payments_preserves_pass_controller_behavior() {
    let blank = CashierClosingPayments::default();
    assert_eq!(blank.mode_of_payment, None);
    assert_eq!(blank.amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CashierClosingPayments::new("Cash", "250.00");
    assert_eq!(row.mode_of_payment.as_deref(), Some("Cash"));
    assert_eq!(row.amount.as_deref(), Some("250.00"));
    assert_eq!(row.doctype(), "Cashier Closing Payments");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
