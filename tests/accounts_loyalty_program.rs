use tokio_erp::erpnext::accounts::doctype::loyalty_program::loyalty_program::{
    get_loyalty_details_plan, get_redeemption_factor, select_loyalty_tier, validate_loyalty_points,
    LoyaltyPointValidationContext, LoyaltyPointValidationUpdate, LoyaltyProgram,
    LoyaltyProgramError,
};
use tokio_erp::erpnext::accounts::doctype::loyalty_program_collection::loyalty_program_collection::LoyaltyProgramCollection;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn loyalty_program_matches_erpnext_metadata() {
    assert_eq!(LoyaltyProgram::DOCTYPE, "Loyalty Program");
    assert_eq!(LoyaltyProgram::MODULE, "Accounts");
    assert_eq!(
        LoyaltyProgram::FIELD_ORDER,
        [
            "loyalty_program_name",
            "loyalty_program_type",
            "from_date",
            "to_date",
            "column_break_7",
            "customer_group",
            "customer_territory",
            "auto_opt_in",
            "rules",
            "collection_rules",
            "redemption",
            "conversion_factor",
            "expiry_duration",
            "column_break_10",
            "expense_account",
            "company",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
            "project",
            "help_section",
            "loyalty_program_help",
        ]
    );
    assert_eq!(LoyaltyProgram::AUTONAME, "field:loyalty_program_name");
    assert_eq!(LoyaltyProgram::SORT_FIELD, "creation");
    assert_eq!(LoyaltyProgram::SORT_ORDER, "DESC");
    assert!(LoyaltyProgram::QUICK_ENTRY);
    assert!(LoyaltyProgram::TRACK_CHANGES);

    let fields = LoyaltyProgram::fields();
    assert_eq!(fields.len(), 22);
    assert_eq!(
        fields[0],
        FieldSpec::data("loyalty_program_name", "Loyalty Program Name")
            .in_list_view()
            .required()
            .unique()
    );
    assert_eq!(
        fields[9],
        FieldSpec::table("collection_rules", "Collection Rules")
            .options("Loyalty Program Collection")
            .required()
    );
    assert_eq!(
        fields[11],
        FieldSpec::float("conversion_factor", "Conversion Factor")
            .description("1 Loyalty Points = How much base currency?")
    );
}

#[test]
fn loyalty_program_lowest_tier_validation_matches_erpnext() {
    let ok = LoyaltyProgram {
        loyalty_program_name: "Rewards".to_string(),
        collection_rules: vec![
            LoyaltyProgramCollection {
                tier_name: Some("Silver".to_string()),
                min_spent: 500.0,
                collection_factor: 1.0,
            },
            LoyaltyProgramCollection {
                tier_name: Some("Base".to_string()),
                min_spent: 0.0,
                collection_factor: 0.5,
            },
        ],
        ..Default::default()
    };
    assert_eq!(ok.custom_hooks(), ["validate"]);
    assert_eq!(ok.validate(), Ok(()));
    assert_eq!(ok.doctype(), "Loyalty Program");
    assert_eq!(ok.module(), "Accounts");

    let bad = LoyaltyProgram {
        loyalty_program_name: "Rewards".to_string(),
        collection_rules: vec![LoyaltyProgramCollection {
            tier_name: Some("Silver".to_string()),
            min_spent: 100.0,
            collection_factor: 1.0,
        }],
        ..Default::default()
    };
    assert_eq!(
        bad.validate(),
        Err(LoyaltyProgramError::LowestTierMustStartAtZero)
    );
}

#[test]
fn loyalty_program_query_and_tier_helpers_match_erpnext() {
    let details = get_loyalty_details_plan(
        "CUST-001",
        "Rewards",
        None,
        Some("Acme"),
        false,
        "2026-06-01",
    );
    assert_eq!(details.doctype, "Loyalty Point Entry");
    assert_eq!(
        details.select,
        [
            "sum(loyalty_points) as loyalty_points",
            "sum(purchase_amount) as total_spent"
        ]
    );
    assert_eq!(
        details.filters,
        vec![
            ("customer", "=", "CUST-001".to_string()),
            ("loyalty_program", "=", "Rewards".to_string()),
            ("posting_date", "<=", "2026-06-01".to_string()),
            ("company", "=", "Acme".to_string()),
            ("expiry_date", ">=", "2026-06-01".to_string()),
        ]
    );
    assert_eq!(details.group_by, Some("customer"));

    let expired = get_loyalty_details_plan(
        "CUST-001",
        "Rewards",
        Some("2026-05-01"),
        None,
        true,
        "unused",
    );
    assert_eq!(
        expired.filters,
        vec![
            ("customer", "=", "CUST-001".to_string()),
            ("loyalty_program", "=", "Rewards".to_string()),
            ("posting_date", "<=", "2026-05-01".to_string()),
        ]
    );

    let tiers = vec![
        LoyaltyProgramCollection {
            tier_name: Some("Gold".to_string()),
            min_spent: 1000.0,
            collection_factor: 2.0,
        },
        LoyaltyProgramCollection {
            tier_name: Some("Silver".to_string()),
            min_spent: 0.0,
            collection_factor: 1.0,
        },
    ];
    assert_eq!(
        select_loyalty_tier(&tiers, 900.0, 150.0),
        Some(("Gold".to_string(), 2.0))
    );
    assert_eq!(get_redeemption_factor(Some(0.25), None), Ok(0.25));
    assert_eq!(
        get_redeemption_factor(None, None),
        Err(LoyaltyProgramError::CustomerNotEnrolled)
    );
}

#[test]
fn validate_loyalty_points_matches_erpnext_amount_checks() {
    let ctx = LoyaltyPointValidationContext {
        ref_doctype: "Sales Invoice".to_string(),
        customer: "CUST-001".to_string(),
        company: "Acme".to_string(),
        posting_date: Some("2026-06-01".to_string()),
        ref_doc_loyalty_program: Some("Rewards".to_string()),
        customer_loyalty_program: None,
        loyalty_program_company: Some("Acme".to_string()),
        available_loyalty_points: 100,
        conversion_factor: 0.5,
        expense_account: Some("Loyalty Expense - AC".to_string()),
        cost_center: Some("Main - AC".to_string()),
        grand_total: 60.0,
        rounded_total: 50.0,
        rounded_total_disabled: false,
        existing_loyalty_amount: None,
        existing_loyalty_points: None,
        existing_redemption_account: None,
        existing_redemption_cost_center: None,
    };

    assert_eq!(
        validate_loyalty_points(&ctx, 80),
        Ok(Some(LoyaltyPointValidationUpdate {
            loyalty_program: Some("Rewards".to_string()),
            loyalty_amount: Some(40.0),
            loyalty_points: Some(80),
            loyalty_redemption_account: Some("Loyalty Expense - AC".to_string()),
            loyalty_redemption_cost_center: Some("Main - AC".to_string()),
            sales_order_loyalty_amount: None,
        }))
    );

    assert_eq!(
        validate_loyalty_points(&ctx, 120),
        Err(LoyaltyProgramError::NotEnoughLoyaltyPoints)
    );
    let amount_exceeds = LoyaltyPointValidationContext {
        available_loyalty_points: 200,
        ..ctx.clone()
    };
    assert_eq!(
        validate_loyalty_points(&amount_exceeds, 101),
        Err(LoyaltyProgramError::LoyaltyAmountExceedsTotal)
    );

    let wrong_company = LoyaltyPointValidationContext {
        loyalty_program_company: Some("Other".to_string()),
        ..ctx.clone()
    };
    assert_eq!(
        validate_loyalty_points(&wrong_company, 10),
        Err(LoyaltyProgramError::ProgramInvalidForCompany)
    );

    let sales_order = LoyaltyPointValidationContext {
        ref_doctype: "Sales Order".to_string(),
        ..ctx
    };
    assert_eq!(
        validate_loyalty_points(&sales_order, 20)
            .unwrap()
            .unwrap()
            .sales_order_loyalty_amount,
        Some(10.0)
    );
}
