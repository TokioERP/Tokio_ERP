pub mod purchase_invoice_taxes;
pub mod sales_invoice_taxes;

pub const MODULE: &str = "Accounts";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrintFormatFieldTemplate {
    pub folder: &'static str,
    pub name: &'static str,
    pub document_type: &'static str,
    pub field: &'static str,
    pub template_file: &'static str,
    pub standard: bool,
}

pub const PRINT_FORMAT_FIELD_TEMPLATES: [PrintFormatFieldTemplate; 2] = [
    PrintFormatFieldTemplate {
        folder: purchase_invoice_taxes::FOLDER_NAME,
        name: purchase_invoice_taxes::TEMPLATE_NAME,
        document_type: "Purchase Invoice",
        field: "taxes",
        template_file: "templates/print_formats/includes/taxes_and_charges.html",
        standard: true,
    },
    PrintFormatFieldTemplate {
        folder: sales_invoice_taxes::FOLDER_NAME,
        name: sales_invoice_taxes::TEMPLATE_NAME,
        document_type: "Sales Invoice",
        field: "taxes",
        template_file: "templates/print_formats/includes/taxes_and_charges.html",
        standard: true,
    },
];
