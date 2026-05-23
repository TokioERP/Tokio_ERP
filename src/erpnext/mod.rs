pub mod accounts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldSpec {
    pub fieldname: &'static str,
    pub fieldtype: &'static str,
    pub label: Option<&'static str>,
    pub options: Option<&'static str>,
    pub default: Option<&'static str>,
    pub description: Option<&'static str>,
    pub documentation_url: Option<&'static str>,
    pub columns: Option<u8>,
    pub depends_on: Option<&'static str>,
    pub mandatory_depends_on: Option<&'static str>,
    pub oldfieldname: Option<&'static str>,
    pub oldfieldtype: Option<&'static str>,
    pub width: Option<&'static str>,
    pub fetch_from: Option<&'static str>,
    pub precision: Option<&'static str>,
    pub read_only: bool,
    pub fetch_if_empty: bool,
    pub in_filter: bool,
    pub in_list_view: bool,
    pub in_standard_filter: bool,
    pub ignore_user_permissions: bool,
    pub search_index: bool,
    pub allow_on_submit: bool,
    pub collapsible: bool,
    pub hidden: bool,
    pub no_copy: bool,
    pub print_hide: bool,
    pub report_hide: bool,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn read_only_field(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Read Only",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn table(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Table",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn table_multiselect(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Table MultiSelect",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn table_unlabeled(fieldname: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Table",
            label: None,
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn section_break(fieldname: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Section Break",
            label: None,
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn currency(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Currency",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn float(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Float",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn int(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Int",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn percent(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Percent",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn text(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Text",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn small_text(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Small Text",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn long_text(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Long Text",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn text_editor(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Text Editor",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn html(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "HTML",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn button(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "Button",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
            required: false,
            unique: false,
        }
    }

    pub const fn json(fieldname: &'static str, label: &'static str) -> Self {
        Self {
            fieldname,
            fieldtype: "JSON",
            label: Some(label),
            options: None,
            default: None,
            description: None,
            documentation_url: None,
            columns: None,
            depends_on: None,
            mandatory_depends_on: None,
            oldfieldname: None,
            oldfieldtype: None,
            width: None,
            fetch_from: None,
            precision: None,
            read_only: false,
            fetch_if_empty: false,
            in_filter: false,
            in_list_view: false,
            in_standard_filter: false,
            ignore_user_permissions: false,
            search_index: false,
            allow_on_submit: false,
            collapsible: false,
            hidden: false,
            no_copy: false,
            print_hide: false,
            report_hide: false,
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

    pub const fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn documentation_url(mut self, documentation_url: &'static str) -> Self {
        self.documentation_url = Some(documentation_url);
        self
    }

    pub const fn columns(mut self, columns: u8) -> Self {
        self.columns = Some(columns);
        self
    }

    pub const fn depends_on(mut self, depends_on: &'static str) -> Self {
        self.depends_on = Some(depends_on);
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

    pub const fn fetch_from(mut self, fetch_from: &'static str) -> Self {
        self.fetch_from = Some(fetch_from);
        self
    }

    pub const fn precision(mut self, precision: &'static str) -> Self {
        self.precision = Some(precision);
        self
    }

    pub const fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub const fn fetch_if_empty(mut self) -> Self {
        self.fetch_if_empty = true;
        self
    }

    pub const fn in_filter(mut self) -> Self {
        self.in_filter = true;
        self
    }

    pub const fn in_list_view(mut self) -> Self {
        self.in_list_view = true;
        self
    }

    pub const fn in_standard_filter(mut self) -> Self {
        self.in_standard_filter = true;
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

    pub const fn search_index(mut self) -> Self {
        self.search_index = true;
        self
    }

    pub const fn allow_on_submit(mut self) -> Self {
        self.allow_on_submit = true;
        self
    }

    pub const fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    pub const fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    pub const fn no_copy(mut self) -> Self {
        self.no_copy = true;
        self
    }

    pub const fn print_hide(mut self) -> Self {
        self.print_hide = true;
        self
    }

    pub const fn report_hide(mut self) -> Self {
        self.report_hide = true;
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
