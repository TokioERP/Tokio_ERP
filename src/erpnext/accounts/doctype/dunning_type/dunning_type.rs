use crate::erpnext::accounts::doctype::dunning_letter_text::dunning_letter_text::DunningLetterText;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DunningType {
    pub name: Option<String>,
    pub dunning_type: Option<String>,
    pub is_default: bool,
    pub company: Option<String>,
    pub dunning_fee: f64,
    pub rate_of_interest: f64,
    pub dunning_letter_text: Vec<DunningLetterText>,
    pub income_account: Option<String>,
    pub cost_center: Option<String>,
}

impl DunningType {
    pub const DOCTYPE: &'static str = "Dunning Type";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 14] = [
        "dunning_type",
        "is_default",
        "column_break_3",
        "company",
        "section_break_6",
        "dunning_fee",
        "column_break_8",
        "rate_of_interest",
        "text_block_section",
        "dunning_letter_text",
        "section_break_9",
        "income_account",
        "column_break_13",
        "cost_center",
    ];
    pub const ALLOW_RENAME: bool = true;
    pub const BETA: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const NAMING_RULE: &'static str = "By script";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const TRACK_CHANGES: bool = true;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("dunning_type", "Dunning Type")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::check("is_default", "Is Default").default("0"),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::currency("dunning_fee", "Dunning Fee").in_list_view(),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::float("rate_of_interest", "Rate of Interest (%) Yearly").in_list_view(),
            FieldSpec::section_break("text_block_section")
                .label("Dunning Letter")
                .description("This section allows the user to set the Body and Closing text of the Dunning Letter for the Dunning Type based on language, which can be used in Print."),
            FieldSpec::table_unlabeled("dunning_letter_text").options("Dunning Letter Text"),
            FieldSpec::section_break("section_break_9").label("Accounting Details"),
            FieldSpec::link("income_account", "Income Account").options("Account"),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
        ]
    }

    pub fn autoname_with_company_abbr(&mut self, company_abbr: &str) -> String {
        let dunning_type = self.dunning_type.as_deref().unwrap_or_default();
        let name = format!("{dunning_type} - {company_abbr}");
        self.name = Some(name.clone());
        name
    }
}

impl DocumentController for DunningType {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["autoname"]
    }
}
