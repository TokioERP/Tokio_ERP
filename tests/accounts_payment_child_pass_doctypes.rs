use tokio_erp::erpnext::accounts::doctype::advance_taxes_and_charges::advance_taxes_and_charges::AdvanceTaxesAndCharges;
use tokio_erp::erpnext::accounts::doctype::exchange_rate_revaluation_account::exchange_rate_revaluation_account::ExchangeRateRevaluationAccount;
use tokio_erp::erpnext::accounts::doctype::opening_invoice_creation_tool_item::opening_invoice_creation_tool_item::OpeningInvoiceCreationToolItem;
use tokio_erp::erpnext::accounts::doctype::overdue_payment::overdue_payment::OverduePayment;
use tokio_erp::erpnext::accounts::doctype::payment_order_reference::payment_order_reference::PaymentOrderReference;
use tokio_erp::erpnext::accounts::doctype::payment_reconciliation_invoice::payment_reconciliation_invoice::PaymentReconciliationInvoice;
use tokio_erp::erpnext::accounts::doctype::payment_reconciliation_payment::payment_reconciliation_payment::PaymentReconciliationPayment;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn advance_taxes_and_charges_matches_erpnext_metadata_shape() {
    assert_eq!(AdvanceTaxesAndCharges::DOCTYPE, "Advance Taxes and Charges");
    assert!(AdvanceTaxesAndCharges::IS_TABLE);
    assert_eq!(
        AdvanceTaxesAndCharges::FIELD_ORDER,
        [
            "add_deduct_tax",
            "charge_type",
            "row_id",
            "account_head",
            "col_break_1",
            "description",
            "included_in_paid_amount",
            "set_by_item_tax_template",
            "is_tax_withholding_account",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
            "project",
            "section_break_8",
            "rate",
            "section_break_9",
            "currency",
            "net_amount",
            "tax_amount",
            "total",
            "column_break_13",
            "base_tax_amount",
            "base_net_amount",
            "base_total",
        ]
    );
    assert_eq!(AdvanceTaxesAndCharges::fields().len(), 24);
    assert_eq!(
        AdvanceTaxesAndCharges::fields()[0],
        FieldSpec::select("charge_type", "Type")
            .options("\nActual\nOn Paid Amount\nOn Previous Row Amount\nOn Previous Row Total")
            .oldfield("charge_type", "Select")
            .columns(2)
            .required()
            .in_list_view()
    );
    assert_eq!(
        AdvanceTaxesAndCharges::fields()[1],
        FieldSpec::data("row_id", "Reference Row #")
            .oldfield("row_id", "Data")
            .depends_on(
                "eval:[\"On Previous Row Amount\", \"On Previous Row Total\"].indexOf(doc.charge_type)!==-1"
            )
    );
    assert_eq!(
        AdvanceTaxesAndCharges::fields()[12],
        FieldSpec::currency("tax_amount", "Amount")
            .options("currency")
            .columns(2)
            .in_list_view()
    );
    assert_eq!(
        AdvanceTaxesAndCharges::fields()[22],
        FieldSpec::check("set_by_item_tax_template", "Set by Item Tax Template")
            .default("0")
            .read_only()
            .hidden()
            .print_hide()
            .report_hide()
    );
}

#[test]
fn payment_order_reference_matches_erpnext_metadata_shape() {
    assert_eq!(PaymentOrderReference::DOCTYPE, "Payment Order Reference");
    assert!(PaymentOrderReference::IS_TABLE);
    assert!(PaymentOrderReference::QUICK_ENTRY);
    assert!(PaymentOrderReference::TRACK_CHANGES);
    assert_eq!(
        PaymentOrderReference::FIELD_ORDER,
        [
            "reference_doctype",
            "reference_name",
            "amount",
            "column_break_4",
            "supplier",
            "payment_request",
            "mode_of_payment",
            "bank_account_details",
            "bank_account",
            "column_break_10",
            "account",
            "payment_reference",
        ]
    );
    assert_eq!(
        PaymentOrderReference::fields()[0],
        FieldSpec::link("reference_doctype", "Type")
            .options("DocType")
            .required()
            .read_only()
            .in_list_view()
    );
    let row = PaymentOrderReference::new("Purchase Invoice", "PINV-0001", 500.0, "BANK-001");
    assert_eq!(row.reference_name.as_deref(), Some("PINV-0001"));
    assert_eq!(row.amount, 500.0);
    assert_eq!(row.doctype(), "Payment Order Reference");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn payment_reconciliation_child_tables_match_erpnext_metadata_and_get_list_pass() {
    assert_eq!(
        PaymentReconciliationInvoice::FIELD_ORDER,
        [
            "invoice_type",
            "invoice_number",
            "invoice_date",
            "col_break1",
            "amount",
            "outstanding_amount",
            "currency",
            "exchange_rate",
        ]
    );
    assert!(PaymentReconciliationInvoice::IS_TABLE);
    assert!(PaymentReconciliationInvoice::QUICK_ENTRY);
    assert!(PaymentReconciliationInvoice::TRACK_CHANGES);
    assert_eq!(
        PaymentReconciliationInvoice::fields()[0],
        FieldSpec::select("invoice_type", "Invoice Type")
            .options("Sales Invoice\nPurchase Invoice\nJournal Entry")
            .read_only()
            .in_list_view()
    );
    assert_eq!(PaymentReconciliationInvoice::get_list("ignored"), ());

    assert_eq!(
        PaymentReconciliationInvoice::fields()[6],
        FieldSpec::link("currency", "Currency")
            .options("Currency")
            .hidden()
    );

    assert_eq!(
        PaymentReconciliationPayment::FIELD_ORDER,
        [
            "reference_type",
            "reference_name",
            "posting_date",
            "is_advance",
            "reference_row",
            "col_break1",
            "amount",
            "difference_amount",
            "sec_break1",
            "remarks",
            "currency",
            "exchange_rate",
            "cost_center",
        ]
    );
    assert!(PaymentReconciliationPayment::IS_TABLE);
    assert!(PaymentReconciliationPayment::QUICK_ENTRY);
    assert_eq!(
        PaymentReconciliationPayment::fields()[8],
        FieldSpec::link("currency", "Currency")
            .options("Currency")
            .hidden()
    );
    assert_eq!(
        PaymentReconciliationPayment::fields()[12],
        FieldSpec::small_text("remarks", "Remarks").read_only()
    );
    assert_eq!(PaymentReconciliationPayment::get_list("ignored"), ());
}

#[test]
fn revaluation_opening_and_overdue_child_tables_match_erpnext_metadata() {
    assert_eq!(
        ExchangeRateRevaluationAccount::FIELD_ORDER,
        [
            "account",
            "party_type",
            "party",
            "column_break_2",
            "account_currency",
            "account_balances",
            "balance_in_account_currency",
            "column_break_46yz",
            "new_balance_in_account_currency",
            "balances",
            "current_exchange_rate",
            "column_break_xown",
            "new_exchange_rate",
            "column_break_9",
            "balance_in_base_currency",
            "column_break_ukce",
            "new_balance_in_base_currency",
            "section_break_ngrs",
            "gain_loss",
            "zero_balance",
        ]
    );
    assert!(ExchangeRateRevaluationAccount::QUICK_ENTRY);
    assert!(ExchangeRateRevaluationAccount::TRACK_CHANGES);
    assert_eq!(
        ExchangeRateRevaluationAccount::fields()[7],
        FieldSpec::float("current_exchange_rate", "Current Exchange Rate")
            .read_only()
            .precision("9")
    );
    assert_eq!(
        ExchangeRateRevaluationAccount::fields()[13],
        FieldSpec::check("zero_balance", "Zero Balance")
            .default("0")
            .description(
                "This Account has '0' balance in either Base Currency or Account Currency"
            )
    );

    assert_eq!(
        OpeningInvoiceCreationToolItem::FIELD_ORDER,
        [
            "invoice_number",
            "party_type",
            "party",
            "party_name",
            "temporary_opening_account",
            "column_break_3",
            "posting_date",
            "due_date",
            "supplier_invoice_date",
            "section_break_5",
            "item_name",
            "outstanding_amount",
            "column_break_4",
            "qty",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
        ]
    );
    assert_eq!(OpeningInvoiceCreationToolItem::fields().len(), 17);
    assert_eq!(
        OpeningInvoiceCreationToolItem::fields()[0],
        FieldSpec::link("party_type", "Party Type")
            .options("DocType")
            .hidden()
            .read_only()
    );
    assert_eq!(
        OpeningInvoiceCreationToolItem::fields()[1],
        FieldSpec::dynamic_link("party")
            .label("Party ID")
            .options("party_type")
            .mandatory_depends_on("eval: !parent.create_missing_party")
            .in_list_view()
    );
    assert_eq!(
        OpeningInvoiceCreationToolItem::fields()[8],
        FieldSpec::currency("outstanding_amount", "Outstanding Amount")
            .default("0")
            .required()
            .in_list_view()
    );

    assert_eq!(OverduePayment::DOCTYPE, "Overdue Payment");
    assert_eq!(
        OverduePayment::FIELD_ORDER,
        [
            "sales_invoice",
            "payment_schedule",
            "dunning_level",
            "payment_term",
            "section_break_15",
            "description",
            "section_break_4",
            "due_date",
            "overdue_days",
            "mode_of_payment",
            "column_break_5",
            "invoice_portion",
            "section_break_16",
            "payment_amount",
            "outstanding",
            "paid_amount",
            "discounted_amount",
            "interest",
        ]
    );
    assert_eq!(OverduePayment::fields().len(), 18);
    assert_eq!(
        OverduePayment::fields()[0],
        FieldSpec::link("payment_term", "Payment Term")
            .options("Payment Term")
            .columns(2)
            .read_only()
            .print_hide()
    );
    assert_eq!(
        OverduePayment::fields()[2],
        FieldSpec::small_text("description", "Description")
            .columns(2)
            .fetch_from("payment_term.description")
            .read_only()
    );
    assert_eq!(
        OverduePayment::fields()[11],
        FieldSpec::currency("discounted_amount", "Discounted Amount")
            .default("0")
            .depends_on("discounted_amount")
            .read_only()
            .print_hide()
    );
    assert_eq!(
        OverduePayment::fields()[17],
        FieldSpec::currency("interest", "Interest")
            .options("currency")
            .read_only()
            .in_list_view()
    );
}
