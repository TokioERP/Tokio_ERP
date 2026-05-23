use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PurchaseInvoiceItem {
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub qty: Option<String>,
    pub rate: Option<String>,
}

impl PurchaseInvoiceItem {
    pub const DOCTYPE: &'static str = "Purchase Invoice Item";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: &'static [&'static str] = &[
        "item_code",
        "product_bundle",
        "col_break1",
        "item_name",
        "description_section",
        "description",
        "brand",
        "col_break7",
        "item_group",
        "image",
        "image_view",
        "quantity_and_rate",
        "received_qty",
        "qty",
        "rejected_qty",
        "col_break2",
        "uom",
        "conversion_factor",
        "stock_uom",
        "stock_qty",
        "sec_break1",
        "price_list_rate",
        "col_break3",
        "base_price_list_rate",
        "section_break_26",
        "margin_type",
        "margin_rate_or_amount",
        "rate_with_margin",
        "column_break_30",
        "discount_percentage",
        "discount_amount",
        "distributed_discount_amount",
        "base_rate_with_margin",
        "sec_break2",
        "rate",
        "amount",
        "item_tax_template",
        "tax_withholding_category",
        "col_break4",
        "base_rate",
        "base_amount",
        "pricing_rules",
        "stock_uom_rate",
        "is_free_item",
        "apply_tds",
        "allow_zero_valuation_rate",
        "section_break_22",
        "net_rate",
        "net_amount",
        "column_break_25",
        "base_net_rate",
        "base_net_amount",
        "valuation_rate",
        "sales_incoming_rate",
        "item_tax_amount",
        "landed_cost_voucher_amount",
        "rm_supp_cost",
        "warehouse_section",
        "warehouse",
        "add_serial_batch_bundle",
        "serial_and_batch_bundle",
        "use_serial_batch_fields",
        "col_br_wh",
        "from_warehouse",
        "quality_inspection",
        "rejected_warehouse",
        "rejected_serial_and_batch_bundle",
        "section_break_rqbe",
        "serial_no",
        "rejected_serial_no",
        "column_break_vbbb",
        "batch_no",
        "manufacture_details",
        "manufacturer",
        "column_break_13",
        "manufacturer_part_no",
        "accounting",
        "expense_account",
        "wip_composite_asset",
        "col_break5",
        "is_fixed_asset",
        "asset_location",
        "asset_category",
        "deferred_expense_section",
        "deferred_expense_account",
        "service_stop_date",
        "enable_deferred_expense",
        "column_break_58",
        "service_start_date",
        "service_end_date",
        "reference",
        "item_tax_rate",
        "bom",
        "include_exploded_items",
        "purchase_invoice_item",
        "col_break6",
        "purchase_order",
        "po_detail",
        "purchase_receipt",
        "pr_detail",
        "sales_invoice_item",
        "material_request",
        "material_request_item",
        "delivered_by_supplier",
        "item_weight_details",
        "weight_per_unit",
        "total_weight",
        "column_break_38",
        "weight_uom",
        "accounting_dimensions_section",
        "project",
        "dimension_col_break",
        "cost_center",
        "section_break_82",
        "page_break",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(item_code: impl Into<String>) -> Self {
        Self {
            item_code: Some(item_code.into()),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        Self::FIELD_ORDER
            .iter()
            .map(|field| Self::field_for(field))
            .collect()
    }

    fn field_for(fieldname: &'static str) -> FieldSpec {
        match fieldname {
            "item_code" => FieldSpec::link("item_code", "Item")
                .options("Item")
                .print_hide()
                .in_list_view()
                .columns(3),
            "item_name" => FieldSpec::data("item_name", "Item Name")
                .required()
                .fetch_from("item_code.item_name"),
            "qty" => FieldSpec::float("qty", "Accepted Qty")
                .required()
                .in_list_view()
                .columns(2),
            "amount" => FieldSpec::currency("amount", "Amount")
                .options("currency")
                .required()
                .read_only()
                .in_list_view()
                .columns(2),
            "add_serial_batch_bundle" => {
                FieldSpec::button("add_serial_batch_bundle", "Add Serial / Batch No")
                    .depends_on("eval:doc.use_serial_batch_fields === 0 && doc.docstatus === 0")
            }
            field if field.starts_with("col_break") || field.starts_with("column_break") => {
                FieldSpec::column_break(fieldname)
            }
            field if field.contains("section") || field.starts_with("sec_break") => {
                FieldSpec::section_break(fieldname)
            }
            _ => FieldSpec::data(fieldname, fieldname),
        }
    }
}

impl DocumentController for PurchaseInvoiceItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
