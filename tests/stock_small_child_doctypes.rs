use tokio_erp::erpnext::stock::doctype::item_customer_detail::item_customer_detail::ItemCustomerDetail;
use tokio_erp::erpnext::stock::doctype::item_variant::item_variant::ItemVariant;
use tokio_erp::erpnext::stock::doctype::item_website_specification::item_website_specification::ItemWebsiteSpecification;
use tokio_erp::erpnext::stock::doctype::landed_cost_vendor_invoice::landed_cost_vendor_invoice::LandedCostVendorInvoice;
use tokio_erp::erpnext::stock::doctype::shipment_delivery_note::shipment_delivery_note::ShipmentDeliveryNote;
use tokio_erp::erpnext::stock::doctype::shipment_parcel::shipment_parcel::ShipmentParcel;
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn item_customer_detail_matches_erpnext_metadata_and_fields() {
    assert_eq!(ItemCustomerDetail::DOCTYPE, "Item Customer Detail");
    assert_eq!(ItemCustomerDetail::MODULE, "Stock");
    assert_eq!(
        ItemCustomerDetail::FIELD_ORDER,
        ["customer_name", "customer_group", "ref_code"]
    );
    assert!(ItemCustomerDetail::EDITABLE_GRID);
    assert!(ItemCustomerDetail::IS_TABLE);
    assert_eq!(ItemCustomerDetail::SORT_FIELD, "creation");
    assert_eq!(ItemCustomerDetail::SORT_ORDER, "DESC");

    assert_eq!(
        ItemCustomerDetail::fields(),
        vec![
            FieldSpec::link("customer_name", "Customer Name")
                .options("Customer")
                .oldfield("price_list_name", "Select")
                .in_filter()
                .in_list_view()
                .search_index()
                .bold()
                .width("180px"),
            FieldSpec::link("customer_group", "Customer Group")
                .options("Customer Group")
                .in_filter()
                .in_list_view()
                .bold(),
            FieldSpec::data("ref_code", "Ref Code")
                .description("Enter the Item Code that this customer uses at their end. This will be shown in Sales Orders for the customer's reference.")
                .oldfield("ref_rate", "Currency")
                .in_filter()
                .in_list_view()
                .required()
                .search_index()
                .width("120px"),
        ]
    );

    let row = ItemCustomerDetail::new(
        Some("Acme Retail"),
        Some("Retail"),
        "ACME-ITEM",
        Some("ERP Item"),
        Some("customer_items"),
        Some("Item"),
    );
    assert_eq!(row.customer_name.as_deref(), Some("Acme Retail"));
    assert_eq!(row.customer_group.as_deref(), Some("Retail"));
    assert_eq!(row.ref_code.as_deref(), Some("ACME-ITEM"));
    assert_eq!(row.doctype(), "Item Customer Detail");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn item_variant_matches_erpnext_metadata_and_fields() {
    assert_eq!(ItemVariant::DOCTYPE, "Item Variant");
    assert_eq!(ItemVariant::MODULE, "Stock");
    assert_eq!(
        ItemVariant::FIELD_ORDER,
        ["item_attribute", "item_attribute_value"]
    );
    assert!(ItemVariant::EDITABLE_GRID);
    assert!(ItemVariant::IS_TABLE);
    assert_eq!(ItemVariant::SORT_FIELD, "creation");
    assert_eq!(ItemVariant::SORT_ORDER, "DESC");

    assert_eq!(
        ItemVariant::fields(),
        vec![
            FieldSpec::link("item_attribute", "Item Attribute")
                .options("Item Attribute")
                .in_list_view()
                .required(),
            FieldSpec::data("item_attribute_value", "Item Attribute Value")
                .in_list_view()
                .required(),
        ]
    );

    let row = ItemVariant::new(
        "Size",
        "Large",
        Some("T-Shirt"),
        Some("attributes"),
        Some("Item"),
    );
    assert_eq!(row.item_attribute.as_deref(), Some("Size"));
    assert_eq!(row.item_attribute_value.as_deref(), Some("Large"));
    assert_eq!(row.doctype(), "Item Variant");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn landed_cost_vendor_invoice_matches_erpnext_metadata_and_fields() {
    assert_eq!(
        LandedCostVendorInvoice::DOCTYPE,
        "Landed Cost Vendor Invoice"
    );
    assert_eq!(LandedCostVendorInvoice::MODULE, "Stock");
    assert_eq!(
        LandedCostVendorInvoice::FIELD_ORDER,
        ["vendor_invoice", "amount"]
    );
    assert!(LandedCostVendorInvoice::EDITABLE_GRID);
    assert!(LandedCostVendorInvoice::IS_TABLE);
    assert_eq!(LandedCostVendorInvoice::SORT_FIELD, "creation");
    assert_eq!(LandedCostVendorInvoice::SORT_ORDER, "DESC");

    assert_eq!(
        LandedCostVendorInvoice::fields(),
        vec![
            FieldSpec::link("vendor_invoice", "Vendor Invoice")
                .options("Purchase Invoice")
                .in_list_view()
                .search_index(),
            FieldSpec::currency("amount", "Amount (Company Currency)")
                .options("Company:company:default_currency")
                .in_list_view()
                .read_only(),
        ]
    );

    let row = LandedCostVendorInvoice::new(
        Some("PINV-0001"),
        Some(125.5),
        Some("LCV-0001"),
        Some("vendor_invoices"),
        Some("Landed Cost Voucher"),
    );
    assert_eq!(row.vendor_invoice.as_deref(), Some("PINV-0001"));
    assert_eq!(row.amount, Some(125.5));
    assert_eq!(row.doctype(), "Landed Cost Vendor Invoice");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn item_website_specification_matches_erpnext_metadata_and_fields() {
    assert_eq!(
        ItemWebsiteSpecification::DOCTYPE,
        "Item Website Specification"
    );
    assert_eq!(ItemWebsiteSpecification::MODULE, "Stock");
    assert_eq!(
        ItemWebsiteSpecification::FIELD_ORDER,
        ["label", "description"]
    );
    assert!(ItemWebsiteSpecification::EDITABLE_GRID);
    assert!(ItemWebsiteSpecification::IS_TABLE);
    assert_eq!(ItemWebsiteSpecification::SORT_FIELD, "creation");
    assert_eq!(ItemWebsiteSpecification::SORT_ORDER, "DESC");

    assert_eq!(
        ItemWebsiteSpecification::fields(),
        vec![
            FieldSpec::data("label", "Label")
                .in_list_view()
                .width("150px"),
            FieldSpec::text_editor("description", "Description")
                .in_list_view()
                .width("300px"),
        ]
    );

    let row = ItemWebsiteSpecification::new(
        Some("Material"),
        Some("Cotton"),
        Some("T-Shirt"),
        Some("website_specifications"),
        Some("Item"),
    );
    assert_eq!(row.label.as_deref(), Some("Material"));
    assert_eq!(row.description.as_deref(), Some("Cotton"));
    assert_eq!(row.doctype(), "Item Website Specification");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn shipment_delivery_note_matches_erpnext_metadata_and_fields() {
    assert_eq!(ShipmentDeliveryNote::DOCTYPE, "Shipment Delivery Note");
    assert_eq!(ShipmentDeliveryNote::MODULE, "Stock");
    assert_eq!(
        ShipmentDeliveryNote::FIELD_ORDER,
        ["delivery_note", "grand_total"]
    );
    assert!(ShipmentDeliveryNote::EDITABLE_GRID);
    assert!(ShipmentDeliveryNote::IS_TABLE);
    assert!(ShipmentDeliveryNote::QUICK_ENTRY);
    assert_eq!(ShipmentDeliveryNote::SORT_FIELD, "creation");
    assert_eq!(ShipmentDeliveryNote::SORT_ORDER, "DESC");
    assert!(ShipmentDeliveryNote::TRACK_CHANGES);

    assert_eq!(
        ShipmentDeliveryNote::fields(),
        vec![
            FieldSpec::link("delivery_note", "Delivery Note")
                .options("Delivery Note")
                .in_list_view()
                .required(),
            FieldSpec::currency("grand_total", "Value")
                .in_list_view()
                .read_only(),
        ]
    );

    let row = ShipmentDeliveryNote::new(
        "DN-0001",
        Some(500.0),
        Some("SHIP-0001"),
        Some("shipment_delivery_note"),
        Some("Shipment"),
    );
    assert_eq!(row.delivery_note.as_deref(), Some("DN-0001"));
    assert_eq!(row.grand_total, Some(500.0));
    assert_eq!(row.doctype(), "Shipment Delivery Note");
    assert!(row.custom_hooks().is_empty());
}

#[test]
fn shipment_parcel_matches_erpnext_metadata_and_fields() {
    assert_eq!(ShipmentParcel::DOCTYPE, "Shipment Parcel");
    assert_eq!(ShipmentParcel::MODULE, "Stock");
    assert_eq!(
        ShipmentParcel::FIELD_ORDER,
        ["length", "width", "height", "weight", "count"]
    );
    assert!(ShipmentParcel::EDITABLE_GRID);
    assert!(ShipmentParcel::IS_TABLE);
    assert!(ShipmentParcel::QUICK_ENTRY);
    assert_eq!(ShipmentParcel::SORT_FIELD, "creation");
    assert_eq!(ShipmentParcel::SORT_ORDER, "DESC");
    assert!(ShipmentParcel::TRACK_CHANGES);

    assert_eq!(
        ShipmentParcel::fields(),
        vec![
            FieldSpec::float("length", "Length (cm)").in_list_view(),
            FieldSpec::float("width", "Width (cm)").in_list_view(),
            FieldSpec::float("height", "Height (cm)").in_list_view(),
            FieldSpec::float("weight", "Weight (kg)")
                .in_list_view()
                .precision("1")
                .required(),
            FieldSpec::int("count", "Count")
                .default("1")
                .in_list_view()
                .required(),
        ]
    );

    let row = ShipmentParcel::new(
        Some(10.0),
        Some(20.0),
        Some(30.0),
        2.5,
        1,
        Some("SHIP-0001"),
        Some("parcels"),
        Some("Shipment"),
    );
    assert_eq!(row.length, Some(10.0));
    assert_eq!(row.weight, Some(2.5));
    assert_eq!(row.count, Some(1));
    assert_eq!(row.doctype(), "Shipment Parcel");
    assert!(row.custom_hooks().is_empty());
}
