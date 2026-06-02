use tokio_erp::erpnext::accounts::doctype::bank::bank::{
    Bank, BankLifecycleAction, DeleteContactAndAddress,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_matches_erpnext_metadata() {
    assert_eq!(Bank::DOCTYPE, "Bank");
    assert_eq!(Bank::MODULE, "Accounts");
    assert_eq!(Bank::AUTONAME, "field:bank_name");
    assert_eq!(
        Bank::FIELD_ORDER,
        [
            "bank_details_section",
            "bank_name",
            "swift_number",
            "column_break_1",
            "website",
            "address_and_contact",
            "address_html",
            "column_break_13",
            "contact_html",
            "data_import_configuration_section",
            "bank_transaction_mapping",
            "section_break_4",
            "plaid_access_token",
        ]
    );
    assert!(Bank::ALLOW_IMPORT);
    assert!(Bank::ALLOW_RENAME);
    assert_eq!(Bank::DOCUMENT_TYPE, "Setup");
    assert!(Bank::EDITABLE_GRID);
    assert!(Bank::QUICK_ENTRY);
    assert_eq!(Bank::SORT_FIELD, "creation");
    assert_eq!(Bank::SORT_ORDER, "DESC");
    assert!(Bank::TRACK_CHANGES);

    assert_eq!(
        Bank::fields(),
        vec![
            FieldSpec::section_break("bank_details_section").label("Bank Details"),
            FieldSpec::data("bank_name", "Bank Name")
                .required()
                .unique(),
            FieldSpec::data("swift_number", "SWIFT number")
                .unique()
                .in_list_view()
                .in_standard_filter(),
            FieldSpec::column_break("column_break_1").search_index(),
            FieldSpec::data("website", "Website"),
            FieldSpec::section_break("address_and_contact")
                .label("Address and Contact")
                .options("fa fa-map-marker"),
            FieldSpec::html("address_html", "Address HTML"),
            FieldSpec::column_break("column_break_13"),
            FieldSpec::html("contact_html", "Contact HTML"),
            FieldSpec::section_break("data_import_configuration_section")
                .label("Data Import Configuration")
                .collapsible(),
            FieldSpec::table("bank_transaction_mapping", "Bank Transaction Mapping")
                .options("Bank Transaction Mapping"),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::data("plaid_access_token", "Plaid Access Token")
                .hidden()
                .no_copy()
                .read_only(),
        ]
    );
}

#[test]
fn bank_controller_hooks_match_erpnext_lifecycle() {
    let bank = Bank::new("Atlas Bank");

    assert_eq!(bank.doctype(), "Bank");
    assert_eq!(bank.module(), "Accounts");
    assert_eq!(bank.custom_hooks(), ["onload", "on_trash"]);
    assert_eq!(
        bank.onload(),
        BankLifecycleAction::LoadAddressAndContact {
            doctype: "Bank",
            name: "Atlas Bank".to_string(),
        }
    );
    assert_eq!(
        bank.on_trash(),
        BankLifecycleAction::DeleteContactAndAddress(DeleteContactAndAddress {
            doctype: "Bank",
            name: "Atlas Bank".to_string(),
        })
    );
}
