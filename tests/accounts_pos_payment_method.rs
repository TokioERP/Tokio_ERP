use tokio_erp::erpnext::accounts::doctype::pos_payment_method::pos_payment_method::PosPaymentMethod;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_payment_method_matches_erpnext_metadata() {
    assert_eq!(PosPaymentMethod::DOCTYPE, "POS Payment Method");
    assert_eq!(PosPaymentMethod::MODULE, "Accounts");
    assert_eq!(
        PosPaymentMethod::FIELD_ORDER,
        ["default", "allow_in_returns", "mode_of_payment"]
    );
    assert!(PosPaymentMethod::IS_TABLE);
    assert!(PosPaymentMethod::EDITABLE_GRID);
    assert!(PosPaymentMethod::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        PosPaymentMethod::fields(),
        vec![
            FieldSpec::check("default", "Default")
                .in_list_view()
                .default("0")
                .depends_on("eval:parent.doctype == 'POS Profile'"),
            FieldSpec::check("allow_in_returns", "Allow In Returns")
                .in_list_view()
                .default("0"),
            FieldSpec::link("mode_of_payment", "Mode of Payment")
                .options("Mode of Payment")
                .required()
                .in_list_view(),
        ]
    );
}

#[test]
fn pos_payment_method_preserves_pass_controller_behavior() {
    let blank = PosPaymentMethod::default();
    assert!(!blank.default);
    assert!(!blank.allow_in_returns);
    assert_eq!(blank.mode_of_payment, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosPaymentMethod::new("Cash");
    assert_eq!(row.mode_of_payment.as_deref(), Some("Cash"));
    assert!(!row.default);
    assert!(!row.allow_in_returns);
    assert_eq!(row.doctype(), "POS Payment Method");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
