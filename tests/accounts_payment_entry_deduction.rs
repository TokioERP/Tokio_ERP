use tokio_erp::erpnext::accounts::doctype::payment_entry_deduction::payment_entry_deduction::PaymentEntryDeduction;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_entry_deduction_matches_erpnext_metadata() {
    assert_eq!(PaymentEntryDeduction::DOCTYPE, "Payment Entry Deduction");
    assert_eq!(PaymentEntryDeduction::MODULE, "Accounts");
    assert_eq!(
        PaymentEntryDeduction::FIELD_ORDER,
        [
            "account",
            "cost_center",
            "amount",
            "column_break_2",
            "is_exchange_gain_loss",
            "description",
        ]
    );
    assert!(PaymentEntryDeduction::IS_TABLE);
    assert!(PaymentEntryDeduction::EDITABLE_GRID);
    assert!(PaymentEntryDeduction::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(PaymentEntryDeduction::QUICK_ENTRY);
    assert_eq!(PaymentEntryDeduction::ROW_FORMAT, Some("Dynamic"));

    assert_eq!(
        PaymentEntryDeduction::fields(),
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .required()
                .allow_on_submit()
                .print_hide()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::check("is_exchange_gain_loss", "Is Exchange Gain / Loss?")
                .default("0")
                .depends_on("eval:doc.is_exchange_gain_loss")
                .read_only(),
            FieldSpec::small_text("description", "Description"),
        ]
    );
}

#[test]
fn payment_entry_deduction_preserves_pass_controller_behavior() {
    let blank = PaymentEntryDeduction::default();
    assert_eq!(blank.account, None);
    assert_eq!(blank.cost_center, None);
    assert_eq!(blank.amount, None);
    assert_eq!(blank.description, None);
    assert!(!blank.is_exchange_gain_loss);
    assert!(blank.custom_hooks().is_empty());

    let row = PaymentEntryDeduction::new("Cash - TC", "Main - TC", "125.00");
    assert_eq!(row.account.as_deref(), Some("Cash - TC"));
    assert_eq!(row.cost_center.as_deref(), Some("Main - TC"));
    assert_eq!(row.amount.as_deref(), Some("125.00"));
    assert_eq!(row.doctype(), "Payment Entry Deduction");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
