use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::mode_of_payment::mode_of_payment::{
    ModeOfPayment, ModeOfPaymentError,
};
use tokio_erp::erpnext::accounts::doctype::mode_of_payment_account::mode_of_payment_account::ModeOfPaymentAccount;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn mode_of_payment_matches_erpnext_metadata() {
    assert_eq!(ModeOfPayment::DOCTYPE, "Mode of Payment");
    assert_eq!(ModeOfPayment::MODULE, "Accounts");
    assert_eq!(
        ModeOfPayment::FIELD_ORDER,
        ["mode_of_payment", "enabled", "type", "accounts"]
    );
    assert!(ModeOfPayment::ALLOW_IMPORT);
    assert!(ModeOfPayment::ALLOW_RENAME);
    assert_eq!(ModeOfPayment::AUTONAME, "field:mode_of_payment");
    assert_eq!(ModeOfPayment::DOCUMENT_TYPE, "Setup");
    assert_eq!(ModeOfPayment::ICON, "fa fa-credit-card");
    assert!(ModeOfPayment::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(ModeOfPayment::QUICK_ENTRY);
    assert_eq!(ModeOfPayment::ROW_FORMAT, "Dynamic");
    assert!(ModeOfPayment::SHOW_NAME_IN_GLOBAL_SEARCH);
    assert_eq!(ModeOfPayment::SORT_FIELD, "creation");
    assert_eq!(ModeOfPayment::SORT_ORDER, "ASC");
    assert!(ModeOfPayment::TRANSLATED_DOCTYPE);

    assert_eq!(
        ModeOfPayment::fields(),
        vec![
            FieldSpec::data("mode_of_payment", "Mode of Payment")
                .oldfield("mode_of_payment", "Data")
                .in_list_view()
                .required()
                .unique(),
            FieldSpec::select("type", "Type")
                .options("Cash\nBank\nGeneral\nPhone")
                .in_standard_filter(),
            FieldSpec::table("accounts", "Accounts").options("Mode of Payment Account"),
            FieldSpec::check("enabled", "Enabled").default("1"),
        ]
    );
}

#[test]
fn mode_of_payment_account_matches_erpnext_metadata() {
    assert_eq!(ModeOfPaymentAccount::DOCTYPE, "Mode of Payment Account");
    assert_eq!(ModeOfPaymentAccount::MODULE, "Accounts");
    assert_eq!(ModeOfPaymentAccount::FIELD_ORDER, ["company", "default_account"]);
    assert!(ModeOfPaymentAccount::IS_TABLE);
    assert_eq!(ModeOfPaymentAccount::SORT_FIELD, "creation");
    assert_eq!(ModeOfPaymentAccount::SORT_ORDER, "DESC");

    assert_eq!(
        ModeOfPaymentAccount::fields(),
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view(),
            FieldSpec::link("default_account", "Default Account")
                .options("Account")
                .description(
                    "Default account will be automatically updated in POS Invoice when this mode is selected.",
                )
                .in_list_view(),
        ]
    );

    let row = ModeOfPaymentAccount::new("Acme", "Cash - AC");
    assert_eq!(row.doctype(), "Mode of Payment Account");
    assert_eq!(row.module(), "Accounts");
}

#[test]
fn mode_of_payment_validate_matches_erpnext_order_and_errors() {
    let mut account_companies = BTreeMap::new();
    account_companies.insert("Cash - AC".to_string(), "Acme".to_string());
    account_companies.insert("Bank - TC".to_string(), "Test Co".to_string());

    let doc = ModeOfPayment {
        name: Some("Cash".to_string()),
        mode_of_payment: "Cash".to_string(),
        enabled: true,
        accounts: vec![ModeOfPaymentAccount::new("Acme", "Cash - AC")],
        ..Default::default()
    };
    assert_eq!(doc.custom_hooks(), ["validate"]);
    assert_eq!(doc.validate(&account_companies, &[]), Ok(()));
    assert_eq!(doc.doctype(), "Mode of Payment");
    assert_eq!(doc.module(), "Accounts");

    let duplicate = ModeOfPayment {
        name: Some("Cash".to_string()),
        mode_of_payment: "Cash".to_string(),
        accounts: vec![
            ModeOfPaymentAccount::new("Acme", "Cash - AC"),
            ModeOfPaymentAccount::new("Acme", "Cash - AC"),
        ],
        ..Default::default()
    };
    assert_eq!(
        duplicate.validate(&account_companies, &[]),
        Err(ModeOfPaymentError::DuplicateCompany)
    );

    let mismatch = ModeOfPayment {
        name: Some("Cash".to_string()),
        mode_of_payment: "Cash".to_string(),
        accounts: vec![ModeOfPaymentAccount::new("Acme", "Bank - TC")],
        ..Default::default()
    };
    assert_eq!(
        mismatch.validate(&account_companies, &[]),
        Err(ModeOfPaymentError::AccountCompanyMismatch {
            default_account: "Bank - TC".to_string(),
            company: "Acme".to_string(),
            mode_of_payment: "Cash".to_string(),
        })
    );

    let disabled = ModeOfPayment {
        name: Some("Cash".to_string()),
        mode_of_payment: "Cash".to_string(),
        enabled: false,
        accounts: vec![ModeOfPaymentAccount::new("Acme", "Cash - AC")],
        ..Default::default()
    };
    assert_eq!(
        disabled.validate(
            &account_companies,
            &["POS Main".to_string(), "POS Alt".to_string()]
        ),
        Err(ModeOfPaymentError::UsedInPosProfile {
            pos_profiles: vec!["POS Main".to_string(), "POS Alt".to_string()],
            mode_of_payment: "Cash".to_string(),
        })
    );
}
