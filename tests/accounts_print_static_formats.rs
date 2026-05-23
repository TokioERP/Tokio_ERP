use tokio_erp::erpnext::accounts::print_format::{PrintFormat, PRINT_FORMATS};
use tokio_erp::erpnext::accounts::print_format_field_template::{
    PrintFormatFieldTemplate, PRINT_FORMAT_FIELD_TEMPLATES,
};

#[test]
fn print_format_registry_matches_erpnext_static_formats() {
    assert_eq!(PRINT_FORMATS.len(), 25);
    assert_eq!(
        PRINT_FORMATS,
        [
            PrintFormat::report(
                "accounts_payable_standard",
                "Accounts Payable Standard",
                "Accounts Payable"
            ),
            PrintFormat::report(
                "accounts_payable_summary_standard",
                "Accounts Payable Summary Standard",
                "Accounts Payable Summary",
            ),
            PrintFormat::report(
                "accounts_receivable_standard",
                "Accounts Receivable Standard",
                "Accounts Receivable",
            ),
            PrintFormat::report(
                "accounts_receivable_summary_standard",
                "Accounts Receivable Summary Standard",
                "Accounts Receivable Summary",
            ),
            PrintFormat::report(
                "balance_sheet_standard",
                "Balance Sheet Standard",
                "Balance Sheet"
            ),
            PrintFormat::doctype(
                "bank_and_cash_payment_voucher",
                "Bank and Cash Payment Voucher",
                "Payment Entry",
            ),
            PrintFormat::report(
                "cash_flow_statement_standard",
                "Cash Flow Statement Standard",
                "Cash Flow"
            ),
            PrintFormat::doctype(
                "cheque_printing_format",
                "Cheque Printing Format",
                "Journal Entry"
            ),
            PrintFormat::doctype("credit_note", "Credit Note", "Journal Entry"),
            PrintFormat::doctype("dunning_letter", "Dunning Letter", "Dunning"),
            PrintFormat::report(
                "general_ledger_standard",
                "General Ledger Standard",
                "General Ledger"
            ),
            PrintFormat::doctype(
                "journal_auditing_voucher",
                "Journal Auditing Voucher",
                "Journal Entry",
            ),
            PrintFormat::report(
                "p&l_statement_standard",
                "P&L Statement Standard",
                "Profit and Loss Statement"
            ),
            PrintFormat::doctype(
                "payment_receipt_voucher",
                "Payment Receipt Voucher",
                "Journal Entry"
            ),
            PrintFormat::doctype("pos_invoice", "POS Invoice", "Sales Invoice"),
            PrintFormat::doctype_format(
                "pos_invoice_standard",
                "POS Invoice Standard",
                "POS Invoice"
            ),
            PrintFormat::doctype_format(
                "pos_invoice_with_item_image",
                "POS Invoice with Item Image",
                "POS Invoice",
            ),
            PrintFormat::doctype(
                "purchase_auditing_voucher",
                "Purchase Auditing Voucher",
                "Purchase Invoice",
            ),
            PrintFormat::doctype_format(
                "purchase_invoice_standard",
                "Purchase Invoice Standard",
                "Purchase Invoice",
            ),
            PrintFormat::doctype_format(
                "purchase_invoice_with_item_image",
                "Purchase Invoice with Item Image",
                "Purchase Invoice",
            ),
            PrintFormat::doctype(
                "sales_auditing_voucher",
                "Sales Auditing Voucher",
                "Sales Invoice"
            ),
            PrintFormat::doctype(
                "sales_invoice_return",
                "Sales Invoice Return",
                "Sales Invoice"
            ),
            PrintFormat::doctype_format(
                "sales_invoice_standard",
                "Sales Invoice Standard",
                "Sales Invoice"
            ),
            PrintFormat::doctype_format(
                "sales_invoice_with_item_image",
                "Sales Invoice with Item Image",
                "Sales Invoice",
            ),
            PrintFormat::report(
                "trial_balance_standard",
                "Trial Balance Standard",
                "Trial Balance"
            ),
        ]
    );
}

#[test]
fn print_format_init_modules_are_noop_markers() {
    assert_eq!(
        tokio_erp::erpnext::accounts::print_format::MODULE,
        "Accounts"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::print_format::accounts_payable_standard::FORMAT_NAME,
        "Accounts Payable Standard"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::print_format::p_l_statement_standard::FOLDER_NAME,
        "p&l_statement_standard"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::print_format::sales_invoice_with_item_image::FORMAT_NAME,
        "Sales Invoice with Item Image"
    );
}

#[test]
fn print_format_field_template_registry_matches_erpnext_static_templates() {
    assert_eq!(
        PRINT_FORMAT_FIELD_TEMPLATES,
        [
            PrintFormatFieldTemplate {
                folder: "purchase_invoice_taxes",
                name: "Purchase Invoice Taxes",
                document_type: "Purchase Invoice",
                field: "taxes",
                template_file: "templates/print_formats/includes/taxes_and_charges.html",
                standard: true,
            },
            PrintFormatFieldTemplate {
                folder: "sales_invoice_taxes",
                name: "Sales Invoice Taxes",
                document_type: "Sales Invoice",
                field: "taxes",
                template_file: "templates/print_formats/includes/taxes_and_charges.html",
                standard: true,
            },
        ]
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::print_format_field_template::purchase_invoice_taxes::TEMPLATE_NAME,
        "Purchase Invoice Taxes"
    );
    assert_eq!(
        tokio_erp::erpnext::accounts::print_format_field_template::sales_invoice_taxes::TEMPLATE_NAME,
        "Sales Invoice Taxes"
    );
}
