# Stock Module Port Map

This map tracks Stock source files whose Rust parity surface is complete.

## Dashboard

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/dashboard/__init__.py` | `src/erpnext/stock/dashboard/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Stock dashboard module. |
| `stock/dashboard/warehouse_capacity_dashboard.py` | `src/erpnext/stock/dashboard/warehouse_capacity_dashboard.rs` | parity_tested | Rust covers ERPNext filter construction order, parent warehouse descendant filter shape, warehouse permission filter branches, Putaway Rule query plan fields/limits, stock balance row update, HTML escaping, percent occupied math with Frappe legacy banker rounding, numeric sort order, and zero-capacity division error. Live Frappe DB reads and `get_stock_balance` execution remain external integration. |
