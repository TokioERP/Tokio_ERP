use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QualityInspectionParameterGroup {
    pub group_name: Option<String>,
}

impl QualityInspectionParameterGroup {
    pub const DOCTYPE: &'static str = "Quality Inspection Parameter Group";
    pub const MODULE: &'static str = "Stock";
    pub const AUTONAME: &'static str = "field:group_name";
    pub const FIELD_ORDER: [&'static str; 1] = ["group_name"];
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(group_name: impl Into<String>) -> Self {
        Self {
            group_name: Some(group_name.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::data("group_name", "Parameter Group Name")
            .in_list_view()
            .required()
            .unique()]
    }
}

impl DocumentController for QualityInspectionParameterGroup {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
