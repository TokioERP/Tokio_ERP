use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::test::accounts_mixin::{
    account_attribute_name, clear_old_entries_doctypes, default_company_accounts,
    warehouse_attribute_name, AccountSeed, AccountsMixinState, CompanySeed, CustomerSeed,
    ExistingAccount, SupplierSeed, WarehouseSeed,
};

#[test]
fn accounts_mixin_customer_supplier_and_item_creation_match_erpnext_branches() {
    let mut state = AccountsMixinState::default();

    let new_customer = state.create_customer(
        CustomerSeed {
            customer_name: "_Test Customer".to_string(),
            currency: Some("USD".to_string()),
            default_account: Some("Debtors - TC".to_string()),
            company: Some("_Test Company".to_string()),
            exists: false,
        },
        &mut BTreeSet::new(),
    );
    assert_eq!(new_customer.doctype, "Customer");
    assert_eq!(new_customer.insert, true);
    assert_eq!(new_customer.fields["customer_name"], "_Test Customer");
    assert_eq!(new_customer.fields["type"], "Individual");
    assert_eq!(new_customer.fields["default_currency"], "USD");
    assert_eq!(
        new_customer.child_rows["accounts"][0],
        BTreeMap::from([
            ("company".to_string(), "_Test Company".to_string()),
            ("account".to_string(), "Debtors - TC".to_string()),
        ])
    );
    assert_eq!(state.customer.as_deref(), Some("_Test Customer"));

    let existing_customer = state.create_customer(
        CustomerSeed {
            customer_name: "_Test Customer".to_string(),
            default_account: Some("Debtors 2 - TC".to_string()),
            company: Some("_Test Company".to_string()),
            exists: true,
            ..Default::default()
        },
        &mut BTreeSet::new(),
    );
    assert!(!existing_customer.insert);
    assert!(existing_customer.replace_child_table);
    assert_eq!(
        existing_customer.child_rows["accounts"][0]["account"],
        "Debtors 2 - TC"
    );

    let supplier = state.create_supplier(SupplierSeed {
        supplier_name: "_Test Supplier".to_string(),
        currency: Some("EUR".to_string()),
        exists: false,
    });
    assert_eq!(supplier.fields["supplier_type"], "Individual");
    assert_eq!(supplier.fields["supplier_group"], "Local");
    assert_eq!(supplier.fields["default_currency"], "EUR");
    assert_eq!(state.supplier.as_deref(), Some("_Test Supplier"));

    let item = state.create_item(
        "_Test Item",
        false,
        Some("Stores - TC"),
        Some("_Test Company"),
        12.5,
    );
    assert_eq!(item.item_name, "_Test Item");
    assert!(!item.is_stock_item);
    assert_eq!(item.warehouse.as_deref(), Some("Stores - TC"));
    assert_eq!(item.valuation_rate, 12.5);
    assert_eq!(state.item.as_deref(), Some("_Test Item"));
}

#[test]
fn accounts_mixin_company_creation_sets_standard_defaults_and_missing_accounts() {
    let mut state = AccountsMixinState::default();
    let existing_accounts = BTreeSet::from(["Deferred Revenue - _TC".to_string()]);

    let plan = state.create_company(
        CompanySeed {
            company_name: "_Test Company".to_string(),
            abbr: "_TC".to_string(),
            exists: false,
            cost_center: "Main - _TC".to_string(),
        },
        &existing_accounts,
        &[WarehouseSeed {
            name: "Stores - _TC".to_string(),
            warehouse_name: "Stores".to_string(),
        }],
    );

    assert!(plan.create_company);
    assert_eq!(plan.company_fields["country"], "India");
    assert_eq!(plan.company_fields["default_currency"], "INR");
    assert_eq!(state.company.as_deref(), Some("_Test Company"));
    assert_eq!(state.company_abbr.as_deref(), Some("_TC"));
    assert_eq!(state.warehouse.as_deref(), Some("Stores - _TC"));
    assert_eq!(
        state.finished_warehouse.as_deref(),
        Some("Finished Goods - _TC")
    );
    assert_eq!(state.income_account.as_deref(), Some("Sales - _TC"));
    assert_eq!(
        state.deferred_revenue.as_deref(),
        Some("Deferred Revenue - _TC")
    );

    assert_eq!(
        plan.accounts_to_create,
        vec![
            AccountSeed::new("Deferred Expense", "Current Assets - _TC", None),
            AccountSeed::new("HDFC", "Bank Accounts - _TC", Some("Bank")),
            AccountSeed::new(
                "Advance Received",
                "Current Liabilities - _TC",
                Some("Receivable")
            ),
            AccountSeed::new("Advance Paid", "Current Assets - _TC", Some("Payable")),
        ]
    );
    assert_eq!(state.dynamic_attributes["warehouse_stores"], "Stores - _TC");

    assert_eq!(default_company_accounts("_TC").cash, "Cash - _TC");
}

#[test]
fn accounts_mixin_advance_flags_and_usd_accounts_match_source_helpers() {
    let mut state = AccountsMixinState {
        company: Some("_Test Company".to_string()),
        company_abbr: Some("_TC".to_string()),
        advance_received: Some("Advance Received - _TC".to_string()),
        advance_paid: Some("Advance Paid - _TC".to_string()),
        ..Default::default()
    };

    let enabled = state.enable_advance_as_liability();
    assert!(enabled.book_advance_payments_in_separate_party_account);
    assert_eq!(
        enabled.default_advance_received_account.as_deref(),
        Some("Advance Received - _TC")
    );
    assert_eq!(
        enabled.default_advance_paid_account.as_deref(),
        Some("Advance Paid - _TC")
    );

    let disabled = state.disable_advance_as_liability();
    assert!(!disabled.book_advance_payments_in_separate_party_account);
    assert_eq!(disabled.default_advance_received_account, None);
    assert_eq!(disabled.default_advance_paid_account, None);

    let receivable = state.create_usd_receivable_account(None);
    assert!(receivable.create);
    assert_eq!(receivable.account_name, "Debtors USD");
    assert_eq!(receivable.parent_account, "Accounts Receivable - _TC");
    assert_eq!(receivable.account_currency, "USD");
    assert_eq!(receivable.account_type, "Receivable");
    assert_eq!(state.debtors_usd.as_deref(), Some("Debtors USD - _TC"));

    let payable = state.create_usd_payable_account(Some(ExistingAccount {
        name: "Creditors USD - _TC".to_string(),
    }));
    assert!(!payable.create);
    assert_eq!(payable.name, "Creditors USD - _TC");
    assert_eq!(state.creditors_usd.as_deref(), Some("Creditors USD - _TC"));
}

#[test]
fn accounts_mixin_attribute_and_cleanup_helpers_match_python_strings() {
    assert_eq!(
        warehouse_attribute_name("Finished Goods"),
        "warehouse_finished_goods"
    );
    assert_eq!(
        warehouse_attribute_name("  Raw   Material "),
        "warehouse_raw___material"
    );
    assert_eq!(
        account_attribute_name("Deferred Revenue"),
        "deferred_revenue"
    );
    assert_eq!(
        clear_old_entries_doctypes(),
        vec![
            "GL Entry",
            "Payment Ledger Entry",
            "Sales Invoice",
            "Purchase Invoice",
            "Payment Entry",
            "Journal Entry",
            "Sales Order",
            "Exchange Rate Revaluation",
            "Bank Account",
            "Bank Transaction",
        ]
    );
}
