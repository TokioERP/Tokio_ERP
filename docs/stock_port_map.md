# Stock Module Port Map

This map tracks Stock source files whose Rust parity surface is complete.

## Root

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/__init__.py` | `src/erpnext/stock/mod.rs` | parity_tested | Rust covers ERPNext install doc order/fields, warehouse account map cache rebuild rules including company and test-mode behavior, Warehouse `get_all` query plan, parent warehouse account inheritance, missing-parent rebuild-tree action, ancestor-account SQL plan and first-row fallback, company default inventory account cached lookup, Stock account fallback lookup, group warehouse no-throw behavior, and exact missing-account error message. Live Frappe flag storage, DB execution, translation, and nested-set rebuild execution remain adapter/runtime integration. |

## Dashboard

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/dashboard/__init__.py` | `src/erpnext/stock/dashboard/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Stock dashboard module. |
| `stock/dashboard/item_dashboard.py` | `src/erpnext/stock/dashboard/item_dashboard.rs` | parity_tested | Rust covers ERPNext item/warehouse/item-group filter order, warehouse permission branches, Bin query plan fields/or-filters/order/limits, stock reservation item/warehouse request selection, item cached-value enrichment, HTML escaping, quick-entry batch/serial guard, Frappe legacy float rounding for quantity fields, unrounded valuation rate preservation, and reserved-stock fallback. Live Frappe DB reads, item-group tree SQL, cached Item fetches, and SRE query execution remain external integration. |
| `stock/dashboard/warehouse_capacity_dashboard.py` | `src/erpnext/stock/dashboard/warehouse_capacity_dashboard.rs` | parity_tested | Rust covers ERPNext filter construction order, parent warehouse descendant filter shape, warehouse permission filter branches, Putaway Rule query plan fields/limits, stock balance row update, HTML escaping, percent occupied math with Frappe legacy banker rounding, numeric sort order, and zero-capacity division error. Live Frappe DB reads and `get_stock_balance` execution remain external integration. |

## Dashboard Chart Source

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/dashboard_chart_source/__init__.py` | `src/erpnext/stock/dashboard_chart_source/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Stock dashboard chart source module. |
| `stock/dashboard_chart_source/stock_value_by_item_group/__init__.py` | `src/erpnext/stock/dashboard_chart_source/stock_value_by_item_group/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching stock-value-by-item-group module. |
| `stock/dashboard_chart_source/stock_value_by_item_group/stock_value_by_item_group.py` | `src/erpnext/stock/dashboard_chart_source/stock_value_by_item_group/stock_value_by_item_group.rs` | parity_tested | Rust covers ERPNext filter company/default company fallback, Warehouse filter shape, Bin-Item join query plan, optional warehouse `isin` condition, top-10 grouping/order metadata, zero stock-value row skip, and returned chart dataset shape. Live Frappe query execution, translation, cache decorator behavior, and JSON filter parsing remain external integration. |
| `stock/dashboard_chart_source/warehouse_wise_stock_value/__init__.py` | `src/erpnext/stock/dashboard_chart_source/warehouse_wise_stock_value/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching warehouse-wise-stock-value module. |
| `stock/dashboard_chart_source/warehouse_wise_stock_value/warehouse_wise_stock_value.py` | `src/erpnext/stock/dashboard_chart_source/warehouse_wise_stock_value/warehouse_wise_stock_value.rs` | parity_tested | Rust covers ERPNext company Warehouse filter shape, warehouse list name ordering, Bin aggregate query filters/group/order/limit, empty result returning an empty list, and bar chart dataset shape. Live Frappe query execution, translation, cache decorator behavior, and JSON filter parsing remain external integration. |

## Doctype

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/__init__.py` | `src/erpnext/stock/doctype/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Stock doctype module. |

## Doctype / Batch

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/batch/__init__.py` | `src/erpnext/stock/doctype/batch/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Batch doctype module. |
| `stock/doctype/batch/batch_dashboard.py` | `src/erpnext/stock/doctype/batch/batch_dashboard.rs` | parity_tested | Rust preserves the exact frontend dashboard payload contract: fieldname `batch_no`, transaction section order, translated label source strings, and item lists for Buy, Sell, Move, and Quality. Runtime translation remains adapter integration. |
