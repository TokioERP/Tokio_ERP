use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::accounts_receivable_summary::accounts_receivable_summary::{
    execute, get_gl_balance, AccountType, AccountsReceivableSummaryArgs,
    AccountsReceivableSummaryFilters, GlEntry, ReceivableSummaryColumn, ReceivableSummaryContext,
    ReceivableSummarySourceRow,
};

fn filters() -> AccountsReceivableSummaryFilters {
    AccountsReceivableSummaryFilters {
        company: "_Test Company".to_string(),
        report_date: "2026-05-24".to_string(),
        range: "30, 60, 90, 120".to_string(),
        show_gl_balance: true,
        show_future_payments: true,
        show_sales_person: true,
        sales_partner: Some("Partner A".to_string()),
    }
}

fn receivable_args() -> AccountsReceivableSummaryArgs {
    AccountsReceivableSummaryArgs {
        account_type: AccountType::Receivable,
        party_naming_by: "Naming Series".to_string(),
    }
}

fn source_row(party: &str, outstanding: f64) -> ReceivableSummarySourceRow {
    ReceivableSummarySourceRow {
        party: party.to_string(),
        party_type: "Customer".to_string(),
        currency: "USD".to_string(),
        invoiced: 100.0,
        paid: 20.0,
        credit_note: 5.0,
        outstanding,
        total_due: outstanding,
        future_amount: 0.0,
        range0: 99.0,
        ranges: vec![10.0, 20.0, 30.0, 40.0, 50.0],
        territory: Some("Uzbekistan".to_string()),
        customer_group: Some("Retail".to_string()),
        supplier_group: None,
        sales_person: Some("Sales User".to_string()),
        default_sales_partner: Some("Partner A".to_string()),
    }
}

#[test]
fn accounts_receivable_summary_columns_match_erpnext_receivable_shape() {
    let report = execute(
        &filters(),
        &receivable_args(),
        &ReceivableSummaryContext::default(),
    );

    assert_eq!(
        report.columns,
        vec![
            ReceivableSummaryColumn::data("Party Type", "party_type", 100),
            ReceivableSummaryColumn::dynamic_link("Party", "party", "party_type", 180),
            ReceivableSummaryColumn::data("Customer Name", "party_name", 120),
            ReceivableSummaryColumn::currency("Advance Amount", "advance", 120),
            ReceivableSummaryColumn::currency("Invoiced Amount", "invoiced", 120),
            ReceivableSummaryColumn::currency("Paid Amount", "paid", 120),
            ReceivableSummaryColumn::currency("Credit Note", "credit_note", 120),
            ReceivableSummaryColumn::currency("Outstanding Amount", "outstanding", 120),
            ReceivableSummaryColumn::currency("GL Balance", "gl_balance", 120),
            ReceivableSummaryColumn::currency("Difference", "diff", 120),
            ReceivableSummaryColumn::currency("<0", "range0", 120),
            ReceivableSummaryColumn::currency("0-30", "range1", 120),
            ReceivableSummaryColumn::currency("31-60", "range2", 120),
            ReceivableSummaryColumn::currency("61-90", "range3", 120),
            ReceivableSummaryColumn::currency("91-120", "range4", 120),
            ReceivableSummaryColumn::currency("121-Above", "range5", 120),
            ReceivableSummaryColumn::currency("Total Amount Due", "total_due", 120),
            ReceivableSummaryColumn::currency("Future Payment Amount", "future_amount", 120),
            ReceivableSummaryColumn::currency("Remaining Balance", "remaining_balance", 120),
            ReceivableSummaryColumn::link("Territory", "territory", "Territory", 120),
            ReceivableSummaryColumn::link(
                "Customer Group",
                "customer_group",
                "Customer Group",
                120
            ),
            ReceivableSummaryColumn::data("Sales Person", "sales_person", 120),
            ReceivableSummaryColumn::data("Sales Partner", "default_sales_partner", 120),
            ReceivableSummaryColumn::link("Currency", "currency", "Currency", 80),
        ]
    );
}

#[test]
fn accounts_receivable_summary_aggregates_party_totals_and_adjusts_advance_gl_and_future_amounts() {
    let mut context = ReceivableSummaryContext::default();
    context.receivables = vec![
        source_row("CUST-001", 150.0),
        ReceivableSummarySourceRow {
            invoiced: 80.0,
            paid: 10.0,
            credit_note: 2.0,
            outstanding: 50.0,
            total_due: 50.0,
            future_amount: 30.0,
            range0: 88.0,
            ranges: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            sales_person: Some("Second Sales User".to_string()),
            ..source_row("CUST-001", 50.0)
        },
        source_row("CUST-ZERO", 0.0),
    ];
    context.advances = BTreeMap::from([("CUST-001".to_string(), 25.0)]);
    context.gl_balances = BTreeMap::from([("CUST-001".to_string(), 180.0)]);
    context.party_names = BTreeMap::from([("CUST-001".to_string(), "Acme Customer".to_string())]);

    let report = execute(&filters(), &receivable_args(), &context);

    assert_eq!(report.rows.len(), 1);
    let row = &report.rows[0];
    assert_eq!(row.party, "CUST-001");
    assert_eq!(row.party_name.as_deref(), Some("Acme Customer"));
    assert_eq!(row.invoiced, 180.0);
    assert_eq!(row.paid, 5.0);
    assert_eq!(row.credit_note, 7.0);
    assert_eq!(row.outstanding, 200.0);
    assert_eq!(row.advance, 25.0);
    assert_eq!(row.gl_balance, Some(180.0));
    assert_eq!(row.diff, Some(20.0));
    assert_eq!(row.future_amount, 30.0);
    assert_eq!(row.remaining_balance, Some(170.0));
    assert_eq!(row.range0, None);
    assert_eq!(row.ranges, vec![11.0, 22.0, 33.0, 44.0, 55.0]);
    assert_eq!(
        row.sales_person,
        vec!["Sales User".to_string(), "Second Sales User".to_string()]
    );
    assert_eq!(row.default_sales_partner.as_deref(), Some("Partner A"));
}

#[test]
fn accounts_receivable_summary_payable_args_switch_party_name_and_note_labels() {
    let report = execute(
        &AccountsReceivableSummaryFilters {
            show_gl_balance: false,
            show_future_payments: false,
            show_sales_person: false,
            sales_partner: None,
            ..filters()
        },
        &AccountsReceivableSummaryArgs {
            account_type: AccountType::Payable,
            party_naming_by: "Naming Series".to_string(),
        },
        &ReceivableSummaryContext::default(),
    );

    assert!(report.columns.contains(&ReceivableSummaryColumn::data(
        "Supplier Name",
        "party_name",
        120
    )));
    assert!(report.columns.contains(&ReceivableSummaryColumn::currency(
        "Debit Note",
        "credit_note",
        120
    )));
    assert!(report.columns.contains(&ReceivableSummaryColumn::link(
        "Supplier Group",
        "supplier_group",
        "Supplier Group",
        120
    )));
}

#[test]
fn accounts_receivable_summary_gl_balance_matches_erpnext_receivable_and_payable_formulas() {
    let entries = vec![
        GlEntry::new(
            "CUST-001",
            "_Test Company",
            "2026-05-24",
            false,
            120.0,
            20.0,
        ),
        GlEntry::new("CUST-001", "_Test Company", "2026-05-25", false, 500.0, 0.0),
        GlEntry::new("CUST-002", "_Test Company", "2026-05-24", true, 100.0, 0.0),
        GlEntry::new("SUP-001", "_Test Company", "2026-05-24", false, 30.0, 90.0),
        GlEntry::new("SUP-002", "Other Company", "2026-05-24", false, 10.0, 20.0),
    ];

    assert_eq!(
        get_gl_balance(
            "2026-05-24",
            "_Test Company",
            AccountType::Receivable,
            &entries
        ),
        BTreeMap::from([
            ("CUST-001".to_string(), 100.0),
            ("SUP-001".to_string(), -60.0),
        ])
    );
    assert_eq!(
        get_gl_balance(
            "2026-05-24",
            "_Test Company",
            AccountType::Payable,
            &entries
        ),
        BTreeMap::from([
            ("CUST-001".to_string(), -100.0),
            ("SUP-001".to_string(), 60.0),
        ])
    );
}
