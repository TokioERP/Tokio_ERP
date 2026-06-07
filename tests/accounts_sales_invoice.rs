use tokio_erp::erpnext::accounts::doctype::sales_invoice::sales_invoice::{
    CustomerTaxMeta, Indicator, OnloadPlan, PaymentRow, SalesInvoice, SalesInvoiceError,
    SalesInvoiceItemRow, SalesInvoiceStatusUpdaterSpec,
};
use tokio_erp::erpnext::DocumentController;

#[test]
fn sales_invoice_metadata_and_status_updater_match_erpnext() {
    assert_eq!(SalesInvoice::DOCTYPE, "Sales Invoice");
    assert_eq!(SalesInvoice::MODULE, "Accounts");
    assert_eq!(SalesInvoice::AUTONAME, "naming_series:");
    assert_eq!(SalesInvoice::TITLE_FIELD, "customer_name");
    assert_eq!(SalesInvoice::SORT_FIELD, "creation");
    assert_eq!(SalesInvoice::SORT_ORDER, "DESC");
    assert!(SalesInvoice::IS_SUBMITTABLE);
    assert!(SalesInvoice::TRACK_CHANGES);
    assert_eq!(SalesInvoice::FIELD_ORDER.len(), 234);
    assert_eq!(
        &SalesInvoice::FIELD_ORDER[..12],
        [
            "customer_section",
            "naming_series",
            "customer",
            "customer_name",
            "tax_id",
            "project",
            "is_pos",
            "pos_profile",
            "is_return",
            "column_break1",
            "company",
            "cost_center",
        ]
    );
    assert_eq!(
        &SalesInvoice::FIELD_ORDER[229..],
        [
            "column_break_rdke",
            "column_break_rdiw",
            "column_break_iaso",
            "section_break_qllv",
            "title",
        ]
    );

    let doc = SalesInvoice::default();
    assert_eq!(doc.doctype(), "Sales Invoice");
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
        SalesInvoice::status_updater(),
        [SalesInvoiceStatusUpdaterSpec {
            source_dt: "Sales Invoice Item",
            target_field: "billed_amt",
            target_ref_field: "amount",
            target_dt: "Sales Order Item",
            join_field: "so_detail",
            target_parent_dt: "Sales Order",
            target_parent_field: "per_billed",
            source_field: "amount",
            percent_join_field: "sales_order",
            status_field: "billing_status",
            keyword: "Billed",
            overflow_type: "billing",
        }]
    );
}

#[test]
fn sales_invoice_onload_indicator_payments_and_remarks_match_erpnext() {
    let mut doc = SalesInvoice {
        customer: Some("_Test Customer".to_string()),
        po_no: Some("PO-001".to_string()),
        po_date: Some("2026-06-05".to_string()),
        conversion_rate: 2.5,
        payments: vec![
            PaymentRow {
                idx: 1,
                mode_of_payment: Some("Cash".to_string()),
                amount: 10.0,
                ..Default::default()
            },
            PaymentRow {
                idx: 2,
                mode_of_payment: Some("Bank".to_string()),
                amount: 4.0,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    assert_eq!(
        doc.onload_plan(Some(CustomerTaxMeta {
            tax_withholding_category: Some("TDS".to_string()),
            tax_withholding_group: None,
        })),
        OnloadPlan { apply_tds: true }
    );

    doc.set_paid_amount(2);
    assert_eq!(doc.payments[0].base_amount, 25.0);
    assert_eq!(doc.payments[1].base_amount, 10.0);
    assert_eq!(doc.paid_amount, 14.0);
    assert_eq!(doc.base_paid_amount, 35.0);

    doc.add_remarks();
    assert_eq!(
        doc.remarks.as_deref(),
        Some("Against Customer Order PO-001 dated 2026-06-05")
    );

    doc.outstanding_amount = -1.0;
    assert_eq!(
        doc.set_indicator("2026-06-06"),
        Indicator {
            title: "Credit Note Issued".to_string(),
            color: "gray".to_string(),
        }
    );
    doc.outstanding_amount = 10.0;
    doc.due_date = Some("2026-06-06".to_string());
    assert_eq!(
        doc.set_indicator("2026-06-06"),
        Indicator {
            title: "Unpaid".to_string(),
            color: "orange".to_string(),
        }
    );
    doc.due_date = Some("2026-06-01".to_string());
    assert_eq!(doc.set_indicator("2026-06-06").title, "Overdue");
    doc.outstanding_amount = 0.0;
    doc.is_return = 1;
    assert_eq!(doc.set_indicator("2026-06-06").title, "Return");
}

#[test]
fn sales_invoice_core_guards_match_erpnext() {
    let pos_return = SalesInvoice {
        is_pos: true,
        is_return: 1,
        rounded_total: Some(-100.0),
        paid_amount: -90.0,
        write_off_amount: -11.0,
        ..Default::default()
    };
    assert_eq!(
        pos_return.validate_pos_return_totals(2),
        Err(SalesInvoiceError::PaidAndWriteOffGreaterThanGrandTotal)
    );

    let mut write_off = SalesInvoice {
        is_pos: false,
        write_off_account: Some("Write Off - TC".to_string()),
        ..Default::default()
    };
    write_off.allow_write_off_only_on_pos();
    assert_eq!(write_off.write_off_account, None);

    let mut subcontracted = SalesInvoice {
        has_subcontracted: false,
        update_stock: 1,
        items: vec![SalesInvoiceItemRow {
            sales_order: Some("SO-0001".to_string()),
        }],
        ..Default::default()
    };
    assert!(subcontracted.is_subcontracted(true));
    assert_eq!(subcontracted.update_stock, 0);

    let positive_guard = SalesInvoice {
        payments: vec![PaymentRow {
            idx: 3,
            amount: -1.0,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        positive_guard.verify_payment_amount_is_positive(),
        Err(SalesInvoiceError::PaymentAmountMustBePositive { row: 3 })
    );

    let negative_guard = SalesInvoice {
        payments: vec![PaymentRow {
            idx: 4,
            amount: 1.0,
            ..Default::default()
        }],
        ..Default::default()
    };
    assert_eq!(
        negative_guard.verify_payment_amount_is_negative(),
        Err(SalesInvoiceError::PaymentAmountMustBeNegative { row: 4 })
    );
}

#[test]
fn sales_invoice_status_branches_match_erpnext_order() {
    let mut doc = SalesInvoice {
        docstatus: 1,
        grand_total: 100.0,
        rounded_total: Some(100.0),
        base_grand_total: 500.0,
        base_rounded_total: Some(500.0),
        outstanding_amount: 0.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    assert_eq!(doc.set_status(None, "2026-06-06", None), "Paid");

    doc.outstanding_amount = 40.0;
    assert_eq!(doc.set_status(None, "2026-06-06", None), "Partly Paid");
    doc.is_discounted = true;
    assert_eq!(
        doc.set_status(None, "2026-06-06", Some("Disbursed")),
        "Partly Paid and Discounted"
    );
    doc.due_date = Some("2026-06-01".to_string());
    assert_eq!(
        doc.set_status(None, "2026-06-06", Some("Disbursed")),
        "Overdue and Discounted"
    );
    doc.is_discounted = false;
    doc.outstanding_amount = 120.0;
    doc.due_date = Some("2026-06-06".to_string());
    assert_eq!(doc.set_status(None, "2026-06-06", None), "Unpaid");

    doc.outstanding_amount = 0.0;
    doc.is_return = 1;
    assert_eq!(doc.set_status(None, "2026-06-06", None), "Return");
    doc.is_return = 0;
    doc.has_submitted_credit_note = true;
    assert_eq!(
        doc.set_status(None, "2026-06-06", None),
        "Credit Note Issued"
    );

    doc.has_submitted_credit_note = false;
    doc.internal_transfer = true;
    assert_eq!(
        doc.set_status(None, "2026-06-06", None),
        "Internal Transfer"
    );
    doc.docstatus = 2;
    assert_eq!(doc.set_status(None, "2026-06-06", None), "Cancelled");
    assert_eq!(
        doc.set_status(Some("Force Skipped"), "2026-06-06", None),
        "Cancelled"
    );
    doc.docstatus = 0;
    assert_eq!(doc.set_status(None, "2026-06-06", None), "Draft");

    let mut amended_new = SalesInvoice {
        is_new: true,
        amended_from: Some("SINV-OLD".to_string()),
        status: "Submitted".to_string(),
        ..Default::default()
    };
    assert_eq!(amended_new.set_status(None, "2026-06-06", None), "Draft");

    let mut base_total_doc = SalesInvoice {
        docstatus: 1,
        rounded_total: Some(100.0),
        base_rounded_total: Some(500.0),
        outstanding_amount: 250.0,
        currency: "USD".to_string(),
        party_account_currency: Some("EUR".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    assert_eq!(
        base_total_doc.set_status(None, "2026-06-06", None),
        "Partly Paid"
    );

    let mut zero_rounded_total = SalesInvoice {
        docstatus: 1,
        rounded_total: Some(0.0),
        grand_total: 100.0,
        outstanding_amount: 40.0,
        currency: "USD".to_string(),
        party_account_currency: Some("USD".to_string()),
        due_date: Some("2026-06-06".to_string()),
        ..Default::default()
    };
    assert_eq!(
        zero_rounded_total.set_status(None, "2026-06-06", None),
        "Unpaid"
    );
}
