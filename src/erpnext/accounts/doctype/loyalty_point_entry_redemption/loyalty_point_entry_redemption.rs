use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LoyaltyPointEntryRedemption {
    pub sales_invoice: Option<String>,
    pub redemption_date: Option<String>,
    pub redeemed_points: i32,
}

impl LoyaltyPointEntryRedemption {
    pub const DOCTYPE: &'static str = "Loyalty Point Entry Redemption";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] =
        ["sales_invoice", "redemption_date", "redeemed_points"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        sales_invoice: impl Into<String>,
        redemption_date: impl Into<String>,
        redeemed_points: i32,
    ) -> Self {
        Self {
            sales_invoice: Some(sales_invoice.into()),
            redemption_date: Some(redemption_date.into()),
            redeemed_points,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("sales_invoice", "Sales Invoice").in_list_view(),
            FieldSpec::date("redemption_date", "Redemption Date").in_list_view(),
            FieldSpec::int("redeemed_points", "Redeemed Points").in_list_view(),
        ]
    }
}

impl DocumentController for LoyaltyPointEntryRedemption {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
