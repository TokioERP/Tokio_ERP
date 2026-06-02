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
| `doctype` | 191 | 923 | 504 | 282 | 111 | mapped | Multiple controllers are parity-tested, including `account`, `bank_account_subtype`, `pos_profile`, `opening_invoice_creation_tool`, and `pos_closing_entry`; remaining doctypes pending. |
| `financial_report_template` | 7 | 14 | 7 | 7 | 0 | parity_tested | Python init files are empty/no-op; Rust preserves template registry names, modules, report types, row counts, and exact source folder paths where needed in `accounts_financial_report_template`. JSON kept external. |
| `letterhead` | 1 | 2 | 0 | 0 | 0 | not_started | |
| `module_onboarding` | 2 | 1 | 0 | 1 | 0 | not_started | |
| `notification` | 2 | 5 | 3 | 1 | 0 | parity_tested | `notification_for_new_fiscal_year.py` is no-op context; Rust preserves notification metadata/template constants and context behavior in `accounts_notification_for_new_fiscal_year`. JSON/HTML kept external. |
| `number_card` | 5 | 4 | 0 | 4 | 0 | not_started | |
| `onboarding_step` | 7 | 6 | 0 | 6 | 0 | not_started | |
| `page` | 1 | 1 | 1 | 0 | 0 | not_started | |
| `print_format` | 26 | 57 | 26 | 25 | 0 | parity_tested | Python init files are empty/no-op; Rust preserves 25 static print format names, folders, report/doc type targets, format type, and standard flags in `accounts_print_static_formats`. JSON/HTML kept external. |
| `print_format_field_template` | 3 | 5 | 3 | 2 | 0 | parity_tested | Python init files are empty/no-op; Rust preserves field template names, document types, field names, template file paths, and standard flags in `accounts_print_static_formats`. JSON kept external. |
| `report` | 53 | 244 | 128 | 52 | 50 | not_started | |
| `test` | 1 | 4 | 4 | 0 | 0 | mapped | `accounts_mixin.py` parity-tested; remaining test helpers pending. |
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
| `account` | 106 | 9 | 94 | 2 | parity_tested | Rust covers metadata, account validation, parent/root/type/currency/freeze/default-account guards, conversion plans, child-company account propagation plans, autoname/currency/search helpers, account-number update and merge planning in `accounts_account`. JSON/JS/chart metadata kept external. |
| `account_category` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, rename formula update planning, Python-literal formula parsing parity with `ast.literal_eval`, account category JSON import path and bulk-create de-duplication in `accounts_account_category`. JSON/JS kept external. |
| `account_closing_balance` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, closing entry aggregation with dynamic accounting dimensions, previous closing-entry query planning, voucher/date stamping, and reporting-currency amount/error behavior in `accounts_account_closing_balance`. JSON/JS kept external. |
| `accounting_dimension` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, fieldname/doctype/default-company validation, conflict warnings, dimension custom-field/property-setter plans, delete/toggle side effects, dimension/default helpers, child expansion, and doctype custom-field creation plans in `accounts_accounting_dimension`. JSON/JS kept external. |
| `accounting_dimension_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_accounting_dimension_detail`. JSON kept external. |
| `accounting_dimension_filter` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, `before_save`, fieldname derivation, duplicate applicable-account validation, and dimension filter map construction in `accounts_accounting_dimension_filter`. JSON/JS kept external; Sales Invoice enforcement is covered through downstream accounting dimension consumers. |
| `accounting_period` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, company-abbr autoname, overlap error planning, closing doctype bootstrap, and closed-period save guard including Bank Clearance/Asset bypasses and exempted-role behavior in `accounts_accounting_period`. JSON/JS kept external. |
| `accounts_settings` | 6 | 3 | 1 | 2 | parity_tested | Rust covers core validation and side-effect planning: auto-tax conflict, stale days, default updates, payment schedule print setters, section toggles, auto-reconcile ranges, repost allow-on-submit updates, and doctype set constants in `accounts_accounts_settings`. Large JSON/JS UI metadata kept external. |
| `advance_payment_ledger_entry` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, read-only ledger fields, `on_update` outstanding-update gating, and doctype index planning in `accounts_loyalty_and_advance_ledger`. JSON/JS kept external; payment/journal integration remains downstream. |
| `advance_taxes_and_charges` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust mirrors child-table metadata fields and read-only/accounting dimension tax rows in `accounts_payment_child_pass_doctypes`. JSON kept external. |
| `allowed_dimension` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_allowed_dimension`. JSON kept external. |
| `allowed_to_transact_with` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_allowed_to_transact_with`. JSON kept external. |
| `applicable_on_account` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_applicable_on_account`. JSON kept external. |
| `bank` | 6 | 4 | 1 | 1 | parity_tested | Rust covers bank metadata, setup/sort flags, address/contact lifecycle hooks, dashboard metadata, and no-op test parity in `accounts_bank`. JSON/JS kept external. |
| `bank_account` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, setup/search/sort flags, address/contact hooks, autoname, company-account validation, duplicate account guard, default-account reset planning, party/company lookup helpers, and details permission/cache plan in `accounts_bank_account`. JSON/JS kept external. |
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
| `opening_invoice_creation_tool` | 6 | 3 | 1 | 1 | parity_tested | Rust covers onload summary helpers, row defaults, mandatory party validation, party creation plans, invoice dict generation, sync/enqueue/no-op import planning, realtime payloads, and temporary opening account lookup in `accounts_opening_invoice_creation_tool`. JSON/JS kept external. |
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
| `pegged_currencies` | 5 | 3 | 1 | 1 | parity_tested | Python controller/test are no-op; Rust metadata and controller behavior covered by `accounts_pegged_currencies`. JSON/JS kept external. |
| `pegged_currency_details` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pegged_currency_details`. JSON kept external. |
| `period_closing_voucher` | 6 | 3 | 1 | 1 | not_started | |
| `pos_closing_entry` | 7 | 3 | 1 | 2 | parity_tested | Rust covers metadata, validation rules, lifecycle/update plans, cashier helper, invoice query planning, invoice aggregation, payment and tax summaries, and opening-entry-to-closing draft creation in `accounts_pos_closing_entry`. JSON/JS kept external. |
| `pos_closing_entry_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_closing_entry_detail`. JSON kept external. |
| `pos_closing_entry_taxes` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_closing_entry_taxes`. JSON kept external. |
| `pos_customer_group` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_customer_group`. JSON kept external. |
| `pos_field` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_field`. JSON kept external. |
| `pos_invoice` | 7 | 4 | 1 | 2 | not_started | |
| `pos_invoice_item` | 3 | 2 | 1 | 0 | not_started | |
| `pos_invoice_merge_log` | 5 | 3 | 1 | 1 | not_started | |
| `pos_invoice_reference` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_invoice_reference`. JSON kept external. |
| `pos_item_group` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_item_group`. JSON kept external. |
| `pos_opening_entry` | 6 | 3 | 1 | 2 | not_started | |
| `pos_opening_entry_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_opening_entry_detail`. JSON kept external. |
| `pos_payment_method` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_payment_method`. JSON kept external. |
| `pos_profile` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, disabled/default-profile validation, company link checks, duplicate groups, payment method rules, accounting dimension checks, defaults, item-group permission helpers, query fallback, and default-profile update planning in `accounts_pos_profile`. JSON/JS kept external. |
| `pos_profile_user` | 5 | 3 | 1 | 1 | not_started | |
| `pos_search_fields` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pos_search_fields`. JSON kept external. |
| `pos_settings` | 5 | 3 | 1 | 1 | not_started | |
| `pricing_rule` | 6 | 4 | 1 | 1 | not_started | |
| `pricing_rule_brand` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pricing_rule_brand`. JSON kept external. |
| `pricing_rule_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pricing_rule_detail`. JSON kept external. |
| `pricing_rule_item_code` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pricing_rule_item_code`. JSON kept external. |
| `pricing_rule_item_group` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_pricing_rule_item_group`. JSON kept external. |
| `process_deferred_accounting` | 5 | 3 | 1 | 1 | parity_tested | Rust covers validate, submit conversion branching, cancel GL-entry plan, and metadata in `accounts_process_deferred_accounting`. JSON/JS kept external. |
| `process_payment_reconciliation` | 7 | 4 | 1 | 2 | parity_tested | Rust covers lifecycle hooks, account-company validation, dashboard/list helpers, progress seed, allocation grouping, reconcile job names, and metadata in `accounts_process_payment_reconciliation`. JSON/JS kept external. |
| `process_payment_reconciliation_log` | 6 | 3 | 1 | 2 | parity_tested | Python controller is pass/no-op; Rust metadata, progress helper, and list indicator mapping covered by `accounts_process_payment_reconciliation_log`. JSON/JS kept external. |
| `process_payment_reconciliation_log_allocations` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_process_payment_reconciliation_log_allocations`. JSON kept external. |
| `process_period_closing_voucher` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, processing date table generation, lifecycle hooks, client action/progress helpers in `accounts_process_period_closing_voucher`. JSON/JS kept external. |
| `process_period_closing_voucher_detail` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_process_period_closing_voucher_detail`. JSON kept external. |
| `process_statement_of_accounts` | 7 | 3 | 1 | 1 | parity_tested | Rust controller mirrors validation defaults/errors, auto-email dates, GL/AR filter builders, recipients/CC, and behavior-relevant metadata. JSON/JS/HTML kept external. |
| `process_statement_of_accounts_cc` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_process_statement_of_accounts_cc`. JSON kept external. |
| `process_statement_of_accounts_customer` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_process_statement_of_accounts_customer`. JSON kept external. |
| `process_subscription` | 5 | 3 | 1 | 1 | parity_tested | Rust covers submit hook, subscription filtering, 500-size batch enqueue plans, helper document creation, and metadata in `accounts_process_subscription`. JSON/JS kept external. |
| `promotional_scheme` | 6 | 4 | 1 | 1 | parity_tested | Rust covers validation, applicable-for checks, recursion guard, pricing rule draft generation, transaction-exists message, trash delete plan, and metadata in `accounts_promotional_scheme`. JSON/JS kept external. |
| `promotional_scheme_price_discount` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_promotional_scheme_price_discount`. JSON kept external. |
| `promotional_scheme_product_discount` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_promotional_scheme_product_discount`. JSON kept external. |
| `psoa_cost_center` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_psoa_cost_center`. JSON kept external. |
| `psoa_project` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_psoa_project`. JSON kept external. |
| `purchase_invoice` | 9 | 4 | 2 | 2 | not_started | |
| `purchase_invoice_advance` | 4 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_purchase_invoice_advance`. JSON kept external. |
| `purchase_invoice_item` | 4 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust field order, key metadata, and controller behavior covered by `accounts_purchase_invoice_item`. Full JSON kept external. |
| `purchase_taxes_and_charges` | 4 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust field order, key metadata, and controller behavior covered by `accounts_purchase_taxes_and_charges`. JSON kept external. |
| `purchase_taxes_and_charges_template` | 6 | 4 | 1 | 1 | parity_tested | Rust covers metadata, validate delegation, autoname behavior, and hooks in `accounts_purchase_taxes_and_charges_template`. JSON/JS kept external. |
| `repost_accounting_ledger` | 6 | 3 | 1 | 1 | parity_tested | Rust covers metadata, voucher allowed-type validation, deferred-accounting and closed-fiscal-year guards, preview empty-state, submit enqueue/start branching, and deterministic repost action planning in `accounts_repost_accounting_ledger`. JSON/HTML/DB effects kept external. |
| `repost_accounting_ledger_items` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_repost_accounting_ledger_items`. JSON kept external. |
| `repost_allowed_types` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_repost_allowed_types`. JSON kept external. |
| `repost_payment_ledger` | 6 | 3 | 1 | 2 | parity_tested | Rust covers metadata, filter-based voucher loading, queued status assignment, submit enqueue job, PLE delete/create planning, and worker success/failure status handling in `accounts_repost_payment_ledger`. JSON/DB effects kept external. |
| `repost_payment_ledger_items` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_repost_payment_ledger_items`. JSON kept external. |
| `sales_invoice` | 10 | 4 | 2 | 3 | not_started | |
| `sales_invoice_advance` | 4 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_sales_invoice_advance`. JSON kept external. |
| `sales_invoice_item` | 4 | 2 | 1 | 0 | not_started | |
| `sales_invoice_payment` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_sales_invoice_payment`. JSON kept external. |
| `sales_invoice_reference` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_sales_invoice_reference`. JSON kept external. |
| `sales_invoice_timesheet` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_sales_invoice_timesheet`. JSON kept external. |
| `sales_partner_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_sales_partner_item`. JSON kept external. |
| `sales_taxes_and_charges` | 4 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust field order, key metadata, and controller behavior covered by `accounts_sales_taxes_and_charges`. JSON kept external. |
| `sales_taxes_and_charges_template` | 6 | 4 | 1 | 1 | parity_tested | Rust covers metadata, validate delegation, disabled/default and tax-category guards, autoname, missing tax-rate fill, dashboard, JS hook constants, and controller hooks in `accounts_sales_taxes_and_charges_template`. JSON/JS kept external. |
| `share_balance` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_share_balance`. JSON kept external. |
| `share_transfer` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, basic validation, share existence classification, share range removal/splitting, submit/cancel plans, JV draft creation, JS hook constants, and controller hooks in `accounts_share_transfer`. JSON/JS kept external. |
| `share_type` | 6 | 4 | 1 | 1 | parity_tested | Python controller is pass/no-op; Rust metadata, dashboard, JS hook constants, and controller behavior covered by `accounts_share_type`. JSON/JS kept external. |
| `shareholder` | 6 | 4 | 1 | 1 | parity_tested | Rust covers metadata, onload/on_trash actions, before_save share amount calculation, dashboard, JS hook constants, and controller hooks in `accounts_shareholder`. JSON/JS kept external. |
| `shipping_rule` | 7 | 4 | 2 | 1 | parity_tested | Rust covers metadata, validate condition ranges, overlap checks, country restriction, amount selection/conversion, tax-table append/update behavior, dashboard groups, JS hook constants, and controller hooks in `accounts_shipping_rule`. JSON/JS kept external. |
| `shipping_rule_condition` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_shipping_rule`. JSON kept external. |
| `shipping_rule_country` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_shipping_rule`. JSON kept external. |
| `south_africa_vat_account` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_supplier_static_items`. JSON kept external. |
| `subscription` | 6 | 3 | 1 | 2 | not_started | |
| `subscription_invoice` | 5 | 3 | 1 | 1 | not_started | |
| `subscription_plan` | 6 | 4 | 1 | 1 | not_started | |
| `subscription_plan_detail` | 3 | 2 | 1 | 0 | not_started | |
| `subscription_settings` | 5 | 3 | 1 | 1 | not_started | |
| `supplier_group_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_supplier_static_items`. JSON kept external. |
| `supplier_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_supplier_static_items`. JSON kept external. |
| `tax_category` | 6 | 4 | 1 | 1 | parity_tested | Python controller is pass/no-op; Rust metadata, dashboard constants, and controller behavior covered by `accounts_tax_category_static`. JSON/JS kept external. |
| `tax_rule` | 5 | 3 | 1 | 1 | parity_tested | Rust covers metadata, template clearing/mandatory validation, conflict detection by exact filters and date overlap, tax-template lookup specificity/priority/category/customer-group behavior, JS hook constants, and controller hooks in `accounts_tax_rule`. JSON/JS kept external. |
| `tax_withholding_account` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_tax_category_static`. JSON kept external. |
| `tax_withholding_category` | 6 | 4 | 1 | 1 | parity_tested | Rust covers metadata, date overlap validation, duplicate company/account validation, threshold validation, applicable tax row lookup, company account lookup, dashboard constants, and controller hooks in `accounts_tax_withholding_category`. JSON/JS kept external. |
| `tax_withholding_entry` | 4 | 3 | 1 | 0 | ported | Rust covers metadata, status/link difference helpers, withholding amount calculation, adjustment validations, update value helpers, and controller hooks in `accounts_tax_withholding_entry`. Full DB-backed submission flow remains external. |
| `tax_withholding_group` | 5 | 3 | 1 | 1 | parity_tested | Python controller/test are pass/no-op; Rust metadata and controller behavior covered by `accounts_static_tail_items`. JSON/JS kept external. |
| `tax_withholding_rate` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_tax_withholding_category`. JSON kept external. |
| `territory_item` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_static_tail_items`. JSON kept external. |
| `transaction_deletion_record_details` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_static_tail_items`. JSON kept external. |
| `unreconcile_payment` | 5 | 3 | 1 | 1 | ported | Rust covers metadata, supported-type validation, linked payment/advance grouping, selection filtering, submit action planning, and JS query constants in `accounts_unreconcile_payment`. Frappe DB side effects remain represented as actions. |
| `unreconcile_payment_entries` | 3 | 2 | 1 | 0 | parity_tested | Python controller is pass/no-op; Rust metadata and controller behavior covered by `accounts_unreconcile_payment`. JSON kept external. |

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

## Doctype Detail: `pegged_currencies`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pegged_currencies`
Target: `src/erpnext/accounts/doctype/pegged_currencies`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- `test_pegged_currencies.py` contains only imports/comments; Rust parity test covers metadata and no-op controller behavior.
- DocType metadata:
  - `name`: `Pegged Currencies`
  - `module`: `Accounts`
  - `index_web_pages_for_search`: enabled
  - `field_order`: `pegged_currencies_item_section`, `pegged_currency_item`
  - `pegged_currencies_item_section`: `Section Break`
  - `pegged_currency_item`: `Table`, no label, options `Pegged Currency Details`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pegged_currencies.py` | `pegged_currencies.rs` | parity_tested | No-op controller and pegged currencies table metadata represented in Rust. |
| `test_pegged_currencies.py` | `tests/accounts_pegged_currencies.rs` | parity_tested | Empty ERPNext test file replaced by Rust metadata/controller parity tests. |
| `pegged_currencies.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `pegged_currencies.js` | ERPNext UI retained | external_kept | Only commented refresh scaffold exists and remains UI-side. |

## Doctype Detail: `pos_closing_entry_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_closing_entry_detail`
Target: `src/erpnext/accounts/doctype/pos_closing_entry_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes mode of payment, opening/expected/closing/difference amounts, and child table parent fields.
- DocType metadata:
  - `name`: `POS Closing Entry Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `mode_of_payment`, `opening_amount`, `expected_amount`, `closing_amount`, `difference`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`, required, `in_list_view: 1`
  - `opening_amount`: `Currency`, label `Opening Amount`, options `company:company_currency`, required, read-only, `in_list_view: 1`
  - `expected_amount`: `Currency`, label `Expected Amount`, options `company:company_currency`, read-only, `in_list_view: 1`
  - `closing_amount`: `Currency`, label `Closing Amount`, options `company:company_currency`, required, default `0`, `in_list_view: 1`
  - `difference`: `Currency`, label `Difference`, options `company:company_currency`, read-only, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_closing_entry_detail.py` | `pos_closing_entry_detail.rs` | parity_tested | No-op child table controller and POS closing detail metadata represented in Rust. |
| `pos_closing_entry_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_closing_entry_taxes`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_closing_entry_taxes`
Target: `src/erpnext/accounts/doctype/pos_closing_entry_taxes`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes account head, amount, and child table parent fields.
- DocType metadata:
  - `name`: `POS Closing Entry Taxes`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `account_head`, `amount`
  - `account_head`: `Link`, label `Account Head`, options `Account`, read-only, `in_list_view: 1`
  - `amount`: `Currency`, label `Amount`, read-only, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_closing_entry_taxes.py` | `pos_closing_entry_taxes.rs` | parity_tested | No-op child table controller and POS closing tax metadata represented in Rust. |
| `pos_closing_entry_taxes.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_customer_group`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_customer_group`
Target: `src/erpnext/accounts/doctype/pos_customer_group`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes customer group and child table parent fields.
- DocType metadata:
  - `name`: `POS Customer Group`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `customer_group`
  - `customer_group`: `Link`, label `Customer Group`, options `Customer Group`, required, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_customer_group.py` | `pos_customer_group.rs` | parity_tested | No-op child table controller and POS customer group metadata represented in Rust. |
| `pos_customer_group.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_field`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_field`
Target: `src/erpnext/accounts/doctype/pos_field`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes POS field metadata fields and child table parent fields.
- DocType metadata:
  - `name`: `POS Field`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `fieldname`, `label`, `fieldtype`, `column_break_7`, `options`, `default_value`, `reqd`, `read_only`
  - `fieldname`: `Select`, label `Fieldname`, `in_list_view: 1`
  - `label`: `Data`, label `Label`, read-only, `in_list_view: 1`
  - `fieldtype`: `Data`, label `Fieldtype`, read-only, `in_list_view: 1`
  - `column_break_7`: `Column Break`
  - `options`: `Text`, label `Options`, read-only, `in_list_view: 1`
  - `default_value`: `Data`, label `Default Value`
  - `reqd`: `Check`, label `Mandatory`, default `0`
  - `read_only`: `Check`, label `Read Only`, default `0`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_field.py` | `pos_field.rs` | parity_tested | No-op child table controller and POS field metadata represented in Rust. |
| `pos_field.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_invoice_reference`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_invoice_reference`
Target: `src/erpnext/accounts/doctype/pos_invoice_reference`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes POS invoice, posting date, customer, grand total, return flags, and child table parent fields.
- DocType metadata:
  - `name`: `POS Invoice Reference`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `pos_invoice`, `posting_date`, `column_break_3`, `customer`, `grand_total`, `is_return`, `return_against`
  - `pos_invoice`: `Link`, label `POS Invoice`, options `POS Invoice`, required, `in_list_view: 1`
  - `posting_date`: `Date`, label `Date`, required, `in_list_view: 1`
  - `column_break_3`: `Column Break`
  - `customer`: `Link`, label `Customer`, options `Customer`, required, read-only
  - `grand_total`: `Currency`, label `Amount`, required, `in_list_view: 1`
  - `is_return`: `Check`, label `Is Return`, read-only, default `0`
  - `return_against`: `Link`, label `Return Against`, options `POS Invoice`, read-only

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_invoice_reference.py` | `pos_invoice_reference.rs` | parity_tested | No-op child table controller and POS invoice reference metadata represented in Rust. |
| `pos_invoice_reference.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_item_group`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_item_group`
Target: `src/erpnext/accounts/doctype/pos_item_group`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes item group and child table parent fields.
- DocType metadata:
  - `name`: `POS Item Group`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `item_group`
  - `item_group`: `Link`, label `Item Group`, options `Item Group`, required, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_item_group.py` | `pos_item_group.rs` | parity_tested | No-op child table controller and POS item group metadata represented in Rust. |
| `pos_item_group.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_opening_entry_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_opening_entry_detail`
Target: `src/erpnext/accounts/doctype/pos_opening_entry_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes mode of payment, opening amount, and child table parent fields.
- DocType metadata:
  - `name`: `POS Opening Entry Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `mode_of_payment`, `opening_amount`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`, required, `in_list_view: 1`
  - `opening_amount`: `Currency`, label `Opening Amount`, options `company:company_currency`, required, default `0`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_opening_entry_detail.py` | `pos_opening_entry_detail.rs` | parity_tested | No-op child table controller and POS opening detail metadata represented in Rust. |
| `pos_opening_entry_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_payment_method`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_payment_method`
Target: `src/erpnext/accounts/doctype/pos_payment_method`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes default flags, mode of payment, and child table parent fields.
- DocType metadata:
  - `name`: `POS Payment Method`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `default`, `allow_in_returns`, `mode_of_payment`
  - `default`: `Check`, label `Default`, default `0`, `in_list_view: 1`, `depends_on: eval:parent.doctype == 'POS Profile'`
  - `allow_in_returns`: `Check`, label `Allow In Returns`, default `0`, `in_list_view: 1`
  - `mode_of_payment`: `Link`, label `Mode of Payment`, options `Mode of Payment`, required, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_payment_method.py` | `pos_payment_method.rs` | parity_tested | No-op child table controller and POS payment method metadata represented in Rust. |
| `pos_payment_method.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pos_search_fields`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pos_search_fields`
Target: `src/erpnext/accounts/doctype/pos_search_fields`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes search field selector, fieldname, and child table parent fields.
- DocType metadata:
  - `name`: `POS Search Fields`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `field_order`: `field`, `fieldname`
  - `field`: `Select`, label `Field`, required, `in_list_view: 1`
  - `fieldname`: `Data`, label `Fieldname`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pos_search_fields.py` | `pos_search_fields.rs` | parity_tested | No-op child table controller and POS search field metadata represented in Rust. |
| `pos_search_fields.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pricing_rule_brand`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pricing_rule_brand`
Target: `src/erpnext/accounts/doctype/pricing_rule_brand`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes brand, UOM, and child table parent fields.
- DocType metadata:
  - `name`: `Pricing Rule Brand`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `field_order`: `brand`, `uom`
  - `brand`: `Link`, label `Brand`, options `Brand`, `in_list_view: 1`, `depends_on: eval:parent.apply_on == 'Brand'`
  - `uom`: `Link`, label `UOM`, options `UOM`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pricing_rule_brand.py` | `pricing_rule_brand.rs` | parity_tested | No-op child table controller and pricing rule brand metadata represented in Rust. |
| `pricing_rule_brand.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pricing_rule_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pricing_rule_detail`
Target: `src/erpnext/accounts/doctype/pricing_rule_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes pricing rule, item code, margin metadata, child docname, rule applied, and child table parent fields.
- DocType metadata:
  - `name`: `Pricing Rule Detail`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `quick_entry`: enabled
  - `track_changes`: enabled
  - `field_order`: `pricing_rule`, `item_code`, `margin_type`, `rate_or_discount`, `child_docname`, `rule_applied`
  - `pricing_rule`: `Link`, label `Pricing Rule`, options `Pricing Rule`, `in_list_view: 1`, `read_only: 1`
  - `item_code`: `Data`, label `Item Code`, `in_list_view: 1`, `read_only: 1`
  - `margin_type`: `Data`, label `Margin Type`, `hidden: 1`, `read_only: 1`
  - `rate_or_discount`: `Data`, label `Rate or Discount`, `hidden: 1`, `read_only: 1`
  - `child_docname`: `Data`, label `Child Docname`, `hidden: 1`, `no_copy: 1`, `print_hide: 1`, `read_only: 1`
  - `rule_applied`: `Check`, label `Rule Applied`, default `1`, `read_only: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pricing_rule_detail.py` | `pricing_rule_detail.rs` | parity_tested | No-op child table controller and pricing rule detail metadata represented in Rust. |
| `pricing_rule_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pricing_rule_item_code`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pricing_rule_item_code`
Target: `src/erpnext/accounts/doctype/pricing_rule_item_code`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes item code, UOM, and child table parent fields.
- DocType metadata:
  - `name`: `Pricing Rule Item Code`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `track_changes`: enabled
  - `field_order`: `item_code`, `uom`
  - `item_code`: `Link`, label `Item Code`, options `Item`, `in_list_view: 1`, `search_index: 1`, `depends_on: eval:parent.apply_on == 'Item Code'`
  - `uom`: `Link`, label `UOM`, options `UOM`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pricing_rule_item_code.py` | `pricing_rule_item_code.rs` | parity_tested | No-op child table controller and pricing rule item code metadata represented in Rust. |
| `pricing_rule_item_code.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `pricing_rule_item_group`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/pricing_rule_item_group`
Target: `src/erpnext/accounts/doctype/pricing_rule_item_group`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes item group, UOM, and child table parent fields.
- DocType metadata:
  - `name`: `Pricing Rule Item Group`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `track_changes`: enabled
  - `row_format`: `Dynamic`
  - `field_order`: `item_group`, `uom`
  - `item_group`: `Link`, label `Item Group`, options `Item Group`, `in_list_view: 1`, `search_index: 1`, `depends_on: eval:parent.apply_on == 'Item Group'`
  - `uom`: `Link`, label `UOM`, options `UOM`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `pricing_rule_item_group.py` | `pricing_rule_item_group.rs` | parity_tested | No-op child table controller and pricing rule item group metadata represented in Rust. |
| `pricing_rule_item_group.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `process_deferred_accounting`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_deferred_accounting`
Target: `src/erpnext/accounts/doctype/process_deferred_accounting`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `validate` throws `End date cannot be before start date` when `end_date < start_date`.
- `on_submit` builds deferred accounting conditions from `type`, `account`, and `company`.
- `on_submit` calls deferred revenue conversion when `type == "Income"`.
- `on_submit` calls deferred expense conversion for all other types, matching the Python `else` branch.
- `on_cancel` ignores linked `GL Entry` doctypes, fetches GL Entries by `against_voucher_type` and `against_voucher`, then cancels them through `make_gl_entries(cancel=1)`.
- Source Python tests cover submit/cancel GL behavior; Rust tests cover the local controller branching and cancellation plan without owning ERPNext DB/GL side effects.
- DocType metadata:
  - `name`: `Process Deferred Accounting`
  - `module`: `Accounts`
  - `autoname`: `ACC-PDA-.#####`
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `is_submittable`: enabled
  - `field_order`: `company`, `type`, `account`, `column_break_3`, `posting_date`, `start_date`, `end_date`, `amended_from`
  - `company`: `Link`, label `Company`, options `Company`, required
  - `type`: `Select`, label `Type`, options newline, `Income`, `Expense`, required, `in_list_view: 1`
  - `account`: `Link`, label `Account`, options `Account`, `depends_on: eval: doc.type`
  - `posting_date`: `Date`, label `Posting Date`, default `Today`, required, `in_list_view: 1`
  - `start_date`: `Date`, label `Service Start Date`, required, `in_list_view: 1`
  - `end_date`: `Date`, label `Service End Date`, required, `in_list_view: 1`
  - `amended_from`: `Link`, label `Amended From`, options `Process Deferred Accounting`, `no_copy: 1`, `print_hide: 1`, `read_only: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_deferred_accounting.py` | `process_deferred_accounting.rs` | parity_tested | Validate, submit conversion branch, cancel GL plan, hooks, and metadata represented in Rust. |
| `test_process_deferred_accounting.py` | `tests/accounts_process_deferred_accounting.rs` | parity_tested | Rust tests cover deterministic controller behavior; ERPNext integration DB/GL tests remain source reference. |
| `process_deferred_accounting.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `process_deferred_accounting.js` | ERPNext client script retained | external_kept | Client-side query, settings, and date defaults remain UI/Frappe-owned. |

## Doctype Detail: `process_payment_reconciliation`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_payment_reconciliation`
Target: `src/erpnext/accounts/doctype/process_payment_reconciliation`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `on_discard` sets document `status` to `Cancelled`.
- `validate` checks both receivable/payable and bank/cash accounts against the selected company.
- `before_save` clears `status` and `error_log`.
- `on_submit` sets `status` to `Queued` and clears `error_log`.
- `on_cancel` sets `status` to `Cancelled` and cancels the linked reconciliation log when present.
- `get_reconciled_count` returns processed and total allocation counts from the reconciliation log.
- `get_pr_instance` copies core reconciliation filters to a `Payment Reconciliation` instance and sets invoice/payment limits to `1000`.
- `get_next_allocation` picks the first unreconciled allocation by `idx`, then returns all unreconciled rows for the same reference type/name.
- Reconcile job names follow `process_{doc}_reconcile_allocation_{first_idx}_{last_idx}` when an allocation batch exists, otherwise `process_{doc}_reconcile`.
- Dashboard data exposes `Process Payment Reconciliation Log` through `process_pr`.
- List indicators map `Queued`, `Paused`, and `Partially Reconciled` to orange, `Completed` to green, `Running` to blue, and `Failed` to red.
- Source Python test class is currently `pass`; Rust tests cover deterministic behavior and metadata while DB/RQ/Frappe side effects remain external.
- DocType metadata:
  - `name`: `Process Payment Reconciliation`
  - `module`: `Accounts`
  - `autoname`: `format:ACC-PPR-{#####}`
  - `title_field`: `company`
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `is_submittable`: enabled
  - `field_order`: `company`, `party_type`, `column_break_io6c`, `party`, `receivable_payable_account`, `default_advance_account`, `filter_section`, `from_invoice_date`, `to_invoice_date`, `column_break_kegk`, `from_payment_date`, `to_payment_date`, `column_break_uj04`, `cost_center`, `bank_cash_account`, `section_break_2n02`, `status`, `error_log`, `section_break_a8yx`, `amended_from`
  - `company`: `Link`, label `Company`, options `Company`, required, `in_list_view: 1`
  - `party_type`: `Link`, label `Party Type`, options `DocType`, required, `in_list_view: 1`
  - `party`: `Dynamic Link`, label `Party`, options `party_type`, required, `in_list_view: 1`
  - `receivable_payable_account`: `Link`, label `Receivable/Payable Account`, options `Account`, required, `in_list_view: 1`
  - `default_advance_account`: `Link`, label `Default Advance Account`, options `Account`, required, `depends_on: eval:doc.party`, `mandatory_depends_on: doc.party_type`, description and documentation URL retained
  - `status`: `Select`, read-only, allow-on-submit, options `Queued`, `Running`, `Paused`, `Completed`, `Partially Reconciled`, `Failed`, `Cancelled`
  - `error_log`: `Long Text`, label `Error Log`, `depends_on: eval:doc.error_log`
  - `amended_from`: `Link`, label `Amended From`, options `Process Payment Reconciliation`, `no_copy: 1`, `print_hide: 1`, `read_only: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_payment_reconciliation.py` | `process_payment_reconciliation.rs` | parity_tested | Lifecycle hooks, account-company validation, reconciliation seed, allocation grouping, job names, dashboard/list helpers, and metadata represented in Rust. |
| `process_payment_reconciliation_dashboard.py` | `process_payment_reconciliation.rs` | parity_tested | Dashboard data represented by Rust helper and tests. |
| `test_process_payment_reconciliation.py` | `tests/accounts_process_payment_reconciliation.rs` | parity_tested | Source test class is empty; Rust tests cover deterministic controller and helper behavior. |
| `process_payment_reconciliation.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants, including description and documentation URL fields. |
| `process_payment_reconciliation.js` | ERPNext client script retained | external_kept | Form queries, buttons, and client-side calls remain UI/Frappe-owned. |
| `process_payment_reconciliation_list.js` | ERPNext list script retained | external_kept | List indicator mapping is mirrored in Rust helper; UI script remains Frappe-owned. |

## Doctype Detail: `process_payment_reconciliation_log`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_payment_reconciliation_log`
Target: `src/erpnext/accounts/doctype/process_payment_reconciliation_log`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Client refresh script shows reconciliation progress for `Completed`, `Running`, `Paused`, and `Partially Reconciled`.
- Progress is `(reconciled_entries / total_allocations) * 100` when reconciled entries are non-zero.
- Progress is `100` when total allocations are `0` and status is `Completed`.
- List indicators map `Partially Reconciled` and `Paused` to orange, `Reconciled` to green, `Failed` and `Cancelled` to red, and `Running` to blue.
- Source Python test class is currently `pass`; Rust tests cover deterministic metadata, no-op controller behavior, progress, and list indicator logic.
- DocType metadata:
  - `name`: `Process Payment Reconciliation Log`
  - `module`: `Accounts`
  - `autoname`: `format:PPR-LOG-{##}`
  - `editable_grid`: enabled
  - `in_create`: enabled
  - `index_web_pages_for_search`: enabled
  - `search_fields`: `allocated, reconciled, total_allocations, reconciled_entries`
  - `field_order`: `process_pr`, `section_break_fvdw`, `status`, `tasks_section`, `allocated`, `reconciled`, `column_break_yhin`, `total_allocations`, `reconciled_entries`, `section_break_4ywv`, `error_log`, `allocations_section`, `allocations`
  - `process_pr`: `Link`, label `Parent Document`, options `Process Payment Reconciliation`, required, read-only, `in_list_view: 1`
  - `status`: `Select`, label `Status`, options `Running`, `Paused`, `Reconciled`, `Partially Reconciled`, `Failed`, `Cancelled`, read-only
  - `allocated`: `Check`, label `Allocated`, default `0`, description retained, read-only
  - `reconciled`: `Check`, label `Reconciled`, default `0`, description retained, read-only
  - `total_allocations`: `Int`, label `Total Allocations`, read-only, `in_list_view: 1`
  - `reconciled_entries`: `Int`, label `Reconciled Entries`, read-only, `in_list_view: 1`
  - `error_log`: `Long Text`, label `Reconciliation Error Log`, `depends_on: eval:doc.error_log`, read-only
  - `allocations`: `Table`, label `Allocations`, options `Process Payment Reconciliation Log Allocations`, read-only

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_payment_reconciliation_log.py` | `process_payment_reconciliation_log.rs` | parity_tested | No-op controller and metadata represented in Rust. |
| `test_process_payment_reconciliation_log.py` | `tests/accounts_process_payment_reconciliation_log.rs` | parity_tested | Source test class is empty; Rust tests cover deterministic metadata and helper behavior. |
| `process_payment_reconciliation_log.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `process_payment_reconciliation_log.js` | ERPNext client script retained | external_kept | Progress behavior is mirrored by Rust helper; UI script remains Frappe-owned. |
| `process_payment_reconciliation_log_list.js` | ERPNext list script retained | external_kept | List indicator behavior is mirrored by Rust helper; UI script remains Frappe-owned. |

## Doctype Detail: `process_payment_reconciliation_log_allocations`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_payment_reconciliation_log_allocations`
Target: `src/erpnext/accounts/doctype/process_payment_reconciliation_log_allocations`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes reference, invoice, allocation, difference, currency, and reconciliation fields.
- DocType metadata:
  - `name`: `Process Payment Reconciliation Log Allocations`
  - `module`: `Accounts`
  - `istable`: enabled
  - `editable_grid`: enabled
  - `track_changes`: enabled
  - `field_order`: `reference_type`, `reference_name`, `reference_row`, `column_break_3`, `invoice_type`, `invoice_number`, `section_break_6`, `allocated_amount`, `unreconciled_amount`, `column_break_8`, `amount`, `is_advance`, `section_break_5`, `difference_amount`, `gain_loss_posting_date`, `column_break_7`, `difference_account`, `exchange_rate`, `currency`, `reconciled`
  - `reference_type`: `Link`, label `Reference Type`, options `DocType`, required, read-only
  - `reference_name`: `Dynamic Link`, label `Reference Name`, options `reference_type`, required, read-only, `in_list_view: 1`
  - `reference_row`: `Data`, label `Reference Row`, hidden, read-only
  - `invoice_type`: `Link`, label `Invoice Type`, options `DocType`, required, read-only
  - `invoice_number`: `Dynamic Link`, label `Invoice Number`, options `invoice_type`, required, read-only, `in_list_view: 1`
  - `allocated_amount`: `Currency`, label `Allocated Amount`, options `currency`, required, `in_list_view: 1`
  - `unreconciled_amount`: `Currency`, label `Unreconciled Amount`, options `currency`, hidden, read-only
  - `amount`: `Currency`, label `Amount`, options `currency`, hidden, read-only
  - `is_advance`: `Data`, label `Is Advance`, hidden, read-only
  - `difference_amount`: `Currency`, label `Difference Amount`, options `Currency`, read-only, `in_list_view: 1`
  - `gain_loss_posting_date`: `Date`, label `Difference Posting Date`
  - `difference_account`: `Link`, label `Difference Account`, options `Account`, read-only
  - `exchange_rate`: `Float`, label `Exchange Rate`, read-only
  - `currency`: `Link`, label `Currency`, options `Currency`, hidden
  - `reconciled`: `Check`, label `Reconciled`, default `0`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_payment_reconciliation_log_allocations.py` | `process_payment_reconciliation_log_allocations.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `process_payment_reconciliation_log_allocations.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `process_period_closing_voucher`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_period_closing_voucher`
Target: `src/erpnext/accounts/doctype/process_period_closing_voucher`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `on_discard` sets `status` to `Cancelled`.
- `validate` sets `status` to `Queued` and populates processing tables.
- `populate_processing_tables` calls normal balance date generation and opening balance date generation.
- `generate_pcv_dates` creates two queued `normal_balances` rows for each inclusive PCV date: `Profit and Loss` and `Balance Sheet`.
- `generate_opening_balances_dates` creates queued `z_opening_balances` rows for each inclusive GL posting date when the parent PCV is the first period closing voucher.
- `on_submit` delegates to `start_pcv_processing(self.name)` in Python; Rust represents this as a deterministic start hook plan.
- `on_cancel` delegates to `cancel_pcv_processing(self.name)` in Python; Rust represents this as a deterministic cancel hook plan.
- Client script maps submitted `Queued`, `Running`, and `Paused` documents to `Start`, `Pause`, and `Resume` actions.
- Client progress is `(completed normal rows + completed opening rows) / total rows * 100`.
- GL summarization and closing entry posting remain Frappe/DB-owned side effects to port in a later deeper pass.
- DocType metadata:
  - `name`: `Process Period Closing Voucher`
  - `module`: `Accounts`
  - `autoname`: `format:Process-PCV-{###}`
  - `grid_page_length`: `50`
  - `row_format`: `Dynamic`
  - `index_web_pages_for_search`: enabled
  - `is_submittable`: enabled
  - `field_order`: `parent_pcv`, `status`, `p_l_closing_balance`, `normal_balances`, `bs_closing_balance`, `z_opening_balances`, `amended_from`
  - `parent_pcv`: `Link`, label `PCV`, options `Period Closing Voucher`, required, `in_list_view: 1`
  - `status`: `Select`, label `Status`, default `Queued`, options `Queued`, `Running`, `Paused`, `Completed`, `Cancelled`, `no_copy: 1`
  - `p_l_closing_balance`: `JSON`, label `P&L Closing Balance`, `no_copy: 1`
  - `normal_balances`: `Table`, label `Dates to Process`, options `Process Period Closing Voucher Detail`, `no_copy: 1`
  - `bs_closing_balance`: `JSON`, label `Balance Sheet Closing Balance`
  - `z_opening_balances`: `Table`, label `Opening Balances`, options `Process Period Closing Voucher Detail`, `no_copy: 1`
  - `amended_from`: `Link`, label `Amended From`, options `Process Period Closing Voucher`, `no_copy: 1`, `print_hide: 1`, `read_only: 1`, `search_index: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_period_closing_voucher.py` | `process_period_closing_voucher.rs` | parity_tested | Metadata, validation table generation, lifecycle hooks, client action/progress helpers represented in Rust. |
| `test_process_period_closing_voucher.py` | `tests/accounts_process_period_closing_voucher.rs` | parity_tested | Source test file has no test methods; Rust tests cover deterministic controller and helper behavior. |
| `process_period_closing_voucher.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `process_period_closing_voucher.js` | ERPNext client script retained | external_kept | Button and progress behavior is mirrored by Rust helpers; UI script remains Frappe-owned. |

## Doctype Detail: `process_period_closing_voucher_detail`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_period_closing_voucher_detail`
Target: `src/erpnext/accounts/doctype/process_period_closing_voucher_detail`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes processing date, report type, status, and closing balance.
- DocType metadata:
  - `name`: `Process Period Closing Voucher Detail`
  - `module`: `Accounts`
  - `allow_rename`: enabled
  - `index_web_pages_for_search`: enabled
  - `istable`: enabled
  - `grid_page_length`: `50`
  - `row_format`: `Dynamic`
  - `rows_threshold_for_grid_search`: `20`
  - `field_order`: `processing_date`, `report_type`, `status`, `closing_balance`
  - `processing_date`: `Date`, label `Processing Date`, `in_list_view: 1`
  - `report_type`: `Select`, label `Report Type`, options `Profit and Loss`, `Balance Sheet`, default `Profit and Loss`, `in_list_view: 1`
  - `status`: `Select`, label `Status`, options `Queued`, `Running`, `Paused`, `Completed`, `Cancelled`, default `Queued`, `in_list_view: 1`
  - `closing_balance`: `JSON`, label `Closing Balance`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_period_closing_voucher_detail.py` | `process_period_closing_voucher_detail.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `process_period_closing_voucher_detail.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `process_statement_of_accounts`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_statement_of_accounts`
Target: `src/erpnext/accounts/doctype/process_statement_of_accounts`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `validate` sets default `subject`, `body`, and `pdf_name`, requires at least one selected customer, validates account/cost center/project company ownership, validates report print format type, and derives auto-email dates from `start_date` plus `filter_duration`.
- General Ledger filters preserve customer party, party name, date range, presentation currency, categorize-by, project, tax id, and net-party-account flag.
- Accounts Receivable filters preserve report date, party, customer name, payment terms, sales dimensions, ageing basis, future payment flag, and default ageing ranges `30/60/90/120`.
- Recipient generation uses billing email first, adds primary contact emails when primary-contact sending is enabled, and collects configured CC recipients.
- DocType metadata:
  - `name`: `Process Statement Of Accounts`
  - `module`: `Accounts`
  - `autoname`: `Prompt`
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `track_changes`: enabled
  - `row_format`: `Dynamic`
  - `field_order`: 57 fields from `report` through `help_text`
  - `report`: `Select`, required, options `General Ledger`, `Accounts Receivable`
  - `company`: `Link`, options `Company`, required, `in_list_view: 1`
  - `from_date`: `Date`, shown/mandatory for manual General Ledger runs
  - `cost_center`: `Table MultiSelect`, options `PSOA Cost Center`
  - `fetch_customers`: `Button`, options `fetch_customers`, print/report hidden, depends on selected collection
  - `body`: `Text Editor`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_statement_of_accounts.py` | `process_statement_of_accounts.rs` | parity_tested | Validation defaults/errors, auto-email date helpers, report filter builders, recipient collection, and metadata constants represented in Rust. |
| `test_process_statement_of_accounts.py` | `tests/accounts_process_statement_of_accounts.rs` | parity_tested | Rust tests cover deterministic controller behavior without DB/email side effects. |
| `process_statement_of_accounts.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `process_statement_of_accounts.js` | ERPNext client script retained | external_kept | UI actions and form scripting remain Frappe-owned. |
| `process_statement_of_accounts.html` | ERPNext template retained | external_kept | Print/email HTML template remains Frappe-owned. |

## Doctype Detail: `process_statement_of_accounts_cc`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_statement_of_accounts_cc`
Target: `src/erpnext/accounts/doctype/process_statement_of_accounts_cc`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes `cc` as a nullable Link.
- DocType metadata:
  - `name`: `Process Statement Of Accounts CC`
  - `module`: `Accounts`
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `istable`: enabled
  - `field_order`: `cc`
  - `cc`: `Link`, label `CC`, options `User`, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_statement_of_accounts_cc.py` | `process_statement_of_accounts_cc.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `process_statement_of_accounts_cc.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `process_statement_of_accounts_customer`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_statement_of_accounts_customer`
Target: `src/erpnext/accounts/doctype/process_statement_of_accounts_customer`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- Auto-generated type block exposes customer, customer name, billing email, and primary email fields.
- DocType metadata:
  - `name`: `Process Statement Of Accounts Customer`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: `customer`, `customer_name`, `billing_email`, `primary_email`
  - `customer`: `Link`, label `Customer`, options `Customer`, `in_list_view: 1`
  - `primary_email`: `Read Only`, label `Primary Contact Email`, `in_list_view: 1`
  - `billing_email`: `Data`, label `Billing Email`, `in_list_view: 1`
  - `customer_name`: `Data`, label `Customer Name`, fetched from `customer.customer_name`, read-only

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_statement_of_accounts_customer.py` | `process_statement_of_accounts_customer.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `process_statement_of_accounts_customer.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `psoa_cost_center`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/psoa_cost_center`
Target: `src/erpnext/accounts/doctype/psoa_cost_center`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `PSOA Cost Center`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: `cost_center_name`
  - `cost_center_name`: `Link`, label `Cost Center`, options `Cost Center`, required, `in_list_view: 1`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `psoa_cost_center.py` | `psoa_cost_center.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `psoa_cost_center.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `psoa_project`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/psoa_project`
Target: `src/erpnext/accounts/doctype/psoa_project`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `PSOA Project`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: `project_name`
  - `project_name`: `Link`, label `Project`, options `Project`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `psoa_project.py` | `psoa_project.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `psoa_project.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `process_subscription`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/process_subscription`
Target: `src/erpnext/accounts/doctype/process_subscription`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `on_submit` calls `process_all_subscription`.
- `process_all_subscription` loads all non-cancelled subscriptions, narrows to the selected subscription when present, splits names into batches of `500`, and enqueues `erpnext.accounts.doctype.subscription.subscription.process_all` on the `long` queue with the document posting date.
- `create_subscription_process` creates a `Process Subscription` document, sets subscription and posting date, then submits it.
- DocType metadata:
  - `name`: `Process Subscription`
  - `module`: `Accounts`
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `is_submittable`: enabled
  - `field_order`: `posting_date`, `subscription`, `amended_from`
  - `posting_date`: `Date`, label `Posting Date`, required, `in_list_view: 1`
  - `subscription`: `Link`, label `Subscription`, options `Subscription`
  - `amended_from`: `Link`, label `Amended From`, options `Process Subscription`, read-only, print hidden

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `process_subscription.py` | `process_subscription.rs` | parity_tested | Submit hook, subscription filtering, 500-size enqueue batches, helper creation, and metadata represented in Rust. |
| `test_process_subscription.py` | `tests/accounts_process_subscription.rs` | parity_tested | Source test class is empty; Rust tests cover deterministic controller behavior. |
| `process_subscription.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `process_subscription.js` | ERPNext client script retained | external_kept | Client/UI behavior remains Frappe-owned. |

## Doctype Detail: `promotional_scheme`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/promotional_scheme`
Target: `src/erpnext/accounts/doctype/promotional_scheme`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `validate` requires either Selling or Buying, requires at least one price/product discount slab, validates that the selected Applicable For table has values, and rejects recursive product discounts when mixed conditions are enabled.
- `get_args_for_pricing_rule` copies scheme-level pricing rule fields and expands the selected Applicable For table into a value list.
- `get_pricing_rules` creates pricing rule drafts for price discount slabs and product discount slabs, one rule per applicable-for value when applicable, or one rule per slab without party targeting.
- `set_args` behavior is mirrored for deterministic fields: slab `min_amount`/`max_amount` become pricing rule `min_amt`/`max_amt`, slab name becomes `promotional_scheme_id`, scheme name becomes `promotional_scheme`, discount type becomes `Price` or `Product`, and item rows are copied onto the pricing rule draft.
- `raise_for_transaction_exists` returns the ERPNext transaction-blocking message.
- `on_trash` plans deletion of existing Pricing Rules for the scheme.
- DocType metadata:
  - `name`: `Promotional Scheme`
  - `module`: `Accounts`
  - `autoname`: `Prompt`
  - `allow_rename`: enabled
  - `editable_grid`: enabled
  - `track_changes`: enabled
  - `field_order`: 37 fields from `section_break_1` through `product_discount_slabs`
  - `apply_on`: `Select`, required, default `Item Code`, options blank, `Item Code`, `Item Group`, `Brand`, `Transaction`
  - `customer`: `Table MultiSelect`, options `Customer Item`, depends on `Applicable For == Customer`
  - `company`: `Link`, options `Company`, required, `in_list_view: 1`
  - `price_discount_slabs`: `Table`, options `Promotional Scheme Price Discount`
  - `product_discount_slabs`: `Table`, options `Promotional Scheme Product Discount`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `promotional_scheme.py` | `promotional_scheme.rs` | parity_tested | Validation, pricing rule draft generation, transaction-exists message, trash delete plan, and metadata represented in Rust. |
| `test_promotional_scheme.py` | `tests/accounts_promotional_scheme.rs` | parity_tested | Rust tests cover deterministic equivalents of ERPNext validation and pricing rule creation/update behavior. |
| `promotional_scheme.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `promotional_scheme.js` | ERPNext client script retained | external_kept | Client/UI behavior remains Frappe-owned. |

## Doctype Detail: `promotional_scheme_price_discount`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/promotional_scheme_price_discount`
Target: `src/erpnext/accounts/doctype/promotional_scheme_price_discount`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `Promotional Scheme Price Discount`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: 24 fields from `disable` through `apply_discount_on_rate`
  - `rule_description`: `Small Text`, required
  - `min_qty`, `max_qty`: `Float`, default `0`, `in_list_view: 1`
  - `min_amount`, `max_amount`: `Currency`, default `0`, `in_list_view: 1`
  - `rate_or_discount`: `Select`, default `Discount Percentage`, options blank, `Rate`, `Discount Percentage`, `Discount Amount`
  - `apply_discount_on_rate`: `Check`, default `0`, depends on discount type and multiple pricing rules

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `promotional_scheme_price_discount.py` | `promotional_scheme_price_discount.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `promotional_scheme_price_discount.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `promotional_scheme_product_discount`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/promotional_scheme_product_discount`
Target: `src/erpnext/accounts/doctype/promotional_scheme_product_discount`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `Promotional Scheme Product Discount`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: 26 fields from `disable` through `apply_recursion_over`
  - `rule_description`: `Small Text`, required
  - `same_item`: `Check`, default `0`, depends on parent mixed condition
  - `free_item`: `Link`, options `Item`, visible when not same item or parent has mixed conditions, `in_list_view: 1`
  - `recurse_for` and `apply_recursion_over`: `Float`, default `0`, depend on and are mandatory when recursive

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `promotional_scheme_product_discount.py` | `promotional_scheme_product_discount.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `promotional_scheme_product_discount.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `purchase_invoice_advance`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/purchase_invoice_advance`
Target: `src/erpnext/accounts/doctype/purchase_invoice_advance`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `Purchase Invoice Advance`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `index_web_pages_for_search`: enabled
  - `istable`: enabled
  - `field_order`: `reference_type`, `reference_name`, `remarks`, `reference_row`, `col_break1`, `advance_amount`, `allocated_amount`, `exchange_gain_loss`, `ref_exchange_rate`, `difference_posting_date`
  - `reference_name`: `Dynamic Link`, options `reference_type`, read-only, `in_list_view: 1`, `columns: 2`, no-copy
  - `advance_amount`: `Currency`, options `party_account_currency`, read-only, `in_list_view: 1`, `columns: 2`, no-copy
  - `allocated_amount`: `Currency`, options `party_account_currency`, `in_list_view: 1`, `columns: 2`, no-copy

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `purchase_invoice_advance.py` | `purchase_invoice_advance.rs` | parity_tested | No-op child table controller and metadata represented in Rust. |
| `purchase_invoice_advance.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `purchase_invoice_item`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/purchase_invoice_item`
Target: `src/erpnext/accounts/doctype/purchase_invoice_item`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `Purchase Invoice Item`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: 115 fields from `item_code` through `page_break`
  - `item_code`: `Link`, options `Item`, print hidden, `in_list_view: 1`, `columns: 3`
  - `item_name`: `Data`, required, fetched from `item_code.item_name`
  - `qty`: `Float`, required, `in_list_view: 1`, `columns: 2`
  - `amount`: `Currency`, options `currency`, required, read-only, `in_list_view: 1`, `columns: 2`
  - `add_serial_batch_bundle`: `Button`, depends on serial/batch field state and draft docstatus

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `purchase_invoice_item.py` | `purchase_invoice_item.rs` | parity_tested | No-op child table controller, full field order, key metadata, and controller behavior represented in Rust. |
| `purchase_invoice_item.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `purchase_taxes_and_charges`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/purchase_taxes_and_charges`
Target: `src/erpnext/accounts/doctype/purchase_taxes_and_charges`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- No custom hooks, validation, or calculations are defined; the class body is `pass`.
- DocType metadata:
  - `name`: `Purchase Taxes and Charges`
  - `module`: `Accounts`
  - `editable_grid`: enabled
  - `istable`: enabled
  - `field_order`: 29 fields from `category` through `dont_recompute_tax`
  - `category`: `Select`, required, default `Total`, options `Valuation and Total`, `Valuation`, `Total`
  - `charge_type`: `Select`, required, default `On Net Total`, options blank, `Actual`, `On Net Total`, `On Previous Row Amount`, `On Previous Row Total`, `On Item Quantity`
  - `account_head`: `Link`, options `Account`, required, `in_list_view: 1`, `columns: 2`
  - `tax_amount`: `Currency`, options `currency`, `in_list_view: 1`, `columns: 2`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `purchase_taxes_and_charges.py` | `purchase_taxes_and_charges.rs` | parity_tested | No-op child table controller, field order, key metadata, and controller behavior represented in Rust. |
| `purchase_taxes_and_charges.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |

## Doctype Detail: `purchase_taxes_and_charges_template`

Source: `../erpnext/apps/erpnext/erpnext/accounts/doctype/purchase_taxes_and_charges_template`
Target: `src/erpnext/accounts/doctype/purchase_taxes_and_charges_template`

### Behavior

- Python controller inherits `frappe.model.document.Document`.
- `validate` delegates to shared `valdiate_taxes_and_charges_template`.
- `autoname` sets `name` to `{title} - {company_abbr}` when both company and title exist.
- DocType metadata:
  - `name`: `Purchase Taxes and Charges Template`
  - `module`: `Accounts`
  - `allow_rename`: enabled
  - `field_order`: `title`, `is_default`, `disabled`, `column_break4`, `company`, `tax_category`, `section_break6`, `taxes`
  - `title`: `Data`, required, no-copy
  - `company`: `Link`, options `Company`, required, `in_list_view: 1`
  - `taxes`: `Table`, options `Purchase Taxes and Charges`

### File Status

| Source File | Target / Handling | Status | Notes |
|---|---|---|---|
| `__init__.py` | `mod.rs` | parity_tested | Python package marker represented by Rust module declarations. |
| `purchase_taxes_and_charges_template.py` | `purchase_taxes_and_charges_template.rs` | parity_tested | Validate delegation, autoname behavior, hooks, and metadata represented in Rust. |
| `test_purchase_taxes_and_charges_template.py` | `tests/accounts_purchase_taxes_and_charges_template.rs` | parity_tested | Source test class is empty; Rust tests cover deterministic controller behavior. |
| `purchase_taxes_and_charges_template.json` | ERPNext metadata retained | external_kept | Runtime DocType schema remains owned by ERPNext/Frappe. Rust mirrors behavior-relevant metadata constants. |
| `purchase_taxes_and_charges_template.js` | ERPNext client script retained | external_kept | Client/UI behavior remains Frappe-owned. |
