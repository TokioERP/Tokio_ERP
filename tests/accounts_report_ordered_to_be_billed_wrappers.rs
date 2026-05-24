use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::delivered_items_to_be_billed::delivered_items_to_be_billed as delivered;
use tokio_erp::erpnext::accounts::report::non_billed_report::{
    NonBilledDocument, NonBilledFilters, NonBilledItem, NonBilledItemMaster,
};
use tokio_erp::erpnext::accounts::report::received_items_to_be_billed::received_items_to_be_billed as received;

fn filters() -> NonBilledFilters {
    NonBilledFilters {
        company: "_Test Company".to_string(),
        posting_date: "2026-05-24".to_string(),
        references: BTreeMap::new(),
    }
}

fn document(name: &str, party: &str, party_name: &str) -> NonBilledDocument {
    let mut date_fields = BTreeMap::new();
    date_fields.insert("posting_date".to_string(), "2026-05-23".to_string());

    NonBilledDocument {
        name: name.to_string(),
        date_fields,
        party: party.to_string(),
        party_name: party_name.to_string(),
        status: "To Bill".to_string(),
        docstatus: 1,
        company: "_Test Company".to_string(),
        posting_date: "2026-05-23".to_string(),
        conversion_rate: Some(1.0),
        project: Some("PARENT-PROJECT".to_string()),
    }
}

fn child(parent: &str) -> NonBilledItem {
    NonBilledItem {
        parent: parent.to_string(),
        item_code: "ITEM-001".to_string(),
        base_amount: 120.0,
        billed_amt: 20.0,
        base_rate: 10.0,
        returned_qty: Some(1.0),
        amount: 120.0,
        item_name: "Service Item".to_string(),
        description: "Service description".to_string(),
        project: Some("CHILD-PROJECT".to_string()),
    }
}

fn item_master() -> NonBilledItemMaster {
    NonBilledItemMaster {
        name: "ITEM-001".to_string(),
        is_stock_item: true,
    }
}

#[test]
fn delivered_items_to_be_billed_columns_and_args_match_erpnext() {
    assert_eq!(
        delivered::get_column(),
        vec![
            delivered::ReportColumn::link("Delivery Note", "name", "Delivery Note", 160),
            delivered::ReportColumn::date("Date", "date", 100),
            delivered::ReportColumn::link("Customer", "customer", "Customer", 120),
            delivered::ReportColumn::data("Customer Name", "customer_name", 120),
            delivered::ReportColumn::link("Item Code", "item_code", "Item", 120),
            delivered::ReportColumn::currency(
                "Amount",
                "amount",
                "Company:company:default_currency",
                100,
            ),
            delivered::ReportColumn::currency(
                "Billed Amount",
                "billed_amount",
                "Company:company:default_currency",
                100,
            ),
            delivered::ReportColumn::currency(
                "Returned Amount",
                "returned_amount",
                "Company:company:default_currency",
                120,
            ),
            delivered::ReportColumn::currency(
                "Pending Amount",
                "pending_amount",
                "Company:company:default_currency",
                120,
            ),
            delivered::ReportColumn::data("Item Name", "item_name", 120),
            delivered::ReportColumn::data("Description", "description", 120),
            delivered::ReportColumn::link("Project", "project", "Project", 120),
        ]
    );

    let args = delivered::get_args();
    assert_eq!(args.doctype, "Delivery Note");
    assert_eq!(args.party, "customer");
    assert_eq!(args.date, "posting_date");
    assert_eq!(args.order, "name");
    assert_eq!(args.order_by, "desc");
    assert_eq!(args.reference_field, "delivery_note");
}

#[test]
fn delivered_items_to_be_billed_execute_delegates_to_non_billed_helper() {
    let report = delivered::execute(
        &filters(),
        &[document("DN-0001", "CUST-001", "Customer One")],
        &[child("DN-0001")],
        &[item_master()],
        Some(2),
    );

    assert_eq!(report.columns, delivered::get_column());
    assert_eq!(report.rows.len(), 1);
    assert_eq!(report.rows[0].name, "DN-0001");
    assert_eq!(report.rows[0].party, "CUST-001");
    assert_eq!(report.rows[0].party_name, "Customer One");
    assert_eq!(report.rows[0].project.as_deref(), Some("PARENT-PROJECT"));
    assert_eq!(report.rows[0].pending_amount, 90.0);
}

#[test]
fn received_items_to_be_billed_columns_and_args_match_erpnext() {
    assert_eq!(
        received::get_column(),
        vec![
            received::ReportColumn::link("Purchase Receipt", "name", "Purchase Receipt", 160),
            received::ReportColumn::date("Date", "date", 100),
            received::ReportColumn::link("Supplier", "supplier", "Supplier", 120),
            received::ReportColumn::data("Supplier Name", "supplier_name", 120),
            received::ReportColumn::link("Item Code", "item_code", "Item", 120),
            received::ReportColumn::currency(
                "Amount",
                "amount",
                "Company:company:default_currency",
                100,
            ),
            received::ReportColumn::currency(
                "Billed Amount",
                "billed_amount",
                "Company:company:default_currency",
                100,
            ),
            received::ReportColumn::currency(
                "Returned Amount",
                "returned_amount",
                "Company:company:default_currency",
                120,
            ),
            received::ReportColumn::currency(
                "Pending Amount",
                "pending_amount",
                "Company:company:default_currency",
                120,
            ),
            received::ReportColumn::data("Item Name", "item_name", 120),
            received::ReportColumn::data("Description", "description", 120),
            received::ReportColumn::link("Project", "project", "Project", 120),
        ]
    );

    let args = received::get_args();
    assert_eq!(args.doctype, "Purchase Receipt");
    assert_eq!(args.party, "supplier");
    assert_eq!(args.date, "posting_date");
    assert_eq!(args.order, "name");
    assert_eq!(args.order_by, "desc");
    assert_eq!(args.reference_field, "purchase_receipt");
}

#[test]
fn received_items_to_be_billed_execute_uses_supplier_child_project_branch() {
    let report = received::execute(
        &filters(),
        &[document("PR-0001", "SUP-001", "Supplier One")],
        &[child("PR-0001")],
        &[item_master()],
        Some(2),
    );

    assert_eq!(report.columns, received::get_column());
    assert_eq!(report.rows.len(), 1);
    assert_eq!(report.rows[0].name, "PR-0001");
    assert_eq!(report.rows[0].party, "SUP-001");
    assert_eq!(report.rows[0].party_name, "Supplier One");
    assert_eq!(report.rows[0].project.as_deref(), Some("CHILD-PROJECT"));
    assert_eq!(report.rows[0].pending_amount, 90.0);
}
