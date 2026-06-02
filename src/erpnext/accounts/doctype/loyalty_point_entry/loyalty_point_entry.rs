use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoyaltyPointEntry {
    pub loyalty_program: Option<String>,
    pub loyalty_program_tier: Option<String>,
    pub customer: Option<String>,
    pub invoice_type: Option<String>,
    pub invoice: Option<String>,
    pub redeem_against: Option<String>,
    pub loyalty_points: i32,
    pub purchase_amount: f64,
    pub expiry_date: Option<String>,
    pub posting_date: Option<String>,
    pub company: Option<String>,
    pub discretionary_reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoyaltyPointQueryPlan {
    pub doctype: &'static str,
    pub fields: Vec<&'static str>,
    pub filters: Vec<(&'static str, &'static str, String)>,
    pub order_by: Option<&'static str>,
    pub group_by: Option<&'static str>,
    pub select: &'static str,
}

impl LoyaltyPointEntry {
    pub const DOCTYPE: &'static str = "Loyalty Point Entry";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 12] = [
        "loyalty_program",
        "loyalty_program_tier",
        "customer",
        "invoice_type",
        "invoice",
        "redeem_against",
        "loyalty_points",
        "purchase_amount",
        "expiry_date",
        "posting_date",
        "company",
        "discretionary_reason",
    ];
    pub const EXCLUDE_FROM_LINKED_WITH: bool = true;
    pub const IN_CREATE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TITLE_FIELD: &'static str = "customer";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("loyalty_program", "Loyalty Program")
                .options("Loyalty Program")
                .required(),
            FieldSpec::data("loyalty_program_tier", "Loyalty Program Tier"),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .in_list_view()
                .required(),
            FieldSpec::link("invoice_type", "Invoice Type")
                .options("DocType")
                .required(),
            FieldSpec::dynamic_link("invoice")
                .label("Invoice")
                .options("invoice_type")
                .in_list_view(),
            FieldSpec::link("redeem_against", "Redeem Against").options("Loyalty Point Entry"),
            FieldSpec::int("loyalty_points", "Loyalty Points")
                .in_list_view()
                .required(),
            FieldSpec::currency("purchase_amount", "Purchase Amount"),
            FieldSpec::date("expiry_date", "Expiry Date")
                .in_list_view()
                .required(),
            FieldSpec::date("posting_date", "Posting Date").required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::data("discretionary_reason", "Discretionary Reason"),
        ]
    }
}

impl DocumentController for LoyaltyPointEntry {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn get_loyalty_point_entries_plan(
    customer: &str,
    loyalty_program: &str,
    company: &str,
    expiry_date: Option<&str>,
    today: &str,
) -> LoyaltyPointQueryPlan {
    let expiry_date = expiry_date.unwrap_or(today);
    LoyaltyPointQueryPlan {
        doctype: LoyaltyPointEntry::DOCTYPE,
        fields: vec![
            "name",
            "loyalty_points",
            "expiry_date",
            "loyalty_program_tier",
            "invoice_type",
            "invoice",
        ],
        filters: vec![
            ("customer", "=", customer.to_string()),
            ("loyalty_program", "=", loyalty_program.to_string()),
            ("expiry_date", ">=", expiry_date.to_string()),
            ("loyalty_points", ">", "0".to_string()),
            ("company", "=", company.to_string()),
        ],
        order_by: Some("expiry_date"),
        group_by: None,
        select: "name, loyalty_points, expiry_date, loyalty_program_tier, invoice_type, invoice",
    }
}

pub fn get_redemption_details_plan(
    customer: &str,
    loyalty_program: &str,
    company: &str,
) -> LoyaltyPointQueryPlan {
    LoyaltyPointQueryPlan {
        doctype: LoyaltyPointEntry::DOCTYPE,
        fields: vec!["redeem_against", "sum(loyalty_points)"],
        filters: vec![
            ("customer", "=", customer.to_string()),
            ("loyalty_program", "=", loyalty_program.to_string()),
            ("loyalty_points", "<", "0".to_string()),
            ("company", "=", company.to_string()),
        ],
        order_by: None,
        group_by: Some("redeem_against"),
        select: "redeem_against, sum(loyalty_points)",
    }
}
