use tokio_erp::erpnext::accounts::doctype::process_statement_of_accounts::process_statement_of_accounts::{
    ArFilters, CustomerRow, GlFilters, PrintFormatInfo, ProcessStatementOfAccounts,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_statement_of_accounts_matches_erpnext_core_metadata() {
    assert_eq!(
        ProcessStatementOfAccounts::DOCTYPE,
        "Process Statement Of Accounts"
    );
    assert_eq!(ProcessStatementOfAccounts::MODULE, "Accounts");
    assert_eq!(ProcessStatementOfAccounts::AUTONAME, "Prompt");
    assert_eq!(ProcessStatementOfAccounts::ROW_FORMAT, "Dynamic");
    assert_eq!(ProcessStatementOfAccounts::FIELD_ORDER.len(), 57);
    assert_eq!(ProcessStatementOfAccounts::FIELD_ORDER[0], "report");
    assert_eq!(ProcessStatementOfAccounts::FIELD_ORDER[56], "help_text");
    assert!(ProcessStatementOfAccounts::ALLOW_RENAME);
    assert!(ProcessStatementOfAccounts::EDITABLE_GRID);
    assert!(ProcessStatementOfAccounts::TRACK_CHANGES);

    let fields = ProcessStatementOfAccounts::fields();
    assert_eq!(fields.len(), 57);
    assert!(fields.contains(
        &FieldSpec::select("report", "Report")
            .options("General Ledger\nAccounts Receivable")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::date("from_date", "From Date")
            .depends_on("eval:(!doc.enable_auto_email && doc.report == 'General Ledger');")
            .mandatory_depends_on(
                "eval:(!doc.enable_auto_email && doc.report == \"General Ledger\") "
            )
    ));
    assert!(fields.contains(
        &FieldSpec::table_multiselect("cost_center", "Cost Center").options("PSOA Cost Center")
    ));
    assert!(fields.contains(
        &FieldSpec::button("fetch_customers", "Fetch Customers")
            .options("fetch_customers")
            .print_hide()
            .report_hide()
            .depends_on("eval: doc.customer_collection !== ''")
    ));
    assert!(fields.contains(&FieldSpec::text_editor("body", "Body")));
}

#[test]
fn process_statement_of_accounts_validate_defaults_and_errors_match_erpnext() {
    let mut doc = ProcessStatementOfAccounts::new(
        "_Test Process SOA",
        "_Test Company",
        "General Ledger",
        vec![CustomerRow::new("_Test Customer")],
    );

    doc.validate(None, &[], &[], None, "2026-05-23").unwrap();

    assert_eq!(
        doc.subject.as_deref(),
        Some("Statement Of Accounts for {{ customer.customer_name }}")
    );
    assert_eq!(
        doc.body.as_deref(),
        Some("Hello {{ customer.customer_name }},<br>PFA your Statement Of Accounts from {{ doc.from_date }} to {{ doc.to_date }}.")
    );
    assert_eq!(
        doc.pdf_name.as_deref(),
        Some("{{ customer.customer_name }}")
    );

    let mut no_customers = ProcessStatementOfAccounts::new(
        "_Test Process SOA",
        "_Test Company",
        "Accounts Receivable",
        Vec::new(),
    );
    assert_eq!(
        no_customers.validate(None, &[], &[], None, "2026-05-23"),
        Err("Customers not selected.".to_string())
    );

    let mut invalid_account = doc.clone();
    invalid_account.account = Some("Debtors - Other".to_string());
    assert_eq!(
        invalid_account.validate(Some("Other Company"), &[], &[], None, "2026-05-23"),
        Err("Account Debtors - Other doesn't belong to Company _Test Company".to_string())
    );

    let mut invalid_pf = doc;
    invalid_pf.print_format = Some("Bad Format".to_string());
    assert_eq!(
        invalid_pf.validate(
            None,
            &[],
            &[],
            Some(PrintFormatInfo::new(
                "Standard",
                "Report",
                "General Ledger",
                false
            )),
            "2026-05-23"
        ),
        Err("Print Format Type should be Jinja.".to_string())
    );
}

#[test]
fn process_statement_of_accounts_auto_email_dates_match_erpnext() {
    let mut doc = ProcessStatementOfAccounts::new(
        "_Test Process SOA",
        "_Test Company",
        "General Ledger",
        vec![CustomerRow::new("_Test Customer")],
    );
    doc.enable_auto_email = true;
    doc.start_date = Some("2026-06-01".to_string());
    doc.filter_duration = 2;

    doc.validate(None, &[], &[], None, "2026-05-23").unwrap();
    assert_eq!(doc.to_date.as_deref(), Some("2026-06-01"));
    assert_eq!(doc.from_date.as_deref(), Some("2026-04-01"));

    let next = doc.next_auto_email_dates("2026-06-01");
    assert_eq!(next.to_date.as_deref(), Some("2026-06-08"));
    assert_eq!(next.from_date.as_deref(), Some("2026-04-08"));
    assert_eq!(next.posting_date, None);

    let mut ar = ProcessStatementOfAccounts::new(
        "_Test Process SOA",
        "_Test Company",
        "Accounts Receivable",
        vec![CustomerRow::new("_Test Customer")],
    );
    ar.frequency = "Monthly".to_string();
    let next = ar.next_auto_email_dates("2026-01-31");
    assert_eq!(next.posting_date.as_deref(), Some("2026-02-28"));
}

#[test]
fn process_statement_of_accounts_filter_builders_match_erpnext() {
    let mut doc = ProcessStatementOfAccounts::new(
        "_Test Process SOA",
        "_Test Company",
        "General Ledger",
        vec![CustomerRow::named("_Test Customer", "Test Customer")],
    );
    doc.account = Some("Debtors - TC".to_string());
    doc.finance_book = Some("Primary".to_string());
    doc.cost_center = vec!["Main - TC".to_string()];
    doc.project = vec!["PROJ-001".to_string()];
    doc.show_remarks = true;
    doc.from_date = Some("2026-05-01".to_string());
    doc.to_date = Some("2026-05-31".to_string());
    doc.categorize_by = Some("Categorize by Voucher (Consolidated)".to_string());
    doc.currency = Some("USD".to_string());
    doc.show_net_values_in_party_account = true;

    let common = doc.common_filters();
    assert_eq!(common.company, "_Test Company");
    assert_eq!(common.account, vec!["Debtors - TC"]);
    assert_eq!(common.cost_center, vec!["Main - TC"]);
    assert!(common.show_remarks);

    assert_eq!(
        doc.gl_filters(&doc.customers[0], Some("TAX-1"), "USD"),
        GlFilters {
            from_date: Some("2026-05-01".to_string()),
            to_date: Some("2026-05-31".to_string()),
            party_type: "Customer".to_string(),
            party: vec!["_Test Customer".to_string()],
            party_name: vec!["Test Customer".to_string()],
            presentation_currency: "USD".to_string(),
            categorize_by: Some("Categorize by Voucher (Consolidated)".to_string()),
            currency: Some("USD".to_string()),
            project: vec!["PROJ-001".to_string()],
            show_opening_entries: false,
            include_default_book_entries: false,
            tax_id: Some("TAX-1".to_string()),
            show_net_values_in_party_account: true,
        }
    );

    doc.report = "Accounts Receivable".to_string();
    doc.posting_date = Some("2026-05-31".to_string());
    doc.payment_terms_template = Some("Net 30".to_string());
    doc.based_on_payment_terms = true;
    doc.show_future_payments = true;
    assert_eq!(
        doc.ar_filters(&doc.customers[0]),
        ArFilters {
            report_date: Some("2026-05-31".to_string()),
            party_type: "Customer".to_string(),
            party: vec!["_Test Customer".to_string()],
            customer_name: Some("Test Customer".to_string()),
            payment_terms_template: Some("Net 30".to_string()),
            sales_partner: None,
            sales_person: None,
            territory: None,
            based_on_payment_terms: true,
            show_future_payments: true,
            report_name: "Accounts Receivable".to_string(),
            ageing_based_on: "Due Date".to_string(),
            range1: 30,
            range2: 60,
            range3: 90,
            range4: 120,
        }
    );
}

#[test]
fn process_statement_of_accounts_recipients_and_hooks_match_erpnext() {
    let mut customer = CustomerRow::new("_Test Customer");
    customer.primary_email = Some("primary@example.com, owner@example.com".to_string());
    customer.billing_email = Some("billing@example.com".to_string());

    let mut doc = ProcessStatementOfAccounts::new(
        "_Test Process SOA",
        "_Test Company",
        "General Ledger",
        vec![customer],
    );
    doc.cc_to = vec![("user@example.com".to_string(), "cc@example.com".to_string())];
    doc.primary_mandatory = true;

    let (recipients, cc) = doc.recipients_and_cc("_Test Customer");
    assert_eq!(
        recipients,
        vec![
            "billing@example.com",
            "primary@example.com",
            "owner@example.com"
        ]
    );
    assert_eq!(cc, vec!["cc@example.com"]);
    assert_eq!(doc.custom_hooks(), ["validate"]);
    assert_eq!(doc.doctype(), "Process Statement Of Accounts");
    assert_eq!(doc.module(), "Accounts");
}
