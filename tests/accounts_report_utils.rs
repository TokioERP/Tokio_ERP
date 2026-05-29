use std::collections::BTreeMap;

use serde_json::json;
use tokio_erp::erpnext::accounts::report::utils::{
    apply_common_conditions, convert, convert_to_presentation_currency,
    filter_invoices_based_on_dimensions, get_advance_taxes_and_charges_spec, get_currency,
    get_journal_entries_spec, get_opening_row_spec, get_party_details, get_payment_entries_spec,
    get_query_columns, get_taxes_query_spec, get_values_for_columns, AccountingDimension,
    CommonConditionOptions, CurrencyFilters, CurrencyInfo, CurrencyRate, DocumentRow, GlEntry,
    JournalPaymentArgs, PartyRecord, QueryColumn, QuerySpec,
};

fn rates() -> Vec<CurrencyRate> {
    vec![CurrencyRate::new("USD", "UZS", "2026-03-31", 12_500.0)]
}

#[test]
fn report_utils_get_currency_matches_company_presentation_and_report_date_fallbacks() {
    let info = get_currency(
        &CurrencyFilters {
            company: Some("Acme".to_string()),
            presentation_currency: Some("EUR".to_string()),
            to_date: None,
            period_end_date: Some("2026-03-31".to_string()),
            to_fiscal_year: None,
        },
        Some("Default Co"),
        |company| format!("{company} USD"),
        |_| None,
    );

    assert_eq!(
        info,
        CurrencyInfo {
            company: "Acme".to_string(),
            company_currency: "Acme USD".to_string(),
            presentation_currency: "EUR".to_string(),
            report_date: "2026-03-31".to_string(),
        }
    );

    let defaulted = get_currency(
        &CurrencyFilters {
            company: None,
            presentation_currency: None,
            to_date: None,
            period_end_date: None,
            to_fiscal_year: Some("2026".to_string()),
        },
        Some("Default Co"),
        |_| "USD".to_string(),
        |year| Some(format!("{year}-12-31")),
    );

    assert_eq!(defaulted.company, "Default Co");
    assert_eq!(defaulted.presentation_currency, "USD");
    assert_eq!(defaulted.report_date, "2026-12-31");
}

#[test]
fn report_utils_convert_uses_rate_as_divisor_and_defaults_to_one() {
    assert_eq!(convert(25_000.0, "USD", "UZS", "2026-03-31", &rates()), 2.0);
    assert_eq!(convert(42.0, "USD", "EUR", "2026-03-31", &rates()), 42.0);
}

#[test]
fn report_utils_convert_to_presentation_currency_matches_account_currency_shortcut() {
    let currency_info = CurrencyInfo {
        company: "Acme".to_string(),
        company_currency: "UZS".to_string(),
        presentation_currency: "USD".to_string(),
        report_date: "2026-03-31".to_string(),
    };
    let mut entries = vec![GlEntry {
        debit: 12_500.0,
        credit: 0.0,
        debit_in_account_currency: 1.0,
        credit_in_account_currency: 0.0,
        account_currency: "USD".to_string(),
        account: "Cash".to_string(),
    }];

    convert_to_presentation_currency(&mut entries, &currency_info, None, &rates());
    assert_eq!(entries[0].debit, 1.0);

    convert_to_presentation_currency(
        &mut entries,
        &currency_info,
        Some(&["Exchange Gain/Loss".to_string()]),
        &rates(),
    );
    assert_eq!(entries[0].debit, 0.00008);
}

#[test]
fn report_utils_column_helpers_match_python_dict_behaviour() {
    let columns = vec![
        QueryColumn::new("customer"),
        QueryColumn::with_doctype("territory", "Customer"),
    ];
    assert_eq!(
        get_query_columns(&columns),
        "customer, `tabCustomer`.`territory`"
    );

    let mut row = BTreeMap::new();
    row.insert("customer".to_string(), json!("CUST-001"));
    row.insert("territory".to_string(), json!("Tashkent"));
    row.insert("ignored".to_string(), json!(1));

    let values = get_values_for_columns(&columns, &row);
    assert_eq!(values["customer"], json!("CUST-001"));
    assert_eq!(values["territory"], json!("Tashkent"));
    assert!(!values.contains_key("ignored"));
}

#[test]
fn report_utils_party_details_use_supplier_or_customer_fields() {
    let parties = vec![
        PartyRecord::supplier("SUP-1", "TIN-S", "Services"),
        PartyRecord::customer("CUST-1", "TIN-C", "Retail", "Central"),
    ];

    let supplier = get_party_details("Supplier", &["SUP-1".to_string()], &parties);
    assert_eq!(
        supplier["SUP-1"].supplier_group.as_deref(),
        Some("Services")
    );
    assert!(supplier["SUP-1"].customer_group.is_none());

    let customer = get_party_details("Customer", &["CUST-1".to_string()], &parties);
    assert_eq!(customer["CUST-1"].customer_group.as_deref(), Some("Retail"));
    assert_eq!(customer["CUST-1"].territory.as_deref(), Some("Central"));
}

#[test]
fn report_utils_query_specs_match_tax_and_entry_query_shape() {
    let invoices = vec![DocumentRow::new("INV-1"), DocumentRow::new("INV-2")];

    assert_eq!(
        get_taxes_query_spec(&invoices, "Purchase Taxes and Charges", "Purchase Invoice")
            .conditions,
        vec![
            "parenttype = Purchase Invoice",
            "docstatus = 1",
            "account_head is not null",
            "parent in [INV-1, INV-2]",
            "category in [Total, Valuation and Total]",
        ]
    );
    assert_eq!(
        get_taxes_query_spec(&invoices, "Payment Taxes and Charges", "Payment Entry").conditions[4],
        "charge_type in [On Paid Amount, Actual]"
    );

    let args = JournalPaymentArgs {
        account: "account".to_string(),
        party: "party".to_string(),
        party_name: "party_name".to_string(),
        account_fieldname: "paid_from".to_string(),
        party_account: vec!["Debtors".to_string()],
    };
    assert_eq!(get_journal_entries_spec(&args).doctype, "Journal Entry");
    assert_eq!(
        get_payment_entries_spec(&args).selects[3],
        "paid_from as account"
    );
    assert_eq!(
        get_advance_taxes_and_charges_spec(&invoices).group_by,
        vec!["parent", "account_head", "add_deduct_tax"]
    );
}

#[test]
fn report_utils_common_conditions_and_dimension_filters_match_python_branching() {
    let mut filters = BTreeMap::new();
    filters.insert("company".to_string(), json!("Acme"));
    filters.insert("from_date".to_string(), json!("2026-01-01"));
    filters.insert("to_date".to_string(), json!("2026-03-31"));
    filters.insert("cost_center".to_string(), json!("Main"));
    filters.insert("warehouse".to_string(), json!("WH-1"));
    filters.insert("brand".to_string(), json!("Brand A"));
    filters.insert("customer_group".to_string(), json!("Retail"));

    let mut spec = QuerySpec::new("Sales Invoice");
    apply_common_conditions(
        &filters,
        &mut spec,
        CommonConditionOptions {
            doctype: "Sales Invoice".to_string(),
            child_doctype: Some("Sales Invoice Item".to_string()),
            payments: false,
        },
    );

    assert!(spec
        .conditions
        .contains(&"Sales Invoice.company = Acme".to_string()));
    assert!(spec.inner_joins.contains(
        &"Sales Invoice Item on Sales Invoice.name = Sales Invoice Item.parent".to_string()
    ));
    assert!(spec.distinct);

    filter_invoices_based_on_dimensions(
        &filters,
        &mut spec,
        &[AccountingDimension::new_tree(
            "customer_group",
            "Customer Group",
        )],
        |_, value| vec![value.to_string(), "Retail Child".to_string()],
    );

    assert!(spec
        .conditions
        .contains(&"Sales Invoice.customer_group in [Retail, Retail Child]".to_string()));
}

#[test]
fn report_utils_opening_row_spec_matches_party_account_filter() {
    let spec = get_opening_row_spec(
        "Customer",
        "CUST-1",
        "2026-01-01",
        "Acme",
        |_, _, _, include_advance| {
            assert!(include_advance);
            vec!["Debtors".to_string(), "Advance".to_string()]
        },
    );

    assert_eq!(spec.selects[0], "'Opening' as account");
    assert_eq!(spec.conditions[0], "account in [Debtors, Advance]");
    assert_eq!(spec.conditions[1], "party = CUST-1");
    assert_eq!(spec.conditions[2], "posting_date < 2026-01-01");
    assert_eq!(spec.conditions[3], "is_cancelled = 0");
}
