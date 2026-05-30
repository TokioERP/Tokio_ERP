use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosProfileUser {
    pub default: bool,
    pub user: Option<String>,
}

impl PosProfileUser {
    pub const DOCTYPE: &'static str = "POS Profile User";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 2] = ["default", "user"];
    pub const IS_TABLE: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const TRACK_CHANGES: bool = true;

    pub fn new(default: bool, user: impl Into<String>) -> Self {
        Self {
            default,
            user: Some(user.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::check("default", "Default")
                .default("0")
                .in_list_view(),
            FieldSpec::link("user", "User")
                .options("User")
                .in_list_view(),
        ]
    }
}

impl DocumentController for PosProfileUser {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
