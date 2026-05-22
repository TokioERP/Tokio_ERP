use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PosField {
    pub fieldname: Option<String>,
    pub label: Option<String>,
    pub fieldtype: Option<String>,
    pub options: Option<String>,
    pub default_value: Option<String>,
    pub reqd: bool,
    pub read_only: bool,
}

impl PosField {
    pub const DOCTYPE: &'static str = "POS Field";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 8] = [
        "fieldname",
        "label",
        "fieldtype",
        "column_break_7",
        "options",
        "default_value",
        "reqd",
        "read_only",
    ];
    pub const IS_TABLE: bool = true;
    pub const EDITABLE_GRID: bool = true;

    pub fn new(
        fieldname: impl Into<String>,
        label: impl Into<String>,
        fieldtype: impl Into<String>,
    ) -> Self {
        Self {
            fieldname: Some(fieldname.into()),
            label: Some(label.into()),
            fieldtype: Some(fieldtype.into()),
            options: None,
            default_value: None,
            reqd: false,
            read_only: false,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::select("fieldname", "Fieldname").in_list_view(),
            FieldSpec::data("label", "Label").in_list_view().read_only(),
            FieldSpec::data("fieldtype", "Fieldtype")
                .in_list_view()
                .read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::text("options", "Options")
                .in_list_view()
                .read_only(),
            FieldSpec::data("default_value", "Default Value"),
            FieldSpec::check("reqd", "Mandatory").default("0"),
            FieldSpec::check("read_only", "Read Only").default("0"),
        ]
    }
}

impl DocumentController for PosField {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
