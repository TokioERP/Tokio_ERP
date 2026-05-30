use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct JournalEntryAccount;

impl JournalEntryAccount {
    pub const DOCTYPE: &'static str = "Journal Entry Account";
    pub const MODULE: &'static str = "Accounts";
    pub const IS_TABLE: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const FIELD_ORDER: [&'static str; 32] = [
        "account",
        "account_type",
        "col_break1",
        "bank_account",
        "party_type",
        "party",
        "accounting_dimensions_section",
        "cost_center",
        "dimension_col_break",
        "project",
        "currency_section",
        "account_currency",
        "column_break_10",
        "exchange_rate",
        "sec_break1",
        "debit_in_account_currency",
        "debit",
        "col_break2",
        "credit_in_account_currency",
        "credit",
        "reference",
        "reference_type",
        "reference_name",
        "reference_due_date",
        "reference_detail_no",
        "advance_voucher_type",
        "advance_voucher_no",
        "is_tax_withholding_account",
        "col_break3",
        "is_advance",
        "user_remark",
        "against_account",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .oldfield("account", "Link")
                .columns(4)
                .width("250px")
                .required()
                .bold()
                .in_global_search()
                .in_list_view()
                .search_index(),
            FieldSpec::data("account_type", "Account Type")
                .hidden()
                .print_hide(),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .oldfield("cost_center", "Link")
                .default(":Company")
                .description("If Income or Expense")
                .width("180px")
                .allow_on_submit()
                .print_hide(),
            FieldSpec::column_break("col_break1"),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .in_list_view()
                .search_index(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .columns(2)
                .in_list_view(),
            FieldSpec::section_break("currency_section").label("Currency"),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .read_only()
                .print_hide(),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::float("exchange_rate", "Exchange Rate")
                .precision("9")
                .print_hide(),
            FieldSpec::section_break("sec_break1").label("Amount"),
            FieldSpec::currency("debit_in_account_currency", "Debit")
                .options("account_currency")
                .columns(2)
                .bold()
                .print_hide_if_no_value()
                .in_list_view(),
            FieldSpec::currency("debit", "Debit in Company Currency")
                .options("Company:company:default_currency")
                .oldfield("debit", "Currency")
                .bold()
                .read_only()
                .no_copy()
                .print_hide(),
            FieldSpec::column_break("col_break2"),
            FieldSpec::currency("credit_in_account_currency", "Credit")
                .options("account_currency")
                .columns(2)
                .bold()
                .print_hide_if_no_value()
                .in_list_view(),
            FieldSpec::currency("credit", "Credit in Company Currency")
                .options("Company:company:default_currency")
                .oldfield("credit", "Currency")
                .bold()
                .read_only()
                .no_copy()
                .print_hide(),
            FieldSpec::section_break("reference").label("Reference"),
            FieldSpec::select("reference_type", "Reference Type")
                .options("\nSales Invoice\nPurchase Invoice\nJournal Entry\nSales Order\nPurchase Order\nExpense Claim\nAsset\nLoan\nPayroll Entry\nEmployee Advance\nExchange Rate Revaluation\nInvoice Discounting\nFees\nFull and Final Statement\nPayment Entry\nBank Transaction")
                .no_copy()
                .search_index(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .no_copy()
                .search_index(),
            FieldSpec::date("reference_due_date", "Reference Due Date")
                .depends_on("eval:doc.reference_type&&!in_list(doc.reference_type, ['Expense Claim', 'Asset', 'Employee Loan', 'Employee Advance', 'Bank Transaction'])")
                .no_copy(),
            FieldSpec::link("project", "Project")
                .options("Project")
                .allow_on_submit(),
            FieldSpec::column_break("col_break3"),
            FieldSpec::select("is_advance", "Is Advance")
                .options("No\nYes")
                .oldfield("is_advance", "Select")
                .no_copy()
                .print_hide(),
            FieldSpec::small_text("user_remark", "User Remark")
                .no_copy()
                .print_hide(),
            FieldSpec::text("against_account", "Against Account")
                .oldfield("against_account", "Text")
                .hidden()
                .no_copy()
                .print_hide(),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions")
                .collapsible(),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::link("bank_account", "Bank Account").options("Bank Account"),
            FieldSpec::data("reference_detail_no", "Reference Detail No")
                .hidden()
                .no_copy(),
            FieldSpec::link("advance_voucher_type", "Advance Voucher Type")
                .options("DocType")
                .read_only()
                .no_copy()
                .search_index(),
            FieldSpec::dynamic_link("advance_voucher_no")
                .label("Advance Voucher No")
                .options("advance_voucher_type")
                .read_only()
                .no_copy()
                .search_index(),
            FieldSpec::check("is_tax_withholding_account", "Is Tax Withholding Account")
                .default("0")
                .read_only(),
        ]
    }
}

impl DocumentController for JournalEntryAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }
}
