use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CampaignItem {
    pub campaign: Option<String>,
}

impl CampaignItem {
    pub const DOCTYPE: &'static str = "Campaign Item";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["campaign"];
    pub const IS_TABLE: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(campaign: impl Into<String>) -> Self {
        Self {
            campaign: Some(campaign.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("campaign", "Campaign")
            .options("UTM Campaign")
            .in_list_view()]
    }
}

impl DocumentController for CampaignItem {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
