# Accounts Differential Testing

This is the proof layer for the ERPNext `accounts` Rust port.

The normal Rust parity tests prove individual branches. The golden differential tests prove that
selected ERPNext-visible scenarios produce the same canonical JSON output in Rust.

## Current Harness

- Fixture: `tests/fixtures/accounts_golden/core_scenarios.json`
- Test: `tests/accounts_golden_differential.rs`
- Rust runner: `src/erpnext/accounts/parity.rs`

Run:

```sh
cargo test --test accounts_golden_differential
```

## Scenario Contract

Each fixture row has:

- `scenario`: stable runner key.
- `source`: ERPNext Python module/test used as the behavioral source.
- `expected`: canonical ERPNext output.

The Rust runner computes the same scenario and compares `serde_json::Value` directly. Any drift
fails with the scenario name and ERPNext source path.

## First Covered Scenarios

- `payment_entry_receive_gl`: validates Payment Entry amounts and GL map shape.
- `purchase_invoice_supplier_hold`: validates Supplier invoice hold rejection.
- `sales_invoice_cancelled_status`: validates Sales Invoice cancel precedence over internal transfer.
- `pricing_rule_stacked_discounts`: validates stacked Discount Percentage + Discount Amount behavior.
- `journal_entry_validate_core`: validates Journal Entry opening default, amount conversion, totals,
  against-account assignment, remarks, title, and cleared-date reset.
- `sales_invoice_status_matrix`: validates ERPNext Sales Invoice status precedence for paid,
  partly paid, overdue, discounted, unpaid, return, credit note, internal transfer, cancelled,
  draft, amended-new, and party-account-currency total cases.
- `purchase_invoice_status_matrix`: validates ERPNext Purchase Invoice status precedence for paid,
  partly paid, overdue, unpaid, return, debit note, internal transfer, cancelled, draft,
  amended-new, and party-account-currency total cases.
- `payment_entry_hold_guards`: validates supplier payment hold release-date behavior and Purchase
  Invoice on-hold reference rejection.

## Expansion Rule

Add scenarios in this order:

1. Generate or inspect ERPNext output for a narrow behavior.
2. Add the fixture row first.
3. Run `cargo test --test accounts_golden_differential` and see RED if Rust lacks parity.
4. Implement the Rust scenario or fix the port.
5. Re-run the differential test and the relevant target module test.

Do not use this harness as a replacement for targeted unit tests. Use it as the module-level
proof that Rust and ERPNext agree on externally visible outputs.
