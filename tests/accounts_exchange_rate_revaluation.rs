use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::exchange_rate_revaluation::exchange_rate_revaluation::{
    calculate_exchange_rate_using_last_gle, get_account_details, ExchangeRateRevaluation,
    ExchangeRateRevaluationError, ExchangeRateRevaluationGlEntry,
    ExchangeRateRevaluationJournalAccount, ExchangeRateRevaluationJournalEntryAccount,
    ExchangeRateRevaluationRow, RevaluationAccountBalance, RevaluationAccountDetail,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000_001,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn exchange_rate_revaluation_matches_erpnext_metadata() {
    assert_eq!(
        ExchangeRateRevaluation::DOCTYPE,
        "Exchange Rate Revaluation"
    );
    assert_eq!(ExchangeRateRevaluation::MODULE, "Accounts");
    assert!(ExchangeRateRevaluation::ALLOW_IMPORT);
    assert_eq!(ExchangeRateRevaluation::AUTONAME, "ACC-ERR-.YYYY.-.#####");
    assert!(ExchangeRateRevaluation::IS_SUBMITTABLE);
    assert_eq!(
        ExchangeRateRevaluation::NAMING_RULE,
        "Expression (old style)"
    );
    assert_eq!(ExchangeRateRevaluation::SORT_FIELD, "creation");
    assert_eq!(ExchangeRateRevaluation::SORT_ORDER, "DESC");
    assert!(ExchangeRateRevaluation::TRACK_CHANGES);
    assert_eq!(
        ExchangeRateRevaluation::FIELD_ORDER,
        [
            "posting_date",
            "rounding_loss_allowance",
            "column_break_2",
            "company",
            "section_break_4",
            "get_entries",
            "accounts",
            "section_break_6",
            "gain_loss_unbooked",
            "gain_loss_booked",
            "column_break_10",
            "total_gain_loss",
            "amended_from",
        ]
    );

    assert_eq!(
        ExchangeRateRevaluation::fields(),
        vec![
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .required(),
            FieldSpec::section_break("section_break_4"),
            FieldSpec::button("get_entries", "Get Entries"),
            FieldSpec::table("accounts", "Exchange Rate Revaluation Account")
                .options("Exchange Rate Revaluation Account")
                .no_copy()
                .required(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Exchange Rate Revaluation")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::currency("gain_loss_unbooked", "Gain/Loss from Revaluation")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::currency("gain_loss_booked", "Gain/Loss already booked")
                .options("Company:company:default_currency")
                .description("Gain/Loss accumulated in foreign currency account. Accounts with '0' balance in either Base or Account currency")
                .read_only(),
            FieldSpec::currency("total_gain_loss", "Total Gain/Loss")
                .options("Company:company:default_currency")
                .read_only(),
            FieldSpec::column_break("column_break_10"),
            FieldSpec::float("rounding_loss_allowance", "Rounding Loss Allowance")
                .default("0.05")
                .description("Only values between [0,1) are allowed. Like {0.00, 0.04, 0.09, ...}\nEx: If allowance is set at 0.07, accounts that have balance of 0.07 in either of the currencies will be considered as zero balance account")
                .precision("9"),
        ]
    );

    let controller = ExchangeRateRevaluation::default();
    assert_eq!(controller.doctype(), "Exchange Rate Revaluation");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(
        controller.custom_hooks(),
        &["validate", "before_submit", "on_cancel"]
    );
}

#[test]
fn exchange_rate_revaluation_validates_rounding_allowance_and_totals() {
    let mut doc = ExchangeRateRevaluation {
        rounding_loss_allowance: 0.05,
        accounts: vec![
            ExchangeRateRevaluationRow {
                balance_in_base_currency: 100.0,
                new_balance_in_base_currency: 113.456,
                zero_balance: false,
                ..Default::default()
            },
            ExchangeRateRevaluationRow {
                gain_loss: -3.25,
                zero_balance: true,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    doc.validate().expect("valid rounding allowance");

    assert_close(doc.accounts[0].gain_loss, 13.46);
    assert_close(doc.gain_loss_unbooked, 13.46);
    assert_close(doc.gain_loss_booked, -3.25);
    assert_close(doc.total_gain_loss, 10.21);

    let invalid = ExchangeRateRevaluation {
        rounding_loss_allowance: 1.0,
        ..Default::default()
    };
    assert_eq!(
        invalid.validate_rounding_loss_allowance(),
        Err(ExchangeRateRevaluationError::RoundingLossAllowanceOutOfRange)
    );
}

#[test]
fn exchange_rate_revaluation_filters_empty_gain_loss_rows_before_submit() {
    let mut doc = ExchangeRateRevaluation {
        accounts: vec![
            ExchangeRateRevaluationRow {
                account: "Debtors - USD".to_string(),
                gain_loss: 0.0,
                ..Default::default()
            },
            ExchangeRateRevaluationRow {
                account: "Creditors - EUR".to_string(),
                gain_loss: -5.0,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    doc.before_submit().expect("one row has gain/loss");

    assert_eq!(doc.accounts.len(), 1);
    assert_eq!(doc.accounts[0].account, "Creditors - EUR");

    let mut empty = ExchangeRateRevaluation {
        accounts: vec![ExchangeRateRevaluationRow::default()],
        ..Default::default()
    };
    assert_eq!(
        empty.before_submit(),
        Err(ExchangeRateRevaluationError::NoAccountsWithGainLoss)
    );
}

#[test]
fn exchange_rate_revaluation_aggregates_gle_balances_like_erpnext() {
    let rows = ExchangeRateRevaluation::get_account_balance_from_gle(
        &["Debtors - USD".to_string(), "Bank - EUR".to_string()],
        "2026-05-10",
        None,
        None,
        0.05,
        2,
        &[
            ExchangeRateRevaluationGlEntry {
                account: "Debtors - USD".to_string(),
                account_currency: "USD".to_string(),
                posting_date: "2026-05-01".to_string(),
                debit_in_account_currency: 100.004,
                debit: 120.006,
                ..Default::default()
            },
            ExchangeRateRevaluationGlEntry {
                account: "Bank - EUR".to_string(),
                account_currency: "EUR".to_string(),
                posting_date: "2026-05-01".to_string(),
                debit_in_account_currency: 0.04,
                debit: 10.0,
                ..Default::default()
            },
            ExchangeRateRevaluationGlEntry {
                account: "Debtors - USD".to_string(),
                account_currency: "USD".to_string(),
                posting_date: "2026-06-01".to_string(),
                debit_in_account_currency: 500.0,
                debit: 500.0,
                ..Default::default()
            },
        ],
    );

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].account, "Bank - EUR");
    assert_close(rows[0].balance_in_account_currency, 0.0);
    assert_close(rows[0].balance, 10.0);
    assert!(rows[0].zero_balance);
    assert_eq!(rows[1].account, "Debtors - USD");
    assert_close(rows[1].balance_in_account_currency, 100.0);
    assert_close(rows[1].balance, 120.01);
    assert!(!rows[1].zero_balance);
}

#[test]
fn exchange_rate_revaluation_calculates_new_balances_for_regular_and_zero_balance_rows() {
    let exchange_rates = BTreeMap::from([("EUR".to_string(), 2.4)]);
    let last_exchange_rates =
        BTreeMap::from([("Zero Account - GBP|Supplier|Supp-1".to_string(), 2.5)]);

    let rows = ExchangeRateRevaluation::calculate_new_account_balance(
        "UZS",
        &[
            RevaluationAccountBalance {
                account: "Debtors - EUR".to_string(),
                account_currency: "EUR".to_string(),
                balance_in_account_currency: 50.0,
                balance: 100.0,
                zero_balance: false,
                ..Default::default()
            },
            RevaluationAccountBalance {
                account: "Base Only - USD".to_string(),
                account_currency: "USD".to_string(),
                balance: 25.0,
                zero_balance: true,
                ..Default::default()
            },
            RevaluationAccountBalance {
                account: "Zero Account - GBP".to_string(),
                party_type: Some("Supplier".to_string()),
                party: Some("Supp-1".to_string()),
                account_currency: "GBP".to_string(),
                balance_in_account_currency: 40.0,
                balance: 0.0,
                zero_balance: true,
            },
        ],
        &exchange_rates,
        &last_exchange_rates,
        2,
    );

    assert_eq!(rows.len(), 3);
    assert_close(rows[0].current_exchange_rate, 2.0);
    assert_close(rows[0].new_exchange_rate, 2.4);
    assert_close(rows[0].new_balance_in_base_currency, 120.0);
    assert_close(rows[0].gain_loss, 20.0);
    assert_close(rows[1].gain_loss, -25.0);
    assert_close(rows[2].current_exchange_rate, 2.5);
    assert_close(rows[2].gain_loss, -100.0);
}

#[test]
fn exchange_rate_revaluation_builds_journal_condition_and_jv_plans() {
    let doc = ExchangeRateRevaluation {
        name: Some("ACC-ERR-0001".to_string()),
        company: Some("Wind Power LLC".to_string()),
        posting_date: Some("2026-05-10".to_string()),
        gain_loss_booked: -25.0,
        gain_loss_unbooked: 20.0,
        total_gain_loss: -5.0,
        accounts: vec![
            ExchangeRateRevaluationRow {
                account: "Base Only - USD".to_string(),
                account_currency: "USD".to_string(),
                balance_in_base_currency: 25.0,
                gain_loss: -25.0,
                zero_balance: true,
                ..Default::default()
            },
            ExchangeRateRevaluationRow {
                account: "Debtors - EUR".to_string(),
                account_currency: "EUR".to_string(),
                balance_in_account_currency: 50.0,
                current_exchange_rate: 2.0,
                new_exchange_rate: 2.4,
                gain_loss: 20.0,
                zero_balance: false,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    assert!(doc.check_journal_entry_condition(&[], &[], "Unrealized Gain/Loss"));
    assert!(!doc.check_journal_entry_condition(
        &[ExchangeRateRevaluationJournalEntryAccount {
            parent: "JV-1".to_string(),
            reference_type: "Exchange Rate Revaluation".to_string(),
            reference_name: "ACC-ERR-0001".to_string(),
            docstatus: 1,
        }],
        &[ExchangeRateRevaluationGlEntry {
            voucher_type: "Journal Entry".to_string(),
            voucher_no: "JV-1".to_string(),
            account: "Unrealized Gain/Loss".to_string(),
            debit: 5.0,
            ..Default::default()
        }],
        "Unrealized Gain/Loss",
    ));

    let zero_balance_jv = doc
        .make_jv_for_zero_balance_plan("Unrealized Gain/Loss", "Main - CC")
        .expect("zero balance JV");
    assert_eq!(zero_balance_jv.voucher_type, "Exchange Gain Or Loss");
    assert_eq!(zero_balance_jv.accounts.len(), 2);
    assert_eq!(zero_balance_jv.accounts[0].account, "Base Only - USD");
    assert_close(zero_balance_jv.accounts[0].credit, 25.0);
    assert_eq!(zero_balance_jv.accounts[1].account, "Unrealized Gain/Loss");
    assert_close(zero_balance_jv.accounts[1].debit, 25.0);

    let revaluation_jv = doc
        .make_jv_for_revaluation_plan("Unrealized Gain/Loss", "Main - CC", 20.0)
        .expect("revaluation JV");
    assert_eq!(revaluation_jv.voucher_type, "Exchange Rate Revaluation");
    assert_eq!(revaluation_jv.accounts.len(), 3);
    assert_eq!(
        revaluation_jv.accounts[0],
        ExchangeRateRevaluationJournalAccount {
            account: "Debtors - EUR".to_string(),
            account_currency: Some("EUR".to_string()),
            balance: 50.0,
            debit_in_account_currency: 50.0,
            exchange_rate: 2.4,
            cost_center: Some("Main - CC".to_string()),
            reference_type: Some("Exchange Rate Revaluation".to_string()),
            reference_name: Some("ACC-ERR-0001".to_string()),
            ..Default::default()
        }
    );
    assert_eq!(revaluation_jv.accounts[2].account, "Unrealized Gain/Loss");
    assert_close(revaluation_jv.accounts[2].credit_in_account_currency, 20.0);
}

#[test]
fn exchange_rate_revaluation_account_details_validate_mandatory_party_and_last_rate() {
    assert_eq!(
        get_account_details(
            None,
            Some("2026-05-10"),
            "Debtors - USD",
            "USD",
            "Receivable",
            None,
            None,
            &[],
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Err(ExchangeRateRevaluationError::MissingCompanyOrPostingDate)
    );
    assert_eq!(
        get_account_details(
            Some("Wind Power LLC"),
            Some("2026-05-10"),
            "Debtors - USD",
            "USD",
            "Receivable",
            None,
            None,
            &[],
            &BTreeMap::new(),
            &BTreeMap::new(),
        ),
        Err(ExchangeRateRevaluationError::MissingPartyForAccountType {
            account_type: "Receivable".to_string(),
        })
    );

    let rate = calculate_exchange_rate_using_last_gle(
        "Wind Power LLC",
        "Debtors - USD",
        None,
        None,
        &[
            ExchangeRateRevaluationGlEntry {
                company: "Wind Power LLC".to_string(),
                account: "Debtors - USD".to_string(),
                posting_date: "2026-05-01".to_string(),
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SI-1".to_string(),
                debit: 120.0,
                debit_in_account_currency: 100.0,
                ..Default::default()
            },
            ExchangeRateRevaluationGlEntry {
                company: "Wind Power LLC".to_string(),
                account: "Debtors - USD".to_string(),
                posting_date: "2026-05-03".to_string(),
                voucher_type: "Sales Invoice".to_string(),
                voucher_no: "SI-2".to_string(),
                credit: 60.0,
                credit_in_account_currency: 50.0,
                ..Default::default()
            },
        ],
    )
    .expect("last rate");
    assert_close(rate, 1.2);

    let details = get_account_details(
        Some("Wind Power LLC"),
        Some("2026-05-10"),
        "Debtors - USD",
        "USD",
        "Receivable",
        Some("Customer"),
        Some("Cust-1"),
        &[RevaluationAccountBalance {
            account: "Debtors - USD".to_string(),
            account_currency: "USD".to_string(),
            balance_in_account_currency: 100.0,
            balance: 120.0,
            zero_balance: false,
            ..Default::default()
        }],
        &BTreeMap::from([("USD".to_string(), 1.3)]),
        &BTreeMap::new(),
    )
    .expect("account details");
    assert_eq!(
        details,
        RevaluationAccountDetail {
            account_currency: Some("USD".to_string()),
            balance_in_base_currency: Some(120.0),
            balance_in_account_currency: Some(100.0),
            current_exchange_rate: Some(1.2),
            new_exchange_rate: Some(1.3),
            new_balance_in_base_currency: Some(130.0),
            new_balance_in_account_currency: Some(100.0),
            zero_balance: Some(false),
            gain_loss: Some(10.0),
        }
    );
}
