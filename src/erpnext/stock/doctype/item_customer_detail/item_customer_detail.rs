use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ItemCustomerDetail {
    pub customer_name: Option<String>,
    pub customer_group: Option<String>,
    pub ref_code: Option<String>,
    pub parent: Option<String>,
    pub parentfield: Option<String>,
    pub parenttype: Option<String>,
}

impl ItemCustomerDetail {
    pub const DOCTYPE: &'static str = "Item Customer Detail";
    pub const MODULE: &'static str = "Stock";
    pub const FIELD_ORDER: [&'static str; 3] = ["customer_name", "customer_group", "ref_code"];
    pub const EDITABLE_GRID: bool = true;
    pub const IS_TABLE: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn new(
        customer_name: Option<&str>,
        customer_group: Option<&str>,
        ref_code: impl Into<String>,
        parent: Option<&str>,
        parentfield: Option<&str>,
        parenttype: Option<&str>,
    ) -> Self {
        Self {
            customer_name: customer_name.map(ToOwned::to_owned),
            customer_group: customer_group.map(ToOwned::to_owned),
            ref_code: Some(ref_code.into()),
            parent: parent.map(ToOwned::to_owned),
            parentfield: parentfield.map(ToOwned::to_owned),
            parenttype: parenttype.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("customer_name", "Customer Name")
                .options("Customer")
                .oldfield("price_list_name", "Select")
                .in_filter()
                .in_list_view()
                .search_index()
                .bold()
                .width("180px"),
            FieldSpec::link("customer_group", "Customer Group")
                .options("Customer Group")
                .in_filter()
                .in_list_view()
                .bold(),
            FieldSpec::data("ref_code", "Ref Code")
                .description("Enter the Item Code that this customer uses at their end. This will be shown in Sales Orders for the customer's reference.")
                .oldfield("ref_rate", "Currency")
                .in_filter()
                .in_list_view()
                .required()
                .search_index()
                .width("120px"),
        ]
    }
}

impl DocumentController for ItemCustomerDetail {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
