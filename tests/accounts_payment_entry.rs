use tokio_erp::erpnext::accounts::doctype::payment_entry::payment_entry::{
    GlEntryPlan, PaymentEntry, PaymentEntryDeductionRow, PaymentEntryError,
    PaymentEntryReferenceRow,
};
use tokio_erp::erpnext::DocumentController;

fn receive_entry() -> PaymentEntry {
    PaymentEntry {
        name: Some("ACC-PAY-0001".to_string()),
        payment_type: "Receive".to_string(),
        posting_date: "2026-06-06".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        party_type: Some("Customer".to_string()),
        party: Some("_Test Customer".to_string()),
        paid_from: Some("Debtors - TC".to_string()),
        paid_from_account_currency: "USD".to_string(),
        paid_from_account_type: Some("Receivable".to_string()),
        paid_to: Some("Bank - TC".to_string()),
        paid_to_account_currency: "USD".to_string(),
        paid_to_account_type: Some("Bank".to_string()),
        paid_amount: 100.0,
        received_amount: 100.0,
        source_exchange_rate: 1.0,
        target_exchange_rate: 1.0,
        reference_no: Some("CHK-1".to_string()),
        reference_date: Some("2026-06-05".to_string()),
        references: vec![PaymentEntryReferenceRow {
            idx: 1,
            reference_doctype: "Sales Invoice".to_string(),
            reference_name: "SINV-0001".to_string(),
            total_amount: 150.0,
            outstanding_amount: 150.0,
            allocated_amount: 80.0,
            exchange_rate: Some(1.0),
            account: Some("Debtors - TC".to_string()),
            ..Default::default()
        }],
        deductions: vec![PaymentEntryDeductionRow {
            account: "Write Off - TC".to_string(),
            cost_center: Some("Main - TC".to_string()),
            amount: 20.0,
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[test]
fn payment_entry_metadata_matches_erpnext_json() {
    assert_eq!(PaymentEntry::DOCTYPE, "Payment Entry");
    assert_eq!(PaymentEntry::MODULE, "Accounts");
    assert_eq!(PaymentEntry::AUTONAME, "naming_series:");
    assert_eq!(PaymentEntry::TITLE_FIELD, "title");
    assert_eq!(PaymentEntry::SORT_FIELD, "creation");
    assert_eq!(PaymentEntry::SORT_ORDER, "DESC");
    assert!(PaymentEntry::IS_SUBMITTABLE);
    assert!(PaymentEntry::TRACK_CHANGES);
    assert_eq!(PaymentEntry::FIELD_ORDER.len(), 93);
    assert_eq!(
        &PaymentEntry::FIELD_ORDER[..12],
        [
            "type_of_payment",
            "naming_series",
            "payment_type",
            "column_break_5",
            "posting_date",
            "company",
            "cost_center",
            "mode_of_payment",
            "party_section",
            "party_type",
            "party",
            "party_name",
        ]
    );

    let doc = PaymentEntry::default();
    assert_eq!(doc.doctype(), "Payment Entry");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        [
            "validate",
            "before_save",
            "on_submit",
            "validate_for_repost",
            "on_update_after_submit",
            "on_cancel",
        ]
    );
}

#[test]
fn payment_entry_validate_core_receive_flow_matches_erpnext() {
    let mut doc = receive_entry();

    doc.validate().unwrap();

    assert_eq!(doc.party_account_field.as_deref(), Some("paid_from"));
    assert_eq!(doc.party_account.as_deref(), Some("Debtors - TC"));
    assert_eq!(doc.party_account_currency.as_deref(), Some("USD"));
    assert_eq!(doc.received_amount, 100.0);
    assert_eq!(doc.base_paid_amount, 100.0);
    assert_eq!(doc.base_received_amount, 100.0);
    assert_eq!(doc.total_allocated_amount, 80.0);
    assert_eq!(doc.base_total_allocated_amount, 80.0);
    assert_eq!(doc.unallocated_amount, 40.0);
    assert_eq!(doc.difference_amount, 0.0);
    assert_eq!(doc.title.as_deref(), Some("_Test Customer"));
    assert_eq!(doc.status, "Draft");
    assert_eq!(doc.base_in_words.as_deref(), Some("100 USD"));
    assert_eq!(doc.in_words.as_deref(), Some("100 USD"));
    assert!(doc
        .remarks
        .as_deref()
        .unwrap()
        .contains("Amount USD 80 against Sales Invoice SINV-0001"));
    assert!(doc
        .remarks
        .as_deref()
        .unwrap()
        .contains("Amount USD 20 deducted against Write Off - TC"));
}

#[test]
fn payment_entry_validate_core_errors_match_erpnext_guards() {
    let mut invalid_type = receive_entry();
    invalid_type.payment_type = "Refund".to_string();
    assert_eq!(
        invalid_type.validate(),
        Err(PaymentEntryError::InvalidPaymentType)
    );

    let mut missing_party = receive_entry();
    missing_party.party = None;
    assert_eq!(
        missing_party.validate(),
        Err(PaymentEntryError::PartyMandatory)
    );

    let mut missing_amount = receive_entry();
    missing_amount.paid_amount = 0.0;
    assert_eq!(
        missing_amount.validate(),
        Err(PaymentEntryError::MandatoryField("paid_amount"))
    );

    let mut negative_customer_receive = receive_entry();
    negative_customer_receive.references[0].allocated_amount = -10.0;
    assert_eq!(
        negative_customer_receive.validate(),
        Err(PaymentEntryError::CannotReceiveFromCustomerAgainstNegativeOutstanding)
    );

    let mut duplicate = receive_entry();
    duplicate.references.push(duplicate.references[0].clone());
    assert_eq!(
        duplicate.validate(),
        Err(PaymentEntryError::DuplicateReference {
            row: 2,
            reference_doctype: "Sales Invoice".to_string(),
            reference_name: "SINV-0001".to_string(),
        })
    );

    let mut bank_ref_missing = receive_entry();
    bank_ref_missing.reference_no = None;
    assert_eq!(
        bank_ref_missing.validate(),
        Err(PaymentEntryError::BankTransactionReferenceMandatory)
    );
}

#[test]
fn payment_entry_internal_transfer_clears_party_references_and_builds_bank_gl_like_erpnext() {
    let mut doc = PaymentEntry {
        payment_type: "Internal Transfer".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        paid_from: Some("Cash - TC".to_string()),
        paid_from_account_currency: "USD".to_string(),
        paid_from_account_type: Some("Cash".to_string()),
        paid_to: Some("Bank - TC".to_string()),
        paid_to_account_currency: "USD".to_string(),
        paid_to_account_type: Some("Bank".to_string()),
        paid_amount: 50.0,
        received_amount: 50.0,
        source_exchange_rate: 1.0,
        target_exchange_rate: 1.0,
        party: Some("Should clear".to_string()),
        references: vec![PaymentEntryReferenceRow {
            allocated_amount: 50.0,
            ..Default::default()
        }],
        ..Default::default()
    };

    doc.validate().unwrap();

    assert_eq!(doc.party, None);
    assert!(doc.references.is_empty());
    assert_eq!(doc.total_allocated_amount, 0.0);
    assert_eq!(doc.unallocated_amount, 0.0);
    assert_eq!(doc.difference_amount, 0.0);
    assert_eq!(doc.title.as_deref(), Some("Cash - TC - Bank - TC"));
    assert_eq!(
        doc.remarks.as_deref(),
        Some("Amount USD 50 transferred from Cash - TC to Bank - TC")
    );

    let gl = doc.build_gl_map();
    assert_eq!(gl.len(), 2);
    assert_eq!(
        gl[0],
        GlEntryPlan {
            account: "Cash - TC".to_string(),
            account_currency: "USD".to_string(),
            against: Some("Bank - TC".to_string()),
            credit_in_account_currency: 50.0,
            credit: 50.0,
            credit_in_transaction_currency: 50.0,
            cost_center: None,
            post_net_value: true,
            ..Default::default()
        }
    );
    assert_eq!(gl[1].account, "Bank - TC");
    assert_eq!(gl[1].debit_in_account_currency, 50.0);
    assert_eq!(gl[1].debit, 50.0);
}

#[test]
fn payment_entry_build_gl_map_matches_receive_party_bank_and_deductions() {
    let mut doc = receive_entry();
    doc.validate().unwrap();

    let gl = doc.build_gl_map();

    assert_eq!(gl.len(), 4);
    assert_eq!(
        gl[0],
        GlEntryPlan {
            account: "Debtors - TC".to_string(),
            party_type: Some("Customer".to_string()),
            party: Some("_Test Customer".to_string()),
            against: Some("Bank - TC".to_string()),
            account_currency: "USD".to_string(),
            credit_in_account_currency: 80.0,
            credit: 80.0,
            credit_in_transaction_currency: 80.0,
            against_voucher_type: Some("Sales Invoice".to_string()),
            against_voucher: Some("SINV-0001".to_string()),
            transaction_exchange_rate: 1.0,
            ..Default::default()
        }
    );
    assert_eq!(gl[1].account, "Debtors - TC");
    assert_eq!(gl[1].credit_in_account_currency, 40.0);
    assert_eq!(gl[1].credit, 40.0);
    assert_eq!(gl[2].account, "Bank - TC");
    assert_eq!(gl[2].debit_in_account_currency, 100.0);
    assert_eq!(gl[2].debit, 100.0);
    assert_eq!(gl[3].account, "Write Off - TC");
    assert_eq!(gl[3].debit_in_account_currency, 20.0);
    assert_eq!(gl[3].debit, 20.0);
}
