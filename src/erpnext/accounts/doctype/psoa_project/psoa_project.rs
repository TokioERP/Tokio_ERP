use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PsoaProject {
    pub project_name: Option<String>,
}

impl PsoaProject {
    pub const DOCTYPE: &'static str = "PSOA Project";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 1] = ["project_name"];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: Some(project_name.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![FieldSpec::link("project_name", "Project").options("Project")]
    }
}

impl DocumentController for PsoaProject {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
