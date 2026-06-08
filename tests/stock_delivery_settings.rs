use tokio_erp::erpnext::stock::doctype::delivery_settings::delivery_settings::DeliverySettings;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn delivery_settings_matches_erpnext_metadata_and_fields() {
    assert_eq!(DeliverySettings::DOCTYPE, "Delivery Settings");
    assert_eq!(DeliverySettings::MODULE, "Stock");
    assert_eq!(
        DeliverySettings::FIELD_ORDER,
        [
            "sb_dispatch",
            "dispatch_template",
            "dispatch_attachment",
            "send_with_attachment",
            "cb_delivery",
            "stop_delay",
        ]
    );
    assert!(DeliverySettings::EDITABLE_GRID);
    assert_eq!(DeliverySettings::GRID_PAGE_LENGTH, 50);
    assert!(!DeliverySettings::HIDE_TOOLBAR);
    assert!(DeliverySettings::IS_SINGLE);
    assert!(DeliverySettings::QUICK_ENTRY);
    assert_eq!(DeliverySettings::ROW_FORMAT, "Dynamic");
    assert_eq!(DeliverySettings::SORT_FIELD, "creation");
    assert_eq!(DeliverySettings::SORT_ORDER, "DESC");
    assert!(DeliverySettings::TRACK_CHANGES);

    assert_eq!(
        DeliverySettings::fields(),
        vec![
            FieldSpec::section_break("sb_dispatch").label("Dispatch Settings"),
            FieldSpec::link("dispatch_template", "Dispatch Notification Template")
                .options("Email Template"),
            FieldSpec::link("dispatch_attachment", "Dispatch Notification Attachment")
                .options("Print Format")
                .depends_on("send_with_attachment")
                .description("Leave blank to use the standard Delivery Note format"),
            FieldSpec::check("send_with_attachment", "Send with Attachment").default("0"),
            FieldSpec::column_break("cb_delivery"),
            FieldSpec::int("stop_delay", "Delay between Delivery Stops").description("In minutes"),
        ]
    );
}

#[test]
fn delivery_settings_preserves_pass_controller_behavior() {
    let settings = DeliverySettings::new(
        Some("Dispatch Email"),
        Some("Delivery Note Standard"),
        true,
        15,
    );

    assert_eq!(
        settings.dispatch_template.as_deref(),
        Some("Dispatch Email")
    );
    assert_eq!(
        settings.dispatch_attachment.as_deref(),
        Some("Delivery Note Standard")
    );
    assert!(settings.send_with_attachment);
    assert_eq!(settings.stop_delay, 15);
    assert_eq!(settings.doctype(), "Delivery Settings");
    assert_eq!(settings.module(), "Stock");
    assert!(settings.custom_hooks().is_empty());
}
