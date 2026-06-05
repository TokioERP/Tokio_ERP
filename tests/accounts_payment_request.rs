use tokio_erp::erpnext::accounts::doctype::payment_request::payment_request::{
    apply_payment_references, get_amount, get_dummy_message, get_open_payment_requests_query_plan,
    get_print_format_list, make_payment_order_plan, set_payment_references,
    update_payment_requests_as_per_pe_references, validate_payment, AvailablePaymentSchedulesPlan,
    CancelOldPaymentRequestsPlan, ExistingPaymentEntryQueryPlan,
    ExistingPaymentRequestAmountQueryPlan, IntegrationRequestStatusQueryPlan,
    OpenPaymentRequestsQueryPlan, PaymentEntryReferenceRow, PaymentEntryRequestPlan,
    PaymentEntrySourceDoc, PaymentOrderFromRequestPlan, PaymentReferenceRow, PaymentRequest,
    PaymentRequestAmountSource, PaymentRequestError, PaymentRequestLifecyclePlan,
    PaymentRequestUpdate, PaymentSubmitPlan, PaymentUrlPlan, PhonePaymentRequestPlan,
    SendEmailPlan, SubscriptionPlanInput, SubscriptionValidationPlan,
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

#[test]
fn payment_request_schedule_reference_helpers_match_erpnext() {
    let references = set_payment_references(
        r#"[
            {
                "payment_term": "50-50",
                "name": "SCH-001",
                "description": "Advance",
                "due_date": "2026-06-30",
                "payment_amount": 60
            },
            {
                "payment_term": "50-50",
                "name": "SCH-002",
                "description": "Balance",
                "due_date": "2026-07-31",
                "payment_amount": 40
            }
        ]"#,
    )
    .unwrap();
    assert_eq!(
        references,
        vec![
            PaymentReferenceRow {
                payment_term: Some("50-50".to_string()),
                payment_schedule: Some("SCH-001".to_string()),
                description: Some("Advance".to_string()),
                due_date: Some("2026-06-30".to_string()),
                amount: 60.0,
            },
            PaymentReferenceRow {
                payment_term: Some("50-50".to_string()),
                payment_schedule: Some("SCH-002".to_string()),
                description: Some("Balance".to_string()),
                due_date: Some("2026-07-31".to_string()),
                amount: 40.0,
            },
        ]
    );

    let (merged, grand_total) = apply_payment_references(
        vec![PaymentReferenceRow {
            payment_schedule: Some("SCH-001".to_string()),
            amount: 60.0,
            ..PaymentReferenceRow::default()
        }],
        references,
    );
    assert_eq!(merged.len(), 2);
    assert_eq!(grand_total, 100.0);
}

#[test]
fn payment_request_misc_plans_match_erpnext() {
    assert_eq!(
        get_print_format_list("Sales Invoice", &["POS".to_string(), "Compact".to_string()]),
        vec![
            "Standard".to_string(),
            "POS".to_string(),
            "Compact".to_string(),
        ]
    );
    assert!(get_dummy_message().contains("Make Payment"));

    assert_eq!(
        validate_payment("Payment Request", "PREQ-0001", Some("Paid")).unwrap_err(),
        PaymentRequestError::Validation(
            "The Payment Request PREQ-0001 is already paid, cannot process payment twice"
                .to_string()
        )
    );
    assert!(validate_payment("Sales Invoice", "SINV-0001", Some("Paid")).is_ok());

    assert_eq!(
        make_payment_order_plan(&PaymentRequest {
            name: Some("PREQ-0001".to_string()),
            reference_doctype: Some("Purchase Invoice".to_string()),
            reference_name: Some("PINV-0001".to_string()),
            grand_total: 250.0,
            party: Some("SUP-001".to_string()),
            mode_of_payment: Some("Wire Transfer".to_string()),
            bank_account: Some("BA-001".to_string()),
            account: Some("Creditors - TC".to_string()),
            ..PaymentRequest::default()
        }),
        PaymentOrderFromRequestPlan {
            payment_order_type: "Payment Request".to_string(),
            reference_doctype: "Purchase Invoice".to_string(),
            reference_name: "PINV-0001".to_string(),
            amount: 250.0,
            supplier: Some("SUP-001".to_string()),
            payment_request: "PREQ-0001".to_string(),
            mode_of_payment: Some("Wire Transfer".to_string()),
            bank_account: Some("BA-001".to_string()),
            account: Some("Creditors - TC".to_string()),
        }
    );

    assert_eq!(
        get_open_payment_requests_query_plan(
            "Payment Request",
            "REQ",
            "name",
            0,
            20,
            Some("Sales Invoice"),
            Some("SINV-0001"),
        ),
        Some(OpenPaymentRequestsQueryPlan {
            doctype: "Payment Request".to_string(),
            text_filter: Some("REQ".to_string()),
            searchfield: "name".to_string(),
            start: 0,
            page_len: 20,
            reference_doctype: "Sales Invoice".to_string(),
            reference_name: "SINV-0001".to_string(),
            order_by: "transaction_date ASC,creation ASC".to_string(),
        })
    );
    assert_eq!(
        get_open_payment_requests_query_plan("Payment Request", "", "name", 0, 20, None, None),
        None
    );
}

#[test]
fn payment_request_create_payment_entry_and_reference_allocation_match_erpnext() {
    let doc = PaymentRequest {
        name: Some("PREQ-0001".to_string()),
        reference_doctype: Some("Sales Invoice".to_string()),
        reference_name: Some("SINV-0001".to_string()),
        payment_request_type: "Inward".to_string(),
        payment_account: Some("Bank - TC".to_string()),
        mode_of_payment: Some("Credit Card".to_string()),
        outstanding_amount: 125.0,
        currency: Some("USD".to_string()),
        party_account_currency: Some("UZS".to_string()),
        cost_center: Some("Main - TC".to_string()),
        project: Some("PROJ-001".to_string()),
        ..PaymentRequest::default()
    };
    let source = PaymentEntrySourceDoc {
        doctype: "Sales Invoice".to_string(),
        name: "SINV-0001".to_string(),
        debit_to: Some("Debtors - TC".to_string()),
        company_currency: "UZS".to_string(),
        conversion_rate: 12_500.0,
        ..PaymentEntrySourceDoc::default()
    };

    assert_eq!(
        doc.create_payment_entry_plan(&source, true, 2),
        PaymentEntryRequestPlan {
            reference_doctype: "Sales Invoice".to_string(),
            reference_name: "SINV-0001".to_string(),
            party_account: "Debtors - TC".to_string(),
            party_account_currency: "UZS".to_string(),
            party_amount: 125.0,
            bank_account: Some("Bank - TC".to_string()),
            bank_amount: 0.01,
            mode_of_payment: Some("Credit Card".to_string()),
            reference_no: "PREQ-0001".to_string(),
            remarks: "Payment Entry against Sales Invoice SINV-0001 via Payment Request PREQ-0001"
                .to_string(),
            cost_center: Some("Main - TC".to_string()),
            project: Some("PROJ-001".to_string()),
            submit: true,
            created_from_payment_request: true,
            paid_amount_override: None,
        }
    );

    assert_eq!(
        PaymentRequest::allocate_payment_request_to_pe_references(
            "PREQ-0001",
            80.0,
            vec![
                PaymentEntryReferenceRow {
                    idx: 1,
                    allocated_amount: 50.0,
                    ..PaymentEntryReferenceRow::default()
                },
                PaymentEntryReferenceRow {
                    idx: 2,
                    allocated_amount: 50.0,
                    ..PaymentEntryReferenceRow::default()
                },
            ],
            2,
        ),
        vec![
            PaymentEntryReferenceRow {
                idx: 1,
                allocated_amount: 50.0,
                payment_request: Some("PREQ-0001".to_string()),
            },
            PaymentEntryReferenceRow {
                idx: 2,
                allocated_amount: 30.0,
                payment_request: Some("PREQ-0001".to_string()),
            },
            PaymentEntryReferenceRow {
                idx: 3,
                allocated_amount: 20.0,
                payment_request: None,
            },
        ]
    );
}

#[test]
fn payment_request_validation_gateway_and_email_plans_match_erpnext() {
    let doc = PaymentRequest {
        name: Some("PREQ-0001".to_string()),
        reference_doctype: Some("Sales Invoice".to_string()),
        reference_name: Some("SINV-0001".to_string()),
        grand_total: 100.0,
        payment_account: Some("Gateway - TC".to_string()),
        payment_gateway: Some("Stripe".to_string()),
        currency: Some("USD".to_string()),
        email_to: Some("customer@example.com".to_string()),
        phone_number: Some("+998901234567".to_string()),
        subject: Some("Payment Request for SINV-0001".to_string()),
        print_format: Some("Standard".to_string()),
        ..PaymentRequest::default()
    };

    assert_eq!(
        PaymentRequest::validate_payment_request_amount_plan(0.0, false, 100.0, 0.0, 2, 2)
            .unwrap_err(),
        PaymentRequestError::Validation("Grand Total cannot be zero".to_string())
    );
    assert_eq!(
        PaymentRequest::validate_payment_request_amount_plan(75.0, false, 100.0, 30.0, 2, 2)
            .unwrap_err(),
        PaymentRequestError::Validation(
            "Total Payment Request amount cannot be greater than Sales Invoice amount".to_string()
        )
    );
    assert!(
        PaymentRequest::validate_payment_request_amount_plan(70.0, false, 100.0, 30.0, 2, 2)
            .is_ok()
    );

    assert_eq!(
        PaymentRequest::validate_currency(Some("Gateway - TC"), Some("USD"), Some("UZS"))
            .unwrap_err(),
        PaymentRequestError::Validation(
            "Transaction currency must be same as Payment Gateway currency".to_string()
        )
    );

    assert_eq!(
        doc.validate_subscription_details_plan(
            true,
            &[
                SubscriptionPlanInput {
                    name: "ROW-1".to_string(),
                    plan: "PLAN-1".to_string(),
                    qty: 2.0,
                    payment_gateway: Some("Stripe".to_string()),
                    rate: 40.0,
                },
                SubscriptionPlanInput {
                    name: "ROW-2".to_string(),
                    plan: "PLAN-2".to_string(),
                    qty: 1.0,
                    payment_gateway: Some("Stripe".to_string()),
                    rate: 20.0,
                },
            ],
        )
        .unwrap(),
        SubscriptionValidationPlan {
            calculated_amount: 100.0,
            grand_total: 100.0,
            warning: None,
        }
    );

    assert_eq!(
        doc.request_phone_payment_plan().unwrap(),
        PhonePaymentRequestPlan {
            reference_doctype: "Payment Request".to_string(),
            reference_docname: "PREQ-0001".to_string(),
            payment_reference: "SINV-0001".to_string(),
            request_amount: 100.0,
            sender: Some("customer@example.com".to_string()),
            currency: Some("USD".to_string()),
            payment_gateway: Some("Stripe".to_string()),
            phone_number: Some("+998901234567".to_string()),
        }
    );

    assert_eq!(
        doc.get_payment_url_plan(
            Some(("TC", Some("Alice Customer"))),
            Some("fallback@example.com"),
            2
        ),
        PaymentUrlPlan {
            amount: 100.0,
            title: "TC".to_string(),
            description: Some("Payment Request for SINV-0001".to_string()),
            reference_doctype: "Payment Request".to_string(),
            reference_docname: "PREQ-0001".to_string(),
            payer_email: "customer@example.com".to_string(),
            payer_name: Some("Alice Customer".to_string()),
            order_id: "PREQ-0001".to_string(),
            currency: Some("USD".to_string()),
            payment_gateway: Some("Stripe".to_string()),
        }
    );

    assert_eq!(
        doc.send_email_plan(),
        SendEmailPlan {
            recipients: Some("customer@example.com".to_string()),
            sender: None,
            subject: Some("Payment Request for SINV-0001".to_string()),
            message: None,
            reference_doctype: Some("Sales Invoice".to_string()),
            reference_name: Some("SINV-0001".to_string()),
            print_format: Some("Standard".to_string()),
            queue: "short".to_string(),
            timeout: 300,
            enqueue_after_commit: true,
        }
    );
}

#[test]
fn payment_request_query_and_lifecycle_plans_match_erpnext() {
    assert_eq!(
        PaymentRequest::get_existing_payment_entry_plan("SINV-0001"),
        ExistingPaymentEntryQueryPlan {
            reference_name: "SINV-0001".to_string(),
            payment_entry_docstatus_lt: 2,
            limit: 1,
        }
    );
    assert_eq!(
        PaymentRequest::get_irequest_status_plan(&["PREQ-0001".to_string()]),
        IntegrationRequestStatusQueryPlan {
            reference_doctype: "Payment Request".to_string(),
            reference_docnames: vec!["PREQ-0001".to_string()],
            statuses: vec!["Authorized".to_string(), "Completed".to_string()],
        }
    );
    assert_eq!(
        PaymentRequest::get_existing_payment_request_amount_plan(
            "Sales Invoice",
            "SINV-0001",
            Some(vec!["Initiated".to_string(), "Paid".to_string()]),
            "USD",
            "UZS",
            12_500.0,
        ),
        ExistingPaymentRequestAmountQueryPlan {
            reference_doctype: "Sales Invoice".to_string(),
            reference_name: "SINV-0001".to_string(),
            docstatus: 1,
            statuses: Some(vec!["Initiated".to_string(), "Paid".to_string()]),
            convert_to_transaction_currency: true,
            conversion_rate: 12_500.0,
        }
    );

    assert_eq!(
        PaymentRequest::cancel_old_payment_requests_plan(
            "Sales Order",
            "SO-0001",
            vec!["PREQ-0001".to_string(), "PREQ-0002".to_string()],
            false,
        )
        .unwrap(),
        CancelOldPaymentRequestsPlan {
            reference_doctype: "Sales Order".to_string(),
            reference_name: "SO-0001".to_string(),
            candidate_statuses: vec!["Draft".to_string(), "Requested".to_string()],
            cancel_payment_requests: vec!["PREQ-0001".to_string(), "PREQ-0002".to_string()],
            cancel_queued_integration_requests: true,
        }
    );
    assert_eq!(
        PaymentRequest::cancel_old_payment_requests_plan(
            "Sales Order",
            "SO-0001",
            vec!["PREQ-0001".to_string()],
            true,
        )
        .unwrap_err(),
        PaymentRequestError::Validation("Another Payment Request is already processed".to_string())
    );

    assert_eq!(
        PaymentRequest::get_available_payment_schedules_plan(
            true,
            false,
            vec!["SCH-001".to_string(), "SCH-002".to_string()],
            vec!["SCH-001".to_string()],
        ),
        AvailablePaymentSchedulesPlan {
            has_payment_schedule: true,
            has_existing_payment_entry: false,
            available_payment_schedules: vec!["SCH-002".to_string()],
        }
    );

    let doc = PaymentRequest {
        reference_doctype: Some("Sales Order".to_string()),
        reference_name: Some("SO-0001".to_string()),
        status: "Paid".to_string(),
        payment_channel: "Phone".to_string(),
        ..PaymentRequest::default()
    };
    assert_eq!(
        doc.on_discard_plan(),
        PaymentRequestLifecyclePlan {
            status_update: Some(("status".to_string(), "Cancelled".to_string())),
            check_payment_entry_exists: false,
            update_reference_advance_payment_status: false,
            create_payment_entry: false,
            make_invoice: false,
        }
    );
    assert_eq!(
        doc.on_cancel_plan(),
        PaymentRequestLifecyclePlan {
            status_update: Some(("status".to_string(), "Cancelled".to_string())),
            check_payment_entry_exists: true,
            update_reference_advance_payment_status: true,
            create_payment_entry: false,
            make_invoice: false,
        }
    );
    assert_eq!(
        doc.set_as_paid_plan(false),
        PaymentRequestLifecyclePlan {
            status_update: Some(("status".to_string(), "Paid:0".to_string())),
            check_payment_entry_exists: false,
            update_reference_advance_payment_status: false,
            create_payment_entry: false,
            make_invoice: false,
        }
    );
}
