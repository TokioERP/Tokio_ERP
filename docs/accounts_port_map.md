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
| `bank_clearance_detail` | 4 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_bank_clearance_detail`. JSON kept external. |
| `bank_guarantee` | 5 | 3 | 1 | 1 | not_started | |
| `bank_reconciliation_tool` | 5 | 3 | 1 | 1 | not_started | |
| `bank_statement_import` | 7 | 3 | 1 | 2 | not_started | |
| `bank_transaction` | 10 | 7 | 1 | 2 | not_started | |
| `bank_transaction_mapping` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_bank_transaction_mapping`. JSON kept external. |
| `bank_transaction_payments` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_bank_transaction_payments`. JSON kept external. |
| `bisect_accounting_statements` | 5 | 3 | 1 | 1 | not_started | |
| `bisect_nodes` | 5 | 3 | 1 | 1 | not_started | |
| `budget` | 5 | 3 | 1 | 1 | not_started | |
| `budget_account` | 3 | 2 | 1 | 0 | not_started | |
| `budget_distribution` | 3 | 2 | 1 | 0 | not_started | |
| `campaign_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_campaign_item`. JSON kept external. |
| `cashier_closing` | 5 | 3 | 1 | 1 | not_started | |
| `cashier_closing_payments` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_cashier_closing_payments`. JSON kept external. |
| `chart_of_accounts_importer` | 6 | 3 | 1 | 1 | not_started | |
| `cheque_print_template` | 5 | 3 | 1 | 1 | not_started | |
| `closed_document` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_closed_document`. JSON kept external. |
| `cost_center` | 8 | 4 | 1 | 2 | not_started | |
| `cost_center_allocation` | 5 | 3 | 1 | 1 | not_started | |
| `cost_center_allocation_percentage` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_cost_center_allocation_percentage`. JSON kept external. |
| `coupon_code` | 5 | 3 | 1 | 1 | not_started | |
| `currency_exchange_settings` | 5 | 3 | 1 | 1 | not_started | |
| `currency_exchange_settings_details` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_currency_exchange_settings_details`. JSON kept external. |
| `currency_exchange_settings_result` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_currency_exchange_settings_result`. JSON kept external. |
| `customer_group_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_customer_group_item`. JSON kept external. |
| `customer_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_customer_item`. JSON kept external. |
| `discounted_invoice` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_discounted_invoice`. JSON kept external. |
| `dunning` | 6 | 3 | 1 | 2 | not_started | |
| `dunning_letter_text` | 3 | 2 | 1 | 0 | not_started | |
| `dunning_type` | 5 | 3 | 1 | 1 | not_started | |
| `exchange_rate_revaluation` | 6 | 4 | 1 | 1 | not_started | |
| `exchange_rate_revaluation_account` | 3 | 2 | 1 | 0 | not_started | |
| `finance_book` | 6 | 4 | 1 | 1 | not_started | |
| `financial_report_row` | 3 | 2 | 1 | 0 | not_started | |
| `financial_report_template` | 8 | 6 | 1 | 1 | not_started | |
| `fiscal_year` | 7 | 4 | 1 | 1 | not_started | |
| `fiscal_year_company` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_fiscal_year_company`. JSON kept external. |
| `gl_entry` | 6 | 3 | 1 | 1 | not_started | |
| `invoice_discounting` | 7 | 4 | 1 | 2 | not_started | |
| `item_tax_template` | 6 | 4 | 1 | 1 | not_started | |
| `item_tax_template_detail` | 3 | 2 | 1 | 0 | not_started | |
| `item_wise_tax_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_item_wise_tax_detail`. JSON kept external. |
| `journal_entry` | 8 | 3 | 2 | 2 | not_started | |
| `journal_entry_account` | 4 | 2 | 1 | 0 | not_started | |
| `journal_entry_template` | 5 | 3 | 1 | 1 | not_started | |
| `journal_entry_template_account` | 3 | 2 | 1 | 0 | not_started | |
| `ledger_health` | 5 | 3 | 1 | 1 | not_started | |
| `ledger_health_monitor` | 5 | 3 | 1 | 1 | not_started | |
| `ledger_health_monitor_company` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_ledger_health_monitor_company`. JSON kept external. |
| `ledger_merge` | 5 | 3 | 1 | 1 | not_started | |
| `ledger_merge_accounts` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_ledger_merge_accounts`. JSON kept external. |
| `loyalty_point_entry` | 5 | 3 | 1 | 1 | not_started | |
| `loyalty_point_entry_redemption` | 3 | 2 | 1 | 0 | not_started | |
| `loyalty_program` | 6 | 4 | 1 | 1 | not_started | |
| `loyalty_program_collection` | 3 | 2 | 1 | 0 | not_started | |
| `mode_of_payment` | 6 | 3 | 1 | 1 | not_started | |
| `mode_of_payment_account` | 3 | 2 | 1 | 0 | not_started | |
| `monthly_distribution` | 7 | 4 | 1 | 1 | not_started | |
| `monthly_distribution_percentage` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_monthly_distribution_percentage`. JSON kept external. |
| `opening_invoice_creation_tool` | 6 | 3 | 1 | 1 | not_started | |
| `opening_invoice_creation_tool_item` | 3 | 2 | 1 | 0 | not_started | |
| `overdue_payment` | 3 | 2 | 1 | 0 | not_started | |
| `party_account` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_party_account`. JSON kept external. |
| `party_link` | 5 | 3 | 1 | 1 | not_started | |
| `payment_entry` | 6 | 3 | 1 | 2 | not_started | |
| `payment_entry_deduction` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_payment_entry_deduction`. JSON kept external. |
| `payment_entry_reference` | 3 | 2 | 1 | 0 | not_started | |
| `payment_gateway_account` | 6 | 4 | 1 | 1 | not_started | |
| `payment_ledger_entry` | 5 | 3 | 1 | 1 | not_started | |
| `payment_order` | 6 | 4 | 1 | 1 | not_started | |
| `payment_order_reference` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reconciliation` | 5 | 3 | 1 | 1 | not_started | |
| `payment_reconciliation_allocation` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reconciliation_invoice` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reconciliation_payment` | 3 | 2 | 1 | 0 | not_started | |
| `payment_reference` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_payment_reference`. JSON kept external. |
| `payment_request` | 7 | 4 | 1 | 2 | not_started | |
| `payment_schedule` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_payment_schedule`. JSON kept external. |
| `payment_term` | 6 | 4 | 1 | 1 | parity_tested | Python controller is pass/no-op; dashboard data and empty test suite covered by `accounts_payment_term`. JSON/JS kept external. |
| `payment_terms_template` | 6 | 4 | 1 | 1 | parity_tested | Controller validation, dashboard data, and ERPNext test scenarios covered by `accounts_payment_terms_template`. JSON/JS kept external. |
| `payment_terms_template_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_payment_terms_template_detail`. JSON kept external. |
| `pegged_currencies` | 5 | 3 | 1 | 1 | not_started | |
| `pegged_currency_details` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pegged_currency_details`. JSON kept external. |
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

## Doctype Detail: `bank_clearance_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/bank_clearance_detail`
Target: `src/erpnext/accounts/doctype/bank_clearance_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes payment document/entry, account, amount, cheque, posting, and clearance date fields.
- DocType metadata:
  - `name`: `Bank Clearance Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `quick_entry`: enabled
  - `grid_page_length`: `50`
  - `row_format`: `Dynamic`
  - `field_order`: `payment_document`, `payment_entry`, `against_account`, `amount`, `column_break_5`, `posting_date`, `cheque_number`, `cheque_date`, `clearance_date`
  - `payment_document`: `Link`, label `Payment Document`, options `DocType`
  - `payment_entry`: `Dynamic Link`, label `Payment Entry`, options `payment_document`, columns `2`, `in_list_view: 1`, old field `voucher_id:Link`
  - `against_account`: `Data`, label `Against Account`, columns `2`, `in_list_view: 1`, `read_only: 1`, old field `against_account:Data`, width `15`
  - `amount`: `Data`, label `Amount`, columns `2`, `in_list_view: 1`, `read_only: 1`, old field `debit:Currency`
  - `column_break_5`: `Column Break`, width `50%`
  - `posting_date`: `Date`, label `Posting Date`, columns `2`, `read_only: 1`, old field `posting_date:Date`
  - `cheque_number`: `Data`, label `Cheque Number`, columns `1`, `in_list_view: 1`, `read_only: 1`, old field `cheque_number:Data`
  - `cheque_date`: `Date`, label `Cheque Date`, columns `2`, `in_list_view: 1`, `read_only: 1`, old field `cheque_date:Date`
  - `clearance_date`: `Date`, label `Clearance Date`, columns `2`, `in_list_view: 1`, old field `clearance_date:Date`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `bank_clearance_detail.py` | `bank_clearance_detail.rs` | parity_tested | No-op child table controller and payment/date metadata represented in Rust. |
| `bank_clearance_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `bank_transaction_mapping`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/bank_transaction_mapping`
Target: `src/erpnext/accounts/doctype/bank_transaction_mapping`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `bank_transaction_field`, `file_field`, and child table parent fields.
- DocType metadata:
  - `name`: `Bank Transaction Mapping`
  - `module`: `Accounts`
  - `istable`: enabled
  - `field_order`: `bank_transaction_field`, `file_field`
  - `bank_transaction_field`: `Select`, label `Field in Bank Transaction`, `reqd: 1`, `in_list_view: 1`
  - `file_field`: `Data`, label `Column in Bank File`, `reqd: 1`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `bank_transaction_mapping.py` | `bank_transaction_mapping.rs` | parity_tested | No-op child table controller and required mapping fields represented in Rust. |
| `bank_transaction_mapping.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `bank_transaction_payments`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/bank_transaction_payments`
Target: `src/erpnext/accounts/doctype/bank_transaction_payments`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required payment document/entry, allocated amount, clearance date, and child table parent fields.
- DocType metadata:
  - `name`: `Bank Transaction Payments`
  - `module`: `Accounts`
  - `istable`: enabled
  - `quick_entry`: enabled
  - `track_changes`: enabled
  - `field_order`: `payment_document`, `payment_entry`, `allocated_amount`, `clearance_date`
  - `payment_document`: `Link`, label `Payment Document`, options `DocType`, `reqd: 1`, `in_list_view: 1`
  - `payment_entry`: `Dynamic Link`, label `Payment Entry`, options `payment_document`, `reqd: 1`, `in_list_view: 1`
  - `allocated_amount`: `Currency`, label `Allocated Amount`, `reqd: 1`, `in_list_view: 1`
  - `clearance_date`: `Date`, label `Clearance Date`, `depends_on: eval:doc.docstatus==1`, `no_copy: 1`, `print_hide: 1`, `read_only: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `bank_transaction_payments.py` | `bank_transaction_payments.rs` | parity_tested | No-op child table controller and payment allocation metadata represented in Rust. |
| `bank_transaction_payments.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `campaign_item`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/campaign_item`
Target: `src/erpnext/accounts/doctype/campaign_item`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes optional `campaign` and child table parent fields.
- DocType metadata:
  - `name`: `Campaign Item`
  - `module`: `Accounts`
  - `istable`: enabled
  - `index_web_pages_for_search`: enabled
  - `track_changes`: enabled
  - `field_order`: `campaign`
  - `campaign`: `Link`, label `Campaign`, options `UTM Campaign`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `campaign_item.py` | `campaign_item.rs` | parity_tested | No-op child table controller and campaign link metadata represented in Rust. |
| `campaign_item.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `cashier_closing_payments`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/cashier_closing_payments`
Target: `src/erpnext/accounts/doctype/cashier_closing_payments`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `mode_of_payment`, `amount`, and child table parent fields.
- DocType metadata:
  - `name`: `Cashier Closing Payments`
  - `module`: `Accounts`
  - `istable`: enabled
  - `quick_entry`: enabled
  - `track_changes`: enabled
  - `field_order`: `mode_of_payment`, `amount`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`, `reqd: 1`, `in_list_view: 1`, `in_filter: 1`, `in_standard_filter: 1`
  - `amount`: `Float`, label `Amount`, default `0.00`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `cashier_closing_payments.py` | `cashier_closing_payments.rs` | parity_tested | No-op child table controller and payment summary metadata represented in Rust. |
| `cashier_closing_payments.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `closed_document`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/closed_document`
Target: `src/erpnext/accounts/doctype/closed_document`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `document_type`, `closed`, and child table parent fields.
- DocType metadata:
  - `name`: `Closed Document`
  - `module`: `Accounts`
  - `istable`: enabled
  - `quick_entry`: enabled
  - `track_changes`: enabled
  - `field_order`: `document_type`, `closed`
  - `document_type`: `Link`, label `Document Type`, options `DocType`, `reqd: 1`, `in_list_view: 1`
  - `closed`: `Check`, label `Closed`, default `0`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `closed_document.py` | `closed_document.rs` | parity_tested | No-op child table controller and closed-document metadata represented in Rust. |
| `closed_document.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `cost_center_allocation_percentage`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/cost_center_allocation_percentage`
Target: `src/erpnext/accounts/doctype/cost_center_allocation_percentage`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `cost_center`, `percentage`, and child table parent fields.
- DocType metadata:
  - `name`: `Cost Center Allocation Percentage`
  - `module`: `Accounts`
  - `istable`: enabled
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `cost_center`, `percentage`
  - `cost_center`: `Link`, label `Cost Center`, options `Cost Center`, `reqd: 1`, `in_list_view: 1`
  - `percentage`: `Percent`, label `Percentage (%)`, `reqd: 1`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `cost_center_allocation_percentage.py` | `cost_center_allocation_percentage.rs` | parity_tested | No-op child table controller and cost-center percentage metadata represented in Rust. |
| `cost_center_allocation_percentage.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `currency_exchange_settings_details`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/currency_exchange_settings_details`
Target: `src/erpnext/accounts/doctype/currency_exchange_settings_details`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `key`, `value`, and child table parent fields.
- DocType metadata:
  - `name`: `Currency Exchange Settings Details`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `track_changes`: enabled
  - `field_order`: `key`, `value`
  - `key`: `Data`, label `Key`, `reqd: 1`, `in_list_view: 1`
  - `value`: `Data`, label `Value`, `reqd: 1`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `currency_exchange_settings_details.py` | `currency_exchange_settings_details.rs` | parity_tested | No-op child table controller and key/value metadata represented in Rust. |
| `currency_exchange_settings_details.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `currency_exchange_settings_result`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/currency_exchange_settings_result`
Target: `src/erpnext/accounts/doctype/currency_exchange_settings_result`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `key` and child table parent fields.
- DocType metadata:
  - `name`: `Currency Exchange Settings Result`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `track_changes`: enabled
  - `field_order`: `key`
  - `key`: `Data`, label `Key`, `reqd: 1`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `currency_exchange_settings_result.py` | `currency_exchange_settings_result.rs` | parity_tested | No-op child table controller and key metadata represented in Rust. |
| `currency_exchange_settings_result.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `customer_group_item`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/customer_group_item`
Target: `src/erpnext/accounts/doctype/customer_group_item`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes optional `customer_group` and child table parent fields.
- DocType metadata:
  - `name`: `Customer Group Item`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `track_changes`: enabled
  - `field_order`: `customer_group`
  - `customer_group`: `Link`, label `Customer Group`, options `Customer Group`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `customer_group_item.py` | `customer_group_item.rs` | parity_tested | No-op child table controller and customer-group metadata represented in Rust. |
| `customer_group_item.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `customer_item`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/customer_item`
Target: `src/erpnext/accounts/doctype/customer_item`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes optional `customer` and child table parent fields.
- DocType metadata:
  - `name`: `Customer Item`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `track_changes`: enabled
  - `field_order`: `customer`
  - `customer`: `Link`, label `Customer `, options `Customer`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `customer_item.py` | `customer_item.rs` | parity_tested | No-op child table controller and customer metadata represented in Rust. |
| `customer_item.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `discounted_invoice`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/discounted_invoice`
Target: `src/erpnext/accounts/doctype/discounted_invoice`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `sales_invoice`, invoice-derived fields, and child table parent fields.
- DocType metadata:
  - `name`: `Discounted Invoice`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `quick_entry`: enabled
  - `track_changes`: enabled
  - `field_order`: `sales_invoice`, `customer`, `column_break_3`, `posting_date`, `outstanding_amount`, `debit_to`
  - `sales_invoice`: `Link`, label `Invoice`, options `Sales Invoice`, `reqd: 1`, `in_list_view: 1`, `search_index: 1`
  - `customer`: `Link`, label `Customer`, options `Customer`, fetches `sales_invoice.customer`, `read_only: 1`, `in_list_view: 1`
  - `column_break_3`: `Column Break`
  - `posting_date`: `Date`, label `Date`, fetches `sales_invoice.posting_date`, `read_only: 1`, `in_list_view: 1`
  - `outstanding_amount`: `Currency`, label `Outstanding Amount`, options `Company:company:default_currency`, fetches `sales_invoice.outstanding_amount`, `fetch_if_empty: 1`, `in_list_view: 1`
  - `debit_to`: `Link`, label `Debit to`, options `Account`, fetches `sales_invoice.debit_to`, `read_only: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `discounted_invoice.py` | `discounted_invoice.rs` | parity_tested | No-op child table controller and discounted invoice metadata represented in Rust. |
| `discounted_invoice.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `fiscal_year_company`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/fiscal_year_company`
Target: `src/erpnext/accounts/doctype/fiscal_year_company`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `company` and child table parent fields.
- DocType metadata:
  - `name`: `Fiscal Year Company`
  - `module`: `Accounts`
  - `document_type`: `Setup`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `track_changes`: enabled
  - `row_format`: `Dynamic`
  - `field_order`: `company`
  - `company`: `Link`, label `Company`, options `Company`, `reqd: 1`, `ignore_user_permissions: 1`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `fiscal_year_company.py` | `fiscal_year_company.rs` | parity_tested | No-op child table controller and company link metadata represented in Rust. |
| `fiscal_year_company.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `item_wise_tax_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/item_wise_tax_detail`
Target: `src/erpnext/accounts/doctype/item_wise_tax_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes item/tax rows, rate, amount, taxable amount, and child table parent fields.
- DocType metadata:
  - `name`: `Item Wise Tax Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `index_web_pages_for_search`: enabled
  - `grid_page_length`: `50`
  - `row_format`: `Dynamic`
  - `field_order`: `item_row`, `tax_row`, `rate`, `amount`, `taxable_amount`
  - `item_row`: `Data`, label `Item Row`, `reqd: 1`, `in_list_view: 1`
  - `tax_row`: `Data`, label `Tax Row`, `reqd: 1`, `in_list_view: 1`
  - `rate`: `Float`, label `Tax Rate`, `in_list_view: 1`
  - `amount`: `Currency`, label `Tax Amount`, options `Company:company:default_currency`, `in_list_view: 1`
  - `taxable_amount`: `Currency`, label `Taxable Amount`, options `Company:company:default_currency`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `item_wise_tax_detail.py` | `item_wise_tax_detail.rs` | parity_tested | No-op child table controller and item-wise tax metadata represented in Rust. |
| `item_wise_tax_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `ledger_health_monitor_company`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/ledger_health_monitor_company`
Target: `src/erpnext/accounts/doctype/ledger_health_monitor_company`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes optional `company` and child table parent fields.
- DocType metadata:
  - `name`: `Ledger Health Monitor Company`
  - `module`: `Accounts`
  - `istable`: enabled
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `company`
  - `company`: `Link`, label `Company`, options `Company`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `ledger_health_monitor_company.py` | `ledger_health_monitor_company.rs` | parity_tested | No-op child table controller and company metadata represented in Rust. |
| `ledger_health_monitor_company.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `ledger_merge_accounts`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/ledger_merge_accounts`
Target: `src/erpnext/accounts/doctype/ledger_merge_accounts`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `account`, required `account_name`, `merged`, and child table parent fields.
- DocType metadata:
  - `name`: `Ledger Merge Accounts`
  - `module`: `Accounts`
  - `istable`: enabled
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `account`, `account_name`, `merged`
  - `account`: `Link`, label `Account`, options `Account`, columns `4`, `reqd: 1`, `in_list_view: 1`
  - `account_name`: `Data`, label `Account Name`, columns `4`, `reqd: 1`, `read_only: 1`
  - `merged`: `Check`, label `Merged`, columns `2`, default `0`, `read_only: 1`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `ledger_merge_accounts.py` | `ledger_merge_accounts.rs` | parity_tested | No-op child table controller and ledger merge account metadata represented in Rust. |
| `ledger_merge_accounts.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `monthly_distribution_percentage`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/monthly_distribution_percentage`
Target: `src/erpnext/accounts/doctype/monthly_distribution_percentage`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `month`, `percentage_allocation`, and child table parent fields.
- DocType metadata:
  - `name`: `Monthly Distribution Percentage`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `autoname`: `hash`
  - `idx`: `1`
  - `field_order`: `month`, `percentage_allocation`
  - `month`: `Data`, label `Month`, `reqd: 1`, `read_only: 1`, `in_list_view: 1`, old field `month:Data`
  - `percentage_allocation`: `Float`, label `Percentage Allocation`, `in_list_view: 1`, old field `percentage_allocation:Currency`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `monthly_distribution_percentage.py` | `monthly_distribution_percentage.rs` | parity_tested | No-op child table controller and monthly allocation metadata represented in Rust. |
| `monthly_distribution_percentage.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `party_account`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/party_account`
Target: `src/erpnext/accounts/doctype/party_account`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required `company`, optional default/advance accounts, and child table parent fields.
- DocType metadata:
  - `name`: `Party Account`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `quick_entry`: enabled
  - `field_order`: `company`, `account`, `advance_account`
  - `company`: `Link`, label `Company`, options `Company`, `reqd: 1`, `ignore_user_permissions: 1`, `in_list_view: 1`
  - `account`: `Link`, label `Default Account`, options `Account`, `in_list_view: 1`
  - `advance_account`: `Link`, label `Advance Account`, options `Account`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `party_account.py` | `party_account.rs` | parity_tested | No-op child table controller and party account metadata represented in Rust. |
| `party_account.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `payment_entry_deduction`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/payment_entry_deduction`
Target: `src/erpnext/accounts/doctype/payment_entry_deduction`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes required account, cost center, amount, exchange gain/loss flag, optional description, and child table parent fields.
- DocType metadata:
  - `name`: `Payment Entry Deduction`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `quick_entry`: enabled
  - `row_format`: `Dynamic`
  - `field_order`: `account`, `cost_center`, `amount`, `column_break_2`, `is_exchange_gain_loss`, `description`
  - `account`: `Link`, label `Account`, options `Account`, `reqd: 1`, `in_list_view: 1`
  - `cost_center`: `Link`, label `Cost Center`, options `Cost Center`, `reqd: 1`, `allow_on_submit: 1`, `print_hide: 1`, `in_list_view: 1`
  - `amount`: `Currency`, label `Amount (Company Currency)`, options `Company:company:default_currency`, `reqd: 1`, `in_list_view: 1`
  - `column_break_2`: `Column Break`
  - `is_exchange_gain_loss`: `Check`, label `Is Exchange Gain / Loss?`, default `0`, depends on `eval:doc.is_exchange_gain_loss`, `read_only: 1`
  - `description`: `Small Text`, label `Description`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `payment_entry_deduction.py` | `payment_entry_deduction.rs` | parity_tested | No-op child table controller and deduction metadata represented in Rust. |
| `payment_entry_deduction.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `payment_reference`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/payment_reference`
Target: `src/erpnext/accounts/doctype/payment_reference`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes payment term/schedule, description, due date, amount, and child table parent fields.
- DocType metadata:
  - `name`: `Payment Reference`
  - `module`: `Accounts`
  - `istable`: enabled
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `grid_page_length`: `50`
  - `row_format`: `Dynamic`
  - `rows_threshold_for_grid_search`: `20`
  - `field_order`: `payment_term`, `column_break_lnjp`, `payment_schedule`, `section_break_fjhh`, `description`, `section_break_mjlv`, `due_date`, `column_break_qghl`, `amount`
  - `payment_term`: `Link`, label `Payment Term`, options `Payment Term`, `in_list_view: 1`
  - `column_break_lnjp`: `Column Break`
  - `payment_schedule`: `Link`, label `Payment Schedule`, options `Payment Schedule`, `allow_on_submit: 1`, `read_only: 1`
  - `section_break_fjhh`: `Section Break`, label `Description`, `collapsible: 1`
  - `description`: `Small Text`, label `Description`, `in_list_view: 1`
  - `section_break_mjlv`: `Section Break`
  - `due_date`: `Date`, label `Due Date`, `in_list_view: 1`
  - `column_break_qghl`: `Column Break`
  - `amount`: `Currency`, label `Amount`, precision `2`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `payment_reference.py` | `payment_reference.rs` | parity_tested | No-op child table controller and payment reference metadata represented in Rust. |
| `payment_reference.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `payment_schedule`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/payment_schedule`
Target: `src/erpnext/accounts/doctype/payment_schedule`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes payment term, due date, credit/discount fields, payment amounts, base currency amounts, and child table parent fields.
- DocType metadata:
  - `name`: `Payment Schedule`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `row_format`: `Dynamic`
  - `field_order`: `payment_term`, `section_break_15`, `description`, `section_break_4`, `due_date`, `invoice_portion`, `mode_of_payment`, `column_break_5`, `due_date_based_on`, `credit_days`, `credit_months`, `section_break_6`, `discount_date`, `discount`, `discount_type`, `column_break_9`, `discount_validity_based_on`, `discount_validity`, `section_break_9`, `payment_amount`, `outstanding`, `paid_amount`, `discounted_amount`, `column_break_3`, `base_payment_amount`, `base_outstanding`, `base_paid_amount`
  - `payment_term`: `Link`, label `Payment Term`, options `Payment Term`, `in_list_view: 1`, columns `2`
  - `section_break_15`: `Section Break`, label `Description`
  - `description`: `Small Text`, label `Description`, `in_list_view: 1`, columns `2`
  - `section_break_4`: `Section Break`
  - `due_date`: `Date`, label `Due Date`, required, `in_list_view: 1`, columns `2`
  - `invoice_portion`: `Percent`, label `Invoice Portion`, `in_list_view: 1`, columns `2`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`
  - `column_break_5`: `Column Break`
  - `due_date_based_on`: `Select`, read-only, ERPNext due-date basis options retained exactly
  - `credit_days`: `Int`, read-only, ERPNext `depends_on` expression retained exactly
  - `credit_months`: `Int`, read-only, ERPNext `depends_on` expression retained exactly
  - `section_break_6`: `Section Break`
  - `discount_date`: `Date`, label `Discount Date`, `depends_on: discount`
  - `discount`: `Float`, label `Discount`
  - `discount_type`: `Select`, options `Percentage`/`Amount`, default `Percentage`
  - `column_break_9`: `Column Break`
  - `discount_validity_based_on`: `Select`, read-only, `depends_on: discount`, ERPNext discount basis options retained exactly
  - `discount_validity`: `Int`, read-only, `depends_on: discount_validity_based_on`
  - `section_break_9`: `Section Break`
  - `payment_amount`: `Currency`, label `Payment Amount`, options `currency`, required, `in_list_view: 1`, columns `2`
  - `outstanding`: `Currency`, label `Outstanding`, options `currency`, read-only
  - `paid_amount`: `Currency`, label `Paid Amount`, options `currency`, `depends_on: paid_amount`
  - `discounted_amount`: `Currency`, label `Discounted Amount`, read-only, default `0`, `depends_on: discounted_amount`
  - `column_break_3`: `Column Break`
  - `base_payment_amount`: `Currency`, label `Payment Amount (Company Currency)`, options `Company:company:default_currency`
  - `base_outstanding`: `Currency`, label `Outstanding (Company Currency)`, options `Company:company:default_currency`, read-only
  - `base_paid_amount`: `Currency`, label `Paid Amount (Company Currency)`, options `Company:company:default_currency`, read-only, `depends_on: base_paid_amount`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `payment_schedule.py` | `payment_schedule.rs` | parity_tested | No-op child table controller and payment schedule metadata represented in Rust. |
| `payment_schedule.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `payment_term`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/payment_term`
Target: `src/erpnext/accounts/doctype/payment_term`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- `payment_term_dashboard.get_data()` returns static dashboard data for Sales, Purchase, and Payment Terms Template transactions.
- `test_payment_term.py` defines an empty `ERPNextTestSuite` subclass; Rust parity test covers metadata, no-op controller behavior, and dashboard data.
- DocType metadata:
  - `name`: `Payment Term`
  - `module`: `Accounts`
  - `autoname`: `field:payment_term_name`
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `field_order`: `payment_term_name`, `invoice_portion`, `mode_of_payment`, `column_break_3`, `due_date_based_on`, `credit_days`, `credit_months`, `section_break_8`, `discount_type`, `discount`, `column_break_11`, `discount_validity_based_on`, `discount_validity`, `section_break_6`, `description`
  - `payment_term_name`: `Data`, label `Payment Term Name`, `unique: 1`
  - `invoice_portion`: `Float`, label `Invoice Portion (%)`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`
  - `column_break_3`: `Column Break`
  - `due_date_based_on`: `Select`, ERPNext due-date basis options retained exactly
  - `credit_days`: `Int`, ERPNext `depends_on` expression retained exactly
  - `credit_months`: `Int`, ERPNext `depends_on` expression retained exactly
  - `section_break_8`: `Section Break`, label `Discount Settings`
  - `discount_type`: `Select`, options `Percentage`/`Amount`, default `Percentage`
  - `discount`: `Float`, label `Discount`
  - `column_break_11`: `Column Break`
  - `discount_validity_based_on`: `Select`, ERPNext discount basis options retained exactly, default `Day(s) after invoice date`, `depends_on: discount`
  - `discount_validity`: `Int`, `depends_on: discount`
  - `section_break_6`: `Section Break`
  - `description`: `Small Text`, label `Description`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `payment_term.py` | `payment_term.rs` | parity_tested | No-op controller and payment term metadata represented in Rust. |
| `payment_term_dashboard.py` | `payment_term_dashboard.rs` | parity_tested | Static dashboard `get_data()` transactions represented in Rust. |
| `test_payment_term.py` | `tests/accounts_payment_term.rs` | parity_tested | Empty ERPNext test replaced by Rust metadata/controller/dashboard parity tests. |
| `payment_term.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `payment_term.js` | ERPNext UI retained | external_kept | Dynamic discount field description remains UI-side. |

## Doctype Detail: `payment_terms_template`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/payment_terms_template`
Target: `src/erpnext/accounts/doctype/payment_terms_template`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `validate()` calls `validate_invoice_portion()` and then `validate_terms()`.
- `validate_invoice_portion()` sums `terms[].invoice_portion` with Frappe-style `flt`; rounded total at precision `2` must equal `100.00`.
- If invoice portions do not equal `100%`, validation raises `Combined invoice portion must equal 100%` with red indicator.
- `validate_terms()` enforces `payment_term` when `allocate_payment_based_on_payment_terms` is enabled.
- `validate_terms()` rejects duplicate `(payment_term, credit_days, credit_months, due_date_based_on)` tuples and reports the duplicate row.
- `payment_terms_template_dashboard.get_data()` returns static non-standard fieldname mapping and Sales/Purchase/Party/Group transactions.
- `test_payment_terms_template.py` create/invalid/duplicate scenarios are represented by Rust validation tests.
- DocType metadata:
  - `name`: `Payment Terms Template`
  - `module`: `Accounts`
  - `autoname`: `field:template_name`
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `field_order`: `template_name`, `allocate_payment_based_on_payment_terms`, `terms`
  - `template_name`: `Data`, label `Template Name`, `unique: 1`
  - `allocate_payment_based_on_payment_terms`: `Check`, label `Allocate Payment Based On Payment Terms`, default `0`
  - `terms`: `Table`, label `Payment Terms`, options `Payment Terms Template Detail`, required

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `payment_terms_template.py` | `payment_terms_template.rs` | parity_tested | Controller validation and payment terms template metadata represented in Rust. |
| `payment_terms_template_dashboard.py` | `payment_terms_template_dashboard.rs` | parity_tested | Static dashboard `get_data()` mapping and transactions represented in Rust. |
| `test_payment_terms_template.py` | `tests/accounts_payment_terms_template.rs` | parity_tested | ERPNext create/invalid/duplicate scenarios represented by Rust validation tests. |
| `payment_terms_template.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `payment_terms_template.js` | ERPNext UI retained | external_kept | Child table client-side field copy behavior remains UI-side. |

## Doctype Detail: `payment_terms_template_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/payment_terms_template_detail`
Target: `src/erpnext/accounts/doctype/payment_terms_template_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes payment term, invoice portion, due date basis, credit/discount fields, mode of payment, description, and child table parent fields.
- DocType metadata:
  - `name`: `Payment Terms Template Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `payment_term`, `section_break_13`, `description`, `section_break_4`, `invoice_portion`, `mode_of_payment`, `column_break_3`, `due_date_based_on`, `credit_days`, `credit_months`, `section_break_8`, `discount_type`, `discount`, `column_break_11`, `discount_validity_based_on`, `discount_validity`
  - `payment_term`: `Link`, label `Payment Term`, options `Payment Term`, `in_list_view: 1`, columns `2`
  - `section_break_13`: `Section Break`, label `Description`
  - `description`: `Small Text`, label `Description`, `in_list_view: 1`, columns `2`
  - `section_break_4`: `Section Break`
  - `invoice_portion`: `Float`, label `Invoice Portion (%)`, required, `in_list_view: 1`, columns `2`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`
  - `column_break_3`: `Column Break`
  - `due_date_based_on`: `Select`, ERPNext due-date basis options retained exactly, required, `in_list_view: 1`, columns `2`
  - `credit_days`: `Int`, default `0`, ERPNext `depends_on` expression retained exactly, `in_list_view: 1`, columns `2`
  - `credit_months`: `Int`, default `0`, ERPNext `depends_on` expression retained exactly
  - `section_break_8`: `Section Break`, label `Discount Settings`
  - `discount_type`: `Select`, options `Percentage`/`Amount`, default `Percentage`
  - `discount`: `Float`, label `Discount`
  - `column_break_11`: `Column Break`
  - `discount_validity_based_on`: `Select`, ERPNext discount basis options retained exactly, default `Day(s) after invoice date`, `depends_on: discount`
  - `discount_validity`: `Int`, `depends_on: discount`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `payment_terms_template_detail.py` | `payment_terms_template_detail.rs` | parity_tested | No-op child table controller and payment terms template detail metadata represented in Rust. |
| `payment_terms_template_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pegged_currency_details`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pegged_currency_details`
Target: `src/erpnext/accounts/doctype/pegged_currency_details`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes source currency, pegged-against currency, pegged exchange rate, and child table parent fields.
- DocType metadata:
  - `name`: `Pegged Currency Details`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `source_currency`, `pegged_against`, `pegged_exchange_rate`
  - `source_currency`: `Link`, label `Currency`, options `Currency`, `in_list_view: 1`
  - `pegged_against`: `Link`, label `Pegged Against`, options `Currency`, `in_list_view: 1`
  - `pegged_exchange_rate`: `Data`, label `Exchange Rate`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pegged_currency_details.py` | `pegged_currency_details.rs` | parity_tested | No-op child table controller and pegged currency detail metadata represented in Rust. |
| `pegged_currency_details.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
