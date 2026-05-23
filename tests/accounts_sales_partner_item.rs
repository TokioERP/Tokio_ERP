use tokio_erp::erpnext::accounts::doctype::sales_partner_item::sales_partner_item::SalesPartnerItem;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_partner_item_matches_erpnext_metadata() {
    assert_eq!(SalesPartnerItem::DOCTYPE, "Sales Partner Item");
    assert_eq!(SalesPartnerItem::MODULE, "Accounts");
    assert_eq!(SalesPartnerItem::FIELD_ORDER, ["sales_partner"]);
    assert!(SalesPartnerItem::IS_TABLE);
    assert!(SalesPartnerItem::EDITABLE_GRID);
    assert!(SalesPartnerItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(SalesPartnerItem::TRACK_CHANGES);
    assert_eq!(
        SalesPartnerItem::fields(),
        vec![FieldSpec::link("sales_partner", "Sales Partner ")
            .options("Sales Partner")
            .in_list_view()]
    );
}

#[test]
fn sales_partner_item_controller_is_pass_through() {
    let row = SalesPartnerItem::new("SP-0001");
    assert_eq!(row.sales_partner.as_deref(), Some("SP-0001"));
    assert_eq!(row.doctype(), "Sales Partner Item");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
