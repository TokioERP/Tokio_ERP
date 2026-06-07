use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::doctype::account::account::{
    ensure_idle_system, get_account_autoname, get_account_currency,
    get_company_default_account_fields, get_parent_account, get_root_company, merge_account_plan,
    sync_update_account_number_in_child_plan, update_account_number_plan, Account, AccountContext,
    AccountError, AccountRecord, ChildCompanyAccountPlan, CompanyInfo, ConversionPlan,
    MergeAccountInput, ParentAccountInfo, RenamePlan, UpdateAccountNumberInput,
};
use tokio_erp::erpnext::accounts::utils::{get_balance_on_plan, AccountMeta, BalanceOnInput};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_account() -> Account {
    Account {
        name: "Sales - WP".to_string(),
        account_name: "Sales".to_string(),
        account_number: Some("4000".to_string()),
        is_group: false,
        company: "Wind Power LLC".to_string(),
        parent_account: Some("Income - WP".to_string()),
        root_type: Some("Income".to_string()),
        report_type: Some("Profit and Loss".to_string()),
        account_currency: Some("USD".to_string()),
        account_type: Some("Income Account".to_string()),
        freeze_account: "No".to_string(),
        balance_must_be: String::new(),
        ..Default::default()
    }
}

#[test]
fn account_idle_system_guard_matches_rename_and_merge_precondition() {
    assert_eq!(ensure_idle_system(true, None, false), Ok(()));
    assert_eq!(ensure_idle_system(false, None, false), Ok(()));
    assert_eq!(
        ensure_idle_system(false, Some(60), false),
        Err(AccountError::SystemInUse)
    );
    assert_eq!(
        ensure_idle_system(false, Some(60), true),
        Err(AccountError::SystemInUse)
    );
}

fn base_context() -> AccountContext {
    AccountContext {
        parent: Some(ParentAccountInfo {
            name: "Income - WP".to_string(),
            is_group: true,
            company: "Wind Power LLC".to_string(),
            account_type: None,
            report_type: Some("Profit and Loss".to_string()),
            root_type: Some("Income".to_string()),
            account_name: "Income".to_string(),
            account_number: None,
        }),
        company: Some(CompanyInfo {
            name: "Wind Power LLC".to_string(),
            abbr: "WP".to_string(),
            default_currency: "USD".to_string(),
            role_allowed_for_frozen_entries: Some("Accounts Manager".to_string()),
            allow_account_creation_against_child_company: false,
        }),
        user_roles: BTreeSet::from(["Accounts Manager".to_string()]),
        ..Default::default()
    }
}

#[test]
fn account_matches_erpnext_metadata_and_hooks() {
    assert_eq!(Account::DOCTYPE, "Account");
    assert_eq!(Account::MODULE, "Accounts");
    assert_eq!(Account::NSM_PARENT_FIELD, "parent_account");
    assert_eq!(Account::FIELD_ORDER.len(), 21);
    assert_eq!(
        &Account::FIELD_ORDER[..8],
        [
            "properties",
            "column_break0",
            "disabled",
            "account_name",
            "account_number",
            "is_group",
            "company",
            "root_type",
        ]
    );

    let fields = Account::fields();
    assert!(fields.contains(
        &FieldSpec::data("account_name", "Account Name")
            .in_list_view()
            .no_copy()
            .oldfield("account_name", "Data")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .fetch_from("parent_account.company")
            .fetch_if_empty()
            .in_standard_filter()
            .oldfield("company", "Link")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("parent_account", "Parent Account")
            .options("Account")
            .ignore_user_permissions()
            .oldfield("parent_account", "Link")
            .required()
            .search_index()
    ));

    let controller = Account::default();
    assert_eq!(controller.doctype(), "Account");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(
        controller.custom_hooks(),
        &["on_update", "onload", "autoname", "validate", "on_trash"]
    );
}

#[test]
fn account_validate_parent_root_and_report_type_match_erpnext() {
    let mut account = base_account();
    let mut ctx = base_context();
    assert!(account.validate(&mut ctx).is_ok());
    assert_eq!(account.report_type.as_deref(), Some("Profit and Loss"));
    assert_eq!(account.root_type.as_deref(), Some("Income"));

    ctx.parent = None;
    assert_eq!(
        base_account().validate(&mut ctx),
        Err(AccountError::ParentMissing {
            account: "Sales - WP".to_string(),
            parent: "Income - WP".to_string(),
        })
    );

    let mut root_ledger = Account {
        name: "Assets - WP".to_string(),
        account_name: "Assets".to_string(),
        company: "Wind Power LLC".to_string(),
        is_group: false,
        parent_account: None,
        ..Default::default()
    };
    assert_eq!(
        root_ledger.validate(&mut base_context()),
        Err(AccountError::RootMustBeGroup {
            account: "Assets - WP".to_string(),
        })
    );

    let mut same_type = base_account();
    same_type.account_type = Some("Direct Income".to_string());
    let mut same_type_ctx = base_context();
    same_type_ctx.parent.as_mut().unwrap().account_type = Some("Direct Income".to_string());
    assert_eq!(
        same_type.validate(&mut same_type_ctx),
        Err(AccountError::ParentSameType {
            account_type: "Direct Income".to_string(),
        })
    );
}

#[test]
fn account_validate_group_ledger_currency_freeze_and_balance_rules_match_erpnext() {
    let mut ctx = base_context();
    ctx.old_doc = Some(Account {
        is_group: false,
        disabled: false,
        freeze_account: "No".to_string(),
        account_type: Some("Receivable".to_string()),
        ..base_account()
    });
    ctx.gle_exists = true;

    let mut group_change = Account {
        is_group: true,
        ..base_account()
    };
    assert_eq!(
        group_change.validate(&mut ctx),
        Err(AccountError::CannotConvertTransactionAccountToLedger)
    );

    let mut frozen = base_account();
    frozen.freeze_account = "Yes".to_string();
    ctx.gle_exists = false;
    ctx.user_roles.clear();
    assert_eq!(
        frozen.validate(&mut ctx),
        Err(AccountError::UnauthorizedFrozenAccountModifier)
    );

    let mut credit_must = base_account();
    credit_must.balance_must_be = "Credit".to_string();
    ctx.user_roles.insert("Accounts Manager".to_string());
    ctx.balance = 10.0;
    assert_eq!(
        credit_must.validate(&mut ctx),
        Err(AccountError::DebitBalanceCannotBeCredit)
    );

    let mut currency = base_account();
    currency.account_currency = Some("EUR".to_string());
    ctx.balance = 0.0;
    ctx.gl_currency = Some("USD".to_string());
    ctx.any_gl_entry_exists = true;
    assert_eq!(
        currency.validate(&mut ctx),
        Err(AccountError::CurrencyChangeAfterEntries)
    );
}

#[test]
fn account_helpers_autoname_currency_defaults_and_search_match_erpnext() {
    assert_eq!(
        get_account_autoname(
            Some(" 4000 "),
            " Sales ",
            &CompanyInfo {
                name: "Wind Power LLC".to_string(),
                abbr: "WP".to_string(),
                ..Default::default()
            }
        )
        .unwrap(),
        "4000 - Sales - WP"
    );
    assert_eq!(
        get_account_currency(Some("Sales - WP"), Some("EUR"), "USD"),
        Some("EUR".to_string())
    );
    assert_eq!(get_account_currency(None, None, "USD"), None);

    let defaults = get_company_default_account_fields();
    assert_eq!(defaults["default_bank_account"], "Default Bank Account");
    assert_eq!(
        defaults["disposal_account"],
        "Gain/Loss Account on Asset Disposal"
    );

    assert_eq!(
        get_root_company(&["Parent Co".to_string(), "Grandparent".to_string()]),
        vec!["Parent Co"]
    );
    assert_eq!(
        get_parent_account(
            &[AccountRecord {
                name: "Income - WP".to_string(),
                company: "Wind Power LLC".to_string(),
                is_group: true,
                docstatus: 0,
                ..Default::default()
            }],
            "Wind Power LLC",
            "Inc",
            0,
            20,
        ),
        vec!["Income - WP".to_string()]
    );
}

#[test]
fn account_update_number_merge_and_child_sync_plans_match_erpnext() {
    let input = UpdateAccountNumberInput {
        name: "4000 - Sales - WP".to_string(),
        account_name: "Revenue".to_string(),
        account_number: Some("4100".to_string()),
        old_account_name: "Sales".to_string(),
        old_account_number: Some("4000".to_string()),
        company: CompanyInfo {
            name: "Wind Power LLC".to_string(),
            abbr: "WP".to_string(),
            ..Default::default()
        },
        descendants: vec!["Child LLC".to_string()],
        from_descendant: false,
        parent_company_accounts: BTreeMap::new(),
        allow_independent_account_creation: false,
        same_number_account: None,
    };
    let plan = update_account_number_plan(input).unwrap();
    assert_eq!(plan.new_name.as_deref(), Some("4100 - Revenue - WP"));
    assert_eq!(plan.account_number_value.as_deref(), Some("4100"));
    assert_eq!(plan.child_sync.len(), 1);

    assert_eq!(
        sync_update_account_number_in_child_plan(
            &["Child LLC".to_string()],
            "Sales",
            "Revenue",
            Some("4100"),
            Some("4000"),
        ),
        vec![RenamePlan {
            company: "Child LLC".to_string(),
            old_account_name: "Sales".to_string(),
            account_name: "Revenue".to_string(),
            account_number: Some("4100".to_string()),
            old_account_number: Some("4000".to_string()),
            from_descendant: true,
        }]
    );

    let merge = merge_account_plan(MergeAccountInput {
        old: AccountRecord {
            name: "Old - WP".to_string(),
            is_group: false,
            root_type: Some("Asset".to_string()),
            company: "Wind Power LLC".to_string(),
            account_currency: Some("USD".to_string()),
            ..Default::default()
        },
        new: AccountRecord {
            name: "New - WP".to_string(),
            is_group: false,
            root_type: Some("Asset".to_string()),
            company: "Wind Power LLC".to_string(),
            account_currency: Some("USD".to_string()),
            ..Default::default()
        },
    })
    .unwrap();
    assert_eq!(merge.rename_old, "Old - WP");
    assert_eq!(merge.merge_into, "New - WP");
}

#[test]
fn account_conversion_and_child_company_create_plans_match_erpnext() {
    assert_eq!(
        base_account()
            .convert_group_to_ledger_plan(false, false)
            .unwrap(),
        ConversionPlan {
            is_group: false,
            save: true,
        }
    );

    let mut ledger = base_account();
    ledger.account_type = None;
    assert_eq!(
        ledger.convert_ledger_to_group_plan(false, false).unwrap(),
        ConversionPlan {
            is_group: true,
            save: true,
        }
    );

    let mut account = base_account();
    account.currency_explicitly_specified = true;
    let plans = account
        .create_account_for_child_company_plan(
            &BTreeMap::from([("Child LLC".to_string(), "Income - CH".to_string())]),
            &["Child LLC".to_string()],
            "Income",
            &BTreeMap::from([("Income - CH".to_string(), true)]),
            &BTreeMap::new(),
            &BTreeMap::from([("Child LLC".to_string(), "UZS".to_string())]),
        )
        .unwrap();
    assert_eq!(
        plans,
        vec![ChildCompanyAccountPlan::Create {
            company: "Child LLC".to_string(),
            account_currency: "USD".to_string(),
            parent_account: "Income - CH".to_string(),
        }]
    );
}

#[test]
fn account_python_rename_account_regression_trims_number_and_name_like_erpnext() {
    let plan = update_account_number_plan(UpdateAccountNumberInput {
        name: "1210 - Debtors - _TC".to_string(),
        account_name: "Debtors 1 - Test - ".to_string(),
        account_number: Some("1211-11-4 - 6 - ".to_string()),
        old_account_name: "Debtors".to_string(),
        old_account_number: Some("1210".to_string()),
        company: CompanyInfo {
            name: "_Test Company".to_string(),
            abbr: "_TC".to_string(),
            ..Default::default()
        },
        descendants: vec![],
        from_descendant: false,
        parent_company_accounts: BTreeMap::new(),
        allow_independent_account_creation: false,
        same_number_account: None,
    })
    .unwrap();

    assert_eq!(
        plan.new_name.as_deref(),
        Some("1211-11-4 - 6 - - Debtors 1 - Test - - _TC")
    );
    assert_eq!(plan.account_name_value, "Debtors 1 - Test -");
    assert_eq!(
        plan.account_number_value.as_deref(),
        Some("1211-11-4 - 6 -")
    );
}

#[test]
fn account_python_merge_account_regression_checks_parent_group_root_and_currency() {
    let merge = merge_account_plan(MergeAccountInput {
        old: AccountRecord {
            name: "Securities and Deposits - _TC".to_string(),
            is_group: true,
            root_type: Some("Asset".to_string()),
            company: "_Test Company".to_string(),
            account_currency: Some("INR".to_string()),
            parent_account: Some("Current Assets - _TC".to_string()),
            ..Default::default()
        },
        new: AccountRecord {
            name: "Cash In Hand - _TC".to_string(),
            is_group: true,
            root_type: Some("Asset".to_string()),
            company: "_Test Company".to_string(),
            account_currency: Some("INR".to_string()),
            parent_account: Some("Securities and Deposits - _TC".to_string()),
            ..Default::default()
        },
    })
    .unwrap();
    assert_eq!(merge.rename_old, "Securities and Deposits - _TC");
    assert_eq!(merge.merge_into, "Cash In Hand - _TC");
    assert_eq!(
        merge.update_new_parent_account.as_deref(),
        Some("Current Assets - _TC")
    );

    for (old, new) in [
        (
            AccountRecord {
                name: "Current Assets - _TC".to_string(),
                is_group: true,
                root_type: Some("Asset".to_string()),
                account_currency: Some("INR".to_string()),
                company: "_Test Company".to_string(),
                ..Default::default()
            },
            AccountRecord {
                name: "Accumulated Depreciation - _TC".to_string(),
                is_group: false,
                root_type: Some("Asset".to_string()),
                account_currency: Some("INR".to_string()),
                company: "_Test Company".to_string(),
                ..Default::default()
            },
        ),
        (
            AccountRecord {
                name: "Capital Stock - _TC".to_string(),
                is_group: false,
                root_type: Some("Equity".to_string()),
                account_currency: Some("INR".to_string()),
                company: "_Test Company".to_string(),
                ..Default::default()
            },
            AccountRecord {
                name: "Software - _TC".to_string(),
                is_group: false,
                root_type: Some("Asset".to_string()),
                account_currency: Some("INR".to_string()),
                company: "_Test Company".to_string(),
                ..Default::default()
            },
        ),
        (
            AccountRecord {
                name: "Receivable INR - _TC".to_string(),
                is_group: false,
                root_type: Some("Asset".to_string()),
                account_currency: Some("INR".to_string()),
                company: "_Test Company".to_string(),
                ..Default::default()
            },
            AccountRecord {
                name: "Receivable USD - _TC".to_string(),
                is_group: false,
                root_type: Some("Asset".to_string()),
                account_currency: Some("USD".to_string()),
                company: "_Test Company".to_string(),
                ..Default::default()
            },
        ),
    ] {
        assert_eq!(
            merge_account_plan(MergeAccountInput { old, new }),
            Err(AccountError::InvalidAccountMerge)
        );
    }
}

#[test]
fn account_python_child_sync_rename_currency_and_percent_balance_regressions_match_erpnext() {
    let mut parent = Account {
        name: "Test Sync Account - _TC3".to_string(),
        account_name: "Test Sync Account".to_string(),
        parent_account: Some("Temporary Accounts - _TC3".to_string()),
        company: "_Test Company 3".to_string(),
        account_currency: Some("INR".to_string()),
        ..base_account()
    };
    parent.currency_explicitly_specified = false;
    let create_plans = parent
        .create_account_for_child_company_plan(
            &BTreeMap::from([
                (
                    "_Test Company 4".to_string(),
                    "Temporary Accounts - _TC4".to_string(),
                ),
                (
                    "_Test Company 5".to_string(),
                    "Temporary Accounts - _TC5".to_string(),
                ),
            ]),
            &["_Test Company 4".to_string(), "_Test Company 5".to_string()],
            "Temporary Accounts",
            &BTreeMap::from([
                ("Temporary Accounts - _TC4".to_string(), true),
                ("Temporary Accounts - _TC5".to_string(), true),
            ]),
            &BTreeMap::new(),
            &BTreeMap::from([
                ("_Test Company 4".to_string(), "USD".to_string()),
                ("_Test Company 5".to_string(), "USD".to_string()),
            ]),
        )
        .unwrap();
    assert_eq!(
        create_plans,
        vec![
            ChildCompanyAccountPlan::Create {
                company: "_Test Company 4".to_string(),
                account_currency: "USD".to_string(),
                parent_account: "Temporary Accounts - _TC4".to_string(),
            },
            ChildCompanyAccountPlan::Create {
                company: "_Test Company 5".to_string(),
                account_currency: "USD".to_string(),
                parent_account: "Temporary Accounts - _TC5".to_string(),
            },
        ]
    );

    let blocked_child_rename = update_account_number_plan(UpdateAccountNumberInput {
        name: "Test Group Account - _TC5".to_string(),
        account_name: "Test Modified Account".to_string(),
        company: CompanyInfo {
            name: "_Test Company 5".to_string(),
            abbr: "_TC5".to_string(),
            ..Default::default()
        },
        parent_company_accounts: BTreeMap::from([(
            "_Test Company 3".to_string(),
            "Test Group Account - _TC3".to_string(),
        )]),
        ..Default::default()
    });
    assert_eq!(
        blocked_child_rename,
        Err(AccountError::RenameNotAllowed {
            account: "Test Group Account - _TC3".to_string(),
            parent_company: "_Test Company 3".to_string(),
        })
    );

    let allowed_child_rename = update_account_number_plan(UpdateAccountNumberInput {
        allow_independent_account_creation: true,
        parent_company_accounts: BTreeMap::from([(
            "_Test Company 3".to_string(),
            "Test Group Account - _TC3".to_string(),
        )]),
        ..UpdateAccountNumberInput {
            name: "Test Group Account - _TC5".to_string(),
            account_name: "Test Modified Account".to_string(),
            company: CompanyInfo {
                name: "_Test Company 5".to_string(),
                abbr: "_TC5".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .unwrap();
    assert_eq!(
        allowed_child_rename.new_name.as_deref(),
        Some("Test Modified Account - _TC5")
    );

    let mut currency_account = base_account();
    currency_account.name = "Test Currency Account - _TC".to_string();
    currency_account.account_currency = Some("USD".to_string());
    let mut currency_ctx = base_context();
    currency_ctx.gl_currency = Some("INR".to_string());
    currency_ctx.any_gl_entry_exists = true;
    assert_eq!(
        currency_account.validate_account_currency(&currency_ctx),
        Err(AccountError::CurrencyChangeAfterEntries)
    );

    let balance_plan = get_balance_on_plan(BalanceOnInput {
        account_name: Some("Test Percent Account %5 - _TC".to_string()),
        date: Some("2026-06-07".to_string()),
        party_type: None,
        party: None,
        company: None,
        in_account_currency: true,
        cost_center: None,
        ignore_account_permission: false,
        account_type: None,
        start_date: None,
        finance_book: None,
        include_default_fb_balances: false,
        default_finance_book: None,
        company_default_currency: None,
        account_meta: Some(AccountMeta {
            name: "Test Percent Account %5 - _TC".to_string(),
            report_type: "Balance Sheet".to_string(),
            is_group: false,
            account_currency: "INR".to_string(),
            company: "_Test Company".to_string(),
            lft: 1,
            rgt: 2,
        }),
        cost_center_meta: None,
        account_type_accounts: vec![],
        today: "2026-06-07".to_string(),
    });
    assert!(balance_plan
        .conditions
        .contains(&"gle.account = 'Test Percent Account %5 - _TC'".to_string()));
    assert_eq!(
        balance_plan.select_field.as_deref(),
        Some(
            "sum(round(debit_in_account_currency, p)) - sum(round(credit_in_account_currency, p))"
        )
    );
}
