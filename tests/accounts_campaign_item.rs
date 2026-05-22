use tokio_erp::erpnext::accounts::doctype::campaign_item::campaign_item::CampaignItem;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn campaign_item_matches_erpnext_metadata() {
    assert_eq!(CampaignItem::DOCTYPE, "Campaign Item");
    assert_eq!(CampaignItem::MODULE, "Accounts");
    assert_eq!(CampaignItem::FIELD_ORDER, ["campaign"]);
    assert!(CampaignItem::IS_TABLE);
    assert!(CampaignItem::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(CampaignItem::TRACK_CHANGES);

    assert_eq!(
        CampaignItem::fields(),
        vec![FieldSpec::link("campaign", "Campaign")
            .options("UTM Campaign")
            .in_list_view()]
    );
}

#[test]
fn campaign_item_preserves_pass_controller_behavior() {
    let blank = CampaignItem::default();
    assert_eq!(blank.campaign, None);
    assert!(blank.custom_hooks().is_empty());

    let row = CampaignItem::new("Spring Campaign");
    assert_eq!(row.campaign.as_deref(), Some("Spring Campaign"));
    assert_eq!(row.doctype(), "Campaign Item");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
