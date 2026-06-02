use std::collections::{BTreeMap, BTreeSet};

use tokio_erp::erpnext::accounts::doctype::invoice_discounting::invoice_discounting::{
    get_invoices, get_party_account_based_on_invoice_discounting, DiscountedInvoiceRow,
    InvoiceDiscounting, InvoiceDiscountingError, InvoiceDiscountingGlInvoice,
    InvoiceDiscountingStatus, JournalAccountPlan, SalesInvoiceCandidate,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_doc() -> InvoiceDiscounting {
    InvoiceDiscounting {
        name: Some("ACC-INV-DISC-2026-00001".to_string()),
        company: "Wind Power LLC".to_string(),
        posting_date: "2026-05-01".to_string(),
        loan_start_date: Some("2026-05-10".to_string()),
        loan_period: 15,
        short_term_loan: "Short Term Loan - WP".to_string(),
        bank_account: "Bank - WP".to_string(),
        bank_charges_account: "Bank Charges - WP".to_string(),
        accounts_receivable_credit: "AR Credit - WP".to_string(),
        accounts_receivable_discounted: "AR Discounted - WP".to_string(),
        accounts_receivable_unpaid: "AR Unpaid - WP".to_string(),
        invoices: vec![DiscountedInvoiceRow {
            idx: 1,
            sales_invoice: "SI-0001".to_string(),
            customer: "Cust-1".to_string(),
            outstanding_amount: 100.0,
            parent: Some("ACC-INV-DISC-2026-00001".to_string()),
        }],
        ..Default::default()
    }
}

#[test]
fn invoice_discounting_matches_erpnext_metadata() {
    assert_eq!(InvoiceDiscounting::DOCTYPE, "Invoice Discounting");
    assert_eq!(InvoiceDiscounting::MODULE, "Accounts");
    assert_eq!(InvoiceDiscounting::AUTONAME, "ACC-INV-DISC-.YYYY.-.#####");
    assert!(InvoiceDiscounting::IS_SUBMITTABLE);
    assert_eq!(InvoiceDiscounting::FIELD_ORDER.len(), 22);
    assert_eq!(
        &InvoiceDiscounting::FIELD_ORDER[..6],
        [
            "posting_date",
            "loan_start_date",
            "loan_period",
            "loan_end_date",
            "column_break_3",
            "status",
        ]
    );

    let fields = InvoiceDiscounting::fields();
    assert!(fields.contains(
        &FieldSpec::date("posting_date", "Posting Date")
            .default("Today")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::table("invoices", "Invoices")
            .options("Discounted Invoice")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::select("status", "Status")
            .options("Draft\nSanctioned\nDisbursed\nSettled\nCancelled")
            .read_only()
            .no_copy()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("total_amount", "Total Amount")
            .options("Company:company:default_currency")
            .read_only()
    ));

    let controller = InvoiceDiscounting::default();
    assert_eq!(controller.doctype(), "Invoice Discounting");
    assert_eq!(controller.module(), "Accounts");
    assert_eq!(
        controller.custom_hooks(),
        &["validate", "on_submit", "on_cancel"]
    );
}

#[test]
fn invoice_discounting_validate_sets_totals_status_and_end_date() {
    let mut doc = base_doc();
    doc.docstatus = 1;
    doc.invoices.push(DiscountedInvoiceRow {
        idx: 2,
        sales_invoice: "SI-0002".to_string(),
        customer: "Cust-2".to_string(),
        outstanding_amount: 50.0,
        parent: Some("ACC-INV-DISC-2026-00001".to_string()),
    });

    doc.validate(
        &BTreeSet::new(),
        &BTreeMap::from([
            ("SI-0001".to_string(), 100.0),
            ("SI-0002".to_string(), 60.0),
        ]),
    )
    .expect("valid invoice discounting");

    assert_eq!(doc.total_amount, 150.0);
    assert_eq!(doc.status, InvoiceDiscountingStatus::Sanctioned);
    assert_eq!(doc.loan_end_date.as_deref(), Some("2026-05-25"));
}

#[test]
fn invoice_discounting_validate_rejects_mandatory_and_invoice_errors() {
    let mut missing = base_doc();
    missing.docstatus = 1;
    missing.loan_start_date = None;
    assert_eq!(
        missing.validate(&BTreeSet::new(), &BTreeMap::new()),
        Err(InvoiceDiscountingError::MissingLoanStartDateOrPeriod)
    );

    let mut duplicate = base_doc();
    assert_eq!(
        duplicate.validate(
            &BTreeSet::from(["SI-0001".to_string()]),
            &BTreeMap::from([("SI-0001".to_string(), 100.0)]),
        ),
        Err(InvoiceDiscountingError::AlreadyDiscounted {
            row: 1,
            sales_invoice: "SI-0001".to_string(),
            parent: Some("ACC-INV-DISC-2026-00001".to_string()),
        })
    );

    let mut too_high = base_doc();
    too_high.invoices[0].outstanding_amount = 125.0;
    assert_eq!(
        too_high.validate(
            &BTreeSet::new(),
            &BTreeMap::from([("SI-0001".to_string(), 100.0)]),
        ),
        Err(InvoiceDiscountingError::OutstandingGreaterThanActual {
            row: 1,
            sales_invoice: "SI-0001".to_string(),
            actual_outstanding: 100.0,
        })
    );
}

#[test]
fn invoice_discounting_status_and_sales_invoice_update_plans_match_docstatus() {
    let mut doc = base_doc();
    assert_eq!(doc.set_status(None, false), InvoiceDiscountingStatus::Draft);
    doc.docstatus = 1;
    assert_eq!(
        doc.set_status(None, false),
        InvoiceDiscountingStatus::Sanctioned
    );
    assert_eq!(
        doc.set_status(Some(InvoiceDiscountingStatus::Disbursed), false),
        InvoiceDiscountingStatus::Disbursed
    );

    assert_eq!(
        doc.update_sales_invoice_plan(&BTreeSet::new()),
        vec![("SI-0001".to_string(), true)]
    );
    doc.docstatus = 2;
    assert_eq!(
        doc.update_sales_invoice_plan(&BTreeSet::from(["SI-0001".to_string()])),
        vec![("SI-0001".to_string(), true)]
    );
    assert_eq!(
        doc.update_sales_invoice_plan(&BTreeSet::new()),
        vec![("SI-0001".to_string(), false)]
    );
}

#[test]
fn invoice_discounting_builds_gl_entries_for_discounted_invoices() {
    let doc = base_doc();
    let entries = doc.make_gl_entries_plan(
        "USD",
        "USD",
        &[InvoiceDiscountingGlInvoice {
            sales_invoice: "SI-0001".to_string(),
            debit_to: "Debtors - WP".to_string(),
            party_account_currency: "EUR".to_string(),
            conversion_rate: 1.2,
            cost_center: Some("Main - CC".to_string()),
            dimensions: BTreeMap::from([("project".to_string(), "PROJ-1".to_string())]),
        }],
    );

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].account, "Debtors - WP");
    assert_eq!(entries[0].credit, 120.0);
    assert_eq!(entries[0].credit_in_account_currency, 100.0);
    assert_eq!(entries[0].against, Some("AR Credit - WP".to_string()));
    assert_eq!(entries[1].account, "AR Credit - WP");
    assert_eq!(entries[1].debit, 120.0);
    assert_eq!(entries[1].debit_in_account_currency, 120.0);
}

#[test]
fn invoice_discounting_builds_disbursement_and_close_loan_journal_plans() {
    let mut doc = base_doc();
    doc.total_amount = 100.0;
    doc.bank_charges = 5.0;
    doc.loan_end_date = Some("2026-05-25".to_string());

    let je = doc.create_disbursement_entry_plan("Main - CC");
    assert_eq!(je.voucher_type, "Journal Entry");
    assert_eq!(je.accounts.len(), 5);
    assert_eq!(
        je.accounts[0],
        JournalAccountPlan {
            account: "Bank - WP".to_string(),
            debit_in_account_currency: 95.0,
            cost_center: Some("Main - CC".to_string()),
            ..Default::default()
        }
    );
    assert_eq!(je.accounts[1].account, "Bank Charges - WP");
    assert_eq!(je.accounts[2].account, "Short Term Loan - WP");
    assert_eq!(je.accounts[3].account, "AR Discounted - WP");
    assert_eq!(je.accounts[4].account, "AR Credit - WP");

    let close = doc.close_loan_plan(
        "Main - CC",
        "2026-05-20",
        &BTreeMap::from([("SI-0001".to_string(), 45.0)]),
    );
    assert_eq!(close.accounts.len(), 4);
    assert_eq!(close.accounts[0].account, "Short Term Loan - WP");
    assert_eq!(close.accounts[1].account, "Bank - WP");
    assert_eq!(close.accounts[2].account, "AR Discounted - WP");
    assert_eq!(close.accounts[2].credit_in_account_currency, 45.0);
    assert_eq!(close.accounts[3].account, "AR Unpaid - WP");
    assert_eq!(close.accounts[3].debit_in_account_currency, 45.0);

    let after_period = doc.close_loan_plan(
        "Main - CC",
        "2026-05-30",
        &BTreeMap::from([("SI-0001".to_string(), 45.0)]),
    );
    assert_eq!(after_period.accounts.len(), 2);
    assert_eq!(after_period.accounts[0].account, "Short Term Loan - WP");
    assert_eq!(after_period.accounts[1].account, "Bank - WP");
}

#[test]
fn invoice_discounting_filters_invoices_and_party_account_by_status() {
    let invoices = vec![
        SalesInvoiceCandidate {
            sales_invoice: "SI-0001".to_string(),
            customer: "Cust-1".to_string(),
            posting_date: "2026-05-01".to_string(),
            outstanding_amount: 100.0,
            debit_to: "Debtors - WP".to_string(),
            docstatus: 1,
            base_grand_total: 150.0,
        },
        SalesInvoiceCandidate {
            sales_invoice: "SI-0002".to_string(),
            customer: "Cust-2".to_string(),
            posting_date: "2026-04-01".to_string(),
            outstanding_amount: 0.0,
            debit_to: "Debtors - WP".to_string(),
            docstatus: 1,
            base_grand_total: 20.0,
        },
    ];
    let filtered = get_invoices(
        &invoices,
        Some("Cust-1"),
        Some("2026-05-01"),
        Some("2026-05-31"),
        Some(100.0),
        Some(200.0),
        &BTreeSet::new(),
    );
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].sales_invoice, "SI-0001");

    assert_eq!(
        get_party_account_based_on_invoice_discounting(
            "SI-0001",
            &[(
                "SI-0001".to_string(),
                "AR Discounted - WP".to_string(),
                "AR Unpaid - WP".to_string(),
                InvoiceDiscountingStatus::Disbursed,
            )],
        ),
        Some("AR Discounted - WP".to_string())
    );
    assert_eq!(
        get_party_account_based_on_invoice_discounting(
            "SI-0001",
            &[(
                "SI-0001".to_string(),
                "AR Discounted - WP".to_string(),
                "AR Unpaid - WP".to_string(),
                InvoiceDiscountingStatus::Settled,
            )],
        ),
        Some("AR Unpaid - WP".to_string())
    );
    assert_eq!(
        get_party_account_based_on_invoice_discounting(
            "SI-0001",
            &[(
                "SI-0001".to_string(),
                "AR Discounted - WP".to_string(),
                "AR Unpaid - WP".to_string(),
                InvoiceDiscountingStatus::Sanctioned,
            )],
        ),
        None
    );
}
