use tokio_erp::erpnext::accounts::doctype::payment_terms_template_detail::payment_terms_template_detail::PaymentTermsTemplateDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_terms_template_detail_matches_erpnext_metadata() {
    assert_eq!(
        PaymentTermsTemplateDetail::DOCTYPE,
        "Payment Terms Template Detail"
    );
    assert_eq!(PaymentTermsTemplateDetail::MODULE, "Accounts");
    assert_eq!(
        PaymentTermsTemplateDetail::FIELD_ORDER,
        [
            "payment_term",
            "section_break_13",
            "description",
            "section_break_4",
            "invoice_portion",
            "mode_of_payment",
            "column_break_3",
            "due_date_based_on",
            "credit_days",
            "credit_months",
            "section_break_8",
            "discount_type",
            "discount",
            "column_break_11",
            "discount_validity_based_on",
            "discount_validity",
        ]
    );
    assert!(PaymentTermsTemplateDetail::IS_TABLE);
    assert!(PaymentTermsTemplateDetail::EDITABLE_GRID);
    assert!(PaymentTermsTemplateDetail::INDEX_WEB_PAGES_FOR_SEARCH);

    assert_eq!(
        PaymentTermsTemplateDetail::fields(),
        vec![
            FieldSpec::link("payment_term", "Payment Term")
                .options("Payment Term")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_13").label("Description"),
            FieldSpec::small_text("description", "Description")
                .in_list_view()
                .columns(2),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::float("invoice_portion", "Invoice Portion (%)")
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::link("mode_of_payment", "Mode of Payment").options("Mode of Payment"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::select("due_date_based_on", "Due Date Based On")
                .options(
                    "Day(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .required()
                .in_list_view()
                .columns(2),
            FieldSpec::int("credit_days", "Credit Days")
                .in_list_view()
                .columns(2)
                .default("0")
                .depends_on(
                    "eval:in_list(['Day(s) after invoice date', 'Day(s) after the end of the invoice month'], doc.due_date_based_on)",
                ),
            FieldSpec::int("credit_months", "Credit Months")
                .default("0")
                .depends_on("eval:doc.due_date_based_on=='Month(s) after the end of the invoice month'"),
            FieldSpec::section_break("section_break_8").label("Discount Settings"),
            FieldSpec::select("discount_type", "Discount Type")
                .options("Percentage\nAmount")
                .default("Percentage"),
            FieldSpec::float("discount", "Discount"),
            FieldSpec::column_break("column_break_11"),
            FieldSpec::select("discount_validity_based_on", "Discount Validity Based On")
                .options(
                    "Day(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
                )
                .default("Day(s) after invoice date")
                .depends_on("discount"),
            FieldSpec::int("discount_validity", "Discount Validity").depends_on("discount"),
        ]
    );
}

#[test]
fn payment_terms_template_detail_preserves_pass_controller_behavior() {
    let blank = PaymentTermsTemplateDetail::default();
    assert_eq!(blank.payment_term, None);
    assert_eq!(blank.invoice_portion, None);
    assert_eq!(blank.due_date_based_on, None);
    assert!(blank.custom_hooks().is_empty());

    let term =
        PaymentTermsTemplateDetail::new(Some("NET 30"), "50", "Day(s) after invoice date", 1);
    assert_eq!(term.payment_term.as_deref(), Some("NET 30"));
    assert_eq!(term.invoice_portion.as_deref(), Some("50"));
    assert_eq!(
        term.due_date_based_on.as_deref(),
        Some("Day(s) after invoice date")
    );
    assert_eq!(term.idx, 1);
    assert_eq!(term.doctype(), "Payment Terms Template Detail");
    assert_eq!(term.module(), "Accounts");
    assert!(term.custom_hooks().is_empty());
}
