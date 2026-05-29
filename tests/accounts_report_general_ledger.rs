use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::general_ledger::general_ledger::{
    execute, get_balance, get_columns, get_conditions, get_data_with_opening_closing,
    get_gl_entries, get_group_by_field, get_result_as_list, set_account_currency, validate_filters,
    AccountDetail, AccountingDimension, CostCenterDetail, GeneralLedgerFilters, GeneralLedgerInput,
    GlEntry,
};

fn filters() -> GeneralLedgerFilters {
    GeneralLedgerFilters {
        company: "Acme".to_string(),
        from_date: "2026-01-01".to_string(),
        to_date: "2026-12-31".to_string(),
        account: vec!["Cash - A".to_string()],
        party_type: None,
        party: Vec::new(),
        voucher_no: None,
        against_voucher_no: None,
        voucher_no_not_in: Vec::new(),
        ignore_err: false,
        ignore_cr_dr_notes: false,
        project: Vec::new(),
        cost_center: Vec::new(),
        finance_book: None,
        company_fb: Some("Main FB".to_string()),
        include_default_book_entries: true,
        categorize_by: None,
        group_by: None,
        show_cancelled_entries: false,
        disable_opening_balance_calculation: false,
        show_opening_entries: false,
        include_dimensions: false,
        show_remarks: true,
        add_values_in_transaction_currency: true,
        print_in_account_currency: false,
        presentation_currency: None,
        account_currency: None,
        company_currency: None,
        show_amount_in_company_currency: false,
        show_net_values_in_party_account: false,
    }
}

fn input() -> GeneralLedgerInput {
    GeneralLedgerInput {
        company_currency: "USD".to_string(),
        default_company: "Acme".to_string(),
        immutable_ledger: false,
        accounting_dimensions: Vec::new(),
        show_party_name_column: false,
        exchange_rate_revaluation_vouchers: Vec::new(),
        system_generated_cr_dr_vouchers: Vec::new(),
        accounts: BTreeMap::from([
            (
                "Cash - A".to_string(),
                AccountDetail {
                    is_group: false,
                    account_currency: "USD".to_string(),
                    account_type: Some("Cash".to_string()),
                    parent_account: Some("Assets - A".to_string()),
                    lft: 2,
                    rgt: 3,
                },
            ),
            (
                "Receivable - A".to_string(),
                AccountDetail {
                    is_group: false,
                    account_currency: "USD".to_string(),
                    account_type: Some("Receivable".to_string()),
                    parent_account: Some("Assets - A".to_string()),
                    lft: 4,
                    rgt: 5,
                },
            ),
            (
                "Assets - A".to_string(),
                AccountDetail {
                    is_group: true,
                    account_currency: "USD".to_string(),
                    account_type: None,
                    parent_account: None,
                    lft: 1,
                    rgt: 6,
                },
            ),
        ]),
        valid_parties: BTreeMap::from([(
            "Customer".to_string(),
            vec!["CUST-1".to_string(), "CUST-2".to_string()],
        )]),
        party_names: BTreeMap::from([(
            "Customer".to_string(),
            BTreeMap::from([("CUST-1".to_string(), "Customer One".to_string())]),
        )]),
        party_currencies: BTreeMap::from([(
            "Customer".to_string(),
            BTreeMap::from([("CUST-1".to_string(), "EUR".to_string())]),
        )]),
        cost_centers: BTreeMap::from([
            ("Main - A".to_string(), CostCenterDetail { lft: 1, rgt: 4 }),
            ("Sales - A".to_string(), CostCenterDetail { lft: 2, rgt: 3 }),
        ]),
        supplier_invoice_details: BTreeMap::from([("PINV-1".to_string(), "BILL-1".to_string())]),
        gl_entries: vec![
            GlEntry::new(
                "GLE-OPEN",
                "2025-12-31",
                "Cash - A",
                100.0,
                0.0,
                "Yes",
                "Payment Entry",
                "PE-OPEN",
            ),
            GlEntry::new(
                "GLE-1",
                "2026-02-01",
                "Cash - A",
                50.0,
                0.0,
                "No",
                "Sales Invoice",
                "SINV-1",
            )
            .with_party("Customer", "CUST-1")
            .with_against("Purchase Invoice", "PINV-1"),
            GlEntry::new(
                "GLE-2",
                "2026-03-01",
                "Cash - A",
                0.0,
                20.0,
                "No",
                "Payment Entry",
                "PE-1",
            )
            .with_transaction_currency(0.0, 22.0, "EUR"),
            GlEntry::new(
                "GLE-CANCEL",
                "2026-04-01",
                "Cash - A",
                999.0,
                0.0,
                "No",
                "Journal Entry",
                "JV-CANCEL",
            )
            .cancelled(),
            GlEntry::new(
                "GLE-OTHER-PARTY",
                "2026-05-01",
                "Cash - A",
                700.0,
                0.0,
                "No",
                "Sales Invoice",
                "SINV-OTHER-PARTY",
            )
            .with_party("Customer", "CUST-2"),
            GlEntry::new(
                "GLE-OTHER-AGAINST",
                "2026-06-01",
                "Cash - A",
                800.0,
                0.0,
                "No",
                "Sales Invoice",
                "SINV-OTHER-AGAINST",
            )
            .with_against("Purchase Invoice", "PINV-2"),
        ],
    }
}

#[test]
fn general_ledger_excludes_voucher_no_not_in_and_ignored_journals_like_erpnext() {
    let mut input = input();
    input.exchange_rate_revaluation_vouchers = vec!["JV-ERR".to_string()];
    input.system_generated_cr_dr_vouchers = vec!["JV-CRDR".to_string()];
    input.gl_entries.push(GlEntry::new(
        "GLE-ERR",
        "2026-08-01",
        "Cash - A",
        40.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-ERR",
    ));
    input.gl_entries.push(GlEntry::new(
        "GLE-CRDR",
        "2026-08-02",
        "Cash - A",
        50.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-CRDR",
    ));
    input.gl_entries.push(GlEntry::new(
        "GLE-EXPLICIT",
        "2026-08-03",
        "Cash - A",
        60.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-EXPLICIT",
    ));

    let mut f = filters();
    f.ignore_err = true;
    f.ignore_cr_dr_notes = true;
    f.voucher_no_not_in = vec!["JV-EXPLICIT".to_string()];

    let entries = get_gl_entries(&f, &input);
    assert!(!entries
        .iter()
        .any(|row| row.voucher_no.as_deref() == Some("JV-ERR")));
    assert!(!entries
        .iter()
        .any(|row| row.voucher_no.as_deref() == Some("JV-CRDR")));
    assert!(!entries
        .iter()
        .any(|row| row.voucher_no.as_deref() == Some("JV-EXPLICIT")));

    let conditions = get_conditions(&f);
    assert!(conditions.contains(&"voucher_no not in %(voucher_no_not_in)s".to_string()));
}

#[test]
fn general_ledger_group_cost_center_filter_includes_child_cost_centers() {
    let mut input = input();
    let mut child_cost_center = GlEntry::new(
        "GLE-CHILD-COST-CENTER",
        "2026-08-01",
        "Cash - A",
        40.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-CHILD-CC",
    );
    child_cost_center.cost_center = Some("Sales - A".to_string());
    input.gl_entries.push(child_cost_center);

    let mut f = filters();
    f.cost_center = vec!["Main - A".to_string()];

    let entries = get_gl_entries(&f, &input);
    assert!(entries
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-CHILD-COST-CENTER")));
}

#[test]
fn general_ledger_party_only_currency_uses_gl_entry_currency_then_party_default() {
    let mut input = input();
    input.gl_entries[1].account_currency = Some("GBP".to_string());

    let mut f = filters();
    f.account.clear();
    f.party_type = Some("Customer".to_string());
    f.party = vec!["CUST-1".to_string()];

    let f = set_account_currency(f, &input).unwrap();
    assert_eq!(f.account_currency.as_deref(), Some("GBP"));
    assert_eq!(f.presentation_currency.as_deref(), Some("GBP"));

    input.gl_entries[1].account_currency = None;
    let mut f = filters();
    f.account.clear();
    f.party_type = Some("Customer".to_string());
    f.party = vec!["CUST-1".to_string()];

    let f = set_account_currency(f, &input).unwrap();
    assert_eq!(f.account_currency.as_deref(), Some("EUR"));
    assert_eq!(f.presentation_currency.as_deref(), Some("EUR"));
}

#[test]
fn general_ledger_group_account_filter_includes_child_accounts() {
    let mut input = input();
    input.gl_entries.push(GlEntry::new(
        "GLE-RECEIVABLE",
        "2026-08-01",
        "Receivable - A",
        30.0,
        0.0,
        "No",
        "Sales Invoice",
        "SINV-RECEIVABLE",
    ));

    let mut f = filters();
    f.account = vec!["Assets - A".to_string()];

    let entries = get_gl_entries(&f, &input);
    assert!(entries
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-RECEIVABLE")));
}

#[test]
fn general_ledger_finance_book_filter_matches_erpnext_default_book_rules() {
    let mut input = input();
    input.gl_entries[1].finance_book = Some("IFRS".to_string());
    input.gl_entries[2].finance_book = Some("Local GAAP".to_string());
    input.gl_entries.push(GlEntry::new(
        "GLE-EMPTY-FB",
        "2026-08-01",
        "Cash - A",
        10.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-EMPTY-FB",
    ));

    let mut f = filters();
    f.finance_book = Some("IFRS".to_string());
    f.company_fb = Some("IFRS".to_string());
    f.include_default_book_entries = true;

    let entries = get_gl_entries(&f, &input);
    assert!(entries
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-1")));
    assert!(entries
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-EMPTY-FB")));
    assert!(!entries
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-2")));

    let mut different_book = f;
    different_book.finance_book = Some("Tax".to_string());
    different_book.company_fb = Some("IFRS".to_string());
    assert_eq!(
        validate_filters(&mut different_book, &input).unwrap_err(),
        "To use a different finance book, please uncheck 'Include Default FB Entries'"
    );
}

#[test]
fn general_ledger_validate_filters_and_account_currency_match_erpnext() {
    let mut f = filters();
    validate_filters(&mut f, &input()).unwrap();
    let f = set_account_currency(f, &input()).unwrap();

    assert_eq!(f.company_currency.as_deref(), Some("USD"));
    assert_eq!(f.account_currency.as_deref(), Some("USD"));

    let mut missing = filters();
    missing.company.clear();
    assert_eq!(
        validate_filters(&mut missing, &input()).unwrap_err(),
        "Company is mandatory"
    );

    let mut grouped_child = filters();
    grouped_child.categorize_by = Some("Categorize by Account".to_string());
    assert_eq!(
        validate_filters(&mut grouped_child, &input()).unwrap_err(),
        "Can not filter based on Child Account, if grouped by Account"
    );

    let mut invalid_cost_center = filters();
    invalid_cost_center.cost_center = vec!["Missing - A".to_string()];
    assert_eq!(
        validate_filters(&mut invalid_cost_center, &input()).unwrap_err(),
        "Cost Center: Missing - A does not exist"
    );
}

#[test]
fn general_ledger_conditions_follow_filters_and_finance_book_rules() {
    let f = filters();
    let conditions = get_conditions(&f);

    assert!(conditions.contains(&"account in %(account)s".to_string()));
    assert!(conditions.contains(&"(posting_date <=%(to_date)s or is_opening = 'Yes')".to_string()));
    assert!(conditions
        .contains(&"(finance_book in (%(company_fb)s, '') OR finance_book IS NULL)".to_string()));
    assert!(conditions.contains(&"is_cancelled = 0".to_string()));

    let mut party_grouped = filters();
    party_grouped.account.clear();
    party_grouped.categorize_by = Some("Categorize by Party".to_string());
    let conditions = get_conditions(&party_grouped);
    assert!(conditions.contains(&"party_type in ('Customer', 'Supplier')".to_string()));
}

#[test]
fn general_ledger_execute_applies_party_against_voucher_and_project_filters() {
    let mut f = filters();
    f.party_type = Some("Customer".to_string());
    f.party = vec!["CUST-1".to_string()];
    f.against_voucher_no = Some("PINV-1".to_string());
    let report = execute(set_account_currency(f, &input()).unwrap(), input()).unwrap();

    assert!(report
        .rows
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-1")));
    assert!(!report
        .rows
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-OTHER-PARTY")));
    assert!(!report
        .rows
        .iter()
        .any(|row| row.gl_entry.as_deref() == Some("GLE-OTHER-AGAINST")));
}

#[test]
fn general_ledger_default_output_groups_entries_by_voucher_like_erpnext_gle_map() {
    let mut input = input();
    input.gl_entries.push(
        GlEntry::new(
            "GLE-SINV-1-LATE",
            "2026-07-01",
            "Cash - A",
            5.0,
            0.0,
            "No",
            "Sales Invoice",
            "SINV-1",
        )
        .with_party("Customer", "CUST-1"),
    );

    let report = execute(set_account_currency(filters(), &input).unwrap(), input).unwrap();
    let positions = report
        .rows
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| (row.voucher_no.as_deref() == Some("SINV-1")).then_some(idx))
        .collect::<Vec<_>>();

    assert_eq!(positions.len(), 2);
    assert_eq!(positions[1], positions[0] + 1);
}

#[test]
fn general_ledger_opening_total_closing_and_running_balance_are_added() {
    let f = set_account_currency(filters(), &input()).unwrap();
    let report = execute(f, input()).unwrap();

    assert_eq!(report.columns[0].fieldname, "gl_entry");
    assert_eq!(report.rows[0].account.as_deref(), Some("'Opening'"));
    assert_eq!(report.rows[0].debit, 100.0);
    assert_eq!(report.rows[0].balance, 100.0);
    assert_eq!(report.rows[0].presentation_currency.as_deref(), Some("USD"));

    let invoice = report
        .rows
        .iter()
        .find(|row| row.gl_entry.as_deref() == Some("GLE-1"))
        .unwrap();
    assert_eq!(invoice.party_name.as_deref(), Some("Customer One"));
    assert_eq!(invoice.bill_no.as_deref(), Some("BILL-1"));
    assert_eq!(invoice.balance, 150.0);

    let total = report
        .rows
        .iter()
        .find(|row| row.account.as_deref() == Some("'Total'"))
        .unwrap();
    assert_eq!(total.debit, 1550.0);
    assert_eq!(total.credit, 20.0);
    assert_eq!(total.debit_in_transaction_currency, None);
    assert_eq!(total.credit_in_transaction_currency, None);

    let closing = report.rows.last().unwrap();
    assert_eq!(
        closing.account.as_deref(),
        Some("'Closing (Opening + Total)'")
    );
    assert_eq!(closing.balance, 1630.0);
}

#[test]
fn general_ledger_grouping_by_account_adds_group_opening_total_and_closing() {
    let mut f = filters();
    f.account.clear();
    f.categorize_by = Some("Categorize by Account".to_string());
    let data = get_data_with_opening_closing(&f, &input().gl_entries, &input());

    assert!(data.iter().any(|row| row.is_blank));
    assert!(data
        .iter()
        .any(|row| row.account.as_deref() == Some("'Opening'")));
    assert!(data
        .iter()
        .any(|row| row.account.as_deref() == Some("'Closing (Opening + Total)'")));
    assert_eq!(get_group_by_field(f.categorize_by.as_deref()), "account");
}

#[test]
fn general_ledger_consolidates_voucher_rows_and_columns_include_transaction_currency() {
    let mut f = filters();
    f.categorize_by = Some("Categorize by Voucher (Consolidated)".to_string());
    let mut entries = input().gl_entries;
    entries.push(
        GlEntry::new(
            "GLE-1B",
            "2026-02-01",
            "Cash - A",
            25.0,
            0.0,
            "No",
            "Sales Invoice",
            "SINV-1",
        )
        .with_party("Customer", "CUST-1"),
    );
    let mut input = input();
    input.gl_entries = entries;

    let data = get_data_with_opening_closing(&f, &input.gl_entries, &input);
    let consolidated = data
        .iter()
        .find(|row| row.voucher_no.as_deref() == Some("SINV-1"))
        .unwrap();
    assert_eq!(consolidated.debit, 75.0);

    let columns = get_columns(&f, &input).unwrap();
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "transaction_currency"));
    assert!(columns.iter().any(|column| column.fieldname == "remarks"));
}

#[test]
fn general_ledger_columns_match_dynamic_link_party_name_and_dimension_rules() {
    let mut f = filters();
    f.add_values_in_transaction_currency = false;
    f.show_remarks = false;
    let mut input = input();

    let columns = get_columns(&f, &input).unwrap();
    assert_eq!(
        columns
            .iter()
            .find(|column| column.fieldname == "voucher_no")
            .unwrap()
            .fieldtype,
        "Dynamic Link"
    );
    assert_eq!(
        columns
            .iter()
            .find(|column| column.fieldname == "against_voucher")
            .unwrap()
            .fieldtype,
        "Dynamic Link"
    );
    assert!(!columns
        .iter()
        .any(|column| column.fieldname == "party_name"));

    input.show_party_name_column = true;
    let columns = get_columns(&f, &input).unwrap();
    assert!(columns
        .iter()
        .any(|column| column.fieldname == "party_name"));

    f.include_dimensions = true;
    input.accounting_dimensions = vec![AccountingDimension {
        label: "Branch".to_string(),
        fieldname: "branch".to_string(),
    }];
    let columns = get_columns(&f, &input).unwrap();
    let ordered_fields = columns
        .iter()
        .map(|column| column.fieldname.as_str())
        .collect::<Vec<_>>();
    let party_idx = ordered_fields
        .iter()
        .position(|field| *field == "party")
        .unwrap();
    let project_idx = ordered_fields
        .iter()
        .position(|field| *field == "project")
        .unwrap();
    let branch_idx = ordered_fields
        .iter()
        .position(|field| *field == "branch")
        .unwrap();
    let cost_center_idx = ordered_fields
        .iter()
        .position(|field| *field == "cost_center")
        .unwrap();
    let against_idx = ordered_fields
        .iter()
        .position(|field| *field == "against_voucher_type")
        .unwrap();
    assert!(party_idx < project_idx);
    assert!(project_idx < branch_idx);
    assert!(branch_idx < cost_center_idx);
    assert!(cost_center_idx < against_idx);
}

#[test]
fn general_ledger_consolidated_voucher_key_keeps_dimension_and_creation_rows_separate() {
    let mut f = filters();
    f.categorize_by = Some("Categorize by Voucher (Consolidated)".to_string());
    f.include_dimensions = true;

    let mut first = GlEntry::new(
        "GLE-DIM-1",
        "2026-09-01",
        "Cash - A",
        10.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-DIM",
    );
    first.cost_center = Some("Main - A".to_string());
    first
        .dimensions
        .insert("branch".to_string(), "North".to_string());

    let mut second = GlEntry::new(
        "GLE-DIM-2",
        "2026-09-01",
        "Cash - A",
        25.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-DIM",
    );
    second.cost_center = Some("Main - A".to_string());
    second
        .dimensions
        .insert("branch".to_string(), "South".to_string());

    let mut dim_input = input();
    dim_input.accounting_dimensions = vec![AccountingDimension {
        label: "Branch".to_string(),
        fieldname: "branch".to_string(),
    }];
    dim_input.gl_entries = vec![first, second];
    let data = get_data_with_opening_closing(&f, &dim_input.gl_entries, &dim_input);
    let entries = data
        .iter()
        .filter(|row| row.voucher_no.as_deref() == Some("JV-DIM"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 2);

    let mut f = filters();
    f.categorize_by = Some("Categorize by Voucher (Consolidated)".to_string());
    let mut first = GlEntry::new(
        "GLE-CREATE-1",
        "2026-09-02",
        "Cash - A",
        10.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-CREATE",
    );
    first.creation = Some("2026-09-02 10:00:00".to_string());
    let mut second = GlEntry::new(
        "GLE-CREATE-2",
        "2026-09-02",
        "Cash - A",
        25.0,
        0.0,
        "No",
        "Journal Entry",
        "JV-CREATE",
    );
    second.creation = Some("2026-09-02 10:01:00".to_string());

    let mut input = input();
    input.immutable_ledger = true;
    input.gl_entries = vec![first, second];
    let data = get_data_with_opening_closing(&f, &input.gl_entries, &input);
    let entries = data
        .iter()
        .filter(|row| row.voucher_no.as_deref() == Some("JV-CREATE"))
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 2);
}

#[test]
fn general_ledger_get_balance_is_debit_minus_credit() {
    let mut row = GlEntry::empty_row();
    row.debit = 10.0;
    row.credit = 3.0;
    assert_eq!(get_balance(&row, 5.0, "debit", "credit"), 12.0);

    let mut rows = vec![row];
    let f = set_account_currency(filters(), &input()).unwrap();
    get_result_as_list(&mut rows, &f);
    assert_eq!(rows[0].balance, 7.0);
    assert_eq!(rows[0].account_currency.as_deref(), Some("USD"));
}
