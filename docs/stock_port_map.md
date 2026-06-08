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

## Doctype / Customs Tariff Number

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/customs_tariff_number/__init__.py` | `src/erpnext/stock/doctype/customs_tariff_number/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Customs Tariff Number module. |
| `stock/doctype/customs_tariff_number/customs_tariff_number.py` | `src/erpnext/stock/doctype/customs_tariff_number/customs_tariff_number.rs` | parity_tested | Python controller is pass/no-op; Rust preserves doctype/module names, autoname, field order, allow-rename, quick-entry, sort order, track-changes, tariff number required/unique/list-view field, description list-view field, and empty controller hooks. JSON permissions remain metadata/runtime integration. |
| `stock/doctype/customs_tariff_number/test_customs_tariff_number.py` | `tests/stock_customs_tariff_number.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |

## Doctype / Delivery Settings

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/delivery_settings/__init__.py` | `src/erpnext/stock/doctype/delivery_settings/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Delivery Settings module. |
| `stock/doctype/delivery_settings/delivery_settings.py` | `src/erpnext/stock/doctype/delivery_settings/delivery_settings.rs` | parity_tested | Python controller is pass/no-op; Rust preserves singleton settings metadata, field order, frontend field specs for dispatch template/attachment/send-with-attachment/stop-delay, defaults, dependencies, descriptions, quick-entry, sort order, row format, track-changes, and empty controller hooks. JSON permissions remain metadata/runtime integration. |
| `stock/doctype/delivery_settings/test_delivery_settings.py` | `tests/stock_delivery_settings.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |

## Doctype / Delivery Stop

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/delivery_stop/__init__.py` | `src/erpnext/stock/doctype/delivery_stop/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Delivery Stop doctype module. |
| `stock/doctype/delivery_stop/delivery_stop.py` | `src/erpnext/stock/doctype/delivery_stop/delivery_stop.rs` | parity_tested | Python controller is pass/no-op; Rust preserves child-table metadata, field order, frontend field specs, defaults, list-view/print/no-copy/read-only/hidden/dependency flags, quick-entry, sort order, track-changes, and empty controller hooks. JSON permissions remain metadata/runtime integration. |

## Doctype / Item Supplier

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/item_supplier/__init__.py` | `src/erpnext/stock/doctype/item_supplier/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Item Supplier child-table module. |
| `stock/doctype/item_supplier/item_supplier.py` | `src/erpnext/stock/doctype/item_supplier/item_supplier.rs` | parity_tested | Python controller is pass/no-op; Rust preserves child-table metadata, editable-grid, sort order, track-changes, field-order driven frontend order, supplier required/list-view link metadata, supplier part number list/global-search/width metadata, parent linkage fields from generated type hints, and empty controller hooks. JSON permissions and print-width remain metadata/runtime integration. |

## Doctype / Item Attribute Value

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/item_attribute_value/__init__.py` | `src/erpnext/stock/doctype/item_attribute_value/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Item Attribute Value child-table module. |
| `stock/doctype/item_attribute_value/item_attribute_value.py` | `src/erpnext/stock/doctype/item_attribute_value/item_attribute_value.rs` | parity_tested | Python controller is pass/no-op; Rust preserves child-table metadata, editable-grid, sort order, field-order driven frontend order, attribute value required/list-view metadata, abbreviation required/list-view/search-index/description metadata, parent linkage fields from generated type hints, and empty controller hooks. JSON permissions remain metadata/runtime integration. |

## Doctype / UOM Category

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/uom_category/__init__.py` | `src/erpnext/stock/doctype/uom_category/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching UOM Category doctype module. |
| `stock/doctype/uom_category/uom_category.py` | `src/erpnext/stock/doctype/uom_category/uom_category.rs` | parity_tested | Python controller is pass/no-op; Rust preserves autoname, allow-rename, editable-grid, quick-entry, sort order, category name field metadata, required/list-view/unique flags, and empty controller hooks. JSON permissions remain metadata/runtime integration. |
| `stock/doctype/uom_category/test_uom_category.py` | `tests/stock_uom_category.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |

## Doctype / Warehouse Type

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/warehouse_type/__init__.py` | `src/erpnext/stock/doctype/warehouse_type/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Warehouse Type doctype module. |
| `stock/doctype/warehouse_type/warehouse_type.py` | `src/erpnext/stock/doctype/warehouse_type/warehouse_type.rs` | parity_tested | Python controller is pass/no-op; Rust preserves prompt autoname, quick-entry, sort order, track-changes, description Small Text field metadata, and empty controller hooks. JSON permissions remain metadata/runtime integration. |
| `stock/doctype/warehouse_type/test_warehouse_type.py` | `tests/stock_warehouse_type.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |

## Doctype / Price List Country

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/price_list_country/__init__.py` | `src/erpnext/stock/doctype/price_list_country/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Price List Country child-table module. |
| `stock/doctype/price_list_country/price_list_country.py` | `src/erpnext/stock/doctype/price_list_country/price_list_country.rs` | parity_tested | Python controller is pass/no-op; Rust preserves child-table metadata, editable-grid, sort order, Country link field metadata, required/list-view flags, parent linkage fields from generated type hints, and empty controller hooks. JSON permissions remain metadata/runtime integration. |

## Doctype / Quality Inspection Parameter Group

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/quality_inspection_parameter_group/__init__.py` | `src/erpnext/stock/doctype/quality_inspection_parameter_group/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Quality Inspection Parameter Group doctype module. |
| `stock/doctype/quality_inspection_parameter_group/quality_inspection_parameter_group.py` | `src/erpnext/stock/doctype/quality_inspection_parameter_group/quality_inspection_parameter_group.rs` | parity_tested | Python controller is pass/no-op; Rust preserves field-based autoname, editable-grid, quick-entry, sort order, track-changes, group name field metadata, required/list-view/unique flags, and empty controller hooks. JSON permissions remain metadata/runtime integration. |
| `stock/doctype/quality_inspection_parameter_group/test_quality_inspection_parameter_group.py` | `tests/stock_quality_inspection_parameter_group.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |

## Doctype / Quality Inspection Parameter

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/quality_inspection_parameter/__init__.py` | `src/erpnext/stock/doctype/quality_inspection_parameter/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Quality Inspection Parameter doctype module. |
| `stock/doctype/quality_inspection_parameter/quality_inspection_parameter.py` | `src/erpnext/stock/doctype/quality_inspection_parameter/quality_inspection_parameter.rs` | parity_tested | Python controller is pass/no-op; Rust preserves field-based autoname, editable-grid, quick-entry, sort order, track-changes, field-order driven frontend order, parameter required/list-view/unique flags, parameter group link metadata, description Text Editor metadata, and empty controller hooks. JSON permissions remain metadata/runtime integration. |
| `stock/doctype/quality_inspection_parameter/test_quality_inspection_parameter.py` | `tests/stock_quality_inspection_parameter.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |

## Doctype / Variant Field

| Source | Target | Status | Notes |
| --- | --- | --- | --- |
| `stock/doctype/variant_field/__init__.py` | `src/erpnext/stock/doctype/variant_field/mod.rs` | parity_tested | Python package marker is empty; Rust exposes the matching Variant Field child-table module. |
| `stock/doctype/variant_field/variant_field.py` | `src/erpnext/stock/doctype/variant_field/variant_field.rs` | parity_tested | Python controller is pass/no-op; Rust preserves editable-grid, quick-entry, sort order, track-changes, Autocomplete field metadata, required/list-view flags, parent linkage fields from generated type hints, and empty controller hooks. |
| `stock/doctype/variant_field/test_variant_field.py` | `tests/stock_variant_field.rs` | parity_tested | ERPNext test class is pass/no-op; Rust covers metadata and pass controller behavior directly. |
