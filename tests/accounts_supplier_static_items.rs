use tokio_erp::erpnext::accounts::doctype::south_africa_vat_account::south_africa_vat_account::SouthAfricaVATAccount;
use tokio_erp::erpnext::accounts::doctype::supplier_group_item::supplier_group_item::SupplierGroupItem;
use tokio_erp::erpnext::accounts::doctype::supplier_item::supplier_item::SupplierItem;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn south_africa_vat_account_matches_erpnext_metadata() {
    assert_eq!(SouthAfricaVATAccount::DOCTYPE, "South Africa VAT Account");
    assert_eq!(SouthAfricaVATAccount::MODULE, "Accounts");
    assert_eq!(SouthAfricaVATAccount::AUTONAME, "account");
    assert_eq!(SouthAfricaVATAccount::FIELD_ORDER, ["account"]);
    assert!(SouthAfricaVATAccount::IS_TABLE);
    assert!(SouthAfricaVATAccount::EDITABLE_GRID);
    assert!(SouthAfricaVATAccount::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(SouthAfricaVATAccount::TRACK_CHANGES);
    assert_eq!(
        SouthAfricaVATAccount::fields(),
        vec![FieldSpec::link("account", "Account")
            .options("Account")
            .in_list_view()]
    );
}

#[test]
fn supplier_group_item_matches_erpnext_metadata() {
    assert_eq!(SupplierGroupItem::DOCTYPE, "Supplier Group Item");
    assert_eq!(SupplierGroupItem::MODULE, "Accounts");
    assert_eq!(SupplierGroupItem::FIELD_ORDER, ["supplier_group"]);
    assert!(SupplierGroupItem::IS_TABLE);
    assert!(SupplierGroupItem::EDITABLE_GRID);
    assert!(SupplierGroupItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(SupplierGroupItem::TRACK_CHANGES);
    assert_eq!(
        SupplierGroupItem::fields(),
        vec![FieldSpec::link("supplier_group", "Supplier Group")
            .options("Supplier Group")
            .in_list_view()]
    );
}

#[test]
fn supplier_item_matches_erpnext_metadata() {
    assert_eq!(SupplierItem::DOCTYPE, "Supplier Item");
    assert_eq!(SupplierItem::MODULE, "Accounts");
    assert_eq!(SupplierItem::FIELD_ORDER, ["supplier"]);
    assert!(SupplierItem::IS_TABLE);
    assert!(SupplierItem::EDITABLE_GRID);
    assert!(SupplierItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(SupplierItem::TRACK_CHANGES);
    assert_eq!(
        SupplierItem::fields(),
        vec![FieldSpec::link("supplier", "Supplier")
            .options("Supplier")
            .in_list_view()]
    );
}

#[test]
fn supplier_static_items_preserve_pass_controller_behavior() {
    let vat = SouthAfricaVATAccount::new("VAT - TC");
    assert_eq!(vat.account.as_deref(), Some("VAT - TC"));
    assert_eq!(vat.doctype(), "South Africa VAT Account");
    assert_eq!(vat.module(), "Accounts");
    assert!(vat.custom_hooks().is_empty());

    let group = SupplierGroupItem::new("Services");
    assert_eq!(group.supplier_group.as_deref(), Some("Services"));
    assert_eq!(group.doctype(), "Supplier Group Item");
    assert_eq!(group.module(), "Accounts");
    assert!(group.custom_hooks().is_empty());

    let supplier = SupplierItem::new("_Test Supplier");
    assert_eq!(supplier.supplier.as_deref(), Some("_Test Supplier"));
    assert_eq!(supplier.doctype(), "Supplier Item");
    assert_eq!(supplier.module(), "Accounts");
    assert!(supplier.custom_hooks().is_empty());
}
