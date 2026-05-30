use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinancialReportRow;

impl FinancialReportRow {
    pub const DOCTYPE: &'static str = "Financial Report Row";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const FIELD_ORDER: [&'static str; 21] = [
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
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("reference_code", "Line Reference")
                .description("Code to reference this line in formulas (e.g., REV100, EXP200, ASSET100)")
                .columns(1)
                .in_list_view(),
            FieldSpec::data("display_name", "Display Name")
                .description("Text displayed on the financial statement (e.g., 'Total Revenue', 'Cash and Cash Equivalents')")
                .in_list_view(),
            FieldSpec::int("indentation_level", "Indent Level")
                .description("Indentation level: 0 = Main heading, 1 = Sub-category, 2 = Individual accounts, etc.")
                .columns(1)
                .in_list_view(),
            FieldSpec::select("data_source", "Data Source")
                .options("\nAccount Data\nCalculated Amount\nCustom API\nBlank Line\nColumn Break\nSection Break")
                .description("How this line gets its data")
                .in_list_view(),
            FieldSpec::select("balance_type", "Balance Type")
                .options("\nOpening Balance\nClosing Balance\nPeriod Movement (Debits - Credits)")
                .depends_on("eval:doc.data_source == 'Account Data'")
                .mandatory_depends_on("eval:doc.data_source == 'Account Data'")
                .description("Opening Balance = Start of period, Closing Balance = End of period, Period Movement = Net change during period")
                .in_list_view(),
            FieldSpec::column_break("column_break_hxqu"),
            FieldSpec::check("bold_text", "Bold Text")
                .default("0")
                .description("Bold text for emphasis (totals, major headings)"),
            FieldSpec::check("italic_text", "Italic Text")
                .default("0")
                .description("Italic text for subtotals or notes"),
            FieldSpec::check("hidden_calculation", "Hidden Line (Internal Use Only)")
                .default("0")
                .description("Calculate but don't show on final report"),
            FieldSpec::check("hide_when_empty", "Hide If Zero")
                .default("0")
                .description("Hide this line if amount is zero"),
            FieldSpec::check("reverse_sign", "Reverse Sign")
                .default("0")
                .description("Show negative values as positive (for expenses in P&L)")
                .columns(1)
                .in_list_view(),
            FieldSpec::section_break("section_break_ornw"),
            FieldSpec::code("calculation_formula", "Formula or Account Filter")
                .depends_on("eval: (doc.data_source === \"Account Data\" && doc.advanced_filtering) || [\"Calculated Amount\", \"Custom API\"].includes(doc.data_source);\n")
                .mandatory_depends_on("eval:doc.data_source != 'Blank Line' && doc.data_source != 'Column Break' && doc.data_source != 'Section Break'"),
            FieldSpec::html("formula_description", ""),
            FieldSpec::check("include_in_charts", "Include in Charts")
                .default("0")
                .description("If enabled, this row's values will be displayed on financial charts"),
            FieldSpec::color("color", "Color")
                .description("Color to highlight values (e.g., red for exceptions)"),
            FieldSpec::select("fieldtype", "Value Type")
                .options("\nCurrency\nFloat\nInt\nPercent")
                .description("How to format and present values in the financial report (only if different from column fieldtype)"),
            FieldSpec::html("filters_editor", "")
                .depends_on("eval: doc.data_source === \"Account Data\" && !doc.advanced_filtering"),
            FieldSpec::column_break("column_break_asfe")
                .depends_on("eval: ![\"Blank Line\", \"Column Break\", \"Section Break\"].includes(doc.data_source);"),
            FieldSpec::check("advanced_filtering", "Advanced Filtering")
                .default("0")
                .depends_on("eval: doc.data_source === \"Account Data\"")
                .description("Use <strong>Python</strong> filters to get Accounts")
                .print_hide(),
            FieldSpec::section_break("section_break_pvro"),
        ]
    }
}

impl DocumentController for FinancialReportRow {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
