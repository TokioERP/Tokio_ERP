use std::collections::HashMap;

use tokio_erp::erpnext::accounts::doctype::purchase_invoice::purchase_invoice::{
    AccountMeta, OnloadPlan, PurchaseInvoice, PurchaseInvoiceError, PurchaseInvoiceItemRow,
    StatusUpdaterSpec, SupplierBlockStatus, SupplierTaxMeta,
};
use tokio_erp::erpnext::DocumentController;

#[test]
fn purchase_invoice_metadata_and_status_updater_match_erpnext() {
    assert_eq!(PurchaseInvoice::DOCTYPE, "Purchase Invoice");
    assert_eq!(PurchaseInvoice::MODULE, "Accounts");
    assert_eq!(PurchaseInvoice::AUTONAME, "naming_series:");
    assert_eq!(PurchaseInvoice::TITLE_FIELD, "supplier_name");
    assert_eq!(PurchaseInvoice::SORT_FIELD, "creation");
    assert_eq!(PurchaseInvoice::SORT_ORDER, "DESC");
    assert!(PurchaseInvoice::IS_SUBMITTABLE);
    assert!(PurchaseInvoice::TRACK_CHANGES);
    assert_eq!(PurchaseInvoice::FIELD_ORDER.len(), 201);
    assert_eq!(
        &PurchaseInvoice::FIELD_ORDER[..12],
        [
            "naming_series",
            "supplier",
            "supplier_name",
            "tax_id",
            "due_date",
            "is_paid",
            "is_return",
            "apply_tds",
            "column_break1",
            "company",
            "cost_center",
            "posting_date",
        ]
    );
    assert_eq!(
        &PurchaseInvoice::FIELD_ORDER[196..],
        [
            "base_totals_section",
            "totals_section",
            "automation_section",
            "section_break_hzux",
            "title",
        ]
    );

    let doc = PurchaseInvoice::default();
    assert_eq!(doc.doctype(), "Purchase Invoice");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        [
            "validate",
            "before_save",
            "before_submit",
            "on_submit",
            "on_update_after_submit",
            "before_cancel",
            "on_cancel",
        ]
    );
    assert_eq!(
        PurchaseInvoice::status_updater(),
        [StatusUpdaterSpec {
            source_dt: "Purchase Invoice Item",
            target_dt: "Purchase Order Item",
            join_field: "po_detail",
            target_field: "billed_amt",
            target_parent_dt: "Purchase Order",
            target_parent_field: "per_billed",
            target_ref_field: "amount",
            source_field: "amount",
            percent_join_field: "purchase_order",
            overflow_type: "billing",
        }]
    );
}

#[test]
fn purchase_invoice_onload_before_save_remarks_and_percentage_match_erpnext() {
    let mut doc = PurchaseInvoice {
        supplier: Some("_Test Supplier".to_string()),
        on_hold: false,
        release_date: Some("2026-06-30".to_string()),
        bill_no: Some("SUP-INV-1".to_string()),
        bill_date: Some("2026-06-05".to_string()),
        items: vec![
            PurchaseInvoiceItemRow {
                qty: 5.0,
                received_qty: 4.0,
                purchase_receipt: Some("PREC-0001".to_string()),
                pr_detail: Some("PRI-0001".to_string()),
            },
            PurchaseInvoiceItemRow {
                qty: 10.0,
                received_qty: 10.0,
                purchase_receipt: None,
                pr_detail: Some("PRI-0002".to_string()),
            },
            PurchaseInvoiceItemRow {
                qty: 3.0,
                received_qty: 6.0,
                purchase_receipt: Some("PREC-0002".to_string()),
                pr_detail: Some("PRI-0003".to_string()),
            },
        ],
        ..Default::default()
    };

    assert_eq!(
        doc.onload_plan(
            true,
            Some(SupplierTaxMeta {
                tax_withholding_category: None,
                tax_withholding_group: Some("TDS Group".to_string()),
            })
        ),
        OnloadPlan {
            apply_tds: true,
            clear_tax_withholding_entries: true,
        }
    );
    doc.before_save();
    assert_eq!(doc.release_date, None);

    doc.create_remarks();
    assert_eq!(
        doc.remarks.as_deref(),
        Some("Against Supplier Invoice SUP-INV-1 dated 2026-06-05")
    );

    doc.set_percentage_received();
    assert_eq!(doc.per_received, 125.0);

    doc.on_hold = true;
    doc.release_date = None;
    assert!(doc.invoice_is_blocked("2026-06-06"));
    doc.release_date = Some("2026-06-30".to_string());
    assert!(doc.invoice_is_blocked("2026-06-06"));
    doc.release_date = Some("2026-06-01".to_string());
    assert!(!doc.invoice_is_blocked("2026-06-06"));
}

#[test]
fn purchase_invoice_core_validation_guards_match_erpnext() {
    let mut cash = PurchaseInvoice {
        is_paid: 1,
        paid_amount: 25.0,
        ..Default::default()
    };
    assert_eq!(
        cash.validate_core("2026-06-06", 2, &HashMap::new()),
        Err(PurchaseInvoiceError::CashBankAccountRequired)
    );
    assert_eq!(cash.is_opening, "No");

    let mut overpaid = PurchaseInvoice {
        is_paid: 1,
        paid_amount: 100.006,
        cash_bank_account: Some("Cash - TC".to_string()),
        grand_total: 100.0,
        ..Default::default()
    };
    assert_eq!(
        overpaid.validate_core("2026-06-06", 2, &HashMap::new()),
        Err(PurchaseInvoiceError::PaidAndWriteOffGreaterThanGrandTotal)
    );

    let mut release_date_today = PurchaseInvoice {
        release_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    assert_eq!(
        release_date_today.validate_core("2026-06-06", 2, &HashMap::new()),
        Err(PurchaseInvoiceError::ReleaseDateMustBeFuture)
    );

    let mut profit_loss_account = PurchaseInvoice {
        supplier: Some("_Test Supplier".to_string()),
        credit_to: Some("Creditors - TC".to_string()),
        ..Default::default()
    };
    let accounts = HashMap::from([(
        "Creditors - TC".to_string(),
        AccountMeta {
            account_type: Some("Payable".to_string()),
            report_type: Some("Profit and Loss".to_string()),
            account_currency: Some("USD".to_string()),
        },
    )]);
    assert_eq!(
        profit_loss_account.validate_core("2026-06-06", 2, &accounts),
        Err(PurchaseInvoiceError::CreditToMustBeBalanceSheet)
    );

    let mut non_payable_account = PurchaseInvoice {
        supplier: Some("_Test Supplier".to_string()),
        credit_to: Some("Cash - TC".to_string()),
        ..Default::default()
    };
    let accounts = HashMap::from([(
        "Cash - TC".to_string(),
        AccountMeta {
            account_type: Some("Cash".to_string()),
            report_type: Some("Balance Sheet".to_string()),
            account_currency: Some("USD".to_string()),
        },
    )]);
    assert_eq!(
        non_payable_account.validate_core("2026-06-06", 2, &accounts),
        Err(PurchaseInvoiceError::CreditToMustBePayable)
    );

    let mut blocked_supplier = PurchaseInvoice {
        supplier: Some("_Test Supplier".to_string()),
        supplier_block_status: Some(SupplierBlockStatus {
            on_hold: true,
            hold_type: Some("Invoices".to_string()),
            release_date: None,
        }),
        ..Default::default()
    };
    assert_eq!(
        blocked_supplier.validate_core("2026-06-06", 2, &HashMap::new()),
        Err(PurchaseInvoiceError::SupplierBlocked {
            supplier: "_Test Supplier".to_string()
        })
    );

    blocked_supplier.supplier_block_status = Some(SupplierBlockStatus {
        on_hold: true,
        hold_type: Some("Payments".to_string()),
        release_date: Some("2026-06-06".to_string()),
    });
    blocked_supplier
        .validate_core("2026-06-06", 2, &HashMap::new())
        .unwrap();
}

#[test]
fn purchase_invoice_status_branches_match_erpnext_order() {
    let mut doc = PurchaseInvoice {
        grand_total: 100.0,
        rounded_total: Some(100.0),
        base_grand_total: 500.0,
        base_rounded_total: Some(500.0),
        outstanding_amount: 0.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        docstatus: 1,
        ..Default::default()
    };
    assert_eq!(doc.set_status(None, "2026-06-06"), "Paid");

    doc.outstanding_amount = 40.0;
    assert_eq!(doc.set_status(None, "2026-06-06"), "Partly Paid");
    doc.due_date = Some("2026-06-01".to_string());
    assert_eq!(doc.set_status(None, "2026-06-06"), "Overdue");
    doc.outstanding_amount = 120.0;
    doc.due_date = Some("2026-06-06".to_string());
    assert_eq!(doc.set_status(None, "2026-06-06"), "Unpaid");

    doc.outstanding_amount = 0.0;
    doc.is_return = 1;
    assert_eq!(doc.set_status(None, "2026-06-06"), "Return");
    doc.is_return = 0;
    doc.has_submitted_debit_note = true;
    assert_eq!(doc.set_status(None, "2026-06-06"), "Debit Note Issued");

    doc.has_submitted_debit_note = false;
    doc.internal_transfer = true;
    assert_eq!(doc.set_status(None, "2026-06-06"), "Internal Transfer");
    doc.docstatus = 2;
    assert_eq!(doc.set_status(None, "2026-06-06"), "Cancelled");
    assert_eq!(
        doc.set_status(Some("Force Skipped"), "2026-06-06"),
        "Cancelled"
    );
    doc.docstatus = 0;
    assert_eq!(doc.set_status(None, "2026-06-06"), "Draft");

    let mut amended_new = PurchaseInvoice {
        is_new: true,
        amended_from: Some("PINV-OLD".to_string()),
        status: "Submitted".to_string(),
        ..Default::default()
    };
    assert_eq!(amended_new.set_status(None, "2026-06-06"), "Draft");

    let mut base_total_doc = PurchaseInvoice {
        docstatus: 1,
        rounded_total: Some(100.0),
        base_rounded_total: Some(500.0),
        outstanding_amount: 250.0,
        currency: "USD".to_string(),
        party_account_currency: Some("EUR".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    assert_eq!(base_total_doc.set_status(None, "2026-06-06"), "Partly Paid");

    let mut zero_rounded_total = PurchaseInvoice {
        docstatus: 1,
        rounded_total: Some(0.0),
        grand_total: 100.0,
        outstanding_amount: 40.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    assert_eq!(zero_rounded_total.set_status(None, "2026-06-06"), "Unpaid");
}
