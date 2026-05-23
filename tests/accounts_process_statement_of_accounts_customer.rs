use tokio_erp::erpnext::accounts::doctype::process_statement_of_accounts_customer::process_statement_of_accounts_customer::ProcessStatementOfAccountsCustomer;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_statement_of_accounts_customer_matches_erpnext_metadata() {
    assert_eq!(
        ProcessStatementOfAccountsCustomer::DOCTYPE,
        "Process Statement Of Accounts Customer"
    );
    assert_eq!(ProcessStatementOfAccountsCustomer::MODULE, "Accounts");
    assert_eq!(
        ProcessStatementOfAccountsCustomer::FIELD_ORDER,
        [
            "customer",
            "customer_name",
            "billing_email",
            "primary_email"
        ]
    );
    assert!(ProcessStatementOfAccountsCustomer::IS_TABLE);
    assert!(ProcessStatementOfAccountsCustomer::EDITABLE_GRID);
    assert_eq!(
        ProcessStatementOfAccountsCustomer::fields(),
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
    );
}

#[test]
fn process_statement_of_accounts_customer_preserves_pass_controller_behavior() {
    let blank = ProcessStatementOfAccountsCustomer::default();
    assert_eq!(blank.customer, None);
    assert_eq!(blank.customer_name, None);
    assert_eq!(blank.billing_email, None);
    assert_eq!(blank.primary_email, None);
    assert!(blank.custom_hooks().is_empty());

    let row = ProcessStatementOfAccountsCustomer::new("_Test Customer");
    assert_eq!(row.customer.as_deref(), Some("_Test Customer"));
    assert_eq!(row.doctype(), "Process Statement Of Accounts Customer");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
