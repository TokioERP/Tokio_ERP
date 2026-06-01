use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CostCenter {
    pub name: Option<String>,
    pub cost_center_name: String,
    pub cost_center_number: Option<String>,
    pub parent_cost_center: Option<String>,
    pub company: String,
    pub is_group: bool,
    pub disabled: bool,
    pub lft: i32,
    pub rgt: i32,
    pub old_parent: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CostCenterError {
    ParentCostCenterRequired,
    RootCannotHaveParent,
    ParentMustBeGroup { parent_cost_center: String },
    CannotConvertGroupWithChildren,
    CannotConvertLedgerWithGlEntries,
    CannotConvertLedgerWithAllocationRecords,
    CannotConvertLedgerPartOfAllocation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostCenterRenameUpdate {
    pub cost_center_number: Option<String>,
    pub cost_center_name: Option<String>,
}

impl CostCenter {
    pub const DOCTYPE: &'static str = "Cost Center";
    pub const MODULE: &'static str = "Accounts";
    pub const DEFAULT_VIEW: &'static str = "Tree";
    pub const DESCRIPTION: &'static str =
        "Track separate Income and Expense for product verticals or divisions.";
    pub const DOCUMENT_TYPE: &'static str = "Setup";
    pub const NSM_PARENT_FIELD: &'static str = "parent_cost_center";
    pub const ALLOW_COPY: bool = true;
    pub const ALLOW_IMPORT: bool = true;
    pub const IS_TREE: bool = true;
    pub const SEARCH_FIELDS: &'static str = "parent_cost_center, is_group";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "ASC";
    pub const FIELD_ORDER: [&'static str; 11] = [
        "sb0",
        "cost_center_name",
        "cost_center_number",
        "parent_cost_center",
        "company",
        "cb0",
        "is_group",
        "disabled",
        "lft",
        "rgt",
        "old_parent",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("sb0"),
            FieldSpec::data("cost_center_name", "Cost Center Name")
                .oldfield("cost_center_name", "Data")
                .in_list_view()
                .no_copy()
                .required(),
            FieldSpec::data("cost_center_number", "Cost Center Number")
                .in_list_view()
                .in_standard_filter()
                .read_only_depends_on("eval:!doc.__islocal"),
            FieldSpec::link("parent_cost_center", "Parent Cost Center")
                .options("Cost Center")
                .oldfield("parent_cost_center", "Link")
                .ignore_user_permissions()
                .in_list_view()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .oldfield("company_name", "Link")
                .in_list_view()
                .in_standard_filter()
                .required(),
            FieldSpec::column_break("cb0").width("50%"),
            FieldSpec::check("is_group", "Is Group").default("0"),
            FieldSpec::check("disabled", "Disabled").default("0"),
            FieldSpec::int("lft", "lft")
                .oldfield("lft", "Int")
                .hidden()
                .no_copy()
                .print_hide()
                .report_hide()
                .search_index(),
            FieldSpec::int("rgt", "rgt")
                .oldfield("rgt", "Int")
                .hidden()
                .no_copy()
                .print_hide()
                .report_hide()
                .search_index(),
            FieldSpec::link("old_parent", "old_parent")
                .options("Cost Center")
                .oldfield("old_parent", "Data")
                .hidden()
                .ignore_user_permissions()
                .no_copy()
                .print_hide()
                .report_hide(),
        ]
    }

    pub fn autoname_with_company_abbr(&mut self, company_abbr: &str) -> String {
        let name = get_autoname_with_number(
            self.cost_center_number.as_deref(),
            &self.cost_center_name,
            company_abbr,
        );
        self.name = Some(name.clone());
        name
    }

    pub fn validate(&self, parent_is_group: Option<bool>) -> Result<(), CostCenterError> {
        self.validate_mandatory()?;
        self.validate_parent_cost_center(parent_is_group)
    }

    pub fn validate_mandatory(&self) -> Result<(), CostCenterError> {
        if self.cost_center_name != self.company && self.parent_cost_center.is_none() {
            return Err(CostCenterError::ParentCostCenterRequired);
        }
        if self.cost_center_name == self.company && self.parent_cost_center.is_some() {
            return Err(CostCenterError::RootCannotHaveParent);
        }
        Ok(())
    }

    pub fn validate_parent_cost_center(
        &self,
        parent_is_group: Option<bool>,
    ) -> Result<(), CostCenterError> {
        if let Some(parent_cost_center) = &self.parent_cost_center {
            if parent_is_group == Some(false) {
                return Err(CostCenterError::ParentMustBeGroup {
                    parent_cost_center: parent_cost_center.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn convert_group_to_ledger(
        &mut self,
        child_exists: bool,
        gl_exists: bool,
    ) -> Result<bool, CostCenterError> {
        if child_exists {
            return Err(CostCenterError::CannotConvertGroupWithChildren);
        }
        if gl_exists {
            return Err(CostCenterError::CannotConvertLedgerWithGlEntries);
        }
        self.is_group = false;
        Ok(true)
    }

    pub fn convert_ledger_to_group(
        &mut self,
        allocation_exists: bool,
        part_of_allocation: bool,
        gl_exists: bool,
    ) -> Result<bool, CostCenterError> {
        if allocation_exists {
            return Err(CostCenterError::CannotConvertLedgerWithAllocationRecords);
        }
        if part_of_allocation {
            return Err(CostCenterError::CannotConvertLedgerPartOfAllocation);
        }
        if gl_exists {
            return Err(CostCenterError::CannotConvertLedgerWithGlEntries);
        }
        self.is_group = true;
        Ok(true)
    }

    pub fn before_rename(&self, newdn: &str, company_abbr: &str, merge: bool) -> String {
        let new_cost_center = get_name_with_abbr(newdn, company_abbr);
        if merge {
            new_cost_center
        } else {
            get_name_with_number(&new_cost_center, self.cost_center_number.as_deref())
        }
    }

    pub fn after_rename_plan(
        &self,
        newdn: &str,
        current_cost_center_name: Option<&str>,
        current_cost_center_number: Option<&str>,
    ) -> Option<CostCenterRenameUpdate> {
        let mut new_parts: Vec<&str> = newdn.split(" - ").collect();
        if !new_parts.is_empty() {
            new_parts.pop();
        }
        if new_parts.is_empty() {
            return None;
        }

        let mut number_update = None;
        if new_parts[0]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
        {
            if new_parts.len() == 1 {
                new_parts = newdn.split(' ').collect();
            }
            if current_cost_center_number != Some(new_parts[0]) {
                number_update = Some(new_parts[0].to_string());
            }
            new_parts.remove(0);
        }

        let cost_center_name = new_parts.join(" - ");
        let name_update = if current_cost_center_name != Some(cost_center_name.as_str()) {
            Some(cost_center_name)
        } else {
            None
        };

        if number_update.is_none() && name_update.is_none() {
            None
        } else {
            Some(CostCenterRenameUpdate {
                cost_center_number: number_update,
                cost_center_name: name_update,
            })
        }
    }
}

impl DocumentController for CostCenter {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["autoname", "validate", "before_rename", "after_rename"]
    }
}

pub fn get_autoname_with_number(
    number_value: Option<&str>,
    doc_title: &str,
    company_abbr: &str,
) -> String {
    let mut parts = vec![doc_title.trim().to_string(), company_abbr.to_string()];
    if let Some(number_value) = number_value
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        parts.insert(0, number_value.to_string());
    }
    parts.join(" - ")
}

pub fn get_name_with_number(new_account: &str, account_number: Option<&str>) -> String {
    if let Some(account_number) = account_number.filter(|number| !number.is_empty()) {
        if !new_account
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
        {
            return format!("{account_number} - {new_account}");
        }
    }
    new_account.to_string()
}

pub fn get_name_with_abbr(name: &str, company_abbr: &str) -> String {
    let mut parts: Vec<&str> = name.split(" - ").collect();
    if parts
        .last()
        .is_none_or(|part| !part.eq_ignore_ascii_case(company_abbr))
    {
        parts.push(company_abbr);
    }
    parts.join(" - ")
}
