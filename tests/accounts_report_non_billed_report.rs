use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::report::non_billed_report::{
    get_ordered_to_be_billed_data, get_project_field, NonBilledArgs, NonBilledDocument,
    NonBilledFilters, NonBilledItem, NonBilledItemMaster, NonBilledQueryPlan,
};

fn default_args() -> NonBilledArgs {
    NonBilledArgs {
        doctype: "Sales Order".to_string(),
        party: "customer".to_string(),
        date: "transaction_date".to_string(),
        reference_field: "sales_order".to_string(),
        order: "transaction_date".to_string(),
        order_by: "asc".to_string(),
    }
}

fn filters() -> NonBilledFilters {
    NonBilledFilters {
        company: "Wind Power LLC".to_string(),
        posting_date: "2026-05-24".to_string(),
        references: BTreeMap::new(),
    }
}

fn document(name: &str, date: &str) -> NonBilledDocument {
    let mut date_fields = BTreeMap::new();
    date_fields.insert("transaction_date".to_string(), date.to_string());

    NonBilledDocument {
        name: name.to_string(),
        date_fields,
        party: "CUST-001".to_string(),
        party_name: "Northwind".to_string(),
        status: "To Bill".to_string(),
        docstatus: 1,
        company: "Wind Power LLC".to_string(),
        posting_date: date.to_string(),
        conversion_rate: Some(2.0),
        project: Some("PARENT-PROJECT".to_string()),
    }
}

fn child(parent: &str, item_code: &str) -> NonBilledItem {
    NonBilledItem {
        parent: parent.to_string(),
        item_code: item_code.to_string(),
        base_amount: 100.0,
        billed_amt: 10.125,
        base_rate: 5.0,
        returned_qty: Some(2.0),
        amount: 50.0,
        item_name: "Rotor".to_string(),
        description: "Turbine rotor".to_string(),
        project: Some("CHILD-PROJECT".to_string()),
    }
}

fn stock_item(name: &str, is_stock_item: bool) -> NonBilledItemMaster {
    NonBilledItemMaster {
        name: name.to_string(),
        is_stock_item,
    }
}

#[test]
fn non_billed_report_query_plan_matches_erpnext_builder_shape() {
    let args = default_args();
    let mut report_filters = filters();
    report_filters
        .references
        .insert("sales_order".to_string(), "SO-0002".to_string());

    let plan = NonBilledQueryPlan::from_args(&args, &report_filters, Some(3));

    assert_eq!(plan.doctype, "Sales Order");
    assert_eq!(plan.child_tab, "Sales Order Item");
    assert_eq!(plan.party_field, "customer");
    assert_eq!(plan.party_name_field, "customer_name");
    assert_eq!(plan.date_field, "transaction_date");
    assert_eq!(plan.reference_field, "sales_order");
    assert_eq!(plan.docname_filter.as_deref(), Some("SO-0002"));
    assert_eq!(plan.project_source, "doctype.project");
    assert_eq!(plan.order_field, "transaction_date");
    assert_eq!(plan.order_by, "asc");
    assert_eq!(plan.precision, 3);

    assert_eq!(
        plan.joins,
        [
            "Sales Order.name = Sales Order Item.parent",
            "Item.name = Sales Order Item.item_code"
        ]
    );
    assert_eq!(
        plan.filters,
        [
            "docstatus = 1",
            "status not in Closed,Completed",
            "company = filters.company",
            "posting_date <= filters.posting_date",
            "child.amount > 0",
            "item.is_stock_item = 1",
            "base_amount - Round(billed_amt * IfNull(conversion_rate, 1), precision) - base_rate * IfNull(returned_qty, 0) > 0",
            "name = filters[reference_field]"
        ]
    );
}

#[test]
fn non_billed_report_project_field_uses_child_project_only_for_supplier() {
    assert_eq!(get_project_field("supplier"), "child_doctype.project");
    assert_eq!(get_project_field("customer"), "doctype.project");
}

#[test]
fn non_billed_report_filters_and_calculates_rows_like_erpnext() {
    let args = default_args();
    let report_filters = filters();
    let documents = vec![
        document("SO-0001", "2026-05-22"),
        NonBilledDocument {
            status: "Closed".to_string(),
            ..document("SO-CLOSED", "2026-05-21")
        },
        NonBilledDocument {
            docstatus: 0,
            ..document("SO-DRAFT", "2026-05-21")
        },
        NonBilledDocument {
            company: "Other Company".to_string(),
            ..document("SO-OTHER", "2026-05-21")
        },
        NonBilledDocument {
            posting_date: "2026-05-25".to_string(),
            ..document("SO-FUTURE", "2026-05-25")
        },
    ];
    let items = vec![
        child("SO-0001", "ITEM-001"),
        NonBilledItem {
            amount: 0.0,
            ..child("SO-0001", "ITEM-ZERO")
        },
        NonBilledItem {
            base_amount: 20.25,
            billed_amt: 10.126,
            returned_qty: None,
            ..child("SO-0001", "ITEM-FULL")
        },
        child("SO-CLOSED", "ITEM-CLOSED"),
        child("SO-DRAFT", "ITEM-DRAFT"),
        child("SO-OTHER", "ITEM-OTHER"),
        child("SO-FUTURE", "ITEM-FUTURE"),
        child("SO-0001", "ITEM-NON-STOCK"),
    ];
    let item_masters = vec![
        stock_item("ITEM-001", true),
        stock_item("ITEM-ZERO", true),
        stock_item("ITEM-FULL", true),
        stock_item("ITEM-CLOSED", true),
        stock_item("ITEM-DRAFT", true),
        stock_item("ITEM-OTHER", true),
        stock_item("ITEM-FUTURE", true),
        stock_item("ITEM-NON-STOCK", false),
    ];

    let rows = get_ordered_to_be_billed_data(
        &args,
        &report_filters,
        &documents,
        &items,
        &item_masters,
        Some(2),
    );

    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.name, "SO-0001");
    assert_eq!(row.date, "2026-05-22");
    assert_eq!(row.party, "CUST-001");
    assert_eq!(row.party_name, "Northwind");
    assert_eq!(row.item_code, "ITEM-001");
    assert_eq!(row.amount, 100.0);
    assert_eq!(row.billed_amount, 20.25);
    assert_eq!(row.returned_amount, 10.0);
    assert_eq!(row.pending_amount, 69.75);
    assert_eq!(row.item_name, "Rotor");
    assert_eq!(row.description, "Turbine rotor");
    assert_eq!(row.project.as_deref(), Some("PARENT-PROJECT"));
    assert_eq!(row.company, "Wind Power LLC");
}

#[test]
fn non_billed_report_uses_reference_filter_default_precision_and_supplier_project() {
    let mut args = default_args();
    args.doctype = "Purchase Order".to_string();
    args.party = "supplier".to_string();
    args.reference_field = "purchase_order".to_string();
    args.order_by = "desc".to_string();

    let mut report_filters = filters();
    report_filters
        .references
        .insert("purchase_order".to_string(), "PO-0002".to_string());

    let mut po1 = document("PO-0001", "2026-05-20");
    po1.conversion_rate = None;
    po1.project = Some("PARENT-PO-1".to_string());
    po1.party = "SUP-001".to_string();
    po1.party_name = "Steel Supplier".to_string();

    let mut po2 = document("PO-0002", "2026-05-21");
    po2.conversion_rate = None;
    po2.project = Some("PARENT-PO-2".to_string());
    po2.party = "SUP-002".to_string();
    po2.party_name = "Bearing Supplier".to_string();

    let mut po2_child = child("PO-0002", "ITEM-002");
    po2_child.base_amount = 50.005;
    po2_child.billed_amt = 10.004;
    po2_child.returned_qty = None;
    po2_child.project = Some("CHILD-PO-2".to_string());

    let rows = get_ordered_to_be_billed_data(
        &args,
        &report_filters,
        &[po1, po2],
        &[child("PO-0001", "ITEM-001"), po2_child],
        &[stock_item("ITEM-001", true), stock_item("ITEM-002", true)],
        None,
    );

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "PO-0002");
    assert_eq!(rows[0].party, "SUP-002");
    assert_eq!(rows[0].project.as_deref(), Some("CHILD-PO-2"));
    assert_eq!(rows[0].billed_amount, 10.004);
    assert!((rows[0].pending_amount - 40.001).abs() < 1e-9);
}
