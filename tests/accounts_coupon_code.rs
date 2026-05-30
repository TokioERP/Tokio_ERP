use tokio_erp::erpnext::accounts::doctype::coupon_code::coupon_code::{
    CouponCode, CouponCodeError,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn coupon_code_matches_erpnext_metadata() {
    assert_eq!(CouponCode::DOCTYPE, "Coupon Code");
    assert_eq!(CouponCode::MODULE, "Accounts");
    assert_eq!(
        CouponCode::FIELD_ORDER,
        [
            "coupon_name",
            "coupon_type",
            "customer",
            "column_break_4",
            "coupon_code",
            "from_external_ecomm_platform",
            "pricing_rule",
            "uses",
            "valid_from",
            "valid_upto",
            "maximum_use",
            "used",
            "column_break_11",
            "description",
            "amended_from",
        ]
    );
    assert!(CouponCode::ALLOW_IMPORT);
    assert_eq!(CouponCode::AUTONAME, "field:coupon_name");
    assert_eq!(CouponCode::DOCUMENT_TYPE, "Other");
    assert!(CouponCode::EDITABLE_GRID);
    assert_eq!(CouponCode::NAMING_RULE, "By fieldname");
    assert_eq!(CouponCode::SORT_FIELD, "creation");
    assert_eq!(CouponCode::SORT_ORDER, "DESC");
    assert_eq!(CouponCode::TITLE_FIELD, "coupon_name");
    assert!(CouponCode::TRACK_CHANGES);

    let fields = CouponCode::fields();
    assert_eq!(fields.len(), 15);
    assert_eq!(
        fields[0],
        FieldSpec::data("coupon_name", "Coupon Name")
            .description("e.g. \"Summer Holiday 2019 Offer 20\"")
            .required()
            .unique()
    );
    assert_eq!(
        fields[1],
        FieldSpec::select("coupon_type", "Coupon Type")
            .options("Promotional\nGift Card")
            .in_list_view()
            .required()
    );
    assert_eq!(
        fields[2],
        FieldSpec::link("customer", "Customer")
            .options("Customer")
            .depends_on("eval: doc.coupon_type == \"Gift Card\"")
    );
    assert_eq!(fields[3], FieldSpec::column_break("column_break_4"));
    assert_eq!(
        fields[4],
        FieldSpec::data("coupon_code", "Coupon Code")
            .description("unique e.g. SAVE20  To be used to get discount")
            .no_copy()
            .set_only_once()
            .unique()
    );
    assert_eq!(
        fields[5],
        FieldSpec::check(
            "from_external_ecomm_platform",
            "From External Ecomm Platform"
        )
        .default("0")
    );
    assert_eq!(
        fields[6],
        FieldSpec::link("pricing_rule", "Pricing Rule")
            .options("Pricing Rule")
            .depends_on("eval: !doc.from_external_ecomm_platform")
            .mandatory_depends_on("eval: !doc.from_external_ecomm_platform")
    );
}

#[test]
fn coupon_code_autoname_matches_erpnext() {
    let mut promotional = CouponCode {
        coupon_name: " Summer Holiday 2019 Offer 20 ".to_string(),
        coupon_type: Some("Promotional".to_string()),
        ..Default::default()
    };

    assert_eq!(promotional.custom_hooks(), ["autoname", "validate"]);
    promotional.autoname_with_hash("unusedhash");
    assert_eq!(
        promotional.name.as_deref(),
        Some("Summer Holiday 2019 Offer 20")
    );
    assert_eq!(
        promotional.coupon_name.as_str(),
        "Summer Holiday 2019 Offer 20"
    );
    assert_eq!(promotional.coupon_code.as_deref(), Some("SUMMER H"));

    let mut existing = CouponCode {
        coupon_name: " Save Existing ".to_string(),
        coupon_type: Some("Promotional".to_string()),
        coupon_code: Some("KEEP".to_string()),
        ..Default::default()
    };
    existing.autoname_with_hash("unusedhash");
    assert_eq!(existing.coupon_code.as_deref(), Some("KEEP"));

    let mut gift_card = CouponCode {
        coupon_name: " Gift Card ".to_string(),
        coupon_type: Some("Gift Card".to_string()),
        ..Default::default()
    };
    gift_card.autoname_with_hash("abc123xyz987");
    assert_eq!(gift_card.coupon_code.as_deref(), Some("ABC123XYZ9"));
}

#[test]
fn coupon_code_validate_matches_gift_card_rule() {
    let mut gift_card = CouponCode {
        coupon_name: "Birthday Gift".to_string(),
        coupon_type: Some("Gift Card".to_string()),
        customer: Some("CUST-001".to_string()),
        maximum_use: 9,
        ..Default::default()
    };

    assert_eq!(gift_card.validate(), Ok(()));
    assert_eq!(gift_card.maximum_use, 1);
    assert_eq!(gift_card.doctype(), "Coupon Code");
    assert_eq!(gift_card.module(), "Accounts");

    let mut missing_customer = CouponCode {
        coupon_name: "Birthday Gift".to_string(),
        coupon_type: Some("Gift Card".to_string()),
        ..Default::default()
    };
    assert_eq!(
        missing_customer.validate(),
        Err(CouponCodeError::CustomerRequired)
    );

    let mut promotional = CouponCode {
        coupon_name: "Promo".to_string(),
        coupon_type: Some("Promotional".to_string()),
        maximum_use: 5,
        ..Default::default()
    };
    assert_eq!(promotional.validate(), Ok(()));
    assert_eq!(promotional.maximum_use, 5);
}
