use tokio_erp::erpnext::accounts::doctype::payment_term::payment_term::PaymentTerm;
use tokio_erp::erpnext::accounts::doctype::payment_term::payment_term_dashboard::{
    get_data, DashboardData, DashboardTransaction,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_term_matches_erpnext_metadata() {
    assert_eq!(PaymentTerm::DOCTYPE, "Payment Term");
    assert_eq!(PaymentTerm::MODULE, "Accounts");
    assert_eq!(PaymentTerm::AUTONAME, Some("field:payment_term_name"));
    assert_eq!(
        PaymentTerm::FIELD_ORDER,
        [
            "payment_term_name",
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
            "section_break_6",
            "description",
        ]
    );
    assert!(PaymentTerm::ALLOW_RENAME);
    assert!(PaymentTerm::EDITABLE_GRID);

    assert_eq!(
        PaymentTerm::fields(),
        vec![
            FieldSpec::data("payment_term_name", "Payment Term Name").unique(),
            FieldSpec::float("invoice_portion", "Invoice Portion (%)"),
            FieldSpec::link("mode_of_payment", "Mode of Payment").options("Mode of Payment"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::select("due_date_based_on", "Due Date Based On").options(
                "Day(s) after invoice date\nDay(s) after the end of the invoice month\nMonth(s) after the end of the invoice month",
            ),
            FieldSpec::int("credit_days", "Credit Days").depends_on(
                "eval:in_list(['Day(s) after invoice date', 'Day(s) after the end of the invoice month'], doc.due_date_based_on)",
            ),
            FieldSpec::int("credit_months", "Credit Months")
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
            FieldSpec::section_break("section_break_6"),
            FieldSpec::small_text("description", "Description"),
        ]
    );
}

#[test]
fn payment_term_preserves_pass_controller_behavior() {
    let blank = PaymentTerm::default();
    assert_eq!(blank.payment_term_name, None);
    assert_eq!(blank.invoice_portion, None);
    assert_eq!(blank.mode_of_payment, None);
    assert_eq!(blank.discount_type, None);
    assert!(blank.custom_hooks().is_empty());

    let term = PaymentTerm::new("NET 30");
    assert_eq!(term.payment_term_name.as_deref(), Some("NET 30"));
    assert_eq!(term.doctype(), "Payment Term");
    assert_eq!(term.module(), "Accounts");
    assert!(term.custom_hooks().is_empty());
}

#[test]
fn payment_term_dashboard_matches_erpnext_get_data() {
    assert_eq!(
        get_data(),
        DashboardData {
            fieldname: "payment_term",
            transactions: vec![
                DashboardTransaction {
                    label: Some("Sales"),
                    items: vec!["Sales Invoice", "Sales Order", "Quotation"],
                },
                DashboardTransaction {
                    label: Some("Purchase"),
                    items: vec!["Purchase Invoice", "Purchase Order"],
                },
                DashboardTransaction {
                    label: None,
                    items: vec!["Payment Terms Template"],
                },
            ],
        }
    );
}
