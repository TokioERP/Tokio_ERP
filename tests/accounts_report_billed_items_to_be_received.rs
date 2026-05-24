use tokio_erp::erpnext::accounts::report::billed_items_to_be_received::billed_items_to_be_received::{
    execute, get_columns, get_report_fields, get_report_filters, BilledItemsToBeReceivedFilters,
    BilledItemsToBeReceivedRow, PurchaseInvoice, PurchaseInvoiceItem, ReportColumn, ReportFilter,
    ReportField,
};

fn filters() -> BilledItemsToBeReceivedFilters {
    BilledItemsToBeReceivedFilters {
        company: "_Test Company".to_string(),
        posting_date: "2026-05-24".to_string(),
        purchase_invoice: None,
    }
}

fn invoice(name: &str, per_received: f64) -> PurchaseInvoice {
    PurchaseInvoice {
        name: name.to_string(),
        supplier: "SUP-001".to_string(),
        company: "_Test Company".to_string(),
        posting_date: "2026-05-23".to_string(),
        currency: "USD".to_string(),
        docstatus: 1,
        per_received,
        update_stock: false,
        is_opening: "No".to_string(),
    }
}

fn item(parent: &str, item_code: &str) -> PurchaseInvoiceItem {
    PurchaseInvoiceItem {
        parent: parent.to_string(),
        item_code: item_code.to_string(),
        item_name: format!("{item_code} name"),
        uom: "Nos".to_string(),
        qty: 10.0,
        received_qty: 4.0,
        rate: 25.0,
        amount: 250.0,
    }
}

#[test]
fn billed_items_to_be_received_columns_match_erpnext_report_shape() {
    assert_eq!(
        get_columns(),
        vec![
            ReportColumn::link("Purchase Invoice", "name", "Purchase Invoice", 170),
            ReportColumn::link("Supplier", "supplier", "Supplier", 120),
            ReportColumn::date("Posting Date", "posting_date", 100),
            ReportColumn::link("Item Code", "item_code", "Item", 100),
            ReportColumn::data("Item Name", "item_name", 100),
            ReportColumn::link("UOM", "uom", "UOM", 100),
            ReportColumn::float("Invoiced Qty", "qty", 100),
            ReportColumn::float("Received Qty", "received_qty", 100),
            ReportColumn::currency("Rate", "rate", 100),
            ReportColumn::currency("Amount", "amount", 100),
        ]
    );
}

#[test]
fn billed_items_to_be_received_report_fields_match_erpnext_parent_and_child_fields() {
    assert_eq!(
        get_report_fields(),
        vec![
            ReportField::new("Purchase Invoice", "name"),
            ReportField::new("Purchase Invoice", "supplier"),
            ReportField::new("Purchase Invoice", "company"),
            ReportField::new("Purchase Invoice", "posting_date"),
            ReportField::new("Purchase Invoice", "currency"),
            ReportField::new("Purchase Invoice Item", "item_code"),
            ReportField::new("Purchase Invoice Item", "item_name"),
            ReportField::new("Purchase Invoice Item", "uom"),
            ReportField::new("Purchase Invoice Item", "qty"),
            ReportField::new("Purchase Invoice Item", "received_qty"),
            ReportField::new("Purchase Invoice Item", "rate"),
            ReportField::new("Purchase Invoice Item", "amount"),
        ]
    );
}

#[test]
fn billed_items_to_be_received_report_filters_match_erpnext_base_filters() {
    assert_eq!(
        get_report_filters(&filters()),
        vec![
            ReportFilter::new("Purchase Invoice", "company", "=", "_Test Company"),
            ReportFilter::new("Purchase Invoice", "posting_date", "<=", "2026-05-24"),
            ReportFilter::new("Purchase Invoice", "docstatus", "=", "1"),
            ReportFilter::new("Purchase Invoice", "per_received", "<", "100"),
            ReportFilter::new("Purchase Invoice", "update_stock", "=", "0"),
            ReportFilter::new("Purchase Invoice", "is_opening", "!=", "Yes"),
        ]
    );
}

#[test]
fn billed_items_to_be_received_purchase_invoice_branch_preserves_erpnext_per_received_filter() {
    let report_filters = BilledItemsToBeReceivedFilters {
        purchase_invoice: Some("PINV-0001".to_string()),
        ..filters()
    };

    assert_eq!(
        get_report_filters(&report_filters),
        vec![
            ReportFilter::new("Purchase Invoice", "company", "=", "_Test Company"),
            ReportFilter::new("Purchase Invoice", "posting_date", "<=", "2026-05-24"),
            ReportFilter::new("Purchase Invoice", "docstatus", "=", "1"),
            ReportFilter::new("Purchase Invoice", "per_received", "<", "100"),
            ReportFilter::new("Purchase Invoice", "update_stock", "=", "0"),
            ReportFilter::new("Purchase Invoice", "is_opening", "!=", "Yes"),
            ReportFilter::list("Purchase Invoice", "per_received", "in", vec!["PINV-0001"]),
        ]
    );
}

#[test]
fn billed_items_to_be_received_execute_returns_child_rows_for_matching_purchase_invoices() {
    let mut wrong_company = invoice("PINV-WRONG-COMPANY", 25.0);
    wrong_company.company = "Other Company".to_string();
    let mut future_invoice = invoice("PINV-FUTURE", 25.0);
    future_invoice.posting_date = "2026-05-25".to_string();
    let mut completed_invoice = invoice("PINV-COMPLETE", 100.0);
    completed_invoice.per_received = 100.0;
    let mut stock_invoice = invoice("PINV-STOCK", 25.0);
    stock_invoice.update_stock = true;
    let mut opening_invoice = invoice("PINV-OPENING", 25.0);
    opening_invoice.is_opening = "Yes".to_string();
    let mut draft_invoice = invoice("PINV-DRAFT", 25.0);
    draft_invoice.docstatus = 0;

    let report = execute(
        &filters(),
        &[
            invoice("PINV-0002", 50.0),
            invoice("PINV-0001", 25.0),
            wrong_company,
            future_invoice,
            completed_invoice,
            stock_invoice,
            opening_invoice,
            draft_invoice,
        ],
        &[
            item("PINV-0001", "ITEM-001"),
            item("PINV-0001", "ITEM-002"),
            item("PINV-0002", "ITEM-003"),
            item("PINV-WRONG-COMPANY", "ITEM-004"),
        ],
    );

    assert_eq!(report.columns, get_columns());
    assert_eq!(
        report.rows,
        vec![
            BilledItemsToBeReceivedRow {
                name: "PINV-0002".to_string(),
                supplier: "SUP-001".to_string(),
                company: "_Test Company".to_string(),
                posting_date: "2026-05-23".to_string(),
                currency: "USD".to_string(),
                item_code: "ITEM-003".to_string(),
                item_name: "ITEM-003 name".to_string(),
                uom: "Nos".to_string(),
                qty: 10.0,
                received_qty: 4.0,
                rate: 25.0,
                amount: 250.0,
            },
            BilledItemsToBeReceivedRow {
                name: "PINV-0001".to_string(),
                supplier: "SUP-001".to_string(),
                company: "_Test Company".to_string(),
                posting_date: "2026-05-23".to_string(),
                currency: "USD".to_string(),
                item_code: "ITEM-001".to_string(),
                item_name: "ITEM-001 name".to_string(),
                uom: "Nos".to_string(),
                qty: 10.0,
                received_qty: 4.0,
                rate: 25.0,
                amount: 250.0,
            },
            BilledItemsToBeReceivedRow {
                name: "PINV-0001".to_string(),
                supplier: "SUP-001".to_string(),
                company: "_Test Company".to_string(),
                posting_date: "2026-05-23".to_string(),
                currency: "USD".to_string(),
                item_code: "ITEM-002".to_string(),
                item_name: "ITEM-002 name".to_string(),
                uom: "Nos".to_string(),
                qty: 10.0,
                received_qty: 4.0,
                rate: 25.0,
                amount: 250.0,
            },
        ]
    );
}
