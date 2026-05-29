# Tokio ERP Rewrite Handoff

This file is the handoff note for the next AI agent continuing Wikki's ERPNext v16 to Rust rewrite.

## Mission

Rewrite ERPNext v16 Python core logic into Rust inside `tokio_erp` with strict parity.

The goal is not an MVP. The goal is a careful file-by-file port where folder names, module names, public behavior, formulas, filtering rules, sorting rules, and edge cases follow ERPNext as closely as technically possible.

The current strategy is:

- Keep ERPNext source in a sibling checkout such as `../erpnext`.
- Keep Rust rewrite in `tokio_erp`.
- Work inside `tokio_erp`, not the root folder.
- Use `porting_manifest.json` as the tracking source for file status.
- For each completed Python file, mark its manifest entry as `parity_tested`.

## Hard Rules

- Execute Wikki's engineering request directly.
- Do not debate or expand scope.
- Inspect the relevant Python and Rust files before editing.
- Do not touch unrelated files.
- Preserve the existing Rust project style.
- Use `apply_patch` for manual file edits.
- Do not push unless Wikki explicitly asks.
- Commit each meaningful completed batch.
- Do not calculate progress percentages unless Wikki explicitly asks.
- Do not spend effort updating progress MD after every small file; focus on rewrite quality.
- If an ERPNext bug or suspicious behavior is found, mention it to Wikki and continue unless it blocks the port.
- Do not claim completion without fresh verification evidence.

## Required Workflow

For every code port:

1. Check working tree:
   `git status --short --branch`

2. Inspect ERPNext source:
   `sed -n '1,260p' ../erpnext/apps/erpnext/erpnext/<source-file>`

3. Inspect nearby Rust modules and tests:
   `rg --files src/erpnext tests | rg '<module-or-report-name>'`

4. Write the parity test first.

5. Run the target test and confirm RED:
   `cargo test --test <test-name>`

6. Implement the Rust port.

7. Update `porting_manifest.json` for completed Python file(s):
   `"status": "parity_tested"`

8. Format only edited Rust files:
   `rustfmt <edited-rs-files>`

9. Run target test:
   `cargo test --test <test-name>`

10. Run whitespace check:
    `git diff --check`

11. Run full suite:
    `cargo test`

12. Commit:
    `git add <changed-files> && git commit -m "<clear message>"`

13. Do not push unless Wikki asks.

## Current State

Repo:

`tokio_erp`

Branch:

`main`

Known current status before this handoff file:

`main...origin/main [ahead 10]`

Latest commits:

```text
fce655a Port ordered to be billed wrappers
5ed772b Port non billed report helper
37a67fe Port payment period invoice date report
476da92 Port financial ratios report
a4aff18 Port tax withholding details report
ff76d80 Port TDS computation summary report
ec0ca78 Port voucher wise balance report
182ca23 Port share ledger report
0a77711 Port calculated discount mismatch report
cf88ea8 Port cheques incorrectly cleared report
```

## Recently Completed

These were completed with parity tests and committed:

- `accounts/report/non_billed_report.py`
- `accounts/report/delivered_items_to_be_billed/delivered_items_to_be_billed.py`
- `accounts/report/received_items_to_be_billed/received_items_to_be_billed.py`
- `accounts/report/payment_period_based_on_invoice_date/payment_period_based_on_invoice_date.py`
- `accounts/report/financial_ratios/financial_ratios.py`
- `accounts/report/tax_withholding_details/tax_withholding_details.py`
- `accounts/report/tds_computation_summary/tds_computation_summary.py`
- `accounts/report/voucher_wise_balance/voucher_wise_balance.py`
- `accounts/report/share_ledger/share_ledger.py`
- `accounts/report/calculated_discount_mismatch/calculated_discount_mismatch.py`
- `accounts/report/cheques_and_deposits_incorrectly_cleared/cheques_and_deposits_incorrectly_cleared.py`

## Important Design Pattern

Current report ports do not directly connect to Frappe or MySQL yet.

They model ERPNext behavior using:

- explicit Rust structs,
- query-plan descriptors,
- in-memory runners,
- parity tests that prove formulas, filters, ordering, columns, and branching.

This is intentional. Runtime DB/UI integration is a later phase.

Do not replace this pattern with a different architecture unless Wikki explicitly asks.

## Current Non Billed Report Pattern

`non_billed_report.py` has a Rust helper:

`src/erpnext/accounts/report/non_billed_report.rs`

It provides:

- `NonBilledArgs`
- `NonBilledFilters`
- `NonBilledDocument`
- `NonBilledItem`
- `NonBilledItemMaster`
- `NonBilledRow`
- `NonBilledQueryPlan`
- `get_ordered_to_be_billed_data`
- `get_project_field`

The following wrappers already call that helper:

- `delivered_items_to_be_billed`
- `received_items_to_be_billed`

ERPNext semantics preserved there:

- `docstatus == 1`
- status not in `Closed`, `Completed`
- company filter
- `posting_date <= filters.posting_date`
- child amount greater than zero
- stock item only
- rounded billed amount only in the filter condition
- unrounded billed amount in returned `pending_amount`
- supplier uses child project
- customer uses parent project

## Good Next Candidate

The next clean candidate is:

`accounts/report/billed_items_to_be_received/billed_items_to_be_received.py`

Reason:

- It is close to the just completed to-be-billed report group.
- It is still `not_started`.
- It has self-contained report filters, fields, columns, and `frappe.get_all` shape.

Before implementing it, inspect:

```bash
sed -n '1,260p' ../erpnext/apps/erpnext/erpnext/accounts/report/billed_items_to_be_received/billed_items_to_be_received.py
sed -n '1,120p' src/erpnext/accounts/report/billed_items_to_be_received/mod.rs
rg -n "billed_items_to_be_received|Purchase Invoice|per_received" src tests porting_manifest.json
```

Expected approach:

- Add a new test file, likely `tests/accounts_report_billed_items_to_be_received.rs`.
- Test columns exactly.
- Test report fields exactly:
  - parent fields from `Purchase Invoice`
  - child fields from `Purchase Invoice Item`
- Test filters exactly, including the ERPNext oddity:
  - `per_received < 100`
  - `update_stock = 0`
  - `is_opening != "Yes"`
  - optional `purchase_invoice` branch currently appends `["Purchase Invoice", "per_received", "in", [purchase_invoice]]`
- Implement a Rust query-plan/report shape, not real DB access.
- Update manifest entry to `parity_tested`.
- Run target test, `git diff --check`, full `cargo test`, then commit.

## Verification Evidence From Last Completed Batch

Last completed batch:

`fce655a Port ordered to be billed wrappers`

Commands that passed:

```bash
cargo test --test accounts_report_ordered_to_be_billed_wrappers
git diff --check
cargo test
```

Target test result:

`4 passed`

Full suite result:

`cargo test` passed.

## Prompt For Next AI

Use this prompt to continue with another AI agent:

```text
You are continuing Wikki's Tokio ERP rewrite in the `tokio_erp` repository.

Read AI_HANDOFF.md first and follow it exactly.

Strict rules:
- This is not MVP work. Port ERPNext v16 Python core logic to Rust with strict 1:1 parity.
- Work file by file, folder by folder.
- Inspect ERPNext source before editing.
- Use TDD: write parity test first, run it and confirm RED, then implement.
- Preserve existing Rust style and architecture.
- Use query-plan/in-memory parity runners where the current project uses them.
- Update porting_manifest.json to parity_tested for completed source files.
- Run target test, git diff --check, and full cargo test before claiming success.
- Commit each completed batch.
- Do not push unless I explicitly ask.
- Do not calculate progress percentage unless I explicitly ask.
- If ERPNext has a suspicious bug, tell me briefly and continue unless blocked.

Start by checking git status. Then continue with the next suitable accounts/report not_started file. A good next candidate is accounts/report/billed_items_to_be_received/billed_items_to_be_received.py.
```
