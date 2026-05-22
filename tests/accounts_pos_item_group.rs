use tokio_erp::erpnext::accounts::doctype::pos_item_group::pos_item_group::PosItemGroup;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_item_group_matches_erpnext_metadata() {
    assert_eq!(PosItemGroup::DOCTYPE, "POS Item Group");
    assert_eq!(PosItemGroup::MODULE, "Accounts");
    assert_eq!(PosItemGroup::FIELD_ORDER, ["item_group"]);
    assert!(PosItemGroup::IS_TABLE);
    assert!(PosItemGroup::EDITABLE_GRID);

    assert_eq!(
        PosItemGroup::fields(),
        vec![FieldSpec::link("item_group", "Item Group")
            .options("Item Group")
            .required()
            .in_list_view(),]
    );
}

#[test]
fn pos_item_group_preserves_pass_controller_behavior() {
    let blank = PosItemGroup::default();
    assert_eq!(blank.item_group, None);
    assert!(blank.custom_hooks().is_empty());

    let row = PosItemGroup::new("Products");
    assert_eq!(row.item_group.as_deref(), Some("Products"));
    assert_eq!(row.doctype(), "POS Item Group");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
