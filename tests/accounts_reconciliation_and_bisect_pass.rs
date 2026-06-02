use tokio_erp::erpnext::accounts::doctype::bisect_nodes::bisect_nodes::BisectNodes;
use tokio_erp::erpnext::accounts::doctype::payment_reconciliation_allocation::payment_reconciliation_allocation::PaymentReconciliationAllocation;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bisect_nodes_matches_erpnext_pass_controller_metadata() {
    assert_eq!(BisectNodes::DOCTYPE, "Bisect Nodes");
    assert_eq!(BisectNodes::MODULE, "Accounts");
    assert_eq!(
        BisectNodes::FIELD_ORDER,
        [
            "root",
            "left_child",
            "right_child",
            "period_from_date",
            "period_to_date",
            "difference",
            "balance_sheet_summary",
            "profit_loss_summary",
            "generated",
        ]
    );
    assert_eq!(
        BisectNodes::fields(),
        vec![
            FieldSpec::link("root", "Root").options("Bisect Nodes"),
            FieldSpec::link("left_child", "Left Child").options("Bisect Nodes"),
            FieldSpec::link("right_child", "Right Child").options("Bisect Nodes"),
            FieldSpec::datetime("period_from_date", "Period_from_date"),
            FieldSpec::datetime("period_to_date", "Period To Date"),
            FieldSpec::float("difference", "Difference"),
            FieldSpec::float("balance_sheet_summary", "Balance Sheet Summary"),
            FieldSpec::float("profit_loss_summary", "Profit and Loss Summary"),
            FieldSpec::check("generated", "Generated").default("0"),
        ]
    );

    let node = BisectNodes;
    assert_eq!(node.doctype(), "Bisect Nodes");
    assert!(node.custom_hooks().is_empty());
}

#[test]
fn payment_reconciliation_allocation_matches_erpnext_virtual_child_metadata() {
    assert_eq!(
        PaymentReconciliationAllocation::DOCTYPE,
        "Payment Reconciliation Allocation"
    );
    assert_eq!(PaymentReconciliationAllocation::MODULE, "Accounts");
    assert!(PaymentReconciliationAllocation::IS_TABLE);
    assert!(PaymentReconciliationAllocation::IS_VIRTUAL);
    assert_eq!(PaymentReconciliationAllocation::ROW_FORMAT, "Dynamic");
    assert_eq!(PaymentReconciliationAllocation::SORT_FIELD, "creation");
    assert_eq!(PaymentReconciliationAllocation::SORT_ORDER, "DESC");
    assert!(PaymentReconciliationAllocation::TRACK_CHANGES);
    assert_eq!(
        PaymentReconciliationAllocation::FIELD_ORDER,
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
            "debit_or_credit_note_posting_date",
            "column_break_7",
            "difference_account",
            "exchange_rate",
            "currency",
            "accounting_dimensions_section",
            "cost_center",
            "dimension_col_break",
        ]
    );
    assert_eq!(
        PaymentReconciliationAllocation::fields(),
        vec![
            FieldSpec::dynamic_link("invoice_number")
                .label("Invoice Number")
                .options("invoice_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::currency("allocated_amount", "Allocated Amount")
                .options("currency")
                .required()
                .in_list_view(),
            FieldSpec::column_break("column_break_3"),
            FieldSpec::section_break("section_break_5"),
            FieldSpec::link("difference_account", "Difference Account")
                .options("Account")
                .read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::currency("difference_amount", "Difference Amount")
                .options("Currency")
                .read_only()
                .in_list_view(),
            FieldSpec::dynamic_link("reference_name")
                .label("Reference Name")
                .options("reference_type")
                .required()
                .read_only()
                .in_list_view(),
            FieldSpec::data("is_advance", "Is Advance")
                .read_only()
                .hidden(),
            FieldSpec::link("reference_type", "Reference Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::link("invoice_type", "Invoice Type")
                .options("DocType")
                .required()
                .read_only(),
            FieldSpec::section_break("section_break_6"),
            FieldSpec::column_break("column_break_8"),
            FieldSpec::currency("unreconciled_amount", "Unreconciled Amount")
                .options("currency")
                .read_only()
                .hidden(),
            FieldSpec::currency("amount", "Amount")
                .options("currency")
                .read_only()
                .hidden(),
            FieldSpec::data("reference_row", "Reference Row")
                .read_only()
                .hidden(),
            FieldSpec::link("currency", "Currency")
                .options("Currency")
                .hidden(),
            FieldSpec::float("exchange_rate", "Exchange Rate").read_only(),
            FieldSpec::link("cost_center", "Cost Center").options("Cost Center"),
            FieldSpec::date("gain_loss_posting_date", "Difference Posting Date"),
            FieldSpec::section_break("accounting_dimensions_section")
                .label("Accounting Dimensions"),
            FieldSpec::column_break("dimension_col_break"),
            FieldSpec::date(
                "debit_or_credit_note_posting_date",
                "Debit / Credit Note Posting Date",
            ),
        ]
    );
    assert_eq!(PaymentReconciliationAllocation::get_list("ignored"), ());
    let allocation = PaymentReconciliationAllocation;
    assert_eq!(allocation.doctype(), "Payment Reconciliation Allocation");
    assert_eq!(allocation.module(), "Accounts");
}
