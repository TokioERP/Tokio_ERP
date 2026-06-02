use crate::erpnext::accounts::doctype::sales_invoice_item::sales_invoice_item::{
    SalesInvoiceItem, SalesInvoiceItemError,
};
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PosInvoiceItem {
    pub base: SalesInvoiceItem,
}

impl PosInvoiceItem {
    pub const DOCTYPE: &'static str = "POS Invoice Item";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "hash";
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const FIELD_ORDER: [&'static str; 98] = [
        "barcode",
        "has_item_scanned",
        "item_code",
        "col_break1",
        "item_name",
        "customer_item_code",
        "description_section",
        "description",
        "item_group",
        "brand",
        "image_section",
        "image",
        "image_view",
        "quantity_and_rate",
        "qty",
        "stock_uom",
        "col_break2",
        "uom",
        "conversion_factor",
        "stock_qty",
        "section_break_17",
        "price_list_rate",
        "base_price_list_rate",
        "discount_and_margin",
        "margin_type",
        "margin_rate_or_amount",
        "rate_with_margin",
        "column_break_19",
        "discount_percentage",
        "discount_amount",
        "distributed_discount_amount",
        "base_rate_with_margin",
        "section_break1",
        "rate",
        "amount",
        "item_tax_template",
        "col_break3",
        "base_rate",
        "base_amount",
        "pricing_rules",
        "is_free_item",
        "grant_commission",
        "section_break_21",
        "net_rate",
        "net_amount",
        "column_break_24",
        "base_net_rate",
        "base_net_amount",
        "drop_ship",
        "delivered_by_supplier",
        "accounting",
        "income_account",
        "is_fixed_asset",
        "asset",
        "finance_book",
        "col_break4",
        "expense_account",
        "deferred_revenue",
        "deferred_revenue_account",
        "service_stop_date",
        "enable_deferred_revenue",
        "column_break_50",
        "service_start_date",
        "service_end_date",
        "section_break_18",
        "weight_per_unit",
        "total_weight",
        "column_break_21",
        "weight_uom",
        "warehouse_and_reference",
        "warehouse",
        "target_warehouse",
        "quality_inspection",
        "serial_and_batch_bundle",
        "use_serial_batch_fields",
        "col_break5",
        "allow_zero_valuation_rate",
        "item_tax_rate",
        "actual_batch_qty",
        "actual_qty",
        "section_break_tlhi",
        "serial_no",
        "column_break_ciit",
        "batch_no",
        "edit_references",
        "sales_order",
        "so_detail",
        "pos_invoice_item",
        "column_break_74",
        "delivery_note",
        "dn_detail",
        "delivered_qty",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "section_break_54",
        "page_break",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("barcode", "Barcode").print_hide(),
            FieldSpec::check("has_item_scanned", "Has Item Scanned")
                .default("0")
                .read_only()
                .depends_on("barcode"),
            FieldSpec::link("item_code", "Item")
                .options("Item")
                .oldfield("item_code", "Link")
                .columns(4)
                .bold()
                .in_list_view()
                .search_index(),
            FieldSpec::data("item_name", "Item Name")
                .oldfield("item_name", "Data")
                .in_global_search()
                .print_hide()
                .required(),
            FieldSpec::data("customer_item_code", "Customer's Item Code")
                .print_hide()
                .read_only(),
            FieldSpec::text_editor("description", "Description")
                .oldfield("description", "Text")
                .width("200px"),
            FieldSpec::attach("image", "Image"),
            FieldSpec::image("image_view", "Image View")
                .options("image")
                .print_hide(),
            FieldSpec::float("qty", "Quantity")
                .oldfield("qty", "Currency")
                .columns(2)
                .bold()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .oldfield("export_amount", "Currency")
                .columns(2)
                .read_only()
                .required()
                .in_list_view(),
            FieldSpec::link("income_account", "Income Account")
                .options("Account")
                .oldfield("income_account", "Link")
                .width("120px")
                .print_hide()
                .required(),
            FieldSpec::check("is_fixed_asset", "Is Fixed Asset")
                .default("0")
                .print_hide()
                .read_only()
                .no_copy(),
            FieldSpec::link("asset", "Asset").options("Asset").no_copy(),
            FieldSpec::link("deferred_revenue_account", "Deferred Revenue Account")
                .options("Account")
                .depends_on("enable_deferred_revenue"),
            FieldSpec::date("service_stop_date", "Service Stop Date")
                .allow_on_submit()
                .no_copy()
                .depends_on("enable_deferred_revenue"),
            FieldSpec::check("enable_deferred_revenue", "Enable Deferred Revenue").default("0"),
            FieldSpec::link("warehouse", "Warehouse")
                .options("Warehouse")
                .oldfield("warehouse", "Link")
                .in_list_view()
                .print_hide(),
            FieldSpec::float("actual_qty", "Available Qty at Warehouse")
                .oldfield("actual_qty", "Currency")
                .print_hide()
                .read_only()
                .allow_on_submit(),
            FieldSpec::data("pos_invoice_item", "POS Invoice Item")
                .print_hide()
                .read_only()
                .no_copy(),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .default(":Company")
                .oldfield("cost_center", "Link")
                .width("120px")
                .print_hide()
                .required(),
            FieldSpec::link("project", "Project").options("Project"),
            FieldSpec::check("page_break", "Page Break")
                .default("0")
                .print_hide()
                .allow_on_submit()
                .no_copy(),
        ]
    }

    pub fn from_sales_invoice_item(base: SalesInvoiceItem) -> Self {
        Self { base }
    }

    pub fn sales_invoice_item(&self) -> &SalesInvoiceItem {
        &self.base
    }

    pub fn sales_invoice_item_mut(&mut self) -> &mut SalesInvoiceItem {
        &mut self.base
    }

    pub fn validate_cost_center(
        &self,
        company: &str,
        cost_center_company: Option<&str>,
    ) -> Result<(), SalesInvoiceItemError> {
        self.base.validate_cost_center(company, cost_center_company)
    }

    pub fn set_actual_qty(&mut self, bin_actual_qty: Option<f64>) {
        self.base.set_actual_qty(bin_actual_qty);
    }

    pub fn set_income_account_for_fixed_asset(
        &mut self,
        disposal_account: &str,
        depreciation_cost_center: Option<&str>,
    ) {
        self.base
            .set_income_account_for_fixed_asset(disposal_account, depreciation_cost_center);
    }
}

impl DocumentController for PosInvoiceItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
