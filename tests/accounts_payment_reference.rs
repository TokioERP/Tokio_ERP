use tokio_erp::erpnext::accounts::doctype::payment_reference::payment_reference::PaymentReference;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_reference_matches_erpnext_metadata() {
    assert_eq!(PaymentReference::DOCTYPE, "Payment Reference");
    assert_eq!(PaymentReference::MODULE, "Accounts");
    assert_eq!(
        PaymentReference::FIELD_ORDER,
        [
            "payment_term",
            "column_break_lnjp",
            "payment_schedule",
            "section_break_fjhh",
            "description",
            "section_break_mjlv",
            "due_date",
            "column_break_qghl",
            "amount",
        ]
    );
    assert!(PaymentReference::IS_TABLE);
    assert!(PaymentReference::ALLOW_RENAME);
    assert!(PaymentReference::EDITABLE_GRID);
    assert!(PaymentReference::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(PaymentReference::GRID_PAGE_LENGTH, Some(50));
    assert_eq!(PaymentReference::ROW_FORMAT, Some("Dynamic"));
    assert_eq!(PaymentReference::ROWS_THRESHOLD_FOR_GRID_SEARCH, Some(20));

    assert_eq!(
        PaymentReference::fields(),
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .in_list_view(),
            FieldSpec::column_break("column_break_lnjp"),
            FieldSpec::link("payment_schedule", "Payment Schedule")
                .options("Payment Schedule")
                .allow_on_submit()
                .read_only(),
            FieldSpec::section_break("section_break_fjhh")
                .label("Description")
                .collapsible(),
            FieldSpec::small_text("description", "Description").in_list_view(),
            FieldSpec::section_break("section_break_mjlv"),
            FieldSpec::date("due_date", "Due Date").in_list_view(),
            FieldSpec::column_break("column_break_qghl"),
            FieldSpec::currency("amount", "Amount")
                .precision("2")
                .in_list_view(),
        ]
    );
}

#[test]
fn payment_reference_preserves_pass_controller_behavior() {
    let blank = PaymentReference::default();
    assert_eq!(blank.payment_term, None);
    assert_eq!(blank.payment_schedule, None);
    assert_eq!(blank.description, None);
    assert_eq!(blank.due_date, None);
    assert_eq!(blank.amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PaymentReference::new("NET 30");
    assert_eq!(row.payment_term.as_deref(), Some("NET 30"));
    assert_eq!(row.doctype(), "Payment Reference");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
