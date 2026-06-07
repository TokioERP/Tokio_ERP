use serde_json::{json, Map, Value};

use crate::erpnext::accounts::doctype::payment_entry::payment_entry::{
    GlEntryPlan, PaymentEntry, PaymentEntryDeductionRow, PaymentEntryReferenceRow,
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
