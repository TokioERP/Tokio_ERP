use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LandedCostPurchaseReceipt {
    pub receipt_document_type: Option<String>,
    pub receipt_document: Option<String>,
    pub supplier: Option<String>,
    pub posting_date: Option<String>,
    pub grand_total: Option<f64>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl LandedCostPurchaseReceipt {
    pub const DOCTYPE: &'static str = "Landed Cost Purchase Receipt";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 6] = [
        "receipt_document_type",
        "receipt_document",
        "supplier",
        "col_break1",
        "posting_date",
        "grand_total",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "ASC";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        receipt_document_type: impl Into<String>,
        receipt_document: impl Into<String>,
        supplier: Option<&str>,
        posting_date: Option<&str>,
        grand_total: Option<f64>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            receipt_document_type: Some(receipt_document_type.into()),
            receipt_document: Some(receipt_document.into()),
            supplier: supplier.map(ToOwned::to_owned),
            posting_date: posting_date.map(ToOwned::to_owned),
            grand_total,
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("receipt_document_type", "Receipt Document Type")
                .options(
                    "\nPurchase Invoice\nPurchase Receipt\nStock Entry\nSubcontracting Receipt",
                )
                .in_list_view()
                .required(),
            FieldSpec::dynamic_link("receipt_document")
                .label("Receipt Document")
                .options("receipt_document_type")
                .oldfield("purchase_receipt_no", "Link")
                .in_list_view()
                .required()
                .width("220px"),
            FieldSpec::link("supplier", "Supplier")
                .options("Supplier")
                .in_list_view()
                .read_only(),
            FieldSpec::column_break("col_break1").width("50%"),
            FieldSpec::date("posting_date", "Posting Date").read_only(),
            FieldSpec::currency("grand_total", "Grand Total")
                .options("Company:company:default_currency")
                .in_list_view()
                .read_only(),
        ]
    }
}

impl DocumentController for LandedCostPurchaseReceipt {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
