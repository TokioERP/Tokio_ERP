use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::account_closing_balance::account_closing_balance::{
    aggregate_with_last_account_closing_balance, generate_key, make_closing_entries,
    previous_closing_entries_plan, set_amount_in_reporting_currency, AccountClosingBalance,
    AccountClosingEntry, AccountClosingEntryKey, PreviousClosingEntriesPlan,
    ReportingCurrencyError,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn account_closing_balance_matches_erpnext_metadata() {
    assert_eq!(AccountClosingBalance::DOCTYPE, "Account Closing Balance");
    assert_eq!(AccountClosingBalance::MODULE, "Accounts");
    assert_eq!(
        AccountClosingBalance::FIELD_ORDER,
        [
            "closing_date",
            "account",
            "cost_center",
            "debit",
            "credit",
            "reporting_currency_exchange_rate",
            "debit_in_reporting_currency",
            "credit_in_reporting_currency",
            "account_currency",
            "debit_in_account_currency",
            "credit_in_account_currency",
            "project",
            "company",
            "finance_book",
            "period_closing_voucher",
            "is_period_closing_voucher_entry",
        ]
    );
    assert_eq!(
        AccountClosingBalance::fields(),
        vec![
            FieldSpec::date("closing_date", "Closing Date")
                .in_filter()
                .in_list_view()
                .oldfield("posting_date", "Date")
                .search_index(),
            FieldSpec::link("account", "Account")
                .options("Account")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .oldfield("account", "Link")
                .search_index(),
            FieldSpec::link("cost_center", "Cost Center")
                .options("Cost Center")
                .in_filter()
                .in_list_view()
                .oldfield("cost_center", "Link"),
            FieldSpec::currency("debit", "Debit Amount")
                .options("Company:company:default_currency")
                .oldfield("debit", "Currency"),
            FieldSpec::currency("credit", "Credit Amount")
                .options("Company:company:default_currency")
                .oldfield("credit", "Currency"),
            FieldSpec::float(
                "reporting_currency_exchange_rate",
                "Reporting Currency Exchange Rate",
            )
            .precision("9"),
            FieldSpec::currency(
                "debit_in_reporting_currency",
                "Debit Amount in Reporting Currency"
            )
            .options("Company:company:reporting_currency"),
            FieldSpec::currency(
                "credit_in_reporting_currency",
                "Credit Amount in Reporting Currency",
            )
            .options("Company:company:reporting_currency"),
            FieldSpec::link("account_currency", "Account Currency").options("Currency"),
            FieldSpec::currency(
                "debit_in_account_currency",
                "Debit Amount in Account Currency"
            )
            .options("account_currency"),
            FieldSpec::currency(
                "credit_in_account_currency",
                "Credit Amount in Account Currency"
            )
            .options("account_currency"),
            FieldSpec::link("project", "Project").options("Project"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_filter()
                .in_list_view()
                .in_standard_filter()
                .oldfield("company", "Link")
                .search_index(),
            FieldSpec::link("finance_book", "Finance Book").options("Finance Book"),
            FieldSpec::link("period_closing_voucher", "Period Closing Voucher")
                .options("Period Closing Voucher")
                .in_standard_filter()
                .search_index(),
            FieldSpec::check(
                "is_period_closing_voucher_entry",
                "Is Period Closing Voucher Entry",
            )
            .default("0"),
        ]
    );
}

#[test]
fn account_closing_balance_controller_is_pass_through() {
    let doc = AccountClosingBalance::default();

    assert_eq!(doc.doctype(), "Account Closing Balance");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), [] as [&str; 0]);
}

#[test]
fn generate_key_matches_erpnext_base_and_dynamic_dimensions() {
    let mut entry = AccountClosingEntry::new("Company A", "Cash - A", "USD");
    entry.cost_center = "Main - A".to_string();
    entry.project = "PROJ-1".to_string();
    entry.finance_book = "IFRS".to_string();
    entry.is_period_closing_voucher_entry = true;
    entry
        .dimensions
        .insert("branch".to_string(), "North".to_string());

    let (key, values) = generate_key(&entry, &["branch".to_string()]);

    assert_eq!(
        key,
        AccountClosingEntryKey {
            parts: vec![
                "Cash - A".to_string(),
                "USD".to_string(),
                "Main - A".to_string(),
                "PROJ-1".to_string(),
                "IFRS".to_string(),
                "1".to_string(),
                "North".to_string(),
            ],
        }
    );
    assert_eq!(values.get("company").map(String::as_str), Some("Company A"));
    assert_eq!(values.get("branch").map(String::as_str), Some("North"));
}

#[test]
fn aggregate_with_last_account_closing_balance_sums_like_erpnext() {
    let mut first = AccountClosingEntry::new("Company A", "Cash - A", "USD");
    first.debit = 100.0;
    first.credit = 20.0;
    first.debit_in_account_currency = 100.0;
    first.credit_in_account_currency = 20.0;
    first
        .dimensions
        .insert("branch".to_string(), "North".to_string());

    let mut second = first.clone();
    second.debit = 40.0;
    second.credit = 5.0;
    second.debit_in_account_currency = 40.0;
    second.credit_in_account_currency = 5.0;

    let aggregated =
        aggregate_with_last_account_closing_balance(&[first, second], &["branch".to_string()]);
    let row = aggregated.values().next().expect("one merged key");

    assert_eq!(row.debit, 140.0);
    assert_eq!(row.credit, 25.0);
    assert_eq!(row.debit_in_account_currency, 140.0);
    assert_eq!(row.credit_in_account_currency, 25.0);
    assert_eq!(
        row.dimensions.get("branch").map(String::as_str),
        Some("North")
    );
}

#[test]
fn previous_closing_entries_plan_matches_erpnext_query_shape() {
    assert_eq!(
        previous_closing_entries_plan("Company A", "2026-03-31", &["branch".to_string()]),
        PreviousClosingEntriesPlan {
            period_closing_voucher_filters: BTreeMap::from([
                ("company".to_string(), "Company A".to_string()),
                ("docstatus".to_string(), "1".to_string()),
                ("period_end_date".to_string(), "<2026-03-31".to_string()),
            ]),
            period_closing_voucher_order_by: "period_end_date desc",
            period_closing_voucher_limit: 1,
            account_closing_balance_fields: vec![
                "company".to_string(),
                "account".to_string(),
                "account_currency".to_string(),
                "debit".to_string(),
                "credit".to_string(),
                "debit_in_account_currency".to_string(),
                "credit_in_account_currency".to_string(),
                "cost_center".to_string(),
                "project".to_string(),
                "finance_book".to_string(),
                "is_period_closing_voucher_entry".to_string(),
                "branch".to_string(),
            ],
        }
    );
}

#[test]
fn make_closing_entries_adds_voucher_date_and_reporting_currency() {
    let mut current = AccountClosingEntry::new("Company A", "Cash - A", "USD");
    current.debit = 100.0;
    current.credit = 25.0;

    let entries = make_closing_entries(
        &[current],
        &[],
        "PCV-0001",
        "Company A",
        "2026-03-31",
        &[],
        Some(1.25),
    )
    .expect("exchange rate exists");

    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].period_closing_voucher.as_deref(),
        Some("PCV-0001")
    );
    assert_eq!(entries[0].closing_date.as_deref(), Some("2026-03-31"));
    assert_eq!(entries[0].reporting_currency_exchange_rate, 1.25);
    assert_eq!(entries[0].debit_in_reporting_currency, 125.0);
    assert_eq!(entries[0].credit_in_reporting_currency, 31.25);
}

#[test]
fn reporting_currency_requires_exchange_rate_like_erpnext() {
    let mut closing_balance = AccountClosingBalance {
        debit: 10.0,
        credit: 2.0,
        ..AccountClosingBalance::default()
    };

    assert_eq!(
        set_amount_in_reporting_currency(&mut closing_balance, "USD", "UZS", "2026-03-31", None),
        Err(ReportingCurrencyError {
            title: "Reporting Currency Exchange Not Found",
            message: "Unable to find exchange rate for USD to UZS for key date 2026-03-31. Please create a Currency Exchange record manually.".to_string(),
        })
    );
}
