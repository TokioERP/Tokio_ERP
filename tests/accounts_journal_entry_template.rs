use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::journal_entry_template::journal_entry_template::{
    get_naming_series, JournalEntryTemplate, JournalEntryTemplateError,
};
use tokio_erp::erpnext::accounts::doctype::journal_entry_template_account::journal_entry_template_account::JournalEntryTemplateAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn journal_entry_template_matches_erpnext_metadata() {
    assert_eq!(JournalEntryTemplate::DOCTYPE, "Journal Entry Template");
    assert_eq!(JournalEntryTemplate::MODULE, "Accounts");
    assert_eq!(
        JournalEntryTemplate::FIELD_ORDER,
        [
            "section_break_1",
            "template_title",
            "voucher_type",
            "naming_series",
            "column_break_3",
            "company",
            "is_opening",
            "multi_currency",
            "section_break_3",
            "accounts",
        ]
    );
    assert_eq!(JournalEntryTemplate::SORT_FIELD, "creation");
    assert_eq!(JournalEntryTemplate::SORT_ORDER, "DESC");
    assert!(JournalEntryTemplate::TRACK_CHANGES);

    assert_eq!(
        JournalEntryTemplate::fields(),
        vec![
            FieldSpec::section_break("section_break_1"),
            FieldSpec::data("template_title", "Template Title")
                .required()
                .unique(),
            FieldSpec::select("voucher_type", "Journal Entry Type")
                .options(JournalEntryTemplate::VOUCHER_TYPE_OPTIONS)
                .in_list_view()
                .required(),
            FieldSpec::select("naming_series", "Series")
                .no_copy()
                .print_hide()
                .required()
                .set_only_once(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .in_standard_filter()
                .required(),
            FieldSpec::select("is_opening", "Is Opening")
                .options("No\nYes")
                .default("No"),
            FieldSpec::check("multi_currency", "Multi Currency").default("0"),
            FieldSpec::section_break("section_break_3"),
            FieldSpec::table("accounts", "Accounting Entries")
                .options("Journal Entry Template Account"),
        ]
    );
}

#[test]
fn journal_entry_template_validate_party_matches_erpnext() {
    let mut account_types = BTreeMap::new();
    account_types.insert("Debtors - AC".to_string(), "Receivable".to_string());
    account_types.insert("Creditors - AC".to_string(), "Payable".to_string());
    account_types.insert("Bank - AC".to_string(), "Bank".to_string());

    let ok = JournalEntryTemplate {
        template_title: "Customer Receipt".to_string(),
        accounts: vec![JournalEntryTemplateAccount {
            account: Some("Debtors - AC".to_string()),
            party_type: Some("Customer".to_string()),
            party: Some("CUST-001".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(ok.custom_hooks(), ["validate"]);
    assert_eq!(ok.validate(&account_types), Ok(()));
    assert_eq!(ok.doctype(), "Journal Entry Template");
    assert_eq!(ok.module(), "Accounts");

    let invalid_party_type = JournalEntryTemplate {
        template_title: "Bad Party Type".to_string(),
        accounts: vec![JournalEntryTemplateAccount {
            account: Some("Bank - AC".to_string()),
            party_type: Some("Customer".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        invalid_party_type.validate(&account_types),
        Err(
            JournalEntryTemplateError::PartyTypeOnlyAllowedForReceivableOrPayable {
                row: 1,
                account: "Bank - AC".to_string(),
            }
        )
    );

    let party_without_type = JournalEntryTemplate {
        template_title: "Bad Party".to_string(),
        accounts: vec![JournalEntryTemplateAccount {
            account: Some("Debtors - AC".to_string()),
            party: Some("CUST-001".to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        party_without_type.validate(&account_types),
        Err(JournalEntryTemplateError::PartyRequiresPartyType {
            row: 1,
            account: "Debtors - AC".to_string(),
        })
    );

    assert_eq!(get_naming_series("JV-.YYYY.-"), "JV-.YYYY.-");
}
