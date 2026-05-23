use tokio_erp::erpnext::accounts::doctype::bank_clearance::bank_clearance::{
    get_payment_entries_for_bank_clearance_plan, BankClearance, BankClearanceClientConfig,
    BankClearanceError, BankClearanceRawEntry, ClearanceDateUpdateAction, ClearanceDateUpdatePlan,
    ClearanceDateUpdateRow, PaymentEntrySource,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_clearance_matches_erpnext_metadata_fields_and_client_hooks() {
    assert_eq!(BankClearance::DOCTYPE, "Bank Clearance");
    assert_eq!(BankClearance::MODULE, "Accounts");
    assert_eq!(
        BankClearance::FIELD_ORDER,
        [
            "account",
            "account_currency",
            "from_date",
            "to_date",
            "column_break_5",
            "bank_account",
            "include_reconciled_entries",
            "include_pos_transactions",
            "section_break_10",
            "payment_entries",
        ]
    );
    assert!(BankClearance::ALLOW_COPY);
    assert!(BankClearance::HIDE_TOOLBAR);
    assert!(BankClearance::IS_SINGLE);
    assert!(BankClearance::QUICK_ENTRY);
    assert!(BankClearance::READ_ONLY);
    assert_eq!(BankClearance::DOCUMENT_TYPE, "Document");
    assert_eq!(BankClearance::ICON, "fa fa-check");
    assert_eq!(BankClearance::SORT_FIELD, "creation");
    assert_eq!(BankClearance::SORT_ORDER, "ASC");

    assert_eq!(
        BankClearance::fields(),
        vec![
            FieldSpec::link("account", "Account")
                .options("Account")
                .fetch_from("bank_account.account")
                .fetch_if_empty()
                .in_list_view()
                .required(),
            FieldSpec::link("account_currency", "Account Currency")
                .options("Currency")
                .hidden()
                .print_hide(),
            FieldSpec::date("from_date", "From Date")
                .in_list_view()
                .required(),
            FieldSpec::date("to_date", "To Date")
                .in_list_view()
                .required(),
            FieldSpec::column_break("column_break_5"),
            FieldSpec::link("bank_account", "Bank Account")
                .options("Bank Account")
                .description("Select the Bank Account to reconcile."),
            FieldSpec::check("include_reconciled_entries", "Include Reconciled Entries")
                .default("0")
                .in_list_view(),
            FieldSpec::check("include_pos_transactions", "Include POS Transactions").default("0"),
            FieldSpec::section_break("section_break_10"),
            FieldSpec::table("payment_entries", "Payment Entries")
                .options("Bank Clearance Detail")
                .allow_bulk_edit(),
        ]
    );

    let config = BankClearanceClientConfig::from_erpnext_js();
    assert_eq!(
        config.account_fetch,
        ("account", "account_currency", "account_currency")
    );
    assert_eq!(config.account_type_filter, ["Bank", "Cash"]);
    assert!(!config.account_is_group);
    assert!(config.bank_account_is_company_account);
    assert_eq!(config.onload_from_date, "month_start");
    assert_eq!(config.onload_to_date, "month_end");
    assert_eq!(config.primary_button_without_entries, "Get Payment Entries");
    assert_eq!(config.primary_button_with_entries, "Update Clearance Date");
}

#[test]
fn bank_clearance_get_payment_entries_matches_validation_sorting_and_amount_formatting() {
    let mut doc = BankClearance::default();
    assert_eq!(
        doc.get_payment_entries_from_hooks(vec![], 2, "USD"),
        Err(BankClearanceError::FromDateAndToDateMandatory)
    );

    doc.from_date = Some("2026-05-01".to_string());
    doc.to_date = Some("2026-05-31".to_string());
    assert_eq!(
        doc.get_payment_entries_from_hooks(vec![], 2, "USD"),
        Err(BankClearanceError::AccountMandatory)
    );

    doc.account = Some("Bank - TC".to_string());
    doc.get_payment_entries_from_hooks(
        vec![
            vec![BankClearanceRawEntry {
                payment_document: "Payment Entry".to_string(),
                payment_entry: "PE-0002".to_string(),
                posting_date: "2026-05-11".to_string(),
                debit: 0.0,
                credit: 125.5,
                account_currency: None,
                ..BankClearanceRawEntry::default()
            }],
            vec![BankClearanceRawEntry {
                payment_document: "Journal Entry".to_string(),
                payment_entry: "JE-0001".to_string(),
                posting_date: "2026-05-10".to_string(),
                debit: 250.0,
                credit: 10.0,
                account_currency: Some("EUR".to_string()),
                cheque_number: Some("CHK-1".to_string()),
                ..BankClearanceRawEntry::default()
            }],
        ],
        2,
        "USD",
    )
    .expect("valid bank clearance rows");

    assert_eq!(doc.payment_entries.len(), 2);
    assert_eq!(
        doc.payment_entries[0].payment_document.as_deref(),
        Some("Journal Entry")
    );
    assert_eq!(
        doc.payment_entries[0].payment_entry.as_deref(),
        Some("JE-0001")
    );
    assert_eq!(
        doc.payment_entries[0].amount.as_deref(),
        Some("240.00 EUR Dr")
    );
    assert_eq!(
        doc.payment_entries[0].cheque_number.as_deref(),
        Some("CHK-1")
    );
    assert_eq!(
        doc.payment_entries[1].payment_entry.as_deref(),
        Some("PE-0002")
    );
    assert_eq!(
        doc.payment_entries[1].amount.as_deref(),
        Some("125.50 USD Cr")
    );
}

#[test]
fn bank_clearance_update_clearance_date_matches_validation_and_update_targets() {
    let doc = BankClearance {
        account: Some("Bank - TC".to_string()),
        include_reconciled_entries: false,
        ..BankClearance::default()
    };

    assert_eq!(
        doc.update_clearance_date_plan(&[
            ClearanceDateUpdateRow {
                idx: 1,
                payment_document: None,
                payment_entry: "PE-0001".to_string(),
                cheque_date: None,
                clearance_date: Some("2026-05-12".to_string()),
                old_clearance_date: None,
            },
            ClearanceDateUpdateRow {
                idx: 2,
                payment_document: Some("Payment Entry".to_string()),
                payment_entry: "PE-0002".to_string(),
                cheque_date: Some("2026-05-13".to_string()),
                clearance_date: Some("2026-05-12".to_string()),
                old_clearance_date: None,
            },
        ]),
        ClearanceDateUpdatePlan::InvalidRows {
            invalid_document_rows: vec![1],
            invalid_cheque_date_rows: vec![2],
            message: "<p>Please correct the following row(s):</p><ul><li>Payment document required for row(s): 1</li><li>Clearance date must be after cheque date for row(s): 2</li></ul>".to_string(),
        }
    );

    assert_eq!(
        doc.update_clearance_date_plan(&[ClearanceDateUpdateRow {
            idx: 1,
            payment_document: Some("Payment Entry".to_string()),
            payment_entry: "PE-0001".to_string(),
            cheque_date: None,
            clearance_date: None,
            old_clearance_date: None,
        }]),
        ClearanceDateUpdatePlan::NoClearanceDateMentioned {
            message: "Clearance Date not mentioned",
        }
    );

    assert_eq!(
        doc.update_clearance_date_plan(&[
            ClearanceDateUpdateRow {
                idx: 1,
                payment_document: Some("Sales Invoice".to_string()),
                payment_entry: "SINV-0001".to_string(),
                cheque_date: Some("2026-05-10".to_string()),
                clearance_date: Some("2026-05-12".to_string()),
                old_clearance_date: Some("2026-05-11".to_string()),
            },
            ClearanceDateUpdateRow {
                idx: 2,
                payment_document: Some("Payment Entry".to_string()),
                payment_entry: "PE-0001".to_string(),
                cheque_date: Some("2026-05-10".to_string()),
                clearance_date: Some("2026-05-12".to_string()),
                old_clearance_date: None,
            },
        ]),
        ClearanceDateUpdatePlan::Updated {
            actions: vec![
                ClearanceDateUpdateAction::SalesInvoicePayment {
                    parent: "SINV-0001".to_string(),
                    account: "Bank - TC".to_string(),
                    amount_greater_than_zero: true,
                    clearance_date: Some("2026-05-12".to_string()),
                    comment: "Clearance date changed from 2026-05-11 to 2026-05-12 via Bank Clearance Tool".to_string(),
                },
                ClearanceDateUpdateAction::DocumentDbSet {
                    doctype: "Payment Entry".to_string(),
                    name: "PE-0001".to_string(),
                    fieldname: "clearance_date",
                    clearance_date: Some("2026-05-12".to_string()),
                    comment: "Clearance date changed from None to 2026-05-12 via Bank Clearance Tool".to_string(),
                },
            ],
            refresh_payment_entries: true,
            message: "Clearance Date updated",
        }
    );
}

#[test]
fn bank_clearance_query_plan_matches_erpnext_sources_and_reconciled_filters() {
    let plan = get_payment_entries_for_bank_clearance_plan(
        "2026-05-01",
        "2026-05-31",
        "Bank - TC",
        Some("Company Bank Account"),
        false,
        true,
    );

    assert_eq!(
        plan.sources
            .iter()
            .map(|source| source.source)
            .collect::<Vec<_>>(),
        vec![
            PaymentEntrySource::JournalEntry,
            PaymentEntrySource::PaymentEntry,
            PaymentEntrySource::PaidPurchaseInvoice,
            PaymentEntrySource::PosSalesInvoice,
        ]
    );
    assert!(plan.sources.iter().all(|source| source.unreconciled_only));
    assert!(plan
        .sources
        .iter()
        .all(|source| source.account == "Bank - TC"));
    assert!(plan
        .sources
        .iter()
        .all(|source| source.from_date == "2026-05-01"));
    assert!(plan
        .sources
        .iter()
        .all(|source| source.to_date == "2026-05-31"));
    assert!(plan
        .sources
        .iter()
        .all(|source| source.bank_account_filter.is_none()));
    assert_eq!(plan.sort_order, ["posting_date asc", "name desc"]);

    let without_pos = get_payment_entries_for_bank_clearance_plan(
        "2026-05-01",
        "2026-05-31",
        "Bank - TC",
        None,
        true,
        false,
    );
    assert_eq!(without_pos.sources.len(), 3);
    assert!(without_pos
        .sources
        .iter()
        .all(|source| !source.unreconciled_only));
    assert!(!without_pos
        .sources
        .iter()
        .any(|source| source.source == PaymentEntrySource::PosSalesInvoice));
}

#[test]
fn bank_clearance_controller_hooks_match_whitelisted_methods() {
    let doc = BankClearance::default();
    assert_eq!(doc.doctype(), "Bank Clearance");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        ["get_payment_entries", "update_clearance_date"]
    );
}
