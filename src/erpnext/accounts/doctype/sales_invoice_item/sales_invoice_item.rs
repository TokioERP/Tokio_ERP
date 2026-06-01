use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SalesInvoiceItem {
    pub idx: usize,
    pub item_code: Option<String>,
    pub warehouse: Option<String>,
    pub cost_center: Option<String>,
    pub income_account: Option<String>,
    pub is_fixed_asset: bool,
    pub actual_qty: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SalesInvoiceItemError {
    CostCenterWrongCompany {
        row: usize,
        cost_center: String,
        company: String,
    },
}

impl SalesInvoiceItem {
    pub const DOCTYPE: &'static str = "Sales Invoice Item";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "hash";
    pub const DOCUMENT_TYPE: &'static str = "Document";
    pub const EDITABLE_GRID: bool = true;
    pub const FIELD_ORDER: [&'static str; 115] = [
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
        "tax_withholding_category",
        "col_break3",
        "base_rate",
        "base_amount",
        "pricing_rules",
        "stock_uom_rate",
        "is_free_item",
        "apply_tds",
        "grant_commission",
        "allow_zero_valuation_rate",
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
        "discount_account",
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
        "pick_serial_and_batch",
        "serial_and_batch_bundle",
        "use_serial_batch_fields",
        "col_break5",
        "incoming_rate",
        "item_tax_rate",
        "actual_batch_qty",
        "section_break_eoec",
        "serial_no",
        "column_break_ytgd",
        "batch_no",
        "available_quantity_section",
        "actual_qty",
        "column_break_ogff",
        "company_total_stock",
        "edit_references",
        "sales_order",
        "so_detail",
        "sales_invoice_item",
        "column_break_74",
        "delivery_note",
        "dn_detail",
        "delivered_qty",
        "column_break_vwhb",
        "pos_invoice",
        "pos_invoice_item",
        "scio_detail",
        "internal_transfer_section",
        "purchase_order",
        "column_break_92",
        "purchase_order_item",
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
            FieldSpec::link("item_code", "Item")
                .options("Item")
                .oldfield("item_code", "Link")
                .columns(4)
                .bold()
                .in_list_view()
                .search_index(),
            FieldSpec::column_break("col_break1"),
            FieldSpec::data("item_name", "Item Name")
                .oldfield("item_name", "Data")
                .in_global_search()
                .print_hide()
                .required(),
            FieldSpec::data("customer_item_code", "Customer's Item Code")
                .hidden()
                .print_hide()
                .read_only(),
            FieldSpec::section_break("description_section")
                .label("Description")
                .collapsible(),
            FieldSpec::text_editor("description", "Description")
                .oldfield("description", "Text")
                .width("200px"),
            FieldSpec::image("image_view", "Image View")
                .options("image")
                .print_hide(),
            FieldSpec::section_break("quantity_and_rate"),
            FieldSpec::float("qty", "Quantity")
                .oldfield("qty", "Currency")
                .columns(2)
                .bold()
                .in_list_view(),
            FieldSpec::currency("rate", "Rate")
                .options("currency")
                .oldfield("export_rate", "Currency")
                .columns(2)
                .bold()
                .required()
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
                .required(),
            FieldSpec::check("is_fixed_asset", "Is Fixed Asset")
                .default("0")
                .read_only(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
        ]
    }

    pub fn validate_cost_center(
        &self,
        company: &str,
        cost_center_company: Option<&str>,
    ) -> Result<(), SalesInvoiceItemError> {
        if cost_center_company != Some(company) {
            return Err(SalesInvoiceItemError::CostCenterWrongCompany {
                row: self.idx,
                cost_center: self.cost_center.clone().unwrap_or_default(),
                company: company.to_string(),
            });
        }
        Ok(())
    }

    pub fn set_actual_qty(&mut self, bin_actual_qty: Option<f64>) {
        if self.item_code.is_some() && self.warehouse.is_some() {
            self.actual_qty = bin_actual_qty.unwrap_or(0.0);
        }
    }

    pub fn set_income_account_for_fixed_asset(
        &mut self,
        disposal_account: &str,
        depreciation_cost_center: Option<&str>,
    ) {
        if !self.is_fixed_asset {
            return;
        }

        self.income_account = Some(disposal_account.to_string());
        if self.cost_center.is_none() {
            self.cost_center = depreciation_cost_center.map(str::to_string);
        }
    }
}

impl DocumentController for SalesInvoiceItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
