use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Bank {
    pub name: Option<String>,
    pub bank_name: Option<String>,
    pub plaid_access_token: Option<String>,
    pub swift_number: Option<String>,
    pub website: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankLifecycleAction {
    LoadAddressAndContact { doctype: &'static str, name: String },
    DeleteContactAndAddress(DeleteContactAndAddress),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeleteContactAndAddress {
    pub doctype: &'static str,
    pub name: String,
}

impl Bank {
    pub const DOCTYPE: &'static str = "Bank";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:bank_name";
    pub const FIELD_ORDER: [&'static str; 13] = [
        "bank_details_section",
        "bank_name",
        "swift_number",
        "column_break_1",
        "website",
        "address_and_contact",
        "address_html",
        "column_break_13",
        "contact_html",
        "data_import_configuration_section",
        "bank_transaction_mapping",
        "section_break_4",
        "plaid_access_token",
    ];
    pub const ALLOW_IMPORT: bool = true;
    pub const ALLOW_RENAME: bool = true;
    pub const DOCUMENT_TYPE: &'static str = "Setup";
    pub const EDITABLE_GRID: bool = true;
    pub const QUICK_ENTRY: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: Some(name.clone()),
            bank_name: Some(name),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("bank_details_section").label("Bank Details"),
            FieldSpec::data("bank_name", "Bank Name")
                .required()
                .unique(),
            FieldSpec::data("swift_number", "SWIFT number")
                .unique()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::column_break("column_break_1").search_index(),
            FieldSpec::data("website", "Website"),
            FieldSpec::section_break("address_and_contact")
                .label("Address and Contact")
                .options("fa fa-map-marker"),
            FieldSpec::html("address_html", "Address HTML"),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::html("contact_html", "Contact HTML"),
            FieldSpec::section_break("data_import_configuration_section")
                .label("Data Import Configuration")
                .collapsible(),
            FieldSpec::table("bank_transaction_mapping", "Bank Transaction Mapping")
                .options("Bank Transaction Mapping"),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::data("plaid_access_token", "Plaid Access Token")
                .hidden()
                .no_copy()
                .read_only(),
        ]
    }

    pub fn onload(&self) -> BankLifecycleAction {
        BankLifecycleAction::LoadAddressAndContact {
            doctype: Self::DOCTYPE,
            name: self.name.clone().unwrap_or_default(),
        }
    }

    pub fn on_trash(&self) -> BankLifecycleAction {
        BankLifecycleAction::DeleteContactAndAddress(DeleteContactAndAddress {
            doctype: Self::DOCTYPE,
            name: self.name.clone().unwrap_or_default(),
        })
    }
}

impl DocumentController for Bank {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["onload", "on_trash"]
    }
}
