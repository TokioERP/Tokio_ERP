use serde_json::Value;
use tokio_erp::erpnext::accounts::parity::{accounts_golden_snapshot, AccountsGoldenScenario};

#[test]
fn accounts_core_golden_scenarios_match_erpnext_snapshots() {
    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/accounts_golden/core_scenarios.json")).unwrap();
    let fixtures = fixtures.as_array().unwrap();

    for fixture in fixtures {
        let scenario = match fixture["scenario"].as_str().unwrap() {
            "payment_entry_receive_gl" => AccountsGoldenScenario::PaymentEntryReceiveGl,
            "purchase_invoice_supplier_hold" => AccountsGoldenScenario::PurchaseInvoiceSupplierHold,
            "sales_invoice_cancelled_status" => AccountsGoldenScenario::SalesInvoiceCancelledStatus,
            "pricing_rule_stacked_discounts" => AccountsGoldenScenario::PricingRuleStackedDiscounts,
            "journal_entry_validate_core" => AccountsGoldenScenario::JournalEntryValidateCore,
            "sales_invoice_status_matrix" => AccountsGoldenScenario::SalesInvoiceStatusMatrix,
            "purchase_invoice_status_matrix" => AccountsGoldenScenario::PurchaseInvoiceStatusMatrix,
            "payment_entry_hold_guards" => AccountsGoldenScenario::PaymentEntryHoldGuards,
            other => panic!("unknown accounts golden scenario: {other}"),
        };

        let actual = accounts_golden_snapshot(scenario);
        assert_eq!(
            actual, fixture["expected"],
            "accounts golden mismatch for {} from {}",
            fixture["scenario"], fixture["source"]
        );
    }
}
