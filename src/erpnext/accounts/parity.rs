use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::erpnext::accounts::doctype::journal_entry::journal_entry::{
    AccountMeta, JournalEntry, JournalEntryAccountRow, PartyTypeMeta, ReferenceDoc,
};
use crate::erpnext::accounts::doctype::payment_entry::payment_entry::{
    GlEntryPlan, PaymentEntry, PaymentEntryDeductionRow, PaymentEntryError,
    PaymentEntryReferenceRow, SupplierBlockStatus as PaymentEntrySupplierBlockStatus,
};
use crate::erpnext::accounts::doctype::pricing_rule::pricing_rule::{
    apply_price_discount_rule, PricingItemDetails, PricingRule, PricingRuleArgs, PricingRuleChild,
};
use crate::erpnext::accounts::doctype::purchase_invoice::purchase_invoice::{
    PurchaseInvoice, PurchaseInvoiceError, SupplierBlockStatus,
};
use crate::erpnext::accounts::doctype::sales_invoice::sales_invoice::SalesInvoice;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountsGoldenScenario {
    PaymentEntryReceiveGl,
    PurchaseInvoiceSupplierHold,
    SalesInvoiceCancelledStatus,
    PricingRuleStackedDiscounts,
    JournalEntryValidateCore,
    SalesInvoiceStatusMatrix,
    PurchaseInvoiceStatusMatrix,
    PaymentEntryHoldGuards,
}

pub fn accounts_golden_snapshot(scenario: AccountsGoldenScenario) -> Value {
    match scenario {
        AccountsGoldenScenario::PaymentEntryReceiveGl => payment_entry_receive_gl_snapshot(),
        AccountsGoldenScenario::PurchaseInvoiceSupplierHold => {
            purchase_invoice_supplier_hold_snapshot()
        }
        AccountsGoldenScenario::SalesInvoiceCancelledStatus => {
            sales_invoice_cancelled_status_snapshot()
        }
        AccountsGoldenScenario::PricingRuleStackedDiscounts => {
            pricing_rule_stacked_discounts_snapshot()
        }
        AccountsGoldenScenario::JournalEntryValidateCore => journal_entry_validate_core_snapshot(),
        AccountsGoldenScenario::SalesInvoiceStatusMatrix => sales_invoice_status_matrix_snapshot(),
        AccountsGoldenScenario::PurchaseInvoiceStatusMatrix => {
            purchase_invoice_status_matrix_snapshot()
        }
        AccountsGoldenScenario::PaymentEntryHoldGuards => payment_entry_hold_guards_snapshot(),
    }
}

fn payment_entry_receive_gl_snapshot() -> Value {
    let mut doc = PaymentEntry {
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
    };
    doc.validate().unwrap();

    json!({
        "difference_amount": doc.difference_amount,
        "gl_entries": doc
            .build_gl_map()
            .into_iter()
            .map(gl_entry_snapshot)
            .collect::<Vec<_>>(),
        "status": doc.status,
        "total_allocated_amount": doc.total_allocated_amount,
        "unallocated_amount": doc.unallocated_amount,
    })
}

fn payment_entry_hold_guards_snapshot() -> Value {
    let mut supplier_blocked = supplier_payment_entry();
    supplier_blocked.supplier_block_status = Some(PaymentEntrySupplierBlockStatus {
        on_hold: true,
        hold_type: Some("Payments".to_string()),
        release_date: Some("2026-06-06".to_string()),
    });

    let mut released_supplier_payment = supplier_payment_entry();
    released_supplier_payment.supplier_block_status = Some(PaymentEntrySupplierBlockStatus {
        on_hold: true,
        hold_type: Some("Payments".to_string()),
        release_date: Some("2018-03-01".to_string()),
    });

    let mut reference_on_hold = supplier_payment_entry();
    reference_on_hold.references[0].reference_name = "PINV-HOLD-0001".to_string();
    reference_on_hold.references[0].on_hold = true;

    json!({
        "released_supplier_payment": match released_supplier_payment.validate() {
            Ok(()) => json!({ "status": released_supplier_payment.status }),
            Err(error) => payment_entry_error_snapshot(error),
        },
        "supplier_blocked": match supplier_blocked.validate() {
            Ok(()) => json!({ "status": supplier_blocked.status }),
            Err(error) => payment_entry_error_snapshot(error),
        },
        "reference_on_hold": match reference_on_hold.validate() {
            Ok(()) => json!({ "status": reference_on_hold.status }),
            Err(error) => payment_entry_error_snapshot(error),
        },
    })
}

fn supplier_payment_entry() -> PaymentEntry {
    PaymentEntry {
        payment_type: "Pay".to_string(),
        posting_date: "2026-06-06".to_string(),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        party_type: Some("Supplier".to_string()),
        party: Some("_Test Supplier".to_string()),
        paid_from: Some("Bank - TC".to_string()),
        paid_from_account_currency: "USD".to_string(),
        paid_from_account_type: Some("Bank".to_string()),
        paid_to: Some("Creditors - TC".to_string()),
        paid_to_account_currency: "USD".to_string(),
        paid_to_account_type: Some("Payable".to_string()),
        paid_amount: 100.0,
        received_amount: 100.0,
        source_exchange_rate: 1.0,
        target_exchange_rate: 1.0,
        reference_no: Some("CHK-SUP-1".to_string()),
        reference_date: Some("2026-06-06".to_string()),
        references: vec![PaymentEntryReferenceRow {
            idx: 1,
            reference_doctype: "Purchase Invoice".to_string(),
            reference_name: "PINV-0001".to_string(),
            total_amount: 100.0,
            outstanding_amount: 100.0,
            allocated_amount: 100.0,
            exchange_rate: Some(1.0),
            account: Some("Creditors - TC".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    }
}

fn payment_entry_error_snapshot(error: PaymentEntryError) -> Value {
    match error {
        PaymentEntryError::SupplierBlocked { supplier } => {
            json!({ "error": { "supplier": supplier, "type": "SupplierBlocked" } })
        }
        PaymentEntryError::ReferenceDocumentOnHold {
            reference_doctype,
            reference_name,
        } => json!({
            "error": {
                "reference_doctype": reference_doctype,
                "reference_name": reference_name,
                "type": "ReferenceDocumentOnHold"
            }
        }),
        other => json!({ "error": { "type": format!("{other:?}") } }),
    }
}

fn purchase_invoice_supplier_hold_snapshot() -> Value {
    let mut doc = PurchaseInvoice {
        supplier: Some("_Test Supplier".to_string()),
        supplier_block_status: Some(SupplierBlockStatus {
            on_hold: true,
            hold_type: Some("Invoices".to_string()),
            release_date: None,
        }),
        ..Default::default()
    };

    match doc.validate_core("2026-06-06", 2, &Default::default()) {
        Ok(()) => json!({ "ok": true }),
        Err(PurchaseInvoiceError::SupplierBlocked { supplier }) => {
            json!({ "error": { "supplier": supplier, "type": "SupplierBlocked" } })
        }
        Err(error) => json!({ "error": { "type": format!("{error:?}") } }),
    }
}

fn sales_invoice_cancelled_status_snapshot() -> Value {
    let mut doc = SalesInvoice {
        docstatus: 2,
        internal_transfer: true,
        status: "Internal Transfer".to_string(),
        ..Default::default()
    };

    json!({ "status": doc.set_status(None, "2026-06-06", None) })
}

fn sales_invoice_status_matrix_snapshot() -> Value {
    let mut doc = SalesInvoice {
        docstatus: 1,
        grand_total: 100.0,
        rounded_total: Some(100.0),
        base_grand_total: 500.0,
        base_rounded_total: Some(500.0),
        outstanding_amount: 0.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };

    let mut row = Map::new();
    insert_string(&mut row, "paid", doc.set_status(None, "2026-06-06", None));
    doc.outstanding_amount = 40.0;
    insert_string(
        &mut row,
        "partly_paid",
        doc.set_status(None, "2026-06-06", None),
    );
    doc.is_discounted = true;
    insert_string(
        &mut row,
        "discounted_partly_paid",
        doc.set_status(None, "2026-06-06", Some("Disbursed")),
    );
    doc.due_date = Some("2026-06-01".to_string());
    insert_string(
        &mut row,
        "discounted_overdue",
        doc.set_status(None, "2026-06-06", Some("Disbursed")),
    );
    doc.is_discounted = false;
    insert_string(
        &mut row,
        "overdue",
        doc.set_status(None, "2026-06-06", None),
    );
    doc.outstanding_amount = 120.0;
    doc.due_date = Some("2026-06-06".to_string());
    insert_string(&mut row, "unpaid", doc.set_status(None, "2026-06-06", None));

    doc.outstanding_amount = 0.0;
    doc.is_return = 1;
    insert_string(&mut row, "return", doc.set_status(None, "2026-06-06", None));
    doc.is_return = 0;
    doc.has_submitted_credit_note = true;
    insert_string(
        &mut row,
        "credit_note_issued",
        doc.set_status(None, "2026-06-06", None),
    );

    doc.has_submitted_credit_note = false;
    doc.internal_transfer = true;
    insert_string(
        &mut row,
        "internal_transfer",
        doc.set_status(None, "2026-06-06", None),
    );
    doc.docstatus = 2;
    doc.set_status(None, "2026-06-06", None);
    insert_string(
        &mut row,
        "cancelled_ignores_forced_status",
        doc.set_status(Some("Force Skipped"), "2026-06-06", None),
    );
    doc.docstatus = 0;
    insert_string(&mut row, "draft", doc.set_status(None, "2026-06-06", None));

    let mut amended_new = SalesInvoice {
        is_new: true,
        amended_from: Some("SINV-OLD".to_string()),
        status: "Submitted".to_string(),
        ..Default::default()
    };
    insert_string(
        &mut row,
        "amended_new",
        amended_new.set_status(None, "2026-06-06", None),
    );

    let mut base_total_doc = SalesInvoice {
        docstatus: 1,
        rounded_total: Some(100.0),
        base_rounded_total: Some(500.0),
        outstanding_amount: 250.0,
        currency: "USD".to_string(),
        party_account_currency: Some("EUR".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    insert_string(
        &mut row,
        "base_total_party_currency",
        base_total_doc.set_status(None, "2026-06-06", None),
    );

    let mut zero_rounded_total = SalesInvoice {
        docstatus: 1,
        rounded_total: Some(0.0),
        grand_total: 100.0,
        outstanding_amount: 40.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    insert_string(
        &mut row,
        "zero_rounded_total",
        zero_rounded_total.set_status(None, "2026-06-06", None),
    );

    Value::Object(row)
}

fn purchase_invoice_status_matrix_snapshot() -> Value {
    let mut doc = PurchaseInvoice {
        rounded_total: Some(100.0),
        base_grand_total: 500.0,
        base_rounded_total: Some(500.0),
        outstanding_amount: 0.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        docstatus: 1,
        ..Default::default()
    };

    let mut row = Map::new();
    insert_string(&mut row, "paid", doc.set_status(None, "2026-06-06"));
    doc.outstanding_amount = 40.0;
    insert_string(&mut row, "partly_paid", doc.set_status(None, "2026-06-06"));
    doc.due_date = Some("2026-06-01".to_string());
    insert_string(&mut row, "overdue", doc.set_status(None, "2026-06-06"));
    doc.outstanding_amount = 120.0;
    doc.due_date = Some("2026-06-06".to_string());
    insert_string(&mut row, "unpaid", doc.set_status(None, "2026-06-06"));

    doc.outstanding_amount = 0.0;
    doc.is_return = 1;
    insert_string(&mut row, "return", doc.set_status(None, "2026-06-06"));
    doc.is_return = 0;
    doc.has_submitted_debit_note = true;
    insert_string(
        &mut row,
        "debit_note_issued",
        doc.set_status(None, "2026-06-06"),
    );

    doc.has_submitted_debit_note = false;
    doc.internal_transfer = true;
    insert_string(
        &mut row,
        "internal_transfer",
        doc.set_status(None, "2026-06-06"),
    );
    doc.docstatus = 2;
    doc.set_status(None, "2026-06-06");
    insert_string(
        &mut row,
        "cancelled_ignores_forced_status",
        doc.set_status(Some("Force Skipped"), "2026-06-06"),
    );
    doc.docstatus = 0;
    insert_string(&mut row, "draft", doc.set_status(None, "2026-06-06"));

    let mut amended_new = PurchaseInvoice {
        is_new: true,
        amended_from: Some("PINV-OLD".to_string()),
        status: "Submitted".to_string(),
        ..Default::default()
    };
    insert_string(
        &mut row,
        "amended_new",
        amended_new.set_status(None, "2026-06-06"),
    );

    let mut base_total_doc = PurchaseInvoice {
        docstatus: 1,
        rounded_total: Some(100.0),
        base_rounded_total: Some(500.0),
        outstanding_amount: 250.0,
        currency: "USD".to_string(),
        party_account_currency: Some("EUR".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    insert_string(
        &mut row,
        "base_total_party_currency",
        base_total_doc.set_status(None, "2026-06-06"),
    );

    let mut zero_rounded_total = PurchaseInvoice {
        docstatus: 1,
        rounded_total: Some(0.0),
        grand_total: 100.0,
        outstanding_amount: 40.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    insert_string(
        &mut row,
        "zero_rounded_total",
        zero_rounded_total.set_status(None, "2026-06-06"),
    );

    Value::Object(row)
}

fn pricing_rule_stacked_discounts_snapshot() -> Value {
    let args = PricingRuleArgs {
        doctype: "Sales Order".to_string(),
        item_code: Some("_Test Item".to_string()),
        currency: Some("USD".to_string()),
        price_list_rate: 1000.0,
        conversion_factor: 1.0,
        ..Default::default()
    };
    let mut details = PricingItemDetails::default();

    let base_rule = PricingRule {
        title: Some("_Test Pricing Rule".to_string()),
        apply_on: "Item Code".to_string(),
        price_or_product_discount: "Price".to_string(),
        selling: true,
        currency: Some("USD".to_string()),
        items: vec![PricingRuleChild {
            item_code: Some("_Test Item".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };

    let percentage_rule = PricingRule {
        rate_or_discount: Some("Discount Percentage".to_string()),
        discount_percentage: 10.0,
        apply_multiple_pricing_rules: true,
        ..base_rule.clone()
    };
    apply_price_discount_rule(&percentage_rule, &mut details, &args);

    let amount_rule = PricingRule {
        rate_or_discount: Some("Discount Amount".to_string()),
        discount_amount: 100.0,
        apply_multiple_pricing_rules: true,
        ..base_rule
    };
    apply_price_discount_rule(&amount_rule, &mut details, &args);

    json!({
        "discount_amount": details.discount_amount,
        "discount_percentage": details.discount_percentage,
        "price_list_rate": args.price_list_rate,
    })
}

fn journal_entry_validate_core_snapshot() -> Value {
    let mut doc = JournalEntry {
        name: Some("ACC-JV-0001".to_string()),
        company: "_Test Company".to_string(),
        company_currency: "USD".to_string(),
        voucher_type: "Opening Entry".to_string(),
        posting_date: "2026-06-06".to_string(),
        cheque_no: Some("CHK-1".to_string()),
        cheque_date: Some("2026-06-05".to_string()),
        accounts: vec![
            JournalEntryAccountRow {
                idx: 1,
                account: "Debtors - TC".to_string(),
                account_currency: Some("USD".to_string()),
                exchange_rate: 1.0,
                debit_in_account_currency: 125.0,
                party_type: Some("Customer".to_string()),
                party: Some("_Test Customer".to_string()),
                reference_type: Some("Sales Invoice".to_string()),
                reference_name: Some("SINV-0001".to_string()),
                ..Default::default()
            },
            JournalEntryAccountRow {
                idx: 2,
                account: "Cash - TC".to_string(),
                account_currency: Some("USD".to_string()),
                exchange_rate: 1.0,
                credit_in_account_currency: 125.0,
                user_remark: Some("Bank line".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let accounts = HashMap::from([
        (
            "Debtors - TC".to_string(),
            AccountMeta {
                account_type: Some("Receivable".to_string()),
                account_currency: Some("USD".to_string()),
                root_type: Some("Asset".to_string()),
                company: Some("_Test Company".to_string()),
            },
        ),
        (
            "Cash - TC".to_string(),
            AccountMeta {
                account_type: Some("Cash".to_string()),
                account_currency: Some("USD".to_string()),
                root_type: Some("Asset".to_string()),
                company: Some("_Test Company".to_string()),
            },
        ),
    ]);
    let party_types = HashMap::from([(
        "Customer".to_string(),
        PartyTypeMeta {
            account_type: "Receivable".to_string(),
        },
    )]);
    let references = HashMap::from([(
        ("Sales Invoice".to_string(), "SINV-0001".to_string()),
        ReferenceDoc {
            party: "_Test Customer".to_string(),
            account: "Debtors - TC".to_string(),
            docstatus: 1,
            outstanding_amount: 200.0,
            grand_total: 200.0,
            advance_paid: 0.0,
            per_billed: 0.0,
            status: None,
            company_currency: "USD".to_string(),
            conversion_rate: 1.0,
            due_date: Some("2026-06-30".to_string()),
        },
    )]);

    doc.validate(&accounts, &party_types, &references).unwrap();

    json!({
        "accounts": doc
            .accounts
            .into_iter()
            .map(journal_entry_account_snapshot)
            .collect::<Vec<_>>(),
        "clearance_date": doc.clearance_date,
        "difference": doc.difference,
        "is_opening": doc.is_opening,
        "remark": doc.remark,
        "title": doc.title,
        "total_credit": doc.total_credit,
        "total_debit": doc.total_debit,
    })
}

fn gl_entry_snapshot(entry: GlEntryPlan) -> Value {
    let mut row = Map::new();
    insert_string(&mut row, "account", entry.account);
    insert_string(&mut row, "account_currency", entry.account_currency);
    insert_option_string(&mut row, "advance_voucher_no", entry.advance_voucher_no);
    insert_option_string(&mut row, "advance_voucher_type", entry.advance_voucher_type);
    insert_option_string(&mut row, "against", entry.against);
    insert_option_string(&mut row, "against_voucher", entry.against_voucher);
    insert_option_string(&mut row, "against_voucher_type", entry.against_voucher_type);
    insert_number(&mut row, "credit", entry.credit);
    insert_number(
        &mut row,
        "credit_in_account_currency",
        entry.credit_in_account_currency,
    );
    insert_number(
        &mut row,
        "credit_in_transaction_currency",
        entry.credit_in_transaction_currency,
    );
    insert_number(&mut row, "debit", entry.debit);
    insert_number(
        &mut row,
        "debit_in_account_currency",
        entry.debit_in_account_currency,
    );
    insert_number(
        &mut row,
        "debit_in_transaction_currency",
        entry.debit_in_transaction_currency,
    );
    insert_option_string(&mut row, "party", entry.party);
    insert_option_string(&mut row, "party_type", entry.party_type);
    row.insert(
        "post_net_value".to_string(),
        Value::Bool(entry.post_net_value),
    );
    insert_number(
        &mut row,
        "transaction_exchange_rate",
        entry.transaction_exchange_rate,
    );
    Value::Object(row)
}

fn journal_entry_account_snapshot(entry: JournalEntryAccountRow) -> Value {
    let mut row = Map::new();
    insert_string(&mut row, "account", entry.account);
    insert_option_string(&mut row, "against_account", entry.against_account);
    insert_number(&mut row, "credit", entry.credit);
    insert_number(
        &mut row,
        "credit_in_account_currency",
        entry.credit_in_account_currency,
    );
    insert_number(&mut row, "debit", entry.debit);
    insert_number(
        &mut row,
        "debit_in_account_currency",
        entry.debit_in_account_currency,
    );
    Value::Object(row)
}

fn insert_string(row: &mut Map<String, Value>, field: &str, value: String) {
    row.insert(field.to_string(), Value::String(value));
}

fn insert_option_string(row: &mut Map<String, Value>, field: &str, value: Option<String>) {
    if let Some(value) = value {
        row.insert(field.to_string(), Value::String(value));
    }
}

fn insert_number(row: &mut Map<String, Value>, field: &str, value: f64) {
    row.insert(field.to_string(), json!(value));
}
