use tokio_erp::erpnext::accounts::doctype::journal_entry_account::journal_entry_account::JournalEntryAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn journal_entry_account_matches_erpnext_pass_controller_metadata() {
    assert_eq!(JournalEntryAccount::DOCTYPE, "Journal Entry Account");
    assert_eq!(JournalEntryAccount::MODULE, "Accounts");
    assert!(JournalEntryAccount::IS_TABLE);
    assert!(JournalEntryAccount::TRACK_CHANGES);
    assert_eq!(JournalEntryAccount::AUTONAME, "hash");
    assert_eq!(JournalEntryAccount::NAMING_RULE, "Random");
    assert_eq!(JournalEntryAccount::ROW_FORMAT, "Dynamic");
    assert_eq!(JournalEntryAccount::SORT_FIELD, "creation");
    assert_eq!(JournalEntryAccount::SORT_ORDER, "DESC");
    assert_eq!(
        JournalEntryAccount::FIELD_ORDER,
        [
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
        ]
    );

    let fields = JournalEntryAccount::fields();
    assert_eq!(fields.len(), 32);
    assert_eq!(
        fields[0],
        FieldSpec::link("account", "Account")
            .options("Account")
            .oldfield("account", "Link")
            .columns(4)
            .width("250px")
            .required()
            .bold()
            .in_global_search()
            .in_list_view()
            .search_index()
    );
    assert_eq!(
        fields[11],
        FieldSpec::currency("debit_in_account_currency", "Debit")
            .options("account_currency")
            .columns(2)
            .bold()
            .print_hide_if_no_value()
            .in_list_view()
    );
    assert_eq!(
        fields[17],
        FieldSpec::select("reference_type", "Reference Type")
            .options("\nSales Invoice\nPurchase Invoice\nJournal Entry\nSales Order\nPurchase Order\nExpense Claim\nAsset\nLoan\nPayroll Entry\nEmployee Advance\nExchange Rate Revaluation\nInvoice Discounting\nFees\nFull and Final Statement\nPayment Entry\nBank Transaction")
            .no_copy()
            .search_index()
    );
    assert_eq!(
        fields[19],
        FieldSpec::date("reference_due_date", "Reference Due Date")
            .depends_on("eval:doc.reference_type&&!in_list(doc.reference_type, ['Expense Claim', 'Asset', 'Employee Loan', 'Employee Advance', 'Bank Transaction'])")
            .no_copy()
    );
    assert_eq!(
        fields[25],
        FieldSpec::section_break("accounting_dimensions_section")
            .label("Accounting Dimensions")
            .collapsible()
    );
    assert_eq!(
        fields[31],
        FieldSpec::check("is_tax_withholding_account", "Is Tax Withholding Account")
            .default("0")
            .read_only()
    );

    let row = JournalEntryAccount;
    assert_eq!(row.doctype(), "Journal Entry Account");
    assert!(row.custom_hooks().is_empty());
}
