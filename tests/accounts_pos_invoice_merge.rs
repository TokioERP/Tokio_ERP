use tokio_erp::erpnext::accounts::doctype::pos_invoice_merge_log::pos_invoice_merge_log::{
    PosInvoiceMergeLog, PosInvoiceMergeLogDocument, PosInvoiceMergeLogPayment,
    PosInvoiceMergeLogProfileDefaults, PosInvoiceMergeLogTax,
};

#[test]
fn pos_invoice_merge_discount_scenarios_match_erpnext_test_pos_invoice_merge() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    let discounted = PosInvoiceMergeLogDocument {
        rounded_total: 270.0,
        base_rounded_total: 270.0,
        payments: vec![PosInvoiceMergeLogPayment {
            mode_of_payment: "Cash".to_string(),
            account: "Cash - _TC".to_string(),
            amount: 270.0,
            base_amount: 270.0,
        }],
        ..PosInvoiceMergeLogDocument::new("POS-DISCOUNT")
    };
    let regular = PosInvoiceMergeLogDocument {
        rounded_total: 3200.0,
        base_rounded_total: 3200.0,
        payments: vec![PosInvoiceMergeLogPayment {
            mode_of_payment: "Cash".to_string(),
            account: "Cash - _TC".to_string(),
            amount: 3200.0,
            base_amount: 3200.0,
        }],
        ..PosInvoiceMergeLogDocument::new("POS-REGULAR")
    };

    let plan = log.merge_pos_invoice_into_plan(
        &[discounted, regular],
        &PosInvoiceMergeLogProfileDefaults::default(),
        true,
        &[],
    );

    assert_eq!(plan.rounded_total, 3470.0);
    assert_eq!(plan.base_rounded_total, 3470.0);
    assert_eq!(plan.payments.len(), 1);
    assert_eq!(plan.payments[0].amount, 3470.0);
}

#[test]
fn pos_invoice_merge_inclusive_tax_discount_scenario_matches_erpnext() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    let inclusive_regular = PosInvoiceMergeLogDocument {
        rounded_total: 300.0,
        base_rounded_total: 300.0,
        taxes: vec![service_tax("TAX-1", 36.8421052632)],
        payments: vec![cash_payment(300.0)],
        ..PosInvoiceMergeLogDocument::new("POS-INCL-1")
    };
    let inclusive_discounted = PosInvoiceMergeLogDocument {
        rounded_total: 540.0,
        base_rounded_total: 540.0,
        taxes: vec![service_tax("TAX-2", 66.3157894737)],
        payments: vec![cash_payment(540.0)],
        ..PosInvoiceMergeLogDocument::new("POS-INCL-2")
    };

    let plan = log.merge_pos_invoice_into_plan(
        &[inclusive_regular, inclusive_discounted],
        &PosInvoiceMergeLogProfileDefaults::default(),
        true,
        &[],
    );

    assert_eq!(plan.rounded_total, 840.0);
    assert_eq!(plan.base_rounded_total, 840.0);
    assert_eq!(plan.payments[0].amount, 840.0);
    assert_eq!(plan.taxes.len(), 1);
    assert!((plan.taxes[0].tax_amount - 103.1578947369).abs() < 0.000001);
}

#[test]
fn pos_invoice_merge_selling_price_validation_keeps_only_valid_submitted_invoice() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    let valid_invoice = PosInvoiceMergeLogDocument {
        rounded_total: 400.0,
        base_rounded_total: 400.0,
        taxes: vec![service_tax("TAX-VALID", 49.1228070175)],
        payments: vec![cash_payment(400.0)],
        ..PosInvoiceMergeLogDocument::new("POS-VALID-SELLING-PRICE")
    };

    let plan = log.merge_pos_invoice_into_plan(
        &[valid_invoice],
        &PosInvoiceMergeLogProfileDefaults::default(),
        true,
        &[],
    );

    assert_eq!(plan.rounded_total, 400.0);
    assert_eq!(plan.base_rounded_total, 400.0);
    assert_eq!(plan.payments[0].amount, 400.0);
}

fn cash_payment(amount: f64) -> PosInvoiceMergeLogPayment {
    PosInvoiceMergeLogPayment {
        mode_of_payment: "Cash".to_string(),
        account: "Cash - _TC".to_string(),
        amount,
        base_amount: amount,
    }
}

fn service_tax(name: &str, amount: f64) -> PosInvoiceMergeLogTax {
    PosInvoiceMergeLogTax {
        name: name.to_string(),
        account_head: "_Test Account Service Tax - _TC".to_string(),
        cost_center: "_Test Cost Center - _TC".to_string(),
        tax_amount_after_discount_amount: amount,
        base_tax_amount_after_discount_amount: amount,
    }
}
