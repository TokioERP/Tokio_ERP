use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CouponCode {
    pub name: Option<String>,
    pub coupon_name: String,
    pub coupon_type: Option<String>,
    pub customer: Option<String>,
    pub coupon_code: Option<String>,
    pub from_external_ecomm_platform: bool,
    pub pricing_rule: Option<String>,
    pub valid_from: Option<String>,
    pub valid_upto: Option<String>,
    pub maximum_use: i32,
    pub used: i32,
    pub description: Option<String>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CouponCodeError {
    CustomerRequired,
}

impl Default for CouponCode {
    fn default() -> Self {
        Self {
            name: None,
            coupon_name: String::new(),
            coupon_type: None,
            customer: None,
            coupon_code: None,
            from_external_ecomm_platform: false,
            pricing_rule: None,
            valid_from: None,
            valid_upto: None,
            maximum_use: 0,
            used: 0,
            description: None,
            amended_from: None,
        }
    }
}

impl CouponCode {
    pub const DOCTYPE: &'static str = "Coupon Code";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 15] = [
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
    ];
    pub const ALLOW_IMPORT: bool = true;
    pub const AUTONAME: &'static str = "field:coupon_name";
    pub const DOCUMENT_TYPE: &'static str = "Other";
    pub const EDITABLE_GRID: bool = true;
    pub const NAMING_RULE: &'static str = "By fieldname";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TITLE_FIELD: &'static str = "coupon_name";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("coupon_name", "Coupon Name")
                .description("e.g. \"Summer Holiday 2019 Offer 20\"")
                .required()
                .unique(),
            FieldSpec::select("coupon_type", "Coupon Type")
                .options("Promotional\nGift Card")
                .in_list_view()
                .required(),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .depends_on("eval: doc.coupon_type == \"Gift Card\""),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::data("coupon_code", "Coupon Code")
                .description("unique e.g. SAVE20  To be used to get discount")
                .no_copy()
                .set_only_once()
                .unique(),
            FieldSpec::check(
                "from_external_ecomm_platform",
                "From External Ecomm Platform",
            )
            .default("0"),
            FieldSpec::link("pricing_rule", "Pricing Rule")
                .options("Pricing Rule")
                .depends_on("eval: !doc.from_external_ecomm_platform")
                .mandatory_depends_on("eval: !doc.from_external_ecomm_platform"),
            FieldSpec::section_break("uses").label("Validity and Usage"),
            FieldSpec::date("valid_from", "Valid From").in_list_view(),
            FieldSpec::date("valid_upto", "Valid Up To"),
            FieldSpec::int("maximum_use", "Maximum Use")
                .depends_on("eval: doc.coupon_type == \"Promotional\""),
            FieldSpec::int("used", "Used")
                .default("0")
                .no_copy()
                .read_only(),
            FieldSpec::column_break("column_break_11"),
            FieldSpec::text_editor("description", "Coupon Description"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Coupon Code")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    }

    pub fn autoname_with_hash(&mut self, generated_hash: &str) {
        self.coupon_name = self.coupon_name.trim().to_string();
        self.name = Some(self.coupon_name.clone());

        if self.coupon_code.is_none() {
            match self.coupon_type.as_deref() {
                Some("Promotional") => {
                    let code: String = self
                        .coupon_name
                        .chars()
                        .filter(|ch| !ch.is_ascii_digit())
                        .take(8)
                        .flat_map(char::to_uppercase)
                        .collect();
                    self.coupon_code = Some(code);
                }
                Some("Gift Card") => {
                    self.coupon_code = Some(
                        generated_hash
                            .chars()
                            .take(10)
                            .flat_map(char::to_uppercase)
                            .collect(),
                    );
                }
                _ => {}
            }
        }
    }

    pub fn validate(&mut self) -> Result<(), CouponCodeError> {
        if self.coupon_type.as_deref() == Some("Gift Card") {
            self.maximum_use = 1;
            if self.customer.is_none() {
                return Err(CouponCodeError::CustomerRequired);
            }
        }
        Ok(())
    }
}

impl DocumentController for CouponCode {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["autoname", "validate"]
    }
}
