use tokio_erp::erpnext::accounts::doctype::pos_customer_group::pos_customer_group::PosCustomerGroup;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_customer_group_matches_erpnext_metadata() {
    assert_eq!(PosCustomerGroup::DOCTYPE, "POS Customer Group");
    assert_eq!(PosCustomerGroup::MODULE, "Accounts");
    assert_eq!(PosCustomerGroup::FIELD_ORDER, ["customer_group"]);
    assert!(PosCustomerGroup::IS_TABLE);
    assert!(PosCustomerGroup::EDITABLE_GRID);

    assert_eq!(
        PosCustomerGroup::fields(),
        vec![FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .required()
            .in_list_view(),]
    );
}

#[test]
fn pos_customer_group_preserves_pass_controller_behavior() {
    let blank = PosCustomerGroup::default();
    assert_eq!(blank.customer_group, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosCustomerGroup::new("All Customer Groups");
    assert_eq!(row.customer_group.as_deref(), Some("All Customer Groups"));
    assert_eq!(row.doctype(), "POS Customer Group");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
