use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::report::customer_ledger_summary::customer_ledger_summary::{
    execute, scrub, PartyDetail, PartyLedgerArgs, PartyLedgerColumn, PartyLedgerFilters,
    PartyLedgerGlEntry, PartyLedgerSummaryContext, PartyType,
};

fn filters() -> PartyLedgerFilters {
    PartyLedgerFilters {
        company: "_Test Company".to_string(),
        from_date: "2026-05-01".to_string(),
        to_date: "2026-05-31".to_string(),
    }
}

fn customer_args() -> PartyLedgerArgs {
    PartyLedgerArgs {
        party_type: PartyType::Customer,
        party_naming_by: "Naming Series".to_string(),
    }
}

fn customer_detail(party: &str) -> PartyDetail {
    PartyDetail {
        party: party.to_string(),
        party_name: format!("{party} name"),
        party_group: "Retail".to_string(),
        territory: Some("Uzbekistan".to_string()),
    }
}

fn gle(
    party: &str,
    posting_date: &str,
    voucher_no: &str,
    against_voucher: Option<&str>,
    debit: f64,
    credit: f64,
    is_opening: &str,
) -> PartyLedgerGlEntry {
    PartyLedgerGlEntry {
        posting_date: posting_date.to_string(),
        party: party.to_string(),
        voucher_type: "Sales Invoice".to_string(),
        voucher_no: voucher_no.to_string(),
        against_voucher: against_voucher.map(str::to_string),
        debit,
        credit,
        is_opening: is_opening.to_string(),
    }
}

#[test]
fn customer_ledger_summary_columns_match_erpnext_customer_shape_with_adjustments() {
    let context = PartyLedgerSummaryContext {
        party_adjustment_accounts: vec!["Discount Allowed".to_string()],
        ..PartyLedgerSummaryContext::default()
    };

    let report = execute(&filters(), &customer_args(), &context).unwrap();

    assert_eq!(
        report.columns,
        vec![
            PartyLedgerColumn::link("Customer", "party", "Customer", 200),
            PartyLedgerColumn::data("Customer Name", "party_name", 150),
            PartyLedgerColumn::link("Customer Group", "customer_group", "Customer Group", 120),
            PartyLedgerColumn::link("Territory", "territory", "Territory", 120),
            PartyLedgerColumn::currency("Opening Balance", "opening_balance", 120),
            PartyLedgerColumn::currency("Invoiced Amount", "invoiced_amount", 120),
            PartyLedgerColumn::currency("Paid Amount", "paid_amount", 120),
            PartyLedgerColumn::currency("Credit Note", "return_amount", 120),
            PartyLedgerColumn::adjustment("Discount Allowed", "adj_discount_allowed"),
            PartyLedgerColumn::currency("Closing Balance", "closing_balance", 120),
            PartyLedgerColumn::hidden_currency(),
            PartyLedgerColumn::data("Dr/Cr", "dr_or_cr", 100),
        ]
    );
}

#[test]
fn customer_ledger_summary_applies_customer_amount_return_and_adjustment_formulas() {
    let mut context = PartyLedgerSummaryContext {
        party_details: BTreeMap::from([("CUST-001".to_string(), customer_detail("CUST-001"))]),
        gl_entries: vec![
            gle("CUST-001", "2026-04-30", "OPEN-1", None, 100.0, 20.0, "No"),
            gle("CUST-001", "2026-05-02", "SI-1", None, 300.0, 0.0, "No"),
            gle("CUST-001", "2026-05-03", "PAY-1", None, 0.0, 120.0, "No"),
            gle("CUST-001", "2026-05-04", "CN-1", None, 0.0, 30.0, "No"),
            gle(
                "CUST-001",
                "2026-05-05",
                "PAY-CN",
                Some("CN-1"),
                40.0,
                0.0,
                "No",
            ),
            gle("CUST-001", "2026-05-06", "ADJ-1", None, 0.0, 10.0, "Yes"),
            gle("CUST-ZERO", "2026-05-02", "ZERO", None, 0.0, 0.0, "No"),
        ],
        return_invoices: BTreeSet::from(["CN-1".to_string()]),
        party_adjustment_accounts: vec!["Discount Allowed".to_string()],
        party_adjustment_details: BTreeMap::from([(
            "CUST-001".to_string(),
            BTreeMap::from([("Discount Allowed".to_string(), 15.0)]),
        )]),
        company_currency: "USD".to_string(),
    };
    context
        .party_details
        .insert("CUST-ZERO".to_string(), customer_detail("CUST-ZERO"));

    let report = execute(&filters(), &customer_args(), &context).unwrap();

    assert_eq!(report.rows.len(), 1);
    let row = &report.rows[0];
    assert_eq!(row.party, "CUST-001");
    assert_eq!(row.party_name.as_deref(), Some("CUST-001 name"));
    assert_eq!(row.customer_group.as_deref(), Some("Retail"));
    assert_eq!(row.territory.as_deref(), Some("Uzbekistan"));
    assert_eq!(row.opening_balance, 70.0);
    assert_eq!(row.invoiced_amount, 300.0);
    assert_eq!(row.paid_amount, 65.0);
    assert_eq!(row.return_amount, 30.0);
    assert_eq!(row.adjustments["Discount Allowed"], 15.0);
    assert_eq!(row.closing_balance, 260.0);
    assert_eq!(row.dr_or_cr.as_deref(), Some("Dr"));
    assert_eq!(row.currency, "USD");
}

#[test]
fn customer_ledger_summary_supplier_mode_reverses_amounts_and_labels() {
    let supplier_args = PartyLedgerArgs {
        party_type: PartyType::Supplier,
        party_naming_by: "Naming Series".to_string(),
    };
    let context = PartyLedgerSummaryContext {
        party_details: BTreeMap::from([(
            "SUP-001".to_string(),
            PartyDetail {
                party: "SUP-001".to_string(),
                party_name: "Supplier One".to_string(),
                party_group: "Services".to_string(),
                territory: None,
            },
        )]),
        gl_entries: vec![
            PartyLedgerGlEntry {
                posting_date: "2026-05-02".to_string(),
                party: "SUP-001".to_string(),
                voucher_type: "Purchase Invoice".to_string(),
                voucher_no: "PI-1".to_string(),
                against_voucher: None,
                debit: 0.0,
                credit: 200.0,
                is_opening: "No".to_string(),
            },
            PartyLedgerGlEntry {
                posting_date: "2026-05-03".to_string(),
                party: "SUP-001".to_string(),
                voucher_type: "Payment Entry".to_string(),
                voucher_no: "PAY-1".to_string(),
                against_voucher: None,
                debit: 50.0,
                credit: 0.0,
                is_opening: "No".to_string(),
            },
        ],
        company_currency: "USD".to_string(),
        ..PartyLedgerSummaryContext::default()
    };

    let report = execute(&filters(), &supplier_args, &context).unwrap();

    assert!(report.columns.contains(&PartyLedgerColumn::link(
        "Supplier", "party", "Supplier", 200
    )));
    assert!(report.columns.contains(&PartyLedgerColumn::currency(
        "Debit Note",
        "return_amount",
        120
    )));
    assert!(report.columns.contains(&PartyLedgerColumn::link(
        "Supplier Group",
        "supplier_group",
        "Supplier Group",
        120
    )));
    assert_eq!(report.rows[0].invoiced_amount, 200.0);
    assert_eq!(report.rows[0].paid_amount, 50.0);
    assert_eq!(report.rows[0].closing_balance, 150.0);
    assert_eq!(report.rows[0].dr_or_cr.as_deref(), Some("Cr"));
}

#[test]
fn customer_ledger_summary_validates_required_filters_and_scrubs_adjustment_accounts() {
    let missing_company = PartyLedgerFilters {
        company: String::new(),
        ..filters()
    };
    assert!(execute(
        &missing_company,
        &customer_args(),
        &PartyLedgerSummaryContext::default()
    )
    .is_err());

    let bad_dates = PartyLedgerFilters {
        from_date: "2026-06-01".to_string(),
        to_date: "2026-05-31".to_string(),
        ..filters()
    };
    assert!(execute(
        &bad_dates,
        &customer_args(),
        &PartyLedgerSummaryContext::default()
    )
    .is_err());

    assert_eq!(scrub("Discount Allowed - 10%"), "discount_allowed_10");
}
