use crate::erpnext::accounts::doctype::share_balance::share_balance::ShareBalance;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Shareholder {
    pub name: String,
    pub title: String,
    pub naming_series: Option<String>,
    pub folio_no: Option<String>,
    pub company: String,
    pub is_company: bool,
    pub share_balance: Vec<ShareBalance>,
    pub contact_list: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShareholderAction {
    LoadAddressAndContact,
    DeleteContactAndAddress { doctype: String, name: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareholderDashboard {
    pub fieldname: &'static str,
    pub non_standard_fieldnames: Vec<(&'static str, &'static str)>,
    pub transactions: Vec<Vec<&'static str>>,
}

impl Shareholder {
    pub const DOCTYPE: &'static str = "Shareholder";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "naming_series:";
    pub const TITLE_FIELD: &'static str = "title";
    pub const SEARCH_FIELDS: &'static str = "folio_no";
    pub const FIELD_ORDER: [&'static str; 15] = [
        "title",
        "column_break_2",
        "naming_series",
        "section_break_2",
        "folio_no",
        "column_break_4",
        "company",
        "is_company",
        "address_contacts",
        "address_html",
        "column_break_9",
        "contact_html",
        "section_break_3",
        "share_balance",
        "contact_list",
    ];
    pub const TRACK_CHANGES: bool = true;

    pub fn new(title: impl Into<String>, company: impl Into<String>) -> Self {
        let title = title.into();
        Self {
            name: title.clone(),
            title,
            company: company.into(),
            ..Self::default()
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("title", "Title").required(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::select("naming_series", "").options("ACC-SH-.YYYY.-"),
            FieldSpec::section_break("section_break_2"),
            FieldSpec::data("folio_no", "Folio no.")
                .read_only()
                .unique(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required()
                .in_list_view(),
            FieldSpec::check("is_company", "Is Company")
                .default("0")
                .hidden()
                .read_only(),
            FieldSpec::section_break("address_contacts")
                .label("Address and Contacts")
                .options("fa fa-map-marker"),
            FieldSpec::html("address_html", "Address HTML").read_only(),
            FieldSpec::column_break("column_break_9"),
            FieldSpec::html("contact_html", "Contact HTML").read_only(),
            FieldSpec::section_break("section_break_3").label("Share Balance"),
            FieldSpec::table("share_balance", "Share Balance")
                .options("Share Balance")
                .read_only(),
            FieldSpec::code("contact_list", "Contact List")
                .description("Hidden list maintaining the list of contacts linked to Shareholder")
                .hidden()
                .read_only(),
        ]
    }

    pub fn onload(&self) -> ShareholderAction {
        ShareholderAction::LoadAddressAndContact
    }

    pub fn on_trash(&self) -> ShareholderAction {
        ShareholderAction::DeleteContactAndAddress {
            doctype: Self::DOCTYPE.to_string(),
            name: self.name.clone(),
        }
    }

    pub fn before_save(&mut self) {
        for entry in &mut self.share_balance {
            entry.amount = entry.no_of_shares * entry.rate;
        }
    }
}

impl DocumentController for Shareholder {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["onload", "on_trash", "before_save"]
    }
}

pub fn shareholder_dashboard() -> ShareholderDashboard {
    ShareholderDashboard {
        fieldname: "shareholder",
        non_standard_fieldnames: vec![("Share Transfer", "to_shareholder")],
        transactions: vec![vec!["Share Transfer"]],
    }
}

pub fn shareholder_js_hooks() -> [&'static str; 2] {
    ["refresh", "validate"]
}
