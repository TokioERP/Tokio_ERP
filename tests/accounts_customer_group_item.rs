use tokio_erp::erpnext::accounts::doctype::customer_group_item::customer_group_item::CustomerGroupItem;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn customer_group_item_matches_erpnext_metadata() {
    assert_eq!(CustomerGroupItem::DOCTYPE, "Customer Group Item");
    assert_eq!(CustomerGroupItem::MODULE, "Accounts");
    assert_eq!(CustomerGroupItem::FIELD_ORDER, ["customer_group"]);
    assert!(CustomerGroupItem::IS_TABLE);
    assert!(CustomerGroupItem::EDITABLE_GRID);
    assert!(CustomerGroupItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(CustomerGroupItem::TRACK_CHANGES);

    assert_eq!(
        CustomerGroupItem::fields(),
        vec![FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .in_list_view()]
    );
}

#[test]
fn customer_group_item_preserves_pass_controller_behavior() {
    let blank = CustomerGroupItem::default();
    assert_eq!(blank.customer_group, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CustomerGroupItem::new("Retail");
    assert_eq!(row.customer_group.as_deref(), Some("Retail"));
    assert_eq!(row.doctype(), "Customer Group Item");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
