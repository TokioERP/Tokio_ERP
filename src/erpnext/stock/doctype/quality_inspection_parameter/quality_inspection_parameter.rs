use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QualityInspectionParameter {
    pub parameter: Option<String>,
    pub parameter_group: Option<String>,
    pub description: Option<String>,
}

impl QualityInspectionParameter {
    pub const DOCTYPE: &'static str = "Quality Inspection Parameter";
    pub const MODULE: &'static str = "Stock";
    pub const AUTONAME: &'static str = "field:parameter";
    pub const FIELD_ORDER: [&'static str; 3] = ["parameter", "parameter_group", "description"];
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        parameter: impl Into<String>,
        parameter_group: Option<&str>,
        description: Option<&str>,
    ) -> Self {
        Self {
            parameter: Some(parameter.into()),
            parameter_group: parameter_group.map(ToOwned::to_owned),
            description: description.map(ToOwned::to_owned),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("parameter", "Parameter")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::link("parameter_group", "Parameter Group")
                .options("Quality Inspection Parameter Group")
                .in_list_view(),
            FieldSpec::text_editor("description", "Description"),
        ]
    }
}

impl DocumentController for QualityInspectionParameter {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
