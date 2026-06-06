use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::party::{
    build_tax_args, choose_address_tax_category, choose_party_account,
    choose_party_advance_account, get_due_date_from_template, get_party_shipping_address,
    get_payment_terms_template, set_account_and_due_date, validate_party_frozen_disabled,
    AddressRecord, CompanyDefaults, PartyAccountInputs, PartyFrozenDisabledError, PartyRecord,
    PaymentTerm, PURCHASE_TRANSACTION_TYPES, SALES_TRANSACTION_TYPES, TRANSACTION_TYPES,
};

#[test]
fn party_transaction_type_constants_match_erpnext_sets() {
    assert!(PURCHASE_TRANSACTION_TYPES.contains(&"Purchase Invoice"));
    assert!(PURCHASE_TRANSACTION_TYPES.contains(&"Purchase Receipt"));
    assert!(SALES_TRANSACTION_TYPES.contains(&"Sales Invoice"));
    assert!(SALES_TRANSACTION_TYPES.contains(&"POS Invoice"));
    assert!(TRANSACTION_TYPES.contains(&"Quotation"));
    assert!(TRANSACTION_TYPES.contains(&"Supplier Quotation"));
}

#[test]
fn party_account_and_due_date_helpers_match_erpnext_fallback_order() {
    let mut direct_accounts = BTreeMap::new();
    direct_accounts.insert(
        (
            "Customer".to_string(),
            "CUST-1".to_string(),
            "Acme".to_string(),
        ),
        "Debtors - A".to_string(),
    );
    let inputs = PartyAccountInputs {
        direct_accounts,
        direct_advance_accounts: BTreeMap::from([(
            (
                "Customer".to_string(),
                "CUST-1".to_string(),
                "Acme".to_string(),
            ),
            "Advance Received - A".to_string(),
        )]),
        group_accounts: BTreeMap::from([(
            (
                "Customer Group".to_string(),
                "Retail".to_string(),
                "Acme".to_string(),
            ),
            "Debtors Retail - A".to_string(),
        )]),
        group_advance_accounts: BTreeMap::new(),
        party_groups: BTreeMap::from([(
            ("Customer".to_string(), "CUST-1".to_string()),
            "Retail".to_string(),
        )]),
        company_defaults: BTreeMap::from([(
            "Acme".to_string(),
            CompanyDefaults {
                default_receivable_account: Some("Debtors Default - A".to_string()),
                default_payable_account: Some("Creditors Default - A".to_string()),
                default_advance_received_account: Some("Adv Rec Default - A".to_string()),
                default_advance_paid_account: Some("Adv Paid Default - A".to_string()),
                default_payment_terms: Some("Net 30".to_string()),
            },
        )]),
        party_type_account_types: BTreeMap::new(),
        fallback_default_accounts: BTreeMap::new(),
        account_currencies: BTreeMap::from([("Debtors - A".to_string(), "USD".to_string())]),
        existing_gle_currency: None,
        existing_gle_account: None,
    };

    assert_eq!(
        choose_party_account("Customer", Some("CUST-1"), "Acme", false, &inputs),
        vec!["Debtors - A".to_string()]
    );
    assert_eq!(
        choose_party_account("Customer", Some("CUST-1"), "Acme", true, &inputs),
        vec![
            "Debtors - A".to_string(),
            "Advance Received - A".to_string()
        ]
    );
    assert_eq!(
        choose_party_advance_account("Supplier", "SUP-1", "Acme", &inputs).as_deref(),
        Some("Adv Paid Default - A")
    );

    let result = set_account_and_due_date(
        Some("CUST-1"),
        Some("Manual Debtors - A"),
        "Customer",
        Some("Acme"),
        Some("2026-06-06"),
        None,
        "Sales Order",
        &inputs,
        &[],
    );
    assert_eq!(result.party.as_deref(), Some("CUST-1"));
    assert_eq!(result.account_fieldname, None);
    assert_eq!(result.account, None);
    assert_eq!(result.due_date, None);

    let result = set_account_and_due_date(
        Some("CUST-1"),
        None,
        "Customer",
        Some("Acme"),
        Some("2026-06-06"),
        None,
        "Sales Invoice",
        &inputs,
        &[PaymentTerm::days_after_invoice(10)],
    );
    assert_eq!(result.account_fieldname.as_deref(), Some("debit_to"));
    assert_eq!(result.account.as_deref(), Some("Debtors - A"));
    assert_eq!(result.due_date.as_deref(), Some("2026-06-16"));
}

#[test]
fn party_due_date_payment_terms_and_tax_helpers_match_erpnext() {
    let terms = vec![
        PaymentTerm::days_after_invoice(10),
        PaymentTerm::days_after_invoice_month_end(5),
        PaymentTerm::months_after_invoice_month_end(2),
    ];
    assert_eq!(
        get_due_date_from_template("2026-01-15", None, &terms),
        "2026-04-30"
    );
    assert_eq!(
        get_due_date_from_template(
            "2026-01-15",
            Some("2026-01-20"),
            &[PaymentTerm::days_after_invoice(3)]
        ),
        "2026-01-23"
    );

    let party = PartyRecord {
        payment_terms: None,
        group: Some("Retail".to_string()),
        disabled: false,
        frozen: false,
    };
    assert_eq!(
        get_payment_terms_template(
            "Customer",
            &party,
            &BTreeMap::from([("Retail".to_string(), "Group Terms".to_string())]),
            Some("Company Terms"),
        )
        .as_deref(),
        Some("Group Terms")
    );

    assert_eq!(
        choose_address_tax_category(
            Some("Default Tax"),
            Some("Billing Tax"),
            Some("Shipping Tax"),
            "Shipping Address"
        ),
        "Shipping Tax"
    );
    assert_eq!(
        choose_address_tax_category(Some("Default Tax"), None, None, "Billing Address"),
        "Default Tax"
    );

    let args = build_tax_args(
        "LEAD-1",
        "Lead",
        "Acme",
        Some("Retail"),
        None,
        Some("In-State"),
        Some("BILL-1"),
        Some("SHIP-1"),
        Some(true),
    );
    assert_eq!(args["company"], "Acme");
    assert_eq!(args["tax_type"], "Sales");
    assert_eq!(args["customer"], "");
    assert!(!args.contains_key("lead"));
    assert_eq!(args["use_for_shopping_cart"], "1");
}

#[test]
fn party_validation_and_shipping_selection_match_erpnext_branches() {
    assert_eq!(
        validate_party_frozen_disabled(
            "Acme",
            "Customer",
            "CUST-1",
            &PartyRecord {
                payment_terms: None,
                group: None,
                disabled: true,
                frozen: false,
            },
            Some("Accounts Manager"),
            &["Accounts User".to_string()],
            false,
        ),
        Err(PartyFrozenDisabledError::Disabled)
    );
    assert_eq!(
        validate_party_frozen_disabled(
            "Acme",
            "Supplier",
            "SUP-1",
            &PartyRecord {
                payment_terms: None,
                group: None,
                disabled: false,
                frozen: true,
            },
            Some("Accounts Manager"),
            &["Accounts Manager".to_string()],
            false,
        ),
        Ok(())
    );

    let addresses = vec![
        AddressRecord {
            name: "ADDR-2".to_string(),
            is_shipping_address: true,
        },
        AddressRecord {
            name: "ADDR-1".to_string(),
            is_shipping_address: false,
        },
    ];
    assert_eq!(
        get_party_shipping_address(&addresses).as_deref(),
        Some("ADDR-2")
    );
    assert_eq!(
        get_party_shipping_address(&[AddressRecord {
            name: "ADDR-ONLY".to_string(),
            is_shipping_address: false,
        }])
        .as_deref(),
        Some("ADDR-ONLY")
    );
    assert_eq!(
        get_party_shipping_address(&[
            AddressRecord {
                name: "ADDR-1".to_string(),
                is_shipping_address: false,
            },
            AddressRecord {
                name: "ADDR-3".to_string(),
                is_shipping_address: false,
            },
        ]),
        None
    );
}
