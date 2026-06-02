use tokio_erp::erpnext::accounts::doctype::budget_account::budget_account::BudgetAccount;
use tokio_erp::erpnext::accounts::doctype::budget_distribution::budget_distribution::BudgetDistribution;
use tokio_erp::erpnext::accounts::doctype::dunning_letter_text::dunning_letter_text::DunningLetterText;
use tokio_erp::erpnext::accounts::doctype::finance_book::finance_book::FinanceBook;
use tokio_erp::erpnext::accounts::doctype::item_tax_template_detail::item_tax_template_detail::ItemTaxTemplateDetail;
use tokio_erp::erpnext::accounts::doctype::journal_entry_template_account::journal_entry_template_account::JournalEntryTemplateAccount;
use tokio_erp::erpnext::accounts::doctype::ledger_health::ledger_health::LedgerHealth;
use tokio_erp::erpnext::accounts::doctype::loyalty_point_entry_redemption::loyalty_point_entry_redemption::LoyaltyPointEntryRedemption;
use tokio_erp::erpnext::accounts::doctype::loyalty_program_collection::loyalty_program_collection::LoyaltyProgramCollection;
use tokio_erp::erpnext::accounts::doctype::mode_of_payment_account::mode_of_payment_account::ModeOfPaymentAccount;
use tokio_erp::erpnext::accounts::doctype::pos_profile_user::pos_profile_user::PosProfileUser;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn finance_book_matches_erpnext_pass_controller_metadata() {
    assert_eq!(FinanceBook::DOCTYPE, "Finance Book");
    assert_eq!(FinanceBook::MODULE, "Accounts");
    assert_eq!(FinanceBook::FIELD_ORDER, ["finance_book_name"]);
    assert!(FinanceBook::QUICK_ENTRY);
    assert!(FinanceBook::TRACK_CHANGES);
    assert_eq!(
        FinanceBook::fields(),
        vec![FieldSpec::data("finance_book_name", "Name")]
    );

    let doc = FinanceBook::new("Primary Book");
    assert_eq!(doc.finance_book_name.as_deref(), Some("Primary Book"));
    assert_eq!(doc.doctype(), "Finance Book");
    assert_eq!(doc.module(), "Accounts");
    assert!(doc.custom_hooks().is_empty());
}

#[test]
fn budget_account_matches_erpnext_pass_controller_metadata() {
    assert_eq!(BudgetAccount::DOCTYPE, "Budget Account");
    assert_eq!(BudgetAccount::FIELD_ORDER, ["account", "budget_amount"]);
    assert!(BudgetAccount::IS_TABLE);
    assert!(BudgetAccount::QUICK_ENTRY);
    assert_eq!(
        BudgetAccount::fields(),
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::currency("budget_amount", "Budget Amount")
                .options("Company:company:default_currency")
                .required()
                .in_list_view(),
        ]
    );

    let doc = BudgetAccount::new("Expense - TC", 1200.0);
    assert_eq!(doc.account.as_deref(), Some("Expense - TC"));
    assert_eq!(doc.budget_amount, 1200.0);
    assert_eq!(doc.doctype(), "Budget Account");
    assert!(doc.custom_hooks().is_empty());
}

#[test]
fn mode_of_payment_account_and_pos_profile_user_match_erpnext_metadata() {
    assert_eq!(ModeOfPaymentAccount::DOCTYPE, "Mode of Payment Account");
    assert_eq!(
        ModeOfPaymentAccount::FIELD_ORDER,
        ["company", "default_account"]
    );
    assert!(ModeOfPaymentAccount::IS_TABLE);
    assert_eq!(
        ModeOfPaymentAccount::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view(),
            FieldSpec::link("default_account", "Default Account")
                .options("Account")
                .in_list_view(),
        ]
    );
    let account = ModeOfPaymentAccount::new("_Test Company", "Cash - TC");
    assert_eq!(account.company.as_deref(), Some("_Test Company"));
    assert_eq!(account.default_account.as_deref(), Some("Cash - TC"));
    assert!(account.custom_hooks().is_empty());

    assert_eq!(PosProfileUser::DOCTYPE, "POS Profile User");
    assert_eq!(PosProfileUser::FIELD_ORDER, ["default", "user"]);
    assert!(PosProfileUser::IS_TABLE);
    assert!(PosProfileUser::QUICK_ENTRY);
    assert!(PosProfileUser::TRACK_CHANGES);
    assert_eq!(
        PosProfileUser::fields(),
        vec![
            FieldSpec::check("default", "Default")
                .default("0")
                .in_list_view(),
            FieldSpec::link("user", "User")
                .options("User")
                .in_list_view(),
        ]
    );
    let user = PosProfileUser::new(true, "cashier@example.com");
    assert!(user.default);
    assert_eq!(user.user.as_deref(), Some("cashier@example.com"));
}

#[test]
fn tax_ledger_and_loyalty_child_tables_match_erpnext_metadata() {
    assert_eq!(ItemTaxTemplateDetail::DOCTYPE, "Item Tax Template Detail");
    assert_eq!(
        ItemTaxTemplateDetail::FIELD_ORDER,
        ["tax_type", "tax_rate", "not_applicable"]
    );
    assert!(ItemTaxTemplateDetail::IS_TABLE);
    assert!(ItemTaxTemplateDetail::QUICK_ENTRY);
    assert!(ItemTaxTemplateDetail::TRACK_CHANGES);
    assert_eq!(
        ItemTaxTemplateDetail::fields(),
        vec![
            FieldSpec::link("tax_type", "Tax")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::float("tax_rate", "Tax Rate")
                .read_only_depends_on("eval:doc.not_applicable")
                .in_list_view(),
            FieldSpec::check("not_applicable", "Not Applicable")
                .default("0")
                .in_list_view(),
        ]
    );

    assert_eq!(LedgerHealth::DOCTYPE, "Ledger Health");
    assert_eq!(LedgerHealth::MODULE, "Accounts");
    assert_eq!(LedgerHealth::AUTONAME, "autoincrement");
    assert_eq!(LedgerHealth::NAMING_RULE, "Autoincrement");
    assert!(LedgerHealth::IN_CREATE);
    assert!(LedgerHealth::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(LedgerHealth::READ_ONLY);
    assert_eq!(LedgerHealth::SORT_FIELD, "modified");
    assert_eq!(LedgerHealth::SORT_ORDER, "DESC");
    assert_eq!(
        LedgerHealth::FIELD_ORDER,
        [
            "voucher_type",
            "voucher_no",
            "checked_on",
            "debit_credit_mismatch",
            "general_and_payment_ledger_mismatch",
        ]
    );
    assert_eq!(
        LedgerHealth::fields(),
        vec![
            FieldSpec::data("voucher_type", "Voucher Type"),
            FieldSpec::data("voucher_no", "Voucher No"),
            FieldSpec::check("debit_credit_mismatch", "Debit-Credit mismatch").default("0"),
            FieldSpec::datetime("checked_on", "Checked On"),
            FieldSpec::check(
                "general_and_payment_ledger_mismatch",
                "General and Payment Ledger mismatch",
            )
            .default("0"),
        ]
    );

    assert_eq!(
        LoyaltyPointEntryRedemption::fields(),
        vec![
            FieldSpec::data("sales_invoice", "Sales Invoice").in_list_view(),
            FieldSpec::date("redemption_date", "Redemption Date").in_list_view(),
            FieldSpec::int("redeemed_points", "Redeemed Points").in_list_view(),
        ]
    );
    assert_eq!(LoyaltyPointEntryRedemption::SORT_FIELD, "creation");
    assert_eq!(LoyaltyPointEntryRedemption::SORT_ORDER, "DESC");
    let redemption = LoyaltyPointEntryRedemption::new("SINV-0001", "2026-05-30", 25);
    assert_eq!(redemption.redeemed_points, 25);
}

#[test]
fn loyalty_collection_budget_distribution_and_dunning_text_match_erpnext_metadata() {
    assert_eq!(
        LoyaltyProgramCollection::fields(),
        vec![
            FieldSpec::data("tier_name", "Tier Name")
                .required()
                .columns(3)
                .in_list_view(),
            FieldSpec::currency("min_spent", "Minimum Total Spent")
                .columns(3)
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::currency("collection_factor", "Collection Factor (=1 LP)")
                .required()
                .columns(3)
                .description("For how much spent = 1 Loyalty Point")
                .in_list_view(),
        ]
    );
    assert_eq!(LoyaltyProgramCollection::SORT_FIELD, "creation");
    assert_eq!(LoyaltyProgramCollection::SORT_ORDER, "DESC");

    assert_eq!(
        BudgetDistribution::fields(),
        vec![
            FieldSpec::date("start_date", "Start Date")
                .read_only()
                .in_list_view(),
            FieldSpec::date("end_date", "End Date")
                .read_only()
                .in_list_view(),
            FieldSpec::currency("amount", "Amount").in_list_view(),
            FieldSpec::percent("percent", "Percent").in_list_view(),
        ]
    );

    assert_eq!(DunningLetterText::DOCTYPE, "Dunning Letter Text");
    assert_eq!(
        DunningLetterText::FIELD_ORDER,
        [
            "language",
            "is_default_language",
            "section_break_4",
            "body_text",
            "closing_text",
            "section_break_7",
            "body_and_closing_text_help",
        ]
    );
    assert!(DunningLetterText::IS_TABLE);
    assert!(DunningLetterText::TRACK_CHANGES);
    assert_eq!(DunningLetterText::fields().len(), 7);
    assert_eq!(
        DunningLetterText::fields()[0],
        FieldSpec::link("language", "Language")
            .options("Language")
            .in_list_view()
    );
    assert_eq!(
        DunningLetterText::fields()[6],
        FieldSpec::html("body_and_closing_text_help", "Body and Closing Text Help")
            .options(DunningLetterText::BODY_AND_CLOSING_TEXT_HELP)
    );
}

#[test]
fn journal_entry_template_account_matches_erpnext_metadata() {
    assert_eq!(
        JournalEntryTemplateAccount::FIELD_ORDER,
        [
            "account",
            "party_type",
            "party",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
            "project",
        ]
    );
    assert!(JournalEntryTemplateAccount::IS_TABLE);
    assert!(JournalEntryTemplateAccount::TRACK_CHANGES);
    assert_eq!(JournalEntryTemplateAccount::ROW_FORMAT, "Dynamic");
    assert_eq!(JournalEntryTemplateAccount::SORT_FIELD, "creation");
    assert_eq!(JournalEntryTemplateAccount::SORT_ORDER, "DESC");
    assert_eq!(
        JournalEntryTemplateAccount::fields(),
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .required()
                .in_list_view(),
            FieldSpec::link("party_type", "Party Type")
                .options("DocType")
                .in_list_view(),
            FieldSpec::dynamic_link("party")
                .label("Party")
                .options("party_type")
                .in_list_view(),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions")
                .collapsible(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::link("project", "Project").options("Project"),
        ]
    );

    let account = JournalEntryTemplateAccount::new("Bank - TC");
    assert_eq!(account.account.as_deref(), Some("Bank - TC"));
    assert_eq!(account.doctype(), "Journal Entry Template Account");
    assert_eq!(account.module(), "Accounts");
    assert!(account.custom_hooks().is_empty());
}
