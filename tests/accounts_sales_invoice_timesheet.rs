use tokio_erp::erpnext::accounts::doctype::sales_invoice_timesheet::sales_invoice_timesheet::SalesInvoiceTimesheet;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn sales_invoice_timesheet_matches_erpnext_metadata() {
    assert_eq!(SalesInvoiceTimesheet::DOCTYPE, "Sales Invoice Timesheet");
    assert_eq!(SalesInvoiceTimesheet::MODULE, "Accounts");
    assert_eq!(SalesInvoiceTimesheet::FIELD_ORDER.len(), 15);
    assert_eq!(SalesInvoiceTimesheet::FIELD_ORDER[0], "activity_type");
    assert_eq!(SalesInvoiceTimesheet::FIELD_ORDER[14], "project_name");
    assert!(SalesInvoiceTimesheet::IS_TABLE);
    assert!(SalesInvoiceTimesheet::EDITABLE_GRID);
    assert!(SalesInvoiceTimesheet::QUICK_ENTRY);
    assert!(SalesInvoiceTimesheet::TRACK_CHANGES);

    let fields = SalesInvoiceTimesheet::fields();
    assert_eq!(fields.len(), 15);
    assert!(fields.contains(
        &FieldSpec::link("time_sheet", "Time Sheet")
            .options("Timesheet")
            .read_only()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::float("billing_hours", "Billing Hours")
            .read_only()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::currency("billing_amount", "Billing Amount")
            .options("currency")
            .read_only()
            .in_list_view()
    ));
    assert!(fields.contains(&FieldSpec::datetime("from_time", "From Time")));
    assert!(fields.contains(&FieldSpec::datetime("to_time", "To Time")));
    assert!(fields.contains(
        &FieldSpec::data("timesheet_detail", "Timesheet Detail")
            .read_only()
            .hidden()
            .print_hide()
            .allow_on_submit()
    ));
}

#[test]
fn sales_invoice_timesheet_controller_is_pass_through() {
    let row = SalesInvoiceTimesheet::new("TS-0001", "TS-DETAIL-0001");
    assert_eq!(row.time_sheet.as_deref(), Some("TS-0001"));
    assert_eq!(row.timesheet_detail.as_deref(), Some("TS-DETAIL-0001"));
    assert_eq!(row.doctype(), "Sales Invoice Timesheet");
    assert_eq!(row.module(), "Accounts");
    assert!(row.custom_hooks().is_empty());
}
