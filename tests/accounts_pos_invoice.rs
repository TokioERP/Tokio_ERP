use tokio_erp::erpnext::accounts::doctype::pos_invoice::pos_invoice::{
    PosInvoice, PosInvoiceError, PosInvoicePayment,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_invoice() -> PosInvoice {
    PosInvoice {
        name: Some("POS-0001".to_string()),
        company: Some("TC".to_string()),
        customer: Some("CUST-001".to_string()),
        is_pos: true,
        grand_total: 100.0,
        rounded_total: Some(100.0),
        base_grand_total: 1_250_000.0,
        base_rounded_total: Some(1_250_000.0),
        conversion_rate: 12_500.0,
        paid_amount: 100.0,
        base_paid_amount: 1_250_000.0,
        docstatus: 1,
        due_date: Some("2026-06-05".to_string()),
        posting_date: Some("2026-06-05".to_string()),
        payments: vec![PosInvoicePayment {
            idx: 1,
            mode_of_payment: Some("Cash".to_string()),
            payment_type: "Cash".to_string(),
            account: Some("Cash - TC".to_string()),
            amount: 100.0,
        }],
        ..PosInvoice::default()
    }
}

#[test]
fn pos_invoice_metadata_and_basic_validate_guards_match_erpnext() {
    assert_eq!(PosInvoice::DOCTYPE, "POS Invoice");
    assert_eq!(PosInvoice::MODULE, "Accounts");
    assert_eq!(PosInvoice::AUTONAME, "naming_series:");
    assert!(PosInvoice::IS_SUBMITTABLE);
    assert_eq!(PosInvoice::FIELD_ORDER[0], "naming_series");

    let doc = base_invoice();
    assert_eq!(doc.doctype(), "POS Invoice");
    assert_eq!(
        PosInvoice::fields(),
        vec![
            FieldSpec::data("naming_series", "Series").default("ACC-PSINV-.YYYY.-"),
            FieldSpec::link("customer", "Customer").options("Customer"),
            FieldSpec::check("is_pos", "Include Payment").default("1"),
            FieldSpec::table("items", "Items").options("POS Invoice Item"),
            FieldSpec::table("payments", "Payments").options("Sales Invoice Payment"),
            FieldSpec::select("status", "Status")
                .options("\nDraft\nReturn\nCredit Note Issued\nConsolidated\nSubmitted\nPaid\nPartly Paid\nUnpaid\nPartly Paid and Discounted\nUnpaid and Discounted\nOverdue and Discounted\nOverdue\nCancelled"),
        ]
    );
    assert!(doc.validate_basic().is_ok());

    let missing_customer = PosInvoice {
        is_pos: true,
        ..PosInvoice::default()
    };
    assert_eq!(
        missing_customer.validate_basic().unwrap_err(),
        PosInvoiceError::Validation("Please select Customer first".to_string())
    );
    let not_pos = PosInvoice {
        customer: Some("CUST-001".to_string()),
        is_pos: false,
        ..PosInvoice::default()
    };
    assert_eq!(
        not_pos.validate_basic().unwrap_err(),
        PosInvoiceError::Validation(
            "POS Invoice should have the field Include Payment checked.".to_string()
        )
    );
}

#[test]
fn pos_invoice_payment_change_company_and_outstanding_rules_match_erpnext() {
    let mut doc = base_invoice();
    assert!(doc.validate_mode_of_payment().is_ok());
    assert!(doc.validate_payment_amount(2).is_ok());

    doc.payments[0].amount = -1.0;
    assert_eq!(
        doc.validate_payment_amount(2).unwrap_err(),
        PosInvoiceError::Validation("Row #1 (Payment Table): Amount must be positive".to_string())
    );
    doc.is_return = true;
    doc.payments[0].amount = 1.0;
    assert_eq!(
        doc.validate_payment_amount(2).unwrap_err(),
        PosInvoiceError::Validation("Row #1 (Payment Table): Amount must be negative".to_string())
    );

    let empty_payments = PosInvoice {
        customer: Some("CUST-001".to_string()),
        is_pos: true,
        ..PosInvoice::default()
    };
    assert_eq!(
        empty_payments.validate_mode_of_payment().unwrap_err(),
        PosInvoiceError::Validation(
            "At least one mode of payment is required for POS invoice.".to_string()
        )
    );

    let mut change = base_invoice();
    change.paid_amount = 120.0;
    change.base_paid_amount = 1_500_000.0;
    change.write_off_amount = 5.0;
    change.base_write_off_amount = 62_500.0;
    change.validate_change_amount();
    assert_eq!(change.change_amount, 25.0);
    assert_eq!(change.base_change_amount, 312_500.0);
    assert_eq!(
        change.validate_change_account(None).unwrap_err(),
        PosInvoiceError::Validation("Please enter Account for Change Amount".to_string())
    );
    assert!(change
        .validate_company_with_pos_company(Some("TC"), Some("TC"))
        .is_ok());
    assert_eq!(
        change
            .validate_company_with_pos_company(Some("TC"), Some("Other"))
            .unwrap_err(),
        PosInvoiceError::Validation(
            "Company TC does not match with POS Profile Company Other".to_string()
        )
    );

    let mut outstanding = base_invoice();
    outstanding.paid_amount = 40.0;
    outstanding.set_outstanding_amount();
    assert_eq!(outstanding.outstanding_amount, 60.0);
}

#[test]
fn pos_invoice_status_lifecycle_and_phone_payment_rules_match_erpnext() {
    let mut doc = base_invoice();
    doc.outstanding_amount = 0.0;
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Paid");

    doc.outstanding_amount = 25.0;
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Partly Paid");
    doc.due_date = Some("2026-06-01".to_string());
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Overdue");
    doc.consolidated_invoice = Some("SINV-0001".to_string());
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Consolidated");
    doc.docstatus = 2;
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Cancelled");

    assert_eq!(doc.before_submit_plan(), "set_outstanding_amount");
    assert_eq!(
        doc.before_cancel_plan(true, Some("POS-CLOSE-0001")).unwrap_err(),
        PosInvoiceError::Validation(
            "You need to cancel POS Closing Entry POS-CLOSE-0001 to be able to cancel this document."
                .to_string()
        )
    );
    assert_eq!(
        doc.on_cancel_plan(),
        vec![
            "ignore_linked:Payment Ledger Entry,Serial and Batch Bundle".to_string(),
            "sales_invoice_on_cancel".to_string(),
            "set_status:Cancelled".to_string(),
            "delink_serial_and_batch_bundle".to_string(),
        ]
    );

    let mut phone = base_invoice();
    phone.payments[0].payment_type = "Phone".to_string();
    phone.payments[0].amount = 50.0;
    assert_eq!(
        phone.check_phone_payments(Some(40.0)).unwrap_err(),
        PosInvoiceError::Validation("Payment related to Cash is not completed".to_string())
    );
}
