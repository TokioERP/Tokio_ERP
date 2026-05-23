use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessStatementOfAccountsCustomer {
    pub customer: Option<String>,
    pub customer_name: Option<String>,
    pub billing_email: Option<String>,
    pub primary_email: Option<String>,
}

impl ProcessStatementOfAccountsCustomer {
    pub const DOCTYPE: &'static str = "Process Statement Of Accounts Customer";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 4] = [
        "customer",
        "customer_name",
        "billing_email",
        "primary_email",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(customer: impl Into<String>) -> Self {
        Self {
            customer: Some(customer.into()),
            customer_name: None,
            billing_email: None,
            primary_email: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .in_list_view(),
            FieldSpec::read_only_field("primary_email", "Primary Contact Email").in_list_view(),
            FieldSpec::data("billing_email", "Billing Email").in_list_view(),
            FieldSpec::data("customer_name", "Customer Name")
                .fetch_from("customer.customer_name")
                .read_only(),
        ]
    }
}

impl DocumentController for ProcessStatementOfAccountsCustomer {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
