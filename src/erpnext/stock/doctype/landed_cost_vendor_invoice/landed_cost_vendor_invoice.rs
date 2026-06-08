use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LandedCostVendorInvoice {
    pub vendor_invoice: Option<String>,
    pub amount: Option<f64>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl LandedCostVendorInvoice {
    pub const DOCTYPE: &'static str = "Landed Cost Vendor Invoice";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 2] = ["vendor_invoice", "amount"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        vendor_invoice: Option<&str>,
        amount: Option<f64>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            vendor_invoice: vendor_invoice.map(ToOwned::to_owned),
            amount,
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("vendor_invoice", "Vendor Invoice")
                .options("Purchase Invoice")
                .in_list_view()
                .search_index(),
            FieldSpec::currency("amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .in_list_view()
                .read_only(),
        ]
    }
}

impl DocumentController for LandedCostVendorInvoice {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
