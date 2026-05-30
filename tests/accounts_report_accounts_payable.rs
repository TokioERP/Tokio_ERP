use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::accounts_payable::accounts_payable;
use tokio_erp::erpnext::accounts::report::accounts_receivable::accounts_receivable::{
    add_project_and_cost_center_conditions, allocate_outstanding_based_on_payment_terms,
    init_voucher_balance, payment_term_template_filter_conditions, prepare_voucher_balance_rows,
    set_party_details, update_voucher_balance, AccountType, PartyDetails, PartyDetailsRow,
    PaymentLedgerEntry, PaymentTermAllocationRow, PaymentTermDetail, ReceivablePayableFilters,
    VoucherBalanceKey, VoucherBalanceRow,
};
use tokio_erp::erpnext::accounts::report::{DelegatedReportExecution, ReportArg};

fn filters() -> ReceivablePayableFilters {
    ReceivablePayableFilters {
        company: Some("_Test Company".to_string()),
        report_date: Some("2026-05-30".to_string()),
        finance_book: None,
        party_type: Some("Supplier".to_string()),
        customer_group: None,
        territory: None,
        supplier_group: None,
        payment_terms_template: None,
        cost_center: Vec::new(),
        project: Vec::new(),
        accounting_dimensions: BTreeMap::new(),
        calculate_ageing_with: None,
        ageing_based_on: None,
        range: Some("30, 60, 90, 120".to_string()),
        account_type: Some(AccountType::Payable),
        group_by_party: false,
        in_party_currency: false,
        party: Vec::new(),
        party_account: None,
        based_on_payment_terms: false,
        show_future_payments: false,
        show_delivery_notes: false,
        show_sales_person: false,
        sales_person: None,
        show_remarks: false,
        sales_partner: None,
        ignore_accounts: false,
        handle_employee_advances: false,
        for_revaluation_journals: false,
    }
}

fn payable_ple(
    amount: f64,
    amount_in_account_currency: f64,
    project: Option<&str>,
) -> PaymentLedgerEntry {
    PaymentLedgerEntry {
        account: "Creditors USD - TC".to_string(),
        voucher_type: "Purchase Invoice".to_string(),
        voucher_no: "PINV-0001".to_string(),
        against_voucher_type: "Purchase Invoice".to_string(),
        against_voucher_no: "PINV-0001".to_string(),
        party_type: "Supplier".to_string(),
        party: "SUPP-USD".to_string(),
        posting_date: "2026-05-01".to_string(),
        due_date: Some("2026-05-31".to_string()),
        account_currency: "USD".to_string(),
        remarks: None,
        cost_center: None,
        project: project.map(str::to_string),
        amount,
        amount_in_account_currency,
    }
}

fn payment_term_detail(term: &str, due_date: &str, amount: f64) -> PaymentTermDetail {
    PaymentTermDetail {
        party_account_currency: "USD".to_string(),
        currency: "USD".to_string(),
        total_advance: 0.0,
        due_date: due_date.to_string(),
        payment_term: term.to_string(),
        payment_amount: amount,
        base_payment_amount: amount,
        description: Some(term.to_string()),
        paid_amount: 0.0,
        base_paid_amount: 0.0,
        discounted_amount: 0.0,
    }
}

#[test]
fn accounts_payable_execute_delegates_to_receivable_payable_report_like_erpnext() {
    let filters = BTreeMap::from([("company".to_string(), "_Test Company".to_string())]);

    assert_eq!(
        accounts_payable::execute(Some(filters.clone())),
        DelegatedReportExecution {
            delegate: "ReceivablePayableReport",
            filters: Some(filters),
            args: vec![
                ("account_type", ReportArg::Text("Payable")),
                (
                    "naming_by",
                    ReportArg::List(&["Buying Settings", "supp_master_name"])
                ),
            ],
        }
    );
}

#[test]
fn accounts_payable_foreign_currency_supplier_and_project_rows_match_erpnext_test() {
    let invoice = payable_ple(24_000.0, 300.0, Some("PROJ-AP"));
    let payable_filters = ReceivablePayableFilters {
        in_party_currency: true,
        party: vec!["SUPP-USD".to_string()],
        project: vec!["PROJ-AP".to_string()],
        ..filters()
    };
    let mut balances: BTreeMap<VoucherBalanceKey, VoucherBalanceRow> = BTreeMap::new();
    let mut invoices = Vec::new();

    init_voucher_balance(&mut balances, &mut invoices, &invoice, &payable_filters, &[]);
    update_voucher_balance(&mut balances, &invoice, &payable_filters, &BTreeMap::new(), &[]);
    let rows = prepare_voucher_balance_rows(&balances, &payable_filters, 2, &[]);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].voucher_no, "PINV-0001");
    assert_eq!(rows[0].outstanding, 300.0);
    assert_eq!(rows[0].account_currency, "USD");
    assert_eq!(rows[0].project.as_deref(), Some("PROJ-AP"));
    assert_eq!(invoices, vec!["PINV-0001"]);

    let mut party_row = PartyDetailsRow {
        party: "SUPP-USD".to_string(),
        account_currency: rows[0].account_currency.clone(),
        currency: None,
        customer_name: None,
        territory: None,
        customer_group: None,
        customer_primary_contact: None,
        default_sales_partner: None,
        supplier_name: None,
        supplier_group: None,
    };
    let party_details = BTreeMap::from([(
        "SUPP-USD".to_string(),
        PartyDetails {
            customer_name: None,
            territory: None,
            customer_group: None,
            customer_primary_contact: None,
            default_sales_partner: None,
            supplier_name: Some("Test Supplier2".to_string()),
            supplier_group: Some("Services".to_string()),
        },
    )]);
    set_party_details(&mut party_row, &payable_filters, "INR", &party_details);

    assert_eq!(party_row.currency.as_deref(), Some("USD"));
    assert_eq!(party_row.supplier_name.as_deref(), Some("Test Supplier2"));
}

#[test]
fn accounts_payable_payment_terms_template_and_project_filters_match_erpnext_test() {
    let payable_filters = ReceivablePayableFilters {
        payment_terms_template: Some("_Test 50-50".to_string()),
        supplier_group: Some("All Supplier Groups".to_string()),
        party: vec!["SUPP-USD".to_string()],
        cost_center: vec!["Main - TC".to_string()],
        party_account: Some("Creditors USD - TC".to_string()),
        project: vec!["PROJ-AP".to_string()],
        based_on_payment_terms: true,
        ..filters()
    };

    assert_eq!(
        payment_term_template_filter_conditions(
            "Purchase Invoice",
            &payable_filters,
            &["All Supplier Groups".to_string()],
            &["Main - TC".to_string()],
        ),
        vec![
            "payment_terms_template = '_Test 50-50'",
            "company = '_Test Company'",
            "supplier_group IN ('All Supplier Groups')",
            "supplier IN ('SUPP-USD')",
            "cost_center IN ('Main - TC')",
            "credit_to = 'Creditors USD - TC'",
        ]
    );
    assert_eq!(
        add_project_and_cost_center_conditions(&payable_filters, &["Main - TC".to_string()]),
        vec![
            "cost_center IN ('Main - TC')",
            "project IN ('PROJ-AP')",
        ]
    );

    let mut row = PaymentTermAllocationRow {
        invoiced: 300.0,
        paid: 0.0,
        credit_note: 0.0,
        payment_terms: Vec::new(),
    };
    allocate_outstanding_based_on_payment_terms(
        &mut row,
        &payable_filters,
        &[
            payment_term_detail("_Test 50% on 30 Days", "2026-05-31", 150.0),
            payment_term_detail("_Test 50% on 15 Days", "2026-05-16", 150.0),
        ],
        "INR",
    );

    assert_eq!(row.payment_terms.len(), 2);
    assert_eq!(row.payment_terms[0].payment_term, "_Test 50% on 15 Days");
    assert_eq!(row.payment_terms[0].invoiced, 150.0);
    assert_eq!(row.payment_terms[0].outstanding, 150.0);
    assert_eq!(row.payment_terms[1].payment_term, "_Test 50% on 30 Days");
}
