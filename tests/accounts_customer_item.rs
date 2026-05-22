use tokio_erp::erpnext::accounts::doctype::customer_item::customer_item::CustomerItem;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn customer_item_matches_erpnext_metadata() {
    assert_eq!(CustomerItem::DOCTYPE, "Customer Item");
    assert_eq!(CustomerItem::MODULE, "Accounts");
    assert_eq!(CustomerItem::FIELD_ORDER, ["customer"]);
    assert!(CustomerItem::IS_TABLE);
    assert!(CustomerItem::EDITABLE_GRID);
    assert!(CustomerItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(CustomerItem::TRACK_CHANGES);

    assert_eq!(
        CustomerItem::fields(),
        vec![FieldSpec::link("customer", "Customer ")
            .options("Customer")
            .in_list_view()]
    );
}

#[test]
fn customer_item_preserves_pass_controller_behavior() {
    let blank = CustomerItem::default();
    assert_eq!(blank.customer, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CustomerItem::new("Acme");
    assert_eq!(row.customer.as_deref(), Some("Acme"));
    assert_eq!(row.doctype(), "Customer Item");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
