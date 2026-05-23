use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::bank_account::bank_account::{
    get_bank_account_details_plan, get_default_company_bank_account, get_party_bank_account_plan,
    BankAccount, BankAccountDetailsPlan, BankAccountFilter, BankAccountLookupPlan,
    BankAccountValidationError, DefaultBankAccountUpdate,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_account_matches_erpnext_metadata_and_fields() {
    assert_eq!(BankAccount::DOCTYPE, "Bank Account");
    assert_eq!(BankAccount::MODULE, "Accounts");
    assert_eq!(BankAccount::AUTONAME, "field:account_name-bank");
    assert_eq!(
        BankAccount::FIELD_ORDER,
        [
            "account_name",
            "account",
            "bank",
            "account_type",
            "account_subtype",
            "column_break_7",
            "disabled",
            "is_default",
            "is_company_account",
            "company",
            "section_break_11",
            "party_type",
            "column_break_14",
            "party",
            "account_details_section",
            "iban",
            "column_break_12",
            "branch_code",
            "bank_account_no",
            "address_and_contact",
            "address_html",
            "column_break_13",
            "contact_html",
            "integration_details_section",
            "integration_id",
            "last_integration_date",
            "column_break_27",
            "mask",
        ]
    );
    assert!(BankAccount::ALLOW_IMPORT);
    assert!(BankAccount::ALLOW_RENAME);
    assert!(BankAccount::TRACK_CHANGES);

    let fields = BankAccount::fields();
    assert!(fields.contains(
        &FieldSpec::data("account_name", "Account Name")
            .required()
            .in_global_search()
            .in_list_view()
            .in_standard_filter()
    ));
    assert!(fields.contains(
        &FieldSpec::link("account", "Company Account")
            .options("Account")
            .depends_on("is_company_account")
            .mandatory_depends_on("is_company_account")
            .in_list_view()
    ));
    assert!(fields.contains(&FieldSpec::link("bank", "Bank").options("Bank").required()));
    assert!(fields.contains(
        &FieldSpec::dynamic_link("party")
            .label("Party")
            .options("party_type")
    ));
    assert!(fields.contains(
        &FieldSpec::section_break("address_and_contact")
            .label("Address and Contact")
            .options("fa fa-map-marker")
    ));
    assert!(fields.contains(
        &FieldSpec::data("iban", "IBAN")
            .options("IBAN")
            .length(34)
            .in_list_view()
    ));
    assert!(fields.contains(&FieldSpec::data("branch_code", "Branch Code").in_global_search()));
    assert!(fields.contains(
        &FieldSpec::data("bank_account_no", "Bank Account No")
            .length(30)
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::data("integration_id", "Integration ID")
            .hidden()
            .no_copy()
            .read_only()
            .unique()
    ));
}

#[test]
fn bank_account_lifecycle_and_autoname_match_erpnext() {
    let mut doc = BankAccount::new("Operating", "Atlas Bank");

    assert_eq!(doc.doctype(), "Bank Account");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        ["onload", "autoname", "on_trash", "validate"]
    );
    assert_eq!(doc.autoname(), "Operating - Atlas Bank");
    assert_eq!(doc.name.as_deref(), Some("Operating - Atlas Bank"));
    assert_eq!(doc.onload().doctype, "Bank Account");
    assert_eq!(doc.on_trash().name, "Operating - Atlas Bank");
}

#[test]
fn bank_account_company_account_validation_matches_erpnext_guards() {
    let mut doc = BankAccount::new("Operating", "Atlas Bank");
    doc.is_company_account = true;

    assert_eq!(
        doc.validate_is_company_account(&[]),
        Err(BankAccountValidationError::CompanyMandatoryForCompanyAccount)
    );

    doc.company = Some("_Test Company".to_string());
    assert_eq!(
        doc.validate_is_company_account(&[]),
        Err(BankAccountValidationError::CompanyAccountMandatory)
    );

    doc.account = Some("Cash - TC".to_string());
    assert_eq!(
        doc.validate_is_company_account(&["Other Bank Account".to_string()]),
        Err(BankAccountValidationError::AccountAlreadyUsed {
            account: "Cash - TC".to_string(),
            links: vec!["Bank Account/Other Bank Account".to_string()],
        })
    );
    assert_eq!(doc.validate_is_company_account(&[]), Ok(()));
}

#[test]
fn bank_account_default_update_filter_matches_erpnext() {
    let mut doc = BankAccount::new("Operating", "Atlas Bank");
    doc.party_type = Some("Customer".to_string());
    doc.party = Some("CUST-0001".to_string());
    doc.company = Some("_Test Company".to_string());
    doc.is_company_account = false;
    doc.is_default = true;

    assert_eq!(
        doc.update_default_bank_account(),
        Some(DefaultBankAccountUpdate {
            doctype: "Bank Account",
            filters: BankAccountFilter {
                party_type: Some("Customer".to_string()),
                party: Some("CUST-0001".to_string()),
                is_company_account: false,
                company: Some("_Test Company".to_string()),
                is_default: true,
                disabled: false,
            },
            fieldname: "is_default",
            value: false,
        })
    );

    doc.disabled = true;
    assert_eq!(doc.update_default_bank_account(), None);
}

#[test]
fn bank_account_lookup_helpers_match_erpnext_queries() {
    assert_eq!(
        get_party_bank_account_plan("Customer", "CUST-0001"),
        BankAccountLookupPlan {
            doctype: "Bank Account",
            filters: BTreeMap::from([
                ("party_type".to_string(), "Customer".to_string()),
                ("party".to_string(), "CUST-0001".to_string()),
                ("is_default".to_string(), "1".to_string()),
                ("disabled".to_string(), "0".to_string()),
            ]),
            fieldname: "name",
        }
    );

    assert_eq!(
        get_default_company_bank_account(
            "_Test Company",
            Some("BA-0001"),
            Some("_Test Company"),
            Some("BA-FALLBACK"),
        ),
        Some("BA-0001".to_string())
    );
    assert_eq!(
        get_default_company_bank_account(
            "_Test Company",
            Some("BA-0002"),
            Some("Other Company"),
            Some("BA-FALLBACK"),
        ),
        Some("BA-FALLBACK".to_string())
    );
    assert_eq!(
        get_default_company_bank_account("_Test Company", None, None, Some("BA-FALLBACK")),
        Some("BA-FALLBACK".to_string())
    );
}

#[test]
fn bank_account_details_plan_matches_permission_and_cached_fields() {
    assert_eq!(
        get_bank_account_details_plan("BA-0001"),
        BankAccountDetailsPlan {
            permission_doctype: "Bank Account",
            permission_doc: "BA-0001".to_string(),
            permission_type: "read",
            cached_doctype: "Bank Account",
            cached_name: "BA-0001".to_string(),
            fields: ["account", "bank", "bank_account_no"],
        }
    );
}
