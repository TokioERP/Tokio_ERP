use tokio_erp::erpnext::accounts::doctype::payment_terms_template::payment_terms_template::{
    PaymentTermsTemplate, ValidationError,
};
use tokio_erp::erpnext::accounts::doctype::payment_terms_template::payment_terms_template_dashboard::{
    get_data, DashboardData, DashboardTransaction,
};
use tokio_erp::erpnext::accounts::doctype::payment_terms_template_detail::payment_terms_template_detail::PaymentTermsTemplateDetail;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_terms_template_matches_erpnext_metadata() {
    assert_eq!(PaymentTermsTemplate::DOCTYPE, "Payment Terms Template");
    assert_eq!(PaymentTermsTemplate::MODULE, "Accounts");
    assert_eq!(PaymentTermsTemplate::AUTONAME, Some("field:template_name"));
    assert_eq!(
        PaymentTermsTemplate::FIELD_ORDER,
        [
            "template_name",
            "allocate_payment_based_on_payment_terms",
            "terms",
        ]
    );
    assert!(PaymentTermsTemplate::ALLOW_RENAME);
    assert!(PaymentTermsTemplate::EDITABLE_GRID);

    assert_eq!(
        PaymentTermsTemplate::fields(),
        vec![
            FieldSpec::data("template_name", "Template Name").unique(),
            FieldSpec::check(
                "allocate_payment_based_on_payment_terms",
                "Allocate Payment Based On Payment Terms",
            )
            .default("0"),
            FieldSpec::table("terms", "Payment Terms")
                .options("Payment Terms Template Detail")
                .required(),
        ]
    );
}

#[test]
fn payment_terms_template_dashboard_matches_erpnext_get_data() {
    assert_eq!(
        get_data(),
        DashboardData {
            fieldname: "payment_terms_template",
            non_standard_fieldnames: vec![
                ("Customer Group", "payment_terms"),
                ("Supplier Group", "payment_terms"),
                ("Supplier", "payment_terms"),
                ("Customer", "payment_terms"),
            ],
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
                    label: Some("Party"),
                    items: vec!["Customer", "Supplier"],
                },
                DashboardTransaction {
                    label: Some("Group"),
                    items: vec!["Customer Group", "Supplier Group"],
                },
            ],
        }
    );
}

#[test]
fn payment_terms_template_validate_requires_invoice_portion_total_to_equal_100() {
    let template = PaymentTermsTemplate {
        terms: vec![
            PaymentTermsTemplateDetail::new(Some("NET 30"), "40", "Day(s) after invoice date", 1),
            PaymentTermsTemplateDetail::new(Some("NET 60"), "50", "Day(s) after invoice date", 2),
        ],
        ..PaymentTermsTemplate::default()
    };

    assert_eq!(
        template.validate(),
        Err(ValidationError {
            message: "Combined invoice portion must equal 100%".to_string(),
            indicator: Some("red"),
        })
    );
}

#[test]
fn payment_terms_template_validate_requires_payment_term_when_allocating_by_terms() {
    let template = PaymentTermsTemplate {
        allocate_payment_based_on_payment_terms: true,
        terms: vec![PaymentTermsTemplateDetail::new(
            None::<String>,
            "100",
            "Day(s) after invoice date",
            1,
        )],
        ..PaymentTermsTemplate::default()
    };

    assert_eq!(
        template.validate(),
        Err(ValidationError {
            message: "Row 1: Payment Term is mandatory".to_string(),
            indicator: None,
        })
    );
}

#[test]
fn payment_terms_template_validate_rejects_duplicate_payment_term_tuple() {
    let mut first =
        PaymentTermsTemplateDetail::new(Some("NET 30"), "50", "Day(s) after invoice date", 1);
    first.credit_days = Some("30".to_string());
    let mut second =
        PaymentTermsTemplateDetail::new(Some("NET 30"), "50", "Day(s) after invoice date", 2);
    second.credit_days = Some("30".to_string());

    let template = PaymentTermsTemplate {
        terms: vec![first, second],
        ..PaymentTermsTemplate::default()
    };

    assert_eq!(
        template.validate(),
        Err(ValidationError {
            message: "The Payment Term at row 2 is possibly a duplicate.".to_string(),
            indicator: Some("red"),
        })
    );
}

#[test]
fn payment_terms_template_validate_accepts_valid_terms() {
    let template = PaymentTermsTemplate {
        template_name: Some("Standard".to_string()),
        terms: vec![
            PaymentTermsTemplateDetail::new(Some("NET 30"), "50", "Day(s) after invoice date", 1),
            PaymentTermsTemplateDetail::new(Some("NET 60"), "50", "Day(s) after invoice date", 2),
        ],
        ..PaymentTermsTemplate::default()
    };

    assert_eq!(template.validate(), Ok(()));
    assert_eq!(template.doctype(), "Payment Terms Template");
    assert_eq!(template.module(), "Accounts");
    assert!(template.custom_hooks().is_empty());
}
