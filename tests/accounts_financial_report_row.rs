use tokio_erp::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn financial_report_row_matches_erpnext_pass_controller_metadata() {
    assert_eq!(FinancialReportRow::DOCTYPE, "Financial Report Row");
    assert_eq!(FinancialReportRow::MODULE, "Accounts");
    assert!(FinancialReportRow::IS_TABLE);
    assert_eq!(
        FinancialReportRow::FIELD_ORDER,
        [
            "reference_code",
            "display_name",
            "indentation_level",
            "data_source",
            "balance_type",
            "column_break_hxqu",
            "fieldtype",
            "color",
            "bold_text",
            "italic_text",
            "hidden_calculation",
            "hide_when_empty",
            "reverse_sign",
            "include_in_charts",
            "section_break_ornw",
            "column_break_asfe",
            "advanced_filtering",
            "filters_editor",
            "calculation_formula",
            "section_break_pvro",
            "formula_description",
        ]
    );

    let fields = FinancialReportRow::fields();
    assert_eq!(fields.len(), 21);
    assert_eq!(
        fields[0],
        FieldSpec::data("reference_code", "Line Reference")
            .description("Code to reference this line in formulas (e.g., REV100, EXP200, ASSET100)")
            .columns(1)
            .in_list_view()
    );
    assert_eq!(
        fields[4],
        FieldSpec::select("balance_type", "Balance Type")
            .options("\nOpening Balance\nClosing Balance\nPeriod Movement (Debits - Credits)")
            .depends_on("eval:doc.data_source == 'Account Data'")
            .mandatory_depends_on("eval:doc.data_source == 'Account Data'")
            .description("Opening Balance = Start of period, Closing Balance = End of period, Period Movement = Net change during period")
            .in_list_view()
    );
    assert_eq!(
        fields[12],
        FieldSpec::code("calculation_formula", "Formula or Account Filter")
            .depends_on("eval: (doc.data_source === \"Account Data\" && doc.advanced_filtering) || [\"Calculated Amount\", \"Custom API\"].includes(doc.data_source);\n")
            .mandatory_depends_on("eval:doc.data_source != 'Blank Line' && doc.data_source != 'Column Break' && doc.data_source != 'Section Break'")
    );
    assert_eq!(
        fields[15],
        FieldSpec::color("color", "Color")
            .description("Color to highlight values (e.g., red for exceptions)")
    );
    assert_eq!(
        fields[19],
        FieldSpec::check("advanced_filtering", "Advanced Filtering")
            .default("0")
            .depends_on("eval: doc.data_source === \"Account Data\"")
            .description("Use <strong>Python</strong> filters to get Accounts")
            .print_hide()
    );

    let row = FinancialReportRow;
    assert_eq!(row.doctype(), "Financial Report Row");
    assert!(row.custom_hooks().is_empty());
}
