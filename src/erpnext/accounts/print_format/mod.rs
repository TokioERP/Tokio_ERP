pub mod accounts_payable_standard;
pub mod accounts_payable_summary_standard;
pub mod accounts_receivable_standard;
pub mod accounts_receivable_summary_standard;
pub mod balance_sheet_standard;
pub mod bank_and_cash_payment_voucher;
pub mod cash_flow_statement_standard;
pub mod cheque_printing_format;
pub mod credit_note;
pub mod dunning_letter;
pub mod general_ledger_standard;
pub mod journal_auditing_voucher;
#[path = "p&l_statement_standard/mod.rs"]
pub mod p_l_statement_standard;
pub mod payment_receipt_voucher;
pub mod pos_invoice;
pub mod pos_invoice_standard;
pub mod pos_invoice_with_item_image;
pub mod purchase_auditing_voucher;
pub mod purchase_invoice_standard;
pub mod purchase_invoice_with_item_image;
pub mod sales_auditing_voucher;
pub mod sales_invoice_return;
pub mod sales_invoice_standard;
pub mod sales_invoice_with_item_image;
pub mod trial_balance_standard;

pub const MODULE: &str = "Accounts";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrintFormat {
    pub folder: &'static str,
    pub name: &'static str,
    pub print_format_for: &'static str,
    pub print_format_type: &'static str,
    pub report: &'static str,
    pub doc_type: &'static str,
    pub standard: &'static str,
}

impl PrintFormat {
    pub const fn report(folder: &'static str, name: &'static str, report: &'static str) -> Self {
        Self {
            folder,
            name,
            print_format_for: "Report",
            print_format_type: "JS",
            report,
            doc_type: "",
            standard: "Yes",
        }
    }

    pub const fn doctype(folder: &'static str, name: &'static str, doc_type: &'static str) -> Self {
        Self {
            folder,
            name,
            print_format_for: "",
            print_format_type: "Jinja",
            report: "",
            doc_type,
            standard: "Yes",
        }
    }

    pub const fn doctype_format(
        folder: &'static str,
        name: &'static str,
        doc_type: &'static str,
    ) -> Self {
        Self {
            folder,
            name,
            print_format_for: "DocType",
            print_format_type: "Jinja",
            report: "",
            doc_type,
            standard: "Yes",
        }
    }
}

pub const PRINT_FORMATS: [PrintFormat; 25] = [
    PrintFormat::report(
        accounts_payable_standard::FOLDER_NAME,
        accounts_payable_standard::FORMAT_NAME,
        "Accounts Payable",
    ),
    PrintFormat::report(
        accounts_payable_summary_standard::FOLDER_NAME,
        accounts_payable_summary_standard::FORMAT_NAME,
        "Accounts Payable Summary",
    ),
    PrintFormat::report(
        accounts_receivable_standard::FOLDER_NAME,
        accounts_receivable_standard::FORMAT_NAME,
        "Accounts Receivable",
    ),
    PrintFormat::report(
        accounts_receivable_summary_standard::FOLDER_NAME,
        accounts_receivable_summary_standard::FORMAT_NAME,
        "Accounts Receivable Summary",
    ),
    PrintFormat::report(
        balance_sheet_standard::FOLDER_NAME,
        balance_sheet_standard::FORMAT_NAME,
        "Balance Sheet",
    ),
    PrintFormat::doctype(
        bank_and_cash_payment_voucher::FOLDER_NAME,
        bank_and_cash_payment_voucher::FORMAT_NAME,
        "Payment Entry",
    ),
    PrintFormat::report(
        cash_flow_statement_standard::FOLDER_NAME,
        cash_flow_statement_standard::FORMAT_NAME,
        "Cash Flow",
    ),
    PrintFormat::doctype(
        cheque_printing_format::FOLDER_NAME,
        cheque_printing_format::FORMAT_NAME,
        "Journal Entry",
    ),
    PrintFormat::doctype(
        credit_note::FOLDER_NAME,
        credit_note::FORMAT_NAME,
        "Journal Entry",
    ),
    PrintFormat::doctype(
        dunning_letter::FOLDER_NAME,
        dunning_letter::FORMAT_NAME,
        "Dunning",
    ),
    PrintFormat::report(
        general_ledger_standard::FOLDER_NAME,
        general_ledger_standard::FORMAT_NAME,
        "General Ledger",
    ),
    PrintFormat::doctype(
        journal_auditing_voucher::FOLDER_NAME,
        journal_auditing_voucher::FORMAT_NAME,
        "Journal Entry",
    ),
    PrintFormat::report(
        p_l_statement_standard::FOLDER_NAME,
        p_l_statement_standard::FORMAT_NAME,
        "Profit and Loss Statement",
    ),
    PrintFormat::doctype(
        payment_receipt_voucher::FOLDER_NAME,
        payment_receipt_voucher::FORMAT_NAME,
        "Journal Entry",
    ),
    PrintFormat::doctype(
        pos_invoice::FOLDER_NAME,
        pos_invoice::FORMAT_NAME,
        "Sales Invoice",
    ),
    PrintFormat::doctype_format(
        pos_invoice_standard::FOLDER_NAME,
        pos_invoice_standard::FORMAT_NAME,
        "POS Invoice",
    ),
    PrintFormat::doctype_format(
        pos_invoice_with_item_image::FOLDER_NAME,
        pos_invoice_with_item_image::FORMAT_NAME,
        "POS Invoice",
    ),
    PrintFormat::doctype(
        purchase_auditing_voucher::FOLDER_NAME,
        purchase_auditing_voucher::FORMAT_NAME,
        "Purchase Invoice",
    ),
    PrintFormat::doctype_format(
        purchase_invoice_standard::FOLDER_NAME,
        purchase_invoice_standard::FORMAT_NAME,
        "Purchase Invoice",
    ),
    PrintFormat::doctype_format(
        purchase_invoice_with_item_image::FOLDER_NAME,
        purchase_invoice_with_item_image::FORMAT_NAME,
        "Purchase Invoice",
    ),
    PrintFormat::doctype(
        sales_auditing_voucher::FOLDER_NAME,
        sales_auditing_voucher::FORMAT_NAME,
        "Sales Invoice",
    ),
    PrintFormat::doctype(
        sales_invoice_return::FOLDER_NAME,
        sales_invoice_return::FORMAT_NAME,
        "Sales Invoice",
    ),
    PrintFormat::doctype_format(
        sales_invoice_standard::FOLDER_NAME,
        sales_invoice_standard::FORMAT_NAME,
        "Sales Invoice",
    ),
    PrintFormat::doctype_format(
        sales_invoice_with_item_image::FOLDER_NAME,
        sales_invoice_with_item_image::FORMAT_NAME,
        "Sales Invoice",
    ),
    PrintFormat::report(
        trial_balance_standard::FOLDER_NAME,
        trial_balance_standard::FORMAT_NAME,
        "Trial Balance",
    ),
];
