use tokio_erp::erpnext::accounts::doctype::payment_schedule::payment_schedule::PaymentSchedule;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_schedule_matches_erpnext_metadata() {
    assert_eq!(PaymentSchedule::DOCTYPE, "Payment Schedule");
    assert_eq!(PaymentSchedule::MODULE, "Accounts");
    assert_eq!(
        PaymentSchedule::FIELD_ORDER,
        [
            "payment_term",
            "section_break_15",
            "description",
            "section_break_4",
            "due_date",
            "invoice_portion",
            "mode_of_payment",
            "column_break_5",
            "due_date_based_on",
            "credit_days",
            "credit_months",
            "section_break_6",
            "discount_date",
            "discount",
            "discount_type",
            "column_break_9",
            "discount_validity_based_on",
            "discount_validity",
            "section_break_9",
            "payment_amount",
            "outstanding",
            "paid_amount",
            "discounted_amount",
            "column_break_3",
            "base_payment_amount",
            "base_outstanding",
            "base_paid_amount",
        ]
    );
    assert!(PaymentSchedule::IS_TABLE);
    assert!(PaymentSchedule::EDITABLE_GRID);
    assert!(PaymentSchedule::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(PaymentSchedule::ROW_FORMAT, Some("Dynamic"));

    assert_eq!(
        PaymentSchedule::fields(),
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_15").label("Description"),
            FieldSpec::small_text("description", "Description")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::date("due_date", "Due Date")
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::percent("invoice_portion", "Invoice Portion")
                .in_list_view()
                .columns(2),
            FieldSpec::link("mode_of_payment", "Mode of Payment").options("Mode of Payment"),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::select("due_date_based_on", "Due Date Based On")
                .options(
                    "\nDay(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .read_only(),
            FieldSpec::int("credit_days", "Credit Days")
                .read_only()
                .depends_on(
                    "eval:in_list(['Day(s) after invoice date', 'Day(s) after the end of the invoice month'], doc.due_date_based_on)",
                ),
            FieldSpec::int("credit_months", "Credit Months")
                .read_only()
                .depends_on("eval:doc.due_date_based_on=='Month(s) after the end of the invoice month'"),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::date("discount_date", "Discount Date").depends_on("discount"),
            FieldSpec::float("discount", "Discount"),
            FieldSpec::select("discount_type", "Discount Type")
                .options("Percentage\nAmount")
                .default("Percentage"),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::select("discount_validity_based_on", "Discount Validity Based On")
                .options(
                    "\nDay(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .read_only()
                .depends_on("discount"),
            FieldSpec::int("discount_validity", "Discount Validity")
                .read_only()
                .depends_on("discount_validity_based_on"),
            FieldSpec::section_break("section_break_9"),
            FieldSpec::currency("payment_amount", "Payment Amount")
                .options("currency")
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::currency("outstanding", "Outstanding")
                .options("currency")
                .read_only(),
            FieldSpec::currency("paid_amount", "Paid Amount")
                .options("currency")
                .depends_on("paid_amount"),
            FieldSpec::currency("discounted_amount", "Discounted Amount")
                .read_only()
                .default("0")
                .depends_on("discounted_amount"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::currency("base_payment_amount", "Payment Amount (Company Currency)")
                .options("Company:company:default_currency"),
            FieldSpec::currency("base_outstanding", "Outstanding (Company Currency)")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::currency("base_paid_amount", "Paid Amount (Company Currency)")
                .options("Company:company:default_currency")
                .read_only()
                .depends_on("base_paid_amount"),
        ]
    );
}

#[test]
fn payment_schedule_preserves_pass_controller_behavior() {
    let blank = PaymentSchedule::default();
    assert_eq!(blank.payment_term, None);
    assert_eq!(blank.description, None);
    assert_eq!(blank.due_date, None);
    assert_eq!(blank.payment_amount, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PaymentSchedule::new("NET 30", "2026-05-22", "100.00");
    assert_eq!(row.payment_term.as_deref(), Some("NET 30"));
    assert_eq!(row.due_date.as_deref(), Some("2026-05-22"));
    assert_eq!(row.payment_amount.as_deref(), Some("100.00"));
    assert_eq!(row.doctype(), "Payment Schedule");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
