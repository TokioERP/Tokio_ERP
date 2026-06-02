use std::collections::{BTreeMap, BTreeSet};

use crate::erpnext::{DocumentController, FieldSpec};

pub const PROHIBITED_DIMENSION_DOCTYPES: [&str; 25] = [
    "DefaultValue",
    "DocType",
    "DocField",
    "DocPerm",
    "DocType Action",
    "DocType Link",
    "User",
    "Role",
    "Has Role",
    "Page",
    "Module Def",
    "Print Format",
    "Report",
    "Customize Form",
    "Customize Form Field",
    "Property Setter",
    "Custom Field",
    "Client Script",
    "Accounting Dimension",
    "Project",
    "Cost Center",
    "Accounting Dimension Detail",
    "Company",
    "Account",
    "Finance Book",
];

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimension {
    pub name: Option<String>,
    pub document_type: String,
    pub label: Option<String>,
    pub fieldname: Option<String>,
    pub dimension_defaults: Vec<AccountingDimensionDefault>,
    pub disabled: bool,
    pub is_new: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimensionDefault {
    pub parent: Option<String>,
    pub company: String,
    pub default_dimension: Option<String>,
    pub mandatory_for_pl: bool,
    pub mandatory_for_bs: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimensionRecord {
    pub label: String,
    pub fieldname: String,
    pub disabled: bool,
    pub document_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountingDimensionError {
    NotAllowedDocumentType { document_type: String },
    DuplicateDocumentType,
    DocumentTypeChanged,
    InvalidFieldname { fieldname: String },
    DuplicateDefaultCompany { company: String },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CustomFieldPlan {
    pub doctype: String,
    pub fieldname: String,
    pub label: String,
    pub fieldtype: String,
    pub options: String,
    pub insert_after: String,
    pub owner: Option<String>,
    pub allow_on_submit: bool,
    pub depends_on: Option<String>,
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PropertySetterPlan {
    pub name: String,
    pub doctype_or_field: String,
    pub doc_type: String,
    pub field_name: String,
    pub property: String,
    pub property_type: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AccountingDimensionSideEffectPlan {
    pub custom_fields: Vec<CustomFieldPlan>,
    pub property_setters: Vec<PropertySetterPlan>,
    pub clear_cache_doctypes: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeleteAccountingDimensionPlan {
    pub fieldname: String,
    pub doctypes: Vec<String>,
    pub budget_against_options: String,
    pub clear_cache_doctypes: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DimensionTogglePlan {
    pub updates: Vec<(String, String, bool)>,
    pub clear_cache_doctypes: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DimensionNode {
    pub name: String,
    pub lft: i32,
    pub rgt: i32,
}

impl AccountingDimension {
    pub const DOCTYPE: &'static str = "Accounting Dimension";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:label";
    pub const FIELD_ORDER: [&'static str; 5] = [
        "document_type",
        "label",
        "fieldname",
        "dimension_defaults",
        "disabled",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("document_type", "Reference Document Type")
                .options("DocType")
                .read_only_depends_on("eval:!doc.__islocal")
                .required()
                .search_index(),
            FieldSpec::data("label", "Dimension Name")
                .in_list_view()
                .unique(),
            FieldSpec::data("fieldname", "Fieldname").hidden(),
            FieldSpec::table("dimension_defaults", "Dimension Defaults")
                .options("Accounting Dimension Detail"),
            FieldSpec::check("disabled", "Disable")
                .default("0")
                .read_only()
                .hidden(),
        ]
    }

    pub fn before_insert(&mut self) {
        self.set_fieldname_and_label();
    }

    pub fn validate(
        &self,
        existing_document_types: &BTreeSet<String>,
        document_type_exists_for_new: bool,
        document_type_before_save: Option<&str>,
        doctype_fields: &[(String, Vec<String>)],
    ) -> Result<Vec<String>, AccountingDimensionError> {
        self.validate_doctype(existing_document_types, document_type_exists_for_new)?;
        if !self.is_new {
            self.validate_document_type_change(document_type_before_save)?;
        }
        self.validate_column_name()?;
        let conflicting_doctypes = self.validate_fieldname_conflict(doctype_fields);
        self.validate_dimension_defaults()?;

        Ok(conflicting_doctypes)
    }

    pub fn validate_doctype(
        &self,
        existing_document_types: &BTreeSet<String>,
        document_type_exists_for_new: bool,
    ) -> Result<(), AccountingDimensionError> {
        if PROHIBITED_DIMENSION_DOCTYPES.contains(&self.document_type.as_str()) {
            return Err(AccountingDimensionError::NotAllowedDocumentType {
                document_type: self.document_type.clone(),
            });
        }

        if (document_type_exists_for_new || existing_document_types.contains(&self.document_type))
            && self.is_new
        {
            return Err(AccountingDimensionError::DuplicateDocumentType);
        }

        Ok(())
    }

    pub fn validate_document_type_change(
        &self,
        document_type_before_save: Option<&str>,
    ) -> Result<(), AccountingDimensionError> {
        if document_type_before_save.is_some_and(|before| before != self.document_type) {
            return Err(AccountingDimensionError::DocumentTypeChanged);
        }

        Ok(())
    }

    pub fn validate_fieldname_conflict(
        &self,
        doctype_fields: &[(String, Vec<String>)],
    ) -> Vec<String> {
        let fieldname = self.fieldname.as_deref().unwrap_or_default();
        doctype_fields
            .iter()
            .filter(|(_, fields)| fields.iter().any(|field| field == fieldname))
            .map(|(doctype, _)| doctype.clone())
            .collect()
    }

    pub fn validate_dimension_defaults(&self) -> Result<(), AccountingDimensionError> {
        let mut companies = BTreeSet::new();
        for default in &self.dimension_defaults {
            if !companies.insert(default.company.clone()) {
                return Err(AccountingDimensionError::DuplicateDefaultCompany {
                    company: default.company.clone(),
                });
            }
        }

        Ok(())
    }

    pub fn set_fieldname_and_label(&mut self) {
        if self.label.as_deref().unwrap_or_default().is_empty() {
            self.label = Some(self.document_type.clone());
        }

        if self.fieldname.as_deref().unwrap_or_default().is_empty() {
            self.fieldname = self.label.as_deref().map(scrub);
        }
    }

    fn validate_column_name(&self) -> Result<(), AccountingDimensionError> {
        let fieldname = self.fieldname.as_deref().unwrap_or_default();
        let valid = !fieldname.is_empty()
            && fieldname
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
            && fieldname
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_lowercase());

        if valid {
            Ok(())
        } else {
            Err(AccountingDimensionError::InvalidFieldname {
                fieldname: fieldname.to_string(),
            })
        }
    }
}

impl DocumentController for AccountingDimension {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["before_insert", "validate", "on_update", "on_trash"]
    }
}

pub fn make_dimension_in_accounting_doctypes_plan(
    doc: &AccountingDimension,
    doclist: &[&str],
    accounting_dimension_count: usize,
    repostable_doctypes: &[&str],
    existing_custom_fields: &BTreeMap<String, Vec<String>>,
    existing_budget_against_options: Option<&str>,
) -> AccountingDimensionSideEffectPlan {
    let mut plan = AccountingDimensionSideEffectPlan::default();
    let fieldname = doc.fieldname.as_deref().unwrap_or_default();
    let label = doc.label.as_deref().unwrap_or_default();

    for doctype in doclist {
        let insert_after = if (accounting_dimension_count + 1) % 2 == 0 {
            "dimension_col_break"
        } else {
            "accounting_dimensions_section"
        };
        let custom_field = CustomFieldPlan {
            doctype: (*doctype).to_string(),
            fieldname: fieldname.to_string(),
            label: label.to_string(),
            fieldtype: "Link".to_string(),
            options: doc.document_type.clone(),
            insert_after: insert_after.to_string(),
            owner: Some("Administrator".to_string()),
            allow_on_submit: repostable_doctypes.contains(doctype),
            depends_on: None,
            read_only: None,
        };

        let field_exists = existing_custom_fields
            .get(*doctype)
            .is_some_and(|fields| fields.iter().any(|field| field == fieldname));
        if !field_exists {
            if *doctype == "Budget" {
                let (budget_field, property_setter) = add_dimension_to_budget_doctype(
                    custom_field,
                    doc,
                    existing_budget_against_options,
                );
                plan.custom_fields.push(budget_field);
                plan.property_setters.push(property_setter);
            } else {
                plan.custom_fields.push(custom_field);
            }
        }

        plan.clear_cache_doctypes.push((*doctype).to_string());
    }

    plan
}

pub fn add_dimension_to_budget_doctype(
    mut custom_field: CustomFieldPlan,
    doc: &AccountingDimension,
    existing_budget_against_options: Option<&str>,
) -> (CustomFieldPlan, PropertySetterPlan) {
    custom_field.insert_after = "cost_center".to_string();
    custom_field.depends_on = Some(format!(
        "eval:doc.budget_against == '{}'",
        doc.document_type
    ));

    let value = existing_budget_against_options
        .map(|value| format!("{value}\n{}", doc.document_type))
        .unwrap_or_else(|| format!("\nCost Center\nProject\n{}", doc.document_type));

    (
        custom_field,
        PropertySetterPlan {
            name: "Budget-budget_against-options".to_string(),
            doctype_or_field: "DocField".to_string(),
            doc_type: "Budget".to_string(),
            field_name: "budget_against".to_string(),
            property: "options".to_string(),
            property_type: "Text".to_string(),
            value,
        },
    )
}

pub fn delete_accounting_dimension_plan(
    doc: &AccountingDimension,
    doclist: &[&str],
    budget_against_options: &str,
) -> DeleteAccountingDimensionPlan {
    let mut value_list = budget_against_options
        .split('\n')
        .skip(3)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    value_list.retain(|value| value != &doc.document_type);

    DeleteAccountingDimensionPlan {
        fieldname: doc.fieldname.clone().unwrap_or_default(),
        doctypes: doclist
            .iter()
            .map(|doctype| (*doctype).to_string())
            .collect(),
        budget_against_options: format!("\nCost Center\nProject\n{}", value_list.join("\n")),
        clear_cache_doctypes: doclist
            .iter()
            .map(|doctype| (*doctype).to_string())
            .collect(),
    }
}

pub fn toggle_disabling_plan(
    fieldname: &str,
    disabled: bool,
    doclist: &[&str],
    existing_custom_fields: &BTreeSet<String>,
) -> DimensionTogglePlan {
    DimensionTogglePlan {
        updates: doclist
            .iter()
            .filter(|doctype| existing_custom_fields.contains(&format!("{doctype}:{fieldname}")))
            .map(|doctype| ((*doctype).to_string(), fieldname.to_string(), disabled))
            .collect(),
        clear_cache_doctypes: doclist
            .iter()
            .map(|doctype| (*doctype).to_string())
            .collect(),
    }
}

pub fn get_accounting_dimensions(
    records: &[AccountingDimensionRecord],
    as_list: bool,
) -> Vec<AccountingDimensionRecord> {
    let active = records
        .iter()
        .filter(|record| !record.disabled)
        .cloned()
        .collect::<Vec<_>>();
    if as_list {
        active
            .into_iter()
            .map(|record| AccountingDimensionRecord {
                fieldname: record.fieldname,
                ..Default::default()
            })
            .collect()
    } else {
        active
    }
}

pub fn get_checks_for_pl_and_bs_accounts(
    records: &[AccountingDimensionRecord],
    defaults: &[AccountingDimensionDefault],
) -> Vec<(String, bool, String, Option<String>, String, bool, bool)> {
    defaults
        .iter()
        .filter_map(|default| {
            let parent = default.parent.as_deref()?;
            let record = records
                .iter()
                .find(|record| !record.disabled && record.label == parent)?;
            Some((
                record.label.clone(),
                record.disabled,
                record.fieldname.clone(),
                default.default_dimension.clone(),
                default.company.clone(),
                default.mandatory_for_pl,
                default.mandatory_for_bs,
            ))
        })
        .collect()
}

pub fn get_dimension_with_children(nodes: &[DimensionNode], dimensions: &[String]) -> Vec<String> {
    let mut all_dimensions = Vec::new();

    for dimension in dimensions {
        if let Some(root) = nodes.iter().find(|node| &node.name == dimension) {
            let mut children = nodes
                .iter()
                .filter(|node| node.lft >= root.lft && node.rgt <= root.rgt)
                .collect::<Vec<_>>();
            children.sort_by_key(|node| node.lft);
            all_dimensions.extend(children.into_iter().map(|node| node.name.clone()));
        }
    }

    all_dimensions
}

pub fn get_dimensions(
    accounting_dimensions: &[AccountingDimensionRecord],
    default_dimensions: &[AccountingDimensionDefault],
    with_cost_center_and_project: impl IntoWithCostCenterFlag,
) -> (
    Vec<AccountingDimensionRecord>,
    BTreeMap<String, BTreeMap<String, Option<String>>>,
) {
    let mut dimension_filters = accounting_dimensions
        .iter()
        .filter(|dimension| !dimension.disabled)
        .cloned()
        .collect::<Vec<_>>();

    if with_cost_center_and_project.into_flag() {
        dimension_filters.extend([
            AccountingDimensionRecord {
                fieldname: "cost_center".to_string(),
                document_type: "Cost Center".to_string(),
                ..Default::default()
            },
            AccountingDimensionRecord {
                fieldname: "project".to_string(),
                document_type: "Project".to_string(),
                ..Default::default()
            },
        ]);
    }

    let label_to_fieldname = accounting_dimensions
        .iter()
        .map(|dimension| (dimension.label.as_str(), dimension.fieldname.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut default_dimensions_map: BTreeMap<String, BTreeMap<String, Option<String>>> =
        BTreeMap::new();
    for dimension in default_dimensions {
        if let Some(parent) = dimension.parent.as_deref() {
            if let Some(fieldname) = label_to_fieldname.get(parent) {
                default_dimensions_map
                    .entry(dimension.company.clone())
                    .or_default()
                    .insert(
                        (*fieldname).to_string(),
                        dimension.default_dimension.clone(),
                    );
            }
        }
    }

    (dimension_filters, default_dimensions_map)
}

pub fn create_accounting_dimensions_for_doctype_plan(
    doctype: &str,
    accounting_dimensions: &[AccountingDimensionRecord],
    existing_custom_fields: &BTreeSet<String>,
) -> AccountingDimensionSideEffectPlan {
    let mut plan = AccountingDimensionSideEffectPlan::default();

    for dimension in accounting_dimensions {
        let key = format!("{}:{}", doctype, dimension.fieldname);
        if existing_custom_fields.contains(&key) {
            continue;
        }

        plan.custom_fields.push(CustomFieldPlan {
            doctype: doctype.to_string(),
            fieldname: dimension.fieldname.clone(),
            label: dimension.label.clone(),
            fieldtype: "Link".to_string(),
            options: dimension.document_type.clone(),
            insert_after: "accounting_dimensions_section".to_string(),
            owner: None,
            allow_on_submit: false,
            depends_on: None,
            read_only: None,
        });
    }

    if !accounting_dimensions.is_empty() {
        plan.clear_cache_doctypes.push(doctype.to_string());
    }

    plan
}

pub trait IntoWithCostCenterFlag {
    fn into_flag(self) -> bool;
}

impl IntoWithCostCenterFlag for bool {
    fn into_flag(self) -> bool {
        self
    }
}

impl IntoWithCostCenterFlag for &str {
    fn into_flag(self) -> bool {
        self.to_ascii_lowercase().trim() == "true"
    }
}

fn scrub(value: &str) -> String {
    let mut scrubbed = String::new();
    let mut previous_underscore = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            scrubbed.push(ch.to_ascii_lowercase());
            previous_underscore = false;
        } else if !previous_underscore {
            scrubbed.push('_');
            previous_underscore = true;
        }
    }

    scrubbed.trim_matches('_').to_string()
}
