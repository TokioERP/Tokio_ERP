use tokio_erp::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use tokio_erp::erpnext::accounts::doctype::financial_report_template::financial_report_template::{
    FinancialReportTemplate, FinancialReportTemplateExportPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn financial_report_template_matches_erpnext_metadata() {
    assert_eq!(
        FinancialReportTemplate::DOCTYPE,
        "Financial Report Template"
    );
    assert_eq!(FinancialReportTemplate::MODULE, "Accounts");
    assert_eq!(
        FinancialReportTemplate::FIELD_ORDER,
        [
            "template_name",
            "report_type",
            "module",
            "column_break_lvnq",
            "disabled",
            "section_break_fvlw",
            "rows",
        ]
    );
    assert_eq!(FinancialReportTemplate::SORT_FIELD, "creation");
    assert_eq!(FinancialReportTemplate::SORT_ORDER, "DESC");

    assert_eq!(
        FinancialReportTemplate::fields(),
        vec![
            FieldSpec::data("template_name", "Template Name")
                .description("Descriptive name for your template (e.g., 'Standard P&L', 'Detailed Balance Sheet')")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::select("report_type", "Report Type")
                .options("\nProfit and Loss Statement\nBalance Sheet\nCash Flow\nCustom Financial Statement")
                .description("Type of financial statement this template generates")
                .in_list_view()
                .in_standard_filter()
                .required(),
            FieldSpec::link("module", "Module (for Export)")
                .options("Module Def")
                .depends_on("eval:frappe.boot.developer_mode"),
            FieldSpec::column_break("column_break_lvnq"),
            FieldSpec::check("disabled", "Disabled")
                .default("0")
                .description("Disable template to prevent use in reports"),
            FieldSpec::section_break("section_break_fvlw"),
            FieldSpec::table("rows", "Report Line Items")
                .options("Financial Report Row")
                .allow_bulk_edit(),
        ]
    );
}

#[test]
fn financial_report_template_clear_hidden_fields_matches_erpnext() {
    let mut template = FinancialReportTemplate {
        template_name: "P&L".to_string(),
        rows: vec![
            FinancialReportRow {
                data_source: Some("Account Data".to_string()),
                balance_type: Some("Closing Balance".to_string()),
                calculation_formula: Some("account_category == 'Revenue'".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                data_source: Some("Blank Line".to_string()),
                balance_type: Some("Opening Balance".to_string()),
                calculation_formula: Some("REV100 + REV200".to_string()),
                ..Default::default()
            },
            FinancialReportRow {
                data_source: Some("Calculated Amount".to_string()),
                balance_type: Some("Period Movement (Debits - Credits)".to_string()),
                calculation_formula: Some("REV100 - EXP100".to_string()),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    assert_eq!(
        template.custom_hooks(),
        ["before_validate", "validate", "on_update", "on_trash"]
    );
    template.before_validate();

    assert_eq!(
        template.rows[0].balance_type.as_deref(),
        Some("Closing Balance")
    );
    assert_eq!(
        template.rows[0].calculation_formula.as_deref(),
        Some("account_category == 'Revenue'")
    );
    assert_eq!(template.rows[1].balance_type, None);
    assert_eq!(template.rows[1].calculation_formula, None);
    assert_eq!(template.rows[2].balance_type, None);
    assert_eq!(
        template.rows[2].calculation_formula.as_deref(),
        Some("REV100 - EXP100")
    );
    assert_eq!(template.doctype(), "Financial Report Template");
    assert_eq!(template.module(), "Accounts");
}

#[test]
fn financial_report_template_export_and_delete_plans_match_erpnext_guards() {
    let template = FinancialReportTemplate {
        name: Some("Standard P&L".to_string()),
        template_name: "Standard P&L".to_string(),
        module: Some("Accounts".to_string()),
        ..Default::default()
    };

    assert_eq!(
        template.export_template_plan(false, false),
        Some(FinancialReportTemplateExportPlan {
            template_doctype: "Financial Report Template",
            module: "Accounts".to_string(),
            export_account_categories: false,
        })
    );
    assert_eq!(
        template
            .export_template_plan(false, true)
            .unwrap()
            .export_account_categories,
        true
    );
    assert_eq!(
        template
            .export_template_plan(true, false)
            .unwrap()
            .export_account_categories,
        false
    );
    assert_eq!(
        template.delete_template_dir_plan(true),
        Some("Accounts/financial_report_template/standard_p&l".to_string())
    );

    let no_module = FinancialReportTemplate::default();
    assert_eq!(no_module.export_template_plan(false, false), None);
    assert_eq!(template.delete_template_dir_plan(false), None);
}

#[test]
fn financial_report_template_test_fixture_rows_match_erpnext_testcase() {
    let template = FinancialReportTemplate::test_profit_and_loss_template();

    assert_eq!(template.template_name, "Test P&L Template");
    assert_eq!(
        template.report_type.as_deref(),
        Some("Profit and Loss Statement")
    );
    assert_eq!(template.rows.len(), 3);
    assert_eq!(template.rows[0].reference_code.as_deref(), Some("INC001"));
    assert_eq!(template.rows[0].display_name.as_deref(), Some("Income"));
    assert_eq!(template.rows[0].indentation_level, 0);
    assert_eq!(
        template.rows[0].data_source.as_deref(),
        Some("Account Data")
    );
    assert_eq!(
        template.rows[0].balance_type.as_deref(),
        Some("Closing Balance")
    );
    assert_eq!(template.rows[0].bold_text, 1);
    assert_eq!(
        template.rows[0].calculation_formula.as_deref(),
        Some("[\"root_type\", \"=\", \"Income\"]")
    );
    assert_eq!(template.rows[1].reference_code.as_deref(), Some("EXP001"));
    assert_eq!(template.rows[2].reference_code.as_deref(), Some("NET001"));
    assert_eq!(
        template.rows[2].calculation_formula.as_deref(),
        Some("INC001 - EXP001")
    );
}
