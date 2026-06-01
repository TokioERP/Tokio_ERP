use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, PartialEq)]
pub struct ChequePrintTemplate {
    pub bank_name: String,
    pub cheque_size: String,
    pub starting_position_from_top_edge: f64,
    pub cheque_width: f64,
    pub cheque_height: f64,
    pub scanned_cheque: Option<String>,
    pub is_account_payable: bool,
    pub acc_pay_dist_from_top_edge: f64,
    pub acc_pay_dist_from_left_edge: f64,
    pub message_to_show: Option<String>,
    pub date_dist_from_top_edge: f64,
    pub date_dist_from_left_edge: f64,
    pub payer_name_from_top_edge: f64,
    pub payer_name_from_left_edge: f64,
    pub amt_in_words_from_top_edge: f64,
    pub amt_in_words_from_left_edge: f64,
    pub amt_in_word_width: f64,
    pub amt_in_words_line_spacing: f64,
    pub amt_in_figures_from_top_edge: f64,
    pub amt_in_figures_from_left_edge: f64,
    pub acc_no_dist_from_top_edge: f64,
    pub acc_no_dist_from_left_edge: f64,
    pub signatory_from_top_edge: f64,
    pub signatory_from_left_edge: f64,
    pub has_print_format: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrintFormatPlan {
    pub exists: bool,
    pub doc_type: String,
    pub standard: String,
    pub custom_format: bool,
    pub print_format_type: String,
    pub name: String,
    pub html: String,
    pub set_has_print_format_for: String,
}

impl Default for ChequePrintTemplate {
    fn default() -> Self {
        Self {
            bank_name: String::new(),
            cheque_size: "Regular".to_string(),
            starting_position_from_top_edge: 0.0,
            cheque_width: 20.0,
            cheque_height: 9.0,
            scanned_cheque: None,
            is_account_payable: true,
            acc_pay_dist_from_top_edge: 1.0,
            acc_pay_dist_from_left_edge: 9.0,
            message_to_show: Some("Acc. Payee".to_string()),
            date_dist_from_top_edge: 1.0,
            date_dist_from_left_edge: 15.0,
            payer_name_from_top_edge: 2.0,
            payer_name_from_left_edge: 3.0,
            amt_in_words_from_top_edge: 3.0,
            amt_in_words_from_left_edge: 4.0,
            amt_in_word_width: 15.0,
            amt_in_words_line_spacing: 0.5,
            amt_in_figures_from_top_edge: 3.5,
            amt_in_figures_from_left_edge: 16.0,
            acc_no_dist_from_top_edge: 5.0,
            acc_no_dist_from_left_edge: 4.0,
            signatory_from_top_edge: 6.0,
            signatory_from_left_edge: 15.0,
            has_print_format: false,
        }
    }
}

impl ChequePrintTemplate {
    pub const DOCTYPE: &'static str = "Cheque Print Template";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "field:bank_name";
    pub const FIELD_ORDER: [&'static str; 39] = [
        "settings",
        "has_print_format",
        "primary_settings",
        "bank_name",
        "cheque_size",
        "starting_position_from_top_edge",
        "cheque_width",
        "cheque_height",
        "scanned_cheque",
        "column_break_5",
        "is_account_payable",
        "acc_pay_dist_from_top_edge",
        "acc_pay_dist_from_left_edge",
        "message_to_show",
        "date_and_payer_settings",
        "date_settings",
        "date_dist_from_top_edge",
        "date_dist_from_left_edge",
        "payer_settings",
        "payer_name_from_top_edge",
        "payer_name_from_left_edge",
        "amount_in_words_and_figure_settings",
        "html_19",
        "amt_in_words_from_top_edge",
        "amt_in_words_from_left_edge",
        "amt_in_word_width",
        "amt_in_words_line_spacing",
        "amount_in_figure",
        "amt_in_figures_from_top_edge",
        "amt_in_figures_from_left_edge",
        "account_number_and_signatory_settings",
        "account_no_settings",
        "acc_no_dist_from_top_edge",
        "acc_no_dist_from_left_edge",
        "signatory_position",
        "signatory_from_top_edge",
        "signatory_from_left_edge",
        "preview",
        "cheque_print_preview",
    ];
    pub const MAX_ATTACHMENTS: u8 = 1;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::html("settings", "")
                .options("<div>\n<h3> All dimensions in centimeter only </h3>\n</div>"),
            FieldSpec::check("has_print_format", "Has Print Format")
                .default("0")
                .hidden()
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::section_break("primary_settings").label("Primary Settings"),
            FieldSpec::data("bank_name", "Bank Name")
                .in_list_view()
                .no_copy()
                .required()
                .unique(),
            FieldSpec::select("cheque_size", "Cheque Size")
                .options("\nRegular\nA4")
                .default("Regular"),
            FieldSpec::float("starting_position_from_top_edge", "Starting position from top edge")
                .depends_on("eval:doc.cheque_size==\"A4\"")
                .precision("2"),
            FieldSpec::float("cheque_width", "Cheque Width")
                .default("20.00")
                .precision("2"),
            FieldSpec::float("cheque_height", "Cheque Height")
                .default("9.00")
                .precision("2"),
            FieldSpec::attach("scanned_cheque", "Scanned Cheque"),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::check("is_account_payable", "Is Account Payable").default("1"),
            FieldSpec::float("acc_pay_dist_from_top_edge", "Distance from top edge")
                .default("1.00")
                .depends_on("eval:doc.is_account_payable")
                .precision("2"),
            FieldSpec::float("acc_pay_dist_from_left_edge", "Distance from left edge")
                .default("9.00")
                .depends_on("eval:doc.is_account_payable")
                .precision("2"),
            FieldSpec::data("message_to_show", "Message to show")
                .default("Acc. Payee")
                .depends_on("eval:doc.is_account_payable"),
            FieldSpec::section_break("date_and_payer_settings"),
            FieldSpec::html("date_settings", "Date Settings").options(
                "<label class=\"control-label\" style=\"margin-bottom: 0px;\">Date Settings</label>",
            ),
            FieldSpec::float("date_dist_from_top_edge", "Distance from top edge")
                .default("1.00")
                .precision("2"),
            FieldSpec::float("date_dist_from_left_edge", "Starting location from left edge")
                .default("15.00")
                .precision("2"),
            FieldSpec::column_break("payer_settings").label("Payer Settings"),
            FieldSpec::float("payer_name_from_top_edge", "Distance from top edge")
                .default("2.00")
                .precision("2"),
            FieldSpec::float("payer_name_from_left_edge", "Starting location from left edge")
                .default("3.00")
                .precision("2"),
            FieldSpec::section_break("amount_in_words_and_figure_settings"),
            FieldSpec::html("html_19", "").options(
                "<label class=\"control-label\" style=\"margin-bottom: 0px;\">Amount In Words</label>",
            ),
            FieldSpec::float("amt_in_words_from_top_edge", "Distance from top edge")
                .default("3.00")
                .precision("2"),
            FieldSpec::float("amt_in_words_from_left_edge", "Starting location from left edge")
                .default("4.00")
                .precision("2"),
            FieldSpec::float("amt_in_word_width", "Width of amount in word")
                .default("15.00")
                .precision("2"),
            FieldSpec::float("amt_in_words_line_spacing", "Line spacing for amount in words")
                .default("0.50")
                .precision("2"),
            FieldSpec::column_break("amount_in_figure").label("Amount In Figure"),
            FieldSpec::float("amt_in_figures_from_top_edge", "Distance from top edge")
                .default("3.50")
                .precision("2"),
            FieldSpec::float("amt_in_figures_from_left_edge", "Starting location from left edge")
                .default("16.00")
                .precision("2"),
            FieldSpec::section_break("account_number_and_signatory_settings"),
            FieldSpec::html("account_no_settings", "").options(
                "<label class=\"control-label\" style=\"margin-bottom: 0px;\">Account Number Settings</label>",
            ),
            FieldSpec::float("acc_no_dist_from_top_edge", "Distance from top edge")
                .default("5.00")
                .precision("2"),
            FieldSpec::float("acc_no_dist_from_left_edge", "Starting location from left edge")
                .default("4.00")
                .precision("2"),
            FieldSpec::column_break("signatory_position").label("Signatory Position"),
            FieldSpec::float("signatory_from_top_edge", "Distance from top edge")
                .default("6.00")
                .precision("2"),
            FieldSpec::float("signatory_from_left_edge", "Starting location from left edge")
                .default("15.00")
                .precision("2"),
            FieldSpec::section_break("preview").label("Preview"),
            FieldSpec::html("cheque_print_preview", ""),
        ]
    }
}

impl DocumentController for ChequePrintTemplate {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}

pub fn create_or_update_cheque_print_format_plan(
    doc: &ChequePrintTemplate,
    exists: bool,
) -> PrintFormatPlan {
    PrintFormatPlan {
        exists,
        doc_type: "Payment Entry".to_string(),
        standard: "No".to_string(),
        custom_format: true,
        print_format_type: "Jinja".to_string(),
        name: doc.bank_name.clone(),
        html: render_cheque_print_html(doc),
        set_has_print_format_for: doc.bank_name.clone(),
    }
}

fn render_cheque_print_html(doc: &ChequePrintTemplate) -> String {
    let starting_position_from_top_edge = if doc.cheque_size == "A4" {
        doc.starting_position_from_top_edge
    } else {
        0.0
    };
    let message_to_show = doc
        .message_to_show
        .as_deref()
        .filter(|message| !message.is_empty())
        .unwrap_or("Account Pay Only");

    format!(
        "\n<style>\n\t.print-format {{\n\t\tpadding: 0px;\n\t}}\n\t@media screen {{\n\t\t.print-format {{\n\t\t\tpadding: 0in;\n\t\t}}\n\t}}\n</style>\n<div style=\"position: relative; top:{}cm\">\n\t<div style=\"width:{}cm;height:{}cm;\">\n\t\t<span style=\"top:{}cm; left:{}cm;\n\t\t\tborder-bottom: solid 1px;border-top:solid 1px; width:2cm;text-align: center; position: absolute;\">\n\t\t\t\t{}\n\t\t</span>\n\t\t<span style=\"top:{}cm; left:{}cm;\n\t\t\tposition: absolute;\">\n\t\t\t{{{{ frappe.utils.formatdate(doc.reference_date) or '' }}}}\n\t\t</span>\n\t\t<span style=\"top:{}cm;left:{}cm;\n\t\t\tposition: absolute;  min-width: 6cm;\">\n\t\t\t{{{{ doc.account_no or '' }}}}\n\t\t</span>\n\t\t<span style=\"top:{}cm;left: {}cm;\n\t\t\tposition: absolute;  min-width: 6cm;\">\n\t\t\t{{{{doc.party_name}}}}\n\t\t</span>\n\t\t<span style=\"top:{}cm; left:{}cm;\n\t\t\tposition: absolute; display: block; width: {}cm;\n\t\t\tline-height:{}cm; word-wrap: break-word;\">\n\t\t\t\t{{{{frappe.utils.money_in_words(doc.base_paid_amount or doc.base_received_amount)}}}}\n\t\t</span>\n\t\t<span style=\"top:{}cm;left: {}cm;\n\t\t\tposition: absolute; min-width: 4cm;\">\n\t\t\t{{{{doc.get_formatted(\"base_paid_amount\") or doc.get_formatted(\"base_received_amount\")}}}}\n\t\t</span>\n\t\t<span style=\"top:{}cm;left: {}cm;\n\t\t\tposition: absolute;  min-width: 6cm;\">\n\t\t\t{{{{doc.company}}}}\n\t\t</span>\n\t</div>\n</div>",
        py_float(starting_position_from_top_edge),
        py_float(doc.cheque_width),
        py_float(doc.cheque_height),
        py_float(doc.acc_pay_dist_from_top_edge),
        py_float(doc.acc_pay_dist_from_left_edge),
        message_to_show,
        py_float(doc.date_dist_from_top_edge),
        py_float(doc.date_dist_from_left_edge),
        py_float(doc.acc_no_dist_from_top_edge),
        py_float(doc.acc_no_dist_from_left_edge),
        py_float(doc.payer_name_from_top_edge),
        py_float(doc.payer_name_from_left_edge),
        py_float(doc.amt_in_words_from_top_edge),
        py_float(doc.amt_in_words_from_left_edge),
        py_float(doc.amt_in_word_width),
        py_float(doc.amt_in_words_line_spacing),
        py_float(doc.amt_in_figures_from_top_edge),
        py_float(doc.amt_in_figures_from_left_edge),
        py_float(doc.signatory_from_top_edge),
        py_float(doc.signatory_from_left_edge),
    )
}

fn py_float(value: f64) -> String {
    let value = value.to_string();
    if value.contains('.') {
        value
    } else {
        format!("{value}.0")
    }
}
