use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DunningLetterText {
    pub language: Option<String>,
    pub is_default_language: bool,
    pub body_text: Option<String>,
    pub closing_text: Option<String>,
}

impl DunningLetterText {
    pub const DOCTYPE: &'static str = "Dunning Letter Text";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "language",
        "is_default_language",
        "section_break_4",
        "body_text",
        "closing_text",
        "section_break_7",
        "body_and_closing_text_help",
    ];
    pub const IS_TABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const BODY_AND_CLOSING_TEXT_HELP: &'static str = "<h4>Body Text and Closing Text Example</h4>\n\n<div>We have noticed that you have not yet paid invoice {{sales_invoice}} for {{frappe.db.get_value(\"Currency\", currency, \"symbol\")}} {{outstanding_amount}}. This is a friendly reminder that the invoice was due on {{due_date}}. Please pay the amount due immediately to avoid any further dunning cost.</div>\n\n<h4>How to get fieldnames</h4>\n\n<p>The fieldnames you can use in your template are the fields in the document. You can find out the fields of any documents via Setup &gt; Customize Form View and selecting the document type (e.g. Sales Invoice)</p>\n\n<h4>Templating</h4>\n\n<p>Templates are compiled using the Jinja Templating Language. To learn more about Jinja, <a class=\"strong\" href=\"http://jinja.pocoo.org/docs/dev/templates/\">read this documentation.</a></p>";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("language", "Language")
                .options("Language")
                .in_list_view(),
            FieldSpec::check("is_default_language", "Is Default Language").default("0"),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::text_editor("body_text", "Body Text").in_list_view(),
            FieldSpec::text_editor("closing_text", "Closing Text").in_list_view(),
            FieldSpec::section_break("section_break_7"),
            FieldSpec::html("body_and_closing_text_help", "Body and Closing Text Help")
                .options(Self::BODY_AND_CLOSING_TEXT_HELP),
        ]
    }
}

impl DocumentController for DunningLetterText {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
