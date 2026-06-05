use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::payment_reconciliation::payment_reconciliation::{
    get_queries_for_dimension_filters, PaymentReconciliation, PaymentReconciliationAllocation,
    PaymentReconciliationError, PaymentReconciliationInvoice, PaymentReconciliationPayment,
    ReconcileDrCrNotePlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_doc() -> PaymentReconciliation {
    PaymentReconciliation {
        company: Some("TC".to_string()),
        party_type: Some("Customer".to_string()),
        party: Some("CUST-001".to_string()),
        receivable_payable_account: Some("Debtors - TC".to_string()),
        cost_center: Some("Main - TC".to_string()),
        dimensions: vec!["department".to_string()],
        dimension_values: BTreeMap::from([("department".to_string(), "Sales".to_string())]),
        ..PaymentReconciliation::default()
    }
}

#[test]
fn payment_reconciliation_metadata_defaults_and_fetch_guards_match_erpnext() {
    let doc = PaymentReconciliation::load_from_db_defaults();
    assert_eq!(PaymentReconciliation::DOCTYPE, "Payment Reconciliation");
    assert_eq!(PaymentReconciliation::MODULE, "Accounts");
    assert!(PaymentReconciliation::ALLOW_COPY);
    assert_eq!(PaymentReconciliation::FIELD_ORDER.len(), 32);
    assert_eq!(PaymentReconciliation::FIELD_ORDER[0], "company");
    assert_eq!(PaymentReconciliation::FIELD_ORDER[31], "allocation");
    assert_eq!(doc.invoice_limit, 50);
    assert_eq!(doc.payment_limit, 50);
    assert_eq!(doc.doctype(), "Payment Reconciliation");
    assert_eq!(
        PaymentReconciliation::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .required(),
            FieldSpec::dynamic_link("party")
                .options("party_type")
                .required(),
            FieldSpec::link("receivable_payable_account", "Receivable / Payable Account")
                .options("Account")
                .required(),
            FieldSpec::table("payments", "Payments").options("Payment Reconciliation Payment"),
            FieldSpec::table("invoices", "Invoices").options("Payment Reconciliation Invoice"),
            FieldSpec::table("allocation", "Allocation")
                .options("Payment Reconciliation Allocation"),
        ]
    );

    let mut missing = PaymentReconciliation::default();
    assert_eq!(
        missing.check_mandatory_to_fetch().unwrap_err(),
        PaymentReconciliationError::Validation("Please select Company first".to_string())
    );
    missing.company = Some("TC".to_string());
    assert_eq!(
        missing.check_mandatory_to_fetch().unwrap_err(),
        PaymentReconciliationError::Validation("Please select Party Type first".to_string())
    );

    let doc = base_doc();
    assert!(doc.check_mandatory_to_fetch().is_ok());
    assert_eq!(
        doc.validate_entries().unwrap_err(),
        PaymentReconciliationError::Validation(
            "No records found in the Invoices table".to_string()
        )
    );
}

#[test]
fn payment_reconciliation_difference_allocation_and_payment_details_match_erpnext() {
    let doc = base_doc();
    let payment = PaymentReconciliationPayment {
        reference_type: "Payment Entry".to_string(),
        reference_name: "PE-0001".to_string(),
        reference_row: Some("PER-1".to_string()),
        amount: 150.0,
        unreconciled_amount: 150.0,
        exchange_rate: Some(1.2),
        currency: Some("USD".to_string()),
        cost_center: Some("Main - TC".to_string()),
        ..PaymentReconciliationPayment::default()
    };
    let invoice = PaymentReconciliationInvoice {
        invoice_type: "Sales Invoice".to_string(),
        invoice_number: "SINV-0001".to_string(),
        invoice_date: Some("2026-05-01".to_string()),
        amount: 120.0,
        outstanding_amount: 100.0,
        exchange_rate: Some(1.4),
        currency: Some("USD".to_string()),
    };

    assert_eq!(
        doc.get_difference_amount(&payment, &invoice, 100.0, "Receivable", "USD", "UZS", 2,),
        -20.0
    );

    let allocations = doc.allocate_entries(
        vec![payment.clone()],
        vec![invoice.clone()],
        &BTreeMap::from([("SINV-0001".to_string(), 1.4)]),
        Some("Exchange Gain/Loss - TC"),
        "Invoice",
        "2026-06-05",
        "Receivable",
        "USD",
        "UZS",
        2,
    );
    assert_eq!(
        allocations,
        vec![PaymentReconciliationAllocation {
            reference_type: "Payment Entry".to_string(),
            reference_name: "PE-0001".to_string(),
            reference_row: Some("PER-1".to_string()),
            invoice_type: "Sales Invoice".to_string(),
            invoice_number: "SINV-0001".to_string(),
            unreconciled_amount: 150.0,
            amount: 150.0,
            allocated_amount: 100.0,
            difference_amount: -20.0,
            difference_account: Some("Exchange Gain/Loss - TC".to_string()),
            gain_loss_posting_date: Some("2026-05-01".to_string()),
            exchange_rate: Some(1.4),
            currency: Some("USD".to_string()),
            cost_center: Some("Main - TC".to_string()),
            dimensions: BTreeMap::from([("department".to_string(), "Sales".to_string())]),
            ..PaymentReconciliationAllocation::default()
        }]
    );

    let details = doc.get_payment_details(&allocations[0], "credit_in_account_currency");
    assert_eq!(details.voucher_type, "Payment Entry");
    assert_eq!(details.against_voucher, "SINV-0001");
    assert_eq!(details.dr_or_cr, "credit_in_account_currency");
    assert_eq!(details.allocated_amount, 100.0);
    assert_eq!(
        details.dimensions.get("department").map(String::as_str),
        Some("Sales")
    );
}

#[test]
fn payment_reconciliation_allocation_validation_and_dimension_filters_match_erpnext() {
    let mut doc = base_doc();
    doc.invoices = vec![PaymentReconciliationInvoice {
        invoice_type: "Sales Invoice".to_string(),
        invoice_number: "SINV-0001".to_string(),
        outstanding_amount: 100.0,
        ..PaymentReconciliationInvoice::default()
    }];
    doc.payments = vec![PaymentReconciliationPayment {
        reference_type: "Payment Entry".to_string(),
        reference_name: "PE-0001".to_string(),
        amount: 80.0,
        ..PaymentReconciliationPayment::default()
    }];
    doc.allocation = vec![PaymentReconciliationAllocation {
        idx: 1,
        invoice_type: "Sales Invoice".to_string(),
        invoice_number: "SINV-0001".to_string(),
        amount: 80.0,
        allocated_amount: 90.0,
        ..PaymentReconciliationAllocation::default()
    }];
    assert_eq!(
        doc.validate_allocation().unwrap_err(),
        PaymentReconciliationError::Validation(
            "Row 1: Allocated amount 90 must be less than or equal to remaining payment amount 80"
                .to_string()
        )
    );
    doc.allocation[0].amount = 120.0;
    doc.allocation[0].allocated_amount = 101.0;
    assert_eq!(
        doc.validate_allocation().unwrap_err(),
        PaymentReconciliationError::Validation(
            "Row 1: Allocated amount 101 must be less than or equal to invoice outstanding amount 100"
                .to_string()
        )
    );
    doc.allocation[0].allocated_amount = 100.0;
    assert!(doc.validate_allocation().is_ok());

    assert_eq!(
        get_queries_for_dimension_filters(
            Some("TC"),
            &[("department".to_string(), "Department".to_string(), true)],
        ),
        vec![(
            "department".to_string(),
            BTreeMap::from([
                ("company".to_string(), "TC".to_string()),
                ("is_group".to_string(), "0".to_string()),
            ]),
        )]
    );
}

#[test]
fn payment_reconciliation_filter_plans_and_dr_cr_note_reconcile_match_erpnext() {
    let mut doc = base_doc();
    doc.from_invoice_date = Some("2026-01-01".to_string());
    doc.to_invoice_date = Some("2026-01-31".to_string());
    doc.from_payment_date = Some("2026-02-01".to_string());
    doc.to_payment_date = Some("2026-02-28".to_string());
    doc.minimum_payment_amount = Some(10.0);
    doc.maximum_payment_amount = Some(500.0);
    doc.payment_name = Some("PAY".to_string());
    doc.bank_cash_account = Some("Bank - TC".to_string());

    let invoice_filters = doc.build_qb_filter_conditions(true, false);
    assert_eq!(invoice_filters.common, vec!["company = 'TC'".to_string()]);
    assert_eq!(
        invoice_filters.posting_date,
        vec![
            "posting_date >= '2026-01-01'".to_string(),
            "posting_date <= '2026-01-31'".to_string(),
        ]
    );
    assert_eq!(
        invoice_filters.accounting_dimensions,
        vec![
            "cost_center = 'Main - TC'".to_string(),
            "department = 'Sales'".to_string(),
        ]
    );

    let journal_filters = doc.get_journal_filter_conditions();
    assert_eq!(
        journal_filters,
        vec![
            "Journal Entry.company = 'TC'".to_string(),
            "Journal Entry.posting_date >= '2026-02-01'".to_string(),
            "Journal Entry.posting_date <= '2026-02-28'".to_string(),
            "Journal Entry.total_debit >= 10".to_string(),
            "Journal Entry.total_debit <= 500".to_string(),
        ]
    );

    let mut note = PaymentReconciliationAllocation {
        reference_type: "Sales Invoice".to_string(),
        reference_name: "SINV-RET-0001".to_string(),
        invoice_type: "Sales Invoice".to_string(),
        invoice_number: "SINV-0001".to_string(),
        allocated_amount: 80.0,
        unreconciled_amount: -100.0,
        amount: -100.0,
        difference_amount: -5.0,
        difference_account: Some("Exchange Gain/Loss - TC".to_string()),
        gain_loss_posting_date: Some("2026-03-01".to_string()),
        exchange_rate: Some(1.0),
        currency: Some("USD".to_string()),
        cost_center: Some("Main - TC".to_string()),
        dimensions: BTreeMap::from([("department".to_string(), "Sales".to_string())]),
        ..PaymentReconciliationAllocation::default()
    };
    let plans = PaymentReconciliation::reconcile_dr_cr_note_plans(
        &mut [note.clone()],
        "TC",
        "USD",
        Some("Default CC - TC"),
        &BTreeMap::from([("SINV-RET-0001".to_string(), -100.0)]),
    )
    .unwrap();
    assert_eq!(
        plans,
        vec![ReconcileDrCrNotePlan {
            voucher_type: "Credit Note".to_string(),
            posting_date: "2026-06-05".to_string(),
            company: "TC".to_string(),
            multi_currency: false,
            debit_or_credit_account_field: "credit_in_account_currency".to_string(),
            reverse_dr_or_cr: "debit".to_string(),
            allocated_amount: 80.0,
            reference_type: "Sales Invoice".to_string(),
            reference_name: "SINV-0001".to_string(),
            note_reference_type: "Sales Invoice".to_string(),
            note_reference_name: "SINV-RET-0001".to_string(),
            cost_center: "Main - TC".to_string(),
            dimensions: BTreeMap::from([("department".to_string(), "Sales".to_string())]),
            gain_loss_dr_or_cr: Some("credit".to_string()),
            gain_loss_reverse_dr_or_cr: Some("debit".to_string()),
            difference_amount: -5.0,
        }]
    );

    note.allocated_amount = 120.0;
    assert_eq!(
        PaymentReconciliation::reconcile_dr_cr_note_plans(
            &mut [note],
            "TC",
            "USD",
            Some("Default CC - TC"),
            &BTreeMap::from([("SINV-RET-0001".to_string(), -100.0)]),
        )
        .unwrap_err(),
        PaymentReconciliationError::Validation(
            "Sales Invoice has been modified after you pulled it. Please pull it again."
                .to_string()
        )
    );
}
