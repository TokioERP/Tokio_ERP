use tokio_erp::erpnext::accounts::doctype::gl_entry::gl_entry::{
    rename_temporarily_named_docs, update_against_account, update_outstanding_amount,
    validate_balance_type, validate_frozen_account, AccountDetails, AgainstAccountUpdate,
    CostCenterDetails, DimensionCheck, GlEntry, GlEntryContext, GlEntryError, GlEntryLedgerRow,
    OutstandingInput, OutstandingUpdate, RenamePlan, TemporaryRenameRow, TemporaryRenameUpdate,
};
use tokio_erp::erpnext::accounts::general_ledger::{
    process_debit_credit_difference, GlEntry as LedgerGlEntry, RoundOffSettings,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_entry() -> GlEntry {
    GlEntry {
        account: Some("Debtors - TC".to_string()),
        company: Some("_Test Company".to_string()),
        voucher_type: Some("Sales Invoice".to_string()),
        voucher_no: Some("SINV-0001".to_string()),
        posting_date: Some("2026-05-23".to_string()),
        debit: 100.0,
        debit_in_account_currency: 100.0,
        account_currency: Some("USD".to_string()),
        ..GlEntry::default()
    }
}

fn base_context() -> GlEntryContext {
    GlEntryContext {
        fiscal_year: Some("2026".to_string()),
        company_default_currency: Some("USD".to_string()),
        company_reporting_currency: Some("UZS".to_string()),
        reporting_currency_exchange_rate: Some(12_500.0),
        account_currency: Some("USD".to_string()),
        account: Some(AccountDetails {
            is_group: false,
            docstatus: 0,
            company: "_Test Company".to_string(),
            account_type: Some("Receivable".to_string()),
            report_type: Some("Balance Sheet".to_string()),
        }),
        cost_center: Some(CostCenterDetails {
            is_group: false,
            company: "_Test Company".to_string(),
        }),
        ..GlEntryContext::default()
    }
}

#[test]
fn gl_entry_metadata_and_autoname_match_erpnext() {
    assert_eq!(GlEntry::DOCTYPE, "GL Entry");
    assert_eq!(GlEntry::MODULE, "Accounts");
    assert_eq!(GlEntry::AUTONAME, "ACC-GLE-.YYYY.-.#####");
    assert_eq!(GlEntry::FIELD_ORDER.len(), 47);
    assert_eq!(GlEntry::FIELD_ORDER[0], "dates_section");
    assert_eq!(GlEntry::FIELD_ORDER[46], "remarks");

    let doc = GlEntry::default();
    assert_eq!(doc.doctype(), "GL Entry");
    assert_eq!(doc.module(), "Accounts");
    assert!(GlEntry::fields().contains(
        &FieldSpec::link("account", "Account")
            .options("Account")
            .in_filter()
            .in_list_view()
            .in_standard_filter()
            .search_index()
    ));
    assert!(GlEntry::fields().contains(
        &FieldSpec::currency("debit", "Debit Amount").options("Company:company:default_currency")
    ));

    let mut entry = GlEntry::default();
    entry.autoname("hash000001", "ACC-GLE-.YYYY.-.#####");
    assert_eq!(entry.name.as_deref(), Some("hash000001"));
    assert!(entry.to_rename);

    entry.autoname("hash000002", "hash");
    assert_eq!(entry.name.as_deref(), Some("hash000002"));
    assert!(!entry.to_rename);
}

#[test]
fn gl_entry_core_validate_matches_mandatory_party_pl_and_reporting_currency_rules() {
    let mut missing_party = base_entry();
    let ctx = base_context();
    assert_eq!(
        missing_party.validate_core(&ctx).unwrap_err(),
        GlEntryError::Validation(
            "Sales Invoice SINV-0001: Customer is required against Receivable account Debtors - TC"
                .to_string()
        )
    );

    missing_party.party_type = Some("Customer".to_string());
    missing_party.party = Some("CUST-0001".to_string());
    missing_party.validate_core(&ctx).unwrap();
    assert_eq!(missing_party.fiscal_year.as_deref(), Some("2026"));
    assert_eq!(missing_party.reporting_currency_exchange_rate, 12_500.0);
    assert_eq!(missing_party.debit_in_reporting_currency, 1_250_000.0);

    let mut pl_entry = GlEntry {
        account: Some("Sales - TC".to_string()),
        voucher_type: Some("Sales Invoice".to_string()),
        voucher_no: Some("SINV-0002".to_string()),
        company: Some("_Test Company".to_string()),
        debit: 10.0,
        posting_date: Some("2026-05-23".to_string()),
        ..GlEntry::default()
    };
    let mut pl_ctx = base_context();
    pl_ctx.account.as_mut().unwrap().account_type = None;
    pl_ctx.account.as_mut().unwrap().report_type = Some("Profit and Loss".to_string());
    assert_eq!(
        pl_entry.validate_core(&pl_ctx).unwrap_err(),
        GlEntryError::Validation(
            "Sales Invoice SINV-0002: Cost Center is required for 'Profit and Loss' account Sales - TC. Please set the cost center field in Sales Invoice or setup a default Cost Center for the Company."
                .to_string()
        )
    );
}

#[test]
fn gl_entry_account_cost_center_and_currency_validations_match_erpnext() {
    let mut entry = base_entry();
    entry.party_type = Some("Customer".to_string());
    entry.party = Some("CUST-0001".to_string());

    let mut ctx = base_context();
    ctx.account.as_mut().unwrap().is_group = true;
    assert_eq!(
        entry.validate_account_details(&ctx, false).unwrap_err(),
        GlEntryError::Validation(
            "Sales Invoice SINV-0001: Account Debtors - TC is a Group Account and group accounts cannot be used in transactions"
                .to_string()
        )
    );

    ctx = base_context();
    entry.cost_center = Some("Group CC - TC".to_string());
    ctx.cost_center = Some(CostCenterDetails {
        is_group: true,
        company: "_Test Company".to_string(),
    });
    assert_eq!(
        entry.validate_cost_center(&ctx).unwrap_err(),
        GlEntryError::Validation(
            "Sales Invoice SINV-0001: Cost Center <b>Group CC - TC</b> is a group cost center and group cost centers cannot be used in transactions"
                .to_string()
        )
    );

    ctx = base_context();
    entry.cost_center = None;
    entry.account_currency = Some("EUR".to_string());
    assert_eq!(
        entry.validate_currency(&ctx).unwrap_err(),
        GlEntryError::InvalidAccountCurrency(
            "Sales Invoice SINV-0001: Accounting Entry for Debtors - TC can only be made in currency: USD"
                .to_string()
        )
    );
}

#[test]
fn gl_entry_balance_frozen_and_outstanding_helpers_match_erpnext() {
    assert_eq!(
        validate_balance_type("Cash - TC", false, Some("Debit"), -1.0).unwrap_err(),
        GlEntryError::Validation("Balance for Account Cash - TC must always be Debit".to_string())
    );
    assert_eq!(
        validate_frozen_account("Debtors - TC", false, Some("Yes"), None, &[]).unwrap_err(),
        GlEntryError::Validation("Account Debtors - TC is frozen".to_string())
    );
    assert_eq!(
        validate_frozen_account(
            "Debtors - TC",
            false,
            Some("Yes"),
            Some("Accounts Manager"),
            &["Accounts User".to_string()]
        )
        .unwrap_err(),
        GlEntryError::Validation("Not authorized to edit frozen Account Debtors - TC".to_string())
    );

    assert_eq!(
        update_outstanding_amount(&OutstandingInput {
            account: "Creditors - TC".to_string(),
            against_voucher_type: "Purchase Invoice".to_string(),
            against_voucher: "PINV-0001".to_string(),
            gl_balance: -250.0,
            journal_entry_unadjusted_amount: None,
            on_cancel: false,
        })
        .unwrap(),
        OutstandingUpdate {
            doctype: "Purchase Invoice".to_string(),
            name: "PINV-0001".to_string(),
            outstanding_amount: 250.0,
            set_status_update: true,
        }
    );

    assert_eq!(
        update_outstanding_amount(&OutstandingInput {
            account: "Debtors - TC".to_string(),
            against_voucher_type: "Journal Entry".to_string(),
            against_voucher: "JV-0001".to_string(),
            gl_balance: -80.0,
            journal_entry_unadjusted_amount: Some(100.0),
            on_cancel: false,
        })
        .unwrap(),
        OutstandingUpdate {
            doctype: "Journal Entry".to_string(),
            name: "JV-0001".to_string(),
            outstanding_amount: 20.0,
            set_status_update: true,
        }
    );
}

#[test]
fn gl_entry_update_against_account_matches_erpnext_debit_credit_sets() {
    let updates = update_against_account(&[
        GlEntryLedgerRow::new("GLE-1", Some("Customer A"), 100.0, 0.0, "Debtors - TC"),
        GlEntryLedgerRow::new("GLE-2", None, 0.0, 60.0, "Sales - TC"),
        GlEntryLedgerRow::new("GLE-3", Some("Tax Office"), 0.0, 40.0, "Tax - TC"),
    ]);

    assert_eq!(
        updates,
        vec![
            AgainstAccountUpdate::new("GLE-1", "Sales - TC, Tax Office"),
            AgainstAccountUpdate::new("GLE-2", "Customer A"),
            AgainstAccountUpdate::new("GLE-3", "Customer A"),
        ]
    );
}

#[test]
fn gl_entry_dimension_cancel_index_and_rename_plans_match_erpnext() {
    let mut entry = base_entry();
    entry.party_type = Some("Customer".to_string());
    entry.party = Some("CUST-0001".to_string());

    let mut ctx = base_context();
    ctx.account.as_mut().unwrap().report_type = Some("Profit and Loss".to_string());
    ctx.dimensions = vec![DimensionCheck {
        company: "_Test Company".to_string(),
        fieldname: "department".to_string(),
        label: "Department".to_string(),
        mandatory_for_pl: true,
        mandatory_for_bs: false,
        value: None,
    }];
    assert_eq!(
        entry.validate_dimensions_for_pl_and_bs(&ctx).unwrap_err(),
        GlEntryError::Validation(
            "Accounting Dimension <b>Department</b> is required for 'Profit and Loss' account Debtors - TC."
                .to_string()
        )
    );

    assert_eq!(
        entry.on_cancel_error(),
        GlEntryError::Validation(
            "Individual GL Entry cannot be cancelled.<br>Please cancel related transaction."
                .to_string()
        )
    );
    assert_eq!(
        GlEntry::doctype_update_indexes(),
        vec![
            vec!["voucher_type", "voucher_no"],
            vec!["posting_date", "company"],
            vec!["party_type", "party"],
        ]
    );
    assert_eq!(
        GlEntry::rename_gle_sle_doctypes(),
        ["GL Entry", "Stock Ledger Entry"]
    );
    assert_eq!(
        GlEntry::rename_temporarily_named_docs_plan("GL Entry", "ACC-GLE-.YYYY.-.#####"),
        RenamePlan {
            doctype: "GL Entry",
            filters: vec![("to_rename", "1")],
            order_by: "creation",
            limit: 50_000,
            batch_size: 100,
            autoname: "ACC-GLE-.YYYY.-.#####",
            hooks: vec!["on_gle_rename", "on_sle_rename"],
        }
    );
}

#[test]
fn gl_entry_python_round_off_entry_regression_matches_erpnext() {
    let mut gl_map = vec![
        LedgerGlEntry {
            company: "_Test Company".to_string(),
            account: "_Test Account Cost for Goods Sold - _TC".to_string(),
            posting_date: "2026-05-23".to_string(),
            voucher_type: "Journal Entry".to_string(),
            voucher_no: "ACC-JV-0001".to_string(),
            cost_center: Some("_Test Cost Center - _TC".to_string()),
            debit: 100.01,
            debit_in_account_currency: 100.01,
            debit_in_transaction_currency: 100.01,
            ..LedgerGlEntry::default()
        },
        LedgerGlEntry {
            company: "_Test Company".to_string(),
            account: "_Test Bank - _TC".to_string(),
            posting_date: "2026-05-23".to_string(),
            voucher_type: "Journal Entry".to_string(),
            voucher_no: "ACC-JV-0001".to_string(),
            cost_center: Some("_Test Cost Center - _TC".to_string()),
            credit: 100.0,
            credit_in_account_currency: 100.0,
            credit_in_transaction_currency: 100.0,
            ..LedgerGlEntry::default()
        },
    ];

    process_debit_credit_difference(
        &mut gl_map,
        2,
        RoundOffSettings {
            round_off_account: Some("_Test Write Off - _TC".to_string()),
            round_off_cost_center: Some("_Test Cost Center - _TC".to_string()),
            round_off_for_opening: None,
            default_expense_account: None,
        },
        false,
    )
    .unwrap();

    let round_off_entry = gl_map
        .iter()
        .find(|entry| {
            entry.voucher_type == "Journal Entry"
                && entry.voucher_no == "ACC-JV-0001"
                && entry.account == "_Test Write Off - _TC"
                && entry.cost_center.as_deref() == Some("_Test Cost Center - _TC")
        })
        .expect("round-off GL Entry");

    assert_eq!((round_off_entry.debit, round_off_entry.credit), (0.0, 0.01));
}

#[test]
fn gl_entry_python_rename_entries_regression_updates_names_flags_and_series() {
    let rows = vec![
        TemporaryRenameRow::new("tmp-gle-a", true),
        TemporaryRenameRow::new("tmp-gle-b", true),
    ];

    let plan =
        rename_temporarily_named_docs("GL Entry", &rows, "ACC-GLE-.YYYY.-.#####", 27, "2026");

    assert_eq!(
        plan.updates,
        vec![
            TemporaryRenameUpdate {
                old_name: "tmp-gle-a".to_string(),
                new_name: "ACC-GLE-2026-00028".to_string(),
                to_rename: false,
            },
            TemporaryRenameUpdate {
                old_name: "tmp-gle-b".to_string(),
                new_name: "ACC-GLE-2026-00029".to_string(),
                to_rename: false,
            },
        ]
    );
    assert_eq!(plan.series_current_value, 29);
    assert!(plan
        .updates
        .iter()
        .all(|update| update.old_name != update.new_name));
}

#[test]
fn gl_entry_python_party_type_regression_rejects_non_receivable_payable_and_allows_equity() {
    let mut entry = GlEntry {
        account: Some("_Test Account Cost for Goods Sold - _TC".to_string()),
        company: Some("_Test Company".to_string()),
        voucher_type: Some("Journal Entry".to_string()),
        voucher_no: Some("ACC-JV-0002".to_string()),
        posting_date: Some("2026-05-23".to_string()),
        cost_center: Some("_Test Cost Center - _TC".to_string()),
        debit: 100.0,
        debit_in_account_currency: 100.0,
        party_type: Some("Supplier".to_string()),
        party: Some("_Test Supplier".to_string()),
        ..GlEntry::default()
    };
    let mut ctx = base_context();
    ctx.account.as_mut().unwrap().account_type = Some("Expense Account".to_string());
    ctx.account.as_mut().unwrap().report_type = Some("Profit and Loss".to_string());

    assert_eq!(
        entry.validate_core(&ctx).unwrap_err(),
        GlEntryError::Validation(
            "Party Type and Party can only be set for Receivable / Payable account<br><br>_Test Account Cost for Goods Sold - _TC"
                .to_string()
        )
    );

    let mut shareholder = GlEntry {
        account: Some("Opening Balance Equity - _TC".to_string()),
        company: Some("_Test Company".to_string()),
        voucher_type: Some("Journal Entry".to_string()),
        voucher_no: Some("ACC-JV-0003".to_string()),
        posting_date: Some("2026-05-23".to_string()),
        credit: 100.0,
        credit_in_account_currency: 100.0,
        party_type: Some("Shareholder".to_string()),
        party: Some("_Test Shareholder".to_string()),
        ..GlEntry::default()
    };
    let mut shareholder_ctx = base_context();
    shareholder_ctx.account.as_mut().unwrap().account_type = Some("Equity".to_string());
    shareholder_ctx.account.as_mut().unwrap().report_type = Some("Balance Sheet".to_string());

    shareholder.validate_core(&shareholder_ctx).unwrap();
}
