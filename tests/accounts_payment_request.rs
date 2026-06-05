use tokio_erp::erpnext::accounts::doctype::payment_request::payment_request::{
    get_amount, update_payment_requests_as_per_pe_references, PaymentReferenceRow, PaymentRequest,
    PaymentRequestAmountSource, PaymentRequestError, PaymentRequestUpdate, PaymentSubmitPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_request_metadata_and_reference_validation_match_erpnext() {
    assert_eq!(PaymentRequest::DOCTYPE, "Payment Request");
    assert_eq!(PaymentRequest::MODULE, "Accounts");
    assert_eq!(
        PaymentRequest::ALLOWED_DOCTYPES_FOR_PAYMENT_REQUEST,
        [
            "Sales Order",
            "Purchase Order",
            "Sales Invoice",
            "Purchase Invoice",
            "POS Invoice",
            "Fees",
        ]
    );

    let mut doc = PaymentRequest {
        name: Some("PREQ-0001".to_string()),
        reference_doctype: Some("Sales Invoice".to_string()),
        reference_name: Some("SINV-0001".to_string()),
        grand_total: 100.0,
        payment_reference: vec![
            PaymentReferenceRow {
                payment_schedule: Some("SCH-001".to_string()),
                amount: 60.0,
                ..PaymentReferenceRow::default()
            },
            PaymentReferenceRow {
                payment_schedule: Some("SCH-002".to_string()),
                amount: 40.0,
                ..PaymentReferenceRow::default()
            },
        ],
        ..PaymentRequest::default()
    };

    assert_eq!(doc.doctype(), "Payment Request");
    assert_eq!(
        PaymentRequest::fields(),
        vec![
            FieldSpec::data("naming_series", "Series").default("ACC-PRQ-.YYYY.-"),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::dynamic_link("reference_name").options("reference_doctype"),
            FieldSpec::currency("grand_total", "Grand Total"),
            FieldSpec::currency("outstanding_amount", "Outstanding Amount"),
            FieldSpec::select("payment_request_type", "Payment Request Type")
                .options("\nOutward\nInward"),
            FieldSpec::select("status", "Status")
                .options("\nDraft\nRequested\nInitiated\nPartially Paid\nPayment Ordered\nPaid\nFailed\nCancelled"),
            FieldSpec::table("payment_reference", "Payment Reference").options("Payment Reference"),
        ]
    );
    assert!(doc.validate_reference_document().is_ok());
    assert!(doc.validate_against_payment_reference(2).is_ok());

    doc.payment_reference[1].payment_schedule = Some("SCH-001".to_string());
    assert_eq!(
        doc.validate_against_payment_reference(2).unwrap_err(),
        PaymentRequestError::Validation("Duplicate Payment Schedule selected".to_string())
    );
    doc.payment_reference[1].payment_schedule = Some("SCH-002".to_string());
    doc.payment_reference[1].amount = 39.99;
    assert_eq!(
        doc.validate_against_payment_reference(2).unwrap_err(),
        PaymentRequestError::Validation(
            "Grand Total must match sum of Payment References".to_string()
        )
    );

    let missing_reference = PaymentRequest::default();
    assert_eq!(
        missing_reference.validate_reference_document().unwrap_err(),
        PaymentRequestError::Validation(
            "To create a Payment Request reference document is required".to_string()
        )
    );
}

#[test]
fn payment_request_amount_and_submit_plans_match_erpnext() {
    let mut doc = PaymentRequest {
        name: Some("PREQ-0001".to_string()),
        company: Some("TC".to_string()),
        currency: Some("USD".to_string()),
        party_account_currency: Some("UZS".to_string()),
        payment_request_type: "Inward".to_string(),
        payment_channel: "Email".to_string(),
        grand_total: 100.0,
        mute_email: false,
        flags_mute_email: false,
        payment_account: Some("Gateway - TC".to_string()),
        payment_gateway: Some("Stripe".to_string()),
        ..PaymentRequest::default()
    };

    assert_eq!(doc.get_request_amount(&[]).unwrap(), 100.0);
    assert_eq!(
        doc.get_request_amount(&[r#"{"request_amount": 25}"#, r#"{"request_amount": 75.5}"#,])
            .unwrap(),
        100.5
    );

    assert_eq!(
        doc.before_submit_plan(
            "UZS",
            Some((Some(100.0), None, Some(1_250_000.0), None)),
            true,
        ),
        PaymentSubmitPlan {
            outstanding_amount: 1_250_000.0,
            status: "Requested".to_string(),
            request_phone_payment: false,
            set_payment_request_url: true,
            send_email: true,
            make_communication_entry: true,
        }
    );

    doc.payment_channel = "Phone".to_string();
    assert_eq!(
        doc.before_submit_plan("UZS", None, true),
        PaymentSubmitPlan {
            outstanding_amount: 100.0,
            status: "Requested".to_string(),
            request_phone_payment: true,
            set_payment_request_url: false,
            send_email: false,
            make_communication_entry: false,
        }
    );

    doc.payment_request_type = "Outward".to_string();
    assert_eq!(
        doc.before_submit_plan("UZS", None, true).status,
        "Initiated"
    );
}

#[test]
fn payment_request_get_amount_and_status_updates_match_erpnext() {
    assert_eq!(
        get_amount(
            &PaymentRequestAmountSource {
                doctype: "Sales Order".to_string(),
                rounded_total: Some(1000.0),
                advance_paid: 250.0,
                party_account_currency: Some("USD".to_string()),
                currency: Some("USD".to_string()),
                ..PaymentRequestAmountSource::default()
            },
            None,
            2
        ),
        750.0
    );
    assert_eq!(
        get_amount(
            &PaymentRequestAmountSource {
                doctype: "Sales Invoice".to_string(),
                outstanding_amount: 500.0,
                party_account_currency: Some("UZS".to_string()),
                currency: Some("USD".to_string()),
                conversion_rate: 12_500.0,
                ..PaymentRequestAmountSource::default()
            },
            None,
            2
        ),
        0.04
    );
    assert_eq!(
        get_amount(
            &PaymentRequestAmountSource {
                doctype: "Fees".to_string(),
                outstanding_amount: 120.0,
                ..PaymentRequestAmountSource::default()
            },
            None,
            2
        ),
        120.0
    );

    assert_eq!(
        update_payment_requests_as_per_pe_references(
            &[
                PaymentRequestUpdate {
                    payment_request: "PREQ-0001".to_string(),
                    grand_total: 100.0,
                    outstanding_amount: 100.0,
                    payment_request_type: "Inward".to_string(),
                    allocated_amount: 60.0,
                },
                PaymentRequestUpdate {
                    payment_request: "PREQ-0001".to_string(),
                    grand_total: 100.0,
                    outstanding_amount: 40.0,
                    payment_request_type: "Inward".to_string(),
                    allocated_amount: 40.0,
                },
            ],
            false,
            2,
        )
        .unwrap(),
        vec![("PREQ-0001".to_string(), 0.0, "Paid".to_string(),)]
    );

    assert_eq!(
        update_payment_requests_as_per_pe_references(
            &[PaymentRequestUpdate {
                payment_request: "PREQ-0002".to_string(),
                grand_total: 100.0,
                outstanding_amount: 50.0,
                payment_request_type: "Outward".to_string(),
                allocated_amount: 20.0,
            }],
            true,
            2,
        )
        .unwrap(),
        vec![("PREQ-0002".to_string(), 70.0, "Partially Paid".to_string(),)]
    );
}
