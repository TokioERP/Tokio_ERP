use tokio_erp::erpnext::accounts::doctype::process_payment_reconciliation_log_allocations::process_payment_reconciliation_log_allocations::ProcessPaymentReconciliationLogAllocations;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn process_payment_reconciliation_log_allocations_matches_erpnext_metadata() {
    assert_eq!(
        ProcessPaymentReconciliationLogAllocations::DOCTYPE,
        "Process Payment Reconciliation Log Allocations"
    );
    assert_eq!(
        ProcessPaymentReconciliationLogAllocations::MODULE,
        "Accounts"
    );
    assert_eq!(
        ProcessPaymentReconciliationLogAllocations::FIELD_ORDER,
        [
            "reference_type",
            "reference_name",
            "reference_row",
            "column_break_3",
            "invoice_type",
            "invoice_number",
            "section_break_6",
            "allocated_amount",
            "unreconciled_amount",
            "column_break_8",
            "amount",
            "is_advance",
            "section_break_5",
            "difference_amount",
            "gain_loss_posting_date",
            "column_break_7",
            "difference_account",
            "exchange_rate",
            "currency",
            "reconciled",
        ]
    );
    assert!(ProcessPaymentReconciliationLogAllocations::IS_TABLE);
    assert!(ProcessPaymentReconciliationLogAllocations::EDITABLE_GRID);
    assert!(ProcessPaymentReconciliationLogAllocations::TRACK_CHANGES);

    assert_eq!(
        ProcessPaymentReconciliationLogAllocations::fields(),
        vec![
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("reference_row", "Reference Row")
                .hidden()
                .read_only(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::link("invoice_type", "Invoice Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::dynamic_link("invoice_number")
                .label("Invoice Number")
                .options("invoice_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("currency")
                .required()
                .in_list_view(),
            FieldSpec::currency("unreconciled_amount", "Unreconciled Amount")
                .options("currency")
                .hidden()
                .read_only(),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .hidden()
                .read_only(),
            FieldSpec::data("is_advance", "Is Advance")
                .hidden()
                .read_only(),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::currency("difference_amount", "Difference Amount")
                .options("Currency")
                .read_only()
                .in_list_view(),
            FieldSpec::date("gain_loss_posting_date", "Difference Posting Date"),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::link("difference_account", "Difference Account")
                .options("Account")
                .read_only(),
            FieldSpec::float("exchange_rate", "Exchange Rate").read_only(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::check("reconciled", "Reconciled")
                .default("0")
                .in_list_view(),
        ]
    );
}

#[test]
fn process_payment_reconciliation_log_allocations_preserves_pass_controller_behavior() {
    let blank = ProcessPaymentReconciliationLogAllocations::default();
    assert_eq!(blank.reference_type, None);
    assert_eq!(blank.reference_name, None);
    assert_eq!(blank.invoice_type, None);
    assert_eq!(blank.invoice_number, None);
    assert_eq!(blank.allocated_amount, None);
    assert!(!blank.reconciled);
    assert!(blank.custom_hooks().is_empty());

    let row = ProcessPaymentReconciliationLogAllocations::new(
        "Payment Entry",
        "PE-0001",
        "Sales Invoice",
        "SINV-0001",
        "100.00",
    );
    assert_eq!(row.reference_type.as_deref(), Some("Payment Entry"));
    assert_eq!(row.reference_name.as_deref(), Some("PE-0001"));
    assert_eq!(row.invoice_type.as_deref(), Some("Sales Invoice"));
    assert_eq!(row.invoice_number.as_deref(), Some("SINV-0001"));
    assert_eq!(row.allocated_amount.as_deref(), Some("100.00"));
    assert_eq!(
        row.doctype(),
        "Process Payment Reconciliation Log Allocations"
    );
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
