# AI Handoff

This file is the operational handoff for any AI agent continuing Wikki's Tokio ERP
rewrite work.

## Mission

Tokio ERP is an independent GPLv3 Rust rewrite of ERPNext v16 core logic.

The goal is not an inspired clone. The goal is traceable 1:1 behavioral parity:
inspect ERPNext Python, write Rust tests, see RED, port the smallest matching Rust
surface, see GREEN, update the manifest, and commit the completed batch.

Repo:

```text
/Volumes/Samsung990P/rust_erp/tokio_erp
```

ERPNext source baseline:

```text
/Volumes/Samsung990P/rust_erp/erpnext/apps/erpnext/erpnext
```

Current baseline from `README.md`:

```text
Frappe:  version-16 at 69be97cf314e53e314678c5af98fef25f5c2d7e6
ERPNext: version-16 at 6ef4a2d82cfe3815a8bd491436256df3e4ed79b8
```

## Non-Negotiable Work Rules

Start every user turn with:

```sh
git status --short
```

Before changing Rust code:

1. Inspect the relevant ERPNext Python source first.
2. Inspect the existing Rust module and tests.
3. Write or extend a focused test/fixture first.
4. Run the target test and confirm RED for the missing behavior.
5. Implement the smallest Rust parity change.
6. Run the target test and confirm GREEN.
7. Run formatting/checks appropriate to the batch.
8. Update `porting_manifest.json` and the relevant docs/map when a source file is completed.
9. Commit each completed batch.

Do not push unless Wikki explicitly says to push.

Do not calculate or report progress percentages.

Do not run repeated full `cargo test`/full builds for small changes. Wikki's machine
overheats under repeated full-suite runs. Use focused target tests while porting.
Run full suite only at larger review/module checkpoints or when Wikki explicitly asks.

Use `rg`/`rg --files` for search.

Use `apply_patch` for file edits.

Never revert user changes unless Wikki explicitly asks.

## Verification Policy

For a normal focused batch, run:

```sh
cargo test --test <target_test>
cargo fmt --check
git diff --check
```

When the manifest was touched, also run:

```sh
cargo test --test porting_manifest
```

For module-close or larger checkpoint review, run the relevant module target tests.
Only run full:

```sh
cargo test
```

when closing a large module group, after many modules, or when Wikki asks.

Never claim parity, completion, or passing tests without fresh command output.

## Commit And Push Discipline

Commit every completed batch with a short concrete message.

Push only after Wikki says push. Recent push already happened after:

```text
fb62f09 Expand accounts golden differential scenarios
```

Remote:

```text
https://github.com/TokioERP/Tokio_ERP.git
```

## Current Git Checkpoint

Latest commits at handoff:

```text
fb62f09 Expand accounts golden differential scenarios
727efd9 Add accounts golden differential harness
adabb45 Port sales invoice cancel status regression
9f86100 Port purchase invoice hold status regressions
49f9fba Port pricing rule cleanup regressions
304cc2c Port payment entry hold regressions
afff592 Port journal entry test regressions
7c9c40b Port budget test regressions
```

At the time this handoff was written, `git status --short` was clean before edits.

## Accounts Status

`accounts/` is complete in the manifest:

```text
686 accounts entries
686 parity_tested
```

The useful command:

```sh
jq -r '[.entries[] | select(.source|startswith("accounts/"))] | {total:length, statuses:(group_by(.status)|map({status:.[0].status,count:length}))}' porting_manifest.json
```

Do not treat this as production/runtime replacement proof. The correct claim is:

```text
Accounts core logic has strong targeted parity coverage and every accounts manifest
entry is parity_tested. Full ERPNext runtime replacement still requires integration
proof after enough dependent modules and runtime infrastructure exist.
```

Accounts proof layer:

```text
docs/accounts_differential_testing.md
tests/accounts_golden_differential.rs
tests/fixtures/accounts_golden/core_scenarios.json
src/erpnext/accounts/parity.rs
```

Current golden scenarios:

```text
payment_entry_receive_gl
purchase_invoice_supplier_hold
sales_invoice_cancelled_status
pricing_rule_stacked_discounts
journal_entry_validate_core
sales_invoice_status_matrix
purchase_invoice_status_matrix
payment_entry_hold_guards
```

Last verified accounts proof command:

```sh
cargo test --test accounts_golden_differential --test accounts_journal_entry --test accounts_sales_invoice --test accounts_purchase_invoice --test accounts_payment_entry
```

It passed before commit `fb62f09`.

Also passed:

```sh
cargo fmt --check
git diff --check
```

## Manifest And Dashboard Rule

Wikki watches a progress/dashboard app. It depends on manifest/doc state.

When a file/module is completed:

1. Set the corresponding `porting_manifest.json` entry to `parity_tested`.
2. Update the relevant port map doc, especially `docs/accounts_port_map.md` for accounts.
3. Run `cargo test --test porting_manifest` when the manifest changes.
4. Commit the manifest and code together.

Do not say a file is closed if the manifest still leaves it unclosed.

## Legal And Public Repo Context

Tokio ERP is GPLv3 and must keep attribution/trademark boundaries clean.

Relevant files:

```text
README.md
LICENSE
NOTICE.md
TRADEMARKS.md
```

Do not present Tokio ERP as affiliated with, endorsed by, sponsored by, or approved
by Frappe Technologies. Use ERPNext/Frappe names only for attribution, source
reference, and compatibility description.

## How To Continue Porting

The next large functional area after accounts is `stock/`.

Current non-accounts remaining manifest counts at handoff:

```text
patches:       428
stock:         337
manufacturing: 177
setup:         128
selling:       121
crm:            97
buying:         94
assets:         79
projects:       62
regional:       50
```

`patches/` is numerically largest, but for business-core parity the next practical
functional module is `stock/`. Choose the next exact file by inspecting manifest
order and ERPNext source; do not guess behavior from memory.

Useful command for stock candidates:

```sh
jq -r '[.entries[] | select((.status != "parity_tested") and (.source|startswith("stock/"))) | .source] | .[:30][]' porting_manifest.json
```

Initial visible stock candidates at handoff included:

```text
stock/dashboard/item_dashboard.py
stock/dashboard/warehouse_capacity_dashboard.py
stock/dashboard_chart_source/stock_value_by_item_group/stock_value_by_item_group.py
stock/dashboard_chart_source/warehouse_wise_stock_value/warehouse_wise_stock_value.py
stock/deprecated_serial_batch.py
stock/doctype/batch/batch.py
stock/doctype/batch/test_batch.py
stock/doctype/bin/bin.py
stock/doctype/bin/test_bin.py
stock/doctype/customs_tariff_number/customs_tariff_number.py
stock/doctype/customs_tariff_number/test_customs_tariff_number.py
stock/doctype/delivery_note/delivery_note.py
stock/doctype/delivery_note/test_delivery_note.py
```

Start with a narrow, low-risk stock file if Wikki asks to continue fast but safely.
Good first candidates are pass/no-op metadata doctypes or small deterministic
helpers. For bigger files like Delivery Note, inspect deeply and split into
larger-but-safe TDD batches.

## Style Of Work Wikki Expects

Wikki calls this "zargar rejim" work.

That means:

- exact ERPNext source inspection,
- no invented behavior,
- no loose paraphrase of Python logic,
- focused tests before implementation,
- larger safe batches when the source shape allows it,
- no tiny artificial 10-line progress unless the logic truly demands it,
- no full-suite heat cycles for every small change,
- honest review before claiming closure,
- manifest synced so the dashboard reflects reality.

When Wikki asks a question, answer that exact question.

When Wikki gives a command, execute it.

Use Uzbek in conversation with Wikki unless a repo artifact is better in English.

## Known Proof Gap

Accounts can be called parity-tested at core-logic/module-boundary level. It cannot
yet be called a full drop-in runtime replacement for ERPNext accounts because full
runtime proof needs:

```text
Frappe document lifecycle
database writes and queries
hooks and permissions
cross-module workflows
submit/cancel/repost side effects
reports against real data
integration with stock, selling, buying, setup, assets, and related modules
```

The correct path is to keep collecting module-level parity proof now, then run
runtime/integration differential tests once enough dependent modules and runtime
infrastructure are ported.

## If You Are The Next Agent

Do this first:

```sh
cd /Volumes/Samsung990P/rust_erp/tokio_erp
git status --short
sed -n '1,260p' AI_HANDOFF.md
jq -r '[.entries[] | select(.status != "parity_tested")] | {remaining:length, statuses:(group_by(.status)|map({status:.[0].status,count:length}))}' porting_manifest.json
```

Then follow Wikki's latest message, not old conversation context.
