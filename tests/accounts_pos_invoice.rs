use tokio_erp::erpnext::accounts::doctype::pos_invoice::pos_invoice::{
    create_payments_on_invoice, get_bundle_availability, get_pos_reserved_qty,
    get_stock_availability, item_query_plan, make_merge_log_plan, make_sales_return_plan,
    BundleAvailabilityRow, ClearUnallocatedPaymentsPlan, ConsolidatedSalesInvoicePlan,
    ItemQueryPlan, LoyaltyTransactionPlan, MergeLogInvoiceInput, MergeLogPlan, MissingValuesPlan,
    OnSubmitPlan, PaymentRequestPlan, PosInvoice, PosInvoiceError, PosInvoiceItem,
    PosInvoicePayment, PosProfile, ProductBundleItem, ReturnSalesInvoiceItemPlan,
    ReturnSalesInvoicePlan, SalesInvoicePaymentPlan, SerialBatchBundlePlan,
    SerialBatchBundleSubmitPlan, SetPosFieldsPlan, StockAvailability, UpdatePaymentsPlan,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

fn base_invoice() -> PosInvoice {
    PosInvoice {
        name: Some("POS-0001".to_string()),
        company: Some("TC".to_string()),
        customer: Some("CUST-001".to_string()),
        is_pos: true,
        grand_total: 100.0,
        rounded_total: Some(100.0),
        base_grand_total: 1_250_000.0,
        base_rounded_total: Some(1_250_000.0),
        conversion_rate: 12_500.0,
        paid_amount: 100.0,
        base_paid_amount: 1_250_000.0,
        docstatus: 1,
        due_date: Some("2026-06-05".to_string()),
        posting_date: Some("2026-06-05".to_string()),
        payments: vec![PosInvoicePayment {
            idx: 1,
            mode_of_payment: Some("Cash".to_string()),
            payment_type: "Cash".to_string(),
            account: Some("Cash - TC".to_string()),
            amount: 100.0,
        }],
        items: vec![PosInvoiceItem {
            idx: 1,
            item_code: "ITEM-001".to_string(),
            warehouse: Some("Stores - TC".to_string()),
            stock_qty: 2.0,
            qty: 2.0,
            ..PosInvoiceItem::default()
        }],
        ..PosInvoice::default()
    }
}

#[test]
fn pos_invoice_metadata_and_basic_validate_guards_match_erpnext() {
    assert_eq!(PosInvoice::DOCTYPE, "POS Invoice");
    assert_eq!(PosInvoice::MODULE, "Accounts");
    assert_eq!(PosInvoice::AUTONAME, "naming_series:");
    assert!(PosInvoice::IS_SUBMITTABLE);
    assert_eq!(PosInvoice::FIELD_ORDER[0], "naming_series");

    let doc = base_invoice();
    assert_eq!(doc.doctype(), "POS Invoice");
    assert_eq!(
        PosInvoice::fields(),
        vec![
            FieldSpec::data("naming_series", "Series").default("ACC-PSINV-.YYYY.-"),
            FieldSpec::link("customer", "Customer").options("Customer"),
            FieldSpec::check("is_pos", "Include Payment").default("1"),
            FieldSpec::table("items", "Items").options("POS Invoice Item"),
            FieldSpec::table("payments", "Payments").options("Sales Invoice Payment"),
            FieldSpec::select("status", "Status")
                .options("\nDraft\nReturn\nCredit Note Issued\nConsolidated\nSubmitted\nPaid\nPartly Paid\nUnpaid\nPartly Paid and Discounted\nUnpaid and Discounted\nOverdue and Discounted\nOverdue\nCancelled"),
        ]
    );
    assert!(doc.validate_basic().is_ok());

    let missing_customer = PosInvoice {
        is_pos: true,
        ..PosInvoice::default()
    };
    assert_eq!(
        missing_customer.validate_basic().unwrap_err(),
        PosInvoiceError::Validation("Please select Customer first".to_string())
    );
    let not_pos = PosInvoice {
        customer: Some("CUST-001".to_string()),
        is_pos: false,
        ..PosInvoice::default()
    };
    assert_eq!(
        not_pos.validate_basic().unwrap_err(),
        PosInvoiceError::Validation(
            "POS Invoice should have the field Include Payment checked.".to_string()
        )
    );
}

#[test]
fn pos_invoice_payment_change_company_and_outstanding_rules_match_erpnext() {
    let mut doc = base_invoice();
    assert!(doc.validate_mode_of_payment().is_ok());
    assert!(doc.validate_payment_amount(2).is_ok());

    doc.payments[0].amount = -1.0;
    assert_eq!(
        doc.validate_payment_amount(2).unwrap_err(),
        PosInvoiceError::Validation("Row #1 (Payment Table): Amount must be positive".to_string())
    );
    doc.is_return = true;
    doc.payments[0].amount = 1.0;
    assert_eq!(
        doc.validate_payment_amount(2).unwrap_err(),
        PosInvoiceError::Validation("Row #1 (Payment Table): Amount must be negative".to_string())
    );

    let empty_payments = PosInvoice {
        customer: Some("CUST-001".to_string()),
        is_pos: true,
        ..PosInvoice::default()
    };
    assert_eq!(
        empty_payments.validate_mode_of_payment().unwrap_err(),
        PosInvoiceError::Validation(
            "At least one mode of payment is required for POS invoice.".to_string()
        )
    );

    let mut change = base_invoice();
    change.paid_amount = 120.0;
    change.base_paid_amount = 1_500_000.0;
    change.write_off_amount = 5.0;
    change.base_write_off_amount = 62_500.0;
    change.validate_change_amount();
    assert_eq!(change.change_amount, 25.0);
    assert_eq!(change.base_change_amount, 312_500.0);
    assert_eq!(
        change.validate_change_account(None).unwrap_err(),
        PosInvoiceError::Validation("Please enter Account for Change Amount".to_string())
    );
    assert!(change
        .validate_company_with_pos_company(Some("TC"), Some("TC"))
        .is_ok());
    assert_eq!(
        change
            .validate_company_with_pos_company(Some("TC"), Some("Other"))
            .unwrap_err(),
        PosInvoiceError::Validation(
            "Company TC does not match with POS Profile Company Other".to_string()
        )
    );

    let mut outstanding = base_invoice();
    outstanding.paid_amount = 40.0;
    outstanding.set_outstanding_amount();
    assert_eq!(outstanding.outstanding_amount, 60.0);
}

#[test]
fn pos_invoice_status_lifecycle_and_phone_payment_rules_match_erpnext() {
    let mut doc = base_invoice();
    doc.outstanding_amount = 0.0;
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Paid");

    doc.outstanding_amount = 25.0;
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Partly Paid");
    doc.due_date = Some("2026-06-01".to_string());
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Overdue");
    doc.consolidated_invoice = Some("SINV-0001".to_string());
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Consolidated");
    doc.docstatus = 2;
    assert_eq!(doc.set_status(false, None, "2026-06-05"), "Cancelled");

    let mut discounted = base_invoice();
    discounted.outstanding_amount = 25.0;
    discounted.is_discounted = true;
    discounted.discounting_status = Some("Disbursed".to_string());
    discounted.due_date = Some("2026-06-01".to_string());
    assert_eq!(
        discounted.set_status(false, None, "2026-06-05"),
        "Overdue and Discounted"
    );
    discounted.due_date = Some("2026-06-05".to_string());
    assert_eq!(
        discounted.set_status(false, None, "2026-06-05"),
        "Partly Paid and Discounted"
    );
    discounted.outstanding_amount = 125.0;
    assert_eq!(
        discounted.set_status(false, None, "2026-06-05"),
        "Unpaid and Discounted"
    );

    let mut credited = base_invoice();
    credited.outstanding_amount = 0.0;
    credited.has_submitted_return = true;
    assert_eq!(
        credited.set_status(false, None, "2026-06-05"),
        "Credit Note Issued"
    );

    assert_eq!(doc.before_submit_plan(), "set_outstanding_amount");
    assert_eq!(
        doc.before_cancel_plan(true, Some("POS-CLOSE-0001")).unwrap_err(),
        PosInvoiceError::Validation(
            "You need to cancel POS Closing Entry POS-CLOSE-0001 to be able to cancel this document."
                .to_string()
        )
    );
    assert_eq!(
        doc.on_cancel_plan(),
        vec![
            "ignore_linked:Payment Ledger Entry,Serial and Batch Bundle".to_string(),
            "sales_invoice_on_cancel".to_string(),
            "set_status:Cancelled".to_string(),
            "delink_serial_and_batch_bundle".to_string(),
        ]
    );

    let mut phone = base_invoice();
    phone.payments[0].payment_type = "Phone".to_string();
    phone.payments[0].amount = 50.0;
    assert_eq!(
        phone.check_phone_payments(Some(40.0)).unwrap_err(),
        PosInvoiceError::Validation("Payment related to Cash is not completed".to_string())
    );
}

#[test]
fn pos_invoice_stock_serial_batch_and_return_guards_match_erpnext() {
    let mut doc = base_invoice();
    assert_eq!(
        doc.validate_is_pos_using_sales_invoice("Sales Invoice")
            .unwrap_err(),
        PosInvoiceError::Validation(
            "Sales Invoice mode is activated in POS. Please create Sales Invoice instead."
                .to_string()
        )
    );

    doc.is_return = true;
    assert!(doc
        .validate_is_pos_using_sales_invoice("Sales Invoice")
        .is_ok());
    doc.items[0].qty = 1.0;
    assert_eq!(
        doc.validate_return_items_qty(&[]).unwrap_err(),
        PosInvoiceError::Validation(
            "Row #1: You cannot add positive quantities in a return invoice. Please remove item ITEM-001 to complete the return."
                .to_string()
        )
    );

    let mut serial_doc = base_invoice();
    serial_doc.items[0].has_serial_no = true;
    assert_eq!(
        serial_doc
            .validate_serialised_or_batched_item()
            .unwrap_err(),
        PosInvoiceError::Validation(
            "Row #1: Please select Serial No. for item ITEM-001".to_string()
        )
    );
    serial_doc.items[0].has_serial_no = false;
    serial_doc.items[0].has_batch_no = true;
    assert_eq!(
        serial_doc
            .validate_serialised_or_batched_item()
            .unwrap_err(),
        PosInvoiceError::Validation(
            "Row #1: Please select Batch No. for item ITEM-001".to_string()
        )
    );

    let stock_doc = base_invoice();
    assert_eq!(
        stock_doc
            .validate_stock_availability(
                false,
                &[StockAvailability {
                    item_code: "ITEM-001".to_string(),
                    warehouse: "Stores - TC".to_string(),
                    available: 1.5,
                    is_stock_item: true,
                    is_negative_stock_allowed: false,
                    bundle_items: Vec::new(),
                }],
            )
            .unwrap_err(),
        PosInvoiceError::Validation(
            "Row #1: Item ITEM-001 in warehouse Stores - TC: Available 1.5, Needed 2.".to_string()
        )
    );
    assert!(stock_doc
        .validate_stock_availability(
            false,
            &[StockAvailability {
                item_code: "ITEM-001".to_string(),
                warehouse: "Stores - TC".to_string(),
                available: 1.5,
                is_stock_item: true,
                is_negative_stock_allowed: true,
                bundle_items: Vec::new(),
            }],
        )
        .is_ok());

    assert_eq!(
        get_stock_availability(
            "ITEM-001",
            "Stores - TC",
            true,
            false,
            10.0,
            3.0,
            false,
            None
        ),
        StockAvailability {
            item_code: "ITEM-001".to_string(),
            warehouse: "Stores - TC".to_string(),
            available: 7.0,
            is_stock_item: true,
            is_negative_stock_allowed: false,
            bundle_items: Vec::new(),
        }
    );
    assert_eq!(
        get_bundle_availability(
            "BUNDLE-001",
            "Stores - TC",
            &[
                ProductBundleItem {
                    item_code: "ITEM-A".to_string(),
                    qty: 2.0,
                    bin_qty: 10.0,
                    is_stock_item: true,
                },
                ProductBundleItem {
                    item_code: "ITEM-B".to_string(),
                    qty: 5.0,
                    bin_qty: 20.0,
                    is_stock_item: true,
                },
            ],
            1.0,
        ),
        3.0
    );
    assert_eq!(get_pos_reserved_qty(2.5, 1.5), 4.0);
    assert_eq!(
        StockAvailability::bundle(
            "BUNDLE-001",
            "Stores - TC",
            vec![BundleAvailabilityRow {
                item_code: "ITEM-A".to_string(),
                required: 4.0,
                available: 3.0,
            }],
            false,
        )
        .bundle_error(1),
        Some("<b>Row #1:</b> Bundle BUNDLE-001 in warehouse Stores - TC has insufficient packed items:<br><div style='margin-top: 15px;'><ul style='line-height: 0.8;'><li>Packed Item ITEM-A: Required 4, Available 3</li></ul></div>".to_string())
    );
}

#[test]
fn pos_invoice_profile_payment_request_update_and_query_plans_match_erpnext() {
    let mut doc = base_invoice();
    doc.pos_profile = Some("POS-PROFILE-001".to_string());
    doc.contact_mobile = Some("+998901234567".to_string());
    doc.debit_to = None;
    doc.due_date = None;

    let profile = PosProfile {
        name: "POS-PROFILE-001".to_string(),
        company: "TC".to_string(),
        customer: Some("CUST-POS".to_string()),
        currency: Some("USD".to_string()),
        warehouse: Some("Stores - TC".to_string()),
        account_for_change_amount: Some("Cash - TC".to_string()),
        print_format: Some("POS Invoice".to_string()),
        allow_print_before_pay: true,
        set_grand_total_to_default_mop: true,
        utm_source: Some("Counter".to_string()),
        utm_campaign: Some("Retail".to_string()),
        utm_medium: Some("POS".to_string()),
        selling_price_list: Some("Retail USD".to_string()),
    };
    assert_eq!(
        doc.set_missing_values_plan(
            &profile,
            false,
            Some("Debtors - TC"),
            Some("USD"),
            Some("2026-06-30"),
        ),
        MissingValuesPlan {
            pos_profile: "POS-PROFILE-001".to_string(),
            company: Some("TC".to_string()),
            customer: Some("CUST-001".to_string()),
            debit_to: Some("Debtors - TC".to_string()),
            party_account_currency: Some("USD".to_string()),
            due_date: Some("2026-06-30".to_string()),
            print_format: Some("POS Invoice".to_string()),
            allow_print_before_pay: true,
            set_default_payment: true,
            utm_source: Some("Counter".to_string()),
            utm_campaign: Some("Retail".to_string()),
            utm_medium: Some("POS".to_string()),
        }
    );
    assert_eq!(
        doc.reset_mode_of_payments_plan(),
        Some("update_multi_mode_option")
    );

    let mut phone = base_invoice();
    phone.contact_mobile = Some("+998901234567".to_string());
    phone.payments[0].payment_type = "Phone".to_string();
    phone.payments[0].mode_of_payment = Some("PhonePe".to_string());
    phone.payments[0].account = Some("Phone Gateway - TC".to_string());
    phone.payments[0].amount = 75.0;
    assert_eq!(
        phone.create_payment_request_plan(false).unwrap(),
        PaymentRequestPlan {
            use_existing_request: false,
            reference_doctype: "POS Invoice".to_string(),
            reference_name: "POS-0001".to_string(),
            recipient_id: Some("+998901234567".to_string()),
            mode_of_payment: Some("PhonePe".to_string()),
            payment_account: Some("Phone Gateway - TC".to_string()),
            payment_request_type: "Inward".to_string(),
            party_type: "Customer".to_string(),
            party: Some("CUST-001".to_string()),
            return_doc: true,
        }
    );
    phone.payments[0].amount = 0.0;
    assert_eq!(
        phone.create_payment_request_plan(false).unwrap_err(),
        PosInvoiceError::Validation("Payment amount cannot be less than or equal to 0".to_string())
    );

    let mut paid = base_invoice();
    paid.paid_amount = 60.0;
    assert_eq!(
        paid.update_payments_plan(
            &[PosInvoicePayment {
                idx: 0,
                mode_of_payment: Some("Card".to_string()),
                payment_type: "Card".to_string(),
                account: Some("Card - TC".to_string()),
                amount: 30.0,
            }],
            2,
        )
        .unwrap(),
        UpdatePaymentsPlan {
            new_paid_amount: 90.0,
            new_base_paid_amount: 1_125_000.0,
            new_outstanding_amount: 10.0,
            new_change_amount: 0.0,
            added_payment_count: 1,
            set_status_update: true,
        }
    );

    assert_eq!(
        make_sales_return_plan("POS-0001"),
        ("POS Invoice".to_string(), "POS-0001".to_string())
    );
    assert_eq!(
        make_merge_log_plan(vec![MergeLogInvoiceInput {
            name: "POS-0001".to_string(),
            customer: "CUST-001".to_string(),
            posting_date: "2026-06-05".to_string(),
            grand_total: 100.0,
        }])
        .unwrap(),
        MergeLogPlan {
            posting_date_source: "today".to_string(),
            customer: Some("CUST-001".to_string()),
            invoices: vec![MergeLogInvoiceInput {
                name: "POS-0001".to_string(),
                customer: "CUST-001".to_string(),
                posting_date: "2026-06-05".to_string(),
                grand_total: 100.0,
            }],
        }
    );
    assert_eq!(
        item_query_plan(
            "Item",
            "coffee",
            "item_name",
            0,
            20,
            Some(vec!["Products".to_string(), "Beverages".to_string()]),
            false,
        ),
        ItemQueryPlan {
            doctype: "Item".to_string(),
            txt: "coffee".to_string(),
            searchfield: "item_name".to_string(),
            start: 0,
            page_len: 20,
            item_groups: Some(vec!["Products".to_string(), "Beverages".to_string()]),
            as_dict: false,
        }
    );
    assert_eq!(
        create_payments_on_invoice(&doc, 2, &doc.payments[0]),
        SalesInvoicePaymentPlan {
            idx: 2,
            mode_of_payment: Some("Cash".to_string()),
            amount: 100.0,
            base_amount: 1_250_000.0,
            parent: Some("POS-0001".to_string()),
            parentfield: "payments".to_string(),
        }
    );
}

#[test]
fn pos_invoice_return_invoice_cleanup_and_serial_bundle_plans_match_erpnext() {
    let mut doc = base_invoice();
    doc.name = Some("POS-RET-0001".to_string());
    doc.is_return = true;
    doc.return_against = Some("POS-ORIG-0001".to_string());
    doc.items[0].name = Some("POS-RET-ITEM-0001".to_string());
    doc.items[0].pos_invoice_item = Some("POS-ORIG-ITEM-0001".to_string());
    doc.items[0].serial_and_batch_bundle = Some("SBB-0001".to_string());
    doc.payments.push(PosInvoicePayment {
        idx: 2,
        mode_of_payment: Some("Card".to_string()),
        payment_type: "Card".to_string(),
        account: Some("Card - TC".to_string()),
        amount: 0.0,
    });

    assert_eq!(
        doc.clear_unallocated_mode_of_payments_plan(),
        ClearUnallocatedPaymentsPlan {
            kept_payments: vec![doc.payments[0].clone()],
            delete_zero_payments_for_parent: Some("POS-RET-0001".to_string()),
        }
    );
    assert_eq!(
        doc.create_return_sales_invoice_plan(
            Some("SINV-ORIG-0001"),
            &[(
                "POS-ORIG-ITEM-0001".to_string(),
                "SINV-ITEM-0001".to_string()
            )],
        ),
        ReturnSalesInvoicePlan {
            source_pos_invoice: Some("POS-RET-0001".to_string()),
            is_pos: true,
            is_return: true,
            is_created_using_pos: true,
            is_consolidated: true,
            return_against: Some("SINV-ORIG-0001".to_string()),
            items: vec![ReturnSalesInvoiceItemPlan {
                source_name: Some("POS-RET-ITEM-0001".to_string()),
                pos_invoice: Some("POS-RET-0001".to_string()),
                pos_invoice_item: Some("POS-RET-ITEM-0001".to_string()),
                sales_invoice_item: Some("SINV-ITEM-0001".to_string()),
            }],
            payment_count: 2,
        }
    );
    assert_eq!(
        doc.create_and_add_consolidated_sales_invoice_plan(
            "SINV-RET-0001",
            Some("SINV-ORIG-0001"),
            &[(
                "POS-ORIG-ITEM-0001".to_string(),
                "SINV-ITEM-0001".to_string()
            )],
        ),
        ConsolidatedSalesInvoicePlan {
            sales_invoice_name: "SINV-RET-0001".to_string(),
            db_set_consolidated_invoice: Some("SINV-RET-0001".to_string()),
            set_status_update: true,
            return_sales_invoice: ReturnSalesInvoicePlan {
                source_pos_invoice: Some("POS-RET-0001".to_string()),
                is_pos: true,
                is_return: true,
                is_created_using_pos: true,
                is_consolidated: true,
                return_against: Some("SINV-ORIG-0001".to_string()),
                items: vec![ReturnSalesInvoiceItemPlan {
                    source_name: Some("POS-RET-ITEM-0001".to_string()),
                    pos_invoice: Some("POS-RET-0001".to_string()),
                    pos_invoice_item: Some("POS-RET-ITEM-0001".to_string()),
                    sales_invoice_item: Some("SINV-ITEM-0001".to_string()),
                }],
                payment_count: 2,
            },
        }
    );
    assert_eq!(
        doc.delink_serial_and_batch_bundle_plan(),
        vec![SerialBatchBundlePlan {
            bundle: "SBB-0001".to_string(),
            clear_voucher_no: true,
            cancel_bundle: true,
            clear_row_link: true,
        }]
    );
    assert_eq!(
        doc.submit_serial_batch_bundle_plan("items", &["SBB-0001".to_string()]),
        vec![SerialBatchBundleSubmitPlan {
            table_name: "items".to_string(),
            bundle: "SBB-0001".to_string(),
            ignore_voucher_validation: true,
            submit: true,
        }]
    );
}

#[test]
fn pos_invoice_pos_field_defaults_and_submit_orchestration_match_erpnext() {
    let profile = PosProfile {
        name: "POS-PROFILE-001".to_string(),
        company: "TC".to_string(),
        customer: Some("CUST-POS".to_string()),
        currency: Some("USD".to_string()),
        warehouse: Some("Stores - TC".to_string()),
        account_for_change_amount: Some("Cash - TC".to_string()),
        print_format: None,
        allow_print_before_pay: false,
        set_grand_total_to_default_mop: false,
        utm_source: None,
        utm_campaign: None,
        utm_medium: None,
        selling_price_list: Some("Retail USD".to_string()),
    };
    let draft = PosInvoice {
        company: Some("OLD".to_string()),
        is_pos: true,
        is_return: true,
        items: vec![PosInvoiceItem {
            idx: 1,
            item_code: "ITEM-001".to_string(),
            ..PosInvoiceItem::default()
        }],
        ..PosInvoice::default()
    };
    assert_eq!(
        draft
            .set_pos_fields_plan(
                Some(&profile),
                false,
                Some("Default Cash - TC"),
                Some("Customer Retail USD"),
                None,
                Some("EUR"),
            )
            .unwrap(),
        SetPosFieldsPlan {
            pos_profile: Some("POS-PROFILE-001".to_string()),
            company: Some("TC".to_string()),
            customer: Some("CUST-POS".to_string()),
            account_for_change_amount: Some("Cash - TC".to_string()),
            set_warehouse: Some("Stores - TC".to_string()),
            update_multi_mode_option: true,
            add_return_modes: true,
            selling_price_list: Some("Customer Retail USD".to_string()),
            currency: Some("EUR".to_string()),
            item_defaults_count: 1,
        }
    );
    assert_eq!(
        PosInvoice::default()
            .set_pos_fields_plan(None, false, None, None, None, None)
            .unwrap_err(),
        PosInvoiceError::Validation(
            "No POS Profile found. Please create a New POS Profile first".to_string()
        )
    );

    let mut submit = base_invoice();
    submit.loyalty_program = Some("LP-001".to_string());
    submit.redeem_loyalty_points = true;
    submit.loyalty_points = 25;
    submit.coupon_code = Some("COUPON-001".to_string());
    assert_eq!(
        submit.on_submit_plan("POS Invoice"),
        OnSubmitPlan {
            actions: vec![
                "make_loyalty_point_entry".to_string(),
                "apply_loyalty_points".to_string(),
                "check_phone_payments".to_string(),
                "set_status:update".to_string(),
                "make_bundle_for_sales_purchase_return".to_string(),
                "make_bundle_using_old_serial_batch_fields:items".to_string(),
                "submit_serial_batch_bundle:items".to_string(),
                "make_bundle_using_old_serial_batch_fields:packed_items".to_string(),
                "submit_serial_batch_bundle:packed_items".to_string(),
                "update_coupon_code_count:used".to_string(),
                "clear_unallocated_mode_of_payments".to_string(),
            ],
        }
    );

    let mut return_submit = base_invoice();
    return_submit.is_return = true;
    return_submit.return_against = Some("POS-ORIG-0001".to_string());
    return_submit.loyalty_program = Some("LP-001".to_string());
    assert!(return_submit
        .on_submit_plan("Sales Invoice")
        .actions
        .contains(&"create_and_add_consolidated_sales_invoice".to_string()));

    let mut loyalty = base_invoice();
    loyalty.redeem_loyalty_points = true;
    loyalty.loyalty_program = Some("LP-001".to_string());
    loyalty.loyalty_points = 25;
    assert_eq!(
        loyalty.validate_loyalty_transaction_plan(Some("Loyalty Expense - TC"), Some("Main - TC")),
        LoyaltyTransactionPlan {
            loyalty_redemption_account: Some("Loyalty Expense - TC".to_string()),
            loyalty_redemption_cost_center: Some("Main - TC".to_string()),
            validate_loyalty_points: true,
        }
    );

    let mut cancel_return = base_invoice();
    cancel_return.is_return = true;
    cancel_return.return_against = Some("POS-ORIG-0001".to_string());
    cancel_return.loyalty_program = Some("LP-001".to_string());
    assert_eq!(
        cancel_return.on_cancel_plan(),
        vec![
            "ignore_linked:Payment Ledger Entry,Serial and Batch Bundle".to_string(),
            "sales_invoice_on_cancel".to_string(),
            "return_against_delete_loyalty_point_entry".to_string(),
            "return_against_make_loyalty_point_entry".to_string(),
            "set_status:Cancelled".to_string(),
            "delink_serial_and_batch_bundle".to_string(),
        ]
    );
}
