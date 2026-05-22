# Accounts Module Port Map

Source: `../erpnext/apps/erpnext/erpnext/accounts`
Target: `src/erpnext/accounts`

## Rule

This is not an MVP list. Every source folder and file is tracked. A row is closed only when the Rust side preserves the ERPNext-visible behavior for that item or the item is explicitly classified as metadata/UI/static and intentionally left to ERPNext.

## Status Values

- `not_started`: inventory exists, no port work done.
- `mapped`: source behavior and target Rust file/path are identified.
- `ported`: Rust implementation exists.
- `parity_tested`: compared against ERPNext behavior and passed.
- `external_kept`: JSON, JS, HTML, CSV, CSS, or static metadata intentionally remains in ERPNext/Frappe.

## Totals

- Directories: 317
- Files: 1291
- `py`: 686
- `json`: 393
- `js`: 162
- `html`: 28
- `md`: 20
- `csv`: 1
- `css`: 1

## Submodules

| Submodule | Dirs | Files | Python | JSON | JS | Status | Notes |
|---|---:|---:|---:|---:|---:|---|---|
| `accounts_dashboard` | 3 | 2 | 0 | 2 | 0 | not_started | |
| `custom` | 1 | 2 | 1 | 1 | 0 | not_started | |
| `dashboard_chart` | 8 | 7 | 0 | 7 | 0 | not_started | |
| `dashboard_chart_source` | 2 | 5 | 3 | 1 | 1 | not_started | |
| `doctype` | 191 | 923 | 504 | 282 | 111 | mapped | `bank_account_subtype` parity-tested; remaining doctypes pending. |
| `financial_report_template` | 7 | 14 | 7 | 7 | 0 | not_started | |
| `letterhead` | 1 | 2 | 0 | 0 | 0 | not_started | |
| `module_onboarding` | 2 | 1 | 0 | 1 | 0 | not_started | |
| `notification` | 2 | 5 | 3 | 1 | 0 | not_started | |
| `number_card` | 5 | 4 | 0 | 4 | 0 | not_started | |
| `onboarding_step` | 7 | 6 | 0 | 6 | 0 | not_started | |
| `page` | 1 | 1 | 1 | 0 | 0 | not_started | |
| `print_format` | 26 | 57 | 26 | 25 | 0 | not_started | |
| `print_format_field_template` | 3 | 5 | 3 | 2 | 0 | not_started | |
| `report` | 53 | 244 | 128 | 52 | 50 | not_started | |
| `test` | 1 | 4 | 4 | 0 | 0 | not_started | |
| `workspace` | 3 | 2 | 0 | 2 | 0 | not_started | |

## First Pass Order

1. `doctype`: primary Accounts Python controllers and child table controllers.
2. `report`: Python report logic after controller mapping exists.
3. `dashboard_chart_source`, `notification`, `custom`, `page`, `print_format_field_template`, `financial_report_template`, `print_format`, `test`: smaller Python logic groups.
4. `accounts_dashboard`, `dashboard_chart`, `letterhead`, `module_onboarding`, `number_card`, `onboarding_step`, `workspace`: mostly metadata/static files, classify as `external_kept` after verification.

## Closure Checklist

For a Python file to move from `not_started` to `parity_tested`:

1. Source file path and Rust target path are recorded in `porting_manifest.json`.
2. Python behavior is summarized in this document or a linked child map.
3. Rust file exists at the mirrored path.
4. Unit test covers pure behavior where possible.
5. ERPNext parity test or fixture comparison passes where ERPNext-visible behavior exists.
6. This markdown row is updated with the closed status.

## Doctype Inventory

| Doctype Folder | Files | Python | JSON | JS | Status | Notes |
|---|---:|---:|---:|---:|---|---|
| `account` | 106 | 9 | 94 | 2 | not_started | |
| `account_category` | 5 | 3 | 1 | 1 | not_started | |
| `account_closing_balance` | 5 | 3 | 1 | 1 | not_started | |
| `accounting_dimension` | 5 | 3 | 1 | 1 | not_started | |
| `accounting_dimension_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_accounting_dimension_detail`. JSON kept external. |
| `accounting_dimension_filter` | 5 | 3 | 1 | 1 | not_started | |
| `accounting_period` | 5 | 3 | 1 | 1 | not_started | |
| `accounts_settings` | 6 | 3 | 1 | 2 | not_started | |
| `advance_payment_ledger_entry` | 5 | 3 | 1 | 1 | not_started | |
| `advance_taxes_and_charges` | 3 | 2 | 1 | 0 | not_started | |
| `allowed_dimension` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_allowed_dimension`. JSON kept external. |
| `allowed_to_transact_with` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_allowed_to_transact_with`. JSON kept external. |
| `applicable_on_account` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_applicable_on_account`. JSON kept external. |
| `bank` | 6 | 4 | 1 | 1 | not_started | |
| `bank_account` | 5 | 3 | 1 | 1 | not_started | |
| `bank_account_subtype` | 5 | 3 | 1 | 1 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_bank_account_subtype`. JSON/JS kept external. |
| `bank_account_type` | 5 | 3 | 1 | 1 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_bank_account_type`. JSON/JS kept external. |
| `bank_clearance` | 6 | 3 | 1 | 1 | not_started | |
| `bank_clearance_detail` | 4 | 2 | 1 | 0 | not_started | |
| `bank_guarantee` | 5 | 3 | 1 | 1 | not_started | |
| `bank_reconciliation_tool` | 5 | 3 | 1 | 1 | not_started | |
| `bank_statement_import` | 7 | 3 | 1 | 2 | not_started | |
| `bank_transaction` | 10 | 7 | 1 | 2 | not_started | |
| `bank_transaction_mapping` | 3 | 2 | 1 | 0 | not_started | |
| `bank_transaction_payments` | 3 | 2 | 1 | 0 | not_started | |
| `bisect_accounting_statements` | 5 | 3 | 1 | 1 | not_started | |
| `bisect_nodes` | 5 | 3 | 1 | 1 | not_started | |
| `budget` | 5 | 3 | 1 | 1 | not_started | |
| `budget_account` | 3 | 2 | 1 | 0 | not_started | |
| `budget_distribution` | 3 | 2 | 1 | 0 | not_started | |
| `campaign_item` | 3 | 2 | 1 | 0 | not_started | |
| `cashier_closing` | 5 | 3 | 1 | 1 | not_started | |
| `cashier_closing_payments` | 3 | 2 | 1 | 0 | not_started | |
| `chart_of_accounts_importer` | 6 | 3 | 1 | 1 | not_started | |
| `cheque_print_template` | 5 | 3 | 1 | 1 | not_started | |
| `closed_document` | 3 | 2 | 1 | 0 | not_started | |
| `cost_center` | 8 | 4 | 1 | 2 | not_started | |
| `cost_center_allocation` | 5 | 3 | 1 | 1 | not_started | |
| `cost_center_allocation_percentage` | 3 | 2 | 1 | 0 | not_started | |
| `coupon_code` | 5 | 3 | 1 | 1 | not_started | |
| `currency_exchange_settings` | 5 | 3 | 1 | 1 | not_started | |
| `currency_exchange_settings_details` | 3 | 2 | 1 | 0 | not_started | |
| `currency_exchange_settings_result` | 3 | 2 | 1 | 0 | not_started | |
| `customer_group_item` | 3 | 2 | 1 | 0 | not_started | |
| `customer_item` | 3 | 2 | 1 | 0 | not_started | |
| `discounted_invoice` | 3 | 2 | 1 | 0 | not_started | |
| `dunning` | 6 | 3 | 1 | 2 | not_started | |
| `dunning_letter_text` | 3 | 2 | 1 | 0 | not_started | |
| `dunning_type` | 5 | 3 | 1 | 1 | not_started | |
| `exchange_rate_revaluation` | 6 | 4 | 1 | 1 | not_started | |
| `exchange_rate_revaluation_account` | 3 | 2 | 1 | 0 | not_started | |
| `finance_book` | 6 | 4 | 1 | 1 | not_started | |
| `financial_report_row` | 3 | 2 | 1 | 0 | not_started | |
| `financial_report_template` | 8 | 6 | 1 | 1 | not_started | |
| `fiscal_year` | 7 | 4 | 1 | 1 | not_started | |
| `fiscal_year_company` | 3 | 2 | 1 | 0 | not_started | |
| `gl_entry` | 6 | 3 | 1 | 1 | not_started | |
| `invoice_discounting` | 7 | 4 | 1 | 2 | not_started | |
| `item_tax_template` | 6 | 4 | 1 | 1 | not_started | |
| `item_tax_template_detail` | 3 | 2 | 1 | 0 | not_started | |
| `item_wise_tax_detail` | 3 | 2 | 1 | 0 | not_started | |
| `journal_entry` | 8 | 3 | 2 | 2 | not_started | |
| `journal_entry_account` | 4 | 2 | 1 | 0 | not_started | |
| `journal_entry_template` | 5 | 3 | 1 | 1 | not_started | |
| `journal_entry_template_account` | 3 | 2 | 1 | 0 | not_started | |
| `ledger_health` | 5 | 3 | 1 | 1 | not_started | |
| `ledger_health_monitor` | 5 | 3 | 1 | 1 | not_started | |
| `ledger_health_monitor_company` | 3 | 2 | 1 | 0 | not_started | |
| `ledger_merge` | 5 | 3 | 1 | 1 | not_started | |
| `ledger_merge_accounts` | 3 | 2 | 1 | 0 | not_started | |
| `loyalty_point_entry` | 5 | 3 | 1 | 1 | not_started | |
| `loyalty_point_entry_redemption` | 3 | 2 | 1 | 0 | not_started | |
| `loyalty_program` | 6 | 4 | 1 | 1 | not_started | |
| `loyalty_program_collection` | 3 | 2 | 1 | 0 | not_started | |
| `mode_of_payment` | 6 | 3 | 1 | 1 | not_started | |
| `mode_of_payment_account` | 3 | 2 | 1 | 0 | not_started | |
| `monthly_distribution` | 7 | 4 | 1 | 1 | not_started | |
| `monthly_distribution_percentage` | 3 | 2 | 1 | 0 | not_started | |
| `opening_invoice_creation_tool` | 6 | 3 | 1 | 1 | not_started | |
| `opening_invoice_creation_tool_item` | 3 | 2 | 1 | 0 | not_started | |
| `overdue_payment` | 3 | 2 | 1 | 0 | not_started | |
| `party_account` | 3 | 2 | 1 | 0 | not_started | |
| `party_link` | 5 | 3 | 1 | 1 | not_started | |
| `payment_entry` | 6 | 3 | 1 | 2 | not_started | |
| `payment_entry_deduction` | 3 | 2 | 1 | 0 | not_started | |
| `payment_entry_reference` | 3 | 2 | 1 | 0 | not_started | |
| `payment_gateway_account` | 6 | 4 | 1 | 1 | not_started | |
| `payment_ledger_entry` | 5 | 3 | 1 | 1 | not_started | |
| `payment_order` | 6 | 4 | 1 | 1 | not_started | |
| `payment_order_reference` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reconciliation` | 5 | 3 | 1 | 1 | not_started | |
| `payment_reconciliation_allocation` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reconciliation_invoice` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reconciliation_payment` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reference` | 3 | 2 | 1 | 0 | not_started | |
| `payment_request` | 7 | 4 | 1 | 2 | not_started | |
| `payment_schedule` | 3 | 2 | 1 | 0 | not_started | |
| `payment_term` | 6 | 4 | 1 | 1 | not_started | |
| `payment_terms_template` | 6 | 4 | 1 | 1 | not_started | |
| `payment_terms_template_detail` | 3 | 2 | 1 | 0 | not_started | |
| `pegged_currencies` | 5 | 3 | 1 | 1 | not_started | |
| `pegged_currency_details` | 3 | 2 | 1 | 0 | not_started | |
| `period_closing_voucher` | 6 | 3 | 1 | 1 | not_started | |
| `pos_closing_entry` | 7 | 3 | 1 | 2 | not_started | |
| `pos_closing_entry_detail` | 3 | 2 | 1 | 0 | not_started | |
| `pos_closing_entry_taxes` | 3 | 2 | 1 | 0 | not_started | |
| `pos_customer_group` | 3 | 2 | 1 | 0 | not_started | |
| `pos_field` | 3 | 2 | 1 | 0 | not_started | |
| `pos_invoice` | 7 | 4 | 1 | 2 | not_started | |
| `pos_invoice_item` | 3 | 2 | 1 | 0 | not_started | |
| `pos_invoice_merge_log` | 5 | 3 | 1 | 1 | not_started | |
| `pos_invoice_reference` | 3 | 2 | 1 | 0 | not_started | |
| `pos_item_group` | 3 | 2 | 1 | 0 | not_started | |
| `pos_opening_entry` | 6 | 3 | 1 | 2 | not_started | |
| `pos_opening_entry_detail` | 3 | 2 | 1 | 0 | not_started | |
| `pos_payment_method` | 3 | 2 | 1 | 0 | not_started | |
| `pos_profile` | 5 | 3 | 1 | 1 | not_started | |
| `pos_profile_user` | 5 | 3 | 1 | 1 | not_started | |
| `pos_search_fields` | 3 | 2 | 1 | 0 | not_started | |
| `pos_settings` | 5 | 3 | 1 | 1 | not_started | |
| `pricing_rule` | 6 | 4 | 1 | 1 | not_started | |
| `pricing_rule_brand` | 3 | 2 | 1 | 0 | not_started | |
| `pricing_rule_detail` | 3 | 2 | 1 | 0 | not_started | |
| `pricing_rule_item_code` | 3 | 2 | 1 | 0 | not_started | |
| `pricing_rule_item_group` | 3 | 2 | 1 | 0 | not_started | |
| `process_deferred_accounting` | 5 | 3 | 1 | 1 | not_started | |
| `process_payment_reconciliation` | 7 | 4 | 1 | 2 | not_started | |
| `process_payment_reconciliation_log` | 6 | 3 | 1 | 2 | not_started | |
| `process_payment_reconciliation_log_allocations` | 3 | 2 | 1 | 0 | not_started | |
| `process_period_closing_voucher` | 5 | 3 | 1 | 1 | not_started | |
| `process_period_closing_voucher_detail` | 3 | 2 | 1 | 0 | not_started | |
| `process_statement_of_accounts` | 7 | 3 | 1 | 1 | not_started | |
| `process_statement_of_accounts_cc` | 3 | 2 | 1 | 0 | not_started | |
| `process_statement_of_accounts_customer` | 3 | 2 | 1 | 0 | not_started | |
| `process_subscription` | 5 | 3 | 1 | 1 | not_started | |
| `promotional_scheme` | 6 | 4 | 1 | 1 | not_started | |
| `promotional_scheme_price_discount` | 3 | 2 | 1 | 0 | not_started | |
| `promotional_scheme_product_discount` | 3 | 2 | 1 | 0 | not_started | |
| `psoa_cost_center` | 3 | 2 | 1 | 0 | not_started | |
| `psoa_project` | 3 | 2 | 1 | 0 | not_started | |
| `purchase_invoice` | 9 | 4 | 2 | 2 | not_started | |
| `purchase_invoice_advance` | 4 | 2 | 1 | 0 | not_started | |
| `purchase_invoice_item` | 4 | 2 | 1 | 0 | not_started | |
| `purchase_taxes_and_charges` | 4 | 2 | 1 | 0 | not_started | |
| `purchase_taxes_and_charges_template` | 6 | 4 | 1 | 1 | not_started | |
| `repost_accounting_ledger` | 6 | 3 | 1 | 1 | not_started | |
| `repost_accounting_ledger_items` | 3 | 2 | 1 | 0 | not_started | |
| `repost_allowed_types` | 3 | 2 | 1 | 0 | not_started | |
| `repost_payment_ledger` | 6 | 3 | 1 | 2 | not_started | |
| `repost_payment_ledger_items` | 3 | 2 | 1 | 0 | not_started | |
| `sales_invoice` | 10 | 4 | 2 | 3 | not_started | |
| `sales_invoice_advance` | 4 | 2 | 1 | 0 | not_started | |
| `sales_invoice_item` | 4 | 2 | 1 | 0 | not_started | |
| `sales_invoice_payment` | 3 | 2 | 1 | 0 | not_started | |
| `sales_invoice_reference` | 3 | 2 | 1 | 0 | not_started | |
| `sales_invoice_timesheet` | 3 | 2 | 1 | 0 | not_started | |
| `sales_partner_item` | 3 | 2 | 1 | 0 | not_started | |
| `sales_taxes_and_charges` | 4 | 2 | 1 | 0 | not_started | |
| `sales_taxes_and_charges_template` | 6 | 4 | 1 | 1 | not_started | |
| `share_balance` | 3 | 2 | 1 | 0 | not_started | |
| `share_transfer` | 5 | 3 | 1 | 1 | not_started | |
| `share_type` | 6 | 4 | 1 | 1 | not_started | |
| `shareholder` | 6 | 4 | 1 | 1 | not_started | |
| `shipping_rule` | 7 | 4 | 2 | 1 | not_started | |
| `shipping_rule_condition` | 3 | 2 | 1 | 0 | not_started | |
| `shipping_rule_country` | 3 | 2 | 1 | 0 | not_started | |
| `south_africa_vat_account` | 3 | 2 | 1 | 0 | not_started | |
| `subscription` | 6 | 3 | 1 | 2 | not_started | |
| `subscription_invoice` | 5 | 3 | 1 | 1 | not_started | |
| `subscription_plan` | 6 | 4 | 1 | 1 | not_started | |
| `subscription_plan_detail` | 3 | 2 | 1 | 0 | not_started | |
| `subscription_settings` | 5 | 3 | 1 | 1 | not_started | |
| `supplier_group_item` | 3 | 2 | 1 | 0 | not_started | |
| `supplier_item` | 3 | 2 | 1 | 0 | not_started | |
| `tax_category` | 6 | 4 | 1 | 1 | not_started | |
| `tax_rule` | 5 | 3 | 1 | 1 | not_started | |
| `tax_withholding_account` | 3 | 2 | 1 | 0 | not_started | |
| `tax_withholding_category` | 6 | 4 | 1 | 1 | not_started | |
| `tax_withholding_entry` | 4 | 3 | 1 | 0 | not_started | |
| `tax_withholding_group` | 5 | 3 | 1 | 1 | not_started | |
| `tax_withholding_rate` | 3 | 2 | 1 | 0 | not_started | |
| `territory_item` | 3 | 2 | 1 | 0 | not_started | |
| `transaction_deletion_record_details` | 3 | 2 | 1 | 0 | not_started | |
| `unreconcile_payment` | 5 | 3 | 1 | 1 | not_started | |
| `unreconcile_payment_entries` | 3 | 2 | 1 | 0 | not_started | |

## Doctype Detail: `bank_account_subtype`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/bank_account_subtype`
Target: `src/erpnext/accounts/doctype/bank_account_subtype`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes `account_subtype: DF.Data | None`.
- DocType metadata:
  - `name`: `Bank Account Subtype`
  - `module`: `Accounts`
  - `autoname`: `field:account_subtype`
  - `field_order`: `account_subtype`
  - `account_subtype`: `Data`, label `Account Subtype`, `unique: 1`
  - `allow_import`, `allow_rename`, and `quick_entry` are enabled.

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `bank_account_subtype.py` | `bank_account_subtype.rs` | parity_tested | No-op controller and `account_subtype` field represented in Rust. |
| `test_bank_account_subtype.py` | `tests/accounts_bank_account_subtype.rs` | parity_tested | Empty ERPNext test replaced by Rust metadata/controller parity tests. |
| `bank_account_subtype.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `bank_account_subtype.js` | ERPNext UI retained | external_kept | `refresh` handler is empty/no-op and remains UI-side. |

## Doctype Detail: `bank_account_type`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/bank_account_type`
Target: `src/erpnext/accounts/doctype/bank_account_type`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes `account_type: DF.Data | None`.
- DocType metadata:
  - `name`: `Bank Account Type`
  - `module`: `Accounts`
  - `autoname`: `field:account_type`
  - `field_order`: `account_type`
  - `account_type`: `Data`, label `Account Type`, `unique: 1`
  - `allow_import`, `allow_rename`, and `quick_entry` are enabled.

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `bank_account_type.py` | `bank_account_type.rs` | parity_tested | No-op controller and `account_type` field represented in Rust. |
| `test_bank_account_type.py` | `tests/accounts_bank_account_type.rs` | parity_tested | Empty ERPNext test replaced by Rust metadata/controller parity tests. |
| `bank_account_type.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `bank_account_type.js` | ERPNext UI retained | external_kept | Only commented refresh scaffold exists and remains UI-side. |

## Doctype Detail: `allowed_dimension`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/allowed_dimension`
Target: `src/erpnext/accounts/doctype/allowed_dimension`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes `accounting_dimension`, `dimension_value`, and child table parent fields.
- DocType metadata:
  - `name`: `Allowed Dimension`
  - `module`: `Accounts`
  - `istable`: enabled
  - `field_order`: `accounting_dimension`, `dimension_value`
  - `accounting_dimension`: `Link`, label `Accounting Dimension`, options `DocType`, `read_only: 1`
  - `dimension_value`: `Dynamic Link`, options `accounting_dimension`, `in_list_view: 1`
  - `quick_entry`, `index_web_pages_for_search`, and `track_changes` are enabled.

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `allowed_dimension.py` | `allowed_dimension.rs` | parity_tested | No-op child table controller and fields represented in Rust. |
| `allowed_dimension.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `allowed_to_transact_with`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/allowed_to_transact_with`
Target: `src/erpnext/accounts/doctype/allowed_to_transact_with`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `company` and child table parent fields.
- DocType metadata:
  - `name`: `Allowed To Transact With`
  - `module`: `Accounts`
  - `istable`: enabled
  - `field_order`: `company`
  - `company`: `Link`, label `Company`, options `Company`, `reqd: 1`, `in_list_view: 1`, `ignore_user_permissions: 1`
  - `quick_entry` and `track_changes` are enabled.

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `allowed_to_transact_with.py` | `allowed_to_transact_with.rs` | parity_tested | No-op child table controller and required company link represented in Rust. |
| `allowed_to_transact_with.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `applicable_on_account`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/applicable_on_account`
Target: `src/erpnext/accounts/doctype/applicable_on_account`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `applicable_on_account`, `is_mandatory`, and child table parent fields.
- DocType metadata:
  - `name`: `Applicable On Account`
  - `module`: `Accounts`
  - `istable`: enabled
  - `field_order`: `applicable_on_account`, `is_mandatory`
  - `applicable_on_account`: `Link`, label `Accounts`, options `Account`, `reqd: 1`, `in_list_view: 1`
  - `is_mandatory`: `Check`, label `Is Mandatory`, default `0`, columns `2`, `in_list_view: 1`
  - `quick_entry`, `index_web_pages_for_search`, and `track_changes` are enabled.

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `applicable_on_account.py` | `applicable_on_account.rs` | parity_tested | No-op child table controller and account/check fields represented in Rust. |
| `applicable_on_account.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `accounting_dimension_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/accounting_dimension_detail`
Target: `src/erpnext/accounts/doctype/accounting_dimension_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes company, reference/default dimension links, check fields, offsetting account, and child table parent fields.
- DocType metadata:
  - `name`: `Accounting Dimension Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `field_order`: `company`, `reference_document`, `default_dimension`, `mandatory_for_bs`, `mandatory_for_pl`, `column_break_lqns`, `automatically_post_balancing_accounting_entry`, `offsetting_account`
  - `company`: `Link`, label `Company`, options `Company`, columns `2`, `in_list_view: 1`
  - `reference_document`: `Link`, label `Reference Document`, options `DocType`, `hidden: 1`, `read_only: 1`
  - `default_dimension`: `Dynamic Link`, label `Default Dimension`, options `reference_document`, columns `2`, `in_list_view: 1`
  - `mandatory_for_bs`: `Check`, label `Mandatory For Balance Sheet`, default `0`, columns `3`, `in_list_view: 1`
  - `mandatory_for_pl`: `Check`, label `Mandatory For Profit and Loss Account`, default `0`, columns `3`, `in_list_view: 1`
  - `automatically_post_balancing_accounting_entry`: `Check`, default `0`
  - `offsetting_account`: `Link`, label `Offsetting Account`, options `Account`, `mandatory_depends_on: eval: doc.automatically_post_balancing_accounting_entry`
  - `column_break_lqns`: `Column Break`
  - `track_changes` is enabled.

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `accounting_dimension_detail.py` | `accounting_dimension_detail.rs` | parity_tested | No-op child table controller and metadata fields represented in Rust. |
| `accounting_dimension_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
