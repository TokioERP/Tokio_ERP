use crate::erpnext::accounts::doctype::financial_report_row::financial_report_row::FinancialReportRow;
use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FinancialReportTemplate {
    pub name: Option<String>,
    pub template_name: String,
    pub report_type: Option<String>,
    pub module: Option<String>,
    pub disabled: bool,
    pub rows: Vec<FinancialReportRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialReportTemplateExportPlan {
    pub template_doctype: &'static str,
    pub module: String,
    pub export_account_categories: bool,
}

impl FinancialReportTemplate {
    pub const DOCTYPE: &'static str = "Financial Report Template";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 7] = [
        "template_name",
        "report_type",
        "module",
        "column_break_lvnq",
        "disabled",
        "section_break_fvlw",
        "rows",
    ];
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::data("template_name", "Template Name")
                .description(
                    "Descriptive name for your template (e.g., 'Standard P&L', 'Detailed Balance Sheet')",
                )
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
    }

    pub fn before_validate(&mut self) {
        self.clear_hidden_fields();
    }

    pub fn clear_hidden_fields(&mut self) {
        for row in &mut self.rows {
            if row.data_source.as_deref() != Some("Account Data") {
                row.balance_type = None;
            }

            if matches!(
                row.data_source.as_deref(),
                Some("Blank Line" | "Column Break" | "Section Break")
            ) {
                row.calculation_formula = None;
            }
        }
    }

    pub fn export_template_plan(
        &self,
        in_import: bool,
        developer_mode: bool,
    ) -> Option<FinancialReportTemplateExportPlan> {
        let module = self.module.clone()?;
        Some(FinancialReportTemplateExportPlan {
            template_doctype: Self::DOCTYPE,
            module,
            export_account_categories: developer_mode && !in_import,
        })
    }

    pub fn delete_template_dir_plan(&self, developer_mode: bool) -> Option<String> {
        if !developer_mode {
            return None;
        }
        let module = self.module.as_deref()?;
        let name = self.name.as_deref().unwrap_or(&self.template_name);
        Some(format!(
            "{module}/financial_report_template/{}",
            scrub(name)
        ))
    }
}

impl DocumentController for FinancialReportTemplate {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["before_validate", "validate", "on_update", "on_trash"]
    }
}

fn scrub(value: &str) -> String {
    value.trim().to_lowercase().replace(' ', "_")
}
