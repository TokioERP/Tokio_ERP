use crate::erpnext::accounts::doctype::loyalty_program_collection::loyalty_program_collection::LoyaltyProgramCollection;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoyaltyProgram {
    pub loyalty_program_name: String,
    pub loyalty_program_type: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub customer_group: Option<String>,
    pub customer_territory: Option<String>,
    pub auto_opt_in: bool,
    pub collection_rules: Vec<LoyaltyProgramCollection>,
    pub conversion_factor: f64,
    pub expiry_duration: i32,
    pub expense_account: Option<String>,
    pub company: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoyaltyDetailsQueryPlan {
    pub doctype: &'static str,
    pub select: [&'static str; 2],
    pub filters: Vec<(&'static str, &'static str, String)>,
    pub group_by: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoyaltyPointValidationContext {
    pub ref_doctype: String,
    pub customer: String,
    pub company: String,
    pub posting_date: Option<String>,
    pub ref_doc_loyalty_program: Option<String>,
    pub customer_loyalty_program: Option<String>,
    pub loyalty_program_company: Option<String>,
    pub available_loyalty_points: i32,
    pub conversion_factor: f64,
    pub expense_account: Option<String>,
    pub cost_center: Option<String>,
    pub grand_total: f64,
    pub rounded_total: f64,
    pub rounded_total_disabled: bool,
    pub existing_loyalty_amount: Option<f64>,
    pub existing_loyalty_points: Option<i32>,
    pub existing_redemption_account: Option<String>,
    pub existing_redemption_cost_center: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoyaltyPointValidationUpdate {
    pub loyalty_program: Option<String>,
    pub loyalty_amount: Option<f64>,
    pub loyalty_points: Option<i32>,
    pub loyalty_redemption_account: Option<String>,
    pub loyalty_redemption_cost_center: Option<String>,
    pub sales_order_loyalty_amount: Option<f64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoyaltyProgramError {
    LowestTierMustStartAtZero,
    CustomerNotEnrolled,
    ProgramInvalidForCompany,
    NotEnoughLoyaltyPoints,
    LoyaltyAmountExceedsTotal,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LoyaltyProgramDetails {
    pub loyalty_program: Option<String>,
    pub loyalty_points: i32,
    pub total_spent: f64,
    pub conversion_factor: f64,
    pub expense_account: Option<String>,
    pub cost_center: Option<String>,
    pub tier_name: Option<String>,
    pub collection_factor: Option<f64>,
}

impl LoyaltyProgram {
    pub const DOCTYPE: &'static str = "Loyalty Program";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 22] = [
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
    ];
    pub const AUTONAME: &'static str = "field:loyalty_program_name";
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("loyalty_program_name", "Loyalty Program Name")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::select("loyalty_program_type", "Loyalty Program Type")
                .options("Single Tier Program\nMultiple Tier Program"),
            FieldSpec::date("from_date", "From Date").required(),
            FieldSpec::date("to_date", "To Date"),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("customer_group", "Customer Group").options("Customer Group"),
            FieldSpec::link("customer_territory", "Customer Territory").options("Territory"),
            FieldSpec::check("auto_opt_in", "Auto Opt In (For all customers)").default("0"),
            FieldSpec::section_break("rules").label("Collection Tier"),
            FieldSpec::table("collection_rules", "Collection Rules")
                .options("Loyalty Program Collection")
                .required(),
            FieldSpec::section_break("redemption").label("Redemption"),
            FieldSpec::float("conversion_factor", "Conversion Factor")
                .description("1 Loyalty Points = How much base currency?"),
            FieldSpec::int("expiry_duration", "Expiry Duration (in days)"),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::link("expense_account", "Expense Account").options("Account"),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::link("project", "Project").options("Project"),
            FieldSpec::section_break("help_section").label("Help Section"),
            FieldSpec::html("loyalty_program_help", "Loyalty Program Help"),
        ]
    }

    pub fn validate(&self) -> Result<(), LoyaltyProgramError> {
        self.validate_lowest_tier()
    }

    pub fn validate_lowest_tier(&self) -> Result<(), LoyaltyProgramError> {
        let lowest = self
            .collection_rules
            .iter()
            .min_by(|left, right| left.min_spent.total_cmp(&right.min_spent));
        if lowest.is_some_and(|tier| tier.min_spent != 0.0) {
            Err(LoyaltyProgramError::LowestTierMustStartAtZero)
        } else {
            Ok(())
        }
    }
}

impl DocumentController for LoyaltyProgram {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate"]
    }
}

pub fn get_loyalty_details_plan(
    customer: &str,
    loyalty_program: &str,
    expiry_date: Option<&str>,
    company: Option<&str>,
    include_expired_entry: bool,
    today: &str,
) -> LoyaltyDetailsQueryPlan {
    let expiry_date = expiry_date.unwrap_or(today);
    let mut filters = vec![
        ("customer", "=", customer.to_string()),
        ("loyalty_program", "=", loyalty_program.to_string()),
        ("posting_date", "<=", expiry_date.to_string()),
    ];
    if let Some(company) = company {
        filters.push(("company", "=", company.to_string()));
    }
    if !include_expired_entry {
        filters.push(("expiry_date", ">=", expiry_date.to_string()));
    }

    LoyaltyDetailsQueryPlan {
        doctype: "Loyalty Point Entry",
        select: [
            "sum(loyalty_points) as loyalty_points",
            "sum(purchase_amount) as total_spent",
        ],
        filters,
        group_by: Some("customer"),
    }
}

pub fn select_loyalty_tier(
    collection_rules: &[LoyaltyProgramCollection],
    total_spent: f64,
    current_transaction_amount: f64,
) -> Option<(String, f64)> {
    let mut rules: Vec<&LoyaltyProgramCollection> = collection_rules.iter().collect();
    rules.sort_by(|left, right| left.min_spent.total_cmp(&right.min_spent));

    let mut selected = None;
    for (index, rule) in rules.into_iter().enumerate() {
        if index == 0 || (total_spent + current_transaction_amount) >= rule.min_spent {
            selected = Some((
                rule.tier_name.clone().unwrap_or_default(),
                rule.collection_factor,
            ));
        } else {
            break;
        }
    }
    selected
}

pub fn calculate_points_earned(
    posting_date: &str,
    program_from_date: &str,
    program_to_date: Option<&str>,
    grand_total: f64,
    loyalty_amount: f64,
    returned_amount: f64,
    collection_factor: f64,
) -> i32 {
    if posting_date < program_from_date
        || program_to_date.is_some_and(|to_date| posting_date > to_date)
    {
        return 0;
    }

    let eligible_amount = grand_total - cint(loyalty_amount) as f64 - returned_amount;
    cint(eligible_amount / collection_factor)
}

pub fn get_redeemption_factor(
    loyalty_program_conversion_factor: Option<f64>,
    customer_loyalty_program_conversion_factor: Option<f64>,
) -> Result<f64, LoyaltyProgramError> {
    loyalty_program_conversion_factor
        .or(customer_loyalty_program_conversion_factor)
        .ok_or(LoyaltyProgramError::CustomerNotEnrolled)
}

pub fn validate_loyalty_points(
    ctx: &LoyaltyPointValidationContext,
    points_to_redeem: i32,
) -> Result<Option<LoyaltyPointValidationUpdate>, LoyaltyProgramError> {
    let loyalty_program = ctx
        .ref_doc_loyalty_program
        .as_ref()
        .or(ctx.customer_loyalty_program.as_ref());

    if loyalty_program.is_some()
        && ctx
            .loyalty_program_company
            .as_ref()
            .is_some_and(|company| company != &ctx.company)
    {
        return Err(LoyaltyProgramError::ProgramInvalidForCompany);
    }

    if loyalty_program.is_none() || points_to_redeem == 0 {
        return Ok(None);
    }

    if points_to_redeem > ctx.available_loyalty_points {
        return Err(LoyaltyProgramError::NotEnoughLoyaltyPoints);
    }

    let loyalty_amount = points_to_redeem as f64 * ctx.conversion_factor;
    let total_amount = if ctx.rounded_total_disabled {
        ctx.grand_total
    } else {
        ctx.rounded_total
    };
    if loyalty_amount > total_amount {
        return Err(LoyaltyProgramError::LoyaltyAmountExceedsTotal);
    }

    let loyalty_amount_update = match ctx.existing_loyalty_amount {
        Some(existing) if existing != 0.0 => None,
        _ if ctx.existing_loyalty_amount != Some(loyalty_amount) => Some(loyalty_amount),
        _ => None,
    };
    let loyalty_points_update = match ctx.existing_loyalty_points {
        Some(existing) if existing != 0 => None,
        _ if ctx.existing_loyalty_points != Some(points_to_redeem) => Some(points_to_redeem),
        _ => None,
    };

    if ctx.ref_doctype == "Sales Order" {
        return Ok(Some(LoyaltyPointValidationUpdate {
            loyalty_program: None,
            loyalty_amount: None,
            loyalty_points: None,
            loyalty_redemption_account: None,
            loyalty_redemption_cost_center: None,
            sales_order_loyalty_amount: Some(loyalty_amount),
        }));
    }

    let is_sales_invoice = ctx.ref_doctype == "Sales Invoice";
    Ok(Some(LoyaltyPointValidationUpdate {
        loyalty_program: is_sales_invoice.then(|| loyalty_program.cloned().unwrap_or_default()),
        loyalty_amount: loyalty_amount_update,
        loyalty_points: loyalty_points_update,
        loyalty_redemption_account: (is_sales_invoice && ctx.existing_redemption_account.is_none())
            .then(|| ctx.expense_account.clone())
            .flatten(),
        loyalty_redemption_cost_center: (is_sales_invoice
            && ctx.existing_redemption_cost_center.is_none())
        .then(|| ctx.cost_center.clone())
        .flatten(),
        sales_order_loyalty_amount: None,
    }))
}

fn cint(value: f64) -> i32 {
    if value.is_finite() {
        value.trunc() as i32
    } else {
        0
    }
}
