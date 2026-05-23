use tokio_erp::erpnext::accounts::doctype::unreconcile_payment::unreconcile_payment::{
    create_unreconcile_docs_for_selection, doc_has_references, get_linked_payments_for_doc,
    AdvancePaymentLedgerEntry, PaymentLedgerEntry, UnreconcilePayment, UnreconcileSelection,
    UnreconcileSubmitAction,
};
use tokio_erp::erpnext::accounts::doctype::unreconcile_payment_entries::unreconcile_payment_entries::UnreconcilePaymentEntries;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn unreconcile_payment_matches_erpnext_metadata_and_client_hooks() {
    assert_eq!(UnreconcilePayment::DOCTYPE, "Unreconcile Payment");
    assert_eq!(UnreconcilePayment::MODULE, "Accounts");
    assert_eq!(
        UnreconcilePayment::FIELD_ORDER,
        [
            "company",
            "voucher_type",
            "voucher_no",
            "get_allocations",
            "allocations",
            "amended_from"
        ]
    );
    assert!(UnreconcilePayment::IS_SUBMITTABLE);
    assert!(UnreconcilePayment::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        UnreconcilePayment::fields(),
        vec![
            FieldSpec::link("amended_from", "Amended From")
                .options("Unreconcile Payment")
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::link("voucher_type", "Voucher Type").options("DocType"),
            FieldSpec::dynamic_link("voucher_no")
                .label("Voucher No")
                .options("voucher_type"),
            FieldSpec::button("get_allocations", "Get Allocations"),
            FieldSpec::table("allocations", "Allocations").options("Unreconcile Payment Entries"),
        ]
    );
    assert_eq!(
        UnreconcilePayment::supported_types(),
        ["Payment Entry", "Journal Entry"]
    );
    assert_eq!(
        UnreconcilePayment::voucher_type_query_filter(),
        ["Payment Entry", "Journal Entry"]
    );
    assert_eq!(
        UnreconcilePayment::voucher_no_query_filter("Acme"),
        [("company", "Acme"), ("docstatus", "1")]
    );
}

#[test]
fn unreconcile_payment_entries_match_erpnext_metadata() {
    assert_eq!(
        UnreconcilePaymentEntries::DOCTYPE,
        "Unreconcile Payment Entries"
    );
    assert_eq!(UnreconcilePaymentEntries::MODULE, "Accounts");
    assert_eq!(
        UnreconcilePaymentEntries::FIELD_ORDER,
        [
            "account",
            "party_type",
            "party",
            "reference_doctype",
            "reference_name",
            "allocated_amount",
            "account_currency",
            "unlinked"
        ]
    );
    assert!(UnreconcilePaymentEntries::IS_TABLE);
    assert!(UnreconcilePaymentEntries::EDITABLE_GRID);
    assert!(UnreconcilePaymentEntries::INDEX_WEB_PAGES_FOR_SEARCH);
    assert_eq!(
        UnreconcilePaymentEntries::fields(),
        vec![
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_doctype")
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("account_currency")
                .in_list_view(),
            FieldSpec::check("unlinked", "Unlinked")
                .default("0")
                .read_only()
                .in_list_view(),
            FieldSpec::link("reference_doctype", "Reference Type")
                .options("DocType")
                .in_list_view(),
            FieldSpec::data("account", "Account"),
            FieldSpec::data("party_type", "Party Type"),
            FieldSpec::data("party", "Party"),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .read_only(),
        ]
    );
}

#[test]
fn validate_accepts_only_payment_and_journal_entry_like_erpnext() {
    let payment = UnreconcilePayment::new("Acme", "Payment Entry", "PE-0001");
    assert_eq!(payment.validate(), Ok(()));

    let journal = UnreconcilePayment::new("Acme", "Journal Entry", "JV-0001");
    assert_eq!(journal.validate(), Ok(()));

    let sales_invoice = UnreconcilePayment::new("Acme", "Sales Invoice", "SI-0001");
    assert_eq!(
        sales_invoice.validate(),
        Err("Only Payment Entry and Journal Entry are supported".to_string())
    );
}

#[test]
fn doc_has_references_matches_invoice_payment_and_advance_counts() {
    let payment_entries = vec![
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0001",
            -100.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0002",
            "Sales Invoice",
            "SI-0001",
            25.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0003",
            "PE-0003",
            "PE-0003",
            -5.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0003",
            "Sales Order",
            "SO-0001",
            50.0,
        ),
    ];
    let advances = vec![
        advance_ledger(
            "Acme",
            "Payment Entry",
            "PE-0003",
            "Sales Order",
            "SO-0002",
            75.0,
        ),
        AdvancePaymentLedgerEntry {
            event: "Cancel".to_string(),
            ..advance_ledger(
                "Acme",
                "Payment Entry",
                "PE-0003",
                "Sales Order",
                "SO-0003",
                30.0,
            )
        },
    ];

    assert_eq!(
        doc_has_references(
            Some("Sales Invoice"),
            Some("SI-0001"),
            &payment_entries,
            &advances
        ),
        1
    );
    assert_eq!(
        doc_has_references(
            Some("Payment Entry"),
            Some("PE-0003"),
            &payment_entries,
            &advances
        ),
        2
    );
}

#[test]
fn linked_payments_for_invoice_group_negative_payment_ledger_by_payment_voucher() {
    let payment_entries = vec![
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0001",
            -40.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0001",
            -60.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0002",
            "Sales Invoice",
            "SI-0001",
            10.0,
        ),
        payment_ledger(
            "Other",
            "Payment Entry",
            "PE-0003",
            "Sales Invoice",
            "SI-0001",
            -20.0,
        ),
    ];

    let rows = get_linked_payments_for_doc(
        Some("Acme"),
        Some("Sales Invoice"),
        Some("SI-0001"),
        &payment_entries,
        &[],
    );

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].reference_doctype.as_deref(), Some("Payment Entry"));
    assert_eq!(rows[0].reference_name.as_deref(), Some("PE-0001"));
    assert_eq!(rows[0].allocated_amount, 100.0);
    assert_eq!(rows[0].account.as_deref(), Some("Debtors - A"));
    assert_eq!(rows[0].account_currency.as_deref(), Some("USD"));
}

#[test]
fn linked_payments_for_payment_entry_include_payment_ledger_and_advances() {
    let payment_entries = vec![
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0001",
            40.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0001",
            60.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "PE-0001",
            "PE-0001",
            7.0,
        ),
    ];
    let advances = vec![
        advance_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Order",
            "SO-0001",
            75.0,
        ),
        AdvancePaymentLedgerEntry {
            delinked: true,
            ..advance_ledger(
                "Acme",
                "Payment Entry",
                "PE-0001",
                "Sales Order",
                "SO-0002",
                25.0,
            )
        },
    ];

    let rows = get_linked_payments_for_doc(
        Some("Acme"),
        Some("Payment Entry"),
        Some("PE-0001"),
        &payment_entries,
        &advances,
    );

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].reference_doctype.as_deref(), Some("Sales Invoice"));
    assert_eq!(rows[0].reference_name.as_deref(), Some("SI-0001"));
    assert_eq!(rows[0].allocated_amount, 100.0);
    assert_eq!(rows[1].reference_doctype.as_deref(), Some("Sales Order"));
    assert_eq!(rows[1].reference_name.as_deref(), Some("SO-0001"));
    assert_eq!(rows[1].allocated_amount, 75.0);
    assert_eq!(rows[1].account_currency.as_deref(), Some("USD"));
}

#[test]
fn add_references_and_on_submit_preserve_erpnext_unreconcile_sequence() {
    let mut doc = UnreconcilePayment::new("Acme", "Payment Entry", "PE-0001");
    doc.add_references(vec![UnreconcilePaymentEntries::new(
        "Sales Invoice",
        "SI-0001",
        100.0,
    )
    .with_account_party("Debtors - A", "Customer", "CUST-0001")
    .with_name("ALLOC-0001")]);

    let actions = doc.on_submit();

    assert_eq!(
        actions,
        vec![UnreconcileSubmitAction {
            reference_doctype: "Sales Invoice".to_string(),
            reference_name: "SI-0001".to_string(),
            payment_voucher_no: "PE-0001".to_string(),
            payment_voucher_type: "Payment Entry".to_string(),
            account: Some("Debtors - A".to_string()),
            party_type: Some("Customer".to_string()),
            party: Some("CUST-0001".to_string()),
            allocation_name: Some("ALLOC-0001".to_string()),
            mark_unlinked: true,
        }]
    );
    assert_eq!(doc.custom_hooks(), ["validate", "on_submit"]);
    assert_eq!(doc.doctype(), "Unreconcile Payment");
    assert_eq!(doc.module(), "Accounts");
}

#[test]
fn create_unreconcile_docs_for_selection_filters_unselected_references() {
    let payment_entries = vec![
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0001",
            40.0,
        ),
        payment_ledger(
            "Acme",
            "Payment Entry",
            "PE-0001",
            "Sales Invoice",
            "SI-0002",
            60.0,
        ),
    ];
    let selections = vec![UnreconcileSelection {
        company: "Acme".to_string(),
        voucher_type: "Payment Entry".to_string(),
        voucher_no: "PE-0001".to_string(),
        against_voucher_type: "Sales Invoice".to_string(),
        against_voucher_no: "SI-0002".to_string(),
    }];

    let docs = create_unreconcile_docs_for_selection(&selections, &payment_entries, &[]);

    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].company.as_deref(), Some("Acme"));
    assert_eq!(docs[0].voucher_type.as_deref(), Some("Payment Entry"));
    assert_eq!(docs[0].voucher_no.as_deref(), Some("PE-0001"));
    assert_eq!(docs[0].allocations.len(), 1);
    assert_eq!(
        docs[0].allocations[0].reference_name.as_deref(),
        Some("SI-0002")
    );
}

fn payment_ledger(
    company: &str,
    voucher_type: &str,
    voucher_no: &str,
    against_voucher_type: &str,
    against_voucher_no: &str,
    amount: f64,
) -> PaymentLedgerEntry {
    PaymentLedgerEntry {
        company: company.to_string(),
        delinked: false,
        voucher_type: voucher_type.to_string(),
        voucher_no: voucher_no.to_string(),
        against_voucher_type: against_voucher_type.to_string(),
        against_voucher_no: against_voucher_no.to_string(),
        amount,
        amount_in_account_currency: amount,
        account: "Debtors - A".to_string(),
        party_type: "Customer".to_string(),
        party: "CUST-0001".to_string(),
        account_currency: "USD".to_string(),
    }
}

fn advance_ledger(
    company: &str,
    voucher_type: &str,
    voucher_no: &str,
    against_voucher_type: &str,
    against_voucher_no: &str,
    amount: f64,
) -> AdvancePaymentLedgerEntry {
    AdvancePaymentLedgerEntry {
        company: company.to_string(),
        delinked: false,
        voucher_type: voucher_type.to_string(),
        voucher_no: voucher_no.to_string(),
        event: "Submit".to_string(),
        against_voucher_type: against_voucher_type.to_string(),
        against_voucher_no: against_voucher_no.to_string(),
        amount,
        currency: "USD".to_string(),
    }
}
