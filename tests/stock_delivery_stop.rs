use tokio_erp::erpnext::stock::doctype::delivery_stop::delivery_stop::DeliveryStop;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn delivery_stop_matches_erpnext_metadata_and_fields() {
    assert_eq!(DeliveryStop::DOCTYPE, "Delivery Stop");
    assert_eq!(DeliveryStop::MODULE, "Stock");
    assert_eq!(
        DeliveryStop::FIELD_ORDER,
        [
            "customer",
            "address",
            "locked",
            "column_break_6",
            "customer_address",
            "visited",
            "order_information_section",
            "delivery_note",
            "cb_order",
            "grand_total",
            "section_break_7",
            "contact",
            "email_sent_to",
            "column_break_7",
            "customer_contact",
            "section_break_9",
            "distance",
            "estimated_arrival",
            "lat",
            "column_break_19",
            "uom",
            "lng",
            "more_information_section",
            "details",
        ]
    );
    assert!(DeliveryStop::EDITABLE_GRID);
    assert!(DeliveryStop::IS_TABLE);
    assert!(DeliveryStop::QUICK_ENTRY);
    assert_eq!(DeliveryStop::SORT_FIELD, "creation");
    assert_eq!(DeliveryStop::SORT_ORDER, "DESC");
    assert!(DeliveryStop::TRACK_CHANGES);

    assert_eq!(
        DeliveryStop::fields(),
        vec![
            FieldSpec::link("customer", "Customer")
                .options("Customer")
                .in_list_view()
                .columns(2),
            FieldSpec::link("address", "Address Name")
                .options("Address")
                .in_list_view()
                .print_hide()
                .required(),
            FieldSpec::check("locked", "Locked")
                .default("0")
                .in_list_view(),
            FieldSpec::column_break("column_break_6"),
            FieldSpec::small_text("customer_address", "Customer Address").read_only(),
            FieldSpec::check("visited", "Visited")
                .default("0")
                .allow_on_submit()
                .depends_on("eval:doc.docstatus==1")
                .in_list_view()
                .no_copy()
                .print_hide(),
            FieldSpec::section_break("order_information_section").label("Order Information"),
            FieldSpec::link("delivery_note", "Delivery Note")
                .options("Delivery Note")
                .in_list_view()
                .no_copy()
                .print_hide()
                .read_only(),
            FieldSpec::column_break("cb_order"),
            FieldSpec::currency("grand_total", "Grand Total").read_only(),
            FieldSpec::section_break("section_break_7").label("Contact Information"),
            FieldSpec::link("contact", "Contact Name")
                .options("Contact")
                .print_hide(),
            FieldSpec::data("email_sent_to", "Email sent to").read_only(),
            FieldSpec::column_break("column_break_7"),
            FieldSpec::small_text("customer_contact", "Customer Contact").read_only(),
            FieldSpec::section_break("section_break_9").label("Dispatch Information"),
            FieldSpec::float("distance", "Distance")
                .precision("2")
                .read_only(),
            FieldSpec::datetime("estimated_arrival", "Estimated Arrival").in_list_view(),
            FieldSpec::float("lat", "Latitude").hidden(),
            FieldSpec::column_break("column_break_19"),
            FieldSpec::link("uom", "UOM")
                .options("UOM")
                .depends_on("eval:doc.distance")
                .read_only(),
            FieldSpec::float("lng", "Longitude").hidden(),
            FieldSpec::section_break("more_information_section").label("More Information"),
            FieldSpec::text_editor("details", "Details"),
        ]
    );
}

#[test]
fn delivery_stop_preserves_pass_controller_behavior() {
    let stop = DeliveryStop {
        customer: Some("Wind Power LLC".to_owned()),
        address: Some("Wind Power Billing".to_owned()),
        locked: true,
        customer_address: Some("221B Baker Street".to_owned()),
        visited: true,
        delivery_note: Some("DN-0001".to_owned()),
        grand_total: Some(1250.75),
        contact: Some("Jane Smith".to_owned()),
        email_sent_to: Some("dispatch@example.com".to_owned()),
        customer_contact: Some("+998 90 000 00 00".to_owned()),
        distance: Some(42.5),
        estimated_arrival: Some("2026-06-08 13:30:00".to_owned()),
        lat: Some(41.2995),
        uom: Some("Kilometer".to_owned()),
        lng: Some(69.2401),
        details: Some("Leave at dock 4".to_owned()),
    };

    assert_eq!(stop.customer.as_deref(), Some("Wind Power LLC"));
    assert_eq!(stop.address.as_deref(), Some("Wind Power Billing"));
    assert!(stop.locked);
    assert_eq!(stop.customer_address.as_deref(), Some("221B Baker Street"));
    assert!(stop.visited);
    assert_eq!(stop.delivery_note.as_deref(), Some("DN-0001"));
    assert_eq!(stop.grand_total, Some(1250.75));
    assert_eq!(stop.contact.as_deref(), Some("Jane Smith"));
    assert_eq!(stop.email_sent_to.as_deref(), Some("dispatch@example.com"));
    assert_eq!(stop.customer_contact.as_deref(), Some("+998 90 000 00 00"));
    assert_eq!(stop.distance, Some(42.5));
    assert_eq!(
        stop.estimated_arrival.as_deref(),
        Some("2026-06-08 13:30:00")
    );
    assert_eq!(stop.lat, Some(41.2995));
    assert_eq!(stop.uom.as_deref(), Some("Kilometer"));
    assert_eq!(stop.lng, Some(69.2401));
    assert_eq!(stop.details.as_deref(), Some("Leave at dock 4"));
    assert_eq!(stop.doctype(), "Delivery Stop");
    assert_eq!(stop.module(), "Stock");
    assert!(stop.custom_hooks().is_empty());
}

#[test]
fn delivery_stop_defaults_match_empty_document_state() {
    let stop = DeliveryStop::default();

    assert_eq!(stop.customer, None);
    assert_eq!(stop.address, None);
    assert!(!stop.locked);
    assert!(!stop.visited);
    assert_eq!(stop.grand_total, None);
    assert_eq!(stop.distance, None);
}
