use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::pos_invoice_merge_log::pos_invoice_merge_log::{
    cancel_merge_logs_plan, check_scheduler_status, consolidate_pos_invoices_plan,
    create_merge_logs_plan, distinguish_return_pos_invoices_plan, enqueue_job_plan,
    get_error_message, get_invoice_customer_map, split_invoices,
    split_invoices_by_accounting_dimension, unconsolidate_pos_invoices_plan, EnqueueJobKind,
    ErrorMessage, MergeInvoicesBasedOn, NewSalesInvoicePlan, PosInvoiceMergeLog,
    PosInvoiceMergeLogAction, PosInvoiceMergeLogCancelCandidate, PosInvoiceMergeLogClosingEntry,
    PosInvoiceMergeLogDocument, PosInvoiceMergeLogError, PosInvoiceMergeLogInvoice,
    PosInvoiceMergeLogItem, PosInvoiceMergeLogItemWiseTaxDetail, PosInvoiceMergeLogPayment,
    PosInvoiceMergeLogProfileDefaults, PosInvoiceMergeLogReturnInvoice,
    PosInvoiceMergeLogSourceInvoice, PosInvoiceMergeLogSplitInvoice, PosInvoiceMergeLogTax,
    PosInvoiceMergeLogUpdateInvoice, SchedulerStatus,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn pos_invoice_merge_log_matches_erpnext_metadata() {
    assert_eq!(PosInvoiceMergeLog::DOCTYPE, "POS Invoice Merge Log");
    assert_eq!(PosInvoiceMergeLog::MODULE, "Accounts");
    assert_eq!(
        PosInvoiceMergeLog::FIELD_ORDER,
        [
            "company",
            "posting_date",
            "posting_time",
            "merge_invoices_based_on",
            "column_break_3",
            "pos_closing_entry",
            "customer",
            "customer_group",
            "section_break_3",
            "pos_invoices",
            "references_section",
            "consolidated_invoice",
            "column_break_7",
            "consolidated_credit_note",
            "amended_from",
        ]
    );
    assert!(PosInvoiceMergeLog::EDITABLE_GRID);
    assert!(PosInvoiceMergeLog::INDEX_WEB_PAGES_FOR_SEARCH);
    assert!(PosInvoiceMergeLog::IS_SUBMITTABLE);
    assert_eq!(PosInvoiceMergeLog::ROW_FORMAT, "Dynamic");
    assert_eq!(PosInvoiceMergeLog::SORT_FIELD, "creation");
    assert_eq!(PosInvoiceMergeLog::SORT_ORDER, "DESC");
    assert!(PosInvoiceMergeLog::TRACK_CHANGES);

    let fields = PosInvoiceMergeLog::fields();
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .in_standard_filter()
            .print_hide()
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::table("pos_invoices", "POS Invoices")
            .options("POS Invoice Reference")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::select("merge_invoices_based_on", "Merge Invoices Based On")
            .options("Customer\nCustomer Group")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("customer_group", "Customer Group")
            .options("Customer Group")
            .depends_on("eval:doc.merge_invoices_based_on == 'Customer Group'")
            .mandatory_depends_on("eval:doc.merge_invoices_based_on == 'Customer Group'")
    ));
}

#[test]
fn pos_invoice_merge_log_validates_duplicates_customer_and_status() {
    let mut log =
        PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");
    log.pos_invoices = vec![
        PosInvoiceMergeLogInvoice::submitted(1, "POS-001", "_Test Customer"),
        PosInvoiceMergeLogInvoice::submitted(2, "POS-001", "_Test Customer"),
    ];
    assert_eq!(
        log.validate_duplicate_pos_invoices(),
        Err(PosInvoiceMergeLogError::DuplicatePosInvoices {
            invoice: "POS-001".to_string(),
            rows: vec![1, 2],
        })
    );

    log.pos_invoices = vec![PosInvoiceMergeLogInvoice::submitted(
        3,
        "POS-002",
        "Other Customer",
    )];
    assert_eq!(
        log.validate_customer(),
        Err(PosInvoiceMergeLogError::CustomerMismatch {
            row: 3,
            pos_invoice: "POS-002".to_string(),
            customer: "_Test Customer".to_string(),
        })
    );

    log.merge_invoices_based_on = MergeInvoicesBasedOn::CustomerGroup;
    assert!(log.validate_customer().is_ok());

    log.pos_invoices = vec![PosInvoiceMergeLogInvoice {
        idx: 4,
        pos_invoice: "POS-003".to_string(),
        customer: "_Test Customer".to_string(),
        status: "Draft".to_string(),
        docstatus: 0,
        is_return: false,
        return_against: None,
        return_against_status: None,
    }];
    assert_eq!(
        log.validate_pos_invoice_status(),
        Err(PosInvoiceMergeLogError::PosInvoiceNotSubmitted {
            row: 4,
            pos_invoice: "POS-003".to_string(),
        })
    );
}

#[test]
fn pos_invoice_merge_log_splits_serial_returns_before_following_sales() {
    let invoices = vec![
        PosInvoiceMergeLogSplitInvoice::sale("POS-SALE-1"),
        PosInvoiceMergeLogSplitInvoice::serial_return("POS-RETURN-1", "POS-SALE-1", false),
        PosInvoiceMergeLogSplitInvoice::sale("POS-SALE-2"),
    ];

    assert_eq!(
        split_invoices(&invoices),
        vec![
            vec!["POS-SALE-1".to_string()],
            vec!["POS-RETURN-1".to_string(), "POS-SALE-2".to_string()],
        ]
    );
}

#[test]
fn pos_invoice_merge_log_groups_customer_map_by_accounting_dimensions() {
    let invoices = vec![
        PosInvoiceMergeLogSourceInvoice::new("POS-001", "_Test Customer")
            .dimension("cost_center", "Main - TC")
            .dimension("project", "PROJ-1"),
        PosInvoiceMergeLogSourceInvoice::new("POS-002", "_Test Customer")
            .dimension("cost_center", "Main - TC")
            .dimension("project", "PROJ-1"),
        PosInvoiceMergeLogSourceInvoice::new("POS-003", "_Test Customer")
            .dimension("cost_center", "Other - TC")
            .dimension("project", "PROJ-2"),
        PosInvoiceMergeLogSourceInvoice::new("POS-004", "Second Customer")
            .dimension("cost_center", "Main - TC"),
    ];

    let customer_map = get_invoice_customer_map(&invoices);
    assert_eq!(customer_map.len(), 2);
    assert_eq!(customer_map["_Test Customer"].len(), 2);
    assert_eq!(customer_map["Second Customer"].len(), 1);

    let grouped = split_invoices_by_accounting_dimension(&invoices[..3]);
    assert_eq!(
        grouped
            .values()
            .map(|invoices| invoices.len())
            .collect::<Vec<_>>(),
        vec![2, 1]
    );
}

#[test]
fn pos_invoice_merge_log_plans_queue_or_inline_consolidation_like_erpnext_thresholds() {
    let grouped: BTreeMap<String, BTreeMap<Vec<(String, String)>, Vec<String>>> =
        BTreeMap::from([(
            "_Test Customer".to_string(),
            BTreeMap::from([(
                vec![("cost_center".to_string(), "Main - TC".to_string())],
                vec!["POS-001".to_string()],
            )]),
        )]);

    assert_eq!(
        consolidate_pos_invoices_plan(10, Some("POS-CLOSE-0001"), grouped.clone()),
        vec![
            PosInvoiceMergeLogAction::SetClosingEntryStatus {
                status: "Queued".to_string(),
            },
            PosInvoiceMergeLogAction::EnqueueJob {
                kind: EnqueueJobKind::CreateMergeLogs,
                job_id: "pos_invoice_merge::POS-CLOSE-0001".to_string(),
                message: "POS Invoices will be consolidated in a background process".to_string(),
            },
        ]
    );
    assert_eq!(
        consolidate_pos_invoices_plan(9, Some("POS-CLOSE-0001"), grouped),
        vec![PosInvoiceMergeLogAction::CreateMergeLogs]
    );
    assert_eq!(
        unconsolidate_pos_invoices_plan(10, Some("POS-CLOSE-0001")),
        vec![
            PosInvoiceMergeLogAction::SetClosingEntryStatus {
                status: "Queued".to_string(),
            },
            PosInvoiceMergeLogAction::EnqueueJob {
                kind: EnqueueJobKind::CancelMergeLogs,
                job_id: "pos_invoice_merge::POS-CLOSE-0001".to_string(),
                message: "POS Invoices will be unconsolidated in a background process".to_string(),
            },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_enqueue_and_error_helpers_match_erpnext_branches() {
    assert_eq!(
        check_scheduler_status(false, true),
        Err(PosInvoiceMergeLogError::SchedulerInactive)
    );
    assert_eq!(check_scheduler_status(true, true), Ok(()));
    assert_eq!(
        enqueue_job_plan(
            EnqueueJobKind::CreateMergeLogs,
            Some("POS-CLOSE-0001"),
            false,
            SchedulerStatus {
                developer_mode: false,
                in_test: true,
                scheduler_inactive: false,
            },
        ),
        Ok(Some(PosInvoiceMergeLogAction::EnqueueJob {
            kind: EnqueueJobKind::CreateMergeLogs,
            job_id: "pos_invoice_merge::POS-CLOSE-0001".to_string(),
            message: "POS Invoices will be consolidated in a background process".to_string(),
        }))
    );
    assert_eq!(
        enqueue_job_plan(
            EnqueueJobKind::CancelMergeLogs,
            Some("POS-CLOSE-0001"),
            true,
            SchedulerStatus {
                developer_mode: false,
                in_test: false,
                scheduler_inactive: false,
            },
        ),
        Ok(None)
    );
    assert_eq!(
        get_error_message(ErrorMessage::Dict(BTreeMap::from([(
            "message".to_string(),
            "Detailed failure".to_string(),
        )]))),
        "Detailed failure"
    );
    assert_eq!(
        get_error_message(ErrorMessage::Text("fallback failure".to_string())),
        "fallback failure"
    );
}

#[test]
fn pos_invoice_merge_log_merge_plan_aggregates_child_tables_like_erpnext() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    let docs = vec![
        PosInvoiceMergeLogDocument {
            name: "POS-001".to_string(),
            posting_date: Some("2026-06-03".to_string()),
            posting_time: Some("09:30:00".to_string()),
            redeem_loyalty_points: true,
            loyalty_redemption_account: Some("Loyalty - TC".to_string()),
            loyalty_redemption_cost_center: Some("Main - TC".to_string()),
            loyalty_points: 10,
            loyalty_amount: 25.0,
            rounding_adjustment: 0.25,
            rounded_total: 110.0,
            base_rounding_adjustment: 0.25,
            base_rounded_total: 110.0,
            items: vec![PosInvoiceMergeLogItem {
                name: "ITEM-ROW-1".to_string(),
                net_rate: 100.0,
                net_amount: 100.0,
                base_net_amount: 100.0,
                pos_invoice_item: Some("OLD-ITEM-1".to_string()),
                serial_and_batch_bundle: Some("SBB-1".to_string()),
            }],
            taxes: vec![PosInvoiceMergeLogTax {
                name: "TAX-1".to_string(),
                account_head: "VAT - TC".to_string(),
                cost_center: "Main - TC".to_string(),
                tax_amount_after_discount_amount: 9.0,
                base_tax_amount_after_discount_amount: 9.0,
            }],
            payments: vec![PosInvoiceMergeLogPayment {
                account: "Cash - TC".to_string(),
                mode_of_payment: "Cash".to_string(),
                amount: 109.0,
                base_amount: 109.0,
            }],
            item_wise_tax_details: vec![PosInvoiceMergeLogItemWiseTaxDetail {
                item_row: "ITEM-ROW-1".to_string(),
                tax_row: "TAX-1".to_string(),
                amount: 9.0,
                rate: 9.0,
                taxable_amount: 100.0,
            }],
            ..PosInvoiceMergeLogDocument::new("POS-001")
        },
        PosInvoiceMergeLogDocument {
            name: "POS-002".to_string(),
            posting_date: Some("2026-06-04".to_string()),
            posting_time: Some("10:45:00".to_string()),
            rounding_adjustment: 0.5,
            rounded_total: 55.0,
            base_rounding_adjustment: 0.5,
            base_rounded_total: 55.0,
            items: vec![PosInvoiceMergeLogItem {
                name: "ITEM-ROW-2".to_string(),
                net_rate: 50.0,
                net_amount: 50.0,
                base_net_amount: 50.0,
                pos_invoice_item: None,
                serial_and_batch_bundle: None,
            }],
            taxes: vec![PosInvoiceMergeLogTax {
                name: "TAX-2".to_string(),
                account_head: "VAT - TC".to_string(),
                cost_center: "Main - TC".to_string(),
                tax_amount_after_discount_amount: 4.5,
                base_tax_amount_after_discount_amount: 4.5,
            }],
            payments: vec![PosInvoiceMergeLogPayment {
                account: "Cash - TC".to_string(),
                mode_of_payment: "Cash".to_string(),
                amount: 54.5,
                base_amount: 54.5,
            }],
            item_wise_tax_details: vec![PosInvoiceMergeLogItemWiseTaxDetail {
                item_row: "ITEM-ROW-2".to_string(),
                tax_row: "TAX-2".to_string(),
                amount: 4.5,
                rate: 9.0,
                taxable_amount: 50.0,
            }],
            ..PosInvoiceMergeLogDocument::new("POS-002")
        },
    ];

    let plan = log.merge_pos_invoice_into_plan(
        &docs,
        &PosInvoiceMergeLogProfileDefaults::default(),
        true,
        &[],
    );

    assert_eq!(plan.posting_date.as_deref(), Some("2026-06-04"));
    assert_eq!(plan.posting_time.as_deref(), Some("10:45:00"));
    assert_eq!(plan.items.len(), 2);
    assert_eq!(plan.items[0].row_id, "POS-001:ITEM-ROW-1");
    assert_eq!(plan.items[0].rate, 100.0);
    assert_eq!(plan.items[0].amount, 100.0);
    assert_eq!(plan.items[0].base_amount, 100.0);
    assert_eq!(plan.items[0].price_list_rate, 0.0);
    assert_eq!(plan.items[0].pos_invoice, "POS-001");
    assert_eq!(plan.items[0].pos_invoice_item, "ITEM-ROW-1");
    assert_eq!(
        plan.items[0].serial_and_batch_bundle.as_deref(),
        Some("SBB-1")
    );

    assert_eq!(plan.taxes.len(), 1);
    assert_eq!(plan.taxes[0].row_id, "VAT - TC|Main - TC");
    assert_eq!(plan.taxes[0].charge_type, "Actual");
    assert_eq!(plan.taxes[0].idx, 1);
    assert!(!plan.taxes[0].included_in_print_rate);
    assert!(plan.taxes[0].dont_recompute_tax);
    assert_eq!(plan.taxes[0].tax_amount, 13.5);
    assert_eq!(plan.taxes[0].base_tax_amount, 13.5);

    assert_eq!(plan.payments.len(), 1);
    assert_eq!(plan.payments[0].amount, 163.5);
    assert_eq!(plan.payments[0].base_amount, 163.5);
    assert_eq!(plan.rounding_adjustment, 0.75);
    assert_eq!(plan.rounded_total, 165.0);
    assert_eq!(plan.base_rounding_adjustment, 0.75);
    assert_eq!(plan.base_rounded_total, 165.0);
    assert!(plan.redeem_loyalty_points);
    assert_eq!(plan.loyalty_points, 10);
    assert_eq!(plan.loyalty_amount, 25.0);
    assert_eq!(plan.customer.as_deref(), Some("_Test Customer"));
    assert_eq!(plan.additional_discount_percentage, 0.0);
    assert_eq!(plan.discount_amount, 0.0);
    assert!(plan.taxes_and_charges.is_none());
    assert!(plan.ignore_pricing_rule);
    assert!(plan.disable_rounded_total);
    assert_eq!(plan.item_wise_tax_details.len(), 2);
    assert_eq!(plan.item_wise_tax_details[0].item_row, "POS-001:ITEM-ROW-1");
    assert_eq!(plan.item_wise_tax_details[0].tax_row, "VAT - TC|Main - TC");
}

#[test]
fn pos_invoice_merge_log_merge_plan_uses_profile_dimensions_and_customer_group_flag() {
    let mut log =
        PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");
    log.merge_invoices_based_on = MergeInvoicesBasedOn::CustomerGroup;

    let mut first_doc = PosInvoiceMergeLogDocument::new("POS-001");
    first_doc
        .accounting_dimensions
        .insert("department".to_string(), "Retail - TC".to_string());
    first_doc.cost_center = Some("Store - TC".to_string());

    let mut defaults = PosInvoiceMergeLogProfileDefaults {
        pos_profile: "POS Profile - TC".to_string(),
        cost_center: Some("Main - TC".to_string()),
        project: Some("PROJ-1".to_string()),
        ..PosInvoiceMergeLogProfileDefaults::default()
    };
    defaults
        .accounting_dimensions
        .insert("department".to_string(), "Default Dept - TC".to_string());
    defaults
        .accounting_dimensions
        .insert("business_unit".to_string(), "Default Unit - TC".to_string());

    let plan = log.merge_pos_invoice_into_plan(
        &[first_doc],
        &defaults,
        false,
        &["department".to_string(), "business_unit".to_string()],
    );

    assert!(plan.ignore_pos_profile);
    assert_eq!(plan.pos_profile, "");
    assert_eq!(
        plan.accounting_dimensions,
        BTreeMap::from([
            ("business_unit".to_string(), "Default Unit - TC".to_string()),
            ("department".to_string(), "Retail - TC".to_string()),
        ])
    );
    assert_eq!(plan.cost_center.as_deref(), Some("Store - TC"));
    assert_eq!(plan.project.as_deref(), Some("PROJ-1"));
    assert!(plan.sales_partner.is_none());
    assert_eq!(plan.commission_rate, 0.0);
    assert_eq!(plan.total_commission, 0.0);
}

#[test]
fn pos_invoice_merge_log_update_pos_invoices_plan_matches_submit_and_cancel_branches() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");
    let invoices = vec![
        PosInvoiceMergeLogUpdateInvoice::sale("POS-SALE-1"),
        PosInvoiceMergeLogUpdateInvoice::return_invoice("POS-RET-1"),
        PosInvoiceMergeLogUpdateInvoice::return_invoice("POS-RET-2"),
    ];
    let credit_notes = BTreeMap::from([("SI-CR-1".to_string(), vec!["POS-RET-1".to_string()])]);

    assert_eq!(
        log.update_pos_invoices_plan(&invoices, Some("SI-0001"), &credit_notes, 1),
        vec![
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-SALE-1".to_string(),
                consolidated_invoice: Some("SI-0001".to_string()),
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-SALE-1".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-SALE-1".to_string(),
            },
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-RET-1".to_string(),
                consolidated_invoice: Some("SI-CR-1".to_string()),
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-RET-1".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-RET-1".to_string(),
            },
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-RET-2".to_string(),
                consolidated_invoice: Some("SI-0001".to_string()),
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-RET-2".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-RET-2".to_string(),
            },
        ]
    );

    assert_eq!(
        log.update_pos_invoices_plan(&invoices, Some("SI-0001"), &credit_notes, 2),
        vec![
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-SALE-1".to_string(),
                consolidated_invoice: None,
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-SALE-1".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-SALE-1".to_string(),
            },
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-RET-1".to_string(),
                consolidated_invoice: None,
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-RET-1".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-RET-1".to_string(),
            },
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-RET-2".to_string(),
                consolidated_invoice: None,
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-RET-2".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-RET-2".to_string(),
            },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_on_cancel_plan_matches_erpnext_side_effect_order() {
    let mut log =
        PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");
    log.pos_invoices = vec![
        PosInvoiceMergeLogInvoice::submitted(1, "POS-SALE-1", "_Test Customer"),
        PosInvoiceMergeLogInvoice::submitted(2, "POS-RET-1", "_Test Customer"),
    ];
    log.consolidated_invoice = Some("SI-0001".to_string());
    log.consolidated_credit_note = Some("SI-CR-1".to_string());

    let actions = log.on_cancel_side_effect_plan(&["SBB-1".to_string(), "SBB-2".to_string()]);

    assert_eq!(
        actions,
        vec![
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-SALE-1".to_string(),
                consolidated_invoice: None,
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-SALE-1".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-SALE-1".to_string(),
            },
            PosInvoiceMergeLogAction::UpdatePosInvoiceConsolidatedInvoice {
                pos_invoice: "POS-RET-1".to_string(),
                consolidated_invoice: None,
            },
            PosInvoiceMergeLogAction::RefreshPosInvoiceStatus {
                pos_invoice: "POS-RET-1".to_string(),
            },
            PosInvoiceMergeLogAction::SavePosInvoice {
                pos_invoice: "POS-RET-1".to_string(),
            },
            PosInvoiceMergeLogAction::SetSerialAndBatchBundle {
                pos_invoice: "POS-SALE-1".to_string(),
                table_name: "items".to_string(),
            },
            PosInvoiceMergeLogAction::SetSerialAndBatchBundle {
                pos_invoice: "POS-SALE-1".to_string(),
                table_name: "packed_items".to_string(),
            },
            PosInvoiceMergeLogAction::SetSerialAndBatchBundle {
                pos_invoice: "POS-RET-1".to_string(),
                table_name: "items".to_string(),
            },
            PosInvoiceMergeLogAction::SetSerialAndBatchBundle {
                pos_invoice: "POS-RET-1".to_string(),
                table_name: "packed_items".to_string(),
            },
            PosInvoiceMergeLogAction::CancelLinkedInvoice {
                sales_invoice: "SI-CR-1".to_string(),
                ignore_validate: true,
            },
            PosInvoiceMergeLogAction::CancelLinkedInvoice {
                sales_invoice: "SI-0001".to_string(),
                ignore_validate: true,
            },
            PosInvoiceMergeLogAction::DelinkCancelledStockLedgerBundles {
                bundles: vec!["SBB-1".to_string(), "SBB-2".to_string()],
            },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_create_merge_logs_plan_matches_erpnext_success_path() {
    let closing_entry = PosInvoiceMergeLogClosingEntry {
        name: "POS-CLOSE-0001".to_string(),
        posting_date: "2026-06-04".to_string(),
        posting_time: "21:15:00".to_string(),
        company: "_Test Company".to_string(),
    };
    let invoice_by_customer = BTreeMap::from([(
        "_Test Customer".to_string(),
        vec![vec![
            PosInvoiceMergeLogSplitInvoice::sale("POS-SALE-1"),
            PosInvoiceMergeLogSplitInvoice::serial_return("POS-RET-1", "POS-SALE-1", false),
            PosInvoiceMergeLogSplitInvoice::sale("POS-SALE-2"),
        ]],
    )]);

    assert_eq!(
        create_merge_logs_plan(&invoice_by_customer, Some(&closing_entry)),
        vec![
            PosInvoiceMergeLogAction::CreateMergeLogDocument {
                posting_date: "2026-06-04".to_string(),
                posting_time: "21:15:00".to_string(),
                company: Some("_Test Company".to_string()),
                customer: "_Test Customer".to_string(),
                pos_closing_entry: Some("POS-CLOSE-0001".to_string()),
                pos_invoices: vec!["POS-SALE-1".to_string()],
                ignore_permissions: true,
            },
            PosInvoiceMergeLogAction::SubmitCreatedMergeLog {
                pos_invoices: vec!["POS-SALE-1".to_string()],
            },
            PosInvoiceMergeLogAction::CreateMergeLogDocument {
                posting_date: "2026-06-04".to_string(),
                posting_time: "21:15:00".to_string(),
                company: Some("_Test Company".to_string()),
                customer: "_Test Customer".to_string(),
                pos_closing_entry: Some("POS-CLOSE-0001".to_string()),
                pos_invoices: vec!["POS-RET-1".to_string(), "POS-SALE-2".to_string()],
                ignore_permissions: true,
            },
            PosInvoiceMergeLogAction::SubmitCreatedMergeLog {
                pos_invoices: vec!["POS-RET-1".to_string(), "POS-SALE-2".to_string()],
            },
            PosInvoiceMergeLogAction::SetClosingEntryStatus {
                status: "Submitted".to_string(),
            },
            PosInvoiceMergeLogAction::SetClosingEntryErrorMessage {
                error_message: "".to_string(),
            },
            PosInvoiceMergeLogAction::UpdateOpeningEntry { for_cancel: false },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_cancel_merge_logs_plan_skips_already_cancelled_logs() {
    let closing_entry = PosInvoiceMergeLogClosingEntry {
        name: "POS-CLOSE-0001".to_string(),
        posting_date: "2026-06-04".to_string(),
        posting_time: "21:15:00".to_string(),
        company: "_Test Company".to_string(),
    };
    let merge_logs = vec![
        PosInvoiceMergeLogCancelCandidate {
            name: "PIML-0001".to_string(),
            docstatus: 1,
        },
        PosInvoiceMergeLogCancelCandidate {
            name: "PIML-0002".to_string(),
            docstatus: 2,
        },
    ];

    assert_eq!(
        cancel_merge_logs_plan(&merge_logs, Some(&closing_entry)),
        vec![
            PosInvoiceMergeLogAction::CancelMergeLog {
                merge_log: "PIML-0001".to_string(),
                ignore_permissions: true,
            },
            PosInvoiceMergeLogAction::SetClosingEntryStatus {
                status: "Cancelled".to_string(),
            },
            PosInvoiceMergeLogAction::SetClosingEntryErrorMessage {
                error_message: "".to_string(),
            },
            PosInvoiceMergeLogAction::UpdateOpeningEntry { for_cancel: true },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_distinguishes_returns_by_consolidated_original_invoice() {
    let returns = vec![
        PosInvoiceMergeLogReturnInvoice {
            name: "POS-RET-1".to_string(),
            return_against: "POS-SALE-1".to_string(),
            return_against_consolidated_invoice: Some("SI-OLD-1".to_string()),
        },
        PosInvoiceMergeLogReturnInvoice {
            name: "POS-RET-2".to_string(),
            return_against: "POS-SALE-2".to_string(),
            return_against_consolidated_invoice: None,
        },
        PosInvoiceMergeLogReturnInvoice {
            name: "POS-RET-3".to_string(),
            return_against: "POS-SALE-3".to_string(),
            return_against_consolidated_invoice: Some("SI-OLD-1".to_string()),
        },
    ];

    assert_eq!(
        distinguish_return_pos_invoices_plan(&returns, Some("SI-NEW-1")),
        BTreeMap::from([
            (Some("SI-NEW-1".to_string()), vec!["POS-RET-2".to_string()],),
            (
                Some("SI-OLD-1".to_string()),
                vec!["POS-RET-1".to_string(), "POS-RET-3".to_string()],
            ),
        ])
    );
}

#[test]
fn pos_invoice_merge_log_new_sales_invoice_plan_matches_erpnext_defaults() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    assert_eq!(
        log.get_new_sales_invoice_plan(),
        NewSalesInvoicePlan {
            customer: Some("_Test Customer".to_string()),
            is_pos: true,
            posting_date: None,
            posting_time: None,
        }
    );
}

#[test]
fn pos_invoice_merge_log_process_sales_invoice_plan_matches_erpnext_order() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    assert_eq!(
        log.process_merging_into_sales_invoice_plan(
            "SI-0001",
            &["POS-SALE-1".to_string(), "POS-SALE-2".to_string()],
            false,
            false,
        ),
        vec![
            PosInvoiceMergeLogAction::MergeIntoSalesInvoice {
                sales_invoice: "SI-0001".to_string(),
                is_return: false,
                source_pos_invoices: vec!["POS-SALE-1".to_string(), "POS-SALE-2".to_string()],
            },
            PosInvoiceMergeLogAction::SetSalesInvoiceConsolidated {
                sales_invoice: "SI-0001".to_string(),
                is_consolidated: true,
            },
            PosInvoiceMergeLogAction::SetSalesInvoicePostingTimeFlag {
                sales_invoice: "SI-0001".to_string(),
                set_posting_time: true,
            },
            PosInvoiceMergeLogAction::SetSalesInvoicePostingDate {
                sales_invoice: "SI-0001".to_string(),
                posting_date: "2026-06-04".to_string(),
            },
            PosInvoiceMergeLogAction::SetSalesInvoicePostingTime {
                sales_invoice: "SI-0001".to_string(),
                posting_time: "12:00:00".to_string(),
            },
            PosInvoiceMergeLogAction::SaveSalesInvoice {
                sales_invoice: "SI-0001".to_string(),
            },
            PosInvoiceMergeLogAction::SubmitSalesInvoice {
                sales_invoice: "SI-0001".to_string(),
            },
            PosInvoiceMergeLogAction::SetMergeLogConsolidatedInvoice {
                sales_invoice: "SI-0001".to_string(),
            },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_process_credit_notes_plan_skips_empty_groups_and_returns_mapping() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");
    let returns = BTreeMap::from([
        (Some("SI-0001".to_string()), vec!["POS-RET-1".to_string()]),
        (Some("SI-OLD-1".to_string()), Vec::new()),
    ]);

    let (actions, credit_notes) =
        log.process_merging_into_credit_notes_plan(&returns, &["SI-CR-1".to_string()]);

    assert_eq!(
        credit_notes,
        BTreeMap::from([("SI-CR-1".to_string(), vec!["POS-RET-1".to_string()])])
    );
    assert_eq!(
        actions,
        vec![
            PosInvoiceMergeLogAction::MergeIntoSalesInvoice {
                sales_invoice: "SI-CR-1".to_string(),
                is_return: true,
                source_pos_invoices: vec!["POS-RET-1".to_string()],
            },
            PosInvoiceMergeLogAction::SetSalesInvoiceReturnAgainst {
                sales_invoice: "SI-CR-1".to_string(),
                return_against: Some("SI-0001".to_string()),
            },
            PosInvoiceMergeLogAction::SetSalesInvoiceConsolidated {
                sales_invoice: "SI-CR-1".to_string(),
                is_consolidated: true,
            },
            PosInvoiceMergeLogAction::SetSalesInvoicePostingTimeFlag {
                sales_invoice: "SI-CR-1".to_string(),
                set_posting_time: true,
            },
            PosInvoiceMergeLogAction::SetSalesInvoicePostingDate {
                sales_invoice: "SI-CR-1".to_string(),
                posting_date: "2026-06-04".to_string(),
            },
            PosInvoiceMergeLogAction::SetSalesInvoicePostingTime {
                sales_invoice: "SI-CR-1".to_string(),
                posting_time: "12:00:00".to_string(),
            },
            PosInvoiceMergeLogAction::SaveSalesInvoice {
                sales_invoice: "SI-CR-1".to_string(),
            },
            PosInvoiceMergeLogAction::SubmitSalesInvoice {
                sales_invoice: "SI-CR-1".to_string(),
            },
            PosInvoiceMergeLogAction::SetMergeLogConsolidatedCreditNote {
                sales_invoice: "SI-CR-1".to_string(),
            },
        ]
    );
}

#[test]
fn pos_invoice_merge_log_preserves_controller_hooks() {
    let log = PosInvoiceMergeLog::new("_Test Company", "2026-06-04", "12:00:00", "_Test Customer");

    assert_eq!(log.doctype(), "POS Invoice Merge Log");
    assert_eq!(log.module(), "Accounts");
    assert_eq!(
        log.custom_hooks(),
        vec!["validate", "on_submit", "on_cancel"]
    );
}
