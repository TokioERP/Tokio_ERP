use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::utils::{
    build_dimensions_dict_for_exc_gain_loss, compare_existing_and_expected_gle, convert_to_list,
    get_advance_ledger_entry, get_autoname_with_number, get_balance_on_plan,
    get_currency_precision, get_fiscal_year, get_fiscal_year_filter_field, get_fiscal_years,
    get_future_stock_vouchers, get_journal_entry, get_reconciliation_effect_date,
    get_voucherwise_gl_entries, get_zero_cutoff, parse_naming_series_variable,
    sort_stock_vouchers_by_posting_date, update_voucher_outstanding_plan,
    validate_allocated_amount, AccountMeta, AdvanceLedgerEntry, AllocatedAmountArgs,
    AllocatedAmountError, BalanceOnInput, CostCenterMeta, FiscalYearError, FiscalYearRecord,
    GlEntryLike, NamingSeriesContext, NamingSeriesDoc, ReconciliationEffectInput, StockGlEntry,
    StockLedgerEntryRecord, UpdateVoucherOutstandingPlan, GL_REPOSTING_CHUNK, OUTSTANDING_DOCTYPES,
};

#[test]
fn fiscal_year_helpers_match_erpnext_selection_and_filter_options() {
    assert_eq!(GL_REPOSTING_CHUNK, 100);
    assert_eq!(
        OUTSTANDING_DOCTYPES,
        ["Fees", "Purchase Invoice", "Sales Invoice"]
    );

    let years = vec![
        FiscalYearRecord {
            name: "2025-2026".to_string(),
            year_start_date: "2025-04-01".to_string(),
            year_end_date: "2026-03-31".to_string(),
            disabled: false,
            companies: vec![],
        },
        FiscalYearRecord {
            name: "2026-2027".to_string(),
            year_start_date: "2026-04-01".to_string(),
            year_end_date: "2027-03-31".to_string(),
            disabled: false,
            companies: vec!["Acme".to_string()],
        },
        FiscalYearRecord {
            name: "2024-2025".to_string(),
            year_start_date: "2024-04-01".to_string(),
            year_end_date: "2025-03-31".to_string(),
            disabled: true,
            companies: vec![],
        },
    ];

    let selected = get_fiscal_years(Some("2026-05-01"), None, Some("Acme"), true, &years).unwrap();
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].name, "2026-2027");
    assert_eq!(
        get_fiscal_year(None, None, Some("Acme"), true, false, &years)
            .unwrap()
            .unwrap()
            .name,
        "2026-2027"
    );
    assert_eq!(
        get_fiscal_year(Some("2026-05-01"), None, Some("Acme"), false, true, &years)
            .unwrap()
            .unwrap()
            .name,
        "26-27"
    );

    assert_eq!(
        get_fiscal_years(Some("2026-05-01"), None, Some("Beta"), false, &years),
        Ok(vec![])
    );
    assert_eq!(
        get_fiscal_years(Some("2023-01-01"), None, None, true, &years),
        Err(FiscalYearError::Missing)
    );

    let field = get_fiscal_year_filter_field(Some("Acme"), &years);
    assert_eq!(field.fieldtype, "Select");
    assert_eq!(field.operator, "Between");
    assert!(field.query_value);
    assert_eq!(field.options[0].label, "2026-2027");
    assert_eq!(
        field.options[0].query_value,
        ["2026-04-01".to_string(), "2027-03-31".to_string()]
    );
}

#[test]
fn balance_on_plan_matches_erpnext_condition_and_currency_branches() {
    let plan = get_balance_on_plan(BalanceOnInput {
        account_name: Some("Income - A".to_string()),
        date: Some("2026-06-06".to_string()),
        party_type: Some("Customer".to_string()),
        party: Some("CUST-1".to_string()),
        company: Some("Acme".to_string()),
        in_account_currency: true,
        cost_center: Some("Retail - A".to_string()),
        ignore_account_permission: false,
        account_type: None,
        start_date: Some("2026-04-01".to_string()),
        finance_book: Some("IFRS".to_string()),
        include_default_fb_balances: true,
        default_finance_book: Some("Local".to_string()),
        company_default_currency: Some("USD".to_string()),
        account_meta: Some(AccountMeta {
            name: "Income - A".to_string(),
            report_type: "Profit and Loss".to_string(),
            is_group: true,
            lft: 10,
            rgt: 25,
            account_currency: "USD".to_string(),
            company: "Acme".to_string(),
        }),
        cost_center_meta: Some(CostCenterMeta {
            name: "Retail - A".to_string(),
            is_group: true,
            lft: 3,
            rgt: 8,
        }),
        account_type_accounts: vec![],
        today: "2026-06-06".to_string(),
    });

    assert_eq!(
        plan.conditions,
        vec![
            "is_cancelled=0",
            "posting_date >= '2026-04-01'",
            "posting_date <= '2026-06-06'",
            "exists (select 1 from `tabCost Center` cc where cc.name = gle.cost_center and cc.lft >= 3 and cc.rgt <= 8)",
            "exists (select name from `tabAccount` ac where ac.name = gle.account and ac.lft >= 10 and ac.rgt <= 25)",
            "gle.party_type = 'Customer' and gle.party = 'CUST-1'",
            "gle.company = 'Acme'",
            "(gle.finance_book IN ('IFRS', 'Local') OR gle.finance_book IS NULL)",
        ]
    );
    assert_eq!(
        plan.select_field,
        Some("sum(round(debit, p)) - sum(round(credit, p))".to_string())
    );
    assert_eq!(plan.effective_date, "2026-06-06");
    assert!(!plan.in_account_currency);
}

#[test]
fn reconciliation_validation_currency_and_name_helpers_match_erpnext() {
    let mut entry = BTreeMap::new();
    entry.insert("project".to_string(), "PROJ-1".to_string());
    entry.insert("branch".to_string(), "".to_string());
    entry.insert("cost_center".to_string(), "CC-1".to_string());
    assert_eq!(
        build_dimensions_dict_for_exc_gain_loss(&entry, &["project", "branch", "cost_center"]),
        BTreeMap::from([
            ("cost_center".to_string(), "CC-1".to_string()),
            ("project".to_string(), "PROJ-1".to_string()),
        ])
    );

    assert_eq!(
        validate_allocated_amount(AllocatedAmountArgs {
            allocated_amount: -1.0,
            unadjusted_amount: 10.0,
            precision: Some(2),
        }),
        Err(AllocatedAmountError::Negative)
    );
    assert_eq!(
        validate_allocated_amount(AllocatedAmountArgs {
            allocated_amount: 10.004,
            unadjusted_amount: 10.004,
            precision: Some(2),
        }),
        Ok(())
    );
    assert_eq!(
        validate_allocated_amount(AllocatedAmountArgs {
            allocated_amount: 10.02,
            unadjusted_amount: 10.01,
            precision: Some(2),
        }),
        Err(AllocatedAmountError::GreaterThanUnadjusted)
    );

    assert_eq!(
        get_reconciliation_effect_date(ReconciliationEffectInput {
            reconciliation_takes_effect_on: Some("Oldest Of Invoice Or Advance".to_string()),
            against_voucher_type: "Sales Order".to_string(),
            against_voucher_date: Some("2026-05-30".to_string()),
            posting_date: "2026-06-06".to_string(),
            today: "2026-06-10".to_string(),
        }),
        "2026-06-06"
    );
    assert_eq!(
        get_reconciliation_effect_date(ReconciliationEffectInput {
            reconciliation_takes_effect_on: Some("Reconciliation Date".to_string()),
            against_voucher_type: "Sales Invoice".to_string(),
            against_voucher_date: Some("2026-06-01".to_string()),
            posting_date: "2026-06-06".to_string(),
            today: "2026-06-10".to_string(),
        }),
        "2026-06-10"
    );

    assert_eq!(
        convert_to_list(&[
            vec!["A".to_string(), "ignored".to_string()],
            vec!["B".to_string()]
        ]),
        vec!["A".to_string(), "B".to_string()]
    );
    assert_eq!(get_currency_precision(Some(3), "#,###.##"), 3);
    assert_eq!(get_currency_precision(None, "#,###.000"), 3);
    assert_eq!(get_currency_precision(None, "#,###.##"), 2);
    assert_eq!(get_zero_cutoff(None), 0.005);
    assert_eq!(get_zero_cutoff(Some(0)), 0.5);
    assert_eq!(get_zero_cutoff(Some(1000)), 0.0005);
    assert_eq!(
        get_autoname_with_number(Some(" 100 "), " Main Cost Center ", "TC"),
        "100 - Main Cost Center - TC"
    );
    assert_eq!(
        get_autoname_with_number(Some(" "), " Main Cost Center ", "TC"),
        "Main Cost Center - TC"
    );
}

#[test]
fn gl_comparison_stock_journal_and_ledger_plans_match_erpnext() {
    let existing = vec![GlEntryLike {
        account: "Stock - A".to_string(),
        cost_center: Some("Main".to_string()),
        debit: 10.004,
        credit: 0.0,
    }];
    let expected = vec![GlEntryLike {
        account: "Stock - A".to_string(),
        cost_center: Some("Main".to_string()),
        debit: 10.004,
        credit: 0.0,
    }];
    assert!(compare_existing_and_expected_gle(&existing, &expected, 2));

    let different_cost_center = vec![GlEntryLike {
        account: "Stock - A".to_string(),
        cost_center: Some("Other".to_string()),
        debit: 99.0,
        credit: 0.0,
    }];
    assert!(compare_existing_and_expected_gle(
        &different_cost_center,
        &expected,
        2
    ));

    assert!(!compare_existing_and_expected_gle(
        &existing,
        &[GlEntryLike {
            account: "Missing - A".to_string(),
            cost_center: None,
            debit: 10.0,
            credit: 0.0,
        }],
        2
    ));

    let negative = get_journal_entry("Stock - A", "Adjustment - A", -25.0);
    assert_eq!(negative.accounts[0].account, "Stock - A");
    assert_eq!(negative.accounts[0].credit_in_account_currency, Some(25.0));
    assert_eq!(negative.accounts[1].debit_in_account_currency, Some(25.0));

    assert_eq!(
        get_advance_ledger_entry(
            AdvanceLedgerEntry {
                company: "Acme".to_string(),
                voucher_type: "Payment Entry".to_string(),
                voucher_no: "PE-1".to_string(),
                voucher_detail_no: "row-1".to_string(),
                advance_voucher_type: "Sales Order".to_string(),
                advance_voucher_no: "SO-1".to_string(),
                amount: 50.0,
                currency: "USD".to_string(),
                cancel: false,
                base_amount: Some(60.0),
                exchange_rate: Some(1.2),
            },
            "Payment Entry",
            "PE-1"
        )
        .event,
        "Submit"
    );

    assert_eq!(
        update_voucher_outstanding_plan(
            "Payment Entry",
            "PE-1",
            None,
            None,
            None,
            &["Payment Entry", "Journal Entry"]
        ),
        UpdateVoucherOutstandingPlan::AdvancePayment {
            voucher_type: "Payment Entry".to_string(),
            voucher_no: "PE-1".to_string(),
        }
    );
    assert_eq!(
        update_voucher_outstanding_plan(
            "Sales Invoice",
            "SI-1",
            Some("Debtors - A"),
            Some("Customer"),
            Some("CUST-1"),
            &[]
        ),
        UpdateVoucherOutstandingPlan::OutstandingRecompute {
            voucher_type: "Sales Invoice".to_string(),
            voucher_no: "SI-1".to_string(),
            account: Some("Debtors - A".to_string()),
            party_type: "Customer".to_string(),
            party: "CUST-1".to_string(),
        }
    );
    assert_eq!(
        update_voucher_outstanding_plan("Sales Invoice", "", None, None, None, &[]),
        UpdateVoucherOutstandingPlan::Noop
    );
}

#[test]
fn stock_voucher_helpers_match_erpnext_test_utils_regressions() {
    let sles = vec![
        StockLedgerEntryRecord {
            voucher_type: "Stock Entry".to_string(),
            voucher_no: "SE-3".to_string(),
            posting_date: "2022-03-01".to_string(),
            posting_time: "00:00:00".to_string(),
            creation: "2022-03-01 00:00:03".to_string(),
            item_code: "_Test Item".to_string(),
            warehouse: "_Test Warehouse - _TC".to_string(),
            company: "_Test Company".to_string(),
            is_cancelled: false,
        },
        StockLedgerEntryRecord {
            voucher_type: "Stock Entry".to_string(),
            voucher_no: "SE-1".to_string(),
            posting_date: "2022-01-01".to_string(),
            posting_time: "00:00:00".to_string(),
            creation: "2022-01-01 00:00:01".to_string(),
            item_code: "_Test Item".to_string(),
            warehouse: "_Test Warehouse - _TC".to_string(),
            company: "_Test Company".to_string(),
            is_cancelled: false,
        },
        StockLedgerEntryRecord {
            voucher_type: "Stock Entry".to_string(),
            voucher_no: "SE-2".to_string(),
            posting_date: "2022-02-01".to_string(),
            posting_time: "00:00:00".to_string(),
            creation: "2022-02-01 00:00:02".to_string(),
            item_code: "_Test Item".to_string(),
            warehouse: "_Test Warehouse - _TC".to_string(),
            company: "_Test Company".to_string(),
            is_cancelled: false,
        },
        StockLedgerEntryRecord {
            voucher_type: "Stock Entry".to_string(),
            voucher_no: "SE-CANCELLED".to_string(),
            posting_date: "2022-01-15".to_string(),
            posting_time: "00:00:00".to_string(),
            creation: "2022-01-15 00:00:01".to_string(),
            item_code: "_Test Item".to_string(),
            warehouse: "_Test Warehouse - _TC".to_string(),
            company: "_Test Company".to_string(),
            is_cancelled: true,
        },
    ];

    let vouchers = vec![
        ("Stock Entry".to_string(), "Wat".to_string()),
        ("Stock Entry".to_string(), "SE-2".to_string()),
        ("Stock Entry".to_string(), "SE-3".to_string()),
        ("Stock Entry".to_string(), "SE-1".to_string()),
    ];
    assert_eq!(
        sort_stock_vouchers_by_posting_date(&vouchers, None, &sles),
        vec![
            ("Stock Entry".to_string(), "SE-1".to_string()),
            ("Stock Entry".to_string(), "SE-2".to_string()),
            ("Stock Entry".to_string(), "SE-3".to_string()),
            ("Stock Entry".to_string(), "Wat".to_string()),
        ]
    );

    assert_eq!(
        get_future_stock_vouchers(
            "2022-01-01",
            "00:00:00",
            None,
            Some(&["_Test Item".to_string()]),
            Some("_Test Company"),
            &sles,
        ),
        vec![
            ("Stock Entry".to_string(), "SE-1".to_string()),
            ("Stock Entry".to_string(), "SE-2".to_string()),
            ("Stock Entry".to_string(), "SE-3".to_string()),
        ]
    );

    let grouped = get_voucherwise_gl_entries(
        &[
            ("Stock Entry".to_string(), "SE-1".to_string()),
            ("Stock Entry".to_string(), "SE-3".to_string()),
        ],
        "2022-01-01",
        &[
            StockGlEntry {
                name: "GLE-1".to_string(),
                account: "Stock - TC".to_string(),
                credit: 0.0,
                debit: 10.0,
                cost_center: Some("Main - TC".to_string()),
                project: None,
                voucher_type: "Stock Entry".to_string(),
                voucher_no: "SE-1".to_string(),
                posting_date: "2022-01-01".to_string(),
            },
            StockGlEntry {
                name: "GLE-OLD".to_string(),
                account: "Stock - TC".to_string(),
                credit: 0.0,
                debit: 10.0,
                cost_center: None,
                project: None,
                voucher_type: "Stock Entry".to_string(),
                voucher_no: "SE-3".to_string(),
                posting_date: "2021-12-31".to_string(),
            },
        ],
    );
    assert_eq!(
        grouped.keys().cloned().collect::<Vec<_>>(),
        vec![("Stock Entry".to_string(), "SE-1".to_string())]
    );
    assert_eq!(
        grouped[&("Stock Entry".to_string(), "SE-1".to_string())][0].name,
        "GLE-1"
    );
}

#[test]
fn naming_series_variable_parsing_matches_fy_abbr_and_date_tokens() {
    let fiscal_years = vec![FiscalYearRecord {
        name: "2026-2027".to_string(),
        year_start_date: "2026-04-01".to_string(),
        year_end_date: "2027-03-31".to_string(),
        disabled: false,
        companies: vec!["Acme".to_string()],
    }];
    let context = NamingSeriesContext {
        fiscal_years,
        company_abbrs: BTreeMap::from([("Acme".to_string(), "TC".to_string())]),
        default_company: Some("Acme".to_string()),
        now_datetime: "2026-06-07 12:30:00".to_string(),
        use_posting_datetime_for_naming_documents: true,
        reference_docs: BTreeMap::new(),
    };
    let doc = NamingSeriesDoc {
        doctype: "Supplier".to_string(),
        posting_date: Some("2026-06-06".to_string()),
        transaction_date: None,
        posting_datetime: None,
        company: Some("Acme".to_string()),
        reference_doctype: None,
        reference_name: None,
    };

    assert_eq!(
        parse_naming_series_variable(Some(&doc), "FY", &context),
        "2026-2027"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "TFY", &context),
        "26-27"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "ABBR", &context),
        "TC"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "YY", &context),
        "26"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "YYYY", &context),
        "2026"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "MM", &context),
        "06"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "DD", &context),
        "06"
    );
    assert_eq!(
        parse_naming_series_variable(Some(&doc), "JJJ", &context),
        "157"
    );

    let no_company_doc = NamingSeriesDoc {
        company: None,
        ..doc.clone()
    };
    let unrestricted_context = NamingSeriesContext {
        fiscal_years: vec![FiscalYearRecord {
            name: "2026-BETA".to_string(),
            year_start_date: "2026-04-01".to_string(),
            year_end_date: "2027-03-31".to_string(),
            disabled: false,
            companies: vec!["Beta".to_string()],
        }],
        default_company: Some("Acme".to_string()),
        ..context
    };
    assert_eq!(
        parse_naming_series_variable(Some(&no_company_doc), "FY", &unrestricted_context),
        "2026-BETA"
    );
}
