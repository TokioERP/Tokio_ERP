use tokio_erp::erpnext::accounts::doctype::cheque_print_template::cheque_print_template::{
    create_or_update_cheque_print_format_plan, ChequePrintTemplate, PrintFormatPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn cheque_print_template_matches_erpnext_metadata() {
    assert_eq!(ChequePrintTemplate::DOCTYPE, "Cheque Print Template");
    assert_eq!(ChequePrintTemplate::MODULE, "Accounts");
    assert_eq!(ChequePrintTemplate::AUTONAME, "field:bank_name");
    assert_eq!(ChequePrintTemplate::MAX_ATTACHMENTS, 1);
    assert_eq!(ChequePrintTemplate::SORT_FIELD, "creation");
    assert_eq!(ChequePrintTemplate::SORT_ORDER, "DESC");
    assert_eq!(ChequePrintTemplate::FIELD_ORDER.len(), 39);
    assert_eq!(
        &ChequePrintTemplate::FIELD_ORDER[..8],
        [
            "settings",
            "has_print_format",
            "primary_settings",
            "bank_name",
            "cheque_size",
            "starting_position_from_top_edge",
            "cheque_width",
            "cheque_height",
        ]
    );

    let fields = ChequePrintTemplate::fields();
    assert_eq!(fields.len(), 39);
    assert_eq!(
        fields[0],
        FieldSpec::html("settings", "")
            .options("<div>\n<h3> All dimensions in centimeter only </h3>\n</div>")
    );
    assert_eq!(
        fields[1],
        FieldSpec::check("has_print_format", "Has Print Format")
            .default("0")
            .hidden()
            .no_copy()
            .print_hide()
            .read_only()
    );
    assert_eq!(
        fields[3],
        FieldSpec::data("bank_name", "Bank Name")
            .in_list_view()
            .no_copy()
            .required()
            .unique()
    );
    assert_eq!(
        fields[5],
        FieldSpec::float(
            "starting_position_from_top_edge",
            "Starting position from top edge"
        )
        .depends_on("eval:doc.cheque_size==\"A4\"")
        .precision("2")
    );
}

#[test]
fn cheque_print_template_generates_print_format_like_erpnext() {
    let template = ChequePrintTemplate {
        bank_name: "Main Bank".to_string(),
        cheque_size: "A4".to_string(),
        starting_position_from_top_edge: 1.25,
        cheque_width: 20.0,
        cheque_height: 9.0,
        acc_pay_dist_from_top_edge: 1.0,
        acc_pay_dist_from_left_edge: 9.0,
        message_to_show: None,
        date_dist_from_top_edge: 1.0,
        date_dist_from_left_edge: 15.0,
        acc_no_dist_from_top_edge: 5.0,
        acc_no_dist_from_left_edge: 4.0,
        payer_name_from_top_edge: 2.0,
        payer_name_from_left_edge: 3.0,
        amt_in_words_from_top_edge: 3.0,
        amt_in_words_from_left_edge: 4.0,
        amt_in_word_width: 15.0,
        amt_in_words_line_spacing: 0.5,
        amt_in_figures_from_top_edge: 3.5,
        amt_in_figures_from_left_edge: 16.0,
        signatory_from_top_edge: 6.0,
        signatory_from_left_edge: 15.0,
        ..Default::default()
    };

    assert_eq!(template.doctype(), "Cheque Print Template");
    assert_eq!(template.module(), "Accounts");

    let plan = create_or_update_cheque_print_format_plan(&template, false);
    assert_eq!(plan.doc_type, "Payment Entry");
    assert_eq!(plan.standard, "No");
    assert!(plan.custom_format);
    assert_eq!(plan.print_format_type, "Jinja");
    assert_eq!(plan.name, "Main Bank");
    assert_eq!(plan.set_has_print_format_for, "Main Bank");
    assert!(plan.html.contains("top:1.25cm"));
    assert!(plan.html.contains("Account Pay Only"));
    assert!(plan
        .html
        .contains("{{ frappe.utils.formatdate(doc.reference_date) or '' }}"));
    assert!(plan.html.contains("{{doc.party_name}}"));

    let regular = ChequePrintTemplate {
        cheque_size: "Regular".to_string(),
        message_to_show: Some("Acc. Payee".to_string()),
        ..template
    };
    assert_eq!(
        create_or_update_cheque_print_format_plan(&regular, true),
        PrintFormatPlan {
            exists: true,
            doc_type: "Payment Entry".to_string(),
            standard: "No".to_string(),
            custom_format: true,
            print_format_type: "Jinja".to_string(),
            name: "Main Bank".to_string(),
            html: create_or_update_cheque_print_format_plan(&regular, true).html,
            set_has_print_format_for: "Main Bank".to_string(),
        }
    );
    assert!(create_or_update_cheque_print_format_plan(&regular, true)
        .html
        .contains("top:0.0cm"));
}
