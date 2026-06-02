use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PartyLink {
    pub primary_role: Option<String>,
    pub secondary_role: Option<String>,
    pub primary_party: Option<String>,
    pub secondary_party: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PartyLinkError {
    InvalidPrimaryRole,
    AlreadyLinked {
        primary_role: String,
        primary_party: String,
        secondary_role: String,
        secondary_party: String,
    },
    SecondaryAlreadyLinked {
        secondary_role: String,
        secondary_party: String,
        existing_primary_role: String,
    },
    PrimaryAlreadyLinked {
        primary_role: String,
        primary_party: String,
        existing_primary_role: String,
    },
}

impl PartyLink {
    pub const DOCTYPE: &'static str = "Party Link";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 5] = [
        "primary_role",
        "secondary_role",
        "column_break_2",
        "primary_party",
        "secondary_party",
    ];
    pub const AUTONAME: &'static str = "ACC-PT-LNK-.###.";
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TITLE_FIELD: &'static str = "primary_party";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(
        primary_role: impl Into<String>,
        primary_party: impl Into<String>,
        secondary_role: impl Into<String>,
        secondary_party: impl Into<String>,
    ) -> Self {
        Self {
            primary_role: Some(primary_role.into()),
            secondary_role: Some(secondary_role.into()),
            primary_party: Some(primary_party.into()),
            secondary_party: Some(secondary_party.into()),
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("primary_role", "Primary Role")
                .options("DocType")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("secondary_role", "Secondary Role")
                .options("DocType")
                .depends_on("primary_role")
                .mandatory_depends_on("primary_role"),
            FieldSpec::dynamic_link("primary_party")
                .label("Primary Party")
                .options("primary_role")
                .depends_on("primary_role")
                .mandatory_depends_on("primary_role"),
            FieldSpec::dynamic_link("secondary_party")
                .label("Secondary Party")
                .options("secondary_role")
                .depends_on("secondary_role")
                .mandatory_depends_on("secondary_role"),
        ]
    }

    pub fn validate(
        &self,
        same_pair_existing_primary_role: Option<&str>,
        secondary_as_primary_existing_primary_role: Option<&str>,
        primary_as_secondary_existing_primary_role: Option<&str>,
    ) -> Result<(), PartyLinkError> {
        let primary_role = self.primary_role.as_deref().unwrap_or_default();
        let secondary_role = self.secondary_role.as_deref().unwrap_or_default();
        let primary_party = self.primary_party.as_deref().unwrap_or_default();
        let secondary_party = self.secondary_party.as_deref().unwrap_or_default();

        if !matches!(primary_role, "Customer" | "Supplier") {
            return Err(PartyLinkError::InvalidPrimaryRole);
        }

        if same_pair_existing_primary_role.is_some() {
            return Err(PartyLinkError::AlreadyLinked {
                primary_role: primary_role.to_string(),
                primary_party: primary_party.to_string(),
                secondary_role: secondary_role.to_string(),
                secondary_party: secondary_party.to_string(),
            });
        }

        if let Some(existing_primary_role) = secondary_as_primary_existing_primary_role {
            return Err(PartyLinkError::SecondaryAlreadyLinked {
                secondary_role: secondary_role.to_string(),
                secondary_party: secondary_party.to_string(),
                existing_primary_role: existing_primary_role.to_string(),
            });
        }

        if let Some(existing_primary_role) = primary_as_secondary_existing_primary_role {
            return Err(PartyLinkError::PrimaryAlreadyLinked {
                primary_role: primary_role.to_string(),
                primary_party: primary_party.to_string(),
                existing_primary_role: existing_primary_role.to_string(),
            });
        }

        Ok(())
    }
}

impl DocumentController for PartyLink {
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

pub fn create_party_link(
    primary_role: impl Into<String>,
    primary_party: impl Into<String>,
    secondary_party: impl Into<String>,
) -> PartyLink {
    let primary_role = primary_role.into();
    let secondary_role = if primary_role == "Supplier" {
        "Customer"
    } else {
        "Supplier"
    };

    PartyLink::new(
        primary_role,
        primary_party.into(),
        secondary_role,
        secondary_party.into(),
    )
}
