pub mod accounts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSpec {
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub label: Option<&'static str>,
    pub options: Option<&'static str>,
    pub default: Option<&'static str>,
    pub columns: Option<u8>,
    pub mandatory_depends_on: Option<&'static str>,
    pub oldfieldname: Option<&'static str>,
    pub oldfieldtype: Option<&'static str>,
    pub width: Option<&'static str>,
    pub read_only: bool,
    pub in_list_view: bool,
    pub ignore_user_permissions: bool,
    pub hidden: bool,
    pub required: bool,
    pub unique: bool,
}

impl FieldSpec {
    pub const fn data(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Data",
            label: Some(label),
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn link(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Link",
            label: Some(label),
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn dynamic_link(fieldname: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Dynamic Link",
            label: None,
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn check(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Check",
            label: Some(label),
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn select(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Select",
            label: Some(label),
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn column_break(fieldname: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Column Break",
            label: None,
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn date(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Date",
            label: Some(label),
            options: None,
            default: None,
            columns: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            read_only: false,
            in_list_view: false,
            ignore_user_permissions: false,
            hidden: false,
            required: false,
            unique: false,
        }
    }

    pub const fn label(mut self, label: &'static str) -> Self {
        self.label = Some(label);
        self
    }

    pub const fn options(mut self, options: &'static str) -> Self {
        self.options = Some(options);
        self
    }

    pub const fn default(mut self, default: &'static str) -> Self {
        self.default = Some(default);
        self
    }

    pub const fn columns(mut self, columns: u8) -> Self {
        self.columns = Some(columns);
        self
    }

    pub const fn mandatory_depends_on(mut self, mandatory_depends_on: &'static str) -> Self {
        self.mandatory_depends_on = Some(mandatory_depends_on);
        self
    }

    pub const fn oldfield(
        mut self,
        oldfieldname: &'static str,
        oldfieldtype: &'static str,
    ) -> Self {
        self.oldfieldname = Some(oldfieldname);
        self.oldfieldtype = Some(oldfieldtype);
        self
    }

    pub const fn width(mut self, width: &'static str) -> Self {
        self.width = Some(width);
        self
    }

    pub const fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub const fn in_list_view(mut self) -> Self {
        self.in_list_view = true;
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub const fn ignore_user_permissions(mut self) -> Self {
        self.ignore_user_permissions = true;
        self
    }

    pub const fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub const fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}

pub trait DocumentController {
    fn doctype(&self) -> &'static str;

    fn module(&self) -> &'static str;

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[]
    }
}
