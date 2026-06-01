use std::collections::HashMap;

use tokio_erp::erpnext::accounts::doctype::payment_order::payment_order::{
    make_journal_entry_plan, PaymentOrder, PaymentStatusUpdate,
};
use tokio_erp::erpnext::accounts::doctype::payment_order_reference::payment_order_reference::PaymentOrderReference;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn payment_order_matches_erpnext_metadata() {
    assert_eq!(PaymentOrder::DOCTYPE, "Payment Order");
    assert_eq!(PaymentOrder::MODULE, "Accounts");
    assert_eq!(
        PaymentOrder::FIELD_ORDER,
        [
            "naming_series",
            "company",
            "payment_order_type",
            "party",
            "column_break_2",
            "posting_date",
            "company_bank",
            "company_bank_account",
            "account",
            "section_break_5",
            "references",
            "amended_from",
        ]
    );
    assert_eq!(PaymentOrder::AUTONAME, "naming_series:");
    assert!(PaymentOrder::IS_SUBMITTABLE);
    assert_eq!(PaymentOrder::SORT_FIELD, "creation");
    assert_eq!(PaymentOrder::SORT_ORDER, "DESC");
    assert!(PaymentOrder::TRACK_CHANGES);

    assert_eq!(
        PaymentOrder::fields(),
        vec![
            FieldSpec::select("naming_series", "Series")
                .default("PMO-")
                .options("PMO-")
                .no_copy()
                .required(),
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::select("payment_order_type", "Payment Order Type")
                .options("\nPayment Request\nPayment Entry")
                .read_only()
                .required(),
            FieldSpec::link("party", "Supplier")
                .options("Supplier")
                .depends_on("eval: doc.payment_order_type=='Payment Request';")
                .in_list_view(),
            FieldSpec::column_break("column_break_2"),
            FieldSpec::date("posting_date", "Posting Date")
                .default("Today")
                .in_list_view(),
            FieldSpec::link("company_bank", "Bank")
                .options("Bank")
                .depends_on("company_bank_account")
                .fetch_from("company_bank_account.bank")
                .in_list_view(),
            FieldSpec::link("company_bank_account", "Company Bank Account")
                .options("Bank Account")
                .required(),
            FieldSpec::data("account", "Account")
                .depends_on("company_bank_account")
                .fetch_from("company_bank_account.account"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::table("references", "Payment Order Reference")
                .options("Payment Order Reference")
                .required(),
            FieldSpec::link("amended_from", "Amended From")
                .options("Payment Order")
                .no_copy()
                .print_hide()
                .read_only(),
        ]
    );
}

#[test]
fn payment_order_status_updates_match_erpnext_submit_and_cancel() {
    let mut request_row =
        PaymentOrderReference::new("Purchase Invoice", "PINV-0001", 100.0, "BANK-001");
    request_row.payment_request = Some("PREQ-0001".to_string());

    let payment_request_order = PaymentOrder {
        payment_order_type: "Payment Request".to_string(),
        references: vec![request_row],
        ..Default::default()
    };
    assert_eq!(
        payment_request_order.custom_hooks(),
        ["on_submit", "on_cancel"]
    );
    assert_eq!(payment_request_order.doctype(), "Payment Order");
    assert_eq!(payment_request_order.module(), "Accounts");
    assert_eq!(
        payment_request_order.update_payment_status(false),
        vec![PaymentStatusUpdate {
            doctype: "Payment Request".to_string(),
            name: "PREQ-0001".to_string(),
            field: "status".to_string(),
            status: "Payment Ordered".to_string(),
        }]
    );
    assert_eq!(
        payment_request_order.update_payment_status(true)[0].status,
        "Initiated"
    );

    let entry_order = PaymentOrder {
        payment_order_type: "Payment Entry".to_string(),
        references: vec![PaymentOrderReference::new(
            "Payment Entry",
            "PE-0001",
            50.0,
            "BANK-001",
        )],
        ..Default::default()
    };
    assert_eq!(
        entry_order.update_payment_status(false),
        vec![PaymentStatusUpdate {
            doctype: "Payment Entry".to_string(),
            name: "PE-0001".to_string(),
            field: "payment_order_status".to_string(),
            status: "Payment Ordered".to_string(),
        }]
    );
}

#[test]
fn payment_order_make_journal_entry_plan_matches_erpnext() {
    let mut cash_row =
        PaymentOrderReference::new("Purchase Invoice", "PINV-0001", 100.0, "BANK-001");
    cash_row.supplier = Some("Supp A".to_string());
    cash_row.mode_of_payment = Some("Cash".to_string());

    let mut bank_row =
        PaymentOrderReference::new("Purchase Invoice", "PINV-0002", 40.0, "BANK-001");
    bank_row.supplier = Some("Supp A".to_string());
    bank_row.mode_of_payment = Some("Wire".to_string());

    let doc = PaymentOrder {
        name: Some("PMO-0001".to_string()),
        company: "Acme".to_string(),
        account: Some("Main Bank - AC".to_string()),
        references: vec![cash_row, bank_row],
        ..Default::default()
    };
    let mode_types = HashMap::from([
        ("Cash".to_string(), "Cash".to_string()),
        ("Wire".to_string(), "Bank".to_string()),
    ]);

    let cash_plan = make_journal_entry_plan(
        &doc,
        "Supp A",
        Some("Cash"),
        &mode_types,
        "Creditors - AC",
        "2026-06-01",
    );
    assert_eq!(cash_plan.payment_order, "PMO-0001");
    assert_eq!(cash_plan.posting_date, "2026-06-01");
    assert_eq!(cash_plan.voucher_type, "Cash Entry");
    assert_eq!(cash_plan.accounts.len(), 2);
    assert_eq!(cash_plan.accounts[0].debit_in_account_currency, 100.0);
    assert_eq!(
        cash_plan.accounts[0].reference_name,
        Some("PINV-0001".to_string())
    );
    assert_eq!(cash_plan.accounts[1].account, "Main Bank - AC");
    assert_eq!(cash_plan.accounts[1].credit_in_account_currency, 100.0);

    let bank_plan = make_journal_entry_plan(
        &doc,
        "Supp A",
        None,
        &mode_types,
        "Creditors - AC",
        "2026-06-01",
    );
    assert_eq!(bank_plan.voucher_type, "Bank Entry");
    assert_eq!(bank_plan.accounts[2].credit_in_account_currency, 140.0);
}
