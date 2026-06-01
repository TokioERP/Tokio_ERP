use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSProfile {
    pub name: Option<String>,
    pub company: String,
    pub customer: Option<String>,
    pub disabled: bool,
    pub warehouse: String,
    pub company_address: Option<String>,
    pub applicable_for_users: Vec<POSProfileUserRow>,
    pub payments: Vec<POSPaymentMethodRow>,
    pub item_groups: Vec<POSItemGroupRow>,
    pub customer_groups: Vec<POSCustomerGroupRow>,
    pub currency: String,
    pub write_off_account: String,
    pub write_off_cost_center: String,
    pub income_account: Option<String>,
    pub expense_account: Option<String>,
    pub cost_center: Option<String>,
    pub project: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSProfileUserRow {
    pub name: Option<String>,
    pub idx: usize,
    pub user: String,
    pub default: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSPaymentMethodRow {
    pub mode_of_payment: String,
    pub default: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSItemGroupRow {
    pub item_group: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSCustomerGroupRow {
    pub customer_group: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimensionCheck {
    pub label: String,
    pub fieldname: String,
    pub company: String,
    pub mandatory_for_pl: bool,
    pub mandatory_for_bs: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinkRecord {
    pub doctype: String,
    pub company: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSDefaultsPlan {
    pub clear_is_pos: bool,
    pub user_defaults: Vec<(String, bool)>,
    pub global_default: Option<bool>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChildNode {
    pub name: String,
    pub lft: i32,
    pub rgt: i32,
    pub is_group: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct POSProfileQueryRow {
    pub name: String,
    pub company: String,
    pub disabled: bool,
    pub user: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SetDefaultProfilePlan {
    pub clear_default_for_user: String,
    pub set_default_profile: Option<String>,
    pub company: String,
    pub modified: String,
    pub modified_by: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum POSProfileError {
    CannotDisableWithOpenSessions { profile: String },
    DefaultAlreadySet { profile: String, user: String },
    LinkDoesNotBelongToCompany { link_name: String, company: String },
    DuplicateItemGroup,
    DuplicateCustomerGroup,
    MissingPaymentMethods,
    MissingDefaultPaymentMethod,
    MultipleDefaultPaymentMethods,
    MissingModeOfPaymentAccount { modes: Vec<String> },
    MandatoryAccountingDimension { label: String },
}

impl POSProfile {
    pub const DOCTYPE: &'static str = "POS Profile";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "Prompt";
    pub const FIELD_ORDER: [&'static str; 59] = [
        "company",
        "customer",
        "country",
        "disabled",
        "column_break_9",
        "warehouse",
        "company_address",
        "section_break_15",
        "applicable_for_users",
        "section_break_11",
        "payments",
        "section_break_14",
        "hide_images",
        "hide_unavailable_items",
        "auto_add_item_to_cart",
        "validate_stock_on_save",
        "print_receipt_on_order_complete",
        "action_on_new_invoice",
        "column_break_16",
        "update_stock",
        "ignore_pricing_rule",
        "allow_rate_change",
        "allow_discount_change",
        "set_grand_total_to_default_mop",
        "allow_partial_payment",
        "section_break_23",
        "item_groups",
        "column_break_25",
        "customer_groups",
        "section_break_16",
        "print_format",
        "letter_head",
        "column_break0",
        "tc_name",
        "select_print_heading",
        "section_break_19",
        "selling_price_list",
        "currency",
        "write_off_account",
        "write_off_cost_center",
        "write_off_limit",
        "account_for_change_amount",
        "disable_rounded_total",
        "column_break_23",
        "income_account",
        "expense_account",
        "taxes_and_charges",
        "tax_category",
        "apply_discount_on",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "utm_analytics_section",
        "utm_source",
        "column_break_tvls",
        "utm_campaign",
        "column_break_xygw",
        "utm_medium",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .oldfield("customer_account", "Link"),
            FieldSpec::read_only_field("country", "Country").fetch_from("company.country"),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::link("warehouse", "Warehouse")
                .options("Warehouse")
                .required(),
            FieldSpec::link("company_address", "Company Address").options("Address"),
            FieldSpec::section_break("section_break_15").label("Applicable for Users"),
            FieldSpec::table("applicable_for_users", "Applicable for Users")
                .options("POS Profile User"),
            FieldSpec::section_break("section_break_11").label("Payment Methods"),
            FieldSpec::table("payments", "Payment Methods")
                .options("POS Payment Method")
                .required(),
            FieldSpec::section_break("section_break_14").label("Configuration"),
            FieldSpec::check("hide_images", "Hide Images").default("0"),
            FieldSpec::check("hide_unavailable_items", "Hide Unavailable Items").default("0"),
            FieldSpec::check("auto_add_item_to_cart", "Automatically Add Filtered Item To Cart")
                .default("0"),
            FieldSpec::check("validate_stock_on_save", "Validate Stock on Save").default("0"),
            FieldSpec::check("print_receipt_on_order_complete", "Print Receipt on Order Complete")
                .default("0"),
            FieldSpec::select("action_on_new_invoice", "Action on New Invoice")
                .options("Always Ask\nSave Changes and Load New Invoice\nDiscard Changes and Load New Invoice")
                .default("Always Ask"),
            FieldSpec::column_break("column_break_16"),
            FieldSpec::check("update_stock", "Update Stock")
                .default("1")
                .hidden()
                .read_only(),
            FieldSpec::check("ignore_pricing_rule", "Ignore Pricing Rule").default("0"),
            FieldSpec::check("allow_rate_change", "Allow User to Edit Rate").default("0"),
            FieldSpec::check("allow_discount_change", "Allow User to Edit Discount").default("0"),
            FieldSpec::check("set_grand_total_to_default_mop", "Set Grand Total to Default Payment Method").default("1"),
            FieldSpec::check("allow_partial_payment", "Allow Partial Payment").default("0"),
            FieldSpec::section_break("section_break_23").label("Filters"),
            FieldSpec::table("item_groups", "Item Groups")
                .options("POS Item Group")
                .description("Only show Items from these Item Groups"),
            FieldSpec::column_break("column_break_25"),
            FieldSpec::table("customer_groups", "Customer Groups")
                .options("POS Customer Group")
                .description("Only show Customer of these Customer Groups"),
            FieldSpec::section_break("section_break_16").label("Print Settings"),
            FieldSpec::link("print_format", "Print Format").options("Print Format"),
            FieldSpec::link("letter_head", "Letter Head")
                .options("Letter Head")
                .oldfield("letter_head", "Select")
                .allow_on_submit()
                .print_hide(),
            FieldSpec::column_break("column_break0").oldfield("", "Column Break"),
            FieldSpec::link("tc_name", "Terms and Conditions")
                .options("Terms and Conditions")
                .oldfield("tc_name", "Link"),
            FieldSpec::link("select_print_heading", "Print Heading")
                .options("Print Heading")
                .oldfield("select_print_heading", "Select")
                .allow_on_submit(),
            FieldSpec::section_break("section_break_19").label("Accounting"),
            FieldSpec::link("selling_price_list", "Price List")
                .options("Price List")
                .oldfield("price_list_name", "Select"),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .oldfield("currency", "Select")
                .required(),
            FieldSpec::link("write_off_account", "Write Off Account")
                .options("Account")
                .required(),
            FieldSpec::link("write_off_cost_center", "Write Off Cost Center")
                .options("Cost Center")
                .required(),
            FieldSpec::currency("write_off_limit", "Write Off Limit").required(),
            FieldSpec::link("account_for_change_amount", "Account for Change Amount")
                .options("Account"),
            FieldSpec::check("disable_rounded_total", "Disable Rounded Total")
                .default("0")
                .description("If enabled, the consolidated invoices will have rounded total disabled"),
            FieldSpec::column_break("column_break_23"),
            FieldSpec::link("income_account", "Income Account")
                .options("Account")
                .oldfield("income_account", "Link"),
            FieldSpec::link("expense_account", "Expense Account")
                .options("Account")
                .print_hide(),
            FieldSpec::link("taxes_and_charges", "Taxes and Charges")
                .options("Sales Taxes and Charges Template")
                .oldfield("charge", "Link"),
            FieldSpec::link("tax_category", "Tax Category").options("Tax Category"),
            FieldSpec::select("apply_discount_on", "Apply Discount On")
                .options("Grand Total\nNet Total")
                .default("Grand Total"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions"),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .oldfield("cost_center", "Link"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::link("project", "Project")
                .options("Project")
                .oldfield("cost_center", "Link"),
            FieldSpec::section_break("utm_analytics_section")
                .label("Campaign")
                .collapsible(),
            FieldSpec::link("utm_source", "Source")
                .options("UTM Source")
                .print_hide(),
            FieldSpec::column_break("column_break_tvls"),
            FieldSpec::link("utm_campaign", "Campaign")
                .options("UTM Campaign")
                .print_hide(),
            FieldSpec::column_break("column_break_xygw"),
            FieldSpec::link("utm_medium", "Medium")
                .options("UTM Campaign")
                .print_hide(),
        ]
    }

    pub fn validate(
        &self,
        old_disabled: Option<bool>,
        has_open_session: bool,
        other_default_profile_by_user_company: &BTreeMap<(String, String), String>,
        doctype_company_links: &[LinkRecord],
        mode_default_accounts: &BTreeMap<String, Option<String>>,
        existing_company_links: &BTreeSet<String>,
        accounting_dimensions: &[AccountingDimensionCheck],
    ) -> Result<Vec<String>, POSProfileError> {
        self.validate_disabled(old_disabled, has_open_session)?;
        self.validate_default_profile(other_default_profile_by_user_company)?;
        self.validate_all_link_fields(doctype_company_links, existing_company_links)?;
        self.validate_duplicate_groups()?;
        self.validate_payment_methods(mode_default_accounts)?;
        self.validate_accounting_dimensions(accounting_dimensions)?;
        Ok(Vec::new())
    }

    pub fn validate_accounting_dimensions(
        &self,
        accounting_dimensions: &[AccountingDimensionCheck],
    ) -> Result<(), POSProfileError> {
        for dim in accounting_dimensions {
            if self.company == dim.company
                && self.dimension_value(&dim.fieldname).is_none()
                && (dim.mandatory_for_pl || dim.mandatory_for_bs)
            {
                return Err(POSProfileError::MandatoryAccountingDimension {
                    label: dim.label.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_disabled(
        &self,
        old_disabled: Option<bool>,
        has_open_session: bool,
    ) -> Result<(), POSProfileError> {
        if old_disabled.is_some()
            && self.disabled
            && old_disabled != Some(self.disabled)
            && has_open_session
        {
            return Err(POSProfileError::CannotDisableWithOpenSessions {
                profile: self.name.clone().unwrap_or_default(),
            });
        }
        Ok(())
    }

    pub fn validate_default_profile(
        &self,
        other_default_profile_by_user_company: &BTreeMap<(String, String), String>,
    ) -> Result<Vec<String>, POSProfileError> {
        let mut warnings = Vec::new();
        for row in &self.applicable_for_users {
            let existing = other_default_profile_by_user_company
                .get(&(row.user.clone(), self.company.clone()));
            if row.default {
                if let Some(profile) = existing {
                    return Err(POSProfileError::DefaultAlreadySet {
                        profile: profile.clone(),
                        user: row.user.clone(),
                    });
                }
            } else if existing.is_none() {
                warnings.push(format!(
                    "User {} doesn't have any default POS Profile. Check Default at Row {} for this User.",
                    row.user, row.idx
                ));
            }
        }
        Ok(warnings)
    }

    pub fn validate_all_link_fields(
        &self,
        _doctype_company_links: &[LinkRecord],
        existing_company_links: &BTreeSet<String>,
    ) -> Result<(), POSProfileError> {
        let mut required = vec![("Warehouse", self.warehouse.as_str())];
        if let Some(income_account) = self.income_account.as_deref() {
            required.push(("Account", income_account));
        }
        if let Some(expense_account) = self.expense_account.as_deref() {
            required.push(("Account", expense_account));
        }
        if let Some(cost_center) = self.cost_center.as_deref() {
            required.push(("Cost Center", cost_center));
        }

        for (doctype, name) in required {
            if name.is_empty() {
                continue;
            }
            let key = format!("{doctype}:{}:{name}", self.company);
            if !existing_company_links.contains(&key) {
                return Err(POSProfileError::LinkDoesNotBelongToCompany {
                    link_name: name.to_string(),
                    company: self.company.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_duplicate_groups(&self) -> Result<(), POSProfileError> {
        if has_duplicates(self.item_groups.iter().map(|row| row.item_group.as_str())) {
            return Err(POSProfileError::DuplicateItemGroup);
        }
        if has_duplicates(
            self.customer_groups
                .iter()
                .map(|row| row.customer_group.as_str()),
        ) {
            return Err(POSProfileError::DuplicateCustomerGroup);
        }
        Ok(())
    }

    pub fn validate_payment_methods(
        &self,
        mode_default_accounts: &BTreeMap<String, Option<String>>,
    ) -> Result<(), POSProfileError> {
        if self.payments.is_empty() {
            return Err(POSProfileError::MissingPaymentMethods);
        }
        let default_count = self.payments.iter().filter(|row| row.default).count();
        if default_count == 0 {
            return Err(POSProfileError::MissingDefaultPaymentMethod);
        }
        if default_count > 1 {
            return Err(POSProfileError::MultipleDefaultPaymentMethods);
        }

        let invalid_modes = self
            .payments
            .iter()
            .filter(|row| {
                mode_default_accounts
                    .get(&row.mode_of_payment)
                    .and_then(Clone::clone)
                    .is_none()
            })
            .map(|row| row.mode_of_payment.clone())
            .collect::<Vec<_>>();
        if !invalid_modes.is_empty() {
            return Err(POSProfileError::MissingModeOfPaymentAccount {
                modes: invalid_modes,
            });
        }

        Ok(())
    }

    pub fn set_defaults_plan(
        &self,
        include_current_pos: bool,
        profile_users: &[POSProfileUserRow],
    ) -> POSDefaultsPlan {
        let mut plan = POSDefaultsPlan {
            clear_is_pos: true,
            ..Default::default()
        };
        for row in profile_users.iter().filter(|row| row.default) {
            if !include_current_pos && self.name.as_ref() == row.name.as_ref() {
                continue;
            }
            if row.user.is_empty() {
                plan.global_default = Some(true);
            } else {
                plan.user_defaults.push((row.user.clone(), true));
            }
        }
        plan
    }

    fn dimension_value(&self, fieldname: &str) -> Option<&str> {
        match fieldname {
            "cost_center" => self.cost_center.as_deref(),
            "project" => self.project.as_deref(),
            _ => None,
        }
        .filter(|value| !value.is_empty())
    }
}

impl DocumentController for POSProfile {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "on_update", "on_trash"]
    }
}

pub fn get_item_groups(
    pos_profile: &POSProfile,
    item_group_nodes: &[ChildNode],
    permitted_item_groups: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut item_groups = BTreeSet::new();
    if !pos_profile.item_groups.is_empty() {
        for row in &pos_profile.item_groups {
            item_groups.extend(
                get_child_nodes(item_group_nodes, &row.item_group)
                    .into_iter()
                    .filter(|name| {
                        permitted_item_groups.is_empty() || permitted_item_groups.contains(name)
                    }),
            );
        }
    }
    if item_groups.is_empty() && !permitted_item_groups.is_empty() {
        item_groups.extend(permitted_item_groups.iter().cloned());
    }
    item_groups
}

pub fn get_permitted_nodes(nodes: &[ChildNode], permitted_nodes: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for node_name in permitted_nodes {
        if nodes
            .iter()
            .find(|node| &node.name == node_name)
            .is_some_and(|node| node.is_group)
        {
            out.extend(get_child_nodes(nodes, node_name));
        } else {
            out.push(node_name.clone());
        }
    }
    out
}

pub fn get_child_nodes(nodes: &[ChildNode], root: &str) -> Vec<String> {
    let Some(root_node) = nodes.iter().find(|node| node.name == root) else {
        return Vec::new();
    };
    let mut children = nodes
        .iter()
        .filter(|node| node.lft >= root_node.lft && node.rgt <= root_node.rgt)
        .collect::<Vec<_>>();
    children.sort_by_key(|node| node.lft);
    children.into_iter().map(|node| node.name.clone()).collect()
}

pub fn pos_profile_query(
    rows: &[POSProfileQueryRow],
    user: &str,
    txt: &str,
    start: usize,
    page_len: usize,
    company: Option<&str>,
) -> Vec<String> {
    let company = company.unwrap_or_default();
    let txt = txt.to_ascii_lowercase();
    let mut user_profiles = rows
        .iter()
        .filter(|row| row.user.as_deref() == Some(user))
        .filter(|row| row.company == company)
        .filter(|row| !row.disabled)
        .filter(|row| row.name.to_ascii_lowercase().contains(&txt))
        .map(|row| row.name.clone())
        .collect::<Vec<_>>();
    if !user_profiles.is_empty() {
        return user_profiles
            .into_iter()
            .skip(start)
            .take(page_len)
            .collect();
    }

    user_profiles = rows
        .iter()
        .filter(|row| row.user.as_deref().unwrap_or_default().is_empty())
        .filter(|row| row.company == company)
        .filter(|row| !row.disabled)
        .filter(|row| row.name.to_ascii_lowercase().contains(&txt))
        .map(|row| row.name.clone())
        .collect();
    user_profiles
}

pub fn set_default_profile_plan(
    pos_profile: &str,
    company: &str,
    user: &str,
    modified: &str,
) -> SetDefaultProfilePlan {
    if pos_profile.is_empty() || company.is_empty() {
        return SetDefaultProfilePlan {
            modified: modified.to_string(),
            modified_by: user.to_string(),
            ..Default::default()
        };
    }

    SetDefaultProfilePlan {
        clear_default_for_user: user.to_string(),
        set_default_profile: Some(pos_profile.to_string()),
        company: company.to_string(),
        modified: modified.to_string(),
        modified_by: user.to_string(),
    }
}

fn has_duplicates<'a>(values: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return true;
        }
    }
    false
}
